//! The tray and its native menu live on eframe's Windows message-loop thread.
use crate::lifecycle::{AppEvent, EventSender};
use kairos_platform::integration::{read_run, StartupApproval, RUN_VALUE};
use std::cell::RefCell;
use tray_icon::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent,
};

pub(crate) struct Tray {
    _icon: TrayIcon,
    startup: CheckMenuItem,
}
thread_local! {
    static STARTUP_ITEM: RefCell<Option<CheckMenuItem>> = const { RefCell::new(None) };
}
// TrayIconEvent runs synchronously on the owning Windows message thread, before
// tray-icon enters TrackPopupMenu. Update the native item before that nested loop.
fn refresh_startup_menu() {
    let state = std::env::current_exe()
        .ok()
        .and_then(|exe| read_run(RUN_VALUE, &exe).ok());
    STARTUP_ITEM.with(|slot| {
        if let Some(item) = slot.borrow().as_ref() {
            item.set_checked(state.as_ref().is_some_and(|s| s.command.is_some()));
            item.set_enabled(state.as_ref().is_some_and(|s| {
                !matches!(
                    s.approval,
                    StartupApproval::Disabled | StartupApproval::Unknown
                )
            }));
        }
    });
}
impl Tray {
    pub(crate) fn new(sender: EventSender) -> Result<Self, Box<dyn std::error::Error>> {
        let menu = Menu::new();
        let open = MenuItem::new("Kairos öffnen", true, None);
        let copy = MenuItem::new("Neues Passwort kopieren", true, None);
        let startup = CheckMenuItem::new("Mit Windows starten", false, false, None);
        let quit = MenuItem::new("Beenden", true, None);
        menu.append_items(&[
            &open,
            &copy,
            &PredefinedMenuItem::separator(),
            &startup,
            &PredefinedMenuItem::separator(),
            &quit,
        ])?;
        let icon = Icon::from_rgba(crate::icon_data().rgba, 64, 64)?;
        let icon = TrayIconBuilder::new()
            .with_tooltip("Kairos Passwort")
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .with_menu_on_left_click(false)
            .build()?;
        STARTUP_ITEM.with(|slot| *slot.borrow_mut() = Some(startup.clone()));
        refresh_startup_menu();
        let (open, copy, startup_id, quit) = (
            open.id().clone(),
            copy.id().clone(),
            startup.id().clone(),
            quit.id().clone(),
        );
        let actions = sender.clone();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            let action = if event.id == open {
                AppEvent::Show
            } else if event.id == copy {
                AppEvent::CopyNew
            } else if event.id == startup_id {
                AppEvent::ToggleStartup
            } else if event.id == quit {
                AppEvent::Quit
            } else {
                return;
            };
            actions.send(action);
        }));
        TrayIconEvent::set_event_handler(Some(move |event| {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
                | TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => sender.send(AppEvent::Show),
                // Refresh registry state when interacting with the tray, even if hidden.
                TrayIconEvent::Enter { .. }
                | TrayIconEvent::Click {
                    button: MouseButton::Right,
                    ..
                } => {
                    refresh_startup_menu();
                    sender.send(AppEvent::Refresh);
                }
                _ => {}
            }
        }));
        Ok(Self {
            _icon: icon,
            startup,
        })
    }
    pub(crate) fn set_startup(&self, checked: bool, enabled: bool) {
        self.startup.set_checked(checked);
        self.startup.set_enabled(enabled);
    }
    pub(crate) fn set_status(&self, message: Option<&str>) {
        let text = message.map_or_else(
            || "Kairos Passwort".to_owned(),
            |m| format!("Kairos Passwort · {m}"),
        );
        let _ = self._icon.set_tooltip(Some(text));
    }
}
impl Drop for Tray {
    fn drop(&mut self) {
        STARTUP_ITEM.with(|slot| slot.borrow_mut().take());
        MenuEvent::set_event_handler::<fn(MenuEvent)>(None);
        TrayIconEvent::set_event_handler::<fn(TrayIconEvent)>(None);
    }
}

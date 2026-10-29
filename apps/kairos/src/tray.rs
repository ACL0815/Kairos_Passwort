//! The tray and its native menu live on eframe's Windows message-loop thread.
use crate::lifecycle::{AppEvent, EventSender};
use tray_icon::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent,
};

pub(crate) struct Tray {
    _icon: TrayIcon,
    startup: CheckMenuItem,
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
                } => sender.send(AppEvent::Refresh),
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
}
impl Drop for Tray {
    fn drop(&mut self) {
        MenuEvent::set_event_handler::<fn(MenuEvent)>(None);
        TrayIconEvent::set_event_handler::<fn(TrayIconEvent)>(None);
    }
}

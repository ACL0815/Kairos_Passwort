//! F11 transaction: explicit pairing, save, then a fresh explicit input click.
//! Modelled after: KeePassXC's association and confirmed autotype boundary:
//! https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser/BrowserAction.cpp
use crate::{desktop::InputTarget, KairosApp};
use eframe::egui::{self, RichText};
use kairos_bridge::{Bridge, BrowserWindow, Context, PairingStatus, ProtectedStore};
use kairos_keepassxc::{Error, Group};
use std::{sync::mpsc, time::Duration};
use zeroize::Zeroizing;

pub(crate) trait ContextSource {
    fn latest(&self, window: Option<BrowserWindow>) -> Option<Context>;
}
impl ContextSource for Bridge {
    fn latest(&self, window: Option<BrowserWindow>) -> Option<Context> {
        self.latest_for(window)
    }
}

#[derive(Default)]
pub(crate) struct Integration {
    pub(crate) bridge: Option<Bridge>,
    bridge_error: bool,
    code: Option<Zeroizing<String>>,
    pub(crate) flow: Option<Flow>,
}
pub(crate) struct Flow {
    target: Option<Box<dyn InputTarget>>,
    candidate: Option<Context>,
    url: String,
    username: String,
    group: String,
    group_uuid: String,
    confirmed: bool,
    allow_pairing: bool,
    phase: Phase,
    pending: Option<mpsc::Receiver<Result<(), Error>>>,
    notice: &'static str,
    saved_secret: Option<Zeroizing<String>>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Draft,
    Pairing,
    Saving,
    Accepted,
    Finished,
}

struct Login {
    url: String,
    username: String,
    group: String,
    group_uuid: String,
    password: Zeroizing<String>,
}
trait Vault {
    fn test(&mut self) -> Result<(), Error>;
    fn associate(&mut self) -> Result<(), Error>;
    fn save(&mut self, login: &Login) -> Result<(), Error>;
}
fn save(vault: &mut impl Vault, login: &Login) -> Result<(), Error> {
    vault.test()?;
    vault.save(login)
}
#[cfg(windows)]
impl<T: kairos_keepassxc::Transport, S: kairos_keepassxc::AssociationStore> Vault
    for kairos_keepassxc::Client<T, S>
{
    fn test(&mut self) -> Result<(), Error> {
        self.test_associate().map(|_| ())
    }
    fn associate(&mut self) -> Result<(), Error> {
        self.associate()
    }
    fn save(&mut self, login: &Login) -> Result<(), Error> {
        let group = (!login.group.is_empty()).then_some(Group {
            name: &login.group,
            uuid: &login.group_uuid,
        });
        self.set_login(&login.url, &login.username, &login.password, group)
            .map(|_| ())
    }
}
fn execute(login: Option<Login>) -> Result<(), Error> {
    #[cfg(windows)]
    {
        let transport = kairos_keepassxc::NamedPipeTransport::connect(Duration::from_secs(60))?;
        let store = kairos_keepassxc::DpapiStore::for_current_user()?;
        let mut client = kairos_keepassxc::Client::connect(transport, store)?;
        if let Some(login) = login {
            save(&mut client, &login)
        } else {
            Vault::associate(&mut client)
        }
    }
    #[cfg(not(windows))]
    {
        let _ = login;
        Err(Error::UnsupportedPlatform)
    }
}

impl Flow {
    pub(crate) fn new(
        target: Option<Box<dyn InputTarget>>,
        source: Option<&dyn ContextSource>,
        window: Option<BrowserWindow>,
    ) -> Self {
        // A foreground HWND at receipt is only a hint. The userscript has no
        // trustworthy tab/window ID, and FocusTarget intentionally stays opaque.
        // Keep URL empty until the user explicitly adopts or types the proposal.
        Self {
            target,
            candidate: source.and_then(|s| s.latest(window)),
            url: String::new(),
            username: String::new(),
            group: String::new(),
            group_uuid: String::new(),
            confirmed: false,
            allow_pairing: false,
            phase: Phase::Draft,
            pending: None,
            notice: "URL und Zielfeld prüfen. Ein Userscript kann den aktiven Tab nicht beweisen.",
            saved_secret: None,
        }
    }
    pub(crate) fn busy(&self) -> bool {
        matches!(self.phase, Phase::Pairing | Phase::Saving)
    }
    pub(crate) fn blocks_hotkeys(&self) -> bool {
        self.busy() || self.phase == Phase::Accepted
    }
    fn submit(&mut self, password: &str, pair: bool, ctx: &egui::Context) {
        if self.phase != Phase::Draft {
            return;
        }
        let login = if pair {
            None
        } else {
            let Ok(url) = kairos_bridge::normalize_url(&self.url) else {
                self.notice = "Absolute HTTP(S)-URL ohne Zugangsdaten erforderlich.";
                return;
            };
            if url != self.url {
                self.url = url;
                self.confirmed = false;
                self.notice =
                    "Query/Fragment entfernt. Bereinigte URL erneut prüfen und bestätigen.";
                return;
            }
            if !self.confirmed || password.is_empty() || self.target.is_none() {
                return;
            }
            if password.chars().any(char::is_control) || password.encode_utf16().count() > 1024 {
                self.notice = "Vor KeePassXC abgebrochen: maximal 1024 UTF-16-Einheiten ohne Steuerzeichen für die spätere Eingabe.";
                return;
            }
            if self.username.len() > 320
                || self.username.chars().any(char::is_control)
                || (!self.group.is_empty()
                    && (self.group_uuid.len() != 32
                        || !self.group_uuid.bytes().all(|b| b.is_ascii_hexdigit())))
                || (self.group.is_empty() && !self.group_uuid.is_empty())
            {
                self.notice =
                    "Benutzername oder Gruppe ungültig. Gruppe benötigt Name und 32-stellige UUID.";
                return;
            }
            self.saved_secret = Some(Zeroizing::new(password.to_owned()));
            Some(Login {
                url,
                username: self.username.clone(),
                group: self.group.clone(),
                group_uuid: self.group_uuid.clone(),
                password: Zeroizing::new(password.to_owned()),
            })
        };
        let (send, recv) = mpsc::channel();
        let wake = ctx.clone();
        match std::thread::Builder::new()
            .name("kairos-keepassxc".into())
            .spawn(move || {
                let _ = send.send(execute(login));
                wake.request_repaint();
            }) {
            Ok(_) => {
                self.pending = Some(recv);
                self.phase = if pair { Phase::Pairing } else { Phase::Saving };
                self.notice = "KeePassXC-Anfrage läuft. Dialoge dort prüfen; kein automatischer Wiederholungsversuch.";
            }
            Err(_) => {
                self.saved_secret = None;
                self.notice = "KeePassXC-Worker konnte nicht starten.";
            }
        }
    }
    fn complete(&mut self, result: Result<(), Error>) {
        let pairing = self.phase == Phase::Pairing;
        match result {
            Ok(()) if pairing => {
                self.phase = Phase::Draft;
                self.allow_pairing = false;
                self.notice = "KeePassXC gekoppelt. Eintragen separat bestätigen.";
            }
            Ok(()) => {
                self.phase = Phase::Accepted;
                self.notice = "KeePassXC hat die Anfrage angenommen. Eintrag und Datenbank dort prüfen; Speicherung auf Platte ist nicht bestätigt. Danach ins gemerkte Feld tippen.";
            }
            Err(Error::MissingAssociation) if !pairing => {
                self.phase = Phase::Draft;
                self.saved_secret = None;
                self.allow_pairing = true;
                self.notice = "Keine KeePassXC-Zuordnung. KeePassXC entsperren, Browserintegration aktivieren und unten bewusst koppeln.";
            }
            Err(_) => {
                self.phase = Phase::Finished;
                self.saved_secret = None;
                self.target = None;
                self.notice = "KeePassXC-Anfrage fehlgeschlagen oder abgelehnt. Nichts getippt. Bei Schreibfehler/Timeout kann bereits ein Eintrag existieren: dort prüfen, nicht automatisch wiederholen. Beschädigte/widerrufene Zuordnung: siehe README.";
            }
        }
    }
    fn type_saved(&mut self) -> Result<(), kairos_platform::PlatformError> {
        if self.phase != Phase::Accepted {
            return Err(kairos_platform::PlatformError::TargetUnavailable);
        }
        self.phase = Phase::Finished;
        let target = self
            .target
            .take()
            .ok_or(kairos_platform::PlatformError::TargetUnavailable)?;
        target.type_text(
            self.saved_secret
                .as_deref()
                .map(|s| s.as_str())
                .ok_or(kairos_platform::PlatformError::InvalidText)?,
            true,
        )
    }
}

impl KairosApp {
    pub(crate) fn start_bridge(&mut self) {
        if cfg!(feature = "demo") {
            return;
        }
        match ProtectedStore::for_current_user()
            .and_then(|s| Bridge::start(Box::new(s), foreground_browser))
        {
            Ok(bridge) => self.keepass.bridge = Some(bridge),
            Err(_) => self.keepass.bridge_error = true,
        }
    }
    pub(crate) fn keepass_poll(&mut self) {
        if let Some(flow) = &mut self.keepass.flow {
            let result = flow
                .pending
                .as_ref()
                .and_then(|receiver| match receiver.try_recv() {
                    Ok(result) => Some(result),
                    Err(mpsc::TryRecvError::Disconnected) => Some(Err(Error::Transport)),
                    Err(mpsc::TryRecvError::Empty) => None,
                });
            if let Some(result) = result {
                flow.pending = None;
                flow.complete(result);
            }
        }
        if self
            .keepass
            .bridge
            .as_ref()
            .is_some_and(|b| b.status() != Ok(PairingStatus::Pending))
        {
            self.keepass.code = None;
        }
    }
    pub(crate) fn bridge_controls(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Browser-Brücke koppeln").show(ui, |ui| {
            if self.keepass.bridge_error { ui.label("Browser-Brücke nicht verfügbar (Port belegt oder DPAPI-Datei nicht lesbar). URL im F11-Dialog manuell eingeben. Keine Klartext-Ersatzablage."); }
            if let Some(bridge) = &self.keepass.bridge {
                match bridge.status() {
                    Ok(PairingStatus::Paired) => { ui.label("Browser gekoppelt · 127.0.0.1:19471 · Kontext 5 Sekunden. Neukopplung/Widerruf: siehe browser/README.md."); }
                    Ok(_) => {
                        if ui.button("Neuen Kopplungscode anzeigen (2 Minuten)").clicked() { self.keepass.code = bridge.begin_pairing().ok(); }
                        if let Some(code) = &self.keepass.code {
                            ui.label("Nur im Violentmonkey-Menü → Kairos verbinden eingeben. Nicht in ein Seitenformular. Code nach Kopplung verborgen; Zwischenablage ggf. selbst leeren.");
                            ui.ctx().request_repaint_after(Duration::from_millis(500));
                            // Explicit protected clipboard action, no ordinary egui export.
                            ui.add(egui::Label::new(RichText::new(code.as_str()).monospace()).selectable(false));
                            if ui.button("Kopplungscode geschützt kopieren (30 s)").clicked() {
                                self.notice = if self.clipboard.copy(code).is_ok() { "Kopplungscode geschützt kopiert (30 Sekunden)." } else { "Kopplungscode konnte nicht kopiert werden." };
                            }
                            if ui.button("Kopplung abbrechen").clicked() { bridge.cancel_pairing(); self.keepass.code = None; }
                        }
                    }
                    Err(_) => { ui.label("Browser-Brücke ausgefallen."); }
                }
            }
        });
    }
    pub(crate) fn keepass_view(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let Some(mut flow) = self.keepass.flow.take() else {
            return;
        };
        if ctx.memory(|memory| memory.has_focus(egui::Id::unique("password"))) {
            let copy = ctx.input_mut(|input| {
                let copy = input
                    .events
                    .iter()
                    .any(|event| matches!(event, egui::Event::Copy));
                input
                    .events
                    .retain(|event| !matches!(event, egui::Event::Copy | egui::Event::Cut));
                copy || input.consume_key(egui::Modifiers::CTRL, egui::Key::C)
            });
            if copy {
                self.copy(&ctx, false);
            }
        }
        ui.heading("KeePassXC · Eintrag bestätigen");
        ui.label("Zuerst KeePassXC, dann mit erneutem Klick ins gemerkte Feld. Browser-Feldwechsel im selben Fenster sind nicht erkennbar.");
        let draft = flow.phase == Phase::Draft;
        if flow.target.is_none() {
            ui.label("Kein externes Zielfeld bei F11 erfasst. Dialog schließen und F11 im gewünschten Feld auslösen.");
        }
        ui.add_enabled_ui(draft, |ui| {
            if let Some(candidate) = &flow.candidate {
                ui.label(format!("Bei F11 frischer Browser-Vorschlag (Fensterhinweis, keine Tab-Garantie): {}", candidate.url));
                if ui.button("Vorschlag nach eigener Prüfung übernehmen").clicked() { flow.url = candidate.url.clone(); flow.username = candidate.username.clone().unwrap_or_default(); flow.confirmed = false; }
            } else { ui.label("Keine frische, einem Vordergrund-Browser zuordenbare URL. Bitte manuell eingeben."); }
            ui.label("URL (ohne Query/Fragment)");
            if ui.text_edit_singleline(&mut flow.url).changed() { flow.confirmed = false; }
            ui.label("Benutzername / E-Mail (optional)"); ui.text_edit_singleline(&mut flow.username);
            ui.horizontal(|ui| { ui.label("Gruppe optional: Name"); ui.text_edit_singleline(&mut flow.group); });
            ui.horizontal(|ui| { ui.label("Gruppen-UUID"); ui.text_edit_singleline(&mut flow.group_uuid); });
            self.editor(ui);
            ui.checkbox(&mut flow.confirmed, "URL, Benutzername und ursprüngliches Zielfeld geprüft");
            ui.checkbox(&mut self.copy_on_insert, "Nach angenommener Eingabe auch kopieren (30 s)");
            if ui.add_enabled(flow.confirmed && flow.target.is_some() && !self.password.secret.is_empty(), egui::Button::new("Eintragen in KeePassXC")).clicked() { flow.submit(&self.password.secret, false, &ctx); }
            if flow.allow_pairing && ui.button("KeePassXC jetzt koppeln (Freigabe dort)").clicked() { flow.submit("", true, &ctx); }
        });
        ui.label(flow.notice);
        if flow.phase == Phase::Accepted && ui.button("Jetzt ins gemerkte Feld tippen").clicked() {
            match flow.type_saved() {
                Ok(()) => {
                    flow.notice = "KeePassXC-Anfrage angenommen; Eingabe an Windows übergeben. Eintrag und Zielfeld prüfen.";
                    if self.copy_on_insert
                        && self
                            .clipboard
                            .copy(
                                flow.saved_secret
                                    .as_deref()
                                    .map(|s| s.as_str())
                                    .unwrap_or_default(),
                            )
                            .is_err()
                    {
                        flow.notice = "KeePassXC-Anfrage angenommen und Eingabe übergeben, aber Kopieren fehlgeschlagen. Nur Kopieren erneut versuchen.";
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
                Err(_) => {
                    flow.notice = "KeePassXC-Anfrage angenommen, aber Eingabe fehlgeschlagen (möglicherweise teilweise). Zielfeld und Eintrag prüfen; kein automatischer Retry. Passwort bleibt zur manuellen Kopie erhalten.";
                }
            }
            flow.saved_secret = None;
        }
        if flow.phase == Phase::Finished && ui.button("Passwort manuell kopieren (30 s)").clicked()
        {
            self.copy(&ctx, false);
        }
        if !flow.busy() && ui.button("Dialog schließen").clicked() {
            self.target = crate::desktop::Destination::Unavailable;
        } else {
            self.keepass.flow = Some(flow);
        }
    }
}

/// Local process identity only. Never trust a userscript-supplied HWND or title.
pub(crate) fn foreground_browser() -> Option<BrowserWindow> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::{
            Foundation::CloseHandle,
            System::Threading::{
                OpenProcess, QueryFullProcessImageNameW, PROCESS_QUERY_LIMITED_INFORMATION,
            },
            UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
        };
        let window = unsafe { GetForegroundWindow() };
        if window.is_null() {
            return None;
        }
        let mut process = 0;
        unsafe {
            GetWindowThreadProcessId(window, &mut process);
        }
        let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process) };
        if handle.is_null() {
            return None;
        }
        let mut path = [0u16; 32768];
        let mut length = path.len() as u32;
        let success =
            unsafe { QueryFullProcessImageNameW(handle, 0, path.as_mut_ptr(), &mut length) };
        unsafe {
            CloseHandle(handle);
        }
        if success == 0 {
            return None;
        }
        let path = String::from_utf16_lossy(&path[..length as usize]);
        let name = path.rsplit('\\').next()?.to_ascii_lowercase();
        if !matches!(
            name.as_str(),
            "chrome.exe" | "msedge.exe" | "firefox.exe" | "brave.exe" | "vivaldi.exe" | "opera.exe"
        ) {
            return None;
        }
        Some(BrowserWindow {
            hwnd: window as usize,
            process,
        })
    }
    #[cfg(not(windows))]
    {
        None
    }
}

#[cfg(test)]
mod tests;

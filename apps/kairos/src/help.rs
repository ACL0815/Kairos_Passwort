//! Short public explanations; examples are fixed teaching material, never secrets.

use eframe::egui::{self, RichText};

use crate::strength::{self, CONCEPT_BITS};

pub const PRINCIPLE: &str = "Mehr Länge durch bekannte Zusätze bringt keine Bits. Nur unabhängige Zufallsentscheidungen vergrößern die Auswahl. Die Regeln dürfen dem Angreifer bekannt sein.";
pub const STRENGTH: &str = "Theoretisch (Konzept): feste Referenz aus dem Grundkonzept, 7.000 bzw. 21.000 Wörter mit sechs Buchstaben. Tatsächlich: konservative Min-Entropie aus dem Core mit realer Wortquelle, Sperrliste und Trennerausschlüssen. Vergleich: kürzeste mindestens gleich starke Zufallsalternative aus 94 druckbaren ASCII-Zeichen ohne Leerzeichen, alle vier Zeichenarten enthalten. G50-Modell = 2^H/2 Rateversuche bei Gleichverteilung; bei konservativer Min-Entropie nur Vergleich/Untergrenze, keine gemessene Knackdauer. Nach Handänderung keine Bewertung.";
pub const ROTATION: &str = "Bei jeder Generierung zieht der Core gleichverteilt aus den angehakten Profilen. Mindestens zwei, Wiederholungen sind möglich. Profil 0 nutzt Deutsch, ab Profil 1 DE+EN+LA; der Deutsch-Haken überschreibt alle. Angezeigt wird die konservative Stärke der gesamten Auswahl, nicht die Stärke des gerade gezogenen Profils. Die gewählte Profilnummer wird nicht ausgegeben. Aktivierung und Auswahl werden lokal gespeichert, keine Passwörter.";
pub const BONUS: &str = "Der angezeigte Bonus ist die konservative Berechnung des Generators, höchstens log2(Anzahl). Ungleich starke oder überlappende Profile erhalten keinen Bonus. Bei mehreren Wortquellen rechnet der Generator derzeit vorsichtig mit 0 Bit Bonus. Keine Auswahl der vorhandenen Profile 0–12 garantiert einen Bonus. Dafür wären gleich starke Profile mit nachweisbar getrennten Ausgabemengen und unbekannter aktueller Wahl nötig, etwa zwei eigene Profile mit gleich großen, disjunkten Trennerpools: höchstens +1 Bit. Eigene Profile sind noch nicht im GUI verfügbar. Die optionalen +4 Bit / bis 157,1 Bit im Konzept setzen 16 solche Profile voraus; 13 Profile erlauben höchstens +3,7 Bit.";
pub const EXCLUSIONS: &str = "Angehakte Sonderzeichen werden aus dem Trennerpool entfernt. Weniger Zufallsauswahl kann die tatsächlichen Bits senken. Ein leerer Pool verhindert Generierung; bei Rotation muss jedes gewählte Profil gültig bleiben. Enthält die Auswahl Profil 0 oder 1, muss das Minuszeichen erlaubt sein.";
pub const GERMAN: &str = "Ohne Haken: Profil 0 Deutsch, Profile 1–12 kombinierte DE/EN/LA-Liste; die Sprache wird je Wort unabhängig gewählt. Mit Haken: alle Profile nur Deutsch. Dadurch fehlen die zusätzlichen Wortauswahl-Bits von Profil 1. Die tatsächliche Stärke berücksichtigt die reale, gefilterte Liste.";
pub const BLOCKLIST: &str = "Einzelne Wörter bewusst aus künftigen Ziehungen ausschließen; kleinere Wortquelle bedeutet weniger Auswahlbits. Nur gewählte Wörter werden unverschlüsselt gespeichert, kein Passwortverlauf. Wörter bereits verwendeter Passwörter nicht sperren, da die Datei Bestandteile verraten kann. Nacharbeit bleibt bei gültigen Änderungen erhalten; Lesefehler sperren Generierung bis zum Neuladen.";
pub const CLIPBOARD: &str = "Kopieren / Strg+C kopiert das ganze Passwort geschützt. Kopieren und zurückspringen kehrt zum vorher gemerkten Fenster zurück. Kein Kairos-Passwortverlauf; Windows-Verlauf und Cloud-Sync werden per Kennzeichnung ausgeschlossen. Die eigene, unveränderte Kopie wird nach 30 Sekunden bzw. beim normalen Beenden geleert. Andere Programme können sie vorher lesen; fremde neue Clipboard-Inhalte bleiben erhalten.";
pub const HOTKEYS: &str = "Alle drei mit Strg+Umschalt: F9 merkt das Zielfeld, generiert und öffnet den Editor; ein neuer Klick auf Einfügen tippt nach der Bearbeitung. F10 generiert, tippt direkt und kopiert. F11 generiert und öffnet die KeePassXC-Bestätigung. Strg+R generiert neu, Strg+C kopiert das ganze Passwort. Browser-Feldwechsel im selben Fenster können unerkannt bleiben; ursprüngliches Zielfeld prüfen. Kein automatisches Absenden.";
pub const CAPTURE: &str = "Bildschirmaufnahmeschutz ist standardmäßig aktiv, soweit Windows und Aufnahmeprogramm ihn beachten. --allow-screencapture erlaubt Aufnahmen nur für diesen Start und zeigt ein Dauerbanner. Kameras, privilegierte Werkzeuge und manche Fernwartung bleiben außerhalb des Schutzes. Öffentliche Beispiele für Aufnahmen verwenden.";
pub const HARDENING: &str = "Der Sicherheitsreport zeigt jede beim Start versuchte Prozesshärtung samt Erfolg, fehlender Unterstützung oder Fehler. Teilfehler bleiben sichtbar. Härtung begrenzt bestimmte Dumps und neue Prozesszugriffe, beseitigt aber keine bestehenden Handles, privilegierten Leser oder alle Klartextkopien in UI, Zwischenablage und Zielprogramm.";
pub const BRIDGE: &str = "Violentmonkey liefert nach bewusster Kopplung nur kurzlebige Kontextvorschläge für F11. Den zweiminütigen Code ausschließlich im Violentmonkey-Menü → Kairos verbinden eingeben. Kairos übernimmt URL/Benutzername erst nach eigener Prüfung. Die Brücke überträgt keine generierten Passwörter und sendet keine Formulare ab. Einrichtung und Widerruf: browser/README.md.";
pub const KEEPASS: &str = "Strg+Umschalt+F11 erzeugt frisch. URL, Benutzername und ursprüngliches Zielfeld prüfen und KeePassXC-Anfrage bestätigen; KeePassXC muss eingerichtet und entsperrt sein. Nach Annahme tippt erst ein weiterer Klick ins gemerkte Feld. Annahme bestätigt keine bestimmte Datenbank oder Gruppe. Nach einem Timeout kann der Eintrag bereits existieren: selbst prüfen, kein automatischer Retry.";
pub const EDITOR: &str = "Direkt im Passwortfeld nachbearbeiten. Jede Änderung entfernt die berechnete Stärke und den Profilbonus, auch nach Wiederherstellen des Texts. Kein Undo-Verlauf. Änderungen an Einstellungen bewahren Nacharbeit bis zum nächsten Neu generieren. Neu generieren ersetzt das aktuelle Passwort ohne Verlauf.";

pub struct ProfileInfo {
    pub name: &'static str,
    pub rule: &'static str,
    pub example: &'static str,
}

pub const PROFILES: [ProfileInfo; 13] = [
    ProfileInfo { name: "Vier Wörter · feste Minuszeichen", rule: "Basis: vier unabhängige Zufallswörter aus der deutschen Liste, klein, feste Minuszeichen. Konzept: 7.000 Wörter.", example: "ahorn-wolke-kiesel-falke" },
    ProfileInfo { name: "Vier Wörter · größere Wortauswahl", rule: "Neu: größere kombinierte DE/EN/LA-Liste statt nur Deutsch. Konzept: 21.000 unterschiedliche Wörter; keine extra Sprachbits neben der Wortauswahl.", example: "ahorn-cloud-lapis-falke" },
    ProfileInfo { name: "Fünf mögliche Trenner", rule: "Neu: jeder der drei Trenner wird aus -+.,# gezogen. Maßgeblich sind die aktuellen Core-Trennerregeln und Ausschlüsse.", example: "ahorn+cloud,lapis#falke" },
    ProfileInfo { name: "Zusätzliche Zufallsziffer", rule: "Neu: eine Ziffer 0–9 an einer zufälligen inneren Position eines zufällig gewählten Worts.", example: "ahorn+clo7ud,lapis#falke" },
    ProfileInfo { name: "Acht mögliche Trenner", rule: "Neu: acht mögliche Trenner; zusätzlich * ! ?. Alle bisherigen Regeln bleiben erhalten.", example: "ahorn*clo7ud?lapis!falke" },
    ProfileInfo { name: "Zwölf mögliche Trenner", rule: "Neu: zwölf mögliche Trenner; zusätzlich _ : ; /. Alle bisherigen Regeln bleiben erhalten.", example: "ahorn_clo7ud:lapis/falke" },
    ProfileInfo { name: "16 mögliche Trenner", rule: "Neu: 16 mögliche Trenner; zusätzlich runde Klammern und beide Anführungszeichen.", example: "ahorn(clo7ud)lapis\"falke" },
    ProfileInfo { name: "20 mögliche Trenner", rule: "Neu: 20 mögliche Trenner; zusätzlich $ % & =. Alle bisherigen Regeln bleiben erhalten.", example: "ahorn$clo7ud%lapis&falke" },
    ProfileInfo { name: "Zufällige Wortanfänge", rule: "Neu: jeden Wortanfang unabhängig zufällig groß oder klein schreiben; das liefert im Konzept vier Auswahlbits.", example: "Ahorn$clo7ud%Lapis&falke" },
    ProfileInfo { name: "Fünf Wörter", rule: "Neu: fünf statt vier unabhängige Zufallswörter; bisherige Trenner-, Ziffern- und Wortanfangsregeln bleiben.", example: "Ahorn$clo7ud%Lapis&falke=Amber" },
    ProfileInfo { name: "Sechs Wörter", rule: "Neu: sechs statt fünf unabhängige Zufallswörter; bisherige Regeln bleiben erhalten.", example: "Ahorn$clo7ud%Lapis&falke=Amber?silva" },
    ProfileInfo { name: "Zufälliger Großbuchstabe je Wort", rule: "Neu: genau einen zufällig positionierten Großbuchstaben je Wort statt groß/klein am Wortanfang. Der Core rechnet mit der kleinsten möglichen Auswahl der realen Wörter.", example: "ahOrn$clo7Ud%laPis&falKe=ambEr?silVa" },
    ProfileInfo { name: "Zwei Trenner je Wortpaar", rule: "Neu: zwei zufällig gezogene Trenner zwischen jedem Wortpaar, aus dem Pool von 20. Die tatsächliche Stärke folgt den Core-Regeln.", example: "ahOrn$+clo7Ud%_laPis&!falKe=:ambEr?/silVa" },
];

pub fn profile_text(id: u8) -> String {
    let info = &PROFILES[id as usize];
    let gain = if id == 0 {
        "Basis: kein Vorgänger.".to_owned()
    } else {
        format!(
            "Konzeptgewinn gegenüber Profil {}: +{} Bit (gerundete Referenzwerte).",
            id - 1,
            strength::decimal(CONCEPT_BITS[id as usize] - CONCEPT_BITS[id as usize - 1])
        )
    };
    format!("Profil {id} · {}\n{}\nTheoretisch (Konzept): {} Bit. {gain}\nÖffentliches Lehrbeispiel – niemals verwenden:\n{}\n\n{PRINCIPLE}", info.name, info.rule, strength::decimal(CONCEPT_BITS[id as usize]), info.example)
}

/// Same small hover/click affordance throughout the app. Call inside a stable ID.
pub fn info(ui: &mut egui::Ui, title: &str, text: &str) {
    // Draw the circle ourselves: the bundled proportional font has no ⓘ glyph.
    let response = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(5.0, 2.0);
            ui.spacing_mut().interact_size = egui::vec2(22.0, 22.0);
            ui.add(
                egui::Button::new(RichText::new("i").size(12.0).color(crate::theme::CYAN))
                    .frame(false),
            )
        })
        .inner
        .on_hover_text(text);
    ui.painter().circle_stroke(
        response.rect.center(),
        8.0,
        egui::Stroke::new(1.0, crate::theme::CYAN),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::Role::Button, true, format!("Info: {title}"))
    });
    egui::Popup::menu(&response).show(|ui| {
        ui.set_max_width(440.0);
        ui.strong(title);
        egui::ScrollArea::vertical()
            .max_height(260.0)
            .show(ui, |ui| {
                ui.label(text);
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_thirteen_profiles_explain_rules_gain_and_public_examples() {
        for id in 0..=12 {
            let info = &PROFILES[id as usize];
            assert!(!info.name.is_empty() && !info.rule.is_empty() && !info.example.is_empty());
            let text = profile_text(id);
            assert!(text.contains(&format!("Profil {id}")));
            assert!(text.contains(&strength::decimal(CONCEPT_BITS[id as usize])));
            assert!(text.contains("Öffentliches Lehrbeispiel"));
            assert!(text.contains(PRINCIPLE));
            assert!(text.contains(if id == 0 { "Basis" } else { "Konzeptgewinn" }));
        }
    }

    #[test]
    fn info_opens_by_click_and_closes_with_escape_in_a_small_viewport() {
        let ctx = egui::Context::default();
        crate::theme::install(&ctx);
        let render = |events| {
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(550.0, 360.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| info(ui, "Öffentliche Hilfe", "PUBLIC EXPLANATION"),
            );
            output.textures_delta.clear();
            output
        };
        let initial = render(vec![]);
        let position = initial
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) if text.galley.text() == "i" => {
                    Some(shape.shape.visual_bounding_rect().center())
                }
                _ => None,
            })
            .expect("info icon");
        let _ = render(vec![
            egui::Event::PointerMoved(position),
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]);
        let _ = render(vec![egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        let visible = render(vec![]);
        assert!(visible.shapes.iter().any(|shape| matches!(&shape.shape,
            egui::epaint::Shape::Text(text) if text.galley.text().contains("PUBLIC EXPLANATION"))));
        let _ = render(vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        let closed = render(vec![]);
        assert!(!closed.shapes.iter().any(|shape| matches!(&shape.shape,
            egui::epaint::Shape::Text(text) if text.galley.text().contains("PUBLIC EXPLANATION"))));
    }
}

use std::io::{self, Write};

/// Struktur zum Speichern des aktuellen Zustand der Anwendung
/// (z. B eingegebener Benutzername)
#[derive(Debug, Clone)]

struct AppState {
    username: Option<String>,
}

/// Hauptstruktur der Terminal-Anwendung
/// Diese Struktur kapseln Logik und Zustand
struct TerminalApp {
    state: AppState,
}

impl TerminalApp {
    /// Erstellt eine neue Instanz der Anwendung
    fn new() -> Self {
        Self {
            state: AppState { username: None },
        }
    }
/// Methode für die Benutzereingabe (User Input)
/// Liest eine Zeile aus dem Terminal (stdin)
    fn user_input(&self, promt: &str) -> io::Result<String> {
        // Promt im Terminal anzeigen
        print!("{promt}");
// Flush ist notwendig, damit der Text sofort angezeigt wird
        io::stdout().flush()?;

        let mut buf = String::new();
        // Benutzereingabe
        io::stdin().read_line(&mut buf)?;
        // Whitespace entfernen und String zurückgeben
        Ok(buf.trim().to_string())
    }
/// Methode für die Benutzerausgabe
/// Gibt Text im Terminal aus
    fn user_output(&self, text: &str) {
        println!("{text}");
    }
/// Zeigt das Hauptmenü der Anwendung an
    fn show_menu(&self) {
        println!("\n+++ EMBEDDED USER MENU +++");
        println!("1) Enter / change username");
        println!("2) Show greeting");
        println!("3) Show system info");
        println!("4) Exit");
    }
/// Simulierte Systeminformationen
/// In einem echten Embedded-System könnten hier Hardware- oder OS-Daten stehen
    fn system_info(&self) {
        self.user_output("System Info:");
        self.user_output("- Device: Raspberry Pi (target)");
        self.user_output("- UI: Terminal on monitor");
        self.user_output("- Memory model: Rust ownership / borrowing");
        self.user_output("- Status: OK");
    }
/// Hauptprogrammschleife
/// Verarbeitet Benutzereingabe und steuert die Programmlogik
    fn run(&mut self) -> io::Result<()> {
        loop {
            self.show_menu();
            // Auswahl des Benutzers einlesen
            let choice = self.user_input("Select option (1-4): ")?;

            match choice.as_str() {
            "1" => {
                // Benutzername setzen oder ändern
                let name = self.user_input("Enter username: ")?;
                if name.is_empty() {
                    self.user_output("Username not changed (empty input)");
                } else {
                    self.state.username = Some(name);
                    self.user_output("Username saved");
                }
            }
            "2" => {
                // Begrüßung ausgeben
                let greeting = match &self.state.username {
                    Some(name) => format!("Hello, {name}! :)"),
                    None => "Hello! (No username set yet)".to_string(),
                };
                self.user_output(&greeting);
            }
            "3" => {
                // Systeminformationen anzeigen
                self.system_info();
            }
            "4" => {
                // Programm sauber beenden
                self.user_output("Bye!");
                break;
            }
            _ => {
                // Ungültige Eingabe behandeln
                self.user_output("Invalid option. Please choose 1-4.");
            }
            }
        }

        Ok(())

    }
}

/// Programmeinstiegspunkt
fn main() -> io::Result<()> {
    // Anwendung initialisieren und starten
    let mut app = TerminalApp::new();
    app.run()
}
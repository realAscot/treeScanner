use std::collections::HashSet;
use std::fs;
use std::io::Write;

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct ConfigFile {
    pub max_depth: Option<usize>,
    pub max_files_per_dir: Option<usize>,
    pub ignore: Option<HashSet<String>>,
    pub language: Option<String>,
    pub align_comments: Option<bool>,
    pub output: Option<String>,
    pub viewonly: Option<bool>,
}

/// Lädt die Konfigurationsdatei aus dem Home-Verzeichnis (`~/.treescanner.conf`).
///
/// Falls die Datei nicht existiert, wird eine gültige Standard-Konfigurationsdatei erstellt.
/// Bei Fehlern während des Lesens oder Parsens wird `None` zurückgegeben.
///
/// # Rückgabewert
/// * `Some(ConfigFile)` – wenn Datei existiert und geparst werden konnte
/// * `None` – bei Fehlern oder wenn Datei nicht erstellt/parst werden konnte
pub fn load_config_from_home() -> Option<ConfigFile> {
    let home = dirs::home_dir()?;
    let config_path = home.join(".treescanner.conf");

    // Versuche, die Datei zu lesen
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            match toml::from_str::<ConfigFile>(&content) {
                Ok(cfg) => Some(cfg),
                Err(e) => {
                    eprintln!("⚠️  Konfigurationsdatei ungültig (TOML-Fehler): {e}");
                    println!("ℹ️  Du findest die Datei hier: {}", config_path.display());
                    println!("🔁 Du kannst sie löschen, sie wird beim nächsten Start neu erstellt.");
                    None
                }
            }
        }

        // Datei existiert nicht – erstelle neue Standard-Konfigurationsdatei
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default_config = r#"
# TreeScanner Standard-Konfiguration
# Nicht gesetzte Felder werden intern auf Defaultwerte gesetzt.

# max_depth = 5                    # default: unendlich
# max_files_per_dir = 1000         # default: 100
ignore = [".git", ".fingerprint"]  # default: ""
language = "de"
align_comments = false             # default: false
output = "treescanner.txt"         # default: tree.txt
viewonly = false                   # default: false
"#;

            let result = fs::File::create(&config_path)
                .and_then(|mut f| f.write_all(default_config.trim_start().as_bytes()));

            match result {
                Ok(_) => {
                    println!(
                        "\n✏️  Standard-Konfigurationsdatei erstellt: < {} >\n",
                        config_path.display()
                    );
                    // Lies die frisch geschriebene Datei direkt ein
                    fs::read_to_string(&config_path)
                        .ok()
                        .and_then(|s| toml::from_str(&s).ok())
                }
                Err(e) => {
                    eprintln!("⚠️  Konnte Standard-Konfigurationsdatei nicht schreiben: {e}");
                    None
                }
            }
        }

        // Anderer Lese-Fehler (z. B. Berechtigung)
        Err(e) => {
            eprintln!("⚠️  Fehler beim Lesen der Konfigurationsdatei: {e}");
            None
        }
    }
}
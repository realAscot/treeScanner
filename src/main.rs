//! Ein kleines CLI-Tool, das ein Verzeichnis einliest und
//! eine ASCII-Baumstruktur ausgibt.
//! (C) 2025 - Adam Skotarczak <adam@skotarczak.net>

mod config; // Enthält CLI-Argument-Parsing und Konfigurations-Loader
mod app;    // Business-Logik: TreeBuilder
mod utils;  // Hilfsfunktionen: Spinner, Logger, …


use app::treebuilder::{TreeBuilder, TreeBuilderConfig};
use config::args::CliArgs;                      // Definition der CLI-Argumente
use config::loader::load_config_from_home;      // Lädt optionale Datei-Konfiguration
use utils::ascii_spinner::start_spinner;        // Zeigt einen Lade-Spinner
use clap::Parser;                               // CLI-Parsing mit Clap
use std::fs;
use std::time::Instant;
use utils::logger::init_logger;                 // Initialisiert den Logger


/// Gibt die verstrichene Zeit seit `timer` aus.
fn view_timer(timer: &Instant) {
    // `elapsed()` liefert eine Duration, hier formatiert mit (2) Nachkommastellen
    println!("\n⏱️ Gesamtlaufzeit des Scans: {:.2?}", timer.elapsed());
}


/// Normalisiert einen Pfad-Eintrag für `--ignore`,
/// entfernt führende "./" bzw. ".\" und schließende Slashes.
fn normalize_ignore_entry(s: &str) -> String {
    s.trim_start_matches("./")
     .trim_start_matches(".\\")
     .trim_end_matches('/')
     .trim_end_matches('\\')
     .to_string()
}


fn main() {
    // 1. Logger konfigurieren (z.B. env_logger)
    init_logger();


    // 2. CLI-Argumente parsen: root_path, output, ignore, etc
    let args = CliArgs::parse();

    // 3. Optional: Konfigurationsdatei aus ~/.config/... laden
    let file_config = load_config_from_home().unwrap_or_default();

    // 4. Timer starten, um Laufzeit zu messen
    let timer = Instant::now();

    // 5. Validierung: root_path muss ein Verzeichnis sein
    if !args.root_path.is_dir() {
        eprintln!("⚠️  Fehler: '{}' ist kein gültiges Verzeichnis.", args.root_path.display());
        std::process::exit(1);
    }


    // 6. Falls ein Ausgabepfad angegeben ist, erstelle ggf. das Elternverzeichnis
    if let Some(parent) = args.output.as_ref().and_then(|p| p.parent()) {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("⚠️  Fehler beim Erstellen des Zielordners: {e}");
            std::process::exit(1);
        }
    }


    // 7. Ignored-Dirs: CLI hat Vorrang, sonst Datei-Konfig, sonst leer
    let ignored_dirs: Vec<String> = match (!args.ignore.is_empty(), file_config.ignore) {
        (true, _) => args.ignore.iter().map(|s| normalize_ignore_entry(s)).collect(),
        (false, Some(set)) => set.into_iter().map(|s| normalize_ignore_entry(&s)).collect(),
        (false, None) => vec![],
    };


    // 8. TreeBuilder-Konfiguration zusammenstellen
    let config = TreeBuilderConfig {
        root_path: args.root_path.clone(),
        max_depth: args.max_depth.or(file_config.max_depth), // CLI oder Datei
        max_files_per_dir: if args.max_files_per_dir != 100 {
            args.max_files_per_dir // CLI-Wert, wenn abweichend
        } else {
            file_config.max_files_per_dir.unwrap_or(100) // sonst Datei oder Default
        },
        ignored_dirs,
        folder_icon: "📁".to_string(),      // Alt: ▭
        file_icon: "📄".to_string(),        // Alt: 
        align_comments: args.align_comments,
    };

    // 9. TreeBuilder initialisieren
    let mut builder = TreeBuilder::new(config);

    // 10. Optional: Spinner starten, wenn nicht `--quiet
    let (stop_spinner, spinner_handle) = if !args.quiet {
        let (s, h) = start_spinner(8);
        (Some(s), Some(h))
    } else {
        (None, None)
    };

    // 11. Baumstruktur bauen und zu einem String zusammenfügen
    let mut output = builder.build_tree().join("\n");

    // 12. Kommentare in den Zeilen ausrichten (wenn gewünscht)
    if builder.config.align_comments {
        let lines = output.lines().map(String::from).collect::<Vec<_>>();
        output = builder.align_lines_with_comments(&lines).join("\n");
    }

    // 13. Spinner stoppen und Thread beenden
    if let Some(stop) = stop_spinner {
        let _ = stop.send(());
    }
    if let Some(handle) = spinner_handle {
        let _ = handle.join();
    }

    // 14. Ausgabemodus: in Datei schreiben oder nur anzeigen
    let viewonly = args.viewonly || file_config.viewonly.unwrap_or(false);
    let output_path = args.output.clone().or_else(|| file_config.output.map(Into::into)).unwrap_or_else(|| "tree.txt".into());

    if !viewonly {
        // 15a. Schreibe Ergebnis in Datei
        if let Err(e) = fs::write(&output_path, &output) {
            eprintln!("Fehler beim Schreiben der Datei: {e}");
            std::process::exit(1);
        }
        if !args.quiet {
            // 16a. Statistik ausgeben
            let (folders, files) = builder.stats();
            println!(
                "Erfasst wurden {} Ordner und {} Dateien. Ergebnis in {} gespeichert.",
                folders,
                files,
                output_path.display()
            );
            view_timer(&timer);
        }
    } else {
        // 15b. Nur auf stdout ausgeben
        println!("{}", output);
        view_timer(&timer);
    }
}

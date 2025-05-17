use clap::Parser;
use std::path::PathBuf;

/// CLI-Argumente für TreeScanner
#[derive(Parser, Debug)]
#[command(  author ="Adam Skotarczak <adam@skotarczak.net>",
            version= "1.0.0",
            about = "TreeScanner: Verzeichnisse als ASCII-Baum visualisieren.",

long_about = r#"
TreeScanner ist ein leichtgewichtiges CLI-Tool zur strukturierten Darstellung von Verzeichnisinhalten.

Funktionen:
- Ausgabe als ASCII-Baum
- Optionen für Tiefe, Datei-Limit und Ignorierlisten
- Fortschrittsanzeige im Terminal
- Unterstützung für Konfigurationsdateien und CLI

Beispiel:
treescanner.exe --max-depth 3 --ignore .git,target
"#
)]
pub struct CliArgs {
    /// Root-Verzeichnis für den Scan (Standard: aktuelles Verzeichnis)
    #[arg(default_value = ".")]
    pub root_path: PathBuf,

    /// Maximale Scan-Tiefe
    #[arg(long)]
    pub max_depth: Option<usize>,

    /// Maximale Dateianzahl pro Verzeichnis (Standard: 100)
    #[arg(long, default_value_t = 100)]
    pub max_files_per_dir: usize,

    /// Verzeichnisse ignorieren (z. B. .git,target) durch Komma getrennt, ohne Leerzeichen.
    #[arg(short = 'x', long, value_delimiter = ',')]
    pub ignore: Vec<String>,

    /// Ausgabeziel (Standard: tree.txt)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Nur in Konsole anzeigen, keine Ausgabedatei speichern
    #[arg(long)]
    pub viewonly: bool,

    /// Debug-Modus aktivieren
    #[arg(short = 'D', long)]
    pub debug: bool,

    /// Keine Statusausgaben
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Kommentare ausrichten (DEV: optisch instabil)
    #[arg(long, default_value_t = false)]
    pub align_comments: bool,
} 

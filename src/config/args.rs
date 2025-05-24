use clap::Parser;
use std::path::PathBuf;

/// CLI-Argumente für TreeScanner
#[derive(Parser, Debug)]
#[command(  author = "Adam Skotarczak <adam@skotarczak.net>",
            // holt sich Version aus der Cargo.toml
            version,
            about = "TreeScanner: Verzeichnisse als ASCII-Baum visualisieren.",

long_about = r#"
TreeScanner ist ein leichtgewichtiges CLI-Tool zur strukturierten Darstellung von Verzeichnisinhalten.
Dokumentation [DE]: https://github.com/realAscot/treeScanner/blob/main/README.md

Funktionen:
- Ausgabe als ASCII-Baum mit 
- Optionen für Tiefe, Datei-Limit und Ignorierlisten
- Fortschrittsanzeige im Terminal
- Unterstützung für Konfigurationsdateien und CLI

Beispiel:
treescanner.exe --max-depth 3 --ignore .git,target
"#,
    // hier stellen wir das alte Format wieder her:
    help_template = "\
{name} {version}
{author-section}{about-section}
USAGE:
    {usage}

ARGS:
{all-args}{after-help}"
)]
pub struct CliArgs {
    /// Root-Verzeichnis für den Scan (Standard: aktuelles Verzeichnis)
    #[arg(default_value = ".")]
    pub root_path: PathBuf,

    /// Maximale Scan-Tiefe
    #[arg(short = 'f', long)]
    pub max_depth: Option<usize>,

    /// Maximale Dateianzahl pro Verzeichnis
    #[arg(short = 'e', long, default_value_t = 100)]
    pub max_files_per_dir: usize,

    /// Verzeichnisse ignorieren (z. B. .git,target) durch Komma getrennt, ohne Leerzeichen.
    #[arg(short = 'x', long, value_delimiter = ',')]
    pub ignore: Vec<String>,

    /// Ausgabeziel (Standard: tree.txt)
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Nur in Konsole anzeigen, keine Ausgabedatei speichern
    #[arg(short = 'i', long)]
    pub viewonly: bool,

    /// Debug-Modus aktivieren
    #[arg(short = 'D', long)]
    pub debug: bool,

    /// Keine Statusausgaben
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Kommentare ausrichten (DEV: optisch instabil)
    #[arg(short = 'c', long, default_value_t = false)]
    pub align_comments: bool,
} 

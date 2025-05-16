use clap::Parser;
use std::path::PathBuf;

/// Struktur für Kommandozeilenargumente mit Clap.
#[derive(Parser, Debug)]
#[command(
    name = "TreeScanner",
    author = "Adam Skotarczak <adam@skotarczak.net>",
    version,
    about = "Generiert eine ASCII-Baumstruktur eines Verzeichnisses."
)]
pub struct CliArgs {
    /// Stammverzeichnis (default: aktuelles Verzeichnis)
    #[arg(default_value = ".")]
    pub root_path: PathBuf,

    /// Maximale Anzahl Dateien pro Verzeichnis
    #[arg(short = 'n', long, default_value_t = 100)]
    pub max_files_per_dir: usize,

    /// Maximale Rekursionstiefe (optional)
    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,

    /// Keine Kommentar-Ausrichtung aktivieren (Zukunft)
    #[arg(long)]
    pub no_align_comments: bool,

    /// Sprache der Programmausgabe (de oder en)
    #[arg(short = 'l', long, default_value = "de")]
    pub language: String,

    /// Ignorierte Verzeichnisse (mehrfach möglich)
    #[arg(short = 'x', long, value_name = "DIR", num_args = 0..)]
    pub ignore: Vec<String>,

    /// Pfad zur Ausgabedatei (Default: tree.txt)
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Keine Dateiausgabe, nur Konsole
    #[arg(long)]
    pub viewonly: bool,
}

use std::fs;
use std::path::{Path, PathBuf};

/// Struktur zur Konfiguration des Verzeichnis-Scans.
pub struct TreeBuilderConfig {
    pub root_path: PathBuf,
    pub max_depth: Option<usize>,
    pub max_files_per_dir: usize,
    pub ignored_dirs: Vec<String>,
    pub folder_icon: String,
    pub file_icon: String,
}

/// Verantwortlich für das Erzeugen der ASCII-Baumstruktur.
pub struct TreeBuilder {
    config: TreeBuilderConfig,
    folder_count: usize,
    file_count: usize,
}

impl TreeBuilder {
    pub fn new(config: TreeBuilderConfig) -> Self {
        Self {
            config,
            folder_count: 0,
            file_count: 0,
        }
    }

    /// Startet den Scan und liefert das Ergebnis als String.
    pub fn build_tree(&mut self) -> String {
        let mut lines = vec![format!("{} {}/", self.config.folder_icon, self.config.root_path.display())];
        self.scan_dir(&self.config.root_path.clone(), 0, "", &mut lines);
        lines.join("\n")
    }

    fn scan_dir(&mut self, path: &Path, depth: usize, prefix: &str, lines: &mut Vec<String>) {
        if let Some(max) = self.config.max_depth {
            if depth >= max {
                return;
            }
        }

        let entries = match fs::read_dir(path) {
            Ok(entries) => entries.filter_map(Result::ok).collect::<Vec<_>>(),
            Err(_) => {
                lines.push(format!("{}└── [Zugriff verweigert] {}", prefix, path.display()));
                return;
            }
        };

        let mut folders = vec![];
        let mut files = vec![];

        for entry in entries {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if entry.path().is_dir() {
                if !self.config.ignored_dirs.contains(&file_name) {
                    folders.push((file_name, entry.path()));
                }
            } else {
                files.push(file_name);
            }
        }

        for (idx, (name, path)) in folders.iter().enumerate() {
            self.folder_count += 1;
            let connector = if idx < folders.len() - 1 || !files.is_empty() { "├── " } else { "└── " };
            lines.push(format!("{}{}{} {}", prefix, connector, self.config.folder_icon, name));

            let new_prefix = if idx < folders.len() - 1 || !files.is_empty() {
                format!("{}│   ", prefix)
            } else {
                format!("{}    ", prefix)
            };

            self.scan_dir(path, depth + 1, &new_prefix, lines);
        }

        let visible_files = &files[..files.len().min(self.config.max_files_per_dir)];
        let remaining = files.len().saturating_sub(visible_files.len());

        for (idx, name) in visible_files.iter().enumerate() {
            self.file_count += 1;
            let connector = if idx < visible_files.len() - 1 || remaining > 0 { "├── " } else { "└── " };
            lines.push(format!("{}{}{} {}", prefix, connector, self.config.file_icon, name));
        }

        if remaining > 0 {
            lines.push(format!("{}└── <und {} weitere Dateien>", prefix, remaining));
        }
    }

    /// Gibt die Anzahl gescannter Ordner und Dateien zurück.
    pub fn stats(&self) -> (usize, usize) {
        (self.folder_count, self.file_count)
    }
}

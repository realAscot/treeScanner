mod config;
mod app;
mod utils;

use app::treebuilder::{TreeBuilder, TreeBuilderConfig};
use config::args::CliArgs;
use config::loader::load_config_from_home;
use utils::ascii_spinner::start_spinner;
use clap::Parser;
use std::fs;
use std::time::Instant;
use utils::logger::init_logger;

/// Gibt die verstrichene Zeit seit `timer` aus.
fn view_timer(timer: &Instant) {
    println!("\n⏱️ Gesamtlaufzeit des Scans: {:.2?}", timer.elapsed());
}

fn main() {
    init_logger();
    let args = CliArgs::parse();
    let file_config = load_config_from_home().unwrap_or_default();
    let timer = Instant::now();

    if !args.root_path.is_dir() {
        eprintln!("⚠️  Fehler: '{}' ist kein gültiges Verzeichnis.", args.root_path.display());
        std::process::exit(1);
    }

    if let Some(parent) = args.output.as_ref().and_then(|p| p.parent()) {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("⚠️  Fehler beim Erstellen des Zielordners: {e}");
            std::process::exit(1);
        }
    }

    let config = TreeBuilderConfig {
        root_path: args.root_path.clone(),
        max_depth: args.max_depth.or(file_config.max_depth),
        max_files_per_dir: if args.max_files_per_dir != 100 {
            args.max_files_per_dir
        } else {
            file_config.max_files_per_dir.unwrap_or(100)
        },
        // Ignorierte Verzeichnisse: CLI hat Vorrang, dann Config, sonst leer
        ignored_dirs: match (!args.ignore.is_empty(), file_config.ignore) {
            (true, _) => args.ignore,
            (false, Some(set)) => set.into_iter().collect(),
            (false, None) => vec![],
        },
        folder_icon: "📁".to_string(),
        file_icon: "📄".to_string(),
        align_comments: args.align_comments,
    };

    let mut builder = TreeBuilder::new(config);

    let (stop_spinner, spinner_handle) = if !args.quiet {
        let (s, h) = start_spinner(8);
        (Some(s), Some(h))
    } else {
        (None, None)
    };

    let mut output = builder.build_tree().join("\n");

    if builder.config.align_comments {
        let lines = output.lines().map(String::from).collect::<Vec<_>>();
        output = builder.align_lines_with_comments(&lines).join("\n");
    }

    if let Some(stop) = stop_spinner {
        let _ = stop.send(());
    }
    if let Some(handle) = spinner_handle {
        let _ = handle.join();
    }

    let viewonly = args.viewonly || file_config.viewonly.unwrap_or(false);
    let output_path = args.output.clone().or_else(|| file_config.output.map(Into::into)).unwrap_or_else(|| "tree.txt".into());

    if !viewonly {
        if let Err(e) = fs::write(&output_path, &output) {
            eprintln!("Fehler beim Schreiben der Datei: {e}");
            std::process::exit(1);
        }
        if !args.quiet {
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
        println!("{}", output);
        view_timer(&timer);
    }
}

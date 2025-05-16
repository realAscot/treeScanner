mod config;
mod app;

use app::treebuilder::{TreeBuilder, TreeBuilderConfig};
use config::args::CliArgs;
use config::loader::load_config_from_home;
use clap::Parser;
use std::fs;

fn main() {
    let args = CliArgs::parse();
    let file_config = load_config_from_home().unwrap_or_default();

    if !args.root_path.is_dir() {
        eprintln!("Fehler: '{}' ist kein gültiges Verzeichnis.", args.root_path.display());
        std::process::exit(1);
    }

    if let Some(parent) = args.output.as_ref().and_then(|p| p.parent()) {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Fehler beim Erstellen des Zielordners: {e}");
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
        ignored_dirs: if !args.ignore.is_empty() {
            args.ignore
        } else {
            file_config.ignore.unwrap_or_default().into_iter().collect()
        },
        folder_icon: "📁".to_string(),
        file_icon: "📄".to_string(),
    };

    let mut builder = TreeBuilder::new(config);
    let output = builder.build_tree();

    let viewonly = args.viewonly || file_config.viewonly.unwrap_or(false);
    let output_path = args.output.clone().or_else(|| file_config.output.map(Into::into)).unwrap_or_else(|| "tree.txt".into());

    if !viewonly {
        if let Err(e) = fs::write(&output_path, &output) {
            eprintln!("Fehler beim Schreiben der Datei: {e}");
            std::process::exit(1);
        }
        let (folders, files) = builder.stats();
        println!(
            "Erfasst wurden {} Ordner und {} Dateien. Ergebnis in {} gespeichert.",
            folders,
            files,
            output_path.display()
        );
    } else {
        println!("{}", output);
    }
} 

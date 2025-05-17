use treescanner::config::loader::{load_config_from_home, ConfigFile};
use std::fs;
use std::path::PathBuf;
use std::fs::write;

fn write_temp_config(content: &str) -> PathBuf {
    let path = dirs::home_dir().unwrap().join(".treescanner.conf");
    fs::write(&path, content).expect("Konnte Testdatei nicht schreiben");
    path
}

#[test]
fn test_config_wird_korrekt_geladen() {
    let config_text = r#"
        max_depth = 2
        max_files_per_dir = 5
        ignore = [".git", "target"]
        viewonly = true
    "#;

    let conf_path = write_temp_config(config_text);
    let loaded: ConfigFile = load_config_from_home().expect("Konfig konnte nicht geladen werden");

    assert_eq!(loaded.max_depth, Some(2));
    assert_eq!(loaded.max_files_per_dir, Some(5));
    assert!(loaded.ignore.as_ref().unwrap().contains(".git"));
    assert_eq!(loaded.viewonly, Some(true));

    fs::remove_file(conf_path).ok();
}  

/// Test: `ignore` aus Konfigurationsdatei wird korrekt geladen
#[test]
fn test_ignore_from_config_file() {
    let temp_home = tempfile::tempdir().unwrap();
    let config_path = temp_home.path().join(".treescanner.conf");

    let config_content = r#"
ignore = [".git", "target", ".vscode"]
"#;
    write(&config_path, config_content).expect("Konnte temporäre Config nicht schreiben.");

    unsafe {
        std::env::set_var("HOME", temp_home.path());         // für Unix
        std::env::set_var("USERPROFILE", temp_home.path());  // für Windows
    }

    let config = load_config_from_home().expect("Konfig konnte nicht geladen werden.");
    let ignore_set = config.ignore.expect("ignore-Feld fehlt in Config");

    assert!(ignore_set.contains(".git"));
    assert!(ignore_set.contains("target"));
    assert!(ignore_set.contains(".vscode"));
}
use treescanner::config::loader::{load_config_from_home, ConfigFile};
use std::fs;
use std::path::PathBuf;

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

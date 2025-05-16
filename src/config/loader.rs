use std::collections::HashSet;
use std::fs;

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

pub fn load_config_from_home() -> Option<ConfigFile> {
    let home = dirs::home_dir()?;
    let config_path = home.join(".treescanner.conf");
    if !config_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&config_path).ok()?;
    toml::from_str(&content).ok()
}

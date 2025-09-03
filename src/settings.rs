use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
};

use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "settings.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub channels: Vec<u8>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channels: vec![0, 1],
        }
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = dirs::document_dir()
        .expect("could not find the Document directory")
        .join("ncontrol+");

    create_dir_all(&dir_path)?;

    let mut file_path = dir_path.clone();
    file_path.push(CONFIG_FILE);

    let json = serde_json::to_string_pretty(config).unwrap();
    write(file_path, json).expect("Failed to write config file");

    Ok(())
}

pub fn load_config() -> Config {
    let file_path = dirs::document_dir()
        .expect("Could not find the Document directory")
        .join("ncontrol+")
        .join(CONFIG_FILE);

    if Path::new(&file_path).exists() {
        let contents = read_to_string(file_path).unwrap();
        let output: Config = serde_json::from_str(&contents).unwrap_or_default();
        output
    } else {
        Config::default()
    }
}

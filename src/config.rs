use std::{io, fs};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml::{self, Value};

use dirs;

const CONFIG_DIR: &str = ".config/clash-subscribe/";
const CONFIG_FILE: &str = "config.yaml";
const OVERRIDE_FILE: &str = "override-settings.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainConfig {
    subscribe_url: String,
    local_config: String
}

impl MainConfig {
    pub fn read_from(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;

        serde_yaml::from_reader(file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn default() -> Self {
        let mut config_dir = dirs::home_dir().unwrap_or_default();
        config_dir.push(".config/clash/config.yaml");
        MainConfig {
            subscribe_url: "https://example.com/example_subscribe".to_string(),
            local_config: config_dir.to_str().unwrap_or_default().to_string()
        }
    }

    pub fn write_to(&self, path: &Path) -> io::Result<()> {
        let file = File::create(path)?;
        serde_yaml::to_writer(file, self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn remote_url(&self) -> &str {
        &self.subscribe_url
    }

    pub fn local_config(&self) -> &Path {
        Path::new(&self.local_config)
    }
}

pub fn config_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_default();
    home_dir.join(CONFIG_DIR)
}

pub fn load_config() -> io::Result<(MainConfig, HashMap<String, Value>, bool)> {
    let config_dir = config_path();
    let config_file = config_dir.join(CONFIG_FILE);
    let override_file = config_dir.join(OVERRIDE_FILE);

    let mut config = MainConfig::default();
    let mut override_config = HashMap::new();

    let mut success = true;

    if !config_dir.exists() || !config_dir.is_dir() {
        if config_dir.exists() {
            fs::remove_file(&config_dir)?;
        }
        fs::create_dir_all(&config_dir)?;
        success = false;
    }

    if !config_file.exists() || !config_file.is_file() {
        if config_file.exists() {
            fs::remove_dir(&config_file)?;
        }
        config.write_to(&config_file)?;
        success = false;
    } else {
        config = MainConfig::read_from(&config_file)?;
        if config.remote_url() == MainConfig::default().remote_url() {
            success = false;
        }
    }
    
    if !override_file.exists() || !override_file.is_file() {
        if override_file.exists() {
            fs::remove_dir(&override_file)?;
        }
        serde_yaml::to_writer(
            fs::File::create(&override_file)?,
            &override_config
        ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        success = false;
    } else {
        override_config = serde_yaml::from_reader(fs::File::open(&override_file)?)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    Ok((config, override_config, success))
}
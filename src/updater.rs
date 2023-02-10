use std::collections::HashMap;
use std::fs::File;
use std::io;

use serde_yaml::Value;

use super::config::MainConfig;

pub fn update_clash_subscription(config: &MainConfig, override_map: &HashMap<String, Value>) -> io::Result<()> {
    let req = reqwest::blocking::get(config.remote_url())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut clash_config: HashMap<String, Value> = serde_yaml::from_reader(req)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    for (key, val) in override_map {
        let old_value = clash_config.insert(key.clone(), val.clone());
        drop(old_value);
    }
    
    let clash_config_file = File::create(config.local_config())?;
    serde_yaml::to_writer(clash_config_file, &clash_config)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
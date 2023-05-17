mod config;
mod updater;

use std::error::Error;
use config::*;
use updater::*;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    println!("clash-subscribe v0.1.0");
    let (config, override_map, success) = load_config()?;
    if !success {
        println!("Config not set.");
        println!("Head over to {} for configuration file.", config::config_path().display());
        return Ok(())
    }
    println!("Config load complete.");
    println!("Trying to fetch clash config to {}", config.local_config().display());
    
    let mut successful = false;
    while !successful {
        successful = match update_clash_subscription(&config, &override_map) {
            Ok(()) => {
                println!("Successfully updated subscription! Exiting...");
                true
            },
            Err(err) => {
                println!("{}", err.to_string());
                println!("Failed to update subscription. Retrying...");
                false
            }
        };
    }
    Ok(())
}
mod config;
mod updater;

use std::io;
use config::*;
use updater::*;

fn main() -> io::Result<()> {
    println!("clash-subscribe v0.1.0");
    let (config, override_map, success) = load_config()?;
    if !success {
        println!("Config not set. Exiting...");
        return Ok(())
    }
    println!("Config load complete.");
    println!("Trying to fetch clash config from {} to {}", config.remote_url(), config.local_config().display());
    
    let mut successful = false;
    while !successful {
        successful = match update_clash_subscription(&config, &override_map) {
            Ok(()) => {
                println!("Successfully updated subscription! Exiting...");
                true
            },
            Err(err) => {
                println!("Failed to update subscription. Retrying...");
                println!("{}", err.to_string());
                false
            }
        };
    }
    Ok(())
}
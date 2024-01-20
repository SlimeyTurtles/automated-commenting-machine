use anyhow::{Context, Error};

use dirs::home_dir;
use reqwest::Client;
use std::{time::Duration, fs};

use crate::app_config::config::{self, Config};
use crate::img_handler::img;

fn is_directory(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_dir()
    } else {
        false
    }
}

async fn modify_file(dir: &str) {
    match fs::read_to_string(dir) {
        Result::Ok(contents) => {
            // Successfully read the file contents
            
            println!("File contents:\n{:?}", contents);

            let output = img_driver(contents).await;
            println!("{:?}", output);
        }
        Err(e) => {
            // Handle the error if the file cannot be read
            eprintln!("Error reading file: {:?}", e);
        }
    }
}

pub fn execute_prs(dir: &str, req_file_type: &str) {
    if !is_directory(dir) {
        modify_file(dir);
        return;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            execute_prs(entry.path().to_str().unwrap_or("."), req_file_type);
            continue;
        }
    }
}

async fn img_driver(contents: String) -> Result<String, Error> {
    let config_file = home_dir()
        .context("Failed to retrieve config directory.")?
        .join(".acm/config.toml");

    let config: Config = config::load_config(&config_file).await?;

    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;

    let img_url = img::generate_image_url(&http_client, &config, &contents).await?;
    println!("{}", img_url);
    Ok(img_url)
}
use anyhow::{Context, Error};

use crate::app_config::config::{self, Config};
use crate::git_handler::git;
use crate::img_handler::code_summarizer::generate_slide_summary;
use crate::img_handler::img;
use dirs::home_dir;



use reqwest::Client;
use std::{fs, time::Duration};


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

            img_driver(contents).await;
        }
        Err(e) => {
            // Handle the error if the file cannot be read
            eprintln!("Error reading file: {:?}", e);
        }
    }
}

pub fn get_doc_text(dir: &str) -> Option<String> {
    match fs::read_to_string(dir) {
        Result::Ok(contents) => {
            // Successfully read the file contents

            return Some(contents);
        }
        Err(e) => {
            // Handle the error if the file cannot be read
            println!("Error reading file: {:?}", e);
            return None;
        }
    }
}

pub fn dir_iter(dir: &str, mut arr: Vec<String>) -> Vec<std::string::String> {
    if !is_directory(dir) {
        match get_doc_text(dir) {
            Some(text) => {
                arr.push(text);
                return arr;
            }
            None => {
                return arr;
            }
        }
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            dir_iter(entry.path().to_str().unwrap_or("."), arr.clone());
            continue;
        }
    }

    return arr;
}

pub async fn execute_prs(dir: &str, req_file_type: &str) -> Result<(), Error> {
    let file_text = dir_iter(dir, Vec::new());
    
    let config_file = home_dir()
    .context("Failed to retrieve config directory.")?
    .join(".acm/config.toml");

    let config: Config = config::load_config(&config_file).await?;
    let diffs = git::git_diff().await?;

    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;
    
    generate_slide_summary(&http_client, &config, file_text).await?;
    Ok(())
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

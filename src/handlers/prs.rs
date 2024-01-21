use anyhow::{Context, Error};

use crate::app_config::config::{self, Config};
use crate::img_handler::code_summarizer::generate_slide_summary;
use dirs::home_dir;

use reqwest::Client;
use std::path::Path;
use std::{fs, time::Duration};
use async_recursion::async_recursion;

pub async fn execute_prs(dir: &str, file_type: &str) -> Result<(), Error> {
    
    let mut arr: Vec<String> = Vec::new();

    recursive_search(Path::new(dir), &mut arr).await?;

    Ok(())
}

#[async_recursion]
async fn recursive_search(dir: &Path, arr: &mut Vec<String>) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                recursive_search(&path, arr).await?;
            } else {
                let file_contents = fs::read_to_string(&path)?;
                arr.push(file_contents);
            }
        }
    }

    let config_file = home_dir()
        .context("Failed to retrieve config directory.")?
        .join(".acm/config.toml");

    let config: Config = config::load_config(&config_file).await?;

    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;
    
    generate_slide_summary(&http_client, &config, arr.to_vec()).await?;

    Ok(())
}
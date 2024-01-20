mod config;
mod git;
use anyhow::{Context, Result};
use dirs::home_dir;
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let config_file = home_dir()
        .context("Failed to retrieve config directory.")?
        .join(".acm/config.toml");

    let config = config::load_config(&config_file).await?;

    let diffs = git::git_diff().await?;

    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;

    let commit_message = git::generate_commit_message(&http_client, &config, &diffs).await?;
    println!("Diffs: {}", diffs);
    println!("Commit message: {}", commit_message,);

    Ok(())
}

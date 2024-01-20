mod config;
mod git;
mod img;
use anyhow::{Context, Result};
use config::Config;
use dirs::home_dir;
use reqwest::Client;
use std::time::Duration;

async fn git_commit(config: Config) -> Result<()> {
    git::git_checks().await?;

    let diffs = git::git_diff().await?;

    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;

    let commit_message = git::generate_commit_message(&http_client, &config, &diffs).await?;
    let commit_message = git::edit_commit_message(commit_message.trim())?;
    println!("{}", &git::git_commit(&commit_message).await?);

    Ok(())
}

async fn image(config: Config) -> Result<()> {

    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;

    let img_url = img::generate_image_url(&http_client, &config, "small dog flying through clouds").await?;
    println!("{}", img_url);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_file = home_dir()
        .context("Failed to retrieve config directory.")?
        .join(".acm/config.toml");

    let config = config::load_config(&config_file).await?;

    git_commit(config).await?;

    // image(config).await?;

    Ok(())
}

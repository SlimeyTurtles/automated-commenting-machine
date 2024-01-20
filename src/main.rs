mod config_loader;
mod git;
use anyhow::{Context, Result};
use dirs::home_dir;

#[tokio::main]
async fn main() -> Result<()> {
    let config_file = home_dir()
        .context("Failed to retrieve config directory.")?
        .join(".acm/config.toml");

    let _config = config_loader::load_config(&config_file).await?;

    Ok(())
}

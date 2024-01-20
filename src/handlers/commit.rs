
// use crate::{git_handler::git, app_config::config};

// use anyhow::{Context, Result};
// use dirs::home_dir;
// use reqwest::Client;
// use std::time::Duration;


// pub async fn execute_commit() -> Result<()>  {
//     let config_file = home_dir()
//         .context("Failed to retrieve config directory.")?
//         .join(".acm/config.toml");

//     let config = config::load_config(&config_file).await?;

//     git::git_checks().await?;

//     let diffs = git::git_diff().await?;

//     let http_client = Client::builder()
//         .timeout(Duration::from_secs(config.request_timeout))
//         .build()?;

//     let commit_message = git::generate_commit_message(&http_client, &config, &diffs).await?;
//     let commit_message = git::edit_commit_message(commit_message.trim())?;
//     println!("{}", &git::git_commit(&commit_message).await?);
//     Ok(())
// }

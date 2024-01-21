pub mod app_config;
mod git_handler;
mod handlers;
mod img_handler;
use clap::{Parser, Subcommand};

use handlers::prs::execute_prs;

use crate::{app_config::config, git_handler::git};
use anyhow::{Context, Result};
use dirs::home_dir;
use reqwest::Client;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author = "SF Hacks @ CruzHacks ^-^", version="0.0.1", about="Hii this Devtool automates some boring parts of coding like git commits")]
/// A Very simple Package Hunter
struct Arguments {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Adds a commit to the current path
    Commit {
        
    },
    /// Creates a Google Slide presentation demonstration the paths code 
    Presents { path: Option<String> },

    Comment {
        path: String
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.cmd {
        SubCommand::Comment{ path } => {

        },
        SubCommand::Presents { path } => match &path {
            Some(path) =>  execute_prs(&path),
            None => execute_prs("."),
        }.await?,
        SubCommand::Commit { } => {
            let config_file = home_dir()
                .context("Failed to retrieve config directory.")?
                .join(".acm/config.toml");

            let config = config::load_config(&config_file).await?;

            git::git_checks().await?;

            let diffs = git::git_diff().await?;

            let http_client = Client::builder()
                .timeout(Duration::from_secs(config.request_timeout))
                .build()?;

            let commit_message =
                git::generate_commit_message(&http_client, &config, &diffs).await?;
            let commit_message = git::edit_commit_message(commit_message.trim())?;
            println!("{}", &git::git_commit(&commit_message).await?)

        }  
    }
    Ok(())
}
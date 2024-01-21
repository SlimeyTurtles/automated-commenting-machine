pub mod app_config;
mod comment_handler;
mod git_handler;
mod handlers;
mod img_handler;
use crate::{app_config::config, git_handler::git};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dirs::home_dir;
use handlers::prs::execute_prs;
use reqwest::Client;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(
    author = "SF Hacks @ CruzHacks ^-^",
    version = "0.0.1",
    about = "Hii this Devtool automates some boring parts of coding like git commits"
)]
/// A Very simple Package Hunter
struct Arguments {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    // Writes commit message based on git diff
    Commit {},
    // Creates a README
    CRead { path: Option<String> },
    // Writes JSDocs for TypeScript functions
    Comment { path: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.cmd {
        SubCommand::Comment { path } => {
println!("Generating comments for {}", path);
            let result = comment_handler::comment::comment_file(&path).await?;
            println!("Done!\n{}", result);
        }
        SubCommand::CRead { path } => {
            match &path {
                Some(path) => execute_prs(path),
                None => execute_prs("."),
            }
            .await?
        }
        SubCommand::Commit {} => {
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

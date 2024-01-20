pub mod app_config;
mod git_handler;
mod handlers;
use clap::{App, Arg, SubCommand};

use handlers::prs::execute_prs;

use crate::{app_config::config, git_handler::git};

use anyhow::{Context, Result};
use dirs::home_dir;
use reqwest::Client;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("ACM")
    .version("1.0.0")
    .author("SF Hacks @ CruzHacks ^-^")
    .about("This command tool allows you to use GPT to modify your existing project and add comments")
    .subcommand(
        SubCommand::with_name("prs")
            .about("Reads the files specified in the path and creates slide document ")
            .arg(Arg::with_name("dir").required(true).help("The name of the directory comments are wanted on").default_value("./")).arg(Arg::with_name("type").help("The file type to add comments to")),
    )
  .subcommand(SubCommand::with_name("commit").about("Automates a commit")).get_matches();

    match matches.subcommand() {
        ("prs", Some(cmd)) => {
            let dir = cmd.value_of("dir").unwrap();
            let file_type = cmd.value_of("type").unwrap_or("all");
            execute_prs(dir, file_type)
        }
        ("commit", Some(_)) => {
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
            println!("{}", &git::git_commit(&commit_message).await?);
        }
        _ => {
            println!("No or unknown subcommand provided.");
        }
    }
    Ok(())
}

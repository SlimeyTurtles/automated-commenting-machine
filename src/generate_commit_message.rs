use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseMessage, CreateChatCompletionRequestArgs,
};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use spinoff::{spinners, Spinner};
use std::{path::Path, time::Duration};
use tokio::{
    fs::{create_dir_all, read_to_string, write},
    process::Command,
};
use which::which;

mod config_loader;
mod git_diff;

fn generate_commit_message() {
    // Creates payload for OpenAI API
    let payload = CreateChatCompletionRequestArgs::default()
        .max_tokens(config_loader.Config.max_tokens)
        .model(&config_loader.Config.model_name)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(&config_loader.Config.system_prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(config_loader.Config.user_prompt.replace("{}", ENTER_GIT_DIFFS_HERE))
                .build()?
                .into(),
        ])
        .build()
        .context("Failed to construct the request payload")?;

    let response = http_client
        .post(format!("{}/chat/completions", &config_loader.Config.api_base_url))
        .bearer_auth(&config_loader.Config.api_key)
        .json(&payload)
        .send()
        .await
        .context("Failed to send the request to the Inference API provider")?
        .error_for_status()?
        .json::<CommitMessageCandidates>()
        .await
        .context("Failed to parse the response from the Inference API provider")?;

    let commit_message = response
        .choices
        .first() // Only the first generated commit message is used
        .context("No commit messages generated")?
        .message
        .content
        .as_ref()
        .context("No commit messages generated")?;

    // Post-process the generated commit message to keep only the first line and remove leading and trailing backticks
    let regex_matches = Regex::new(r"(?m)^\s*(?:`\s*(.+?)\s*`|(.+?))\s*$")?
        .captures(&commit_message)
        .context("Failed to post-process the generated commit message")?;

    let commit_message = regex_matches
        .get(1)
        .or(regex_matches.get(2))
        .context("Failed to post-process the generated commit message")?
        .as_str()
        .to_string();

    Ok(commit_message);
}

fn send_api_request() {
    // Create an HTTP client to interact with the Inference API
    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;

    // Start spinner
    let mut spinner = Spinner::new(spinners::Dots, "Generating commit message", None);

    // Generate commit message using a LLM
    let commit_message = generate_commit_message().await;

    // Stop the spinner
    spinner.stop_with_message("");

    let commit_message = commit_message?;
}

fn main() {
    send_api_request();
}
use anyhow::{Context, Result};
use inquire::{required, CustomType, Password, PasswordDisplayMode, Text};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::{create_dir_all, read_to_string, write};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_base_url: String,  // The API base URL
    pub api_key: String,       // Private API key
    pub git_model_name: String,    // Name of LLM model to use for commit messages
    pub commit_prompt: String, // The prompt used when generating commit messages
    pub diff_prompt: String, // Used for formatting the diff that is placed after the commit prompt
    pub img_model_name: String, // Name of LLM model to use for images
    pub slides_prompt: String, // The prompt used when generating slides
    pub img_prompt: String, // The prompt used when generating images
    pub max_chars: u16,      // The max number of characters in the generated commit message
    pub request_timeout: u64, // The timeout for the API request in seconds
}

/// Asynchronously loads the configuration from the specified file path.
///
/// If the configuration file exists, it reads the content using the `read_config` function.
/// Otherwise, it creates a new configuration using the `create_config` function, writes it to the file,
/// and prints a success message. The resulting `Config` is then returned wrapped in a `Result`.
///
/// # Arguments
///
/// * `file` - A reference to the path of the configuration file.
///
/// # Returns
///
/// Returns a `Result` containing the loaded or newly created `Config` on success,
/// or an error if there were issues reading or creating the configuration.
pub async fn load_config(file: &Path) -> Result<Config> {
    let config: Config = if file.exists() {
        read_config(file).await?
    } else {
        // Create the config file
        let config = create_config().await?;
        write_config(file, &config).await?;
        println!("Successfully created config file: {:?}", file);
        config
    };
    Ok(config)
}

/// Asynchronously reads the configuration from the specified file path.
///
/// Reads the content of the configuration file at the given file path and parses it into
/// a `Config` struct using the TOML format. Returns the resulting `Config` wrapped in a `Result`.
///
/// # Arguments
///
/// * `file` - A reference to the path of the configuration file.
///
/// # Returns
///
/// Returns a `Result` containing the parsed `Config` on success,
/// or an error if there were issues reading or parsing the configuration.
async fn read_config(file: &Path) -> Result<Config> {
    let content = read_to_string(file)
        .await
        .context("Failed to read the configuration file.")?;

    let config: Config =
        toml::from_str(&content).context("Failed to parse the configuration file.")?;

    Ok(config)
}

/// Asynchronously creates a new configuration by prompting the user for required information.
///
/// Prompts the user for the API base URL, API key, model name, commit prompt, and the maximum number
/// of characters for generated commit messages. Provides default values where applicable and validates
/// user input. Returns the resulting configuration wrapped in a `Result`.
///
/// # Returns
///
/// Returns a `Result` containing the newly created `Config` on success,
/// or an error if there were issues with user input or validation.
async fn create_config() -> Result<Config> {
    let api_base_url = Text::new("Enter API base url: ")
        .with_default("https://api.together.xyz/v1/completions")
        .prompt()?;

    let api_key = Password::new("Enter your API key: ")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(required!("API key is required."))
        .without_confirmation()
        .prompt()?;

    let git_model_name = Text::new("Enter model name: ")
        .with_default("mistralai/Mixtral-8x7B-Instruct-v0.1")
        .with_validator(required!("Model name is required."))
        .with_help_message("Press Enter to use the default system prompt.")
        .prompt()?;

    let git_default_system_prompt = "You are required to write a meaningful commit message for the given code changes. The commit message must have the format: `type(scope): description`. The `type` must be one of the following: feat, fix, docs, style, refactor, perf, test, build, ci, chore, or revert. The `scope` indicates the area of the codebase that the changes affect. The `description` must be concise and written in a single sentence without a period at the end.";
    let commit_prompt = Text::new("Enter system prompt: ")
        .with_default(git_default_system_prompt)
        .with_validator(required!("System prompt is required."))
        .with_help_message("Press Enter to use the default commit prompt.")
        .prompt()?;

    let img_model_name = Text::new("Enter model name: ")
        .with_default("stabilityai/stable-diffusion-xl-base-1.0")
        .with_validator(required!("Model name is required."))
        .with_help_message("Press Enter to use the default system prompt.")
        .prompt()?;

    let img_default_system_prompt = "You are required to write a create slideshows to describe code projects. These slideshows must describe the problem that the project solves and how it solves it. It must also be clear on how the project works for non-technical people. Write at least 3 sentences per slide and include a detailed description of what the slide would show. Output this purly in json with parameters for id, script, and image.";
    let slides_prompt = Text::new("Enter system prompt: ")
        .with_default(img_default_system_prompt)
        .with_validator(required!("System prompt is required."))
        .with_help_message("Press Enter to use the default commit prompt.")
        .prompt()?;

    let max_chars = CustomType::<u16>::new(
        "Enter the max number of characters for the generated commit messages: ",
    )
    .with_default(128)
    .with_help_message("Press Enter to use the default max characters.")
    .prompt()?;

    Ok(Config {
        api_base_url: api_base_url.trim().to_string(),
        api_key: api_key.trim().to_string(),
        git_model_name: git_model_name.trim().to_string(),
        commit_prompt: commit_prompt.trim().to_string(),
        diff_prompt: "The output of the git diff command:\n```\n{}\n```".to_owned(),
        img_model_name: img_model_name.trim().to_string(),
        slides_prompt: slides_prompt.trim().to_string(),
        img_prompt: "null".to_owned(),
        max_chars,
        request_timeout: 30,
    })
}

/// Asynchronously writes the configuration to the specified file path.
///
/// Creates the configuration directory if it does not exist, serializes the provided `Config`
/// struct into a TOML-formatted string, and writes it to the specified file path. Prints a
/// success message upon completion.
///
/// # Arguments
///
/// * `file` - A reference to the path of the configuration file.
/// * `config` - A reference to the `Config` struct to be written.
///
/// # Returns
///
/// Returns a `Result` indicating success or an error if there were issues creating the directory,
/// serializing the configuration, or writing to the file.
async fn write_config(file: &Path, config: &Config) -> Result<()> {
    create_dir_all(
        file.parent()
            .context("Failed to retrieve the config directory.")?,
    )
    .await
    .expect("Failed to create config directory.");

    write(
        file,
        toml::to_string(config).context("Failed to serialize the config.")?,
    )
    .await
    .expect("Failed to write config to file.");

    println!("Successfully created config file: {:?}", file);
    Ok(())
}

use inquire::{required, CustomType, Password, PasswordDisplayMode, Text};
use std::path::Path;
use tokio::fs::{read_to_string, write};

pub mod config_loader {
    struct Config {
        api_base_url: String,
        api_key: String,
        model_name: String,
        system_prompt: String,
        user_prompt: String,
        max_chars: u16,
        request_timeout: u64,
    }

    pub async fn load_config(file: &Path) -> Result<Config> {
        let config = if file.exists() {
            read_config(file).await;
        } else {
            // Create the config file
        };
        Ok(config);
    }

    async fn read_config(file: &Path) -> Result<Config> {
        let content = read_to_string(file)
            .await
            .context("Failed to read the configuration file.");

        let config: Config =
            toml::from_str(&content).context("Failed to parse the configuration file.");

        Ok(config);
    }

    async fn create_config() -> Result<Config> {
        let api_base_url = Text::new("Enter API base url: ")
            .with_default("https://api.together.xyz/v1/")
            .prompt();

        let api_key = Password::new("Enter your API key: ")
            .with_display_toggle_enabled()
            .with_display_mode(PasswordDisplayModes::Masked)
            .with_validator(required!("API key is required."))
            .without_confirmation()
            .prompt();

        let model_name = Text::new("Enter model name: ")
            .with_default("mistralai/Mixtral-8x7B-Instruct-v0.1")
            .with_validator(required!("Model name is required."))
            .with_help_message("Press Enter to use the default system prompt.")
            .prompt();

        let default_system_prompt = "You are required to write a meaningful commit message for the given code changes. The commit message must have the format: `type(scope): description`. The `type` must be one of the following: feat, fix, docs, style, refactor, perf, test, build, ci, chore, or revert. The `scope` indicates the area of the codebase that the changes affect. The `description` must be concise and written in a single sentence without a period at the end.";
        let system_prompt = Text::new("Enter system prompt: ")
            .with_default(default_system_prompt)
            .with_validator(required!("System prompt is required."))
            .with_help_message(
                "Press Enter to use the default system prompt:\n\"{default_system_prompt}\"",
            )
            .prompt();

        let max_chars = CustomType::<u16>::new(
            "Enter the max number of characters for the generated commit messages: ",
        )
        .with_default(128)
        .with_help_message("Press Enter to use the default max characters.")
        .prompt();

        Ok(Config {
            api_base_url: api_base_url.trim().to_string(),
            api_key: api_key.trim().to_string(),
            model_name: model_name.trim().to_string(),
            system_prompt: system_prompt.trim().to_string(),
            user_prompt: "The output of the git diff command:\n```\n{}\n```",
            max_chars,
            request_timeout: 30,
        });
    }

    async fn write_config(file: &Path, config: &config) -> Result<()> {
        create_dir_all(
            file.parent()
                .context("Failed to retrieve the config directory."),
        )
        .await
        .context("Failed to create config directory.");

        write(
            file,
            toml::to_string(&config).context("Failed to serialize the config."),
        )
        .await
        .context("Failed to write config to file.");

        println!("Successfully created config file: {:?}", file);
        Ok(())
    }
}

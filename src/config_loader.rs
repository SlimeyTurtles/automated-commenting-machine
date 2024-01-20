use tokio::fs::{read_to_string, write};

pub mod config_loader {
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
}

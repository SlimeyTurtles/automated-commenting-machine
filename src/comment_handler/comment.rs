use anyhow::{ensure, Context, Result};
use tokio::process::Command;

async fn run_comment_command(args: &[&str]) -> Result<String> {
    let response = Command::new("aicomment")
        .args(args)
        .output()
        .await
        .context("Failed to execute python script.")?;

    ensure!(
        response.status.success(),
        "{}",
        String::from_utf8_lossy(&response.stderr)
    );

    String::from_utf8(response.stdout).context("Failed to decode output of Python script.")
}

pub async fn comment_file(path: &str) -> Result<String> {
    let result = run_comment_command(&[path, "--gpt4"])
        .await?
        .trim()
        .to_string();

    Ok(result)
}

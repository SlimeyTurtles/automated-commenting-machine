use anyhow::{ensure, Context, Result};
use tokio::process::Command;

async fn run_git_command(args: &[&str]) -> Result<String> {
    let response = Command::new("git")
        .args(args)
        .output()
        .await
        .context("Failed to execute Git command.")?;

    ensure!(
        response.status.success(),
        "{}",
        String::from_utf8_lossy(&response.stderr)
    );

    String::from_utf8(response.stdout).context("Failed to decode output of the Git command.")
}

async fn git_diff() -> Result<()> {
    let git_diffs = run_git_command(&[
        "--no-pager",
        "diff",
        "--staged",
        "--minimal",
        "--no-color",
        "--no-ext-diff",
        "--",
        ":!*.lock",
    ])
    .await?
    .trim()
    .to_string();

    ensure!(
        !git_diffs.is_empty(),
        "There are no staged changes to commit."
    );

    Ok(())
}

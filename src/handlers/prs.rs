use anyhow::{Context, Error};

use crate::app_config::config::{self, Config};

use crate::git_handler::read_me_gen::generate_readme_summary;
use crate::img_handler::code_summarizer::generate_slide_summary;
use dirs::home_dir;

use std::io::Write;
use reqwest::Client;
use std::path::Path;
use std::{fs, time::Duration};

/// Reads the contents of a file specified by the given directory path.
pub fn get_doc_text(dir: &str) -> Option<String> {
    match fs::read_to_string(dir) {
        Result::Ok(contents) => {
            // Successfully read the file contents

            return Some(contents);
        }
        Err(e) => {
            // Handle the error if the file cannot be read
            println!("Error reading file: {:?}", e);
            return None;
        }
    }
}

/// Recursively goes through files and returns a `Vec<String>` with file contents.
pub fn recur(dir: &str, mut arr: Vec<String>) -> Vec<std::string::String> {

    if !Path::new(dir).is_dir() {
        println!("Getting {}", dir);
        match get_doc_text(dir) {
            //return the array with the files content appended
            Some(text) => {
                arr.push(text);
                return arr;
            }
            None => {
                //Push file
                return arr;
            }
        }
    }

    if let Ok(entries) = fs::read_dir(dir) {
        //Loop through all directories in this directory
        for entry in entries.flatten() {
            // Update the arr to include all the files in the current directoy
            arr = recur(entry.path().to_str().unwrap_or("."), arr.clone());
            continue;
        }
    }

    return arr;
}

pub fn create_readme(dir: &str, content: &str) {
    let file_path = format!("{}/{}", dir, "README.md");
    let mut file = fs::File::create(file_path);
    match file {
        Ok(mut file) => {
            file.write_all(content.as_bytes());
        },
        Err(_) => {
            return;
        },
    }


} 

pub async fn execute_prs(dir: &str) -> Result<(), Error> {
    let file_text = recur(dir, Vec::new());

    let config_file = home_dir()
        .context("Failed to retrieve config directory.")?
        .join(".acm/config.toml");

    let config: Config = config::load_config(&config_file).await?;

    let http_client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()?;

    let content = generate_readme_summary(&http_client, &config, file_text).await?;
    create_readme(dir, &content);
    Ok(())
}

// #[async_recursion]
// async fn recursive_search(dir: &Path, arr: &mut Vec<String>) -> Result<(), Error> {
//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();

//             if path.is_dir() {
//                 recursive_search(&path, arr).await?;
//             } else {
//                 let file_contents = fs::read_to_string(&path)?;
//                 println!("{}", file);
//                 arr.push(file_contents);
//             }
//         }
//     }

//     let config_file = home_dir()
//         .context("Failed to retrieve config directory.")?
//         .join(".acm/config.toml");

//     let config: Config = config::load_config(&config_file).await?;

//     let http_client = Client::builder()
//         .timeout(Duration::from_secs(config.request_timeout))
//         .build()?;

//     Ok(())
// }

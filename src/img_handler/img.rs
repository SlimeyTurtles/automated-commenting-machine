// use std::ops::Add;

// use crate::config::Config;
// use anyhow::{Context, Result};
// use async_openai::types::{CreateImageRequestArgs, ImageModel, ResponseFormat::Url};
// use reqwest::Client;
// use serde::Deserialize;

// /// Stores a single commit message candidate generated by the model
// #[derive(Deserialize)]
// struct ImageCandidate {
//     image_base64: String,
// }

// /// Stores all the commit message candidates generated by the model
// #[derive(Deserialize)]
// struct ImageCandidates {
//     choices: Vec<ImageCandidate>,
// }

// pub async fn generate_image_url(
//     http_client: &Client,
//     config: &Config,
//     img: &str,
// ) -> Result<String> {

//     println!("Generating image of: {}", img);

//     let payload = CreateImageRequestArgs::default()
//         .model(ImageModel::Other(config.img_model_name.clone().into()))
//         .prompt(img)
//         .response_format(Url)
//         .build()
//         .context("Failed to construct the request payload")?;

//     let response = http_client
//         .post(format!("{}", &config.img_api_base_url))
//         .bearer_auth(&config.api_key)
//         .json(&payload)
//         .send()
//         .await
//         .context("Failed to send the request to the Inference API provider")?
//         .json::<ImageCandidates>()
//         .await
//         .context("Failed to parse the response from the API provider")?;

//     let url = &response
//         .choices
//         .first() // Only the first generated commit message is used
//         .context("No commit messages generated")?
//         .image_base64;
    
//     let str: String = "data:image/png;base64,".to_string();


//     Ok(str.add(url))
// }

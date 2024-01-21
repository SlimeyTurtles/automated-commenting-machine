use std::fs;

use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct ImgBBResponse {
    data: ImgBBData,
}

#[derive(Deserialize)]
struct ImgBBData {
    image: ImgBBImage,
}

#[derive(Deserialize)]
struct ImgBBImage {
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Replace with your actual ImgBB API key and image data
    let api_key = "33aabadd503c2467397cb8c5b1f9b42b";
    let image_data = fs::read_to_string("./foo.txt")
        .expect("Should have been able to read the file");

    println!("Image data: {}", image_data);

    // ImgBB API endpoint URL
    let endpoint = "https://api.imgbb.com/1/upload";

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Prepare the parameters
    let params = [
        ("key", api_key),
        ("image", &image_data),
        ("name", ""),
    ];

    // Make the API request
    let response = client
        .post(endpoint)
        .form(&params)
        .send()
        .await?;

    // Print the response
    println!("Status: {}", response.status());
    println!("Body: {}", response.json::<ImgBBResponse>().await?.data.image.url);

    Ok(())
}

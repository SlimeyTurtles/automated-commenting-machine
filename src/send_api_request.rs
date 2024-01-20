use reqwest::blocking::Client;

fn send_api_request(prompt: &str, max_token_count: usize, model: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    
    let response = client
        .post(url)
        .header("Authorization", "Bearer YOUR_API_KEY")
        .json(&json!({
            "prompt": prompt,
            "max_tokens": max_token_count,
            "model": model
        }))
        .send()?;
    
    let response_text = response.text()?;
    Ok(response_text)
}

fn main() {
    let prompt = "Hello, how are you?";
    let max_token_count = 100;
    let model = "gpt-3.5-turbo";
    
    match send_api_request(prompt, max_token_count, model) {
        Ok(response) => println!("API response: {}", response),
        Err(err) => eprintln!("API request failed: {}", err),
    }
}

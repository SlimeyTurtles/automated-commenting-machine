
struct Slideshow {
    id: u32,
    script: String,
    description: String
}

async fn driver(input: String) {
    
    let input_Json = json::<Vec<Slideshow>>(input);

    for slideshow in input_Json {

        let mut url = generate_image_url(slideshow.description).await?;
        let mut image = image::load_from_memory(base64::decode(&url).unwrap());
        let mut file = File::create("./imgs/{}", Slideshow.id)?;
        file.write_all(&image);

    }

}
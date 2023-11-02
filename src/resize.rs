use base64::{engine::general_purpose, Engine as _};
use image::imageops::resize;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use worker::*;

pub async fn resize_image(file: File, width: u32, height: u32) -> Result<String> {
    let bytes = file.bytes().await?;

    let img = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .map_err(|_| "Failed to decode image")?;

    let resized = resize(&img, width, height, image::imageops::FilterType::Nearest);

    let mut buf = Cursor::new(Vec::new());

    resized
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .map_err(|_| "Failed to resize image")?;
    let resized_image_data = buf.into_inner();

    Ok(general_purpose::STANDARD.encode(resized_image_data))
}

pub async fn validate_size(size: u32) -> bool {
    (16..=1024).contains(&size) && (size & (size - 1)) == 0
}

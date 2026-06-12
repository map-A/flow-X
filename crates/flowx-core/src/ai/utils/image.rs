//! Image processing utilities

#[cfg(feature = "vision")]
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

#[cfg(feature = "vision")]
pub fn compress_screenshot(
    image_data: &[u8],
    max_width: u32,
    max_height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(image_data)?;

    let (width, height) = img.dimensions();
    let scale = f32::min(
        max_width as f32 / width as f32,
        max_height as f32 / height as f32,
    )
    .min(1.0);

    let resized = if scale < 1.0 {
        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;
        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    let mut output = Cursor::new(Vec::new());
    resized.write_to(&mut output, ImageFormat::Png)?;
    Ok(output.into_inner())
}

#[cfg(not(feature = "vision"))]
pub fn compress_screenshot(
    image_data: &[u8],
    _max_width: u32,
    _max_height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(image_data.to_vec())
}

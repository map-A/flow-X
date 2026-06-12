use crate::engine::{Color, CommandError, Image, Rect};

pub struct ColorFinder;

impl ColorFinder {
    pub fn new() -> Self {
        Self
    }

    /// Find first occurrence of color in image
    pub fn find_color(
        &self,
        image: &Image,
        target: Color,
        _region: Option<Rect>,
        threshold: u8,
    ) -> Result<Option<Rect>, CommandError> {
        let pixels = &image.data;
        let width = image.width as usize;

        for y in 0..image.height as usize {
            for x in 0..width {
                let idx = (y * width + x) * 3;
                if idx + 2 < pixels.len() {
                    let r = pixels[idx];
                    let g = pixels[idx + 1];
                    let b = pixels[idx + 2];

                    let diff_r = (r as i16 - target.r as i16).abs() as u8;
                    let diff_g = (g as i16 - target.g as i16).abs() as u8;
                    let diff_b = (b as i16 - target.b as i16).abs() as u8;

                    if diff_r <= threshold && diff_g <= threshold && diff_b <= threshold {
                        return Ok(Some(Rect {
                            x: x as i32,
                            y: y as i32,
                            width: 1,
                            height: 1,
                        }));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Find all occurrences of color in image
    pub fn find_all_colors(
        &self,
        image: &Image,
        target: Color,
        _region: Option<Rect>,
        threshold: u8,
    ) -> Result<Vec<Rect>, CommandError> {
        let mut results = Vec::new();
        let pixels = &image.data;
        let width = image.width as usize;

        for y in 0..image.height as usize {
            for x in 0..width {
                let idx = (y * width + x) * 3;
                if idx + 2 < pixels.len() {
                    let r = pixels[idx];
                    let g = pixels[idx + 1];
                    let b = pixels[idx + 2];

                    let diff_r = (r as i16 - target.r as i16).abs() as u8;
                    let diff_g = (g as i16 - target.g as i16).abs() as u8;
                    let diff_b = (b as i16 - target.b as i16).abs() as u8;

                    if diff_r <= threshold && diff_g <= threshold && diff_b <= threshold {
                        results.push(Rect {
                            x: x as i32,
                            y: y as i32,
                            width: 1,
                            height: 1,
                        });
                    }
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ImageFormat;

    #[test]
    fn test_color_finder_creation() {
        let finder = ColorFinder::new();
        let image = Image {
            data: vec![255, 0, 0, 255, 0, 0],
            width: 2,
            height: 1,
            format: ImageFormat::RGB,
        };
        let color = Color { r: 255, g: 0, b: 0 };
        let result = finder.find_color(&image, color, None, 10);
        assert!(result.is_ok());
    }
}

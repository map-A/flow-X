use crate::engine::{CommandError, Image, Rect, Text};

pub struct OcrEngine;

impl OcrEngine {
    pub fn new() -> Self {
        Self
    }

    /// Perform OCR on an image
    #[cfg(feature = "ocr")]
    pub fn recognize(
        &self,
        image: &Image,
        _region: Option<Rect>,
    ) -> Result<Vec<Text>, CommandError> {
        use tesseract::Tesseract;

        let temp_path = "/tmp/flowx_ocr_temp.png";
        std::fs::write(temp_path, &image.data).map_err(|e| {
            CommandError::InvalidArgument(format!("Failed to write temp image: {}", e))
        })?;

        let result = Tesseract::new(None, Some("eng"))
            .map_err(|e| CommandError::InvalidArgument(format!("Tesseract init failed: {}", e)))?
            .set_image(temp_path)
            .map_err(|e| {
                CommandError::InvalidArgument(format!("Tesseract set image failed: {}", e))
            })?
            .get_text()
            .map_err(|e| CommandError::InvalidArgument(format!("OCR failed: {}", e)))?;

        Ok(vec![Text {
            content: result,
            bounds: Rect {
                x: 0,
                y: 0,
                width: image.width as i32,
                height: image.height as i32,
            },
            confidence: 1.0,
        }])
    }

    #[cfg(not(feature = "ocr"))]
    pub fn recognize(
        &self,
        _image: &Image,
        _region: Option<Rect>,
    ) -> Result<Vec<Text>, CommandError> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_creation() {
        let ocr = OcrEngine::new();
        let image = Image {
            data: vec![0; 100],
            width: 10,
            height: 10,
            format: crate::engine::ImageFormat::RGB,
        };
        let result = ocr.recognize(&image, None);
        assert!(result.is_ok());
    }
}

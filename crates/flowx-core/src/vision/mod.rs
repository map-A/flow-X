mod color;
/// Vision capabilities module
///
/// Provides OCR, image recognition, and color detection
mod ocr;
mod template_match;

pub use color::ColorFinder;
pub use ocr::OcrEngine;
pub use template_match::TemplateMatcher;

use crate::engine::{Color, CommandError, Image, Rect, Text};

/// Vision processor trait for image analysis
pub trait VisionProcessor: Send + Sync {
    /// Perform OCR on image region
    fn ocr(&self, image: &Image, region: Option<Rect>) -> Result<Vec<Text>, CommandError>;

    /// Find template image in source image
    fn find_image(
        &self,
        source: &Image,
        template: &Image,
        threshold: f32,
    ) -> Result<Option<Rect>, CommandError>;

    /// Find color in image region
    fn find_color(
        &self,
        image: &Image,
        color: Color,
        region: Option<Rect>,
    ) -> Result<Option<Rect>, CommandError>;
}

/// Default implementation combining all vision capabilities
pub struct DefaultVisionProcessor {
    ocr_engine: OcrEngine,
    template_matcher: TemplateMatcher,
    color_finder: ColorFinder,
}

impl DefaultVisionProcessor {
    pub fn new() -> Self {
        Self {
            ocr_engine: OcrEngine::new(),
            template_matcher: TemplateMatcher::new(),
            color_finder: ColorFinder::new(),
        }
    }
}

impl VisionProcessor for DefaultVisionProcessor {
    fn ocr(&self, image: &Image, region: Option<Rect>) -> Result<Vec<Text>, CommandError> {
        self.ocr_engine.recognize(image, region)
    }

    fn find_image(
        &self,
        source: &Image,
        template: &Image,
        threshold: f32,
    ) -> Result<Option<Rect>, CommandError> {
        self.template_matcher
            .find_template(source, template, threshold)
    }

    fn find_color(
        &self,
        image: &Image,
        color: Color,
        region: Option<Rect>,
    ) -> Result<Option<Rect>, CommandError> {
        self.color_finder.find_color(image, color, region, 10)
    }
}

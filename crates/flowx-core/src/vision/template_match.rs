use crate::engine::{CommandError, Image, Rect};

pub struct TemplateMatcher;

impl TemplateMatcher {
    pub fn new() -> Self {
        Self
    }

    /// Find template in source image
    #[cfg(feature = "vision")]
    pub fn find_template(
        &self,
        source: &Image,
        template: &Image,
        threshold: f32,
    ) -> Result<Option<Rect>, CommandError> {
        use opencv::{core::Mat, imgproc, prelude::*};

        let source_mat = image_to_mat(source)?;
        let template_mat = image_to_mat(template)?;

        let mut result = Mat::default();
        imgproc::match_template(
            &source_mat,
            &template_mat,
            &mut result,
            imgproc::TM_CCOEFF_NORMED,
            &Mat::default(),
        )
        .map_err(|e| CommandError::InvalidArgument(format!("Template match failed: {}", e)))?;

        let mut min_val = 0.0;
        let mut max_val = 0.0;
        let mut min_loc = opencv::core::Point::default();
        let mut max_loc = opencv::core::Point::default();

        opencv::core::min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &Mat::default(),
        )
        .map_err(|e| CommandError::InvalidArgument(format!("Min/max loc failed: {}", e)))?;

        if max_val >= threshold as f64 {
            let center_x = max_loc.x + template.width as i32 / 2;
            let center_y = max_loc.y + template.height as i32 / 2;
            Ok(Some(Rect {
                x: center_x,
                y: center_y,
                width: template.width as i32,
                height: template.height as i32,
            }))
        } else {
            Ok(None)
        }
    }

    #[cfg(not(feature = "vision"))]
    pub fn find_template(
        &self,
        _source: &Image,
        _template: &Image,
        _threshold: f32,
    ) -> Result<Option<Rect>, CommandError> {
        Ok(None)
    }

    /// Find all matches of template in source image
    pub fn find_all_templates(
        &self,
        _source: &Image,
        _template: &Image,
        _threshold: f32,
    ) -> Result<Vec<Rect>, CommandError> {
        Ok(Vec::new())
    }
}

#[cfg(feature = "vision")]
fn image_to_mat(image: &Image) -> Result<Mat, CommandError> {
    use opencv::core::{Mat, Vector};

    Mat::from_slice_rows_cols(&image.data, image.height as i32, image.width as i32).map_err(|e| {
        CommandError::InvalidArgument(format!("Image to Mat conversion failed: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ImageFormat;

    #[test]
    fn test_template_matcher_creation() {
        let matcher = TemplateMatcher::new();
        let image = Image {
            data: vec![0; 100],
            width: 10,
            height: 10,
            format: ImageFormat::RGB,
        };
        let result = matcher.find_template(&image, &image, 0.9);
        assert!(result.is_ok());
    }
}

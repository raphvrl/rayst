use crate::error::Result;
use image::{Rgb, RgbImage};

pub struct ImageRenderer {}

impl ImageRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, width: u32, height: u32) -> Result<RgbImage> {
        let mut img = RgbImage::new(width, height);

        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let radius = (width.min(height) as f32 * 0.3).min(100.0);

        for x in 0..width {
            for y in 0..height {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    img.put_pixel(x, y, Rgb([255, 0, 0])); // Rouge
                } else {
                    img.put_pixel(x, y, Rgb([0, 0, 0])); // Noir
                }
            }
        }

        Ok(img)
    }
}

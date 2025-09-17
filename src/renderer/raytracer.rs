use crate::Result;
use crate::scene::{Camera, Scene};
use image::{Rgb, RgbImage};

pub struct Raytracer {
    pub camera: Camera,
}

impl Raytracer {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn render(&self, scene: &Scene, width: u32, height: u32) -> Result<RgbImage> {
        let mut img = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let ray = self.camera.generate_ray(x, y, width, height);

                let color = if let Some(hit) = scene.hit(&ray) {
                    let intensity = 1.0 / (1.0 + hit.distance * hit.distance * 0.01);
                    let intensity = intensity.clamp(0.1, 1.0);

                    let r = (hit.color.0 as f32 * intensity) as u8;
                    let g = (hit.color.1 as f32 * intensity) as u8;
                    let b = (hit.color.2 as f32 * intensity) as u8;
                    (r, g, b)
                } else {
                    scene.background_color
                };

                img.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
            }
        }

        Ok(img)
    }
}

use crate::Result;
use crate::math::Intersection;
use crate::math::Ray;
use crate::scene::{Camera, Scene};
use glam::Vec3;
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
                    self.calculate_lighting(scene, &hit)
                } else {
                    scene.background_color
                };

                img.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
            }
        }

        Ok(img)
    }

    fn calculate_lighting(&self, scene: &Scene, hit: &Intersection) -> (u8, u8, u8) {
        let object_color = hit.color;

        let ambient_r = object_color.0 as f32 * scene.ambient_light;
        let ambient_g = object_color.1 as f32 * scene.ambient_light;
        let ambient_b = object_color.2 as f32 * scene.ambient_light;

        let mut diffuse_r = 0.0;
        let mut diffuse_g = 0.0;
        let mut diffuse_b = 0.0;

        for light in &scene.lights {
            let light_direction = light.direction_to(hit.point);

            if !self.is_in_shadow(scene, hit.point, hit.normal, light) {
                let lambert = hit.normal.dot(light_direction).max(0.0);

                let light_intensity = light.intensity_at(hit.point);

                let contribution = lambert * light_intensity;

                diffuse_r += object_color.0 as f32 * contribution * (light.color.0 as f32 / 255.0);
                diffuse_g += object_color.1 as f32 * contribution * (light.color.1 as f32 / 255.0);
                diffuse_b += object_color.2 as f32 * contribution * (light.color.2 as f32 / 255.0);
            }
        }

        let final_r = (ambient_r + diffuse_r).clamp(0.0, 255.0) as u8;
        let final_g = (ambient_g + diffuse_g).clamp(0.0, 255.0) as u8;
        let final_b = (ambient_b + diffuse_b).clamp(0.0, 255.0) as u8;

        (final_r, final_g, final_b)
    }

    fn is_in_shadow(
        &self,
        scene: &Scene,
        point: Vec3,
        normal: Vec3,
        light: &crate::lighting::PointLight,
    ) -> bool {
        let light_direction = (light.position - point).normalize();

        let light_distance = (light.position - point).length();

        let shadow_ray = Ray::new(point + normal * 0.001, light_direction);

        if let Some(hit) = scene.hit(&shadow_ray) {
            hit.distance < light_distance - 0.001
        } else {
            false
        }
    }
}

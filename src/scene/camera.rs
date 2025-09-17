use crate::math::Ray;
use glam::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, fov: f32) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov,
        }
    }

    pub fn generate_ray(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        offset_x: f32,
        offset_y: f32,
    ) -> Ray {
        let aspect_ratio = width as f32 / height as f32;
        let fov_scale = (self.fov.to_radians() / 2.0).tan();

        let u = (x as f32 + offset_x) / width as f32 * 2.0 - 1.0;
        let v = 1.0 - (y as f32 + offset_y) / height as f32 * 2.0;

        let u = u * aspect_ratio * fov_scale;
        let v = v * fov_scale;

        let right = self.direction.cross(self.up).normalize();
        let up = right.cross(self.direction).normalize();

        let direction = (self.direction + u * right + v * up).normalize();

        Ray::new(self.position, direction)
    }
}

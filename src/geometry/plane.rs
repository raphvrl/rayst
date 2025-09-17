use crate::geometry::Primitive;
use crate::math::{Intersection, Ray};
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: (u8, u8, u8),
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, color: (u8, u8, u8)) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            color,
        }
    }
}

impl Primitive for Plane {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        let denominator = ray.direction.dot(self.normal);

        if denominator.abs() < 1e-6 {
            return None;
        }

        let t = (self.point - ray.origin).dot(self.normal) / denominator;

        if t > 0.001 {
            let point = ray.origin + t * ray.direction;
            Some(Intersection::new(t, point, self.normal, self.color))
        } else {
            None
        }
    }
}

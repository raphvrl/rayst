use crate::geometry::Primitive;
use crate::materials::Material;
use crate::math::Intersection;
use crate::math::Ray;
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Primitive for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        let t = if t1 > 0.001 {
            t1
        } else if t2 > 0.001 {
            t2
        } else {
            return None;
        };

        let point = ray.origin + t * ray.direction;
        let normal = (point - self.center).normalize();

        Some(Intersection::new(t, point, normal, self.material.clone()))
    }
}

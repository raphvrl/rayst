use crate::geometry::Primitive;
use crate::materials::Material;
use crate::math::{Intersection, Ray};
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub material: Material,
    normal: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2).normalize();

        Self {
            v0,
            v1,
            v2,
            material,
            normal,
        }
    }
}

impl Primitive for Triangle {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        const EPSILON: f32 = 1e-8;

        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > EPSILON {
            let point = ray.origin + t * ray.direction;
            Some(Intersection::new(
                t,
                point,
                self.normal,
                self.material.clone(),
            ))
        } else {
            None
        }
    }
}

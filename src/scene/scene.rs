use crate::geometry::Primitive;
use crate::geometry::Sphere;

pub struct Scene {
    pub objects: Vec<Box<dyn Primitive>>,
    pub background_color: (u8, u8, u8),
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            background_color: (0, 0, 0),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.objects.push(Box::new(sphere));
    }

    pub fn hit(&self, ray: &crate::math::Ray) -> Option<crate::math::Intersection> {
        let mut closest_hit: Option<crate::math::Intersection> = None;
        let mut closest_distance = f32::INFINITY;

        for sphere in &self.objects {
            if let Some(hit) = sphere.hit(ray) {
                if hit.distance < closest_distance && hit.distance > 0.001 {
                    closest_distance = hit.distance;
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }
}

use crate::geometry::Primitive;
use crate::lighting::PointLight;

pub struct Scene {
    pub objects: Vec<Box<dyn Primitive>>,
    pub lights: Vec<PointLight>,
    pub background_color: (u8, u8, u8),
    pub ambient_light: f32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            background_color: (0, 0, 0),
            ambient_light: 0.1,
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Primitive>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
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

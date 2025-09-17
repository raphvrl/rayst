use crate::materials::Material;
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersection {
    pub fn new(distance: f32, point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            distance,
            point,
            normal,
            material,
        }
    }
}

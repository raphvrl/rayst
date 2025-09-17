use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub color: (u8, u8, u8),
}

impl Intersection {
    pub fn new(distance: f32, point: Vec3, normal: Vec3, color: (u8, u8, u8)) -> Self {
        Self {
            distance,
            point,
            normal,
            color,
        }
    }
}

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }

    pub fn direction_to(&self, point: Vec3) -> Vec3 {
        (self.position - point).normalize()
    }

    pub fn intensity_at(&self, point: Vec3) -> f32 {
        let distance = (self.position - point).length();
        self.intensity / (1.0 + distance * distance * 0.01)
    }
}

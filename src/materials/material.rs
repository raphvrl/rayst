use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Material {
    pub albedo: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub ao: f32,

    pub emission: Vec3,
    pub transparency: f32,
    pub ior: f32,
}

impl Material {
    pub fn new(albedo: Vec3, metallic: f32, roughness: f32) -> Self {
        Self {
            albedo,
            metallic: metallic.clamp(0.0, 1.0),
            roughness: roughness.clamp(0.0, 1.0),
            ao: 1.0,
            emission: Vec3::ZERO,
            transparency: 0.0,
            ior: 1.0,
        }
    }

    pub fn plastic(color: Vec3) -> Self {
        Self::new(color, 0.0, 0.8)
    }

    pub fn metal(color: Vec3, roughness: f32) -> Self {
        Self::new(color, 1.0, roughness)
    }

    pub fn gold() -> Self {
        Self::new(Vec3::new(1.0, 0.84, 0.0), 1.0, 0.1)
    }

    pub fn silver() -> Self {
        Self::new(Vec3::new(0.95, 0.95, 0.95), 1.0, 0.05)
    }

    pub fn copper() -> Self {
        Self::new(Vec3::new(0.95, 0.64, 0.54), 1.0, 0.2)
    }

    pub fn glass(color: Vec3, transparency: f32) -> Self {
        Self {
            albedo: color,
            metallic: 0.0,
            roughness: 0.0,
            ao: 1.0,
            emission: Vec3::ZERO,
            transparency,
            ior: 1.5,
        }
    }

    pub fn get_f0(&self) -> Vec3 {
        if self.metallic > 0.5 {
            self.albedo
        } else {
            Vec3::splat(0.04)
        }
    }

    pub fn to_rgb_u8(&self) -> (u8, u8, u8) {
        (
            (self.albedo.x * 255.0) as u8,
            (self.albedo.y * 255.0) as u8,
            (self.albedo.z * 255.0) as u8,
        )
    }

    pub fn distribution_ggx(n_dot_h: f32, roughness: f32) -> f32 {
        let a = roughness * roughness;
        let a2 = a * a;
        let n_dot_h2 = n_dot_h * n_dot_h;
        
        let num = a2;
        let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
        let denom = std::f32::consts::PI * denom * denom;
        
        num / denom
    }

    pub fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
        let r = roughness + 1.0;
        let k = (r * r) / 8.0;
        
        let num = n_dot_v;
        let denom = n_dot_v * (1.0 - k) + k;
        
        num / denom
    }

    pub fn geometry_smith(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32 {
        let ggx2 = Self::geometry_schlick_ggx(n_dot_v, roughness);
        let ggx1 = Self::geometry_schlick_ggx(n_dot_l, roughness);
        
        ggx1 * ggx2
    }

    pub fn fresnel_schlick(cos_theta: f32, f0: Vec3) -> Vec3 {
        f0 + (Vec3::ONE - f0) * (1.0 - cos_theta).powf(5.0)
    }
}

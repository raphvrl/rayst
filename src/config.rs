use crate::error::{RaystError, Result};
use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub render: RenderConfig,
    pub objects: Vec<ObjectConfig>,
    pub lights: Vec<LightConfig>,
    pub background: BackgroundConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraConfig {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub fov: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub max_depth: u32,
    pub antialiasing: u32,
    pub output_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "object_type")]
pub enum ObjectConfig {
    #[serde(rename = "sphere")]
    Sphere {
        position: [f32; 3],
        radius: f32,
        material: MaterialConfig,
    },
    #[serde(rename = "plane")]
    Plane {
        position: [f32; 3],
        normal: [f32; 3],
        material: MaterialConfig,
    },
    #[serde(rename = "cube")]
    Cube {
        position: [f32; 3],
        size: f32,
        rotation: Option<[f32; 3]>,
        material: MaterialConfig,
    },
    #[serde(rename = "pyramid")]
    Pyramid {
        position: [f32; 3],
        base_size: f32,
        height: f32,
        rotation: Option<[f32; 3]>,
        material: MaterialConfig,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaterialConfig {
    pub material_type: String,
    pub albedo: [f32; 3],
    pub metallic: Option<f32>,
    pub roughness: Option<f32>,
    pub emission: Option<[f32; 3]>,
    pub transparency: Option<f32>,
    pub ior: Option<f32>,
    pub ao: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LightConfig {
    pub position: [f32; 3],
    pub color: [u8; 3],
    pub intensity: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BackgroundConfig {
    pub color: [u8; 3],
}

impl SceneConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: SceneConfig = toml::from_str(&content)
            .map_err(|e| RaystError::InvalidInput(format!("Failed to load scene config: {}", e)))?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| RaystError::InvalidInput(format!("Failed to save scene config: {}", e)))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn to_camera(&self) -> crate::scene::Camera {
        crate::scene::Camera::new(
            Vec3::from_array(self.camera.position),
            Vec3::from_array(self.camera.direction),
            self.camera.fov,
        )
    }

    pub fn to_material(&self, config: &MaterialConfig) -> crate::materials::Material {
        match config.material_type.as_str() {
            "plastic" => crate::materials::Material::plastic(Vec3::from_array(config.albedo)),
            "metal" => crate::materials::Material::metal(
                Vec3::from_array(config.albedo),
                config.roughness.unwrap_or(0.1),
            ),
            "gold" => crate::materials::Material::gold(),
            "silver" => crate::materials::Material::silver(),
            "copper" => crate::materials::Material::copper(),
            "glass" => crate::materials::Material::glass(
                Vec3::from_array(config.albedo),
                config.transparency.unwrap_or(0.0),
            ),
            _ => crate::materials::Material::new(
                Vec3::from_array(config.albedo),
                config.metallic.unwrap_or(0.0),
                config.roughness.unwrap_or(0.5),
            ),
        }
    }
}

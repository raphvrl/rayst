pub mod config;
pub mod error;
pub mod geometry;
pub mod lighting;
pub mod materials;
pub mod math;
pub mod renderer;
pub mod scene;

pub use config::*;
pub use error::{RaystError, Result};
pub use geometry::{Cube, Plane, Pyramid, Sphere};
pub use lighting::PointLight;
pub use materials::Material;
pub use math::{Intersection, Ray};
pub use renderer::Raytracer;
pub use scene::{Camera, Scene};

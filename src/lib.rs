pub mod error;
pub mod geometry;
pub mod math;
pub mod renderer;
pub mod scene;

pub use error::{RaystError, Result};
pub use geometry::Sphere;
pub use math::{Intersection, Ray};
pub use renderer::Raytracer;
pub use scene::{Camera, Scene};

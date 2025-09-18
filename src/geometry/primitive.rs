use crate::math::{Intersection, Ray};

pub trait Primitive: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<Intersection>;
}

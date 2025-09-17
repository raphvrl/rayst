use crate::math::{Intersection, Ray};

pub trait Primitive {
    fn hit(&self, ray: &Ray) -> Option<Intersection>;
}

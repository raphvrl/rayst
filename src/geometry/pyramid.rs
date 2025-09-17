use crate::geometry::{Primitive, Triangle};
use crate::materials::Material;
use crate::math::{Intersection, Ray};
use glam::{Mat3, Vec3};

pub struct Pyramid {
    pub base_center: Vec3,
    pub base_size: f32,
    pub height: f32,
    pub rotation: Vec3,
    pub material: Material,
    triangles: Vec<Triangle>,
}

impl Pyramid {
    pub fn new(
        base_center: Vec3,
        base_size: f32,
        height: f32,
        rotation: Vec3,
        material: Material,
    ) -> Self {
        let half = base_size / 2.0;

        let base_points = [
            Vec3::new(-half, 0.0, -half),
            Vec3::new(half, 0.0, -half),
            Vec3::new(half, 0.0, half),
            Vec3::new(-half, 0.0, half),
        ];

        let apex = Vec3::new(0.0, height, 0.0);

        let rotation_matrix = Self::create_rotation_matrix(rotation);

        let rotated_base: Vec<Vec3> = base_points
            .iter()
            .map(|&point| base_center + rotation_matrix * point)
            .collect();

        let rotated_apex = base_center + rotation_matrix * apex;

        let mut triangles = Vec::new();

        triangles.push(Triangle::new(
            rotated_base[0],
            rotated_base[2],
            rotated_base[1],
            material.clone(),
        ));
        triangles.push(Triangle::new(
            rotated_base[0],
            rotated_base[3],
            rotated_base[2],
            material.clone(),
        ));

        for i in 0..4 {
            let next_i = (i + 1) % 4;
            triangles.push(Triangle::new(
                rotated_base[i],
                rotated_base[next_i],
                rotated_apex,
                material.clone(),
            ));
        }

        Self {
            base_center,
            base_size,
            height,
            rotation,
            material,
            triangles,
        }
    }

    fn create_rotation_matrix(rotation: Vec3) -> Mat3 {
        let rot_rad = rotation * std::f32::consts::PI / 180.0;

        let rot_x = Mat3::from_rotation_x(rot_rad.x);
        let rot_y = Mat3::from_rotation_y(rot_rad.y);
        let rot_z = Mat3::from_rotation_z(rot_rad.z);

        rot_z * rot_y * rot_x
    }
}

impl Primitive for Pyramid {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_hit: Option<Intersection> = None;
        let mut closest_distance = f32::INFINITY;

        for triangle in &self.triangles {
            if let Some(hit) = triangle.hit(ray) {
                if hit.distance < closest_distance && hit.distance > 0.001 {
                    closest_distance = hit.distance;
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }
}

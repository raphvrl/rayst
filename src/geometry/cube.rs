use crate::geometry::{Plane, Primitive};
use crate::math::{Intersection, Ray};
use glam::{Mat3, Vec3};

pub struct Cube {
    pub center: Vec3,
    pub rotation: Vec3,
    pub size: f32,
    pub color: (u8, u8, u8),
    rotation_matrix: Mat3,
    inverse_rotation_matrix: Mat3,
}

impl Cube {
    pub fn new(center: Vec3, rotation: Vec3, size: f32, color: (u8, u8, u8)) -> Self {
        let rot_rad = rotation * std::f32::consts::PI / 180.0;

        let rot_x = Mat3::from_rotation_x(rot_rad.x);
        let rot_y = Mat3::from_rotation_y(rot_rad.y);
        let rot_z = Mat3::from_rotation_z(rot_rad.z);

        let rotation_matrix = rot_z * rot_y * rot_x;
        let inverse_rotation_matrix = rotation_matrix.transpose();

        Self {
            center,
            rotation,
            size,
            color,
            rotation_matrix,
            inverse_rotation_matrix,
        }
    }

    fn transform_ray_to_local(&self, ray: &Ray) -> Ray {
        let local_origin = ray.origin - self.center;

        let rotated_origin = self.inverse_rotation_matrix * local_origin;
        let rotated_direction = self.inverse_rotation_matrix * ray.direction;

        Ray::new(rotated_origin, rotated_direction)
    }

    fn transform_point_to_world(&self, local_point: Vec3) -> Vec3 {
        self.center + self.rotation_matrix * local_point
    }

    fn transform_normal_to_world(&self, local_normal: Vec3) -> Vec3 {
        (self.rotation_matrix * local_normal).normalize()
    }

    fn calculate_local_normal(&self, local_point: Vec3) -> Vec3 {
        let abs_point = local_point.abs();
        let max_component = abs_point.x.max(abs_point.y).max(abs_point.z);

        if (abs_point.x - max_component).abs() < 1e-6 {
            Vec3::new(local_point.x.signum(), 0.0, 0.0)
        } else if (abs_point.y - max_component).abs() < 1e-6 {
            Vec3::new(0.0, local_point.y.signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, local_point.z.signum())
        }
    }
}

impl Primitive for Cube {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        let local_ray = self.transform_ray_to_local(ray);

        let half = self.size / 2.0;
        let min = Vec3::new(-half, -half, -half);
        let max = Vec3::new(half, half, half);

        let inv_dir = Vec3::new(
            1.0 / local_ray.direction.x,
            1.0 / local_ray.direction.y,
            1.0 / local_ray.direction.z,
        );

        let t1 = (min.x - local_ray.origin.x) * inv_dir.x;
        let t2 = (max.x - local_ray.origin.x) * inv_dir.x;
        let t3 = (min.y - local_ray.origin.y) * inv_dir.y;
        let t4 = (max.y - local_ray.origin.y) * inv_dir.y;
        let t5 = (min.z - local_ray.origin.z) * inv_dir.z;
        let t6 = (max.z - local_ray.origin.z) * inv_dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return None;
        }

        let t = if tmin > 0.001 {
            tmin
        } else if tmax > 0.001 {
            tmax
        } else {
            return None;
        };

        let local_point = local_ray.origin + t * local_ray.direction;

        let local_normal = self.calculate_local_normal(local_point);

        let world_point = self.transform_point_to_world(local_point);
        let world_normal = self.transform_normal_to_world(local_normal);

        let world_distance = (world_point - ray.origin).length();

        Some(Intersection::new(
            world_distance,
            world_point,
            world_normal,
            self.color,
        ))
    }
}

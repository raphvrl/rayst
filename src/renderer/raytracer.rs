use crate::Result;
use crate::lighting::PointLight;
use crate::materials::Material;
use crate::math::Intersection;
use crate::math::Ray;
use crate::scene::{Camera, Scene};
use glam::Vec3;
use image::{Rgb, RgbImage};

pub struct Raytracer {
    pub camera: Camera,
    pub max_depth: u32,
}

impl Raytracer {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            max_depth: 10,
        }
    }

    fn trace_ray(&self, scene: &Scene, ray: &Ray, depth: u32) -> Vec3 {
        if depth >= self.max_depth {
            return Vec3::ZERO;
        }

        if let Some(hit) = scene.hit(ray) {
            let view_dir = -ray.direction;
            let material = &hit.material;

            let direct_lighting = self.calculate_pbr_lighting(scene, &hit, view_dir);

            if material.metallic > 0.0 || (material.roughness < 0.3 && !material.albedo.is_nan()) {
                let reflected_color = self.calculate_reflection(scene, &hit, view_dir, depth);
                let reflection_strength = material.metallic.max(1.0 - material.roughness);

                return direct_lighting * (1.0 - reflection_strength * 0.5)
                    + reflected_color * reflection_strength * 0.5;
            }
            direct_lighting
        } else {
            Vec3::new(
                scene.background_color.0 as f32 / 255.0,
                scene.background_color.1 as f32 / 255.0,
                scene.background_color.2 as f32 / 255.0,
            )
        }
    }

    fn calculate_reflection(
        &self,
        scene: &Scene,
        hit: &Intersection,
        view_dir: Vec3,
        depth: u32,
    ) -> Vec3 {
        let reflect_dir = self.reflect(-view_dir, hit.normal).normalize();
        let reflect_ray = Ray::new(hit.point + hit.normal * 0.001, reflect_dir);

        let reflected_color = self.trace_ray(scene, &reflect_ray, depth + 1);

        if hit.material.metallic > 0.5 {
            reflected_color * hit.material.albedo
        } else {
            reflected_color
        }
    }

    fn reflect(&self, incident: Vec3, normal: Vec3) -> Vec3 {
        incident - 2.0 * incident.dot(normal) * normal
    }

    fn calculate_pbr_lighting(&self, scene: &Scene, hit: &Intersection, view_dir: Vec3) -> Vec3 {
        let material = &hit.material;
        let mut lo = Vec3::ZERO;

        let ambient = Vec3::splat(0.03) * material.albedo * material.ao;

        for light in &scene.lights {
            if !self.is_in_shadow(scene, hit.point, hit.normal, light) {
                let light_pos = light.position;
                let light_dir = (light_pos - hit.point).normalize();
                let light_color = Vec3::new(
                    light.color.0 as f32 / 255.0,
                    light.color.1 as f32 / 255.0,
                    light.color.2 as f32 / 255.0,
                );

                let distance = (light_pos - hit.point).length();
                let attenuation = 1.0 / (distance * distance);
                let radiance = light_color * light.intensity * attenuation;

                let n = hit.normal;
                let v = view_dir;
                let l = light_dir;
                let h = (v + l).normalize();

                let n_dot_v = n.dot(v).max(0.0);
                let n_dot_l = n.dot(l).max(0.0);
                let n_dot_h = n.dot(h).max(0.0);
                let v_dot_h = v.dot(h).max(0.0);

                let f0 = material.get_f0();
                let ndf = Material::distribution_ggx(n_dot_h, material.roughness);
                let g = Material::geometry_smith(n_dot_v, n_dot_l, material.roughness);
                let f = Material::fresnel_schlick(v_dot_h, f0);

                let numerator = ndf * g * f;
                let denominator = 4.0 * n_dot_v * n_dot_l + 0.0001;
                let specular = numerator / denominator;

                let ks = f;
                let mut kd = Vec3::ONE - ks;
                kd *= 1.0 - material.metallic;

                let diffuse = material.albedo / std::f32::consts::PI;

                lo += (kd * diffuse + specular) * radiance * n_dot_l;
            }
        }

        let color = ambient + lo + material.emission;
        let mapped = color / (color + Vec3::ONE);
        mapped.powf(1.0 / 2.2)
    }

    fn is_in_shadow(&self, scene: &Scene, point: Vec3, normal: Vec3, light: &PointLight) -> bool {
        let light_direction = (light.position - point).normalize();
        let light_distance = (light.position - point).length();
        let shadow_ray_origin = point + normal * 0.001;
        let shadow_ray = Ray::new(shadow_ray_origin, light_direction);

        if let Some(hit) = scene.hit(&shadow_ray) {
            if hit.distance < light_distance - 0.001 {
                return hit.material.transparency < 0.5;
            }
        }
        false
    }

    fn vec3_to_rgb(&self, color: Vec3) -> (u8, u8, u8) {
        (
            (color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (color.z.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }

    pub fn render(&self, scene: &Scene, width: u32, height: u32) -> Result<RgbImage> {
        let mut img = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let ray = self.camera.generate_ray(x, y, width, height);

                let color_vec3 = self.trace_ray(scene, &ray, 0);
                let color = self.vec3_to_rgb(color_vec3);

                img.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
            }
        }

        Ok(img)
    }
}

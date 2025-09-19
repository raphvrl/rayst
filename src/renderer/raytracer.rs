use crate::Result;
use crate::lighting::PointLight;
use crate::materials::Material;
use crate::math::Intersection;
use crate::math::Ray;
use crate::scene::{Camera, Scene};
use fastrand;
use glam::Vec3;
use image::{Rgb, RgbImage};
use rayon::prelude::*;

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
            scene.background_color
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
            let shadow_factor = self.calculate_shadow_factor(scene, hit.point, hit.normal, light);

            if shadow_factor > 0.0 {
                let light_pos = light.position;
                let light_dir = (light_pos - hit.point).normalize();
                let light_color = light.color;

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

                lo += (kd * diffuse + specular) * radiance * n_dot_l * shadow_factor;
            }
        }

        let color = ambient + lo + material.emission;
        let mapped = color / (color + Vec3::ONE);
        mapped.powf(1.0 / 2.2)
    }

    fn calculate_shadow_factor(
        &self,
        scene: &Scene,
        point: Vec3,
        normal: Vec3,
        light: &PointLight,
    ) -> f32 {
        let light_pos = light.position;
        let light_direction = (light_pos - point).normalize();

        let light_radius = 0.5;
        let samples = 8;

        let mut shadow_factor = 0.0;
        let shadow_ray_origin = point + normal * 0.001;

        let up = if light_direction.y.abs() < 0.9 {
            Vec3::Y
        } else {
            Vec3::X
        };

        let right = light_direction.cross(up).normalize();
        let forward = right.cross(light_direction).normalize();

        for i in 0..samples {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (samples as f32);
            let radius = light_radius * fastrand::f32().sqrt();

            let offset = right * (radius * angle.cos()) + forward * (radius * angle.sin());
            let sample_light_pos = light_pos + offset;

            let sample_light_direction = (sample_light_pos - point).normalize();
            let sample_light_distance = (sample_light_pos - point).length();

            let shadow_ray = Ray::new(shadow_ray_origin, sample_light_direction);

            let mut occluded = false;
            if let Some(hit) = scene.hit(&shadow_ray) {
                if hit.distance < sample_light_distance - 0.001 {
                    occluded = true;
                }
            }

            if !occluded {
                shadow_factor += 1.0;
            }
        }

        shadow_factor / samples as f32
    }

    fn vec3_to_rgb(&self, color: Vec3) -> (u8, u8, u8) {
        (
            (color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (color.z.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }

    pub fn render(&self, scene: &Scene, width: u32, height: u32, samples: u32) -> Result<RgbImage> {
        let mut img = RgbImage::new(width, height);

        const CHUNK_SIZE: u32 = 64;
        let chunks: Vec<Vec<(u32, u32)>> = (0..height)
            .step_by(CHUNK_SIZE as usize)
            .map(|start_y| {
                let end_y = (start_y + CHUNK_SIZE).min(height);
                (start_y..end_y)
                    .flat_map(|y| (0..width).map(move |x| (x, y)))
                    .collect()
            })
            .collect();

        let chunk_results: Vec<Vec<((u32, u32), (u8, u8, u8))>> = chunks
            .par_iter()
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&(x, y)| {
                        let mut color_sum = Vec3::ZERO;

                        for _ in 0..samples {
                            let offset_x = fastrand::f32();
                            let offset_y = fastrand::f32();

                            let ray = self
                                .camera
                                .generate_ray(x, y, width, height, offset_x, offset_y);
                            let color = self.trace_ray(scene, &ray, 0);
                            color_sum += color;
                        }

                        let final_color = color_sum / samples as f32;
                        let color = self.vec3_to_rgb(final_color);

                        ((x, y), color)
                    })
                    .collect()
            })
            .collect();

        for chunk_result in chunk_results {
            for ((x, y), color) in chunk_result {
                img.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
            }
        }

        Ok(img)
    }
}

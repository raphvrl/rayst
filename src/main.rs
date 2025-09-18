use glam::Vec3;
use rayst::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene_file = args
        .get(1)
        .unwrap_or(&"scenes/example.toml".to_string())
        .clone();

    match run(&scene_file) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(config_path: &str) -> Result<()> {
    let config = SceneConfig::load_from_file(&config_path)?;

    println!("Loading scene from {}", config_path);

    let mut scene = Scene::new(&config);

    for obj_config in &config.objects {
        let object: Box<dyn crate::geometry::Primitive> = match obj_config {
            ObjectConfig::Sphere {
                position,
                radius,
                material,
            } => {
                let mat = config.to_material(material);
                Box::new(Sphere::new(Vec3::from_array(*position), *radius, mat))
            }
            ObjectConfig::Plane {
                position,
                normal,
                material,
            } => {
                let mat = config.to_material(material);
                Box::new(Plane::new(
                    Vec3::from_array(*position),
                    Vec3::from_array(*normal),
                    mat,
                ))
            }
            ObjectConfig::Cube {
                position,
                size,
                rotation,
                material,
            } => {
                let mat = config.to_material(material);
                let rot = Vec3::from_array(rotation.unwrap_or([0.0, 0.0, 0.0]));
                Box::new(Cube::new(Vec3::from_array(*position), rot, *size, mat))
            }
            ObjectConfig::Pyramid {
                position,
                base_size,
                height,
                rotation,
                material,
            } => {
                let mat = config.to_material(material);
                let rot = Vec3::from_array(rotation.unwrap_or([0.0, 0.0, 0.0]));
                Box::new(Pyramid::new(
                    Vec3::from_array(*position),
                    *base_size,
                    *height,
                    rot,
                    mat,
                ))
            }
        };
        scene.add_object(object);
    }

    for light_config in &config.lights {
        scene.add_light(PointLight::new(
            Vec3::from_array(light_config.position),
            Vec3::from_array(light_config.color.map(|c| c as f32)),
            light_config.intensity,
        ));
    }

    let camera = config.to_camera();

    let raytracer = Raytracer::new(camera);

    let img = raytracer.render(
        &scene,
        config.render.width,
        config.render.height,
        config.render.antialiasing,
    );

    img.unwrap().save(&config.render.output_file)?;
    println!("Saved Rendered to: {}", config.render.output_file);

    Ok(())
}

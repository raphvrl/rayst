use glam::Vec3;
use rayst::*;

fn main() {
    match run() {
        Ok(()) => println!("Render completed successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let mut scene = Scene::new();

    scene.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -5.0),
        1.0,
        (255, 0, 0),
    )));
    scene.add_object(Box::new(Pyramid::new(
        Vec3::new(2.0, 2.0, -8.0),
        1.0,
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
        (0, 0, 255),
    )));
    scene.add_object(Box::new(Cube::new(
        Vec3::new(-3.0, 1.0, -7.0),
        Vec3::new(45.0, 45.0, 45.0),
        1.0,
        (0, 255, 0),
    )));
    scene.add_object(Box::new(Plane::new(
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        (255, 255, 255),
    )));

    scene.add_light(PointLight::new(
        Vec3::new(0.0, 10.0, 0.0),
        (255, 255, 255),
        1.0,
    ));

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 60.0);

    let raytracer = Raytracer::new(camera);

    let img = raytracer.render(&scene, 800, 600);

    img.unwrap().save("scene.png")?;

    Ok(())
}

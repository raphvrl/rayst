use glam::Vec3;
use rayst::*;

fn main() {
    match run() {
        Ok(()) => println!("✅ Terminé!"),
        Err(e) => {
            eprintln!("❌ Erreur: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let mut scene = Scene::new();

    scene.add_sphere(Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, (255, 0, 0)));
    scene.add_sphere(Sphere::new(Vec3::new(2.0, -2.0, -8.0), 0.8, (0, 0, 255)));
    scene.add_sphere(Sphere::new(Vec3::new(-3.5, 1.0, -10.0), 0.6, (0, 255, 0)));

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 60.0);

    let raytracer = Raytracer::new(camera);

    let img = raytracer.render(&scene, 800, 600);

    img.unwrap().save("scene.png")?;

    Ok(())
}

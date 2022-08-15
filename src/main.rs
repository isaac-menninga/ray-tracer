use camera::Camera;
use materials::{lambertian::Lambertian, metal::Metal};
use sphere::Sphere;
use std::{env, sync::Arc};
use vector::Vector;

extern crate indicatif;
extern crate lodepng;
extern crate rand;

mod camera;
mod material;
mod materials;
mod ray;
mod scene;
mod sphere;
mod utils;
mod vector;

static ASPECT_RATIO: f64 = 16.0 / 9.0;
static VIEWPORT_WIDTH: i32 = 1600;
static ANTIALIAS_SAMPLES: i32 = 300;
static REFLECTION_DEPTH: i32 = 100;
static BACKGROUND_COLOR: Vector = Vector(0.5, 0.7, 1.0);

fn main() {
    // camera
    let lookfrom = Vector(16.0, 1.6, 3.0);
    let lookat = Vector(0.0, 0.0, 0.0);
    let vup = Vector(0.0, 1.0, 0.0);
    let dist_to_focus = 15.0;
    let aperture = 0.08;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut objects: Vec<Sphere> = Vec::new();

    // basic materials
    let ground_material = Arc::new(Lambertian::new(Vector(0.7, 0.72, 0.62)));

    let red_lambertian: Arc<Lambertian> = Arc::new(Lambertian::new(Vector(0.6, 0.2, 0.2)));
    let blue_lambertian: Arc<Lambertian> = Arc::new(Lambertian::new(Vector(0.2, 0.6, 0.2)));
    let green_lambertian: Arc<Lambertian> = Arc::new(Lambertian::new(Vector(0.2, 0.2, 0.6)));

    let metal: Arc<Metal> = Arc::new(Metal::new(Vector(0.6, 0.6, 0.65)));

    // setup scene objects
    // diffuse material spheres
    objects.push(Sphere::new(&Vector(0.0, -0.7, 0.4), 0.3, red_lambertian));
    objects.push(Sphere::new(&Vector(0.7, -0.7, 0.0), 0.3, blue_lambertian));
    objects.push(Sphere::new(&Vector(-0.7, -0.7, 0.8), 0.3, green_lambertian));

    // metal sphere
    objects.push(Sphere::new(&Vector(-3.0, 0.0, 0.0), 1.0, metal));

    // ground
    objects.push(Sphere::new(
        &Vector(0.0, -1001.0, 0.0),
        1000.0,
        ground_material,
    ));

    // get filename if present
    let mut filename = "out/out.png".to_string();
    if let Some(arg1) = env::args().nth(1) {
        let f = format!("out/{}.png", arg1);
        filename = f;
    }

    let scene: scene::Scene = scene::Scene::new(cam, objects, filename);

    scene.render();
}

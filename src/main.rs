use std::sync::Arc;

use materials::{lambertian::Lambertian, metal::Metal};

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

static ASPECT_RATIO: f32 = 16.0 / 9.0;
static VIEWPORT_WIDTH: i32 = 800;
static FOCAL_LENGTH: f32 = 100.0;

static ANTIALIAS_SAMPLES: i32 = 100;
static REFLECTION_DEPTH: i32 = 50;

static BACKGROUND_COLOR: vector::Vector = vector::Vector(0.5, 0.7, 1.0);

fn main() {
    let c: camera::Camera = camera::Camera::new(vector::Vector(0.0, 0.0, 0.0), VIEWPORT_WIDTH);

    let mut objects: Vec<sphere::Sphere> = Vec::new();

    let ground_material = Arc::new(materials::lambertian::Lambertian::new(vector::Vector(
        0.9, 0.92, 0.92,
    )));

    let basic_lamb_material: Arc<Lambertian> = Arc::new(materials::lambertian::Lambertian::new(
        vector::Vector(0.6, 0.2, 0.2),
    ));

    let basic_metal_material: Arc<Metal> = Arc::new(Metal::new(vector::Vector(0.8, 0.8, 0.8)));

    objects.push(sphere::Sphere::new(
        &vector::Vector(0.0, 0.0, -2.0),
        1.0,
        basic_lamb_material,
    ));

    objects.push(sphere::Sphere::new(
        &vector::Vector(3.0, 0.0, -2.0),
        1.0,
        basic_metal_material,
    ));

    objects.push(sphere::Sphere::new(
        &vector::Vector(0.0, -1001.0, 0.0),
        1000.0,
        ground_material,
    ));

    let scene: scene::Scene = scene::Scene::new(c, objects);

    scene.render();
}

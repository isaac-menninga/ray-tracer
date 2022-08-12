extern crate lodepng;
extern crate rand;

mod camera;
mod material;
mod ray;
mod scene;
mod sphere;
mod vector;

static ASPECT_RATIO: f32 = 16.0 / 9.0;
static VIEWPORT_WIDTH: i32 = 800;
static FOCAL_LENGTH: f32 = 100.0;

static ANTIALIAS_SAMPLES: i32 = 100;
static REFLECTION_DEPTH: i32 = 50;

static BACKGROUND_COLOR: vector::Vector = vector::Vector(0.5, 0.7, 1.0);

fn main() {
    let c = camera::Camera::new(vector::Vector(0.0, 0.0, 0.0), VIEWPORT_WIDTH);

    let mut objects: Vec<sphere::Sphere> = Vec::new();

    objects.push(sphere::Sphere::new(
        &vector::Vector(0.0, 0.0, -2.0),
        1.0,
        &vector::Vector(0.1, 0.1, 0.4),
        &vector::Vector(0.1, 0.0, 0.0),
        &vector::Vector(1.0, 1.0, 1.0),
        100.0,
        1.0,
    ));

    objects.push(sphere::Sphere::new(
        &vector::Vector(0.0, -1001.0, 0.0),
        1000.0,
        &vector::Vector(0.1, 0.1, 0.4),
        &vector::Vector(0.1, 0.0, 0.0),
        &vector::Vector(1.0, 1.0, 1.0),
        100.0,
        1.0,
    ));

    let scene = scene::Scene::new(c, objects);

    scene.render();
}

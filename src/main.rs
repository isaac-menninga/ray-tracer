extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod sphere;
mod scene;

static ASPECT_RATIO: f32 = 16.0 / 9.0;
static VIEWPORT_WIDTH: i32 = 400;

static REFLECTION_DEPTH: usize = 20;
static BACKGROUND_COLOR: vector::Vector = vector::Vector(0.5, 0.7, 1.0);
static FOCAL_LENGTH: f32 = 50.0;

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

    let scene = scene::Scene::new(
        c,
        objects,
    );

    scene.render();
}

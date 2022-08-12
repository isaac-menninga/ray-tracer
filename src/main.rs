extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;
mod sphere;
mod scene;

static WIDTH: usize = 1600;
static HEIGHT: usize = 1600;

static REFLECTION_DEPTH: usize = 4;
static BACKGROUND_COLOR: vector::Vector = vector::Vector(0.08, 0.82, 0.08);

static LIGHT_POS: vector::Vector = vector::Vector(20.0, -20.0, -10.0);
static LIGHT_COLOR: vector::Vector = vector::Vector(1.0, 1.0, 1.0);
static LIGHT_POWER: f32 = 200.0;

fn random_position() -> vector::Vector {
    vector::Vector(
        (rand::random::<f32>() * 9.0) - 4.5,
        (rand::random::<f32>() * 9.0) - 4.5, 
        (rand::random::<f32>() * 8.0) + 3.0
    )
}

fn main() {
    let c = camera::Camera::new(0.0, 0.0, -1.0);

    let mut objects: Vec<sphere::Sphere> = Vec::new();

    objects.push(sphere::Sphere::new(
        &vector::Vector(-3.0, 0.0, 15.0), 
        1.0, 
        &vector::Vector(0.4, 0.2, 0.1), 
        &vector::Vector(0.1, 0.0, 0.0), 
        &vector::Vector(1.0, 1.0, 1.0), 
        32.0, 
        0.95,
    ));

    // TODO: actually make the multiple lights work :(
    let mut lights = Vec::new();
    lights.push(LIGHT_POS);

    let scene = scene::Scene::new(
        c,
        objects,
        HEIGHT,
        WIDTH,
        &lights,
    );

    scene.render();
}

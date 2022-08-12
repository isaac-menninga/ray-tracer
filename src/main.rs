extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;
mod sphere;
mod scene;

static WIDTH: usize = 400;
static HEIGHT: usize = 400;

static REFLECTION_DEPTH: usize = 20;
static BACKGROUND_COLOR: vector::Vector = vector::Vector(0.5, 0.7, 1.0);

fn main() {
    let c = camera::Camera::new(0.0, 0.0, -15.00);

    let mut objects: Vec<sphere::Sphere> = Vec::new();

    objects.push(sphere::Sphere::new(
        &vector::Vector(5.0, 1.0, 6.0), 
        2.0, 
        &vector::Vector(0.1, 0.1, 0.4), 
        &vector::Vector(0.1, 0.0, 0.0), 
        &vector::Vector(1.0, 1.0, 1.0), 
        100.0, 
        1.0,
    ));

    objects.push(sphere::Sphere::new(
        &vector::Vector(0.0, 1005.0, 2.0), 
        1000.0, 
        &vector::Vector(0.4, 0.42, 0.45), 
        &vector::Vector(0.1, 0.0, 0.0), 
        &vector::Vector(1.0, 1.0, 1.0), 
        60.0, 
        1.0,
    ));
    
    objects.push(sphere::Sphere::new(
        &vector::Vector(-3.0, 0.0, 8.0), 
        1.3, 
        &vector::Vector(0.1, 0.4, 0.1), 
        &vector::Vector(0.0, 0.1, 0.0), 
        &vector::Vector(1.0, 1.0, 1.0), 
        10.0, 
        1.0,
    ));

    let scene = scene::Scene::new(
        c,
        objects,
        HEIGHT,
        WIDTH,
    );

    scene.render();
}

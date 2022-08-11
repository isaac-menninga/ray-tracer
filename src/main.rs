extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;
mod sphere;
mod scene;

fn random_position() -> vector::Vector {
    vector::Vector(
        (rand::random::<f32>() * 9.0) - 4.5,
        (rand::random::<f32>() * 9.0) - 4.5, 
        (rand::random::<f32>() * 8.0) + 3.0
    )
}

fn random_sphere(p: &vector::Vector, r: f32) -> sphere::Sphere {
    let diffuse = vector::Vector(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>());
    let ambient = (rand::random::<f32>() / 3.0) * diffuse;
    let specular = vector::Vector(1.0, 1.0, 1.0);
    let shine = rand::random::<f32>() * 100.0;
    let reflectiveness = rand::random::<f32>() * 0.95;

    return sphere::Sphere::new(p, r, &ambient, &diffuse, &specular, shine, reflectiveness);
}

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    const N_SPHERES: i32 = 5;
    const RADIUS: f32 = 1.0;
    const AMBIENT: &vector::Vector = &vector::Vector(1.0, 0.7, 0.7);
    const DIFFUSE: &vector::Vector = &vector::Vector(1.0, 0.7, 0.7);
    const SPECULAR: &vector::Vector = &vector::Vector(1.0, 0.7, 0.7);
    const LIGHT_POS: vector::Vector = vector::Vector(40.0, -80.0, -100.0);

    let c = camera::Camera::new(0.0, 0.0, -1.0);

    let mut objects: Vec<sphere::Sphere> = Vec::new();

    for _x in 0 .. N_SPHERES {
        objects.push(random_sphere(&random_position(), RADIUS));
    }

    // TODO: actually make the multiple lights work :(
    let mut lights = Vec::new();
    lights.push(LIGHT_POS);

    let scene = scene::Scene::new(
        c,
        objects,
        HEIGHT,
        WIDTH,
        &lights,
        AMBIENT,
        DIFFUSE,
        SPECULAR
    );

    scene.render();
}

extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;
mod sphere;
mod scene;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;

    let c = camera::Camera::new(0.0, 0.0, -1.0);

    let objects: [sphere::Sphere; 4] = [
        sphere::Sphere::new(
            &vector::Vector(-0.5, -0.5, 8.0),
            1.0,
            &vector::Vector(0.1, 0.0, 0.0),
            &vector::Vector(0.7, 0.0, 0.0),
            &vector::Vector(1.0, 1.0, 1.0),
            100.0
        ),
        sphere::Sphere::new(
            &vector::Vector(0.5, 0.5, 4.0),
            0.5,
            &vector::Vector(0.0, 0.2, 0.0),
            &vector::Vector(0.0, 0.5, 0.0),
            &vector::Vector(1.0, 1.0, 1.0),
            75.0
        ),
        sphere::Sphere::new(
            &vector::Vector(0.8, 0.8, 2.0),
            0.5,
            &vector::Vector(0.1, 0.1, 0.0),
            &vector::Vector(0.5, 0.7, 0.0),
            &vector::Vector(1.0, 1.0, 1.0),
            40.0
        ),
        sphere::Sphere::new(
            &vector::Vector(0.0, 9000.0, 0.0),
            8998.0,
            &vector::Vector(0.1, 0.1, 0.1),
            &vector::Vector(0.5, 0.5, 0.5),
            &vector::Vector(1.0, 1.0, 1.0),
            70.0
        ),
    ];

    let scene = scene::Scene::new(
        c,
        objects,
        HEIGHT,
        WIDTH,
        3.0,
        -15.0,
        0.0,
        &vector::Vector(1.0, 1.0, 1.0),
        &vector::Vector(1.0, 1.0, 1.0),
        &vector::Vector(1.0, 1.0, 1.0),
    );

    scene.render();
}

extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;
mod sphere;
mod scene;

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    const _NSAMPLES: usize = 100;

    let c = camera::Camera::new(0.0, 0.0, -3.0);

    let objects: [sphere::Sphere; 1] = [
        sphere::Sphere::new(0.0, 0.0, 10.0, 1.0)
    ];

    let scene = scene::Scene::new(c, objects, HEIGHT, WIDTH, 5.0, 5.0, 5.0);
    scene.render();
}

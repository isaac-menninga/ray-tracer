extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod camera;
mod pixel;

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    const _NSAMPLES: usize = 100;

    let c = camera::Camera::new(0.0, 0.0, 0.0, HEIGHT, WIDTH, 1.0);
    c.make_png("out.png".to_string());
}

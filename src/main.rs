extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod viewport;
mod camera;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;
    const _NSAMPLES: usize = 100;

    let v: viewport::Viewport = viewport::Viewport::new(HEIGHT, WIDTH);
    v.make_png("out.png".to_string());
}

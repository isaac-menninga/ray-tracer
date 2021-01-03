extern crate rand;
extern crate lodepng;

mod vector;
mod ray;
mod viewport;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;
    const _NSAMPLES: usize = 100;

    let mut v: viewport::Viewport = viewport::Viewport::new(HEIGHT, WIDTH);
    v.get_gradient();
    v.make_png("out.png".to_string());
}

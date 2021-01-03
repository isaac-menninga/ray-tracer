extern crate rand;
extern crate lodepng;

mod vector;

use lodepng::RGB;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;
    const _NSAMPLES: usize = 100;

    let mut pixels: Vec<RGB<u8>> = Vec::new();

    for y in 0 .. HEIGHT {
        let _j = HEIGHT - 1 - y;
        for i in 0 .. WIDTH {
            let col = vector::Vector(i as f32 / WIDTH as f32, y as f32 / HEIGHT as f32, 0.1);
            let rgb = col.to_u8();
            pixels.push(RGB { r: rgb[0], g: rgb[1], b: rgb[2] });
        }
    }

    let filename = "out.png";
    match lodepng::encode24_file(filename, &pixels, WIDTH, HEIGHT) {
        Ok(()) => {}
        Err(err) => println!("Error writing file \"{}\": {}", filename, err)
    }

}

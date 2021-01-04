use crate::vector::Vector;
use crate::ray::Ray;
use crate::pixel::Pixel;
use lodepng::RGB;

pub struct Camera {
    pub position: Vector,
    pub viewport_distance: f32,
    pub height: usize, 
    pub width: usize,
    pub pixels: Vec<Vec<Pixel>>
}

impl Camera {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        h: usize,
        w: usize,
        d: f32,
    ) -> Self {        
        let mut pixels: Vec<Vec<Pixel>> = Vec::new();

        for i in 0 .. h {
            let mut pixel_row: Vec<Pixel> = Vec::new();

            for j in 0 .. w {
                pixel_row.push(Pixel::new(i as f32, j as f32))
            }
            pixels.push(pixel_row)
        }

        Self {
            position: Vector(x, y, z),
            height: h, 
            width: w, 
            viewport_distance: d,
            pixels: pixels
        }
    }

    pub fn get_ray(&self, pixel_x: usize, pixel_y: usize) -> Ray {
        let pixel = self.pixels[pixel_x][pixel_y].pos;

        Ray::new(
            self.position, 
            (pixel - self.position).to_unit_vector()
        )
    }

    pub fn trace_rays(&self) {
        for i in 0 .. self.height {
            for j in 0 .. self.width {
                let ray = self.get_ray(i, j);
            }
        }
    }

    pub fn make_png(&self, fname: String) -> bool {
        let filename = fname.clone();
        let mut file_data: Vec<RGB<u8>> = Vec::new();

        for i in 0 .. self.height {
            for j in 0 .. self.width {
                file_data.push(self.pixels[i as usize][j as usize].color)
            }
        }

        match lodepng::encode24_file(fname, &file_data, self.width, self.height) {
            Ok(()) => true,
            Err(err) => {
                println!("Error writing file \"{}\": {}", filename, err);
                false
            }
        }
    }
}

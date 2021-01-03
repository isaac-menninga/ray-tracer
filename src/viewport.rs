use lodepng::RGB;
use crate::vector::Vector;

#[derive(Clone, Debug)]
pub struct Viewport {
    pub height: usize, 
    pub width: usize,
    pub pixels: Vec<RGB<u8>>
}

impl Viewport {
    pub fn new(h: usize, w: usize) -> Self {
        Self { 
            height: h,
            width: w,
            pixels: Vec::new() 
        }
    }

    pub fn get_gradient(&mut self) {
        self.pixels = Vec::new();

        for y in 0 .. self.height {
            let _j = self.height - 1 - y;
            let h = y as f32;

            for i in 0 .. self.width {
                let g = i as f32;
                
                let col = Vector(
                    g / self.width as f32,
                    h / self.height as f32,
                    (g + h) / (self.width + self.height) as f32
                );
                
                let rgb = col.to_u8();
                self.pixels.push(RGB { r: rgb[0], g: rgb[1], b: rgb[2] });
            }
        }
    }

    pub fn make_png(&self, fname: String) -> bool {
        let filename = fname.clone();

        match lodepng::encode24_file(fname, &self.pixels, self.width, self.height) {
            Ok(()) => true,
            Err(err) => {
                println!("Error writing file \"{}\": {}", filename, err);
                false
            }
        }
    }
}
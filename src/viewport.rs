use lodepng::RGB;

#[derive(Clone, Debug)]
pub struct Viewport {
    pub height: usize, 
    pub width: usize,
    pub pixels: Vec<RGB<u8>>
}

impl Viewport {
    pub fn new(h: usize, w: usize) -> Self {
        let mut pixels = Vec::new();

        for _y in 0 .. h {
            for _i in 0 .. w {
                pixels.push(RGB { r: 0, g: 0, b: 0 });
            }
        }

        Self { 
            height: h,
            width: w,
            pixels: pixels
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
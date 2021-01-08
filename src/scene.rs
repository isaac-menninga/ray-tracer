use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::pixel::Pixel;
use lodepng::RGB;

pub struct Scene {
    camera: Camera,
    objects: [Sphere; 1],
    pub height: usize, 
    pub width: usize,
    pub pixels: Vec<Vec<Pixel>>,
    pub render_pixels: Vec<RGB<u8>>
}

impl Scene {
    pub fn new(c: Camera, o: [Sphere; 1], h: usize, w: usize) -> Self {
        let mut pixels: Vec<Vec<Pixel>> = Vec::new();
        let y_size = (h as f32) / 2.0;
        let x_size = (w as f32) / 2.0;

        for i in 0 .. h {
            let mut pixel_row: Vec<Pixel> = Vec::new();

            for j in 0 .. w {
                pixel_row.push(Pixel::new((j as f32 - x_size) / w as f32, (i as f32 - y_size) / h as f32))
            }
            pixels.push(pixel_row)
        }

        Self {
            camera: c,
            objects: o,
            height: h,
            width: w,
            pixels: pixels,
            render_pixels: Vec::new()
        }
    }

    pub fn render(mut self) {
        for row in &self.pixels {
            for pixel in row {
                let ray = self.camera.get_ray(pixel.pos);
                let mut min = None;

                for object in &self.objects {
                    if let Some(hit) = object.ray_intersect(ray) {
                        match min {
                            None => min = Some(hit),
                            Some(prev) => if hit.t < prev.t {
                                min = Some(hit)
                            }
                        }
                    }
                }
                match min {
                    Some(_p) => {
                        self.render_pixels.push(RGB { r: 100, g: 100, b: 255 });
                    }
                    None => {
                        self.render_pixels.push(RGB { r: 0, g: 0, b: 0 })
                    }
                }
            }
        }

        self.make_png("out.png".to_string());
    }

    pub fn make_png(&self, fname: String) -> bool {
        let filename = fname.clone();
        let mut file_data: Vec<RGB<u8>> = Vec::new();

        for i in 0 .. self.height {
            for j in 0 .. self.width {
                file_data.push(self.pixels[i as usize][j as usize].color)
            }
        }

        match lodepng::encode24_file(fname, &self.render_pixels, self.width, self.height) {
            Ok(()) => true,
            Err(err) => {
                println!("Error writing file \"{}\": {}", filename, err);
                false
            }
        }
    }
}
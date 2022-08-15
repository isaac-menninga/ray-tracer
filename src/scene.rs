use indicatif::ProgressStyle;
use rand::Rng;

use crate::camera::Camera;
use crate::indicatif::ProgressBar;
use crate::ray::*;
use crate::sphere::Hit;
use crate::sphere::Sphere;
use crate::vector::Vector;

pub struct Scene {
    camera: Camera,
    objects: Vec<Sphere>,
    pub height: i32,
    pub width: i32,
    pub pixels: Vec<lodepng::RGB<u8>>,
}

impl Scene {
    pub fn new(c: Camera, o: Vec<Sphere>) -> Self {
        let pixels: Vec<lodepng::RGB<u8>> = Vec::new();
        let h = (crate::VIEWPORT_WIDTH as f64 / crate::ASPECT_RATIO) as i32;
        let w = crate::VIEWPORT_WIDTH;

        Self {
            camera: c,
            objects: o,
            height: h,
            width: w,
            pixels: pixels,
        }
    }

    pub fn render(mut self) {
        let progress = ProgressBar::new(self.height as u64);
        progress.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        for i in (0..self.height).rev() {
            progress.inc(1);
            for j in 0..self.width {
                let color = self.antialias_color(crate::ANTIALIAS_SAMPLES, j, i);

                self.pixels.push(color.to_rgb());
            }
        }
        self.make_png("out.png".to_string());
        progress.finish();
        println!("Render complete.");
    }

    pub fn check_hits(&self, ray: &Ray) -> Option<Hit> {
        let mut min = None;

        for object in &self.objects {
            if let Some(hit) = object.ray_intersect(ray) {
                match min {
                    None => min = Some(hit),
                    Some(prev) => {
                        if hit.t < prev.t {
                            min = Some(hit);
                        } else {
                            min = Some(prev);
                        }
                    }
                }
            }
        }

        return min;
    }

    pub fn color_model(&self, r: Ray, depth: i32) -> Vector {
        let obj_hit = self.check_hits(&r);
        let color: Vector;

        match obj_hit {
            Some(h) => {
                if crate::REFLECTION_DEPTH > depth {
                    if let Some((scattered, attenuation)) = h.material.scatter(&r, &h) {
                        color = attenuation * self.color_model(scattered, depth - 1)
                    } else {
                        color = Vector(0.0, 0.0, 0.0)
                    }
                } else {
                    color = Vector(0.0, 0.0, 0.0);
                }

                // color with normals
                // color = 0.5 * (h.normal + Vector(1.0, 1.0, 1.0));

                return color;
            }
            None => {
                let unit_direction = r.direction.to_unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                let color = (1.0 - t) * Vector(1.0, 1.0, 1.0) + t * crate::BACKGROUND_COLOR;

                return color;
            }
        }
    }

    pub fn antialias_color(&self, n_samples: i32, pixel_x: i32, pixel_y: i32) -> Vector {
        let mut aa_color = Vector(0.0, 0.0, 0.0);
        for _ in 0..n_samples {
            let mut rng = rand::thread_rng();
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();

            let x = (pixel_x as f64 + random_u) / ((self.width - 1) as f64);
            let y = (pixel_y as f64 + random_v) / ((self.height - 1) as f64);
            let (origin, direction) = self.camera.get_pixel_direction(x, y);
            let ray = get_ray(origin, direction);
            let c = self.color_model(ray, 0);

            aa_color = aa_color + (1.0 / n_samples as f64) * c;
        }
        return aa_color;
    }

    pub fn make_png(&self, fname: String) -> bool {
        let filename = fname.clone();

        match lodepng::encode24_file(
            fname,
            &self.pixels,
            self.width as usize,
            self.height as usize,
        ) {
            Ok(()) => true,
            Err(err) => {
                println!("Error writing file \"{}\": {}", filename, err);
                false
            }
        }
    }
}

use crate::camera::Camera;
use crate::material::Scatter;
use crate::ray::*;
use crate::sphere::Hit;
use crate::sphere::Sphere;
use crate::utils::*;
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
        let h = (crate::VIEWPORT_WIDTH as f32 / crate::ASPECT_RATIO) as i32;
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
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                let cam = self.camera;
                let u = i as f32 / (self.height - 1) as f32;
                let v = j as f32 / (self.width - 1) as f32;
                let origin = cam.position;
                let direction =
                    cam.lower_left_corner + v * cam.horizontal + u * cam.vertical - origin;
                let color = self.antialias_color(crate::ANTIALIAS_SAMPLES, 1.0, direction, origin);

                self.pixels.push(color.to_rgb());
            }
        }
        self.make_png("out.png".to_string());
    }

    pub fn check_hits(&self, ray: &Ray) -> Option<Hit> {
        let mut min = None;

        for object in &self.objects {
            if let Some(hit) = object.ray_intersect(ray) {
                match min {
                    None => min = Some(hit),
                    Some(prev) => {
                        if hit.t < prev.t {
                            min = Some(hit)
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
                    let d = h.p + h.normal + Self::random_vector_in_unit_sphere();
                    color = 0.5 * self.color_model(get_ray(h.p, d - h.p), depth + 1);
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

    pub fn antialias_color(
        &self,
        n_samples: i32,
        offset_amount: f32,
        direction: Vector,
        origin: Vector,
    ) -> Vector {
        let mut aa_color = Vector(0.0, 0.0, 0.0);
        for _ in 0..n_samples {
            let offset_direction = offset_amount
                * Vector(
                    Self::random_in_range(-1.0, 1.0),
                    Self::random_in_range(-1.0, 1.0),
                    0.0,
                )
                + direction;
            let ray = get_ray(origin, offset_direction);
            let c = self.color_model(ray, 0);

            aa_color = aa_color + (1.0 / n_samples as f32) * c;
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

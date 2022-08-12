use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::sphere::Hit;
use crate::pixel::Pixel;
use crate::vector::Vector;
use crate::ray::*;

use lodepng::RGB;

pub struct Scene {
    camera: Camera,
    objects: Vec<Sphere>,
    pub height: usize, 
    pub width: usize,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Scene {
    pub fn new(
        c: Camera, 
        o: Vec<Sphere>, 
        h: usize, 
        w: usize, 
    ) -> Self {
        let mut pixels: Vec<Vec<Pixel>> = Vec::new();
        let y_size = (h as f32) / 2.0;
        let x_size = (w as f32) / 2.0;

        for i in 0 .. h {
            let mut pixel_row: Vec<Pixel> = Vec::new();

            for j in 0 .. w {
                pixel_row.push(Pixel::new((j as f32 - x_size) / w as f32, (i as f32 - y_size) / h as f32, c.position.z() + 1.0))
            }
            pixels.push(pixel_row)
        }

        Self {
            camera: c,
            objects: o,
            height: h,
            width: w,
            pixels: pixels,
        }
    }

    pub fn render(mut self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                let pixel = &self.pixels[y][x];

                let direction = pixel.pos;
                let origin = self.camera.position;

                let color = self.antialias_color(40, 0.001, direction, origin);

                let u = color.to_u8();

                self.pixels[y][x].color = Some(RGB { 
                    r: u[0] as u8, 
                    g: u[1] as u8, 
                    b: u[2] as u8 
                });
            }
        }
        self.make_png("out.png".to_string());
    }

    pub fn trace_ray(&self, hit: &Hit, light: Vector) -> Option<Hit> {
        let intersection = hit.p;

        let light_ray = Ray::new(intersection, light.to_unit_vector());
        let light_hit = self.check_hits(&light_ray);

        let obj_to_light = (light - intersection).to_unit_vector();

        match light_hit {
            // no intersection between origin and light
            // return color based on origin material
            None => {
                return None;
            }
            // some object is hit
            Some(k) => {
                // if object is further away than the light source
                // then we assume it's the same as a no hit (above)
                if obj_to_light.length() < k.p.length() {                            
                    return None;
                // if the object is closer than the light, than the point is completely shadowed.
                } else {
                    return Some(k);
                }
            }
        }
    }

    pub fn check_hits(&self, ray: &Ray) -> Option<Hit> {
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

        return min
    }

    pub fn random_in_range(min: f32, max: f32) -> f32 {
        return rand::random::<f32>() * max + min;
    }

    pub fn random_vector_in_unit_sphere() -> Vector {
        let vec = Vector(
            Self::random_in_range(-1.0, 1.0),
            Self::random_in_range(-1.0, 1.0),
            Self::random_in_range(-1.0, 1.0),
        );

        if vec.length() <= 1.0 {
            return vec.to_unit_vector();
        } else {
            return Self::random_vector_in_unit_sphere();
        }
    
    }

    pub fn color_model(&self, r: Ray, depth: usize) -> Vector {
        let obj_hit = self.check_hits(&r);
        let mut color = Vector(0.0, 0.0, 0.0);

        match obj_hit {
            Some(h) => {
                if crate::REFLECTION_DEPTH > depth {
                    let d = h.p + h.normal + Self::random_vector_in_unit_sphere();
                    color = 0.5 * self.color_model(get_ray(h.p, d - h.p), depth + 1);
                } else {
                    color = Vector(0.0, 0.0, 0.0);
                }
                return color;
            }
            None => {
                let unit_direction = r.direction.to_unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                let color =  (1.0 - t) * Vector(1.0, 1.0, 1.0) + t * crate::BACKGROUND_COLOR;

                return color;
            }
        }
    }

    pub fn antialias_color(&self, n_samples: u8, offset_amount: f32, direction: Vector, origin: Vector) -> Vector {
        let mut aa_color = Vector(0.0, 0.0, 0.0);
        for _ in 0 .. n_samples {
            let offset_direction = offset_amount * Vector(Self::random_in_range(-1.0, 1.0), Self::random_in_range(-1.0, 1.0), 0.0) + direction;
            let ray = get_ray(origin, offset_direction);
            let c = self.color_model(ray, 0);

            aa_color = aa_color + (1.0 / n_samples as f32) * c;
        }
        return aa_color;
    }

    pub fn make_png(&self, fname: String) -> bool {
        let filename = fname.clone();
        let mut render_pixels = Vec::new();

        for y in 0 .. self.height {
            for x in 0 .. self.width {
                match self.pixels[y][x].color {
                    Some(c) => {
                        render_pixels.push(c);
                    }
                    None => {
                        render_pixels.push(RGB { r: 0, g: 0, b: 0 });
                    }
                }
            }
        }

        match lodepng::encode24_file(fname, &render_pixels, self.width, self.height) {
            Ok(()) => true,
            Err(err) => {
                println!("Error writing file \"{}\": {}", filename, err);
                false
            }
        }
    }
}
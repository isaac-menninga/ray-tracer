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
    pub lights: Vec<Vector>,
}

impl Scene {
    pub fn new(
        c: Camera, 
        o: Vec<Sphere>, 
        h: usize, 
        w: usize, 
        lights: &Vec<Vector>,
    ) -> Self {
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
            lights: lights.to_vec(),
        }
    }

    pub fn render(mut self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                let pixel = &self.pixels[y][x];
                let mut color = None;
                let mut direction = pixel.pos;
                let mut origin = self.camera.position;

                let l = self.lights[0].clone();

                let mut reflection = 1.0;
                let mut n_reflections = 0;
                let mut last_hit;

                while n_reflections < crate::REFLECTION_DEPTH {
                    let ray = get_ray(origin, direction);
                    let initial_hit = self.check_hits(&ray);

                    let sampled_color = match initial_hit {
                        None => {
                            last_hit = None;
                            crate::BACKGROUND_COLOR
                        }
                        Some(p) => {
                            // whether object hits something in the way of the light
                            let light_hit = self.trace_ray(&p, l);

                            match light_hit {
                                // no shadow
                                None => {
                                    last_hit = Some(p);
                                    let s = self.reflection_model(p, l);
                                    s
                                }
                                // object casting shadow
                                Some(p) => {
                                    last_hit = None;
                                    crate::BACKGROUND_COLOR
                                }
                            }
                        }
                    };
                    match last_hit {
                        Some(k) => {
                            match color {
                                // if color already exists, add reflection to existing color
                                Some(c) => {
                                    color = Some(c + (reflection * sampled_color));
                                }
                                // if no color exists, it's the sampled color
                                None => {
                                    color = Some(sampled_color);
                                }
                            }
                            n_reflections += 1;
                            reflection = reflection * k.material.reflectiveness;
                            origin = k.p;
                            direction = self.reflected_vector(&k);
                        }
                        // if the last hit wasn't an object
                        None => {
                            match color {
                                Some(c) => {
                                    color = Some(c);
                                }
                                None => {
                                    color = Some(sampled_color);
                                }
                            }
                            break;
                        }
                    }
                }

                match color {
                    Some(c) => {
                        // println!("{} {} {}", c.0, c.1, c.2);

                        self.pixels[y][x].color = Some(RGB { 
                            r: c.0 as u8, 
                            g: c.1 as u8, 
                            b: c.2 as u8 
                        });
                    }
                    None => {
                        self.pixels[y][x].color = Some(RGB { 
                            r: crate::BACKGROUND_COLOR.0 as u8,
                            g: crate::BACKGROUND_COLOR.1 as u8,
                            b: crate::BACKGROUND_COLOR.2 as u8 
                        });
                    }
                }
            }
        }
        self.make_png("out.png".to_string());
    }

    pub fn reflected_vector(&self, hit: &Hit) -> Vector {
        let v = hit.p;
        let a = hit.normal;

        v - ((2.0 * v.dot(a)) * a)
    }

    pub fn trace_ray(&self, hit: &Hit, light: Vector) -> Option<Hit> {
        let intersection = hit.p;
        let object_normal = hit.normal;

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

    pub fn reflection_model(&self, p: Hit, light: Vector) -> Vector {
        // p.normal;
        let mut obj_to_light = light - p.p;
        let mut distance = obj_to_light.length();
        distance = distance * distance;
        obj_to_light = obj_to_light.to_unit_vector();

        let mut lambertian = 0.0;
        let mut specular = 0.0;

        if obj_to_light.dot(p.normal) > 0.0 {
            lambertian = obj_to_light.dot(p.normal);

            let view_dir = -p.p.to_unit_vector();

            let half_dir = (obj_to_light + view_dir).to_unit_vector();
            let specular_angle = half_dir.dot(p.normal); 
            specular = specular_angle.powf(p.material.shine);
        }

        let mut color = p.material.ambient + crate::LIGHT_POWER * lambertian * crate::LIGHT_COLOR * p.material.diffuse / distance;
        color = color + crate::LIGHT_POWER * specular * crate::LIGHT_COLOR / distance;

        return color;
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
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
    pub ambient: Vector,
    pub diffuse: Vector, 
    pub specular: Vector,
}

const NSAMPLES: usize = 5;
const REFLECTION_DEPTH: usize = 10;
const OFFSET_AMOUNT: f32 = 0.03;
const BACKGROUND_COLOR: Vector = Vector(0.1, 0.1, 0.1);
const LIGHT_RADIUS: f32 = 0.3;
const LIGHT_SAMPLES: usize = 3;

impl Scene {
    pub fn new(
        c: Camera, 
        o: Vec<Sphere>, 
        h: usize, 
        w: usize, 
        lights: &Vec<Vector>,
        ambient: &Vector,
        diffuse: &Vector,
        specular: &Vector,
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
            ambient: Vector(ambient.x(), ambient.y(), ambient.z()),
            diffuse: Vector(diffuse.x(), diffuse.y(), diffuse.z()),
            specular: Vector(specular.x(), specular.y(), specular.z()),
        }
    }

    pub fn render(mut self) {
        for f in 0 .. NSAMPLES {
            println!("{}/100", (f as f32 / NSAMPLES as f32) * 100.0);

            for y in 0 .. self.height {
                for x in 0 .. self.width {
                    let pixel = &self.pixels[y][x];
                    let mut color = None;
                    let mut direction = pixel.pos;
                    let mut origin = self.camera.get_random_vector();

                    for light in &self.lights {
                        let l = light.clone();

                        for _ in 0 .. LIGHT_SAMPLES {
                            let mut reflection = 1.0;
                            let mut n_reflections = 0;
                            let mut last_hit;

                            let p = Vector(
                                rand::random::<f32>(),
                                rand::random::<f32>(),
                                rand::random::<f32>()
                            );
                            let light_point = (LIGHT_RADIUS * p.to_unit_vector());
    
                            while n_reflections < REFLECTION_DEPTH {
                                let ray = get_ray(origin, direction);
                                let initial_hit = self.check_hits(&ray);
            
                                let sampled_color = match initial_hit {
                                    None => {
                                        last_hit = None;
                                        BACKGROUND_COLOR
                                    }
                                    Some(p) => {
                                        // whether object hits something in the way of the light
                                        let light_hit = self.trace_ray(&p, light_point);
            
                                        match light_hit {
                                            // no shadow
                                            None => {
                                                last_hit = Some(p);
                                                let s = self.blinn_phong(p, l);
                                                s
                                            }
                                            // object casting shadow
                                            Some(p) => {
                                                last_hit = Some(p);
                                                Vector(0.0, 0.0, 0.0)
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
                                        origin = k.p + OFFSET_AMOUNT * k.normal.to_unit_vector();
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
                                    let f = c.to_u8();
                                    self.pixels[y][x].color = Some(self.pixels[y][x].avg_colors(RGB { r: f[0] as u8, g: f[1] as u8, b: f[2] as u8 }));
                                }
                                None => {
                                    self.pixels[y][x].color = Some(self.pixels[y][x].avg_colors(RGB { r: 0, g: 0, b: 0 }));
                                }
                            }    
                        }
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
        let offset_point = intersection + OFFSET_AMOUNT * object_normal;

        let light_ray = Ray::new(offset_point, light.to_unit_vector());
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

    pub fn blinn_phong(&self, p: Hit, light: Vector) -> Vector {
        let intersection = p.p;
        let object_normal = p.normal;
        let obj_to_light = (light - intersection).to_unit_vector();

        let mut color = Vector(0.0, 0.0, 0.0);
        let obj_to_camera = (self.camera.position - intersection).to_unit_vector();

        // ambient
        color = color + (p.material.ambient * self.ambient);
        // diffuse
        color = color + (obj_to_light.dot(object_normal.to_unit_vector()) * (p.material.diffuse * self.diffuse));
        // specular
        let h = (obj_to_light + obj_to_camera).to_unit_vector();
        color = color + object_normal.dot(h).powf(p.material.shine / 4.0) * (p.material.specular * self.specular);

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
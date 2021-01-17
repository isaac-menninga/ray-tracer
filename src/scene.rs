use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::sphere::Hit;
use crate::pixel::Pixel;
use crate::vector::Vector;
use crate::ray::*;
use lodepng::RGB;

pub struct Scene {
    camera: Camera,
    objects: [Sphere; 4],
    pub height: usize, 
    pub width: usize,
    pub pixels: Vec<Vec<Pixel>>,
    pub render_pixels: Vec<RGB<u8>>,
    pub light: Vector,
    pub ambient: Vector,
    pub diffuse: Vector, 
    pub specular: Vector,
}

const NSAMPLES: usize = 100;
const REFLECTION_DEPTH: usize = 3;

impl Scene {
    pub fn new(
        c: Camera, 
        o: [Sphere; 4], 
        h: usize, 
        w: usize, 
        light_x: f32, 
        light_y: f32, 
        light_z: f32,
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
            render_pixels: Vec::new(),
            light: Vector(light_x, light_y, light_z),
            ambient: Vector(ambient.x(), ambient.y(), ambient.z()),
            diffuse: Vector(diffuse.x(), diffuse.y(), diffuse.z()),
            specular: Vector(specular.x(), specular.y(), specular.z()),
        }
    }

    pub fn render(mut self) {
        for row in &self.pixels {
            for pixel in row {
                let mut color = None;
                let mut origin = self.camera.get_random_vector();
                let mut direction = pixel.pos;
                let mut reflection = 1.0;
                let mut n_reflections = 0;
                let mut last_hit;

                while n_reflections < REFLECTION_DEPTH {
                    let ray = get_ray(origin, direction);
                    let initial_hit = self.check_hits(&ray);

                    let sampled_color = match initial_hit {
                        None => {
                            last_hit = None;
                            (0.3 * Vector(0.5, 0.7, 1.0)) + (0.7 * Vector(1.0, 1.0, 1.0))
                        }
                        Some(p) => {
                            // whether object hits something in the way of the light
                            let light_hit = self.trace_ray(&p);

                            match light_hit {
                                // if the light is closer than any object, use the hit above
                                None => {
                                    last_hit = Some(p);
                                    let s = self.blinn_phong(p);
                                    s
                                }
                                Some(_) => {
                                    last_hit = None;
                                    Vector(0.0, 0.0, 0.0)
                                }
                            }
                        }
                    };
                    match last_hit {
                        Some(k) => {
                            match color {
                                // if color already exists, average it with new color
                                Some(c) => {
                                    color = Some((c + (reflection * sampled_color)) / 2.0);
                                }
                                // if no color exists, it's the sampled color
                                None => {
                                    color = Some(sampled_color);
                                }
                            }
                            n_reflections += 1;
                            reflection = reflection * k.material.reflectiveness;
                            origin = k.p + 0.0005 * k.normal.to_unit_vector();
                            direction = self.reflected_vector(&k);
                        }
                        // if the last hit wasn't another object
                        None => {
                            match color {
                                Some(_) => {}
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
                        self.render_pixels.push(RGB { r: f[0] as u8, g: f[1] as u8, b: f[2] as u8 });
                    }
                    None => {
                        self.render_pixels.push(RGB { r: 0, g: 0, b: 0 });
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

    pub fn trace_ray(&self, hit: &Hit) -> Option<Hit> {
        let intersection = hit.p;
        let object_normal = hit.normal;
        let offset_point = intersection + 0.0005 * object_normal;

        let light_ray = Ray::new(offset_point, self.light.to_unit_vector());
        let light_hit = self.check_hits(&light_ray);

        let obj_to_light = (self.light - intersection).to_unit_vector();

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

    pub fn blinn_phong(&self, p: Hit) -> Vector {
        let intersection = p.p;
        let object_normal = p.normal;
        let obj_to_light = (self.light - intersection).to_unit_vector();

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

        match lodepng::encode24_file(fname, &self.render_pixels, self.width, self.height) {
            Ok(()) => true,
            Err(err) => {
                println!("Error writing file \"{}\": {}", filename, err);
                false
            }
        }
    }
}
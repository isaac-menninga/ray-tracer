use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::sphere::Hit;
use crate::pixel::Pixel;
use crate::vector::Vector;
use crate::ray::Ray;
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
                let pixel_ray = self.camera.get_ray(pixel.pos);
                let pixel_hit = self.check_hits(&pixel_ray);

                match pixel_hit {
                    None => {
                        self.render_pixels.push(RGB { r: 0, g: 0, b: 0 })
                    }
                    Some(p) => {
                        let intersection = p.p;
                        let object_normal = p.normal;
                        let offset_point = intersection + 0.0005 * object_normal;

                        let light_ray = Ray::new(offset_point, self.light.to_unit_vector());
                        let light_hit = self.check_hits(&light_ray);

                        let obj_to_light = (self.light - intersection).to_unit_vector();

                        match light_hit {
                            None => {
                                self.render_pixels.push(self.blinn_phong(p));
                            }
                            Some(p) => {
                                if obj_to_light.length() < p.p.length() {
                                    self.render_pixels.push(self.blinn_phong(p));
                                } else {
                                    self.render_pixels.push(RGB { r: 0, g: 0, b: 0 });
                                }
                            }
                        }
                    }
                }
            }
        }

        self.make_png("out.png".to_string());
    }

    pub fn blinn_phong(&self, p: Hit) -> RGB<u8> {
        let intersection = p.p;
        let object_normal = p.normal;
        let obj_to_light = (self.light - intersection).to_unit_vector();

        let mut color = Vector(0.0, 0.0, 0.0);
        let obj_to_camera = (self.camera.position - intersection).to_unit_vector();

        // ambient
        color = color + (p.ambient * self.ambient);
        // diffuse
        color = color + (obj_to_light.dot(object_normal.to_unit_vector()) * (p.diffuse * self.diffuse));
        // specular
        let h = (obj_to_light + obj_to_camera).to_unit_vector();
        color = color + object_normal.dot(h).powf(p.shine / 4.0) * (p.specular * self.specular);

        return RGB { r: (color.x() * 255.0) as u8, g: (color.y() * 255.0) as u8, b: (color.z() * 255.0) as u8 }
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
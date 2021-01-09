use crate::vector::Vector;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
    pub ambient: Vector,
    pub diffuse: Vector, 
    pub specular: Vector,
    pub shine: f32,
}

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Vector,
    radius: f32,
    ambient: Vector,
    diffuse: Vector, 
    specular: Vector,
    shine: f32,
}

impl Sphere {
    pub fn new(pos: &Vector, r: f32, ambient: &Vector, diffuse: &Vector, specular: &Vector, shine: f32) -> Self {
        Self {
            center: Vector(pos.x(), pos.y(), pos.z()),
            radius: r,
            ambient: Vector(ambient.x(), ambient.y(), ambient.z()),
            diffuse: Vector(diffuse.x(), diffuse.y(), diffuse.z()),
            specular: Vector(specular.x(), specular.y(), specular.z()),
            shine: shine,
        }
    }

    pub fn ray_intersect(&self, r: &Ray) -> Option<Hit> {
        // let oc = r.origin - self.center;
        // let hb = 2.0 * oc.dot(r.direction);
        // let c = oc.dot(oc) - self.radius * self.radius;
        // let discriminant = hb * hb - a * c;
        
        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(r.origin - self.center);
        let c = (r.origin - self.center).dot(r.origin - self.center) - self.radius.powf(2.0);
        let delta = b.powf(2.0) - 4.0 * c * a;

        if delta > 0.0 {
            let t1 = (-b + delta.sqrt()) / 2.0;
            let t2 = (-b - delta.sqrt()) / 2.0;

            if (t1 > 0.0) & (t2 > 0.0) {
                let mut t = t1;
                if t1 > t2 {
                    t = t2;
                }

                if t > 0.0 {
                    let intersection = r.line_to_p(t);

                    return Some(Hit {
                        t: t,
                        p: intersection,
                        normal: (intersection - self.center) / self.radius,
                        ambient: self.ambient,
                        diffuse: self.diffuse, 
                        specular: self.specular,
                        shine: self.shine,
                    })
                }
            }
        }

        return None;
    }
}

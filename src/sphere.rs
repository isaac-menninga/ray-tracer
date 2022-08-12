use crate::vector::Vector;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: Vector,
    pub diffuse: Vector, 
    pub specular: Vector,
    pub shine: f32,
    pub reflectiveness: f32,
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
    pub material: Material
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub fn new(pos: &Vector, r: f32, ambient: &Vector, diffuse: &Vector, specular: &Vector, shine: f32, reflectiveness: f32) -> Self {
        Self {
            center: Vector(pos.x(), pos.y(), pos.z()),
            radius: r,
            material: Material {
                ambient: Vector(ambient.x(), ambient.y(), ambient.z()),
                diffuse: Vector(diffuse.x(), diffuse.y(), diffuse.z()),
                specular: Vector(specular.x(), specular.y(), specular.z()),
                shine: shine,
                reflectiveness: reflectiveness
            }
        }
    }

    pub fn ray_intersect(&self, r: &Ray) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t = (-b - discriminant.sqrt()) / a;

            if t > 0.0003 {
                let intersection = r.line_to_p(t);

                return Some(Hit {
                    t: t,
                    p: intersection,
                    normal: (intersection - self.center).to_unit_vector(),
                    material: self.material
                })
            } else {
                return None;
            }
        }
    }
}

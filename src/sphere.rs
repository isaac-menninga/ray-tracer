use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
    pub material: Material,
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(
        pos: &Vector,
        r: f32,
        ambient: &Vector,
        diffuse: &Vector,
        specular: &Vector,
        shine: f32,
        reflectiveness: f32,
    ) -> Self {
        Self {
            center: Vector(pos.x(), pos.y(), pos.z()),
            radius: r,
            material: Material {
                ambient: Vector(ambient.x(), ambient.y(), ambient.z()),
                diffuse: Vector(diffuse.x(), diffuse.y(), diffuse.z()),
                specular: Vector(specular.x(), specular.y(), specular.z()),
                shine: shine,
                reflectiveness: reflectiveness,
            },
        }
    }

    pub fn ray_intersect(&self, r: &Ray) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t = (-b - discriminant.sqrt()) / a;

            if t > 0.0003 {
                let intersection = r.line_to_p(t);

                let normal = (intersection - self.center).to_unit_vector();
                let mut outward_normal = normal;
                let front_face = r.direction.dot(outward_normal) < 0.0;

                if front_face {
                    outward_normal = outward_normal;
                } else {
                    outward_normal = -outward_normal;
                }

                return Some(Hit {
                    t: t,
                    p: intersection,
                    normal: outward_normal,
                    material: self.material,
                });
            } else {
                return None;
            }
        }
    }
}

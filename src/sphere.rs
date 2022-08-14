use std::sync::Arc;

use crate::material::Scatter;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Hit {
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
    pub material: Arc<dyn Scatter>,
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(pos: &Vector, r: f32, m: Arc<dyn Scatter>) -> Self {
        Self {
            center: Vector(pos.x(), pos.y(), pos.z()),
            radius: r,
            material: m,
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
                    material: self.material.clone(),
                });
            } else {
                return None;
            }
        }
    }
}

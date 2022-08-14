use crate::{material::Scatter, ray::Ray, sphere::Hit, vector::Vector};

pub struct Metal {
    albedo: Vector,
}

impl Metal {
    pub fn new(albedo_color: Vector) -> Self {
        Self {
            albedo: albedo_color,
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Vector)> {
        let reflected = ray.direction.reflect(hit.normal).to_unit_vector();
        let scattered_ray = Ray::new(hit.p, reflected);

        if scattered_ray.direction.dot(hit.normal) > 0.0 {
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}

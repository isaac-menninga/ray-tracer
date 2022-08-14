use crate::{
    material::Scatter, ray::Ray, sphere::Hit, utils::random_vector_in_unit_sphere, vector::Vector,
};

pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo_color: Vector) -> Self {
        Self {
            albedo: albedo_color,
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<(Ray, Vector)> {
        let mut target = hit.p + hit.normal + random_vector_in_unit_sphere();

        // guard against direction being near 0 after random vector
        if target.near_zero() {
            target = hit.normal;
        }

        let scattered = Ray::new(hit.p, target - hit.p);

        Some((scattered, self.albedo))
    }
}

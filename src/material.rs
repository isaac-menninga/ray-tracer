use crate::ray::Ray;
use crate::sphere::Hit;
use crate::vector::Vector;

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Vector)>;
}

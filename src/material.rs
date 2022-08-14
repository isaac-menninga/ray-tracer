use crate::ray::Ray;
use crate::sphere::Hit;
use crate::vector::Vector;

pub trait Scatter<T: Scatter<T>> {
    fn scatter(&self, ray: &Ray, hit: &Hit<T>) -> Option<(Ray, Vector)>;
}

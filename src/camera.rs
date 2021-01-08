use crate::vector::Vector;
use crate::ray::Ray;

pub struct Camera {
    pub position: Vector,
}

impl Camera {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
    ) -> Self {        
        Self {
            position: Vector(x, y, z),
        }
    }

    pub fn get_ray(&self, pos: Vector) -> Ray {
        Ray::new(
            self.position, 
            (pos - self.position).to_unit_vector()
        )
    }
}

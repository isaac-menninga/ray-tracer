use crate::vector::Vector;
use crate::ray::Ray;

pub struct Camera {
    pub position: Vector,
    pub viewport_distance: f32,
}

impl Camera {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        d: f32,
    ) -> Self {        
        Self {
            position: Vector(x, y, z),
            viewport_distance: d,
        }
    }

    pub fn get_ray(&self, pos: Vector) -> Ray {
        Ray::new(
            self.position, 
            (pos - self.position).to_unit_vector()
        )
    }
}

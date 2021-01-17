use crate::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(o: Vector, d: Vector) -> Self {
        Self { origin: o, direction: d }
    }

    pub fn line_to_p(&self, p: f32) -> Vector {
        self.origin + p * self.direction
    }
}

pub fn get_ray(origin: Vector, destination: Vector) -> Ray {
    Ray::new(
        origin, 
        (destination - origin).to_unit_vector()
    )
}
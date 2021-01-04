use crate::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(o: Vector, d: Vector) -> Ray {
        Ray { origin: o, direction: d }
    }

    pub fn line_to_p(&self, p: f32) -> Vector {
        self.origin + p * self.direction
    }
}
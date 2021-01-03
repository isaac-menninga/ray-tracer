mod Vector;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(o: Vector, d: Vector) -> Ray {
        Ray { origin: o, direction: d }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector {
        self.origin + t * self.direction
    }
}
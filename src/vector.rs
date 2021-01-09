use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub f32, pub f32, pub f32);

impl Vector {
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }

    pub fn dot(&self, other: Vector) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector(self.1 * other.2 - self.2 * other.1,
             -(self.0 * other.2 - self.2 * other.0),
             self.0 * other.1 - self.1 * other.0)
    }

    pub fn squared_length(self) -> f32 { self.dot(self) }
    pub fn length(self) -> f32 { self.squared_length().sqrt() }

    pub fn to_u8(&self) -> [u8;3] {
        fn u(f: f32) -> u8 {
            if f < 0.0 {
                0
            } else if f >= 1.0 {
                255
            } else {
                (f * 255.9) as i32 as u8
            }
        }
        [u(self.0), u(self.1), u(self.2)]
    }

    pub fn to_unit_vector(&self) -> Vector {
        *self / self.length()
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector(-self.0, -self.1, -self.2)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector {
        Vector(self * v.0, self * v.1, self * v.2)
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector {
        Vector(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, r: f32) -> Vector {
        (1.0 / r) * self
    }
}


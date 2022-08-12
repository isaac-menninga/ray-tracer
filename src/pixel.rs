use crate::vector::Vector;
use lodepng::RGB;

pub struct Pixel {
    pub pos: Vector,
    pub color: Option<RGB<u8>>,
}

impl Pixel {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector(x, y, 0.0),
            color: None
        }
    }
}

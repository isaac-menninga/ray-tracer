use crate::vector::Vector;
use lodepng::RGB;

pub struct Pixel {
    pub pos: Vector,
    pub color: RGB<u8>,
}

impl Pixel {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector(x, y, 0.0),
            color: RGB { r: 0, g: 0, b: 0 }
        }
    }
}

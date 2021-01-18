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
    pub fn avg_colors(&self, b: RGB<u8>) -> RGB<u8> {
        match self.color {
            Some(c) => {
                return RGB {
                    r: (c.r / 2) + (b.r / 2),
                    g: (c.g / 2) + (b.g / 2),
                    b: (c.b / 2) + (b.b / 2),
                };
            }
            None => {
                return b;
            }
        };
    }
}

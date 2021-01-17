use crate::vector::Vector;
use rand::random;

pub struct Camera {
    pub position: Vector,
}

pub const CAMERA_RADIUS: f32 = 0.0;

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

    pub fn get_random_vector(&self) -> Vector {
        let r = CAMERA_RADIUS * random::<f32>().sqrt();
        let theta = random::<f32>() * 2.0 * std::f32::consts::PI;

        let x = self.position.x() + r * theta.cos();
        let y = self.position.y() + r * theta.sin();

        Vector(x, y, self.position.z())
    }
}

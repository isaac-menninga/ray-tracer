use crate::vector::Vector;
use crate::ray::Ray;
use rand::random;

pub struct Camera {
    pub position: Vector,
}

pub const CAMERA_RADIUS: f32 = 0.0002;

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
        let r = CAMERA_RADIUS * random::<f32>().sqrt();
        let theta = random::<f32>() * 2.0 * std::f32::consts::PI;
        let x = self.position.x() + r * theta.cos();
        let y = self.position.y() + r * theta.sin();

        let random_pos = Vector(x, y, self.position.z());

        Ray::new(
            random_pos, 
            (pos - random_pos).to_unit_vector()
        )
    }
}

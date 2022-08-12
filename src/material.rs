use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: Vector,
    pub diffuse: Vector,
    pub specular: Vector,
    pub shine: f32,
    pub reflectiveness: f32,
}

impl Material {}

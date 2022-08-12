use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vector,
    pub height: i32,
    pub width: i32,
    pub vertical: Vector,
    pub horizontal: Vector,
    pub focal_length: f32,
    pub lower_left_corner: Vector,
}

impl Camera {
    pub fn new(p: Vector, w: i32) -> Self {
        let mut c = Self {
            position: p,
            height: 2,
            width: (2 as f32 * crate::ASPECT_RATIO) as i32,
            vertical: Vector(0.0, w as f32 / crate::ASPECT_RATIO, 0.0),
            horizontal: Vector(w as f32, 0.0, 0.0),
            focal_length: crate::FOCAL_LENGTH,
            lower_left_corner: Vector(0.0, 0.0, 0.0),
        };

        c.lower_left_corner = c.position
            - c.vertical / 2.0
            - c.horizontal / 2.0
            - Vector(0.0, 0.0, crate::FOCAL_LENGTH);
        c
    }
}

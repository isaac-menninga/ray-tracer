use crate::viewport::Viewport;
use crate::vector::Vector;
use crate::ray::Ray;

pub struct Camera {
    pub position: Vector,
    pub viewport: Viewport,
    pub viewport_distance: f32,
    pub lower_left_corner: Vector,
}

impl Camera {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        h: usize,
        w: usize,
        d: f32,
    ) -> Self {
        Self {
            position: Vector(x, y, z),
            viewport: Viewport::new(h, w),
            viewport_distance: d,
            lower_left_corner: Vector(
                x - (w as f32 / 2.0),
                y - (h as f32 / 2.0),
                z + d
            )
        }
    }

    pub fn get_ray(&self, pixel_i: i32) -> Ray {
        Ray {
            origin: self.position.clone(),
            direction: self.lower_left_corner + Vector(
                (pixel_i % self.viewport.height as i32) as f32,
                (pixel_i % self.viewport.width as i32) as f32,
                0.0
            )
        }
    }
}

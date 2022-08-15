use crate::utils;
use crate::vector::Vector;

pub struct Camera {
    pub origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    cu: Vector,
    cv: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vector,
        lookat: Vector,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).to_unit_vector();
        let cu = vup.cross(cw).to_unit_vector();
        let cv = cw.cross(cu);

        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;

        let llc = lookfrom - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            cu: cu,
            cv: cv,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_pixel_direction(&self, x: f64, y: f64) -> (Vector, Vector) {
        let rd = self.lens_radius * utils::random_vector_in_unit_sphere();
        let offset = rd.x() * self.cu + rd.y() * self.cv;

        let direction =
            self.lower_left_corner + x * self.horizontal + y * self.vertical - self.origin - offset;
        let origin = self.origin + offset;

        (origin, direction)
    }
}

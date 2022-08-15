use crate::rand::Rng;
use crate::vector::Vector;

pub fn random_in_range(min: f64, max: f64) -> f64 {
    return rand::thread_rng().gen_range(min, max);
}

pub fn random_vector_in_unit_sphere() -> Vector {
    let vec = Vector(
        random_in_range(-1.0, 1.0),
        random_in_range(-1.0, 1.0),
        random_in_range(-1.0, 1.0),
    );

    if vec.length() <= 1.0 {
        return vec.to_unit_vector();
    } else {
        return random_vector_in_unit_sphere();
    }
}

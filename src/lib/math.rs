use rand::{rngs::{OsRng}, Rng};
use std::f64::consts::PI;

use super::vec::Vector3;

#[allow(dead_code)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    return x;
}

pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::random_bound(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        };
        return p;
    }
}

pub fn random_unit_vector() -> Vector3 {
    return random_in_unit_sphere().unit();
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: Vector3) -> Vector3 {
    let in_unit_sphere = random_in_unit_sphere();
    if Vector3::dot(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn calculate_reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::new(OsRng.gen_range(-1.0..1.0), OsRng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

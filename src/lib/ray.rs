

use super::{vec::Vector3, hittable::{HitRecord, Hittable, hittable_list::HittableList}};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn inherit(&mut self, new: Self) {
        self.origin = new.origin;
        self.direction = new.direction;
    }
    pub fn zero() -> Self {
        Self {
            origin: Vector3::zero(),
            direction: Vector3::zero(),
        }
    }
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f64) -> Vector3 {
        return self.origin + self.direction * t;
    }
    pub fn ray_color(r: &Self, world: &HittableList, depth: u8) -> Vector3 {
        let mut rec = HitRecord::new();

        if depth <= 0 {
            return Vector3::zero();
        };
        if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
            let mut scattered = Ray::zero();
            let mut attenuation = &mut Vector3::zero();

            if rec
                .mat
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return *attenuation * Ray::ray_color(&scattered, world, depth - 1);
            }
            return Vector3::zero();
        }

        let unit_direction = r.direction.unit();
        let t = (unit_direction.y + 1.0) * 0.5;
        return Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;
    }
}

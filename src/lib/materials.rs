use rand::{rngs::{OsRng}, Rng};

use super::{vec::Vector3, ray::Ray, hittable::HitRecord, math::{random_unit_vector, random_in_unit_sphere, calculate_reflectance}};

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    mat_type: MaterialType,
    albedo: Vector3,
    fuzz: Option<f64>,
    ir: Option<f64>,
}
impl Material {
    pub fn lambertian(color: Vector3) -> Self {
        Self {
            albedo: color,
            mat_type: MaterialType::Lambertian,
            fuzz: None,
            ir: None,
        }
    }
    pub fn metal(color: Vector3, fuzz: f64) -> Self {
        Self {
            albedo: color,
            mat_type: MaterialType::Metal,
            fuzz: Some(fuzz),
            ir: None,
        }
    }
    pub fn dielectric(index_of_refraction: f64) -> Self {
        Self {
            albedo: Vector3::zero(),
            mat_type: MaterialType::Dielectric,
            fuzz: None,
            ir: Some(index_of_refraction),
        }
    }
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        match self.mat_type {
            MaterialType::Lambertian => {
                let mut scatter_direction = rec.normal + random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                scattered.inherit(Ray::new(rec.p, scatter_direction));
                attenuation.inherit(self.albedo);
                return true;
            }
            MaterialType::Metal => {
                let reflected = Vector3::reflect(r_in.direction.unit(), rec.normal);
                scattered.inherit(Ray::new(
                    rec.p,
                    reflected + random_in_unit_sphere() * self.fuzz.unwrap(),
                ));
                attenuation.inherit(self.albedo);
                return Vector3::dot(scattered.direction, rec.normal) > 0.0;
            }
            MaterialType::Dielectric => {
                attenuation.inherit(Vector3::new(1.0, 1.0, 1.0));
                let refraction_ratio = if rec.front_face {
                    1.0 / self.ir.unwrap()
                } else {
                    self.ir.unwrap()
                };
                let unit_direction = r_in.direction.unit();

                let cos_theta = Vector3::dot(-unit_direction, rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

                let direction;

                if cannot_refract || calculate_reflectance(cos_theta, refraction_ratio) > OsRng.gen()
                {
                    direction = Vector3::reflect(unit_direction, rec.normal);
                } else {
                    direction = Vector3::refract(unit_direction, rec.normal, refraction_ratio);
                }

                scattered.inherit(Ray::new(rec.p, direction));
                return true;
            }
        }
    }
}

use super::{materials::Material, ray::Ray, vec::Vector3};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    t: f64,
    pub front_face: bool,
    pub mat: Material,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vector3::zero(),
            normal: Vector3::zero(),
            t: 0.0,
            front_face: false,
            mat: Material::lambertian(Vector3::new(0.5, 0.5, 0.5)),
        }
    }

    pub fn inherit(&mut self, new: Self) {
        self.front_face = new.front_face;
        self.normal = new.normal;
        self.p = new.p;
        self.t = new.t;
        self.mat = new.mat;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub mod sphere {
    use crate::lib::{materials::Material, ray::Ray, vec::Vector3};

    use super::{HitRecord, Hittable};

    #[derive(Debug, Clone, Copy)]
    pub struct Sphere {
        center: Vector3,
        radius: f64,
        mat: Material,
    }

    impl Sphere {
        pub fn new(center: Vector3, radius: f64, mat: Material) -> Self {
            Self {
                center,
                radius,
                mat,
            }
        }
    }

    impl Hittable for Sphere {
        fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
            let oc = r.origin - self.center;
            let a = r.direction.length_squared();
            let half_b = Vector3::dot(oc, r.direction);
            let c = oc.length_squared() - self.radius * self.radius;
            let discriminant = half_b * half_b - a * c;

            if discriminant < 0.0 {
                return false;
            }
            let sqrtd = discriminant.sqrt();

            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return false;
                }
            };
            let t = root;
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let front_face = Vector3::dot(r.direction, outward_normal) < 0.0;
            let normal = if front_face {
                outward_normal
            } else {
                -outward_normal
            };

            rec.inherit(HitRecord {
                p,
                normal,
                t,
                front_face,
                mat: self.mat,
            });

            return true;
        }
    }
}

pub mod hittable_list {
    use rand::{rngs::{OsRng}, Rng};

    use crate::lib::{materials::Material, ray::Ray, vec::Vector3};

    use super::{sphere::Sphere, HitRecord, Hittable};

    #[derive(Debug, Clone)]
    pub struct HittableList {
        objects: Vec<Sphere>,
    }

    impl HittableList {
        pub fn new() -> Self {
            Self { objects: vec![] }
        }

        #[allow(dead_code)]
        pub fn clear(&mut self) {
            self.objects.clear();
        }

        pub fn add(&mut self, object: Sphere) {
            self.objects.push(object);
        }

        pub fn random_scene() -> Self {
            let mut world = HittableList::new();

            let ground_material = Material::lambertian(Vector3::new(0.5, 0.5, 0.5));
            world.add(Sphere::new(
                Vector3::new(0.0, -1000.0, 0.0),
                1000.0,
                ground_material,
            ));

            for a in -11..11 {
                for b in -11..11 {
                    let choose_mat: f64 = OsRng.gen();
                    let center = Vector3::new(
                        a as f64 + 0.9 * OsRng.gen::<f64>(),
                        0.2,
                        b as f64 + 0.9 * OsRng.gen::<f64>(),
                    );
                    let sphere_material;

                    if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                        if choose_mat < 0.8 {
                            let albedo = Vector3::random() * Vector3::random();
                            sphere_material = Material::lambertian(albedo);
                        } else if choose_mat < 0.95 {
                            let albedo = Vector3::random_bound( 0.5, 1.0);
                            let fuzz: f64 = OsRng.gen();
                            sphere_material = Material::metal(albedo, fuzz);
                        } else {
                            sphere_material = Material::dielectric(1.5);
                        }
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                }
            }

            let material1 = Material::dielectric(1.5);
            world.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1));

            let material2 = Material::lambertian(Vector3::new(0.4, 0.2, 0.1));
            world.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2));

            let material3 = Material::metal(Vector3::new(0.7, 0.6, 0.5), 0.0);
            world.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3));

            return world;
        }
    }

    impl Hittable for HittableList {
        fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
            let mut temp_rec = HitRecord {
                p: Vector3::zero(),
                normal: Vector3::zero(),
                t: 0.0,
                front_face: false,
                mat: Material::lambertian(Vector3::zero()),
            };
            let mut hit_anything = false;
            let mut closest_so_far = t_max;

            for object in self.objects.iter() {
                if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    rec.inherit(temp_rec);
                };
            }
            return hit_anything;
        }
    }
}

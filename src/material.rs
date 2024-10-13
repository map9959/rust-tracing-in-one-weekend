use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material{
    fn scatter(&self, r_in: &Ray, record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
}

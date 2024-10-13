use crate::{material::{self, Material}, utils::Interval, vec3::dot, Ray, Vec3};
use std::sync::Arc;
pub struct HitRecord{
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}
impl HitRecord{
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool, material: Arc<dyn Material>) -> HitRecord{
        HitRecord{p, normal, t, front_face, material}
    }
    pub fn generate(p: Vec3, outward_normal: Vec3, t: f64, r: &Ray, material: Arc<dyn Material>) -> HitRecord{
        let front_face: bool = dot(&r.direction, &outward_normal) < 0.0;
        let normal: Vec3 = if front_face {outward_normal} else {-outward_normal};
        HitRecord{p, normal, t, front_face, material}
    }
}
pub trait Hittable{
    fn intersect(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
pub struct Sphere{
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>
}
impl Sphere{
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Sphere{
        Sphere{center, radius, material}
    }
}
impl Hittable for Sphere{
    fn intersect(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>{
        let oc: Vec3 = self.center-r.origin;
        let a: f64 = r.direction.length_squared();
        let h: f64 = dot(&r.direction, &oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0{
            return None;
        }else{
            let sqrt_d: f64 = f64::sqrt(discriminant);
            let mut root: f64 = (h-sqrt_d)/a;
            if !ray_t.surrounds(root){
                root = (h+sqrt_d)/a;
                if !ray_t.surrounds(root){
                    return None;
                }
            }
            let p: Vec3 = r.at(root);
            return Some(HitRecord::generate(
                p,
                (p-self.center)/self.radius,
                root,
                r,
                self.material.clone()
            ));
        }
    }
}
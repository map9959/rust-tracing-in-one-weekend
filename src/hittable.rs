use crate::{material::Material, utils::Interval, vec3::dot, Ray, Vec3};
pub struct HitRecord{
    pub p: Vec3,
    pub normal: Vec3,
    //pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool
}
impl HitRecord{
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> HitRecord{
        HitRecord{p, normal, t, front_face}
    }
    pub fn generate(p: Vec3, outward_normal: Vec3, t: f64, r: &Ray) -> HitRecord{
        let front_face: bool = dot(&r.direction, &outward_normal) < 0.0;
        let normal: Vec3 = if front_face {outward_normal} else {-outward_normal};
        HitRecord{p, normal, t, front_face}
    }
}
pub trait Hittable{
    fn intersect(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
pub struct Sphere{
    center: Vec3,
    radius: f64
}
impl Sphere{
    pub fn new(center: Vec3, radius: f64) -> Sphere{
        Sphere{center, radius}
    }
}
impl Hittable for Sphere{
    fn intersect(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>{
        let oc = self.center-r.origin;
        let a = r.direction.length_squared();
        let h = dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

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
            let p = r.at(root);
            return Some(HitRecord::generate(
                p,
                (p-self.center)/self.radius,
                root,
                r
            ));
        }
    }
}
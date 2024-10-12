use crate::{Ray, Vec3, vec3::dot};
pub trait Hittable{
    fn intersect(&self, r: &Ray) -> f64;
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
    fn intersect(&self, r: &Ray) -> f64 {
        let oc = self.center-r.origin;
        let a = r.direction.length_squared();
        let h = dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0{
            return -1.0;
        }else{
            return (h-f64::sqrt(discriminant))/a
        }
    }
}
use crate::{hittable::HitRecord, ray::Ray, vec3::{random_unit_vector, reflect, Vec3}};

pub trait Material{
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)>;
}
pub struct Lambertian{
    albedo: Vec3
}
impl Lambertian{
    pub fn new(albedo: Vec3) -> Lambertian{
        Lambertian{albedo}
    }
}
impl Material for Lambertian{
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_bounce: Vec3 = random_unit_vector();
        let scatter_direction: Vec3 = if !(record.normal + scatter_bounce).near_zero(){
            record.normal + scatter_bounce
        }else{
            record.normal
        };
        let scattered = Ray::new(record.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal{
    albedo: Vec3
}
impl Metal{
    pub fn new(albedo: Vec3) -> Metal{
        Metal{albedo}
    }
}
impl Material for Metal{
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&r_in.direction, &record.normal);
        let scattered = Ray::new(record.p, reflected);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
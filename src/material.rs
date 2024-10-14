use crate::{hittable::HitRecord, ray::Ray, vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3}};

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
    albedo: Vec3,
    fuzz: f64
}
impl Metal{
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal{
        if fuzz > 1.0 || fuzz < 0.0{
            panic!("Fuzz must be between 0 and 1.")
        }
        Metal{albedo, fuzz}
    }
}
impl Material for Metal{
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&r_in.direction, &record.normal);
        let scattered_direction = reflected + random_unit_vector() * self.fuzz;
        let scattered = Ray::new(record.p, scattered_direction);
        let attenuation = self.albedo;
        if dot(&scattered_direction, &record.normal) > 0.0 {
            return Some((scattered, attenuation))
        } else {
            return None
        }
    }
}

pub struct Dielectric{
    refraction_index: f64
}
impl Dielectric{
    pub fn new(refraction_index: f64) -> Dielectric{
        Dielectric { refraction_index }
    }
    pub fn reflectance(&self, cos_theta: f64) -> f64{
        let r0: f64 = (1.0-self.refraction_index)/(1.0+self.refraction_index);
        let r0_squared: f64 = r0*r0;
        r0_squared + (1.0-r0_squared)*f64::powi(1.0-cos_theta, 5)
    }
}
impl Material for Dielectric{
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let ri = if record.front_face {1.0/self.refraction_index} else {self.refraction_index};
        let unit_direction = unit_vector(&r_in.direction);

        let cos_theta = f64::min(dot(&-unit_direction, &record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0-cos_theta*cos_theta);

        let direction = if sin_theta * ri > 1.0 || self.reflectance(cos_theta) > rand::random(){
            reflect(&unit_direction, &record.normal)
        } else {
            refract(&unit_direction, &record.normal, ri)
        };

        let scattered = Ray::new(record.p, direction);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        Some((scattered, attenuation))
    }
}
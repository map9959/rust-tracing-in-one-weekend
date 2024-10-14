use crate::vec3::cross;
use crate::vec3::random_in_unit_disk;
use crate::vec3::Vec3;
use crate::vec3::random_on_hemisphere;
use crate::vec3::random_unit_vector;
use crate::Ray;
use crate::Scene;
use crate::Hittable;
use crate::unit_vector;
use crate::utils::*;
use rand::Rng;
use indicatif::ProgressBar;

pub struct Camera{
    image_width: i32,
    image_height: i32,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    samples_per_pixel: i32,
    max_depth: i32,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3
}
impl Camera{
    pub fn new(
        aspect_ratio: f64, 
        image_width: i32,
        vfov: f64, 
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        samples_per_pixel: i32,
        max_depth: i32
    ) -> Camera{
        let image_height = (image_width as f64/aspect_ratio) as i32;

        let camera_center = look_from;
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64/image_height as f64);

        let w = unit_vector(&(look_from-look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u/image_width as f64;
        let pixel_delta_v = viewport_v/image_height as f64;

        let viewport_upper_left = camera_center - w*focus_dist - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u+pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle/2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera{image_width, image_height, camera_center, pixel_delta_u, pixel_delta_v, pixel00_loc, samples_per_pixel, max_depth, defocus_angle, defocus_disk_u, defocus_disk_v}
    }

    pub fn render(&self, scene: &Scene){
        std::print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let progress_bar = ProgressBar::new(self.image_height as u64);
        for j in 0..self.image_height{
            for i in 0..self.image_width{
                let mut color = Vec3::new(0.0,0.0,0.0);

                for _ in 0..self.samples_per_pixel{
                    let camera_ray = self.get_ray(i, j);
                    color = color + Camera::ray_color(&camera_ray, scene, self.max_depth);
                }

                Camera::write_color(color/self.samples_per_pixel as f64);
            }
            progress_bar.inc(1);
        }
        progress_bar.finish();
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray{
        let offset = Camera::sample_square();
        let pixel_loc = self.pixel00_loc 
            + self.pixel_delta_u*((offset.x+i as f64)) 
            + self.pixel_delta_v*((offset.y+j as f64));
        let ray_origin = if self.defocus_angle <= 0.0 {self.camera_center} else {self.defocus_disk_sample()};
        let pixel_direction = pixel_loc - self.camera_center;
        
        Ray::new(ray_origin, pixel_direction)
    }

    fn sample_square() -> Vec3{
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }
    fn defocus_disk_sample(&self) -> Vec3{
        let p = random_in_unit_disk();
        self.camera_center + self.defocus_disk_u*p.x + self.defocus_disk_v*p.y
    }

    fn ray_color(r: &Ray, scene: &Scene, depth: i32) -> Vec3{
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        match scene.intersect(&r, &NEAR_NON_NEG){
            Some(record) => {
                match record.material.scatter(&r, &record){
                    Some((scattered, attenuation)) => {
                        return Camera::ray_color(&scattered, scene, depth-1)*attenuation;
                    }
                    None => {
                        return Vec3::new(0.0, 0.0, 0.0);
                    }
                }
                //let normal = record.normal;
                //let direction = random_on_hemisphere(&normal) + random_unit_vector();
                //let bounced_ray = Ray::new(record.p, direction);
                //return Camera::ray_color(&bounced_ray, scene, depth-1)*0.3;
            }
            None => {}
        }
    
        let unit_direction = unit_vector(&r.direction);
        let a = unit_direction.y/2.0+0.5;
        Vec3::new(1.0, 1.0, 1.0)*(1.0-a)+Vec3::new(0.5, 0.7, 1.0)*a
    }
    
    fn ray_color_normal(r: &Ray, scene: &Scene) -> Vec3{
        match scene.intersect(&r, &NON_NEG){
            Some(record) => {
                let normal = record.normal;
                return Vec3::new(normal.x+1.0, normal.y+1.0, normal.z+1.0)*0.5;
            }
            None => {}
        }
    
        let unit_direction = unit_vector(&r.direction);
        let a = unit_direction.y/2.0+0.5;
        Vec3::new(1.0, 1.0, 1.0)*(1.0-a)+Vec3::new(0.5, 0.7, 1.0)*a
    }

    pub fn linear_to_gamma(x: f64) -> f64{
        if x > 0.0 {return f64::sqrt(x)} else {return 0.0};
    }
    pub fn write_color(v: Vec3){
        let intensity: Interval = Interval { min: 0.0, max: 0.999};

        let r = Camera::linear_to_gamma(v.x);
        let g = Camera::linear_to_gamma(v.y);
        let b = Camera::linear_to_gamma(v.z);

        let rbyte = (intensity.clamp(r) * 256.0) as i32;
        let gbyte = (intensity.clamp(g) * 256.0) as i32;
        let bbyte = (intensity.clamp(b) * 256.0) as i32;

        println!("{} {} {}", rbyte, gbyte, bbyte);
    }

    fn test_gradient(i: i32, j: i32, image_width: i32, image_height: i32) -> Vec3{
        Vec3::new(
            i as f64/(image_width-1) as f64,
            j as f64/(image_height-1) as f64,
            0.0
        )
    }
}
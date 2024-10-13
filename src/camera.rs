use crate::vec3::Vec3;
use crate::vec3::write_color;
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
    samples_per_pixel: i32
}
impl Camera{
    pub fn new(
        aspect_ratio: f64, 
        image_width: i32, 
        focal_length: f64, 
        viewport_height: f64, 
        camera_center: Vec3,
        samples_per_pixel: i32
    ) -> Camera{
        let image_height = (image_width as f64/aspect_ratio) as i32;

        let viewport_width = viewport_height * (image_width as f64/image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u/image_width as f64;
        let pixel_delta_v = viewport_v/image_height as f64;

        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u+pixel_delta_v) * 0.5;

        Camera{image_width, image_height, camera_center, pixel_delta_u, pixel_delta_v, pixel00_loc, samples_per_pixel}
    }

    pub fn render(&self, scene: &Scene){
        std::print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let progress_bar = ProgressBar::new(self.image_height as u64);
        for j in 0..self.image_height{
            for i in 0..self.image_width{
                let mut color = Vec3::new(0.0,0.0,0.0);

                for _ in 0..self.samples_per_pixel{
                    let camera_ray = self.get_ray(i, j);
                    color = color + Camera::ray_color(&camera_ray, scene);
                }

                write_color(color/self.samples_per_pixel as f64);
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
        let pixel_direction = pixel_loc - self.camera_center;
        
        Ray::new(self.camera_center, pixel_direction)
    }

    fn sample_square() -> Vec3{
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, scene: &Scene) -> Vec3{
        let (hit, record) = scene.intersect(&r, &NON_NEG);
        if hit{
            let hit_record = record.unwrap();
            let normal = hit_record.normal;
            return Vec3::new(normal.x+1.0, normal.y+1.0, normal.z+1.0)*0.5;
        }
    
        let unit_direction = unit_vector(&r.direction);
        let a = unit_direction.y/2.0+0.5;
        Vec3::new(1.0, 1.0, 1.0)*(1.0-a)+Vec3::new(0.5, 0.7, 1.0)*a
    }

    fn test_gradient(i: i32, j: i32, image_width: i32, image_height: i32) -> Vec3{
        Vec3::new(
            i as f64/(image_width-1) as f64,
            j as f64/(image_height-1) as f64,
            0.0
        )
    }
}
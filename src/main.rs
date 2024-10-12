use indicatif::ProgressBar;
use vec3::unit_vector;
use crate::vec3::{Vec3, write_color};
use crate::ray::Ray;
use crate::hittable::Sphere;
use crate::hittable::Hittable;
mod vec3;
mod ray;
mod hittable;

fn test_gradient(i: i32, j: i32, image_width: i32, image_height: i32) -> Vec3{
    Vec3::new(
        i as f64/(image_width-1) as f64,
        j as f64/(image_height-1) as f64,
        0.0
    )
}

fn ray_color(r: Ray) -> Vec3{
    let sphere: Sphere = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5
    );
    let hit = sphere.intersect(&r);
    if hit > 0.0{
        let intersect_point = r.at(hit);
        let normal = unit_vector(&(intersect_point-Vec3::new(0.0,0.0,-1.0)));
        return Vec3::new(normal.x+1.0, normal.y+1.0, normal.z+1.0)*0.5;
    }

    let unit_direction = unit_vector(&r.direction);
    let a = unit_direction.y/2.0+0.5;
    Vec3::new(1.0, 1.0, 1.0)*(1.0-a)+Vec3::new(0.5, 0.7, 1.0)*a
}

fn main() {
    let aspect_ratio = 16.0/9.0;

    let image_width = 400;
    let image_height = (image_width as f64/aspect_ratio) as i32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u/image_width as f64;
    let pixel_delta_v = viewport_v/image_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u+pixel_delta_v) * 0.5;

    std::print!("P3\n{image_width} {image_height}\n255\n");

    let progress_bar = ProgressBar::new(image_height as u64);
    for j in 0..image_height{
        for i in 0..image_width{
            let pixel_loc = pixel00_loc + (pixel_delta_u*i as f64 + pixel_delta_v*j as f64);
            let pixel_direction = pixel_loc - camera_center;
            
            let camera_ray = Ray::new(camera_center, pixel_direction);
            let color = ray_color(camera_ray);
            write_color(color);
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();

    let v1 = Vec3::new(1.0,2.0,3.0);
    //std::println!("{}", v1.length());
    //std::println!("{}", v1.length_squared());
}

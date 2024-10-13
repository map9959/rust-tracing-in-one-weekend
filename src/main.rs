use material::{Lambertian, Metal};
use vec3::unit_vector;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Sphere;
use crate::hittable::Hittable;
use crate::scene::Scene;
use crate::camera::Camera;
use std::sync::Arc;
use rand::Rng;
mod vec3;
mod ray;
mod hittable;
mod scene;
mod utils;
mod camera;
mod material;

fn main() {
    let aspect_ratio = 16.0/9.0;

    let image_width = 400;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let samples_per_pixel = 10;
    let max_depth = 20;

    let mut scene = Scene::new();
    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_s1 = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_s2 = Metal::new(Vec3::new(0.8, 0.8, 0.8));
    let material_s3 = Metal::new(Vec3::new(0.8, 0.6, 0.2));
    let ground: Sphere = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(material_ground)
    );
    let s1: Sphere = Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        Arc::new(material_s1)
    );
    let s2: Sphere = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_s2)
    );
    let s3: Sphere = Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_s3)
    );
    scene.push(Box::new(ground));
    scene.push(Box::new(s1));
    scene.push(Box::new(s2));
    scene.push(Box::new(s3));

    let camera = Camera::new(
        aspect_ratio, 
        image_width, 
        focal_length, 
        viewport_height, 
        camera_center, 
        samples_per_pixel,
        max_depth);
    camera.render(&scene);
}

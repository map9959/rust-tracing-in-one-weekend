use vec3::unit_vector;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Sphere;
use crate::hittable::Hittable;
use crate::scene::Scene;
use crate::camera::Camera;
use rand::Rng;
mod vec3;
mod ray;
mod hittable;
mod scene;
mod utils;
mod camera;

fn main() {
    let aspect_ratio = 16.0/9.0;

    let image_width = 400;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let samples_per_pixel = 20;
    let max_depth = 10;

    let mut scene = Scene::new();
    let sphere1: Sphere = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5
    );
    let sphere2: Sphere = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0
    );
    scene.push(Box::new(sphere1));
    scene.push(Box::new(sphere2));

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

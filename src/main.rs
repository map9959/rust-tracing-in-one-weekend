use material::Dielectric;
use material::{Lambertian, Metal};
use utils::PI;
use vec3::{unit_vector, random_vec, random_vec_range};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Sphere;
use crate::hittable::Hittable;
use crate::scene::Scene;
use crate::camera::Camera;
use std::sync::Arc;
use rand::{random, Rng};
mod vec3;
mod ray;
mod hittable;
mod scene;
mod utils;
mod camera;
mod material;

fn main() {
    let aspect_ratio = 16.0/9.0;

    let image_width = 1200;

    let vfov = 20.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let samples_per_pixel = 100;
    let max_depth = 100;

    let defocus_angle = 0.1;
    let focus_dist = 10.0;

    let scene = generate_full_scene();

    let camera = Camera::new(
        aspect_ratio, 
        image_width, 
        vfov, 
        look_from, 
        look_at,
        vup,
        defocus_angle,
        focus_dist,
        samples_per_pixel,
        max_depth);
    camera.render(&scene);
}
fn generate_simple_scene() -> Scene{
    let mut scene = Scene::new();
    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_s1 = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_s2 = Dielectric::new(1.5);
    let material_s2_bubble = Dielectric::new(1.0/1.5);
    let material_s3 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);
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
    let s2_bubble: Sphere = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        Arc::new(material_s2_bubble)
    );
    let s3: Sphere = Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_s3)
    );
    scene.push(Box::new(ground));
    scene.push(Box::new(s1));
    scene.push(Box::new(s2));
    scene.push(Box::new(s2_bubble));
    scene.push(Box::new(s3));
    
    scene
}
fn generate_full_scene() -> Scene{
    let mut scene = Scene::new();
    let material_ground = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    let ground: Sphere = Sphere::new(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Arc::new(material_ground)
    );
    scene.push(Box::new(ground));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(
                a as f64+0.9*rand::random::<f64>(),
                0.2,
                b as f64+0.9*rand::random::<f64>()
            );

            match choose_mat{
                x if x < 0.8 => {
                    let albedo = random_vec()*random_vec();
                    let sphere_material = Lambertian::new(albedo);
                    let sphere = Sphere::new(
                        center,
                        0.2,
                        Arc::new(sphere_material)
                    );
                    scene.push(Box::new(sphere));
                }
                x if x < 0.95 => {
                    let albedo = random_vec_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>()/2.0;
                    let sphere_material = Metal::new(albedo, fuzz);
                    let sphere = Sphere::new(
                        center,
                        0.2,
                        Arc::new(sphere_material)
                    );
                    scene.push(Box::new(sphere));
                }
                _ => {
                    let sphere_material = Dielectric::new(1.5);
                    let sphere = Sphere::new(
                        center,
                        0.2,
                        Arc::new(sphere_material)
                    );
                    scene.push(Box::new(sphere));
                }
            }
        }
    }

    let material_s1 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    let material_s2 = Dielectric::new(1.5);
    let material_s3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    let s1: Sphere = Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material_s1)
    );
    let s2: Sphere = Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material_s2)
    );
    let s3: Sphere = Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material_s3)
    );
    scene.push(Box::new(s1));
    scene.push(Box::new(s2));
    scene.push(Box::new(s3));
    
    scene
}
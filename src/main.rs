use indicatif::ProgressBar;
use crate::vec3::Vec3;
mod vec3;
fn main() {
    let image_width = 256;
    let image_height = 256;

    std::print!("P3\n{image_width} {image_height}\n255\n");

    let progress_bar = ProgressBar::new(image_height as u64);
    for j in 0..image_height{
        for i in 0..image_width{
            let r: f32 = i as f32/(image_width-1) as f32;
            let g: f32 = j as f32/(image_height-1) as f32;
            let b: f32 = 0.0;

            let ir: i32 = (r*255.999) as i32;
            let ig: i32 = (g*255.999) as i32;
            let ib: i32 = (b*255.999) as i32;

            std::println!("{ir} {ig} {ib}");
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();

    let v1 = Vec3::new(1.0,2.0,3.0);
    std::println!("{}", v1.length());
    std::println!("{}", v1.length_squared());
}

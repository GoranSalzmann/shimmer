use indicatif::ProgressBar;
use rand::{rngs::OsRng, Rng};
use rayon::{
    self,
    prelude::{IntoParallelRefIterator, ParallelIterator},
};
use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::lib::{
    camera::Camera,
    hittable::hittable_list::HittableList,
    output::{push_vector, save_image, Pixel},
    ray::Ray,
    vec::Vector3,
};
mod lib;

fn main() {
    // Image

    static ASPECT_RATIO: f64 = 21.0 / 9.0;
    static IMAGE_WIDTH: u32 = 3440;
    static IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    static SAMPLES_PER_PIXEL: u16 = 100;
    static MAX_DEPTH: u8 = 5;

    // World

    let world = HittableList::random_scene();

    // Camera

    let cam = Camera::new(
        &Vector3::new(13.0, 2.0, 3.0),
        &Vector3::new(0.0, 0.0, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    // Render

    let begin = Instant::now();

    let bar = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH).into());

    let data: Arc<Mutex<Vec<Pixel>>> = Arc::new(Mutex::new(vec![]));

    let jobs = prepare_jobs(IMAGE_HEIGHT, IMAGE_WIDTH);

    jobs.par_iter().for_each(|pair| {
        let pixel = render_pixel(
            SAMPLES_PER_PIXEL,
            pair.0,
            pair.1,
            IMAGE_WIDTH,
            IMAGE_HEIGHT,
            MAX_DEPTH,
            &cam,
            &world,
        );
        data.lock().unwrap().push(pixel);
        bar.inc(1);
    });

    let output = order_pixels(data, IMAGE_WIDTH, SAMPLES_PER_PIXEL);
    save_image(Path::new("./image.png"), IMAGE_WIDTH, IMAGE_HEIGHT, &output);

    bar.finish();
    println!("Rendered in {} seconds.", begin.elapsed().as_secs(),);
}

pub fn order_pixels(data: Arc<Mutex<Vec<Pixel>>>, width: u32, samples: u16) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    data.lock()
        .unwrap()
        .sort_by(|a, b| a.index(width).cmp(&b.index(width)));

    for p in data.lock().unwrap().iter() {
        push_vector(p.color, &mut output, samples);
    }

    return output;
}

fn render_pixel(
    samples: u16,
    i: u32,
    j: u32,
    width: u32,
    height: u32,
    depth: u8,
    cam: &Camera,
    world: &HittableList,
) -> Pixel {
    let mut pixel_color = Vector3::zero();
    for _s in 0..samples {
        let u = (i as f64 + OsRng.gen::<f64>()) / (width - 1) as f64;
        let v = (j as f64 + OsRng.gen::<f64>()) / (height - 1) as f64;
        let r = cam.get_ray(u, v);
        pixel_color += Ray::ray_color(&r, &world, depth);
    }
    return Pixel::new(pixel_color, i, height - j);
}

fn prepare_jobs(height: u32, width: u32) -> Vec<(u32, u32)> {
    let mut vec: Vec<(u32, u32)> = vec![];
    for j in 0..height {
        for i in 0..width {
            vec.push((i, j));
        }
    }
    return vec;
}

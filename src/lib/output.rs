use std::{fs::File, io::BufWriter, path::Path};

use super::vec::Vector3;

pub fn save_image(path: &Path, width: u32, height: u32, data: &Vec<u8>) {
    let file = File::create(path).unwrap();
    let ref mut buf_writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf_writer, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

pub fn push_rgb(mut r: f64, mut g: f64, mut b: f64, data: &mut Vec<u8>, samples_per_pixel: u16) {
    let scale = 1.0 / samples_per_pixel as f64;

    r *= scale;
    g *= scale;
    b *= scale;

    data.push((255.999 * r) as u8);
    data.push((255.999 * g) as u8);
    data.push((255.999 * b) as u8);
}

pub fn push_vector(color: Vector3, data: &mut Vec<u8>, samples_per_pixel: u16) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    push_rgb(r, g, b, data, samples_per_pixel)
}

#[derive(Debug,Clone, Copy)]
pub struct Pixel {
    pub color: Vector3,
    x: u32,
    y: u32,
}

impl Pixel {
    pub fn new(color: Vector3, x: u32, y: u32) -> Self {
        Self { color, x, y }
    }

    pub fn index(&self, width: u32) -> u32 {
        self.y * width + self.x
    }
}

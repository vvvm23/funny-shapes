mod dataset;
mod entry;
mod py;
mod shape;

pub use dataset::Dataset;
use ndarray::Array3;
pub use shape::ShapeType;

use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Debug)]
pub enum RangeOrSingle<T> {
    Range(T, T),
    Single(T),
}

pub fn ndarray_to_image(mut array: Array3<f64>, size: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    array.swap_axes(0, 2);
    array.swap_axes(0, 1);
    let mut array = array.as_standard_layout();
    array.mapv_inplace(|v| if v * 255. > 255. { 255. } else { v * 255. });
    let array = array.mapv(|v| v as u8);
    RgbImage::from_raw(size, size, array.to_owned().into_raw_vec())
        .expect("Failed to create image from raw array!")
}

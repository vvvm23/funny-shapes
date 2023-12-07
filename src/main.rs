// use ndarray::prelude::*;

mod lib;

use funnyshapes::{Color, Dataset, ShapeType};

use image::{Rgb, RgbImage};

fn main() {
    let dataset = Dataset::new(64)
        .shape_types(vec![ShapeType::Square, ShapeType::Circle])
        .color_palette(vec![
            Color::new(255, 0, 0),
            Color::new(0, 255, 0),
            Color::new(0, 0, 255),
        ])
        .size_range(0.1, 0.2)
        .position_range(0.0, 0.8)
        .velocity_range(-0.2, 0.2)
        .num_shapes_range(3, 7);

    println!("{:#?}", dataset);

    let num_to_generate = 10000;

    for i in 0..num_to_generate {
        let random_entry = dataset.generate_random_entry();
        // println!("{:#?}", random_entry);

        let size: u32 = 128;
        let mut arr = random_entry.render_entry(size as u16);
        arr.swap_axes(0, 2);
        arr.swap_axes(0, 1);
        let mut arr = arr.as_standard_layout();
        arr.mapv_inplace(|v| if v * 255. > 255. { 255. } else { v * 255. });

        let arr = arr.mapv(|v| v as u8);

        let img = RgbImage::from_raw(size, size, arr.to_owned().into_raw_vec())
            .expect("Failed to create image from raw array!");

        img.save(format!("outputs/test_{i}.png"))
            .expect("Failed to save image!");

        println!("{} / {num_to_generate} complete.", i + 1);
    }
}

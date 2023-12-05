// use ndarray::prelude::*;

mod lib;

use funnyshapes::{Color, Dataset, ShapeType};

fn main() {
    let dataset = Dataset::new(64)
        .shape_types(vec![ShapeType::Square, ShapeType::Circle])
        .color_palette(vec![
            Color::new(255, 0, 0),
            Color::new(0, 255, 0),
            Color::new(0, 0, 255),
        ])
        .size_range(0.2, 1.2)
        .position_range(0.1, 0.9)
        .velocity_range(-0.2, 0.2)
        .num_shapes_range(1, 5);

    println!("{:#?}", dataset);

    let random_entry = dataset.generate_random_entry();
    println!("{:#?}", random_entry);
}

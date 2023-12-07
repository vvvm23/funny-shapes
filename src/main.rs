// use ndarray::prelude::*;

use funnyshapes::ndarray_to_image;
use funnyshapes::{Dataset, ShapeType};

use std::time::Instant;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Config {
    #[arg(long, short)]
    num_to_generate: u32,

    #[arg(long, short)]
    size: u32,
}

fn main() {
    let args = Config::parse();
    let dataset = Dataset::new()
        .shape_types(vec![ShapeType::Square, ShapeType::Circle])
        .add_color(255, 0, 0)
        .add_color(0, 255, 0)
        .add_color(0, 0, 255)
        .size_range(0.1, 0.2)
        .position_range(0.0, 0.8)
        .velocity_range(-0.2, 0.2)
        .num_shapes_range(3, 7);

    println!("{:#?}", dataset);

    let start_time = Instant::now();
    let size = args.size;
    // 2650ms with circle, 183ms without
    // improved to 531ms with rayon
    for i in 0..args.num_to_generate {
        let random_entry = dataset.generate_random_entry();

        let array = random_entry.render_entry(size as u16);
        let img = ndarray_to_image(array, size);
        img.save(format!("outputs/test_{i}.png"))
            .expect("Failed to save image!");

        println!("{} / {} complete.", i + 1, args.num_to_generate);
    }
    let duration = start_time.elapsed();
    println!("Time taken: {duration:?}");
}

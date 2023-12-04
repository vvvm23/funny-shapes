// use ndarray::prelude::*;

mod lib;

use lib::{Shape, ShapeType};

fn main() {
    println!("Aloha, World!");
    let square = Shape::new(ShapeType::Square)
        .position(0.2, 0.5)
        .velocity(0.2, -0.1)
        .color(255, 0, 0);

    println!("{:?}", square);
}
use rand::seq::SliceRandom;
use rand::Rng;
use std::ops::Add;
use rand::distributions::uniform::SampleUniform;

use super::RangeOrSingle;

use ndarray::prelude::*;
use ndarray::Array3;
#[derive(Debug, Copy, Clone)]
pub enum ShapeType {
    Square,
    Circle,
}

pub trait NewRandom2<T: std::cmp::PartialOrd + SampleUniform + Copy> {
    fn new(x: T, y: T) -> Self;

    fn new_random(lower: T, upper: T) -> Self where Self: Sized{
        let x = rand::thread_rng().gen_range(lower..=upper);
        let y = rand::thread_rng().gen_range(lower..=upper);

        Self::new(x, y)
    }

    fn new_from_range_or_single(range_or_single: &RangeOrSingle<T>) -> Self where Self: Sized {
        match range_or_single {
            RangeOrSingle::Range(l, u) => Self::new_random(*l, *u),
            RangeOrSingle::Single(v) => Self::new(*v, *v)
        }
    }
}

pub trait NewRandom1 <T: std::cmp::PartialOrd + SampleUniform + Copy> {
    fn new(v: T) -> Self;

    fn new_random(lower: T, upper: T) -> Self where Self: Sized{
        let v = rand::thread_rng().gen_range(lower..=upper);

        Self::new(v)
    }

    fn new_from_range_or_single(range_or_single: &RangeOrSingle<T>) -> Self where Self: Sized {
        match range_or_single {
            RangeOrSingle::Range(l, u) => Self::new_random(*l, *u),
            RangeOrSingle::Single(v) => Self::new(*v)
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    pub fn new_random_from_palette(palette: &Vec<Color>) -> Self {
        *palette
            .choose(&mut rand::thread_rng())
            .expect("Failed picking random element from color palette")
    }

    pub fn to_ndarray(&self) -> Array3<f64> {
        array![
            [[(self.0 as f64) / 255.,]],
            [[(self.1 as f64) / 255.,]],
            [[(self.2 as f64) / 255.]]
        ]
    }
}
#[derive(Debug)]
pub struct Position(pub f64, pub f64);
impl NewRandom2<f64> for Position {
    fn new(x: f64, y: f64) -> Self {
        if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
            panic!("Specified position was out of range [0, 1]. Got {x},{y}");
        }

        Position(x, y)
    }

}

#[derive(Debug)]
pub struct Velocity(f64, f64);

impl NewRandom2<f64> for Velocity {
    fn new(x: f64, y: f64) -> Self {
        Velocity(x, y)
    }
}

impl Add<Velocity> for Position {
    type Output = Position;

    fn add(self, other: Velocity) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug)]
pub struct Size(pub f64);

impl NewRandom1<f64> for Size {
    fn new(size: f64) -> Self {
        Self(size)
    }
}

#[derive(Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub color: Color,
    pub position: Position,
    pub velocity: Velocity,
    pub size: Size,
}

impl Shape {
    pub fn new(shape_type: &ShapeType) -> Self {
        Shape {
            shape_type: *shape_type,
            color: Color::new(0xff, 0xff, 0xff),
            position: Position::new(0.0, 0.0),
            velocity: Velocity::new(0.0, 0.0),
            size: Size(1.0),
        }
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Position::new(x, y);
        self
    }

    pub fn velocity(mut self, x: f64, y: f64) -> Self {
        self.velocity = Velocity::new(x, y);
        self
    }

    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Color::new(r, g, b);
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.size = Size(size);
        self
    }
}

use std::collections::btree_map::Range;

use rand::seq::SliceRandom;
use rand::Rng;

use ndarray::Array3;
#[derive(Debug, Copy, Clone)]
pub enum ShapeType {
    Square,
    Circle,
}

#[derive(Debug, Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    fn new_random_from_palette(palette: &Vec<Color>) -> Self {
        *palette
            .choose(&mut rand::thread_rng())
            .expect("Failed picking random element from color palette")
    }
}
#[derive(Debug)]
struct Position(f64, f64);

impl Position {
    fn new(x: f64, y: f64) -> Self {
        if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
            panic!("Specified position was out of range [0, 1]. Got {x},{y}");
        }

        Position(x, y)
    }

    fn new_random(lower: f64, upper: f64) -> Self {
        let x = rand::thread_rng().gen_range(lower..=upper);
        let y = rand::thread_rng().gen_range(lower..=upper);

        Self::new(x, y)
    }
}

#[derive(Debug)]
struct Velocity(f64, f64);

impl Velocity {
    fn new(x: f64, y: f64) -> Self {
        Velocity(x, y)
    }

    fn new_random(lower: f64, upper: f64) -> Self {
        let x = rand::thread_rng().gen_range(lower..=upper);
        let y = rand::thread_rng().gen_range(lower..=upper);

        Self::new(x, y)
    }
}

#[derive(Debug)]
struct Size(f64);

impl Size {
    fn new(size: f64) -> Self {
        Self(size)
    }

    fn new_random(lower: f64, upper: f64) -> Self {
        let v = rand::thread_rng().gen_range(lower..=upper);
        Self::new(v)
    }
}

#[derive(Debug)]
pub struct Shape {
    shape_type: ShapeType,
    color: Color,
    position: Position,
    velocity: Velocity,
    size: Size,
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

#[derive(Debug)]
pub struct Entry {
    shapes: Vec<Shape>,
}

impl Entry {
    fn new(shapes: Vec<Shape>) -> Self {
        Entry { shapes }
    }

    fn new_empty_with_capacity(capacity: usize) -> Self {
        Entry {
            shapes: Vec::with_capacity(capacity),
        }
    }

    fn new_from_random(
        num_shapes: usize,
        shape_types: &Vec<ShapeType>,
        color_palette: &Vec<Color>,
        size_range: &RangeOrSingle<f64>,
        position_range: &RangeOrSingle<f64>,
        velocity_range: &RangeOrSingle<f64>,
    ) -> Self {
        let mut entry = Self::new_empty_with_capacity(num_shapes);

        let mut rng = rand::thread_rng();
        for _ in 0..num_shapes {
            let mut shape = Shape::new(
                shape_types
                    .choose(&mut rng)
                    .expect("Failed picking random shape from vec of possible shapes!"),
            );
            shape.color = Color::new_random_from_palette(&color_palette);

            // TODO: replace this stuff with traits doing such on ranges?
            shape.size = match size_range {
                RangeOrSingle::Range(l, u) => Size::new_random(*l, *u),
                RangeOrSingle::Single(v) => Size::new(*v),
            };

            shape.position = match position_range {
                RangeOrSingle::Range(l, u) => Position::new_random(*l, *u),
                RangeOrSingle::Single(v) => Position::new(*v, *v),
            };

            shape.velocity = match velocity_range {
                RangeOrSingle::Range(l, u) => Velocity::new_random(*l, *u),
                RangeOrSingle::Single(v) => Velocity::new(*v, *v),
            };

            entry.shapes.push(shape);
        }

        entry
    }

    fn render_square_at_coordinate(image: &mut Array3<f64>, shape: &Shape, size: usize) {
        let float_to_coord = |f: f64| (f * (size as f64)) as u16;
        // TODO: render square onto image
    }

    fn render_circle(image: &mut Array3<f64>, shape: &Shape, size: usize) {
        let float_to_coord = |f: f64| (f * (size as f64)) as u16;
        // TODO: render circle onto image
    }

    fn render_entry(&self, size: u16) -> Array3<f64> {
        let size = size as usize;
        let mut image = Array3::zeros((3, size, size));

        for shape in self.shapes.iter() {
            match shape.shape_type {
                ShapeType::Circle => Entry::render_circle(&mut image, shape, size),
                ShapeType::Square => Entry::render_square(&mut image, shape, size),
            };
        }
        image
    }
}

#[derive(Debug)]
enum RangeOrSingle<T> {
    Range(T, T),
    Single(T),
}

#[derive(Debug)]
pub struct Dataset {
    shape_types: Vec<ShapeType>,
    color_palette: Vec<Color>,
    size_range: RangeOrSingle<f64>,
    position_range: RangeOrSingle<f64>,
    velocity_range: RangeOrSingle<f64>,
    num_shapes_range: RangeOrSingle<usize>,

    image_size: usize,
}

impl Dataset {
    pub fn new(size: usize) -> Self {
        Self {
            shape_types: Vec::new(),
            color_palette: Vec::new(),
            size_range: RangeOrSingle::Single(1.0),
            position_range: RangeOrSingle::Range(0.0, 1.0),
            velocity_range: RangeOrSingle::Range(-0.5, 0.5),
            num_shapes_range: RangeOrSingle::Single(3),
            image_size: size,
        }
    }

    pub fn shape_types(mut self, shape_types: Vec<ShapeType>) -> Self {
        self.shape_types = shape_types;
        self
    }

    pub fn color_palette(mut self, color_palette: Vec<Color>) -> Self {
        self.color_palette = color_palette;
        self
    }

    pub fn size_range(mut self, size_lower: f64, size_upper: f64) -> Self {
        self.size_range = RangeOrSingle::Range(size_lower, size_upper);
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.size_range = RangeOrSingle::Single(size);
        self
    }

    pub fn position_range(mut self, position_lower: f64, position_upper: f64) -> Self {
        self.position_range = RangeOrSingle::Range(position_lower, position_upper);
        self
    }

    pub fn position(mut self, position: f64) -> Self {
        self.position_range = RangeOrSingle::Single(position);
        self
    }

    pub fn velocity_range(mut self, velocity_lower: f64, velocity_upper: f64) -> Self {
        self.velocity_range = RangeOrSingle::Range(velocity_lower, velocity_upper);
        self
    }

    pub fn velocity(mut self, velocity: f64) -> Self {
        self.velocity_range = RangeOrSingle::Single(velocity);
        self
    }

    pub fn num_shapes_range(mut self, num_shapes_lower: usize, num_shapes_upper: usize) -> Self {
        self.num_shapes_range = RangeOrSingle::Range(num_shapes_lower, num_shapes_upper);
        self
    }

    pub fn num_shapes(mut self, num_shapes: usize) -> Self {
        self.num_shapes_range = RangeOrSingle::Single(num_shapes);
        self
    }
}

impl Dataset {
    pub fn generate_random_entry(&self) -> Entry {
        let num_shapes: usize = match self.num_shapes_range {
            RangeOrSingle::Range(l, u) => rand::thread_rng().gen_range(l..=u),
            RangeOrSingle::Single(v) => v,
        };

        Entry::new_from_random(
            num_shapes,
            &self.shape_types,
            &self.color_palette,
            &self.size_range,
            &self.position_range,
            &self.velocity_range,
        )
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}

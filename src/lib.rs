use ndarray::Array3;
#[derive(Debug)]
pub enum ShapeType {
    Square,
    Circle,
    Triangle,
}

#[derive(Debug)]
struct Color(u8, u8, u8);

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self{
        Color(r, g, b)
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
}

#[derive(Debug)]
struct Velocity(f64, f64);

impl Velocity {
    fn new(x: f64, y: f64) -> Self {
        Velocity(x, y)
    }
}

#[derive(Debug)]
pub struct Shape {
    shape_type: ShapeType,
    color: Color,
    position: Position,
    velocity: Velocity,
    size: f64
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        Shape {
            shape_type: shape_type,
            color: Color::new(0xff, 0xff, 0xff),
            position: Position::new(0.0, 0.0),
            velocity: Velocity::new(0.0, 0.0),
            size: 1.0,
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
        self.size = size;
        self
    }
}

struct Entry {
    shapes: Vec<Shape>
}

impl Entry {
    fn render_entry(&self, size: u16) -> Array3<f64> {
        let size = size as usize;
        let mut image = Array3::zeros((3, size, size));

        // TODO: iterate over each shape and display it on image
        image
    }
}

enum RangeOrSingle<T> {
    Range(T, T),
    Single(T)
}

struct Dataset {
    shapes: Vec<ShapeType>,
    colors: Vec<Color>,
    size_range: RangeOrSingle<f64>,
    position_range: RangeOrSingle<f64>,
    velocity_range: RangeOrSingle<f64>,
    num_shapes_range: RangeOrSingle<u16>,

    image_size: usize,
}

impl Dataset {
    fn new(size: usize) -> Self {
        Self {
            shapes: Vec::new(),
            colors: Vec::new(),
            size_range: RangeOrSingle::Single(1.0),
            position_range: RangeOrSingle::Range(0.0, 1.0),
            velocity_range: RangeOrSingle::Range(-0.5, 0.5),
            num_shapes_range: RangeOrSingle::Single(3),
            image_size: size
        }
    }

    fn shapes(mut self, shapes: Vec<ShapeType>) -> Self {
        self.shapes = shapes;
        self
    }

    fn colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
        self
    }

    fn size_range(mut self, size_lower: f64, size_upper: f64) -> Self {
        self.size_range = RangeOrSingle::Range(size_lower, size_upper);
        self
    }

    fn size(mut self, size: f64) -> Self {
        self.size_range = RangeOrSingle::Single(size);
        self
    }


    fn position_range(mut self, position_lower: f64, position_upper: f64) -> Self{
        self.position_range = RangeOrSingle::Range(position_lower, position_upper);
        self
    }

    fn position(mut self, position: f64) -> Self {
        self.position_range = RangeOrSingle::Single(position);
        self
    }

    fn velocity_range(mut self, velocity_lower: f64, velocity_upper: f64) -> Self{
        self.velocity_range = RangeOrSingle::Range(velocity_lower, velocity_upper);
        self
    }

    fn velocity(mut self, velocity: f64) -> Self{
        self.velocity_range = RangeOrSingle::Single(velocity);
        self
    }

    fn num_shapes_range(mut self, num_shapes_lower: u16, num_shapes_upper: u16) -> Self{
        self.num_shapes_range = RangeOrSingle::Range(num_shapes_lower, num_shapes_upper);
        self
    }

    fn num_shapes(mut self, num_shapes: u16) -> Self {
        self.num_shapes_range = RangeOrSingle::Single(num_shapes);
        self
    }
}

impl Dataset {
    fn generate_random_entry(&self) -> Entry {
        Entry {shapes: vec![]}
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

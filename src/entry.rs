use rand::seq::SliceRandom;

use super::shape::{Color, Position, Shape, ShapeType, Size, Velocity};
use super::RangeOrSingle;

use ndarray::{Array3, AxisDescription, Slice, Zip};

#[derive(Debug)]
pub struct Entry {
    shapes: Vec<Shape>,
}

impl Entry {
    fn new_empty_with_capacity(capacity: usize) -> Self {
        Entry {
            shapes: Vec::with_capacity(capacity),
        }
    }

    pub fn new_from_random(
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

    fn render_square(image: &mut Array3<f64>, shape: &Shape, size: usize) {
        let float_to_coord = |f: f64| (f * (size as f64)) as isize;
        // TODO: render square onto image

        let x1 = float_to_coord(shape.position.0);
        let y1 = float_to_coord(shape.position.1);

        let size_int = float_to_coord(shape.size.0);

        let x2 = float_to_coord(shape.position.0) + size_int;
        let y2 = float_to_coord(shape.position.1) + size_int;

        let slice_fn = |f: AxisDescription| {
            let axis = f.axis.0;
            match axis {
                0 => Slice::new(0, None, 1),
                1 => Slice::new(y1, Some(y2 + 1), 1),
                2 => Slice::new(x1, Some(x2 + 1), 1),
                _ => panic!("Expected 3-d array but received more dimensions!"),
            }
        };
        let mut slice = image.slice_each_axis_mut(slice_fn);
        slice.assign(&shape.color.to_ndarray());
    }

    fn render_circle(image: &mut Array3<f64>, shape: &Shape, size: usize) {
        let coord_to_float = |f: usize| (f as f64) / (size as f64);

        // TODO: this might cause issues when we move shapes later
        let fix_center = |f: f64| ((f * (size as f64)) as usize) as f64 / (size as f64);

        let radius = shape.size.0 / 2.;
        let x_center = fix_center(shape.position.0 + radius);
        let y_center = fix_center(shape.position.1 + radius);

        // TODO: improve this? iterating over every pixel and channel
        Zip::indexed(image.view_mut()).par_map_collect(|(c, y, x), v| {
            let xf = coord_to_float(x);
            let yf = coord_to_float(y);

            let dist = ((xf - x_center).powf(2.0) + (yf - y_center).powf(2.0)).sqrt();

            if dist <= radius {
                *v = {
                    // TODO: this in particular sucks
                    let color = match c {
                        0 => shape.color.0,
                        1 => shape.color.1,
                        2 => shape.color.2,
                        _ => panic!("Index out of range"),
                    };
                    (color as f64) / 255.
                }
            };
        });

        // Entry::render_square_at_coordinate(image, shape, size);
    }

    pub fn render_entry(&self, size: u16) -> Array3<f64> {
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

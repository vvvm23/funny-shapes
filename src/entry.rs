// use std::intrinsics::offset;

use rand::seq::SliceRandom;
// use rayon::iter::IntoParallelIterator;
// use rayon::iter::IntoParallelRefMutIterator;
// use rayon::iter::ParallelIterator

use super::shape::{Color, NewRandom1, NewRandom2, Position, Shape, ShapeType, Size, Velocity};
use super::RangeOrSingle;

use ndarray::{array, Array1, Array2, Array3, Axis, AxisDescription, Slice, Zip};

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
            shape.size = Size::new_from_range_or_single(size_range);
            shape.position = Position::new_from_range_or_single(position_range);
            shape.velocity = Velocity::new_from_range_or_single(velocity_range);

            entry.shapes.push(shape);
        }

        entry
    }

    fn render_square(image: &mut Array3<f64>, shape: &Shape, size: usize) {
        let float_to_coord = |f: f64| (f * (size as f64)) as isize;

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

        // TODO: implement the below numpy code
        // np.sqrt(((np.mgrid[0:size, 0:size] / size - np.array([[[x_center]], [[y_center]]]))**2).sum(axis=0)) < radius

        // let coords: Array1<f64> = Array1::linspace(0.0, 1.0, size);
        // let coords = coords.broadcast((size, size)).expect("Failed broadcasting coordinates to 2d grid.");
        // let coords = ndarray::stack(Axis(0), &[coords, coords.t()]).expect("Failed to stack transposed coordinates!")

        // let offset_coords = coords - array![y_center, x_center].broadcast((2, size, size)).unwrap();
        // let distances: Array2<f64> = (offset_coords * offset_coords).sum_axis(Axis(0)).into_par_iter().map(|v| v.sqrt()).collect();
        // let distances = distances.broadcast(image.shape());
        // TODO: how to set image based on these distances?

        // TODO: improve this? iterating over every pixel and channel
        Zip::indexed(image.view_mut()).par_map_collect(|(c, y, x), v| {
            let xf = coord_to_float(x);
            let yf = coord_to_float(y);

            let xd = xf - x_center;
            let yd = yf - y_center;

            let dist = (xd * xd + yd * yd).sqrt();

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

    pub fn step_entry(&mut self, step_size: f64) {
        for shape in self.shapes.iter_mut() {
            shape.step_shape(step_size);
        }
    }
}

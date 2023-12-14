use super::entry::Entry;
use super::shape::{Color, ShapeType};
use super::RangeOrSingle;
use ndarray::{s, Array3, Array4, Axis};

use rand::Rng;
#[derive(Debug)]
pub struct Dataset {
    shape_types: Vec<ShapeType>,
    color_palette: Vec<Color>,
    size_range: RangeOrSingle<f64>,
    position_range: RangeOrSingle<f64>,
    velocity_range: RangeOrSingle<f64>,
    num_shapes_range: RangeOrSingle<usize>,
    // TODO: background color palette
    // TODO: weightings for picking shapes / colors
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            shape_types: Vec::new(),
            color_palette: Vec::new(),
            size_range: RangeOrSingle::Single(1.0),
            position_range: RangeOrSingle::Range(0.0, 1.0),
            velocity_range: RangeOrSingle::Range(-0.5, 0.5),
            num_shapes_range: RangeOrSingle::Single(3),
        }
    }

    pub fn shape_types(mut self, shape_types: Vec<ShapeType>) -> Self {
        self.shape_types = shape_types;
        self
    }

    pub fn add_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color_palette.push(Color::new(r, g, b));
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

    pub fn get_random_image_array(&self, size: u16) -> Array3<f64> {
        let entry = self.generate_random_entry();
        entry.render_entry(size)
    }

    pub fn get_random_video_array(
        &self,
        num_frames: usize,
        size: u16,
        step_size: f64,
    ) -> Array4<f64> {
        let mut entry = self.generate_random_entry();

        let mut video_array = Array4::zeros((num_frames as usize, 3, size as usize, size as usize));
        for i in 0..num_frames {
            let array = entry.render_entry(size);
            entry.step_entry(step_size);

            video_array
                .slice_mut(s![i..i + 1, .., .., ..])
                .assign(&array);
        }

        video_array
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

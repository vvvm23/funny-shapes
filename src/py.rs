use numpy::{IntoPyArray, PyArray3};
use pyo3::types::PyTuple;
use pyo3::{pyclass, pymethods, pymodule, types::PyModule, PyResult, Python};

use crate::{Dataset, ShapeType};
use ndarray::Array3;

#[pymodule]
fn funnyshapes<'py>(_py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    fn test() {
        println!("called Rust from Python!");
    }

    #[pyfn(m)]
    #[pyo3(name = "test")]
    fn test_py<'py>(_py: Python<'py>) {
        test();
    }

    fn get_random_frame(size: u16) -> Array3<f64> {
        let dataset = Dataset::new()
            .shape_types(vec![ShapeType::Square, ShapeType::Circle])
            .add_color(255, 0, 0)
            .add_color(0, 255, 0)
            .add_color(0, 0, 255)
            .size_range(0.1, 0.2)
            .position_range(0.0, 0.8)
            .velocity_range(-0.2, 0.2)
            .num_shapes_range(3, 7);

        let entry = dataset.generate_random_entry();
        entry.render_entry(size)
    }

    #[pyfn(m)]
    #[pyo3(name = "get_random_frame")]
    fn get_random_frame_py<'py>(py: Python<'py>, size: u16) -> &'py PyArray3<f64> {
        let array = get_random_frame(size);
        array.into_pyarray(py)
    }

    #[pyclass(name = "FunnyShapesDataset")]
    struct PyDataset {
        inner: Dataset,
    }

    impl PyDataset {
        fn get_random_frame(&self, size: u16) -> Array3<f64> {
            let entry = self.inner.generate_random_entry();
            entry.render_entry(size)
        }
    }

    #[pymethods]
    impl PyDataset {
        #[new]
        fn new(
            colors: Vec<(u8, u8, u8)>,
            size_range: (f64, f64),
            position_range: (f64, f64),
            num_shapes_range: (usize, usize),
        ) -> Self {
            let (size_lower, size_upper) = size_range;
            let (position_lower, position_upper) = position_range;
            let (shapes_lower, shapes_upper) = num_shapes_range;
            let mut dataset = Dataset::new()
                .shape_types(vec![ShapeType::Square, ShapeType::Circle])
                .size_range(size_lower, size_upper)
                .position_range(position_lower, position_upper)
                .velocity_range(-0.2, 0.2)
                .num_shapes_range(shapes_lower, shapes_upper);

            for (r, g, b) in colors {
                dataset = dataset.add_color(r, g, b);
            }

            PyDataset { inner: dataset }
        }

        #[pyo3(name = "get_random_frame")]
        fn get_random_frame_py<'py>(&self, py: Python<'py>, size: u16) -> &'py PyArray3<f64> {
            let array = self.get_random_frame(size);
            array.into_pyarray(py)
        }
    }

    m.add_class::<PyDataset>()?;
    Ok(())
}

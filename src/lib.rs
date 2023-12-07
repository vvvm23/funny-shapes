mod dataset;
mod entry;
mod shape;

#[derive(Debug)]
pub enum RangeOrSingle<T> {
    Range(T, T),
    Single(T),
}

pub use dataset::Dataset;
pub use shape::ShapeType;

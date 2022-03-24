use crate::point::Point;

mod tri;
pub use tri::Tri;

mod polygon;
use polygon::Polygon;

mod image;
pub use image::{
    AxisResolution, BreedMetadata, FitnessMetadata, GAImageMember, GridImage, Resolution,
};

pub use crate::colour::Colour;

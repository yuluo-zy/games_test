//! Shared geometry, curves, sampling, compact storage and filesystem utilities.
//!
//! Numerical policies and input requirements are documented at their API boundaries.
//! Historical provenance and validation scope are recorded in the package README.

pub mod angle_range;
pub mod camera;
pub mod circular_f32;
pub mod comparative_space;
pub mod components;
pub mod curve;
pub mod easing_functions;
pub mod file;
pub mod float_grid;
pub mod geometry;
pub mod halver;
pub mod index_condenser;
pub mod iso_transform;
pub mod local_time;
pub mod loop_range;
pub mod name_hash;
pub mod natural_cubic;
pub mod rdp;
pub mod rgb9e5;
pub mod rng;
pub mod selective_deserialize;
pub mod subtractive_1d_boolean;
pub mod swizzlers;
pub mod threshold_range;
pub mod timer;
pub mod wall_space;
pub use geometry::rotation_from_normal;
pub use iso_transform::IsoTransform;

//! GPU candidate selection, driver advice and initialization error display.
mod advice;
mod controls;
mod errors;
mod model;
#[cfg(feature = "native-gui")]
mod native;
pub use advice::*;
pub use controls::*;
pub use errors::*;
pub use model::*;

#[cfg(test)]
#[path = "gpu_selection/policy_tests.rs"]
mod policy_tests;

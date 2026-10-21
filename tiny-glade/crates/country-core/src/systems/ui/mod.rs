//! Complete widget geometry, real hover picking and captured input ownership.
//! Host registration preserves native set6 ordering; signifier integration has
//! separate outstanding factory producer dependencies.
pub mod hover_intersection;
pub mod ui_update;
pub mod resources;
pub mod signifiers;

pub use signifiers::signifier_create::UiSignifierStream;

//! Roof geometry recovered from the original binary, hosted in country-core.
//!
//! These modules were previously factored into the standalone `system-roof`
//! crate; the PDB source paths (`crates/country-core/src/resources/roofs/*.rs`)
//! and the original dependency direction (systems/roof consumes country-core)
//! place them here. The original `Roof` struct and its surface/ridge math lived
//! in `resources/roofs/roof_shape.rs`; the reconstruction keeps the byte
//! projections in the sibling wrapper modules and the algorithm closures here.
pub mod edit_rules;
pub mod parameters;
pub mod pipeline;
pub mod rectangular_caps;
pub mod rectangular_edges;
pub mod rectangular_faces;
pub mod replay;
pub mod ridge;
pub mod surface;
pub mod tiles;

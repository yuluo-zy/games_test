//! Standalone roof geometry crate (reconstruction evidence base).
//!
//! Modules mirror the recovered `system-roof` geometry chain. The workspace
//! copy lives in `crates/country-core/src/resources/roofs/geometry/` (original
//! PDB source paths `crates/country-core/src/resources/roofs/*.rs`); this crate
//! keeps the same files as an isolated, compilable validation base.
//! Reviewer C: "复用旧实现 O×11 已确认零语义不一致（edit_rules/surface/ridge/
//! tiles/edges/caps/faces/replay 全分支核对过）——这 11 文件可直接作为 roof
//! crate 基础。"
#![allow(clippy::too_many_arguments)]

#[path = "../edit_rules.rs"]
pub mod edit_rules;
#[path = "../parameters.rs"]
pub mod parameters;
#[path = "../pipeline.rs"]
pub mod pipeline;
#[path = "../rectangular_caps.rs"]
pub mod rectangular_caps;
#[path = "../rectangular_edges.rs"]
pub mod rectangular_edges;
#[path = "../rectangular_faces.rs"]
pub mod rectangular_faces;
#[path = "../replay.rs"]
pub mod replay;
#[path = "../ridge.rs"]
pub mod ridge;
#[path = "../roof_state.rs"]
pub mod roof_state;
#[path = "../surface.rs"]
pub mod surface;
#[path = "../tiles.rs"]
pub mod tiles;

//! `country_core::systems::render_loop` — WP19 misc/stylized modules.
//!
//! Skeleton root: `crates/country-core/src/systems/render_loop/`. WP5 owns the
//! frame wiring (`mod.rs`, `compute_passes/`, `draw_passes/`). This crate adds
//! the stylized / boot / LUT-gallery / indirect-args modules (WP19). The owner
//! integrates by declaring these submodules in the shared `render_loop/mod.rs`:
//!
//! ```text
//! pub mod painterly;
//! pub mod vermeer;
//! pub mod menu_background;
//! pub mod session_selection;
//! pub mod lut_gallery;
//! pub mod indirect_params;
//! pub mod resources;   // -> resources/critters.rs (SSBO markers)
//! ```

pub mod indirect_params;
pub mod lut_gallery;
pub mod menu_background;
pub mod painterly;
pub mod resources;
pub mod session_selection;
pub mod vermeer;

//! `resources` — game CPU-side state for the WP16 misc domain.
//!
//! Extends the wp6/wp7 placeholders with the residual families recovered here:
//! ivy growth state, the software-BVH ray-tracing world, window-lights /
//! color-id CPU state. Contracts: `contracts/ivy.md`, `contracts/rt.md`,
//! `contracts/window-lights.md`.

pub mod color_id;
pub mod ivy;
pub mod rt;
pub mod window_lights;

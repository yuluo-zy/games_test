//! Evidence-backed critter business recovery. B baseline only unless stated.
pub mod firefly;
pub mod sheep_pet;
// Behaviour-chain drafts (disassembly evidence, 2026-10-13). Standalone bevy-free
// semantic cores; system wrappers need bevy/country-core deps not in this harness.
#[path = "src/duck.rs"]
pub mod duck;
#[path = "src/shared.rs"]
pub mod shared;
#[path = "src/sheep_terrain.rs"]
pub mod sheep_terrain;
// Extra behaviour-chain drafts (disassembly evidence, 2026-10-13). See
// evidence/multi-baseline/critters-extra-analysis.md.
#[path = "src/wren_boids.rs"]
pub mod wren_boids;
#[path = "src/fish.rs"]
pub mod fish;
#[path = "src/butterfly.rs"]
pub mod butterfly;

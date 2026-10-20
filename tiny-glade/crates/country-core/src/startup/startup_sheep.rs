//! B startup_sheep140a093e0 asset half. Complete ECS birth and audio are
//! system_critters::startup_sheep::spawn_startup_sheep (cross-crate owner).
//! Do not pre-register a bare frame1 mesh: the original mesh cache hit skips
//! animation buffer creation, so a mesh-only entry poisons the real startup.
pub use super::sheep_assets::{
    SheepStartupAssets, build_sheep_animation, load_sheep_startup_assets,
};
pub const SHEEP_BAAH_EVENT: &str = "event:/SFX/Sheep/Baah";
pub const SHEEP_PITCH_PARAMETER: &str = "Pitch";

pub mod height_rules;
pub mod decorator;
pub mod decorator_archivist;
pub mod decorator_storage;
pub mod decorator_subtype;
pub mod cached_decorator_transforms;
pub mod arches;
pub mod curved_halftimber;
pub mod elevation_support_stats;
pub mod material_stats;
pub mod plaster;
pub mod wall_path_segment;
pub mod wall_stone_columns;
pub mod wooden_gates;
pub mod inner_wall_state;
pub mod inner_walls;
pub mod public_walls;
pub mod public_wall_state;
pub mod wall_holes;
pub mod wall_trigger_volume;

pub use inner_wall_state::WallState;
pub use inner_walls::{InnerWalls, WallChanges};

/// B eraser multiplier read as one f32. Startup policy remains with its owner.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct EraserSizeMultiplier(pub f32);

/// Wall identity (B 0x140b34f20): zero is invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WallId(pub u64);
impl WallId {
    pub fn get_inner(self) -> u64 {
        self.0
    }
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// Only the highlighter fields consumed by the recovered render callbacks.
/// This is a safe Rust input representation, not a claim of full original ABI.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default)]
pub struct WallHighlight {
    pub wall_entity: Option<bevy_ecs::entity::Entity>,
    pub wall_id: Option<WallId>,
    pub ty: u8,
}

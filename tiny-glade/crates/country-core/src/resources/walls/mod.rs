pub mod height_rules;

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

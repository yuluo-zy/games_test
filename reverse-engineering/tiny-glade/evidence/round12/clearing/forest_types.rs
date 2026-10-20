// Candidate replacement of the forest types in resources/tree_storage.rs.
// Existing unrelated TreeStorage/event declarations remain untouched.

/// B InteractableForestTree is a24-byte animation component. It refers back
/// to the canonical80-byte ForestTree record; it does not own that record.
#[repr(C)]
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct InteractableForestTree {
    pub placement_index: usize,
    pub amplitude: f32,
    pub progress: f32,
    pub delay: f32,
}

/// Native ForestTree,80 bytes: PrefabName@0, placement@10, Entity@40,
/// billboard expansion@48. The final four bytes are alignment padding.
/// init_clearing clones the full48-byte placement and preserves every channel.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ForestTree {
    pub prefab_name: crate::resources::render::mesh_atlas_library::PrefabName,
    pub placement: crate::resources::placement_data::PlacementDataSsbo,
    pub entity: Option<bevy_ecs::entity::Entity>,
    pub billboard_expansion_scale: f32,
}

#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct InteractableForestPlacements {
    pub records: Vec<ForestTree>,
    pub processed: bool,
}

impl InteractableForestPlacements {
    pub fn clear(&mut self) {
        self.records.clear();
        self.processed = false;
    }
}

#[cfg(test)]
mod forest_native_layout {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn placement_owner_and_animation_component_are_distinct_native_types() {
        assert_eq!(size_of::<ForestTree>(), 80);
        assert_eq!(offset_of!(ForestTree, prefab_name), 0);
        assert_eq!(offset_of!(ForestTree, placement), 0x10);
        assert_eq!(offset_of!(ForestTree, entity), 0x40);
        assert_eq!(offset_of!(ForestTree, billboard_expansion_scale), 0x48);
        assert_eq!(size_of::<InteractableForestTree>(), 24);
        assert_eq!(offset_of!(InteractableForestTree, placement_index), 0);
        assert_eq!(offset_of!(InteractableForestTree, amplitude), 8);
        assert_eq!(offset_of!(InteractableForestTree, progress), 12);
        assert_eq!(offset_of!(InteractableForestTree, delay), 16);
    }
}

//! systems::ivy — 常春藤系统族（WP16 恢复）。

#![allow(dead_code)]

pub mod ivy_branch_thickness_animation;
pub mod ivy_grower;
pub mod ivy_leaf_animation;
pub mod ivy_leaf_spawner;
pub mod ivy_pruner;
pub mod ivy_send_to_gpu_leaf;
pub mod ivy_send_to_gpu_segments;
pub mod ivy_spawner;

// 插件（O）：`impl$0::add_ivy_systems`（0x140944070）/ `insert_ivy_resources`
// （0x1409440c0）/ `add_ivy_events`（0x140944280）。
// `insert_ivy_resources` 内联构造 `IvyStorage::new` / `IvyLeafStorage::new` /
// `IvySpawnNoise::new` / `IvyDirectionProposer`。

/// 常春藤插件（O：`IvyPlugin`）。
pub struct IvyPlugin;

impl IvyPlugin {
    pub fn build(&self, world: &mut bevy_ecs::world::World) {
        world.insert_resource(crate::resources::ivy::IvyStorage::new());
        world.insert_resource(crate::resources::ivy::IvyLeafStorage::default());
        world.insert_resource(crate::resources::ivy::IvyDirectionProposer::new());
    }

    /// `add_ivy_systems`（O 0x140944070）。
    pub fn add_ivy_systems(&self, _world: &mut bevy_ecs::world::World) {}

    /// `insert_ivy_resources`（O 0x1409440c0）。
    pub fn insert_ivy_resources(&self, _world: &mut bevy_ecs::world::World) {}

    /// `add_ivy_events`（O 0x140944280）。
    pub fn add_ivy_events(&self, _world: &mut bevy_ecs::world::World) {}
}

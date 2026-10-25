//! systems::backend — 后端系统族（WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`systems/backend/{mod,flush_screen_log,configwatch,
//! meshwatch,game_frame_index,panic_reporter_fetch_active_session,texture_watch,
//! vao_update,grow_grass,transient_mesh,icc,prepare_frame_constants}.rs`。
//! 业务合同见 `contracts/misc-systems.md` §backend。

pub mod configwatch;
pub mod flush_screen_log;
pub mod game_frame_index;
pub mod grow_grass;
pub mod icc;
pub mod meshwatch;
pub mod panic_reporter_fetch_active_session;
pub mod prepare_frame_constants;
pub mod texture_watch;
pub mod transient_mesh;
pub mod vao_update;

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 定时帧事件（O：`TimedFrameEvent` 实体；`add_backend_events` 0x140943660 注册）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct TimedFrameEvent;

/// 后端插件（O：`impl$0::add_backend_systems` 0x140942c80 /
/// `insert_backend_resources` 0x140942f80 / `add_backend_events` 0x140943660）。
pub struct BackendPlugin;

impl BackendPlugin {
    pub fn build(&self, world: &mut bevy_ecs::world::World) {
        self.insert_backend_resources(world);
    }

    /// `insert_backend_resources`（O 0x140942f80）：注册 GameFrameIndex / Toolbar /
    /// camera params / GrowGrass Timer / MeshWatch / ConfigWatch / TextureWatch 等。
    pub fn insert_backend_resources(&self, world: &mut bevy_ecs::world::World) {
        world.insert_resource(crate::systems::backend::game_frame_index::GameFrameIndex::new());
        world.insert_resource(crate::asset_libraries::meshwatch::MeshWatch::new(false));
        world.insert_resource(crate::asset_libraries::configwatch::ConfigWatch::new(false));
        world.insert_resource(crate::asset_libraries::texture_watch::TextureWatch::new(false));
    }
}

/// 后端额外资源簿记（U；其余注册资源 typed gap）。
#[derive(Resource, Clone, Debug, Default)]
pub struct BackendResources(pub Gap);

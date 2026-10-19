//! system-roof：屋顶系统的 ECS 层（create/edit/delete/replay、触发体、UI 编辑与瓦片装配）。
//!
//! 原始布局（PDB 源路径 `crates/systems/roof/src/lib.rs`、`utils/components.rs`）：
//! 屋顶几何（surface/ridge/tiles/parameters/edit_rules/pipeline/replay/rectangular_*）
//! 归属 country-core `resources/roofs/geometry/`（原 `resources/roofs/roof_shape.rs`
//! 等），本 crate 只含消费 core 屋顶/墙权威的系统。依赖方向 = 原 B：
//! `system_roof → country_core`。
//!
//! 主要系统注册入口：
//! - `add_roof_systems`（0x14062fcb0）：把屋顶系统接入 Bevy 调度。
//! - `add_roof_events_and_resources`（0x1406305f0）：事件/资源注册
//!   （委托 country-core `resources::roofs::register_roof_events`）。
pub mod create_roof;
pub mod deferred_record_roof_history;
pub mod delete_roof;
pub mod edit_roof;
pub mod highlight_roof;
pub mod replay_roof;
pub mod roof_trigger_volume;
pub mod simulate_roof_tile_for_audio;
pub mod ui_edit_roof;
pub mod ui_edit_roof_ridge_dir;
pub mod ui_edit_roof_ridge_length;
pub mod ui_edit_roof_tip;
pub mod ui_focused;
pub mod update_roof_spatial;
pub mod utils;
pub mod visual;
pub mod wall_height_dt;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;

/// 原 `system_roof::impl$0::add_roof_events_and_resources`（0x1406305f0）。
/// 事件/资源的初始化委托 country-core 的注册面（事件类型属于 core）。
pub fn add_roof_events_and_resources(world: &mut World) {
    country_core::resources::roofs::register_roof_events(world);
    // system_roof 自有事件（PDB：system_roof::ui_edit_roof_ridge_dir 域）。
    world.init_resource::<bevy_ecs::event::Events<ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>>();
    world.init_resource::<bevy_ecs::event::Events<ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>>();
}

/// 原 `system_roof::impl$0::add_roof_systems`（0x14062fcb0）。
/// 屋顶系统按原调度阶段接入。历史回放系统加入
/// `HistoryCpuStage::Replay`，其余系统链入 `WallCpuStage::RoofDependencies`
/// 与 `HistoryCpuStage::Apply`（与 country-core 的 add_history_cpu_systems /
/// add_wall_cpu_systems 配合，由宿主 App 组装）。
pub fn add_roof_systems(schedule: &mut Schedule) {
    use country_core::{
        systems::history::HistoryCpuStage,
        systems::wall::registration::WallCpuStage,
    };
    // 注意：`replay_roof::process`（0x1421b48e0）是 ReplayAction 方法
    // （非 SystemParam 面），由 replay_roof 内部调用，不入调度。
    schedule.add_systems(replay_roof::replay_roof.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(
        (
            create_roof::create_roof,
            create_roof::wall_roof_hashmap,
            update_roof_spatial::update_roof_spatial,
            edit_roof::edit_roof,
            edit_roof::switch_roof_direction,
            edit_roof::set_flat_roof_style,
            edit_roof::set_flat_roof_rim_style,
            visual::switch_roof_type::switch_roof_type,
            delete_roof::process_invalid_roof_notifications,
            delete_roof::mark_roof_for_deletion,
            wall_height_dt::wall_height_dt,
            visual::roof_animation::roof_animation,
            delete_roof::delete_roofs,
            update_roof_spatial::update_wall_gable_heights,
            // 本批重建函数补注册（reviewer FIX-4，原 add_roof_systems
            // 0x14062fcb0 注册：roof_record_history/record_switch_roof_ridge_history/
            // switch_roof_ridge_direction）。
            ui_edit_roof_ridge_dir::switch_roof_ridge_direction,
            ui_edit_roof_ridge_dir::record_switch_roof_ridge_history,
            deferred_record_roof_history::roof_record_history,
        )
            .chain()
            .in_set(WallCpuStage::RoofDependencies)
            .in_set(HistoryCpuStage::Apply),
    );
    schedule.add_systems(
        (
            roof_trigger_volume::calculate_roof_trigger_volumes,
            visual::assemble_roof_tiles::assemble_roof_tiles,
            visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom,
            visual::place_spires::place_roof_spires,
            simulate_roof_tile_for_audio::simulate_roof_tile_for_audio,
        )
            .chain()
            .in_set(WallCpuStage::RoofDependencies),
    );
}

/// 屋顶 SSBO 缓冲刷新（B `lib.rs::gpu_flush_buffers` 0x140de50a0）。
///
/// fresh 反编译（2026-10-17 consume-types，`work/consume-types/decomp/0x140de50a0`）：
/// `AssetSsboLibrary::get_mut<RoofTileSsbo>()` → `None` → `Option::unwrap` panic →
/// `GraphicsDeviceResource::deref()`（`&dyn GraphicsDevice`）→
/// `SparseInstanceBuffer<RoofTileSsbo>::gpu_flush` → 写 ResMut 变更 tick
/// （`in_RCX+0x150`；Bevy `ResMut` 的 `deref_mut` 自动置 tick，等价）。
/// panic 串：`"country_core::asset_libraries::ssbo_library::AssetSsboLibrary"`（0x3d B）
/// 与 `"country_core::render::GraphicsDeviceResourceResource requested by  does not exist: "`
/// （0x2c B）——由 Bevy 资源参数面生成，不手写。
pub fn gpu_flush_buffers(
    device: bevy_ecs::prelude::Res<country_core::render::device::GraphicsDeviceResource>,
    mut library: bevy_ecs::prelude::ResMut<
        country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    >,
) {
    use country_core::resources::roofs::roof_tile_ssbo::RoofTileSsbo;

    // O：get_mut<RoofTileSsbo> → None 时 unwrap_failed。
    let buffer = library.get_mut::<RoofTileSsbo>().unwrap();
    // O：GraphicsDeviceResource::deref() → SparseInstanceBuffer::gpu_flush<T>。
    buffer.gpu_flush(device.device());
}

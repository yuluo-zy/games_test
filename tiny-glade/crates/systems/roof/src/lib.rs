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

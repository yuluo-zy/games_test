//! system-wall-constructor：主墙构造系统（拱/檐梁/围栏/门/阶梯/支柱/石膏/半木构）。
//!
//! 原始布局（PDB 源路径 `crates/systems/wall-constructor/src/lib.rs`）：
//! 本 crate 消费 country-core 的墙权威（PublicWalls/PublicWallState 0x2a0
//! 投影、共享建筑策略链）并生产 96B BrickTransformSsbo /
//! 64B WoodenPlankSsbo 等实例记录。SSBO/渲染上传（FUN_141209c40 注册表 +
//! 虚调取缓冲 + Arc/condvar 握手）为渲染边界后置登记。
//!
//! 注册入口 `add_wall_constructor_systems` 按原调度阶段把系统接入 Bevy
//! 调度（`WallCpuStage::Bricks`/`Holes` 等 country-core 阶段集）。
pub mod arch_walker;
pub mod construct_arches;
pub mod construct_elevation_supports;
pub mod construct_fences;
pub mod construct_floors;
pub mod construct_gates;
pub mod construct_lintels;
pub mod construct_platform_stairs;
pub mod construct_playermade_stairs;
pub mod construct_walls;
pub mod curved_halftimber;
pub mod init_wall_height_and_arch_segment_height;
pub mod place_pillars;
pub mod plaster_systems;
pub mod segmentation;
pub mod ui_arch_controls;
pub mod ui_lintel_controls;
pub mod utils;
pub mod wall_constructor;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;

/// 原 `system_wall_constructor::impl$N` 注册（按原调度阶段）。
/// GPU 直写系统（gpu_clear_wall_gate_pieces 0x140773c00 /
/// gpu_flush_plank_buffers 0x140774010）在 render 边界接线后加入
/// （当前登记为结构桩，见 lib.rs 各函数注释）。
pub fn add_wall_constructor_systems(schedule: &mut Schedule) {
    use country_core::systems::wall::registration::WallCpuStage;
    schedule.add_systems(
        (
            init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height,
            segmentation::calculate_wall_path_segmentation::calculate_wall_path_segmentation,
            segmentation::sample_wall_path_intersections::sample_wall_path_intersections,
            segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs,
        )
            .chain()
            .in_set(WallCpuStage::Edit),
    );
    schedule.add_systems(
        (
            construct_walls::construct_walls,
            construct_arches::construct_arches,
            construct_lintels::construct_lintels,
            construct_fences::construct_fences,
            construct_gates::systems::construct_gates,
            construct_gates::systems::update_gates_hashmap,
            place_pillars::place_pillars,
            curved_halftimber::curved_halftimber_maintain_entities,
            construct_platform_stairs::construct_platform_stairs,
            construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::<true>,
            construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals,
            construct_elevation_supports::construct_circle_supports,
            construct_elevation_supports::construct_rectangle_columns,
            construct_elevation_supports::construct_rectangle_brackets,
        )
            .chain()
            .in_set(WallCpuStage::Bricks),
    );
    schedule.add_systems(
        (
            plaster_systems::maintain_plaster,
            plaster_systems::create_plaster,
        )
            .chain()
            .in_set(WallCpuStage::Holes),
    );
}

/// 原 `system_wall_constructor::gpu_flush_plank_buffers`（0x140774010）：
/// 木板 SSBO 缓冲刷写。**未恢复（壳登记）**：机器码已反编译（decomp-
/// 0x140774010），SSBO 直写体待 render 边界接线（不称已恢复）。
pub fn gpu_flush_plank_buffers() {}

/// 原 `system_wall_constructor::gpu_clear_wall_gate_pieces`（0x140773c00）：
/// 门片 GPU 清理。**未恢复（壳登记）**：同 render 边界后置。
pub fn gpu_clear_wall_gate_pieces() {}

/// 原 `system_wall_constructor::debug_force_update_arches`（0x14003bb80）：
/// B 中该符号为 **1B `ret` 存根**（C3+0xCC 填充，无真实业务）；空函数体与之
/// **判等**（非"未恢复"）。若 A/C 版有真实语义则需另行恢复。
pub fn debug_force_update_arches() {}

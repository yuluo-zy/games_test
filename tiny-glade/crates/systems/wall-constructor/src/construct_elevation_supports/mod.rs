//! 高程支撑构造（construct_elevation_supports/mod.rs）。
//!
//! 5 系统（fresh decomp 全落盘）：
//! - `overhang_subscribe_to_tabloids`（0x140786440）
//! - `detect_overhang_support_segments`（0x1407867d0）
//! - `construct_circle_supports`（0x140788160）
//! - `construct_rectangle_columns`（0x140788a80）
//! - `construct_rectangle_brackets`（0x1407896b0）
use bevy_ecs::prelude::*;

pub mod circle_bottom_layer;
pub mod circle_categorize;
pub mod circle_pillars;
pub mod circle_turret;
pub mod find_overlap_segments;
pub mod maintain_stone_column_colliders;
pub mod rectangle_bottom_layer;
pub mod rectangle_large_brackets;
pub mod rectangle_pillars;
pub mod rectangle_small_brackets;
pub mod util_cross_brick_pillar;
pub mod util_pillar_construction;

/// B `OverhangSupportDirtySubscriptions`（检测订阅资源）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct OverhangSupportDirtySubscriptions {
    pub dirty: hashbrown::HashMap<u64, u32>,
}

/// B `overhang_subscribe_to_tabloids`（0x140786440）：订阅悬挑支撑 tabloid。
/// **未恢复（壳登记）**：tabloid subscribe 接线待 render/宿主装配层（不称已恢复）。
pub fn overhang_subscribe_to_tabloids() {}

/// B `detect_overhang_support_segments`（0x1407867d0，4258B）。**未实现**：
/// fresh 已取（`evidence/elevsys-0x1407867d0.c`）；真实为 tabloid 脏订阅 →
/// `find_overhang_support_segments`（43KB）→ ElevationSupports 写面，
/// fVar4 判据 / DirtyTiles AABB 过滤未闭合。旧壳循环（调用空输入）已移除。
pub fn detect_overhang_support_segments(
    _subscriptions: ResMut<OverhangSupportDirtySubscriptions>,
    _cache: ResMut<
        crate::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache,
    >,
) {
    todo!(
        "未实现: detect_overhang_support_segments 0x1407867d0（tabloid 脏订阅→43KB 悬挑段→ElevationSupports 写面）"
    )
}

/// B `construct_circle_supports`（0x140788160，2305B / 544 行 fresh）。
///
/// 已解结构（`work/restorer-remaining-wallconstructor/evidence/
/// elevsys-0x140788160.c`）：
/// - 参数：`EventReader<RecalculateCircleSupports>`（事件 stride 0x28，
///   wall id @+0x18）、`Res<PublicWalls>`、以及 stack 上的
///   `ElevationSupports` 类 map / 输出记录 Vec / 一个 f32 写面；
/// - 逐事件 `PublicWalls` 探测（墙记录 stride 0x2a8）→
///   `PublicWallState::is_circle()` 且字段 == 1 才处理；
/// - `StoneColumnCollisionStorage::clear_and_get_mut()`；
/// - `circle_categorize()`：**false → `construct_turret()`**；
///   true → `construct_circle_bottom_layer()` + 0x28B indexmap bucket
///   `{0,4,0, wall_id, uVar13}`（ElevationSupportStats 记录）+
///   `construct_circle_pillars()`；
/// - 再把本次支柱记录复制/去重进 `ElevationSupports`（0x20B 记录 Vec：
///   复制循环先整块 alloc，再逐 2 条 0x20B 拷贝，旧 Vec dealloc）。
///
/// **未实现（core-type-gap）**：`country_core::resources::walls::
/// elevation_support_stats::ElevationSupports/ElevationSupportStats` 现为
/// `Gap`（布局 U），0x28B stats 记录与 0x20B 支柱记录字段无法逐语句落库；
/// 且被调 `circle_categorize`/`construct_turret`/`construct_circle_bottom_layer`/
/// `construct_circle_pillars` 均未实现。解锁条件：core 落 ElevationSupports
/// 布局 + 四个被调函数重建后按上述全序实现。
pub fn construct_circle_supports() {
    todo!(
        "未实现: construct_circle_supports 0x140788160（ElevationSupports 布局 U + circle_categorize/turret/bottom_layer/pillars 未实现）"
    )
}

/// B `construct_rectangle_columns`（0x140788a80，2674B / 651 行 fresh）。
/// **未实现**：矩形柱构造（4 角 `get_point`/地形门控/逐墙
/// `GladeBorder::is_pos_inside`/3 向 `legacy_raycast_w_params` 净空 ≥2.3/
/// `find_stone_column_start_y`/smoothstep/Rectangle3d 输出/
/// halftimber 二路——即 `construct_rectangle_pillars` 0x14124fcb0 的调用宿主，
/// 该被调函数仍未实现）。
pub fn construct_rectangle_columns() {
    todo!(
        "未实现: construct_rectangle_columns 0x140788a80（宿主链依赖 construct_rectangle_pillars 未实现）"
    )
}

/// B `construct_rectangle_brackets`（0x1407896b0，2626B / 584 行 fresh）。
/// **未实现**：矩形托架构造（`construct_rectangle_small_brackets`
/// 0x141250da0 / `construct_rectangle_large_brackets` 0x141253fc0 调用宿主；
/// 被调两者仍未实现）。
pub fn construct_rectangle_brackets() {
    todo!(
        "未实现: construct_rectangle_brackets 0x1407896b0（宿主链依赖 small/large brackets 未实现）"
    )
}

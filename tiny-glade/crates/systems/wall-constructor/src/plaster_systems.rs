//! 石膏系统（plaster_systems.rs）。
//!
//! 2026-10-17 restorer-remaining-wallconstructor fresh 反编译结论
//! （证据 `work/restorer-remaining-wallconstructor/evidence/`）：
//! 旧实现为**近似体**（create 分支自造实体、delete 分支直接 despawn、
//! recalc 空转），与机器码不符，已按规定降级为 `todo!` 并逐条登记分歧点。
//!
//! ## `maintain_plaster`（0x14075b810，5273B / 53KB decomp）已解结构
//! 1. `puffin` scope 包装（`are_scopes_on`/`begin_scope`/`end_scope`）。
//! 2. 延迟缓冲回收：`retain<tuple<Buffer,i32>, closure_env$0>`
//!    （`maintain_deferred_plaster_buffer_disposal`）。
//! 3. `from_iter<WallId, Map<EventIterator<OnWallChanged>, closure_env$0>>`
//!    —— 消费本帧变更墙 id。
//! 4. 逐墙：`PublicWallState` 查表 → `time_continuous_lerp` 求 δ（peel
//!    量）→ `|new-old| >= 0.01` 才：`try_get_wall_shape`（ShapeParameters::
//!    center）、`max_y`、`xvy` → 0x28B indexmap bucket（tag byte 9 @+0x20、
//!    +0x1c=1.0、+0x24 等）入 signifier/记录 Vec；
//!    `insert<WallId, PlasterPeelAmount>`（0x1411fcad0）；peel 值变化再 push
//!    到脏 Vec。
//! 5. `Decorators::iter_removed_ids` → `remove_entry<WallId, Vec<Entity>>`
//!    （WallPlasters）→ 逐实体 `Entities::contains` → `despawn`。
//! 6. `from_iter<WallId, Copied<Keys<WallId, PublicWallState>>>`（PublicWalls
//!    全 key）→ `sort_by_key<closure_env$4>`（insertion_sort/driftsort）。
//! 7. 再 drain `WallPlasters`（remove_entry + despawn 残留实体）。
//! 8. 逐墙 `StructureType::is_fence` 门控；`FinalizedWallHoles::
//!    holes_no_duplicates()` → `generate_plaster_mesh_and_entities<
//!    Map<Filter<Iter<(Aabb2,HoleType,HoleOrigin)>>>>`。
//!
//! ## `create_plaster`（0x14075d460，1686B）已解结构
//! 逐 `CreatePlasterCmd` 事件（40B stride）：puffin scope → `PublicWalls`
//! 探测（墙记录 stride 0x2a8，缺失 `unwrap_failed`）→ `WallHoles::
//! as_finalized`（stride 0x68）→ `holes_no_duplicates()` → 另一 map 探测
//! （stride 0x20）→ `generate_plaster_mesh_and_entities`。
//!
//! ## 其余
//! - `split_wall_into_sides`（0x14075dc60，3130B）：墙 `+0x258` 字段 < 2
//!   与 bit0 门控、4 侧循环、WallSide 记录/0.1m 格/`psubusw 0x00020002`
//!   下限等，未逐语句。
//! - `generate_plaster_mesh_and_entities`（0x1412269b0，71.6KB 泛型单态）：
//!   mesh 拓扑/UV/buffer 布局未逐语句。
//! - `register_required_components`（0x14075e9e0）：Plaster 组件注册胶水
//!   （待类型落库）。
//!
//! **未实现**：以上 4 个函数均保持 `todo!`；不写近似体。解锁条件：按各
//! 函数新鲜反编译全序重建（`maintain_plaster` 还需 `Decorators`/
//! `WallPlasters`/`PlasterPeelAmount`/`time_continuous_lerp` 的落库面）。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::Roof,
    walls::{
        plaster::{CreatePlasterCmd, DeletePlasterCmd, RecalculatePlasterCmd},
        public_walls::{PublicWalls, WallPlasters},
        wall_holes::WallHoles,
    },
};
use bevy_ecs::prelude::*;

/// B `maintain_plaster`（0x14075b810）：石膏命令消费/实体生命周期维护。
/// **未实现**：真实结构见模块文档（peel 插值/脏链/PublicWalls 排序/
/// holes→mesh 全序），旧近似体已移除。
pub fn maintain_plaster(
    _plasters: ResMut<WallPlasters>,
    _graphics: Res<AssetSsboLibrary>,
    _walls: Res<PublicWalls>,
    _holes: ResMut<WallHoles>,
    _roofs: Query<&Roof>,
    _commands: Commands,
    _create: EventReader<CreatePlasterCmd>,
    _delete: EventReader<DeletePlasterCmd>,
    _recalc: EventReader<RecalculatePlasterCmd>,
) {
    todo!("未实现: maintain_plaster 0x14075b810（peel 插值/iter_removed_ids/PublicWalls sort_by_key/holes→mesh 全序；见模块文档）")
}

/// B `create_plaster`（0x14075d460）。**未实现**：逐事件 `PublicWalls` 探测
/// （wall 缺失 unwrap panic）→ `WallHoles::as_finalized` → `holes_no_duplicates`
/// → `generate_plaster_mesh_and_entities`；旧近似体已移除。
pub fn create_plaster(
    _plasters: ResMut<WallPlasters>,
    _walls: Res<PublicWalls>,
    _commands: Commands,
    _create: EventReader<CreatePlasterCmd>,
) {
    todo!("未实现: create_plaster 0x14075d460（PublicWalls→WallHoles::as_finalized→holes_no_duplicates→generate_plaster_mesh 全序）")
}

/// B `split_wall_into_sides`（0x14075dc60）。**未实现**：4 侧拆分（WallSide
/// 24B/0.1m 格/`psubusw 0x00020002` 下限/roof_flag 枚举）未逐语句。
/// 旧近似体（首尾两点推 [f32;4]）已移除。
pub fn split_wall_into_sides(_points: &[[f32; 3]], _out: &mut Vec<[f32; 4]>) {
    todo!("未实现: split_wall_into_sides 0x14075dc60（4 侧循环/WallSide 24B/0.1m 格/0x00020002 下限）")
}

/// B `generate_plaster_mesh_and_entities`（0x1412269b0）：石膏网格与实体生成
/// （泛型单态 `Map<Filter<Iter<(Aabb2, HoleType, HoleOrigin)>>>`）。
/// **未实现**：逐顶点拓扑/UV/buffer 布局未逐语句。
pub fn generate_plaster_mesh_and_entities(
    _holes: &[(f32, f32, f32, f32, u8, u8)],
    _out_mesh: &mut Vec<[f32; 3]>,
) -> usize {
    todo!("未实现: generate_plaster_mesh_and_entities 0x1412269b0（71.6KB 泛型单态：mesh 拓扑/UV/buffer 布局）")
}

/// B `register_required_components`（0x14075e9e0）：Plaster 的 required 组件。
pub fn register_required_components(_registry: &mut bevy_ecs::component::Components) {}

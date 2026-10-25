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
//! - `split_wall_into_sides`（0x14075dc60，3130B）：**已重建**（纯函数）——
//!   `(shape_tag & 1)==0` 单侧 `[0,1]`；矩形四边 `get_point_u(i)±1e-7`；
//!   gable 方向偶数/奇数边抬升 `max_y + Roof::height()`；0.1m 格数经
//!   `roundf`→`[2,65535]`（`psubusw 0x00020002`）；`WallSide` 24B 布局。
//!   网格/顶点缓冲本体仍由 `generate_plaster_mesh_and_entities`（U）消费。
//! - `generate_plaster_mesh_and_entities`（0x1412269b0，71.6KB 泛型单态）：
//!   mesh 拓扑/UV/buffer 布局未逐语句。
//! - `register_required_components`（0x14075e9e0）：Plaster 组件注册胶水
//!   （待类型落库）。
//!
//! **未实现**：`maintain_plaster`/`create_plaster`/
//! `generate_plaster_mesh_and_entities` 保持 `todo!`（`split_wall_into_sides`
//! 已按上述机器码重建）；不写近似体。解锁条件：按各函数新鲜反编译全序重建
//! （`maintain_plaster` 还需 `Decorators`/`WallPlasters`/`PlasterPeelAmount`/
//! `time_continuous_lerp` 的落库面）。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::Roof,
    walls::{
        plaster::{CreatePlasterCmd, DeletePlasterCmd, RecalculatePlasterCmd},
        public_walls::{PublicWalls, WallPlasters},
        wall_holes::WallHoles,
    },
};

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
    todo!(
        "未实现: maintain_plaster 0x14075b810（peel 插值/iter_removed_ids/PublicWalls sort_by_key/holes→mesh 全序；见模块文档）"
    )
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
    todo!(
        "未实现: create_plaster 0x14075d460（PublicWalls→WallHoles::as_finalized→holes_no_duplicates→generate_plaster_mesh 全序）"
    )
}

/// B `WallSide`（24B=0x18）：`{u_begin@0, u_end@4, height@8, y_offset@0xc,
/// grid u32@0x10（低 u16=水平格数，高 u16=高度格数）, roof_flag u8@0x14}`。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallSide {
    pub u_begin: f32,
    pub u_end: f32,
    pub height: f32,
    pub y_offset: f32,
    pub grid: u32,
    pub roof_flag: u8,
    pub _pad: [u8; 3],
}

/// x86 `ROUNDSS`（四舍五入远离零），随后按原 `MAXSS 0` / `MINSS 65535`
/// 与 `PSUBUSW 0x00020002` 组合把格数钳到 `[2, 65535]`；NaN → 2。
fn side_grid_cells(value: f32) -> u16 {
    let rounded = value.round();
    let lowered = if 0.0 <= rounded { rounded } else { 0.0 };
    let clamped = if lowered <= 65535.0 { lowered } else { 65535.0 };
    let cells = clamped as i32 as u16;
    // psubusw(2, cells) + cells == max(cells, 2)。
    if cells < 2 { 2 } else { cells }
}

/// B `split_wall_into_sides`（0x14075dc60，3130B / 1228 行 fresh:
/// `decomp-0x14075dc60.txt`）。
///
/// - `(shape_tag & 1) == 0`（圆/自由曲线）：单条 `[0.0, 1.0]` 侧；
/// - 矩形：四条边 `get_point_u(i)..get_point_u(i+1)`（i=0..3），
///   `u_begin += 1e-7`、`u_end -= 1e-7`；
/// - `height`：无屋顶/非 gable 时全部 `max_y`；gable 方向 0 抬升边 0/2，
///   方向非 0 抬升边 1/3，抬升值 `max_y + Roof::height()`；
/// - `y_offset = height − wall_height`（公开记录 +0x280）；
/// - `grid = |max(round((u_end−u_begin)·curve_length/0.1), 2) 低 u16
///   | max(round(y_offset/0.1), 2) « 16|`；
/// - `roof_flag = 1` 当该边抬升（与 gable 方向一致）。
///
/// 原函数以 `SmallVec<WallSide,4>` 返回并写 `+0x60` 长度；Rust 侧用 `Vec`
/// 保持同一元素字节序（`WallSide` 布局与 24B 一致）。
pub fn split_wall_into_sides(
    rectangle: Option<utils::geometry::Rectangle2d>,
    max_y: f32,
    wall_height: f32,
    curve_length: f32,
    roof: Option<&country_core::resources::roofs::Roof>,
    out: &mut Vec<WallSide>,
) {
    let gable = roof.is_some_and(|roof| roof.is_gable());
    let roof_height = roof.map(|roof| roof.height()).unwrap_or(0.0);
    let direction = roof.map(|roof| roof.observed()[0x3c]).unwrap_or(0);
    let make = |u_begin: f32, u_end: f32, height: f32, flag: u8| {
        let y_offset = height - wall_height;
        let span = (u_end - u_begin) * curve_length;
        let horizontal = side_grid_cells(span / f32::from_bits(0x3dcccccd));
        let vertical = side_grid_cells(y_offset / f32::from_bits(0x3dcccccd));
        WallSide {
            u_begin,
            u_end,
            height,
            y_offset,
            grid: u32::from(horizontal) | (u32::from(vertical) << 16),
            roof_flag: flag,
            _pad: [0; 3],
        }
    };
    let Some(rectangle) = rectangle else {
        out.push(make(0.0, 1.0, max_y, 0));
        return;
    };
    let epsilon = f32::from_bits(0x33d6bf95); // 1e-7
    let raised = max_y + roof_height;
    for side in 0..4 {
        let u_begin = rectangle.get_point_u(side) + epsilon;
        let u_end = rectangle.get_point_u(side + 1) - epsilon;
        let use_raised = gable && (direction == 0) == (side % 2 == 0);
        let (height, flag) = if use_raised {
            (raised, 1)
        } else {
            (max_y, 0)
        };
        out.push(make(u_begin, u_end, height, flag));
    }
}

#[cfg(test)]
mod split_sides_tests {
    use super::*;
    use glam::Vec2;
    use utils::geometry::Rectangle2d;

    #[test]
    fn non_rectangle_is_one_full_side() {
        let mut out = Vec::new();
        split_wall_into_sides(None, 3.0, 2.0, 10.0, None, &mut out);
        assert_eq!(out.len(), 1);
        let side = out[0];
        assert_eq!((side.u_begin, side.u_end), (0.0, 1.0));
        assert_eq!((side.height, side.y_offset), (3.0, 1.0));
        // 10 m / 0.1 = 100；高度差 1 m / 0.1 = 10。
        assert_eq!(side.grid & 0xffff, 100);
        assert_eq!(side.grid >> 16, 10);
        assert_eq!(side.roof_flag, 0);
    }

    #[test]
    fn rectangle_four_sides_and_grid_floor() {
        let rect = Rectangle2d::new(Vec2::ZERO, Vec2::new(4.0, 2.0), Vec2::X);
        let mut out = Vec::new();
        // max_y == wall_height → 高度格 0 → 下限 2。
        split_wall_into_sides(Some(rect), 2.0, 2.0, 12.0, None, &mut out);
        assert_eq!(out.len(), 4);
        assert_eq!(std::mem::size_of::<WallSide>(), 24);
        for side in &out {
            assert_eq!(side.height, 2.0);
            assert_eq!(side.y_offset, 0.0);
            assert_eq!(side.grid >> 16, 2, "psubusw 下限 0x00020002");
            assert!(side.u_begin < side.u_end);
            assert_eq!(side.roof_flag, 0);
        }
        // 四条边总长 = 2*(4+2) = 12 → 12/0.1 = 120 格总量。
        let total: u32 = out.iter().map(|s| s.grid & 0xffff).sum();
        assert_eq!(total, 120);
    }

    #[test]
    fn gable_direction_raises_alternating_sides() {
        let rect = Rectangle2d::new(Vec2::ZERO, Vec2::new(4.0, 4.0), Vec2::X);
        let mut bytes = [0u8; 88];
        bytes[0x30..0x34].copy_from_slice(&1.5f32.to_le_bytes());
        bytes[0x34..0x38].copy_from_slice(&1.0f32.to_le_bytes());
        bytes[0x3c] = 0;
        let roof = country_core::resources::roofs::Roof::from_observed(bytes);
        assert!(roof.is_gable());
        let mut out = Vec::new();
        split_wall_into_sides(Some(rect), 2.0, 2.0, 16.0, Some(&roof), &mut out);
        assert_eq!(out[0].roof_flag, 1);
        assert_eq!(out[1].roof_flag, 0);
        assert_eq!(out[2].roof_flag, 1);
        assert_eq!(out[3].roof_flag, 0);
        let raised = 2.0 + roof.height();
        assert_eq!(out[0].height, raised);
        assert_eq!(out[1].height, 2.0);
    }
}

/// B `generate_plaster_mesh_and_entities`（0x1412269b0）：石膏网格与实体生成
/// （泛型单态 `Map<Filter<Iter<(Aabb2, HoleType, HoleOrigin)>>>`）。
/// **未实现**：逐顶点拓扑/UV/buffer 布局未逐语句。
pub fn generate_plaster_mesh_and_entities(
    _holes: &[(f32, f32, f32, f32, u8, u8)],
    _out_mesh: &mut Vec<[f32; 3]>,
) -> usize {
    todo!(
        "未实现: generate_plaster_mesh_and_entities 0x1412269b0（71.6KB 泛型单态：mesh 拓扑/UV/buffer 布局）"
    )
}

/// B `register_required_components`（0x14075e9e0）：Plaster 的 required 组件。
pub fn register_required_components(_registry: &mut bevy_ecs::component::Components) {}

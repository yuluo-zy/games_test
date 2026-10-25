//! `country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug`
//! （B 0x1408ec0e0，1931 B；`set 19 chain2` 成员）。
//!
//! 原版 SystemParam 面（四个形参，顺序固定）：
//!
//! | # | 原版形参 | 读取 |
//! |---|---|---|
//! | 1 | `Res<WallSpatialHash>` | 网格维度 `+0x48`（i32 对 = 64×64）、世界尺寸 `+0x50`（f32 对 = 130×130）、格内段 |
//! | 2 | `Res<MouseDepthValue>` | `mouse_position`（`+8` x、`+0x10` z）作圆心 |
//! | 3 | `ResMut<UiSignifierStream>` | `DebugRectangle2d/DebugLine::show` 的提交流 |
//! | 4 | `Res<DebugGui>` | `byte +0xf1 == 1`（`show_spatial_grid`）才工作 |
//!
//! 绘制顺序与参数（均取自反编译立即数/rodata：`0.5`、`-0.5`、`1.0`、`7.0 = 0x40e00000`、
//! `√2 = 0x3fb504f3`）：
//!
//! 1. 世界包围 `Aabb2(-size/2, +size/2)` 转 `Rectangle2d`，白（tag 0）、线宽 1、height 0 的
//!    `DebugRectangle2d`；
//! 2. 竖线 `i in 1..dims.x`：`x = lerp(-size.x/2, size.x/2, i/dims.x)`，两点 `(x,0,∓size.y/2)`，
//!    灰（tag 4）、线宽 1 的 `DebugLine`；
//! 3. 横线 `i in 1..dims.y`：`z = lerp(-size.y/2, size.y/2, i/dims.x)`——**原版分母用的是 `dims.x`**
//!    （0x1408ec2d0 的 `CVTSI2SS XMM2,R13D` 取 `+0x48` 低 32 位），64×64 下与 `dims.y` 等价，但这里保留；
//! 4. `circle = Circle2d::new((mouse.x, mouse.z), 7.0)`，`cells = get_cells_from_circle(circle)`；
//! 5. 圆折线：`circle.as_curve3(0.5, 0.0)` → 白（tag 0）、线宽 1 的 `DebugLine`；
//! 6. `segments = query_with_circle(circle)`（段指针按墙 id `u32` 排序后相邻同墙去重，**无半径筛**）；
//! 7. 每个格子画一个红（tag 1）、线宽 2、height 0 的 `DebugRectangle2d`（格矩形，轴 `(1,0)`）；
//! 8. 每条段画一条绿（tag 3）、线宽 3 的 `DebugLine`：`(a.x, h0, a.y) → (b.x, h1, b.y)`。

use crate::resources::gui::DebugGui;
use crate::resources::read_mouse_depth_value::MouseDepthValue;
use crate::resources::walls::spatial_index::{GRID_SIDE, WORLD_SIZE};
use crate::systems::ui::signifiers::UiSignifierStream;
use crate::systems::wall::wall_state::wall_spatial_hash::{WallSpatialHash, WallSpatialSegment};
use crate::utils::debug_viz::{DebugColor, DebugLine, DebugRectangle2d};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::geometry::{Aabb2, Circle2d, Rectangle2d};

/// 圆查询半径（`0x142a7cd48` = `0x40e00000`）。
pub const CURSOR_CIRCLE_RADIUS: f32 = 7.0;
/// 圆折线最大步长（`0x142925870` = 0.5）。
const CIRCLE_MAX_STEP: f32 = 0.5;
/// `RESAMPLING`（`assertion failed: RESAMPLING < self.cell_size().x` 的常量，与 `0x142925870` 同值 0.5）。
const RESAMPLING: f32 = 0.5;
/// `0x14292e6f4` = `0x3fb504f3`。
const SQRT_2: f32 = f32::from_bits(0x3fb5_04f3);

const DIMS: (i32, i32) = (GRID_SIDE as i32, GRID_SIDE as i32);
const SIZE: Vec2 = Vec2::new(WORLD_SIZE, WORLD_SIZE);

/// `floorf` 后做 Rust 饱和 `as i32`（原版 `CVTTSS2SI` + 越界 `0x7fffffff` + NaN→0 的同式）。
fn floor_to_i32(value: f32) -> i32 {
    value.floor() as i32
}

/// `WallSpatialHash::get_cells_from_circle`（0x1409f0e80）：返回覆盖格的线性下标，行优先。
///
/// 包围盒先夹到网格边界（`MAXPS/MINPS`，NaN 取网格边界），再换算到格坐标；格中心到圆心距离
/// `< radius + cell.x * 0.5 * √2` 的格才入选。
pub fn get_cells_from_circle(circle: Circle2d) -> Vec<usize> {
    let aabb = circle.aabb();
    let grid_min = SIZE * -0.5;
    let grid_max = SIZE * 0.5;
    // `MAXPS`/`MINPS` 带 NaN 掩码：NaN 的包围盒分量退回网格边界。
    let clamp_min = |bbx: f32, grid: f32| if bbx.is_nan() { grid } else if grid > bbx { grid } else { bbx };
    let clamp_max = |bbx: f32, grid: f32| if bbx.is_nan() { grid } else if grid < bbx { grid } else { bbx };
    let bbx_min = Vec2::new(clamp_min(aabb.min.x, grid_min.x), clamp_min(aabb.min.y, grid_min.y));
    let bbx_max = Vec2::new(clamp_max(aabb.max.x, grid_max.x), clamp_max(aabb.max.y, grid_max.y));
    assert!(
        bbx_min.x >= grid_min.x && bbx_min.y >= grid_min.y,
        "assertion failed: bbx_min.x >= grid_min.x && bbx_min.y >= grid_min.y"
    );
    assert!(
        bbx_max.x <= grid_max.x && bbx_max.y <= grid_max.y,
        "assertion failed: bbx_max.x <= grid_max.x && bbx_max.y <= grid_max.y"
    );
    let cell_size = Vec2::new(SIZE.x / DIMS.0 as f32, SIZE.y / DIMS.1 as f32);
    assert!(
        RESAMPLING < cell_size.x,
        "assertion failed: RESAMPLING < self.cell_size().x"
    );
    let half = SIZE * 0.5;
    let x_min = floor_to_i32((bbx_min.x + half.x) / cell_size.x).max(0);
    let y_min = floor_to_i32((bbx_min.y + half.y) / cell_size.y).max(0);
    let x_max = floor_to_i32(((bbx_max.x + 0.0) + half.x) / cell_size.x).min(DIMS.0 - 1);
    let y_max = floor_to_i32(((bbx_max.y + 0.0) + half.y) / cell_size.y).min(DIMS.1 - 1);
    let mut cells = Vec::new();
    if y_max < y_min || x_max < x_min {
        return cells;
    }
    let expansion = cell_size.x * 0.5 * SQRT_2;
    for row in y_min..=y_max {
        for col in x_min..=x_max {
            let index = row.wrapping_mul(DIMS.0).wrapping_add(col);
            let column_fraction = (index % DIMS.0) as f32 / DIMS.0 as f32;
            let row_fraction = floor_to_i32(index as f32 / DIMS.0 as f32) as f32 / DIMS.1 as f32;
            let center = cell_center(column_fraction, row_fraction, half, cell_size);
            let dx = center.x - circle.center.x;
            let dy = center.y - circle.center.y;
            if (dy * dy + dx * dx).sqrt() < circle.radius + expansion {
                cells.push(index as usize);
            }
        }
    }
    cells
}

/// `(frac * half - (1 - frac) * half) + 0.5 * cell`，与反编译的运算顺序一致。
fn cell_center(column_fraction: f32, row_fraction: f32, half: Vec2, cell: Vec2) -> Vec2 {
    Vec2::new(
        (column_fraction * half.x - (1.0 - column_fraction) * half.x) + 0.5 * cell.x,
        (row_fraction * half.y - (1.0 - row_fraction) * half.y) + 0.5 * cell.y,
    )
}

/// 一个格子的 `Rectangle2d`（0x1408ec66c 的 `Rectangle2d::new(cell.x, cell.y, axis (1,0), center)`）。
pub fn cell_rectangle(index: usize) -> Rectangle2d {
    let index = index as i32;
    let cell = Vec2::new(SIZE.x / DIMS.0 as f32, SIZE.y / DIMS.1 as f32);
    let half = SIZE * 0.5;
    let column_fraction = (index % DIMS.0) as f32 / DIMS.0 as f32;
    let row_fraction = floor_to_i32(index as f32 / DIMS.0 as f32) as f32 / DIMS.1 as f32;
    Rectangle2d::new(cell_center(column_fraction, row_fraction, half, cell), cell, Vec2::X)
}

/// `WallSpatialHash::query_with_circle`（0x140aa5940）：收集覆盖格内的全部段，按墙 id 的 `u32`
/// 排序（比较器 0x141338ac3：`cmp dword [seg+0x30]`），再相邻同墙去重（保留每个墙的首段）。
///
/// 未闭合：原版 `len <= 20` 走稳定插入排序，`len > 20` 走 `ipnsort`（不稳定），同墙多段时
/// 保留哪一段在 `len > 20` 时取决于 `ipnsort` 内部顺序；这里统一用稳定排序。
pub fn query_with_circle(spatial: &WallSpatialHash, circle: Circle2d) -> Vec<WallSpatialSegment> {
    let mut segments: Vec<WallSpatialSegment> = Vec::new();
    for cell in get_cells_from_circle(circle) {
        segments.extend(spatial.segments_in_cell(cell).copied());
    }
    segments.sort_by_key(|segment| segment.wall.0 as u32);
    let mut kept: Vec<WallSpatialSegment> = Vec::with_capacity(segments.len());
    for segment in segments {
        if kept
            .last()
            .is_none_or(|last| last.wall.0 as u32 != segment.wall.0 as u32)
        {
            kept.push(segment);
        }
    }
    kept
}

/// 网格竖线/横线折线（两点，灰色，线宽 1）。
pub fn grid_lines() -> Vec<DebugLine> {
    let half = SIZE * 0.5;
    let min = SIZE * -0.5;
    let mut lines = Vec::new();
    for i in 1..DIMS.0 {
        let t = i as f32 / DIMS.0 as f32;
        let x = t * half.x + (1.0 - t) * min.x;
        lines.push(
            DebugLine::new(vec![Vec3::new(x, 0.0, min.y), Vec3::new(x, 0.0, half.y)])
                .color(DebugColor::Grey),
        );
    }
    for i in 1..DIMS.1 {
        // 分母为 `dims.x`（见模块文档）。
        let t = i as f32 / DIMS.0 as f32;
        let z = t * half.y - (1.0 - t) * half.y;
        lines.push(
            DebugLine::new(vec![Vec3::new(min.x, 0.0, z), Vec3::new(half.x, 0.0, z)])
                .color(DebugColor::Grey),
        );
    }
    lines
}

pub fn spatial_hash_debug(
    spatial: Res<WallSpatialHash>,
    mouse_depth: Res<MouseDepthValue>,
    mut stream: ResMut<UiSignifierStream>,
    debug_gui: Res<DebugGui>,
) {
    if !debug_gui.show_spatial_grid {
        return;
    }
    // 1. 世界包围矩形（`Rectangle2d::from(Aabb2)`：size = max - min，center = (min + max) / 2）。
    let bounds = Aabb2 {
        min: SIZE * -0.5,
        max: SIZE * 0.5,
    };
    DebugRectangle2d {
        color: DebugColor::White,
        line_width: 1.0,
        rect: Rectangle2d::new_axis_aligned((bounds.min + bounds.max) * 0.5, bounds.max - bounds.min),
        height: 0.0,
    }
    .show(&mut stream);
    // 2/3. 网格线。
    for line in grid_lines() {
        line.show(&mut stream);
    }
    // 4. 光标圆与覆盖格。
    let cursor = mouse_depth.mouse_position;
    let circle = Circle2d::new(Vec2::new(cursor.x, cursor.z), CURSOR_CIRCLE_RADIUS);
    let cells = get_cells_from_circle(circle);
    // 5. 圆折线。
    DebugLine::new(circle.as_curve3(CIRCLE_MAX_STEP, 0.0).points)
        .color(DebugColor::White)
        .show(&mut stream);
    // 6. 圆内段。
    let segments = query_with_circle(&spatial, circle);
    // 7. 覆盖格矩形。
    for cell in cells {
        DebugRectangle2d {
            color: DebugColor::Red,
            line_width: 2.0,
            rect: cell_rectangle(cell),
            height: 0.0,
        }
        .show(&mut stream);
    }
    // 8. 段折线。
    for segment in segments {
        DebugLine::new(vec![
            Vec3::new(segment.endpoints[0][0], segment.relative_heights[0], segment.endpoints[0][1]),
            Vec3::new(segment.endpoints[1][0], segment.relative_heights[1], segment.endpoints[1][1]),
        ])
        .color(DebugColor::Green)
        .line_width(3.0)
        .show(&mut stream);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::WallId;
    use bevy_ecs::system::RunSystemOnce;

    fn segment(wall: u64, a: [f32; 2], b: [f32; 2]) -> WallSpatialSegment {
        WallSpatialSegment {
            endpoints: [a, b],
            source_u: [0.0, 1.0],
            relative_heights: [1.0, 2.0],
            terrain_heights: [0.0, 0.0],
            wall: WallId(wall),
        }
    }

    fn world(show: bool, cursor: Vec3) -> World {
        let mut world = World::new();
        let mut spatial = WallSpatialHash::default();
        spatial.add_segment(segment(2, [0.0, 0.0], [3.0, 0.0]));
        spatial.add_segment(segment(2, [3.0, 0.0], [6.0, 0.0]));
        spatial.add_segment(segment(1, [0.0, 1.0], [3.0, 1.0]));
        spatial.add_segment(segment(9, [50.0, 50.0], [52.0, 50.0]));
        world.insert_resource(spatial);
        world.insert_resource(MouseDepthValue {
            mouse_position: cursor,
            ..Default::default()
        });
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(DebugGui {
            show_spatial_grid: show,
            ..Default::default()
        });
        world
    }

    #[test]
    fn disabled_flag_draws_nothing() {
        let mut w = world(false, Vec3::ZERO);
        w.run_system_once(spatial_hash_debug).unwrap();
        assert_eq!(w.resource::<UiSignifierStream>().committed_len(), 0);
    }

    #[test]
    fn grid_line_table_matches_the_original_loops() {
        let lines = grid_lines();
        // 63 竖线 + 63 横线。
        assert_eq!(lines.len(), 126);
        // 第一条竖线：i = 1，x = lerp(-65, 65, 1/64)。
        let t = 1.0f32 / 64.0;
        let x = t * 65.0 + (1.0 - t) * -65.0;
        assert_eq!(lines[0].points, vec![Vec3::new(x, 0.0, -65.0), Vec3::new(x, 0.0, 65.0)]);
        assert_eq!(lines[0].color, DebugColor::Grey);
        // 第一条横线：z 由 `t*65 - (1-t)*65` 给出，横跨 x ∈ [-65, 65]。
        let z = t * 65.0 - (1.0 - t) * 65.0;
        assert_eq!(lines[63].points, vec![Vec3::new(-65.0, 0.0, z), Vec3::new(65.0, 0.0, z)]);
    }

    #[test]
    fn cells_are_row_major_and_bounded_by_the_radius_plus_diagonal_half() {
        let circle = Circle2d::new(Vec2::new(0.0, 0.0), 7.0);
        let cells = get_cells_from_circle(circle);
        assert!(!cells.is_empty());
        assert!(cells.windows(2).all(|pair| pair[0] < pair[1]), "row-major order");
        // 圆心所在的格（32,32）必在其内；远处格不在。
        assert!(cells.contains(&(32 * 64 + 32)));
        assert!(!cells.contains(&0));
        // 每个入选格中心到圆心的距离 < 7 + cell*0.5*sqrt2。
        let cell = 130.0f32 / 64.0;
        for index in cells {
            let rect = cell_rectangle(index);
            assert!((rect.center - circle.center).length() < 7.0 + cell * 0.5 * SQRT_2 + 1e-3);
            assert_eq!(rect.size, Vec2::new(cell, cell));
            assert_eq!(rect.axis, Vec2::X);
        }
    }

    #[test]
    fn circle_outside_the_world_is_clamped_to_the_border_cells() {
        let cells = get_cells_from_circle(Circle2d::new(Vec2::new(200.0, 0.0), 7.0));
        assert!(cells.is_empty(), "no overlap with the clamped border: cells {cells:?}");
    }

    #[test]
    fn query_dedups_adjacent_walls_after_sorting_by_wall_id() {
        let w = world(true, Vec3::ZERO);
        let spatial = w.resource::<WallSpatialHash>();
        let hits = query_with_circle(spatial, Circle2d::new(Vec2::ZERO, 7.0));
        let walls: Vec<u64> = hits.iter().map(|s| s.wall.0).collect();
        assert_eq!(walls, vec![1, 2], "one segment per wall, ascending id, far wall 9 excluded");
    }

    #[test]
    fn enabled_flag_emits_bounds_grid_circle_cells_and_segments() {
        let mut w = world(true, Vec3::new(0.0, 0.0, 0.0));
        w.run_system_once(spatial_hash_debug).unwrap();
        let circle = Circle2d::new(Vec2::ZERO, 7.0);
        let cells = get_cells_from_circle(circle).len();
        let segments = query_with_circle(w.resource::<WallSpatialHash>(), circle).len();
        // 1 包围 + 126 网格线 + 1 圆折线 + 格矩形 + 段折线。
        let expected = 1 + 126 + 1 + cells + segments;
        assert_eq!(
            w.resource::<UiSignifierStream>().committed_len() as usize,
            expected
        );
    }
}

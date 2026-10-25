//! 世界编辑区域的脏瓦片位图：每行一个 u64，支持区域置位、查询和发布合并。
//!
//! 64×64 瓦片覆盖 130×130 的世界范围，单瓦片对应 8×8 个栅格像素。
//! 矩形使用半开区间，笔画在枚举前裁剪到位图边界。

use ::utils::float_grid::DEFAULT_WORLD_EXTENT;
use glam::{Vec2, Vec3};
use tabloid::{State, StateDelta};

use crate::utils::conservative_line_raster::conservative_plot_line_in_bounds;

/// 单个瓦片包含的栅格像素边长。
pub const TILE_SIZE: usize = 8;
/// 每个方向的瓦片数量；一行恰好放入一个 u64。
pub const GRID_SIZE: usize = 64;
/// 超出多个世界宽度的坐标改用全图重建，避免 f32 转换精度不足导致漏标。
const MAX_PRECISE_WORLD_COORDINATE: f32 = DEFAULT_WORLD_EXTENT * 16.0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirtyTiles {
    rows: [u64; GRID_SIZE],
}

/// Historical name for the same 64 by 64 dirty tile grid.
pub use DirtyTiles as DirtyTiles64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirtyTileRect {
    pub minimum: [u32; 2],
    /// 不包含的终点，范围为 0..=64。
    pub end: [u32; 2],
}

impl DirtyTileRect {
    pub fn is_empty(self) -> bool {
        self.minimum[0] >= self.end[0] || self.minimum[1] >= self.end[1]
    }
}

/// 世界坐标转连续瓦片坐标。f64 避免极大有限坐标在转换时溢出。
fn world_to_tile(world: f32) -> f64 {
    (f64::from(world) / f64::from(::utils::float_grid::DEFAULT_WORLD_EXTENT) + 0.5)
        * GRID_SIZE as f64
}

/// 世界 XZ 矩形转位图范围；无效或逆序输入返回空区间。
pub fn world_rect_cells(rect: [f32; 4]) -> DirtyTileRect {
    let [min_x, min_z, max_x, max_z] = rect;
    if !rect.iter().all(|value| value.is_finite()) || min_x > max_x || min_z > max_z {
        return DirtyTileRect {
            minimum: [0; 2],
            end: [0; 2],
        };
    }
    let bound = |coordinate: f32, upper: bool| {
        let tile = world_to_tile(coordinate);
        let rounded = if upper { tile.ceil() } else { tile.floor() };
        rounded.clamp(0.0, GRID_SIZE as f64) as u32
    };
    DirtyTileRect {
        minimum: [bound(min_x, false), bound(min_z, false)],
        end: [bound(max_x, true), bound(max_z, true)],
    }
}

/// 已验证的非空半开列区间转连续位掩码，不执行移位 64 位。
fn column_mask(start: u32, end: u32) -> u64 {
    (u64::MAX >> (GRID_SIZE as u32 - (end - start))) << start
}

impl Default for DirtyTiles {
    fn default() -> Self {
        Self::new_clear()
    }
}

impl DirtyTiles {
    /// 查询世界点所在的瓦片；非有限或超出世界边界时返回 None。
    pub fn cell_from_world_space(world: Vec2) -> Option<[u32; 2]> {
        let column = world_to_tile(world.x).floor();
        let row = world_to_tile(world.y).floor();
        let range = 0.0..GRID_SIZE as f64;
        (range.contains(&column) && range.contains(&row)).then_some([column as u32, row as u32])
    }

    /// 创建空位图。
    pub fn new_clear() -> Self {
        Self {
            rows: [0; GRID_SIZE],
        }
    }

    /// 创建全部置脏的位图。
    pub fn new_all_dirty() -> Self {
        Self {
            rows: [u64::MAX; GRID_SIZE],
        }
    }

    pub fn from_rows(rows: [u64; GRID_SIZE]) -> Self {
        Self { rows }
    }

    /// 清空脏标记，不修改发布者或上传状态。
    pub fn clear(&mut self) {
        self.rows.fill(0);
    }

    /// 将全部瓦片置脏。
    pub fn set_all(&mut self) {
        self.rows.fill(u64::MAX);
    }

    pub fn extend(&mut self, other: &Self) {
        for (target, source) in self.rows.iter_mut().zip(other.rows) {
            *target |= source;
        }
    }

    /// 取出待发布快照，并将当前位图恢复为空。
    pub fn take_and_clear(&mut self) -> Self {
        std::mem::take(self)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.iter().all(|row| *row == 0)
    }

    /// 单个瓦片是否置脏；坐标越界为 `false`。
    pub fn contains(&self, x: u32, y: u32) -> bool {
        (x as usize) < GRID_SIZE && (y as usize) < GRID_SIZE && self.rows[y as usize] >> x & 1 != 0
    }

    pub fn tile_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.rows
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(row, mut bits)| {
                std::iter::from_fn(move || {
                    if bits == 0 {
                        return None;
                    }
                    let column = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    Some(row * GRID_SIZE + column)
                })
            })
    }

    pub fn rows(&self) -> &[u64; GRID_SIZE] {
        &self.rows
    }

    /// 按行一次 OR 整段列掩码，最多操作 64 个机器字。
    pub fn rasterize_world_rect(&mut self, rect: [f32; 4]) -> DirtyTileRect {
        let cells = world_rect_cells(rect);
        if !cells.is_empty() {
            let mask = column_mask(cells.minimum[0], cells.end[0]);
            for row in &mut self.rows[cells.minimum[1] as usize..cells.end[1] as usize] {
                *row |= mask;
            }
        }
        cells
    }

    /// 置单格；负数或越界坐标忽略。
    pub fn set_cell(&mut self, column: i32, row: i32) {
        if (0..GRID_SIZE as i32).contains(&column) && (0..GRID_SIZE as i32).contains(&row) {
            self.rows[row as usize] |= 1u64 << column;
        }
    }

    /// 标记世界 XZ 平面中的连续笔画。每段至多访问 4096 格，不创建临时点数组。
    pub fn rasterize_world_space_stroke(&mut self, points: &[Vec3], radius: f32) {
        if points.len() < 2 || !radius.is_finite() || radius < 0.0 {
            return;
        }
        let tiles_per_world_unit = GRID_SIZE as f32 / ::utils::float_grid::DEFAULT_WORLD_EXTENT;
        // 小幅外扩防止刚好落在格线上的笔画因舍入漏掉相邻瓦片。
        let radius_tiles = radius * tiles_per_world_unit + 0.000_4;
        let to_tile = |point: Vec3| (world_to_tile(point.x) as f32, world_to_tile(point.z) as f32);
        for segment in points.windows(2) {
            let [start, end] = [segment[0], segment[1]];
            if ![start.x, start.z, end.x, end.z]
                .iter()
                .all(|value| value.is_finite())
            {
                continue;
            }
            let half_extent = ::utils::float_grid::DEFAULT_WORLD_EXTENT * 0.5;
            if start.x.min(end.x) > half_extent + radius
                || start.x.max(end.x) < -half_extent - radius
                || start.z.min(end.z) > half_extent + radius
                || start.z.max(end.z) < -half_extent - radius
            {
                continue;
            }
            // 超大世界坐标已无法表达瓦片大小的平移。范围可能触及世界时，
            // 保守标记全图，避免转换丢掉世界原点偏移后漏掉真正变化的瓦片。
            if [start.x, start.z, end.x, end.z, radius]
                .iter()
                .any(|value| value.abs() > MAX_PRECISE_WORLD_COORDINATE)
            {
                self.set_all();
                return;
            }
            conservative_plot_line_in_bounds(
                to_tile(segment[0]),
                to_tile(segment[1]),
                radius_tiles,
                [0..=GRID_SIZE as i32 - 1, 0..=GRID_SIZE as i32 - 1],
                |column, row| self.set_cell(column, row),
            );
        }
    }

    /// 使用与区域置位一致的范围，按行检测位掩码交集。
    pub fn intersects_world_rect(&self, rect: [f32; 4]) -> bool {
        let cells = world_rect_cells(rect);
        if cells.is_empty() {
            return false;
        }
        let mask = column_mask(cells.minimum[0], cells.end[0]);
        self.rows[cells.minimum[1] as usize..cells.end[1] as usize]
            .iter()
            .any(|row| row & mask != 0)
    }

    /// 按行、再按从低到高的列访问脏格；回调返回 false 则清位。
    pub fn retain(&mut self, mut keep: impl FnMut(u32, u32) -> bool) {
        for (y, row) in self.rows.iter_mut().enumerate() {
            let mut remaining = *row;
            while remaining != 0 {
                let x = remaining.trailing_zeros();
                remaining &= remaining - 1;
                if !keep(x, y as u32) {
                    *row &= !(1u64 << x);
                }
            }
        }
    }
}

/// tabloid 的状态：空位图。
impl State for DirtyTiles {
    fn empty() -> Self {
        Self::new_clear()
    }
}

/// 发布订阅增量按位合并，避免覆盖尚未消费的变更。
impl StateDelta<DirtyTiles> for DirtyTiles {
    fn apply_to(&self, state: &mut DirtyTiles) {
        state.extend(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tabloid::Publisher;

    #[test]
    fn rect_cells_cover_the_complete_world() {
        // 世界两端映射到半开范围 0..64，最后一行和一列也必须覆盖。
        let cells = world_rect_cells([-65.0, -65.0, 65.0, 65.0]);
        assert_eq!(cells.minimum, [0, 0]);
        assert_eq!(cells.end, [64, 64]);
        let mid = world_rect_cells([0.0, 0.0, 0.0, 0.0]);
        assert_eq!(mid.minimum, [32, 32]);
        assert_eq!(mid.end, [32, 32]);
        assert!(mid.is_empty());
    }

    #[test]
    fn rasterize_sets_row_major_bits_with_row_as_y() {
        let mut tiles = DirtyTiles::new_clear();
        // 世界 (0..4, 10..14)：x 格 32..34，y 格 ≥ 32。
        let cells = tiles.rasterize_world_rect([0.0, 10.0, 4.0, 14.0]);
        assert_eq!(cells.minimum[0], 32);
        for y in cells.minimum[1]..cells.end[1] {
            for x in cells.minimum[0]..cells.end[0] {
                assert!(tiles.contains(x, y));
            }
        }
        assert!(!tiles.contains(31, cells.minimum[1]));
        assert!(!tiles.contains(cells.minimum[0], cells.minimum[1] - 1));
    }

    #[test]
    fn intersects_matches_rasterized_region() {
        let mut tiles = DirtyTiles::new_clear();
        tiles.rasterize_world_rect([0.0, 0.0, 10.0, 10.0]);
        assert!(tiles.intersects_world_rect([1.0, 1.0, 3.0, 3.0]));
        assert!(!tiles.intersects_world_rect([-40.0, -40.0, -30.0, -30.0]));
        // 退化矩形经 floor/ceil 仍覆盖所在格（32.12 → [32, 33)），所以会命中。
        assert!(tiles.intersects_world_rect([1.0, 1.0, 1.0, 1.0]));
        // 而恰好落在格线上的零面积矩形（世界 0 → 32.0）为空。
        assert!(!DirtyTiles::new_all_dirty().intersects_world_rect([0.0, 0.0, 0.0, 0.0]));
    }

    #[test]
    fn stroke_marks_the_tiles_along_a_world_space_polyline() {
        let mut tiles = DirtyTiles::new_clear();
        tiles.rasterize_world_space_stroke(
            &[Vec3::new(-10.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)],
            0.0,
        );
        // 世界 z = 0 → 行 32；x 从 -10 到 10 → 列约 27..37。
        for x in 28..=36 {
            assert!(tiles.contains(x, 32), "tile {x}");
        }
        assert!(!tiles.contains(10, 32));
        assert!(!tiles.contains(32, 10));
    }

    #[test]
    fn stroke_needs_two_points_and_clips_to_the_grid() {
        let mut tiles = DirtyTiles::new_clear();
        tiles.rasterize_world_space_stroke(&[Vec3::ZERO], 5.0);
        assert!(tiles.is_empty());
        tiles.rasterize_world_space_stroke(
            &[Vec3::new(500.0, 0.0, 0.0), Vec3::new(600.0, 0.0, 0.0)],
            1.0,
        );
        assert!(tiles.is_empty());
        tiles.set_cell(-1, 3);
        tiles.set_cell(3, 64);
        assert!(tiles.is_empty());
    }

    #[test]
    fn nan_rect_maps_to_zero_cells() {
        let cells = world_rect_cells([f32::NAN, f32::NAN, f32::NAN, f32::NAN]);
        assert_eq!(cells.minimum, [0, 0]);
        assert_eq!(cells.end, [0, 0]);
    }

    #[test]
    fn retain_visits_in_row_then_bit_order_and_clears_rejected() {
        let mut rows = [0u64; GRID_SIZE];
        rows[0] = 0b1010;
        rows[5] = 1 << 63;
        rows[63] = 1;
        let mut tiles = DirtyTiles::from_rows(rows);
        let mut visited = Vec::new();
        tiles.retain(|x, y| {
            visited.push((x, y));
            x != 3
        });
        assert_eq!(visited, [(1, 0), (3, 0), (63, 5), (0, 63)]);
        assert!(tiles.contains(1, 0));
        assert!(!tiles.contains(3, 0));
        assert!(tiles.contains(63, 5));
        assert!(tiles.contains(0, 63));
    }

    #[test]
    fn all_dirty_is_memset_ff_over_512_bytes() {
        let tiles = DirtyTiles::new_all_dirty();
        assert_eq!(std::mem::size_of::<DirtyTiles>(), 0x200);
        assert_eq!(tiles.tile_indices().count(), 4096);
    }

    #[test]
    fn tabloid_delta_ors_per_subscriber() {
        let mut publisher = Publisher::<DirtyTiles>::new();
        let mut subscriber = publisher.subscribe();
        let mut rows = [0u64; GRID_SIZE];
        rows[2] = 0b01;
        publisher.publish(DirtyTiles::from_rows(rows)).unwrap();
        rows[2] = 0b10;
        publisher.publish(DirtyTiles::from_rows(rows)).unwrap();
        let batch = subscriber.take().unwrap();
        assert_eq!(batch.rows()[2], 0b11);
    }

    #[test]
    fn complete_rect_marks_all_tiles_and_far_outside_does_not_mark_edges() {
        let mut tiles = DirtyTiles::new_clear();
        tiles.rasterize_world_rect([-65.0, -65.0, 65.0, 65.0]);
        assert_eq!(tiles, DirtyTiles::new_all_dirty());
        tiles.clear();
        tiles.rasterize_world_rect([500.0, 500.0, 501.0, 501.0]);
        assert!(tiles.is_empty());
        assert_eq!(
            DirtyTiles::cell_from_world_space(Vec2::splat(f32::NAN)),
            None
        );
        assert_eq!(
            DirtyTiles::cell_from_world_space(Vec2::new(-65.01, 0.0)),
            None
        );
        assert_eq!(
            DirtyTiles::cell_from_world_space(Vec2::splat(64.9)),
            Some([63, 63])
        );
    }

    #[test]
    fn extreme_stroke_is_bounded_but_does_not_skip_a_world_crossing() {
        let mut tiles = DirtyTiles::new_clear();
        tiles.rasterize_world_space_stroke(
            &[
                Vec3::new(-f32::MAX, 0.0, 0.0),
                Vec3::new(f32::MAX, 0.0, 0.0),
            ],
            0.0,
        );
        assert_eq!(tiles.rows()[32], u64::MAX);
        tiles.clear();
        tiles.rasterize_world_space_stroke(&[Vec3::splat(3e9); 2], 0.0);
        assert!(tiles.is_empty());
    }
}

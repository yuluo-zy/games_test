//! `country_core::utils::dirty_tiles` — 64×64 位的脏瓦片位图（0x200 B）。
//!
//! 原版唯一的 `DirtyTiles`：建筑碰撞（`recovered_building`）与世界栅格
//! （`resources::world_raster`）共用同一类型。原版符号（B 构建，PDB 权威地址）：
//!
//! | 方法 | 地址 | 级别 |
//! |---|---|---|
//! | `new_clear` | 0x140960ac0 | O，显式清零 512 B |
//! | `rasterize_world_space_rect` | 0x140960ae0 | O，14 B 跳板进 `iter_world_space_rect_cells`（0x140ade8e0） |
//! | `intersects_world_space_rect` | 0x140960af0 | O，跳板进 0x140adec50，内循环 `bt` 测位 |
//! | `clear` | 0x140960c20 | O |
//! | `extend` / `impl StateDelta::apply_to` | 0x140960c30 / 0x140960ef0 | O，两者同大小（694 B），逐字 OR |
//! | `retain<copy_to_staging 闭包>` | 0x140adfcb0 | O，见 [`DirtyTiles::retain`] |
//!
//! 位图布局（O，0x140adfcb0 的 `tzcnt` 循环与 0x140ade8e0 的 `shl/or [rsi+r8*8]`）：
//! 第 `y` 个 u64 字是第 `y` 行瓦片，第 `x` 位是该行第 `x` 列瓦片，瓦片边长 8 px，
//! 世界边长 130。
//!
//! 未恢复：`cell_from_world_space`（0x140960b20）。它有三次 `fmaf` 与一个四通道输出，
//! 参数/返回布局没有对上，不凭猜测落码（U）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::manual_clamp)]

use tabloid::{State, StateDelta};

/// 瓦片边长（像素）。O：`retain` 里 `shl 0xc`（8 行 × 512 B）与 `lea [rdi*8]`。
pub const TILE_SIZE: usize = 8;
/// 位图边长（瓦片）。O：`cmp edx,0x3f`、0x200 B / 8 = 64 个 u64。
pub const GRID_SIZE: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirtyTiles {
    rows: [u64; GRID_SIZE],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirtyTileRect {
    pub minimum: [u32; 2],
    /// Original raster end coordinates are exclusive and clamp to 63.
    pub end: [u32; 2],
}

impl DirtyTileRect {
    pub fn is_empty(self) -> bool {
        self.minimum[0] >= self.end[0] || self.minimum[1] >= self.end[1]
    }
}

/// Native constants at B 142925874/14292587c and 142ae19d0.
/// scale is exactly 0x3efc0fc1, bias32, clamp63; fmaf occurs before rounding.
fn cell_coordinate(world: f32, upper: bool) -> u32 {
    let value = world.mul_add(f32::from_bits(0x3efc0fc1), 32.0);
    let value = if upper { value.ceil() } else { value.floor() };
    // Original CMPORD mask converts NaN to zero before the upper clamp.
    if value.is_nan() {
        0
    } else {
        value.max(0.0).min(63.0) as u32
    }
}

pub fn world_rect_cells(rect: [f32; 4]) -> DirtyTileRect {
    DirtyTileRect {
        minimum: [
            cell_coordinate(rect[0], false),
            cell_coordinate(rect[1], false),
        ],
        end: [
            cell_coordinate(rect[2], true),
            cell_coordinate(rect[3], true),
        ],
    }
}

impl Default for DirtyTiles {
    fn default() -> Self {
        Self::new_clear()
    }
}

impl DirtyTiles {
    /// B new_clear140960ac0 explicitly zeros all512 bytes.
    pub fn new_clear() -> Self {
        Self {
            rows: [0; GRID_SIZE],
        }
    }

    /// 全脏。O：`AllWorldRasterWrite::write` / `clear_all` 对 +0x98 做 `memset(0xff, 0x200)`。
    pub fn new_all_dirty() -> Self {
        Self {
            rows: [u64::MAX; GRID_SIZE],
        }
    }

    pub fn from_rows(rows: [u64; GRID_SIZE]) -> Self {
        Self { rows }
    }

    /// B clear140960c20; this resets mask contents, not owner availability.
    pub fn clear(&mut self) {
        self.rows.fill(0);
    }

    /// 全部置脏（`memset(0xff)`）。
    pub fn set_all(&mut self) {
        self.rows.fill(u64::MAX);
    }

    pub fn extend(&mut self, other: &Self) {
        for (target, source) in self.rows.iter_mut().zip(other.rows) {
            *target |= source;
        }
    }

    /// B flush copies the bitmap then clears it before publishing the copy.
    pub fn take_and_clear(&mut self) -> Self {
        let snapshot = self.clone();
        self.clear();
        snapshot
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
                    Some(row * 64 + column)
                })
            })
    }

    pub fn rows(&self) -> &[u64; GRID_SIZE] {
        &self.rows
    }

    pub fn rasterize_world_rect(&mut self, rect: [f32; 4]) -> DirtyTileRect {
        let cells = world_rect_cells(rect);
        if !cells.is_empty() {
            for z in cells.minimum[1]..cells.end[1] {
                for x in cells.minimum[0]..cells.end[0] {
                    self.rows[z as usize] |= 1u64 << x;
                }
            }
        }
        cells
    }

    /// O：0x140adec50。同一个取整/钳制规则，内循环对每个格 `bt` 测位，命中置标志。
    pub fn intersects_world_rect(&self, rect: [f32; 4]) -> bool {
        let cells = world_rect_cells(rect);
        if cells.is_empty() {
            return false;
        }
        (cells.minimum[1]..cells.end[1])
            .any(|z| (cells.minimum[0]..cells.end[0]).any(|x| self.rows[z as usize] >> x & 1 != 0))
    }

    /// O：0x140adfcb0 `DirtyTiles::retain<copy_to_staging::closure_env$0<512>>`。
    ///
    /// 行（`y`）由小到大，行内由低位到高位（`tzcnt` + `blsr`）逐个置位瓦片调用
    /// `keep(x, y)`；返回 `false` 的瓦片被 `btr` 清位，并把该行字写回位图。
    /// 回调拿到的是当时的位图拷贝，所以回调内的清位不影响本次遍历。
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

/// O：`impl$2::apply_to`（0x140960ef0）与 `extend` 同为 694 B，逐字 OR。
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
    fn rect_cells_use_the_exe_constants() {
        // 世界 0 → 格 32；±65 为世界边界 → 0 / 64 → 钳到 63。
        let cells = world_rect_cells([-65.0, -65.0, 65.0, 65.0]);
        assert_eq!(cells.minimum, [0, 0]);
        assert_eq!(cells.end, [63, 63]);
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
}

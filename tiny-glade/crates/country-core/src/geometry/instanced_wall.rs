//! `instanced_wall.rs` — 砖实例墙的几何载体与上传（B `geometry::instanced_wall`）。
//!
//! 证据（汇编逐条核对，`components-geometry/ALGORITHMS.md` §5）：
//! - `BrickSourceId::from_wall_id` 0x140858750（PDB 名错位，实为 `From<ColorId>`）与 `impl$1::from` 0x140858790
//!   （`From<WallId>`，只用低 32 位）同逻辑：`d = id - 100000`（u32 回绕），`d <= 11` 且 `0x80f >> d` 的低位为 1
//!   （即 `d ∈ {0,1,2,3,11}`）时取表 0x142a953f0 的 `0x80000000 | d`（瞬态楼梯边），否则原样返回；
//! - `with_transient_stair_edge_id` 0x140858780：`(edge_id << 4) | self | 0x40000000`（u32，bit 30 家族位）；
//! - `InstancedWall::new` 0x140858800（33 B）整体搬入；`update` 0x140858830 整体替换砖缓冲（释放旧缓冲）；
//! - `upload_to_storage_buffer` 0x140858880：**取走**砖 Vec（原处留下 moved-from 哨兵）；写入器缺失 → panic；
//!   对稀疏写入器先 `clear` 再逐块 `push`，`push` 失败 → panic `Out of bricks.`（0x142a94b90，源 instanced_wall.rs）。
//!
//! 原版 `InstancedWall` 有两个 `Option` 字段（`+0x20`、`+0x28`）；两者各自的类型在反编译里没有分开，
//! 这里只保留 `upload_to_storage_buffer` 实际用到的那个稀疏写入器，由调用方每次传入。
use crate::recovered_building::wall_state::WallId;
use crate::render::sparse_instance_buffer::{SparseInstanceBufferWriter, SparseSsbo};
use crate::resources::color_id::ColorId;

/// 墙砖来源 id（O：`u32`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BrickSourceId(pub u32);

/// 瞬态楼梯边的高位标志（O：`i32::MIN`）。
pub const TRANSIENT_STAIR_FLAG: u32 = 0x8000_0000;
/// 查表 0x142a953f0（O：12 项；只有 `{0,1,2,3,11}` 会被取到）。
const TRANSIENT_STAIR_TABLE: [u32; 12] = [
    0x8000_0000,
    0x8000_0001,
    0x8000_0002,
    0x8000_0003,
    0x8000_0000,
    0x8000_0000,
    0x8000_0000,
    0x8000_0000,
    0x8000_0000,
    0x8000_0000,
    0x8000_0000,
    0x8000_000b,
];

fn encode(raw: u32) -> u32 {
    let d = raw.wrapping_sub(100_000);
    if d <= 11 && (0x80f_u32 >> d) & 1 != 0 {
        TRANSIENT_STAIR_TABLE[d as usize]
    } else {
        raw
    }
}

impl BrickSourceId {
    /// 0x140858750（`From<ColorId>`）。
    pub fn from_color_id(id: ColorId) -> Self {
        Self(encode(id.0))
    }

    /// 0x140858790（`From<WallId>`，低 32 位）。
    pub fn from_wall_id(id: WallId) -> Self {
        Self(encode(id.0 as u32))
    }

    /// 0x140858780：`(edge_id << 4) | self | 0x40000000`。
    pub fn with_transient_stair_edge_id(self, edge_id: u32) -> u32 {
        (edge_id << 4) | self.0 | 0x4000_0000
    }
}

/// 砖实例墙。
#[derive(bevy_ecs::component::Component, Clone, Debug)]
pub struct InstancedWall<T> {
    pub id: u64,
    pub brick_transforms: Vec<T>,
}

impl<T> InstancedWall<T> {
    /// 0x140858800。
    pub fn new(id: u64, brick_transforms: Vec<T>) -> Self {
        Self {
            id,
            brick_transforms,
        }
    }

    /// 0x140858830：整体替换砖缓冲，旧缓冲被释放。
    pub fn update(&mut self, brick_transforms: Vec<T>) {
        self.brick_transforms = brick_transforms;
    }
}

impl<T: SparseSsbo + Copy> InstancedWall<T> {
    /// 0x140858880：取走砖缓冲，写入器缺失 panic，`clear` 后逐块 `push`，空间不足 panic `Out of bricks.`。
    pub fn upload_to_storage_buffer(
        &mut self,
        writer: Option<&mut SparseInstanceBufferWriter<'_, T>>,
    ) {
        let bricks = std::mem::take(&mut self.brick_transforms);
        let writer = writer.unwrap();
        writer.clear();
        for brick in &bricks {
            if writer.push(brick).is_err() {
                panic!("Out of bricks.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_five_transient_stair_ids_are_remapped() {
        for (d, expect) in [
            (0u32, 0x8000_0000u32),
            (1, 0x8000_0001),
            (2, 0x8000_0002),
            (3, 0x8000_0003),
            (11, 0x8000_000b),
        ] {
            assert_eq!(
                BrickSourceId::from_wall_id(WallId(100_000 + u64::from(d))).0,
                expect,
                "d {d}"
            );
            assert_eq!(
                BrickSourceId::from_color_id(ColorId(100_000 + d)).0,
                expect,
                "d {d}"
            );
        }
        for d in [4u32, 5, 6, 7, 8, 9, 10, 12] {
            let raw = 100_000 + d;
            assert_eq!(BrickSourceId::from_color_id(ColorId(raw)).0, raw, "d {d}");
        }
    }

    #[test]
    fn ordinary_and_out_of_range_ids_pass_through() {
        for raw in [0u32, 1, 99_999, 100_012, 5_000_000, u32::MAX] {
            assert_eq!(BrickSourceId::from_color_id(ColorId(raw)).0, raw);
            assert_eq!(BrickSourceId::from_wall_id(WallId(u64::from(raw))).0, raw);
        }
    }

    #[test]
    fn wall_ids_use_only_the_low_32_bits() {
        let wall = WallId((7u64 << 32) | 100_001);
        assert_eq!(BrickSourceId::from_wall_id(wall).0, 0x8000_0001);
    }

    #[test]
    fn transient_edge_ids_set_the_bit_30_family_flag() {
        assert_eq!(
            BrickSourceId(0x8000_0001).with_transient_stair_edge_id(3),
            0xC000_0031
        );
        assert_eq!(
            BrickSourceId(0).with_transient_stair_edge_id(0),
            0x4000_0000
        );
        assert_eq!(
            BrickSourceId(5).with_transient_stair_edge_id(0xf),
            0x4000_00f5
        );
    }

    #[test]
    fn table_agrees_with_the_mask_for_every_index() {
        for d in 0..12u32 {
            let selected = (0x80f_u32 >> d) & 1 != 0;
            assert_eq!(selected, matches!(d, 0 | 1 | 2 | 3 | 11), "d {d}");
        }
    }
}

//! read_bush_positions_from_gpu（B 主基线；0x140eebaa0，size 745）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`work/restorer-clutter-remaining/evidence/decomp/0x140eebaa0_*.c`、
//! `evidence/disasm0.txt`、`evidence/disasm3-result.json`、`evidence/disasm4-result.json`）。
//!
//! 真实控制流（O）：
//! 1. `FrozenMap<SsboId, Box<BufferState>>::get(SsboId=0x0deff5ab6a35972a)`；命中后断言
//!    类型名 `country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo`
//!    长度 0x5c，否则 `assert_failed`（0x142f3b328；SsboId 常量实读 0x142f3b268）。
//! 2. 取回读数据 `{ data ptr, len }`；`len==0` → `panic_bounds_check`；
//!    `count = *(u32*)data`；`len <= count` → `slice_end_index_len_fail`。
//!    （GPU 回读缓冲头：前 4 B 为 count，位置数组从 +8 起、stride 8 的 Vec2。）
//! 3. 写 change tick（ResMut 管道），随后把懒网格每格 `positions` 长度清零
//!    （记录 stride 0x20：Vec {cap@0,ptr@8,len@0x10} + hash@0x18）。
//! 4. 对每个 bush 位置：`Aabb2::from_center_radius(center, 0.7)`（半径常量 0x1429cd030）
//!    → `iter_world_space_rect_cells`（0x141cf3ee0）。该遍历对矩形覆盖的每个格
//!    `idx = x*64 + y`（0x140aac630：`((cell>>26)&0xffffffc0)+cell`）把 **bush 位置**
//!    追加进 `grid[idx].positions`。
//! 5. 逐格：`positions.len()>1` → 排序（len<0x15 插入排序、否则 ipnsort；`sort_unstable`
//!    的两种分派），`hash = calculate_hash<&[Vec2]>`（0x141cf7270，SipHash-1-3 零键：
//!    写 len、随后每点写 2 与两个 f32 位型），hash 变化 → 置脏位 **idx**（0x140833250
//!    为恒等：位号=记录下标；0x1000 越界 panic）并标记 changed。
//! 6. changed → 把 0x200B 位图 `Publisher<DirtyTiles>::publish`（0x141cf6500）。
//!
//! U 登记：GPU 回读本身（FrozenMap/设备类型断言）为宿主渲染边界，本模块以
//! [`BushCpuSsbo`] 资源承载已回读数据；缓冲头偏移（+0 为 count）按机器码建模，
//! 资源→真实 GPU 映射不在本 crate。

use bevy_ecs::prelude::*;
use country_core::utils::dirty_tiles::{world_rect_cells, DirtyTiles};
use glam::Vec2;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::populate_autoclutter_regions::DirtyTilesPublisher;

/// 回读 SSBO 类型名（O，0x142f3b328，0x5c B）。
pub const BUSH_CPU_SSBO_NAME: &str =
    "country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo";
/// 回读 SSBO 哈希键（O，0x142f3b268：0x0deff5ab6a35972a）。
pub const BUSH_CPU_SSBO_ID: u64 = 0x0deff5ab_6a35972a;
/// `Aabb2::from_center_radius` 半径（O，0x1429cd030 = f32 0.7）。
pub const BUSH_CELL_RADIUS: f32 = 0.7;
/// 懒网格格数（64×64；位图 0x200 B / 0x1000 位）。
pub const BUSH_GRID_CELLS: usize = 64 * 64;

/// 灌木 CPU 回读缓冲（GPU→CPU 契约；宿主 render_loop 填充）。
///
/// 二进制布局：`data[0]=count(u32)`，位置数组自 `data+8` 起（stride 8 的 Vec2）。
/// 本资源按该布局建模：`count` 为头部计数，`positions.len()` 为切片长度。
#[derive(Resource, Clone, Debug, Default)]
pub struct BushCpuSsbo {
    pub count: u32,
    pub positions: Vec<Vec2>,
}

impl BushCpuSsbo {
    /// 头部类型断言（O：len==0x5c && memcmp 名称）。
    pub fn assert_type(len: usize, name: &str) {
        assert!(
            len == BUSH_CPU_SSBO_NAME.len() && name == BUSH_CPU_SSBO_NAME,
            "read_bush_positions_from_gpu: BushCpuSsbo type assertion failed"
        );
    }
}

/// 兼容旧名（宿主装配）。
pub type BushPositions = BushCpuSsbo;

/// 懒网格单格：本帧 bush 位置表 + 上一帧哈希（O 记录 stride 0x20）。
#[derive(Clone, Debug, Default)]
pub struct BushGridCell {
    pub positions: Vec<Vec2>,
    pub hash: u64,
}

/// 懒网格（64×64；记录下标 = `x*64 + y`，O 0x140aac630）。
#[derive(Resource, Clone, Debug)]
pub struct BushGrid {
    pub cells: Vec<BushGridCell>,
}

impl Default for BushGrid {
    fn default() -> Self {
        Self {
            cells: vec![BushGridCell::default(); BUSH_GRID_CELLS],
        }
    }
}

/// Vec2 序列哈希（O 0x141cf7270：SipHash-1-3 零键，写 len，再逐点写 2 与两个 f32 位型）。
pub fn hash_bush_positions(positions: &[Vec2]) -> u64 {
    let mut h = DefaultHasher::new();
    positions.len().hash(&mut h);
    for p in positions {
        2usize.hash(&mut h);
        p.x.to_bits().hash(&mut h);
        p.y.to_bits().hash(&mut h);
    }
    h.finish()
}

/// read_bush_positions_from_gpu（0x140eebaa0）：GPU 回读灌木位置 → 懒网格分桶 →
/// 哈希去重 → 发布 DirtyTiles。
pub fn read_bush_positions_from_gpu(
    bush: Res<BushCpuSsbo>,
    mut grid: ResMut<BushGrid>,
    mut publisher: ResMut<DirtyTilesPublisher>,
) {
    // 2. 切片边界（O：puVar3[1]==0 → panic_bounds_check；len<=count → slice_end_index_len_fail）。
    let len = bush.positions.len();
    if len == 0 {
        panic!("read_bush_positions_from_gpu: panic_bounds_check (empty buffer)");
    }
    let count = bush.count as usize;
    if len <= count {
        panic!("read_bush_positions_from_gpu: slice_end_index_len_fail");
    }

    // 1. 每格 positions.len 清零（O：`*(u64*)(data + i*0x20 + 0x10)=0`）。
    for cell in grid.cells.iter_mut() {
        cell.positions.clear();
    }

    // 3. 逐 bush：Aabb2::from_center_radius(center, 0.7) → 每覆盖格追加 bush 位置。
    for pos in bush.positions.iter().take(count) {
        let rect = [
            pos.x - BUSH_CELL_RADIUS,
            pos.y - BUSH_CELL_RADIUS,
            pos.x + BUSH_CELL_RADIUS,
            pos.y + BUSH_CELL_RADIUS,
        ];
        let cells = world_rect_cells(rect);
        if cells.is_empty() {
            continue;
        }
        for y in cells.minimum[1]..cells.end[1] {
            for x in cells.minimum[0]..cells.end[0] {
                let idx = x as usize * 64 + y as usize;
                grid.cells[idx].positions.push(*pos);
            }
        }
    }

    // 4. 逐格排序 + 哈希比较 + 置脏位（位号 = 格下标，O）。
    let mut dirty = DirtyTiles::new_clear();
    let mut changed = false;
    for (idx, cell) in grid.cells.iter_mut().enumerate() {
        if cell.positions.len() > 1 {
            // O：`sort_unstable`（len<0x15 insertion_sort_shift_left，否则 ipnsort）；
            // comparator 为 `[f32;2]` 序（I：本实现按 total order，等价于同一确定性全序）。
            cell.positions.sort_unstable_by(|a, b| {
                a.x.total_cmp(&b.x).then_with(|| a.y.total_cmp(&b.y))
            });
        }
        let hash = hash_bush_positions(&cell.positions);
        if cell.hash != hash {
            cell.hash = hash;
            // 位号 = 记录下标：DirtyTiles 位图行 = idx/64、列 = idx%64。
            dirty.set_cell((idx % 64) as i32, (idx / 64) as i32);
            changed = true;
        }
    }
    // 5. changed → publish（O：memcpy 0x200 + Publisher::publish<DirtyTiles>）。
    if changed {
        let _ = publisher.0.publish(dirty);
    }
}

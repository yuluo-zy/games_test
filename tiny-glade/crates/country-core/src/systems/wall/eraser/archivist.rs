//! B `systems/wall/eraser/archivist.rs`: wall eraser archivist（3-map 装配）。
//!
//! fresh 反编译定案（2026-10-17；`impl-walls-return/decomp/14092f990-finish.txt` 等，
//! 本任务轮 fresh 复核 `restorer-eraser-fresh-result.json` 同式，未变）：
//! - `WallEraserArchivist` = 3 张 hashbrown map（各带 `RandomState`，每 map 0x30B）：
//!   map1 @+0x00 `HashMap<OriginalWallId, RewireEnclosedWall>`（条目 0x10B）；
//!   map2 @+0x30 `HashMap<OriginalWallId, EraserWallState>`（条目 0xe8B = 0x10 键 + 0xd8 载荷）；
//!   map3 @+0x60 `HashMap<RewireEnclosedWall, ChildWalls>`（条目 0x90B = 0x10 键 + 0x80 载荷）。
//!   `new` 0x14092e6e0 三 map 均以 `&anon_...` 哨兵 + 0 初始化 + 独立 RandomState 键。
//! - 5 事件方法：`record_rewiring_event` 0x14092e8e0（map1 insert + 旧值 drop）/
//!   `cache_if_original_wall` 0x14092ea50（map1 已存在 → return；map3 已存在 → panic；
//!   克隆源 → `EraserWallState{id, 1.0/0.0}` 插 map2 → map1 插 `(id, id)`）/
//!   `shrink_wall` 0x14092eff0（map1 → map3 → 按 child id 找 ChildWall → `slice` 改写 a/b）/
//!   `add_split_event` 0x14092f280（同 + `swap_remove` + 逐段 `slice` 重插 + map1 插新段）/
//!   `add_fully_erased_event` 0x14092f630（同 + 空桶重写 0x8000000000000000）。
//! - `finish` 0x14092f990（11509B）：drain map2 → 0xf8B `WallHistoryEdit` 变体 0 桶
//!   （local_5b8，op 槽 @+0xe8 = 2）；drain map3 每 entry 两分支：
//!   shrink/split → 变体 0 桶 local_570 + 变体 2 桶 local_558；全擦除 → 变体 1 桶
//!   local_588 + 变体 2 桶 local_5a0；Sip13 inner-walls 探针改写 local_570；末段 5 桶
//!   → 逐条 `Arc<WallHistoryEdit>`（0x108 = 0x10 Arc 头 + 0xf8 载荷）→
//!   `from_iter<ConcurrentEdits>` → `History::add(tag 0x7)` → `RawTable::clear ×2`
//!   （map1 + map2）。
//!
//! history 侧 0xf8B 布局 + 17 变体判别表已由 restorer-walledit 定案
//! （`wall_history_edit.rs`），本文件只做 walls 域装配（消费方勿动）。
//!
//! 缺类型登记（不编造）：
//! - `EraserWallState` / `ChildWalls` 的**内层字段映射**：finish 把 0xd8B/0x80B 载荷
//!   逐偏移读入 0xf8B 记录（载荷 +0x58..+0xd0 各 qword 语义未逐字段取证）。本重建以
//!   `WallAuthoritySnapshot`（原始快照）+ `ErasedRange` 为结构最近拟，载荷字节级
//!   逐偏移装配归 `walls 域待取证`。
//! - finish 的 **Sip13 inner-walls 探针**（`core::hash::sip::impl$5::write<Sip13Rounds>` +
//!   -0x78 stride 表）：需 inner-walls map 类型 + SipHash13 常量，非 walls 域现有类型，
//!   登记不实现。
//! - `ChildWall` 在 archivist 内的记录形态（shrink_wall 对 `puVar6[2]/[3]` 的改写面）
//!   以 `utils::child_wall::ChildWall {id, a, b}` + `slice(id, t1, t2)` 为锚定形态。

use bevy_ecs::prelude::*;
use hashbrown::HashMap;
use std::sync::Arc;

use crate::resources::color_id::{HalftimberColorIds, WallColorIds};
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::wall_edit::WallAuthoritySnapshot;
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::history::wall_edit::WallHistoryEdit;
use crate::resources::history::wall_edit::WallSnapshotError;
use crate::resources::history::wall_edit::slice_source_curve;
use crate::resources::walls::publication_fields::WallId as RecoveredWallId;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::{
    roofs::{Roof, commands::RecordRoofHistory},
    walls::{InnerWalls, WallId, public_walls::PublicWalls},
};
use crate::systems::wall::history_recording::RecordWallHistory;
use crate::utils::child_wall::ChildWall;

/// 原生 `OriginalWallId`（archivist map1/map2 key；u64 newtype，Ghidra 常折叠为
/// `systems::ui::GlobalId`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OriginalWallId(pub u64);

/// 原生 `RewireEnclosedWall`（map1 value == map3 key；u64 newtype）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RewireEnclosedWall(pub u64);

/// Erased range captured per affected wall.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ErasedRange {
    pub low: f32,
    pub high: f32,
}

/// 原生 `EraserWallState`（map2 value，0xd8B 载荷；cache_if_original_wall 写
/// `{id, 0x3f80000000000000}` 起始 + 两个克隆 Vec，finish 把 0xd8B 直接读入记录载荷）。
/// 本型以原始 authority 快照 + 擦除区间为结构最近拟；字节级字段映射登记缺类型。
#[derive(Clone, Debug)]
pub struct EraserWallState {
    pub wall: OriginalWallId,
    pub original: WallAuthoritySnapshot,
    pub range: ErasedRange,
}

/// 原生 `ChildWalls`（map3 value，0x80B 载荷；含 `Vec<ChildWall>` + 墙态字段）。
/// `remove`（swap_remove）/`extend`（逐段 slice 重插）/`is_empty` 为原 inline 方法
/// （inventory line 73/82/98）。本型以 `WallAuthoritySnapshot` + children 为结构最近拟。
///
/// `fully_erased`：原生 map3 载荷 +0x70 判别字段（正数 = shrink/split；空桶重写
/// `0x8000000000000000` = 全擦除）。本重建以 bool 为判别最近拟，字段编码登记缺类型。
#[derive(Clone, Debug)]
pub struct ChildWalls {
    pub wall: RewireEnclosedWall,
    pub original: WallAuthoritySnapshot,
    pub children: Vec<ChildWall>,
    pub fully_erased: bool,
}

impl ChildWalls {
    /// 原生 inline（line 73）：`swap_remove` 语义——把末位 child 抄入被移除槽。
    pub fn remove(&mut self, id: u64) -> Option<ChildWall> {
        let idx = self.children.iter().position(|c| c.id == id)?;
        Some(self.children.swap_remove(idx))
    }

    /// 原生 inline（line 82）：逐段 slice 重插（add_split_event 的 in_R9 段）。
    pub fn extend(&mut self, iter: impl IntoIterator<Item = ChildWall>) {
        self.children.extend(iter);
    }

    /// 原生 inline（line 98）：add_fully_erased_event 的空桶改写判别。
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

/// B `WallEraserArchivist`（原生 3-map；`new` 0x14092e6e0）。
#[derive(Resource, Clone, Debug)]
pub struct WallEraserArchivist {
    /// map1：`OriginalWallId → RewireEnclosedWall`（record_rewiring_event /
    /// cache_if_original_wall 插；shrink/split 循此找 child 所属墙）。
    map1: HashMap<OriginalWallId, RewireEnclosedWall>,
    /// map2：`OriginalWallId → EraserWallState`（cache_if_original_wall 插；finish drain）。
    map2: HashMap<OriginalWallId, EraserWallState>,
    /// map3：`RewireEnclosedWall → ChildWalls`（child 墙列表）。
    map3: HashMap<RewireEnclosedWall, ChildWalls>,
}

impl WallEraserArchivist {
    /// B new（0x14092e6e0）：三空 map + 独立 RandomState。
    pub fn new() -> Self {
        Self::default()
    }

    /// Inline B line 39（inventory）。
    pub fn affected_walls(&self) -> impl Iterator<Item = WallId> + '_ {
        self.map2.keys().map(|k| WallId(k.0))
    }

    /// Inline B line 43（inventory）。
    pub fn has_anything_been_affected(&self) -> bool {
        !self.map2.is_empty()
    }

    /// B record_rewiring_event（0x14092e8e0）：map1 insert（旧值 drop）。
    pub fn record_rewiring_event(&mut self, wall: OriginalWallId, target: RewireEnclosedWall) {
        self.map1.insert(wall, target);
    }

    /// B cache_if_original_wall（0x14092ea50）：cache 首个原始快照。
    ///
    /// fresh：map1 已存在该墙 → return；map3 已存在该墙 → panic；克隆源 → 组装
    /// `EraserWallState` 插 map2；map1 插 `(id, id)`。
    pub fn cache_if_original_wall(
        &mut self,
        wall: WallId,
        snapshot: &WallAuthoritySnapshot,
        range: ErasedRange,
    ) {
        let id = OriginalWallId(wall.0);
        if self.map1.contains_key(&id) {
            return;
        }
        if self.map3.contains_key(&RewireEnclosedWall(wall.0)) {
            panic!("wall eraser: wall already present in ChildWalls during cache_if_original_wall");
        }
        self.map2.insert(
            id,
            EraserWallState {
                wall: id,
                original: snapshot.clone(),
                range,
            },
        );
        // 0x14092ee80：`HashMap<RewireEnclosedWall, ChildWalls>::insert`（map3 @+0x60）。载荷为
        // 原墙快照的克隆 + 单元素 `Vec<ChildWall>`：`{id, 0.0, 1.0}`（0x14092ee16..21：id 取自
        // 调用参数，其后 qword 来自常量 0x1429345b0 = f32 0.0 / 1.0），判别字段为正（shrink/split）。
        // 此前本函数漏掉这一插入，split/shrink/erase 事件因 map3 无条目而无目标。
        self.map3.insert(
            RewireEnclosedWall(wall.0),
            ChildWalls {
                wall: RewireEnclosedWall(wall.0),
                original: snapshot.clone(),
                children: vec![ChildWall::new(wall.0, 0.0, 1.0)],
                fully_erased: false,
            },
        );
        self.map1.insert(id, RewireEnclosedWall(wall.0));
    }

    /// B shrink_wall（0x14092eff0）：map1 → map3 → 按 child id 找 ChildWall → `slice`
    /// 改写 a/b。
    ///
    /// fresh：map1 探针（key == id）→ map3 探针（key == map1 值）→ 逐 child 找
    /// `id` → `ChildWall::slice(id, t1, t2)` 写回 a/b；未命中 map1/map3/child 均
    /// `expect/unwrap_failed`。
    pub fn shrink_wall(&mut self, id: OriginalWallId, t1: f32, t2: f32) {
        let target = self
            .map1
            .get(&id)
            .copied()
            .expect("shrink_wall: wall not in rewire map");
        let child_walls = self
            .map3
            .get_mut(&target)
            .expect("shrink_wall: rewire target has no ChildWalls");
        let idx = child_walls
            .children
            .iter()
            .position(|c| c.id == id.0)
            .unwrap_or_else(|| panic!("shrink_wall: child wall {id:?} not found"));
        let sliced = child_walls.children[idx].slice(id.0, t1, t2);
        child_walls.children[idx].a = sliced.a;
        child_walls.children[idx].b = sliced.b;
    }

    /// B add_split_event（0x14092f280）：同 shrink 寻址 + `swap_remove` 被拆 child +
    /// 逐段 `slice` 重插 + 每新段 map1 插 `(段 id, 段 id)`。
    ///
    /// fresh：map1 → map3 → 按 child id `swap_remove`（末位抄入槽）→ reserve → 对
    /// `in_R9` 段逐段 `ChildWall::slice` push → 每段一次 `HashMap::insert<GlobalId,GlobalId>`
    /// （map1 插新段）。
    pub fn add_split_event(
        &mut self,
        id: OriginalWallId,
        t1: f32,
        t2: f32,
        new_count: usize,
    ) {
        let target = self
            .map1
            .get(&id)
            .copied()
            .expect("add_split_event: wall not in rewire map");
        let child_walls = self
            .map3
            .get_mut(&target)
            .expect("add_split_event: rewire target has no ChildWalls");
        let removed = child_walls
            .remove(id.0)
            .unwrap_or_else(|| panic!("add_split_event: child wall {id:?} not found"));
        let mut pieces = Vec::with_capacity(new_count);
        for _ in 0..new_count {
            pieces.push(removed.slice(id.0, t1, t2));
        }
        child_walls.extend(pieces);
        for _ in 0..new_count {
            // 原生 `HashMap::insert<GlobalId, GlobalId>`（每新段一次）：新段以自身为
            // rewire 目标登记 map1。
            self.map1.insert(id, RewireEnclosedWall(id.0));
        }
    }

    /// B add_fully_erased_event（0x14092f630）：同寻址 + `swap_remove` child；空桶
    /// 改写 0x8000000000000000（变体 1 CreateWall 判别）。
    pub fn add_fully_erased_event(&mut self, id: OriginalWallId) -> Option<ChildWall> {
        let target = self
            .map1
            .get(&id)
            .copied()
            .expect("add_fully_erased_event: wall not in rewire map");
        let child_walls = self
            .map3
            .get_mut(&target)
            .expect("add_fully_erased_event: rewire target has no ChildWalls");
        let removed = child_walls
            .remove(id.0)
            .unwrap_or_else(|| panic!("add_fully_erased_event: child wall {id:?} not found"));
        if child_walls.is_empty() {
            // 原生空桶重写：载荷 +0x70 = `0x8000000000000000`（变体 1 CreateWall 判别）、
            // +0x78 = 被移除 child 值、+0x80 = 0。本重建以 `fully_erased = true` 为
            // 判别最近拟（编码登记缺类型）。
            child_walls.fully_erased = true;
        }
        Some(removed)
    }

    /// B finish（0x14092f990）：drain map2 + map3 → 5 桶 → `WallCompoundHistoryEdit`。
    ///
    /// 逐分支（fresh）：
    /// 1. drain map2（EraserWallState，0xd8B 载荷）→ 0xf8B 变体 0 桶 local_5b8
    ///    （op 槽 @+0xe8 = 2）。
    /// 2. drain map3（ChildWalls，0x90B 条目）每 entry：
    ///    - shrink/split：初始区间 `0.5 ± 0.5·(0.2/height)`（高度夹 0.2、[0,1] 断言 +
    ///      NaN panic）→ `Curve::crashy_slice` → 变体 0 桶 local_570 + 变体 2 桶
    ///      local_558；child 排序（smallsort/driftsort）后逐段中点
    ///      `mid = cur.a·0.5 + prev.b·0.5`（[0,1] 断言 + NaN panic）→ 末位 end=1.0 或
    ///      `next.a·0.5 + cur.b·0.5` → `mid < end` 断言 → `crashy_slice` →
    ///      `inverse_lerp`×2（NAN/INF panic）→ clamp → 装配变体 0。
    ///    - 全擦除：`children.len() < 2 || height <= 0` → panic；child 排序；逐段改写
    ///      变体 1 桶 local_588 + 变体 2 桶 local_5a0。
    /// 3. Sip13 inner-walls 探针（命中改写 local_570）——**缺类型登记**（需 inner-walls
    ///    map + SipHash13），本重建不实现。
    /// 4. 5 桶 → `WallCompoundHistoryEdit{groups}`（history 侧 5 组 ConcurrentEdits，
    ///    空桶保留为空组）→ 清 map1 + map2。
    ///
    /// 返回复合编辑，由 `finish_wall_eraser` 经 `RecordWallHistory` 交 history 域
    /// （等价原生 `History::add(tag 0x7)`）。
    pub fn finish(
        &mut self,
        wall_colors: &WallColorIds,
        fence_colors: &HalftimberColorIds,
    ) -> WallCompoundHistoryEdit {
        let edits = self.compile_eraser_buckets(wall_colors, fence_colors);
        self.map1.clear();
        self.map2.clear();
        edits
    }

    /// finish 的 5 桶装配（record_erase_original_wall / finish_shrink_or_split_event /
    /// finish_erase_event 内联，inventory line 259/353/540）。
    fn compile_eraser_buckets(
        &mut self,
        wall_colors: &WallColorIds,
        fence_colors: &HalftimberColorIds,
    ) -> WallCompoundHistoryEdit {
        let mut bucket_5b8 = Vec::new(); // drain map2 → 变体 0
        let mut bucket_570 = Vec::new(); // map3 shrink/split → 变体 0
        let mut bucket_558 = Vec::new(); // map3 shrink/split → 变体 2 Delete
        let mut bucket_588 = Vec::new(); // map3 全擦除 → 变体 1 Create
        let mut bucket_5a0 = Vec::new(); // map3 全擦除 → 变体 2 Delete

        // 1. drain map2：每 EraserWallState → 变体 0（op=2）。
        for (id, state) in self.map2.drain() {
            bucket_5b8.push(erase_original_state_change(id, &state));
        }

        // 2. drain map3：每 ChildWalls 按判别字段两分支。
        for (_target, child_walls) in self.map3.drain() {
            if child_walls.fully_erased {
                finish_erase_event(
                    &child_walls,
                    wall_colors,
                    fence_colors,
                    &mut bucket_570,
                    &mut bucket_588,
                    &mut bucket_5a0,
                );
            } else {
                finish_shrink_or_split_event(
                    &child_walls,
                    wall_colors,
                    fence_colors,
                    &mut bucket_570,
                    &mut bucket_558,
                );
            }
        }

        // 3. Sip13 inner-walls 探针：登记缺类型（不实现，见模块头）。

        // 4. 5 桶 → groups（插入序保留；空桶保留为空组）。
        WallHistoryEdits {
            bucket_5b8,
            bucket_570,
            bucket_558,
            bucket_588,
            bucket_5a0,
        }
        .compile_history_edit()
    }
}

/// Additive faithful face of `add_split_event` (0x14092f280) used by
/// `wall_splitter`: the original takes the full piece list `(new id, t1, t2)`,
/// not a count.
impl WallEraserArchivist {
    /// True when `add_split_event_pieces(wall, ..)` can find its rewire target
    /// and child wall (map1 -> map3 -> child with `id == wall`).
    pub fn has_split_target(&self, wall: WallId) -> bool {
        self.map1
            .get(&OriginalWallId(wall.0))
            .and_then(|target| self.map3.get(target))
            .is_some_and(|child_walls| child_walls.children.iter().any(|c| c.id == wall.0))
    }

    /// B add_split_event (0x14092f280): map1 -> map3 (both `expect`/unwrap
    /// failures as in the original), `swap_remove` the child whose id is
    /// `wall`, push `removed.slice(piece.id, t1, t2)` for every piece, then
    /// `map1.insert(piece.id, target)` for every piece.
    pub fn add_split_event_pieces(&mut self, wall: WallId, pieces: &[(u64, f32, f32)]) {
        let id = OriginalWallId(wall.0);
        let target = self
            .map1
            .get(&id)
            .copied()
            .expect("add_split_event: wall not in rewire map");
        let child_walls = self
            .map3
            .get_mut(&target)
            .expect("add_split_event: rewire target has no ChildWalls");
        let removed = child_walls
            .remove(wall.0)
            .unwrap_or_else(|| panic!("add_split_event: child wall {id:?} not found"));
        child_walls.extend(
            pieces
                .iter()
                .map(|&(piece, t1, t2)| removed.slice(piece, t1, t2)),
        );
        for &(piece, _, _) in pieces {
            self.map1.insert(OriginalWallId(piece), target);
        }
    }
}

impl Default for WallEraserArchivist {
    fn default() -> Self {
        Self {
            map1: HashMap::new(),
            map2: HashMap::new(),
            map3: HashMap::new(),
        }
    }
}

/// 原生 5 桶（local_5b8/570/558/588/5a0）→ `WallCompoundHistoryEdit`（5 组
/// ConcurrentEdits；inventory `WallHistoryEdits::compile_history_edit` line 633）。
pub struct WallHistoryEdits {
    bucket_5b8: Vec<WallHistoryEdit>,
    bucket_570: Vec<WallHistoryEdit>,
    bucket_558: Vec<WallHistoryEdit>,
    bucket_588: Vec<WallHistoryEdit>,
    bucket_5a0: Vec<WallHistoryEdit>,
}

impl WallHistoryEdits {
    /// native finish 末段：5 桶 → 逐条 `Arc<WallHistoryEdit>`（0x108）→
    /// `from_iter<ConcurrentEdits>` → `History::add(tag 0x7)`。Rust 以
    /// `WallCompoundHistoryEdit.groups` 承载 5 组（`ConcurrentEdits` 语义由
    /// history 侧定义）。
    pub fn compile_history_edit(self) -> WallCompoundHistoryEdit {
        WallCompoundHistoryEdit {
            groups: vec![
                self.bucket_5b8,
                self.bucket_570,
                self.bucket_558,
                self.bucket_588,
                self.bucket_5a0,
            ],
        }
    }
}

/// 变体 0 `observed_operation` 载荷 `{op u32, start f32, end f32}`（原生
/// `u32 @T+0xe8` / `f32 @T+0xec` / `f32 @T+0xf0`）。
fn observed_operation(op: u32, start: f32, end: f32) -> [u8; 12] {
    let mut bytes = [0u8; 12];
    bytes[..4].copy_from_slice(&op.to_le_bytes());
    bytes[4..8].copy_from_slice(&start.to_le_bytes());
    bytes[8..12].copy_from_slice(&end.to_le_bytes());
    bytes
}

/// 擦除后的 authority 快照：曲线按 [low, high] 切片 + freehand shape（tag 2）。
fn erased_snapshot(
    previous: &WallAuthoritySnapshot,
    range: ErasedRange,
) -> Result<WallAuthoritySnapshot, WallSnapshotError> {
    let mut next = previous.clone();
    next.source.curve = slice_source_curve(&previous.source.curve, range.low, range.high)?;
    next.source.shape = WallShapeSnapshot::from_observed({
        let mut bytes = [0u8; 28];
        bytes[..4].copy_from_slice(&2u32.to_le_bytes());
        bytes
    });
    next.source.curve_kind_tag = 2;
    Ok(next)
}

/// `record_erase_original_wall`（inventory line 259）：map2 drain → 变体 0（op=2）。
fn erase_original_state_change(id: OriginalWallId, state: &EraserWallState) -> WallHistoryEdit {
    let next = erased_snapshot(&state.original, state.range)
        .unwrap_or_else(|_| state.original.clone());
    WallHistoryEdit::StateChange {
        wall: RecoveredWallId(id.0),
        previous: state.original.clone(),
        next,
        observed_operation: observed_operation(2, 0.0, 0.0),
    }
}

/// finish（0x14092f990）装配 Delete/Create 时的 color_id 来源：原版按原墙快照 `source +0x65`
/// 标志选表（0 = `WallColorIds`，非 0 = `HalftimberColorIds`），以 child 墙 id 为下标取值，
/// 越界走 `panic_bounds_check`（0x140932..：`pauStack_148 <= idx`）。此前写死 0。
fn child_color_id(
    original: &WallAuthoritySnapshot,
    child: u64,
    wall_colors: &WallColorIds,
    fence_colors: &HalftimberColorIds,
) -> u32 {
    let table = if original.source.clutter_version != 0 {
        &fence_colors.0
    } else {
        &wall_colors.0
    };
    table[child as usize]
}

/// `finish_shrink_or_split_event`（inventory line 353）：shrink/split 分支。
///
/// 逐 child 重映射：排序后逐相邻段算中点边界 `[mid, end]`（`cur.a·0.5+prev.b·0.5` /
/// 末位 `1.0`），断言 `mid < end` + [0,1]，`crashy_slice` 后 `inverse_lerp`×2 得新
/// u 界，装配变体 0（桶 570）；被拆/删除的 child 装配变体 2（桶 558）。
fn finish_shrink_or_split_event(
    child_walls: &ChildWalls,
    wall_colors: &WallColorIds,
    fence_colors: &HalftimberColorIds,
    bucket_570: &mut Vec<WallHistoryEdit>,
    bucket_558: &mut Vec<WallHistoryEdit>,
) {
    let curve = &child_walls.original.source.curve;
    let mut children = child_walls.children.clone();
    children.sort_by(|a, b| a.a.total_cmp(&b.a));

    for (i, child) in children.iter().enumerate() {
        let start = if i == 0 {
            0.0
        } else {
            let mid = child.a * 0.5 + children[i - 1].b * 0.5;
            if !(0.0..=1.0).contains(&mid) || mid.is_nan() {
                panic!("eraser shrink: child midpoint outside [0, 1]");
            }
            mid
        };
        let end = if i + 1 == children.len() {
            1.0
        } else {
            let mid = children[i + 1].a * 0.5 + child.b * 0.5;
            if !(0.0..=1.0).contains(&mid) || mid.is_nan() {
                panic!("eraser shrink: child midpoint outside [0, 1]");
            }
            mid
        };
        if end <= start {
            panic!("eraser shrink: child interval start_u must be smaller than end_u");
        }
        // `Curve::crashy_slice`（native）→ 本域 `slice_source_curve`。
        let Ok(sliced) = slice_source_curve(curve, start, end) else {
            continue;
        };
        let (a, b) = inverse_lerp_pair(curve, &sliced, start, end);
        let mut next = child_walls.original.clone();
        next.source.curve = sliced;
        bucket_570.push(WallHistoryEdit::StateChange {
            wall: RecoveredWallId(child.id),
            previous: child_walls.original.clone(),
            next,
            observed_operation: observed_operation(1, a, b),
        });
        // 被拆掉的 child → 变体 2 Delete（桶 558）。
        bucket_558.push(WallHistoryEdit::Delete {
            wall: RecoveredWallId(child.id),
            source: child_walls.original.clone(),
            color_id: child_color_id(&child_walls.original, child.id, wall_colors, fence_colors),
            restored_height: child_walls.original.source.extra,
        });
    }
}

/// 原生 `utils::inverse_lerp`×2 + clamp：把 child 边界映射到切片后曲线的 u 域。
/// 逐次调用均 NAN/INF panic（fresh）。
fn inverse_lerp_pair(
    original: &crate::resources::roofs::geometry::surface::Curve2,
    sliced: &crate::resources::roofs::geometry::surface::Curve2,
    start: f32,
    end: f32,
) -> (f32, f32) {
    // 原生以切片前后曲线的 `coord_at_u` 对 `inverse_lerp`（`(v-a)/(b-a)`）映射
    // start/end。切片保持端点，因此新 u = 原曲线逆插到切片曲线的两端坐标。
    let remap = |u: f32| -> f32 {
        let (_, coord) = original.coord_at_u(u);
        let (_, c0) = sliced.coord_at_u(0.0);
        let (_, c1) = sliced.coord_at_u(1.0);
        let t = (coord - c0) / (c1 - c0);
        if t.is_nan() {
            panic!("eraser inverse_lerp produced NaN");
        }
        if !t.is_finite() {
            panic!("eraser inverse_lerp produced INF");
        }
        t.clamp(0.0, 1.0)
    };
    let a = remap(start);
    let b = remap(end);
    if b < a {
        panic!("eraser inverse_lerp bounds inverted");
    }
    (a, b)
}

/// `finish_erase_event`（inventory line 540）：全擦除/重建分支。
///
/// fresh：`children.len() < 2 || height <= 0` → panic；child 排序；逐段改写变体 2
/// 桶 5a0（Delete）+ 变体 1 桶 588（Create）；原生同一分支的 remap 循环（LAB_140930c32，
/// 中点 + crashy_slice + inverse_lerp）还产出变体 0 桶 570（StateChange）——本重建
/// 一并装配（判别字段编码登记缺类型）。
fn finish_erase_event(
    child_walls: &ChildWalls,
    wall_colors: &WallColorIds,
    fence_colors: &HalftimberColorIds,
    bucket_570: &mut Vec<WallHistoryEdit>,
    bucket_588: &mut Vec<WallHistoryEdit>,
    bucket_5a0: &mut Vec<WallHistoryEdit>,
) {
    if child_walls.children.len() < 2 || child_walls.original.source.extra <= 0.0 {
        panic!("eraser full-erase requires at least two children and positive height");
    }
    let curve = &child_walls.original.source.curve;
    let mut children = child_walls.children.clone();
    children.sort_by(|a, b| a.a.total_cmp(&b.a));

    for (i, child) in children.iter().enumerate() {
        // 被移除的 child → 变体 2 Delete（桶 5a0）。
        bucket_5a0.push(WallHistoryEdit::Delete {
            wall: RecoveredWallId(child.id),
            source: child_walls.original.clone(),
            color_id: child_color_id(&child_walls.original, child.id, wall_colors, fence_colors),
            restored_height: child_walls.original.source.extra,
        });
        // 重建段 → 变体 1 Create（桶 588；source 非 None，过 History::add 校验）。
        bucket_588.push(WallHistoryEdit::Create {
            wall: RecoveredWallId(child.id),
            source: child_walls.original.clone(),
            color_id: child_color_id(&child_walls.original, child.id, wall_colors, fence_colors),
            restored_height: child_walls.original.source.extra,
            restored_elevation: child_walls.original.source.terrain_floor,
        });
        // 变体 0 StateChange（桶 570）：remap 循环对墙态的中段改写。
        let start = if i == 0 {
            0.0
        } else {
            let mid = child.a * 0.5 + children[i - 1].b * 0.5;
            if !(0.0..=1.0).contains(&mid) || mid.is_nan() {
                panic!("eraser rebuild: child midpoint outside [0, 1]");
            }
            mid
        };
        let end = if i + 1 == children.len() {
            1.0
        } else {
            let mid = children[i + 1].a * 0.5 + child.b * 0.5;
            if !(0.0..=1.0).contains(&mid) || mid.is_nan() {
                panic!("eraser rebuild: child midpoint outside [0, 1]");
            }
            mid
        };
        if end <= start {
            panic!("eraser rebuild: child interval start_u must be smaller than end_u");
        }
        if let Ok(sliced) = slice_source_curve(curve, start, end) {
            let (a, b) = inverse_lerp_pair(curve, &sliced, start, end);
            let mut next = child_walls.original.clone();
            next.source.curve = sliced;
            bucket_570.push(WallHistoryEdit::StateChange {
                wall: RecoveredWallId(child.id),
                previous: child_walls.original.clone(),
                next,
                observed_operation: observed_operation(1, a, b),
            });
        }
    }
}

/// B archivist finish（0x14092f990）的 Bevy 系统面：装配 `WallCompoundHistoryEdit`
/// 经 `RecordWallHistory` 交 history 域（原生 `History::add(tag 0x7)`），并保留本
/// walls 域的既有 roof 输出（原生 finish 本体不含 roof；roof 删除路径单独锚定登记）。
pub fn finish_wall_eraser(
    mut archivist: ResMut<WallEraserArchivist>,
    _inner: Res<InnerWalls>,
    wall_colors: Res<WallColorIds>,
    fence_colors: Res<HalftimberColorIds>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut wall_history: EventWriter<RecordWallHistory>,
    mut roof_history: EventWriter<RecordRoofHistory>,
) {
    // 受影响墙在 finish drain map2 前快照（finish 会清空 map1/map2）。
    let affected: Vec<WallId> = archivist.affected_walls().collect();
    let compound = archivist.finish(&wall_colors, &fence_colors);
    // 既有 walls 域 roof 输出：每受影响墙的屋顶删除记录（finish 本体外）。
    for wall in affected {
        let roof = public
            .roof_entity(wall)
            .and_then(|entity| roofs.get(entity).ok())
            .map(|roof| roof.0);
        if let Some(roof_row) = roof {
            let edit = RoofHistoryEdit::delete_from_roof(&roof_row);
            roof_history.write(RecordRoofHistory(edit));
        }
    }
    wall_history.write(RecordWallHistory(Arc::new(compound)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::inner_wall_state::WallState;

    fn curve() -> Curve2 {
        Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0]], false).unwrap()
    }

    fn snapshot(height: f32) -> WallAuthoritySnapshot {
        WallState::new(curve(), height, 2, 5, false)
            .snapshot()
            .clone()
    }

    fn archivist_with(children: ChildWalls) -> WallEraserArchivist {
        let mut archivist = WallEraserArchivist::new();
        archivist.map1.insert(
            OriginalWallId(1),
            RewireEnclosedWall(children.wall.0),
        );
        archivist
            .map3
            .insert(children.wall, children);
        archivist
    }

    /// `add_split_event_pieces` (0x14092f280): the split child is swap-removed,
    /// each piece is sliced from it and registered in map1 with the same target.
    #[test]
    fn add_split_event_pieces_replaces_child_and_registers_pieces() {
        let mut archivist = archivist_with(ChildWalls {
            wall: RewireEnclosedWall(1),
            original: snapshot(3.0),
            children: vec![ChildWall::new(1, 0.0, 1.0), ChildWall::new(9, 0.0, 0.0)],
            fully_erased: false,
        });
        assert!(archivist.has_split_target(WallId(1)));
        archivist.add_split_event_pieces(WallId(1), &[(2, 0.0, 0.25), (3, 0.25, 1.0)]);
        let ids: Vec<u64> = archivist.map3[&RewireEnclosedWall(1)]
            .children
            .iter()
            .map(|c| c.id)
            .collect();
        assert_eq!(ids, vec![9, 2, 3]);
        assert_eq!(archivist.map1[&OriginalWallId(2)], RewireEnclosedWall(1));
        assert_eq!(archivist.map1[&OriginalWallId(3)], RewireEnclosedWall(1));
        assert!(!archivist.has_split_target(WallId(1)));
    }

    /// cache_if_original_wall：首次 cache → map2/map1 落位；重复（map1 已存在）→
    /// return 不覆盖。
    #[test]
    fn cache_if_original_wall_stores_then_returns_on_repeat() {
        let mut archivist = WallEraserArchivist::new();
        let range = ErasedRange { low: 0.1, high: 0.9 };
        archivist.cache_if_original_wall(WallId(7), &snapshot(3.0), range);
        assert!(archivist.has_anything_been_affected());
        assert!(archivist.affected_walls().eq([WallId(7)]));
        // 重复 cache：map1 已存在 → 直接返回，不覆盖区间。
        archivist.cache_if_original_wall(
            WallId(7),
            &snapshot(3.0),
            ErasedRange { low: 0.2, high: 0.8 },
        );
        let state = archivist.map2.get(&OriginalWallId(7)).unwrap();
        assert_eq!(state.range.low, 0.1);
        assert_eq!(state.range.high, 0.9);
    }

    /// cache_if_original_wall：map3 已存在该墙 → panic（原生未命名 panic）。
    #[test]
    #[should_panic]
    fn cache_if_original_wall_panics_when_present_in_child_walls() {
        let mut archivist = WallEraserArchivist::new();
        archivist.map3.insert(
            RewireEnclosedWall(7),
            ChildWalls {
                wall: RewireEnclosedWall(7),
                original: snapshot(3.0),
                children: Vec::new(),
                fully_erased: false,
            },
        );
        archivist.cache_if_original_wall(WallId(7), &snapshot(3.0), ErasedRange { low: 0.0, high: 1.0 });
    }

    /// record_rewiring_event：map1 覆盖旧值（旧值 drop）。
    #[test]
    fn record_rewiring_event_replaces_target() {
        let mut archivist = WallEraserArchivist::new();
        archivist.record_rewiring_event(OriginalWallId(3), RewireEnclosedWall(9));
        archivist.record_rewiring_event(OriginalWallId(3), RewireEnclosedWall(11));
        assert_eq!(archivist.map1.get(&OriginalWallId(3)), Some(&RewireEnclosedWall(11)));
    }

    /// shrink_wall：按 child id 找到 ChildWall，slice(id, t1, t2) 改写 a/b。
    #[test]
    fn shrink_wall_slices_child_interval() {
        let children = ChildWalls {
            wall: RewireEnclosedWall(5),
            original: snapshot(3.0),
            children: vec![ChildWall::new(1, 0.25, 0.75)],
            fully_erased: false,
        };
        let mut archivist = archivist_with(children);
        archivist.shrink_wall(OriginalWallId(1), 0.0, 1.0);
        let child = &archivist.map3[&RewireEnclosedWall(5)].children[0];
        assert_eq!(child.id, 1);
        assert_eq!(child.a, 0.25); // lerp(0.25, 0.75, 0.0)
        assert_eq!(child.b, 0.75); // lerp(0.25, 0.75, 1.0)
        archivist.shrink_wall(OriginalWallId(1), 0.5, 1.0);
        let child = &archivist.map3[&RewireEnclosedWall(5)].children[0];
        assert_eq!(child.a, 0.5); // lerp(0.25, 0.75, 0.5)
        assert_eq!(child.b, 0.75);
    }

    /// add_split_event：swap_remove 被拆 child，逐段 slice 重插，map1 登记新段。
    #[test]
    fn add_split_event_swap_removes_and_readds_slices() {
        let children = ChildWalls {
            wall: RewireEnclosedWall(5),
            original: snapshot(3.0),
            children: vec![
                ChildWall::new(1, 0.0, 0.5),
                ChildWall::new(2, 0.5, 1.0),
            ],
            fully_erased: false,
        };
        let mut archivist = archivist_with(children);
        archivist.add_split_event(OriginalWallId(1), 0.0, 1.0, 2);
        let children = &archivist.map3[&RewireEnclosedWall(5)].children;
        // swap_remove 把 child(2) 抄入槽 0，再重插 2 段 slice（同 id 1）。
        assert_eq!(children.len(), 3);
        assert_eq!(children.iter().filter(|c| c.id == 1).count(), 2);
        // 原 child(1)（[0.0, 0.5] 区间）已被移除：剩 2 段同 id 1 的 slice。
        assert!(children.iter().any(|c| c.id == 2));
        // map1 登记新段。
        assert_eq!(
            archivist.map1.get(&OriginalWallId(1)),
            Some(&RewireEnclosedWall(1))
        );
    }

    /// add_fully_erased_event：移除最后一个 child → 空桶改写 fully_erased。
    #[test]
    fn add_fully_erased_event_marks_empty_bucket() {
        let children = ChildWalls {
            wall: RewireEnclosedWall(5),
            original: snapshot(3.0),
            children: vec![ChildWall::new(1, 0.0, 1.0)],
            fully_erased: false,
        };
        let mut archivist = archivist_with(children);
        let removed = archivist.add_fully_erased_event(OriginalWallId(1)).unwrap();
        assert_eq!(removed.id, 1);
        let child_walls = &archivist.map3[&RewireEnclosedWall(5)];
        assert!(child_walls.children.is_empty());
        assert!(child_walls.fully_erased);
    }

    /// finish：drain map2 + map3 → 5 组（ConcurrentEdits），空桶保留为空组。
    #[test]
    fn finish_drains_into_five_groups_with_expected_variants() {
        let mut archivist = WallEraserArchivist::new();
        // map2：一个直接擦除的墙。
        archivist.cache_if_original_wall(
            WallId(10),
            &snapshot(3.0),
            ErasedRange { low: 0.2, high: 0.8 },
        );
        // map3：一个 shrink/split 的 child 列表。
        archivist.map1.insert(OriginalWallId(20), RewireEnclosedWall(30));
        archivist.map3.insert(
            RewireEnclosedWall(30),
            ChildWalls {
                wall: RewireEnclosedWall(30),
                original: snapshot(3.0),
                children: vec![ChildWall::new(20, 0.0, 0.5), ChildWall::new(21, 0.5, 1.0)],
                fully_erased: false,
            },
        );
        // map3：一个全擦除的 child 列表。
        archivist.map1.insert(OriginalWallId(22), RewireEnclosedWall(40));
        archivist.map3.insert(
            RewireEnclosedWall(40),
            ChildWalls {
                wall: RewireEnclosedWall(40),
                original: snapshot(3.0),
                children: vec![ChildWall::new(22, 0.0, 0.5), ChildWall::new(23, 0.5, 1.0)],
                fully_erased: true,
            },
        );

        // snapshot() 的 source +0x65 非 0 → 取半木表；颜色按 child 墙 id 下标。
        let wall_colors = WallColorIds::new();
        let mut fence_colors = HalftimberColorIds::new();
        fence_colors.0[22] = 7;
        fence_colors.0[23] = 8;
        let compound = archivist.finish(&wall_colors, &fence_colors);
        assert_eq!(compound.groups.len(), 5);
        let colors = |group: &Vec<WallHistoryEdit>| -> Vec<u32> {
            group
                .iter()
                .map(|edit| match edit {
                    WallHistoryEdit::Create { color_id, .. }
                    | WallHistoryEdit::Delete { color_id, .. } => *color_id,
                    _ => unreachable!(),
                })
                .collect()
        };
        assert_eq!(colors(&compound.groups[3]), vec![7, 8]);
        assert_eq!(colors(&compound.groups[4]), vec![7, 8]);
        // 组 0 = drain map2 → 变体 0（WallChanged / StateChange）。
        assert!(compound.groups[0]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::StateChange { .. })));
        // 组 1 = shrink/split → 变体 0；组 2 = shrink/split → 变体 2（Delete）。
        assert!(compound.groups[1]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::StateChange { .. })));
        assert!(compound.groups[2]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::Delete { .. })));
        // 组 3 = 全擦除 → 变体 1（Create）；组 4 = 全擦除 → 变体 2（Delete）。
        assert!(compound.groups[3]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::Create { .. })));
        assert!(compound.groups[4]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::Delete { .. })));
        // finish 清空 map1 + map2。
        assert!(archivist.map1.is_empty());
        assert!(archivist.map2.is_empty());
    }
}

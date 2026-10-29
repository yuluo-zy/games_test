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
use crate::systems::wall::history_recording::{RecordWallHistory, record_wall_immediately};
use crate::resources::history::History;
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
    /// 原空桶变体的 8 字节参数区间，不包含被移除 child 的 ID。
    pub erased_range: Option<ErasedRange>,
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

    /// 原活动帧遍历 map1 的 key，只对仍存在的活墙发 reanchor。
    pub fn rewired_wall_ids(&self) -> impl Iterator<Item = WallId> + '_ {
        self.map1.keys().map(|id| WallId(id.0))
    }

    /// 原 UI 结束分支按 map3 当前条目数决定是否发擦除提示。
    pub fn has_pending_child_walls(&self) -> bool {
        !self.map3.is_empty()
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
                erased_range: None,
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
            child_walls.erased_range = Some(ErasedRange { low: removed.a, high: removed.b });
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
    ///    - 全擦除：保存的区间生成收尾切片；完整区间按墙高缩至中心短段，
    ///      写入 StateChange 桶 local_570 和 Delete 桶 local_558。
    /// 3. Sip13 inner-walls 探针（命中改写 local_570）——**缺类型登记**（需 inner-walls
    ///    map + SipHash13），本重建不实现。
    /// 4. 5 桶 → `WallCompoundHistoryEdit{groups}`（history 侧 5 组 ConcurrentEdits，
    ///    空桶保留为空组）→ 清 map1 + map2。
    ///
    /// 返回复合编辑，由 `finish_wall_eraser` 同步写入 history 域
    /// （等价原生 `History::add(tag 0x7)`）。
    pub fn finish(
        &mut self,
        wall_colors: &WallColorIds,
        fence_colors: &HalftimberColorIds,
        inner: &InnerWalls,
    ) -> WallCompoundHistoryEdit {
        let edits = self.compile_eraser_buckets(wall_colors, fence_colors, inner);
        self.map1.clear();
        self.map2.clear();
        edits
    }

    /// 原 finish 在拆分/擦除调用点直接写 History，成功返回后清剩余表。
    /// 编译桶期间的 drain 与验证 panic 仍发生在原调用栈内。
    pub fn finish_recorded<S>(
        &mut self,
        wall_colors: &WallColorIds,
        fence_colors: &HalftimberColorIds,
        history: &mut History<S>,
        inner: &InnerWalls,
    ) -> Result<(), crate::resources::history::transactions::HistoryTransactionError> {
        let edits = self.compile_eraser_buckets(wall_colors, fence_colors, inner);
        let edit = crate::resources::history::EditType::Wall(Arc::new(edits));
        history.add(edit.clone()).unwrap_or_else(|_| {
            panic!("Tried writing {edit:?} to History when the input recording is finished")
        });
        self.map1.clear();
        self.map2.clear();
        Ok(())
    }

    /// finish 的 5 桶装配（record_erase_original_wall / finish_shrink_or_split_event /
    /// finish_erase_event 内联，inventory line 259/353/540）。
    fn compile_eraser_buckets(
        &mut self,
        wall_colors: &WallColorIds,
        fence_colors: &HalftimberColorIds,
        inner: &InnerWalls,
    ) -> WallCompoundHistoryEdit {
        let mut bucket_5b8 = Vec::new(); // drain map2 → 变体 0
        let mut bucket_570 = Vec::new(); // map3 shrink/split → 变体 0
        let mut bucket_558 = Vec::new(); // map3 shrink/split → 变体 2 Delete
        let mut bucket_588 = Vec::new(); // 新子墙 Create
        let mut bucket_5a0 = Vec::new(); // 原墙消失时 Delete

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
                    &mut bucket_558,
                );
            } else {
                finish_shrink_or_split_event(
                    &child_walls,
                    wall_colors,
                    fence_colors,
                    inner,
                    &mut bucket_570,
                    &mut bucket_588,
                    &mut bucket_5a0,
                );
            }
        }

        // 3. 活墙查询在子墙循环中借用 InnerWalls 权威 owner。

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
                self.bucket_5a0,
                self.bucket_588,
                self.bucket_570,
                self.bucket_558,
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
    let table = if original.source.clutter_version & 1 != 0 {
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
    inner: &InnerWalls,
    bucket_570: &mut Vec<WallHistoryEdit>,
    bucket_588: &mut Vec<WallHistoryEdit>,
    bucket_5a0: &mut Vec<WallHistoryEdit>,
) {
    let original = &child_walls.original;
    let curve = &original.source.curve;
    assert!(curve.points.len() >= 2 && curve.length > 0.0,
        "擦除历史要求至少两点和正曲线长度");
    let mut children = child_walls.children.clone();
    children.sort_by(|a, b| a.a.total_cmp(&b.a));
    let parent = child_walls.wall.0;
    if !children.iter().any(|child| child.id == parent) {
        bucket_5a0.push(WallHistoryEdit::Delete {
            wall: RecoveredWallId(parent),
            source: original.clone(),
            color_id: child_color_id(original, parent, wall_colors, fence_colors),
            restored_height: original.source.extra,
        });
    }
    for (i, child) in children.iter().enumerate() {
        let start = if i == 0 { 0.0 } else {
            child.a * 0.5 + children[i - 1].b * 0.5
        };
        let end = if i + 1 == children.len() { 1.0 } else {
            children[i + 1].a * 0.5 + child.b * 0.5
        };
        assert!((0.0..=1.0).contains(&start) && (0.0..=1.0).contains(&end) && start < end,
            "擦除历史子墙中点区间无效");
        let sliced = slice_source_curve(curve, start, end)
            .expect("擦除历史原曲线切片失败");
        if child.id != parent {
            bucket_588.push(WallHistoryEdit::Create {
                wall: RecoveredWallId(child.id),
                source: original.clone(),
                color_id: child_color_id(original, parent, wall_colors, fence_colors),
                restored_height: original.source.extra,
                restored_elevation: original.source.terrain_floor,
            });
        }
        let (a, b) = inverse_lerp_pair(start, end, child.a, child.b);
        {
            // 原哈希探针找不到活墙时走 expect，而不是跳过历史项。
            let live = inner.get_unwrapped(WallId(child.id));
            let mut previous = original.clone();
            previous.source.curve = sliced;
            if child.id != parent {
                previous.source.shape = WallShapeSnapshot::from_observed({
                    let mut bytes = [0u8; 28];
                    bytes[..4].copy_from_slice(&2u32.to_le_bytes());
                    bytes
                });
                previous.source.curve_kind_tag = 2;
                previous.source.style = u8::from(original.source.style == 1);
            }
            bucket_570.push(WallHistoryEdit::StateChange {
                wall: RecoveredWallId(child.id),
                previous,
                next: live.snapshot().clone(),
                observed_operation: observed_operation(1, a, b),
            });
        }
    }
}

/// 原生 `utils::inverse_lerp`×2 + clamp：把 child 边界映射到切片后曲线的 u 域。
/// 逐次调用均 NAN/INF panic（fresh）。
fn inverse_lerp_pair(start: f32, end: f32, low: f32, high: f32) -> (f32, f32) {
    // 140931218..1409312a7：两次 inverse_lerp(start,end,child.a/b)。
    let remap = |u: f32| -> f32 {
        let t = (u - start) / (end - start);
        assert!(t.is_finite(), "擦除历史逆插值非有限");
        t.clamp(0.0, 1.0)
    };
    let a = remap(low);
    let b = remap(high);
    assert!(b >= a, "擦除历史逆插值区间倒置");
    (a, b)
}

/// 原 14093010a..14093016b：完整擦除时保留原曲线中间 0.2 米。
/// MAXSS 以 0.2 为源操作数，长度为 NaN 时也选 0.2。
fn fully_erased_center_range(length: f32) -> ErasedRange {
    let length = if length.is_nan() || length <= 0.2 { 0.2 } else { length };
    let ratio = 0.2 / length;
    assert!(ratio > 0.0, "完全擦除长度不能生成正区间");
    let half = ratio * 0.5;
    let low = 0.5 - half;
    let high = half + 0.5;
    ErasedRange {
        low: if low <= 0.0 { 0.0 } else { low },
        high: if 1.0 <= high { 1.0 } else { high },
    }
}

/// 原 finish 的空桶变体（0x1409300d1 起）：保存的区间用于生成收尾切片。
/// 完整 [0,1] 使用曲线长度计算中心短段；输出 StateChange 和 Delete 两桶。
fn finish_erase_event(
    child_walls: &ChildWalls,
    wall_colors: &WallColorIds,
    fence_colors: &HalftimberColorIds,
    bucket_570: &mut Vec<WallHistoryEdit>,
    bucket_558: &mut Vec<WallHistoryEdit>,
) {
    let mut range = child_walls.erased_range.expect("完全擦除缺少原区间");
    if range.low == 0.0 && range.high == 1.0 {
        range = fully_erased_center_range(child_walls.original.source.curve.length);
    }
    let sliced = erased_snapshot(&child_walls.original, range)
        .expect("完全擦除的原曲线切片失败");
    let wall = RecoveredWallId(child_walls.wall.0);
    bucket_570.push(WallHistoryEdit::StateChange {
        wall,
        previous: child_walls.original.clone(),
        next: sliced.clone(),
        observed_operation: observed_operation(1, range.low, range.high),
    });
    bucket_558.push(WallHistoryEdit::Delete {
        wall,
        source: sliced,
        color_id: child_color_id(&child_walls.original, wall.0, wall_colors, fence_colors),
        restored_height: child_walls.original.source.extra,
    });
}

/// B archivist finish（0x14092f990）的宿主适配：装配 `WallCompoundHistoryEdit`
/// 后同步写 history 域（原生 `History::add(tag 0x7)`），并保留本
/// walls 域的既有 roof 输出（原生 finish 本体不含 roof；roof 删除路径单独锚定登记）。
pub fn finish_wall_eraser<S: Send + Sync + 'static>(
    mut archivist: ResMut<WallEraserArchivist>,
    inner: Res<InnerWalls>,
    wall_colors: Res<WallColorIds>,
    fence_colors: Res<HalftimberColorIds>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut wall_history: ResMut<History<S>>,
    mut roof_history: EventWriter<RecordRoofHistory>,
) {
    // 受影响墙在 finish drain map2 前快照（finish 会清空 map1/map2）。
    let affected: Vec<WallId> = archivist.affected_walls().collect();
    let compound = archivist.compile_eraser_buckets(&wall_colors, &fence_colors, &inner);
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
    record_wall_immediately(RecordWallHistory(Arc::new(compound)), &mut wall_history);
    // 原 14092f990 在 History::add 返回后才清剩余两张表。
    archivist.map1.clear();
    archivist.map2.clear();
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

    #[test]
    fn finish_preserves_original_five_nonempty_bucket_order() {
        let edit = |id| WallHistoryEdit::StateChange {
            wall: RecoveredWallId(id), previous: snapshot(3.0), next: snapshot(3.0),
            observed_operation: observed_operation(2, 0.0, 0.0),
        };
        let compound = WallHistoryEdits {
            bucket_5b8: vec![edit(1)], bucket_5a0: vec![edit(2)],
            bucket_588: vec![edit(3)], bucket_570: vec![edit(4)],
            bucket_558: vec![edit(5)],
        }.compile_history_edit();
        let ids: Vec<_> = compound.groups.iter().map(|group| group[0].wall_id().unwrap().0).collect();
        assert_eq!(ids, [1, 2, 3, 4, 5]);
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
            erased_range: None,
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
                erased_range: None,
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
            erased_range: None,
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
            erased_range: None,
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
            erased_range: None,
        };
        let mut archivist = archivist_with(children);
        let removed = archivist.add_fully_erased_event(OriginalWallId(1)).unwrap();
        assert_eq!(removed.id, 1);
        let child_walls = &archivist.map3[&RewireEnclosedWall(5)];
        assert!(child_walls.children.is_empty());
        assert!(child_walls.fully_erased);
        assert_eq!(child_walls.erased_range, Some(ErasedRange { low: removed.a, high: removed.b }));
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
                erased_range: None,
            },
        );
        // map3：一个全擦除的 child 列表。
        archivist.map1.insert(OriginalWallId(22), RewireEnclosedWall(40));
        archivist.map3.insert(
            RewireEnclosedWall(40),
            ChildWalls {
                wall: RewireEnclosedWall(40),
                original: snapshot(3.0),
                children: vec![],
                fully_erased: true,
                erased_range: Some(ErasedRange { low: 0.0, high: 1.0 }),
            },
        );

        // snapshot() 的 source +0x65 非 0 → 取半木表；颜色按 child 墙 id 下标。
        let wall_colors = WallColorIds::new();
        let mut fence_colors = HalftimberColorIds::new();
        fence_colors.0[40] = 7;
        let mut inner = InnerWalls::new();
        for id in [10, 20, 21] {
            inner.new_wall_with_id(WallId(id), WallState::new(curve(), 3.0, 2, 5, false));
        }
        let compound = archivist.finish(&wall_colors, &fence_colors, &inner);
        assert_eq!(compound.groups.len(), 5);
        assert!(compound.groups[4].iter().any(|edit| matches!(edit,
            WallHistoryEdit::Delete { wall, color_id: 7, .. } if wall.0 == 40)));
        // 组 0 = drain map2 → 变体 0（WallChanged / StateChange）。
        assert!(compound.groups[0]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::StateChange { .. })));
        // 原 14092f990 末段五个三槽依次以 5b8、5a0、588、570、558 为长度。
        assert!(compound.groups[3]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::StateChange { .. })));
        assert!(compound.groups[4]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::Delete { .. })));
        // 这组没有原生其他来源的 Create 桶。
        assert!(compound.groups[2]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::Create { .. })));
        assert!(compound.groups[1]
            .iter()
            .all(|e| matches!(e, WallHistoryEdit::Delete { .. })));
        // finish 清空 map1 + map2。
        assert!(archivist.map1.is_empty());
        assert!(archivist.map2.is_empty());
    }

    #[test]
    fn finish_color_table_uses_original_low_bit() {
        let mut original = snapshot(3.0);
        let mut walls = WallColorIds::new();
        let mut fences = HalftimberColorIds::new();
        walls.0[10] = 17;
        fences.0[10] = 29;
        for (flag, expected) in [(0, 17), (1, 29), (2, 17), (3, 29)] {
            original.source.clutter_version = flag;
            assert_eq!(child_color_id(&original, 10, &walls, &fences), expected);
        }
    }

    #[test]
    fn erased_center_range_matches_original_length_and_nan_bits() {
        // 原 RNE 14093010a..16b：输出为两个 f32 的原生位模式。
        for (length, low, high) in [
            (0x00000000, 0x00000000, 0x3f800000),
            (0x3e4ccccd, 0x00000000, 0x3f800000),
            (0x3f800000, 0x3ecccccd, 0x3f19999a),
            (0x40000000, 0x3ee66666, 0x3f0ccccd),
            (0x40400000, 0x3eeeeeef, 0x3f088889),
            (0x7fc12345, 0x00000000, 0x3f800000),
        ] {
            let range = fully_erased_center_range(f32::from_bits(length));
            assert_eq!((range.low.to_bits(), range.high.to_bits()), (low, high));
        }
        // 原曲线长度为 2，墙高为 3；结束分支必须取长度而不是墙高。
        let item = ChildWalls {
            wall: RewireEnclosedWall(10), original: snapshot(3.0), children: vec![],
            fully_erased: true, erased_range: Some(ErasedRange { low: 0.0, high: 1.0 }),
        };
        let mut states = Vec::new();
        let mut deletes = Vec::new();
        finish_erase_event(&item, &WallColorIds::new(), &HalftimberColorIds::new(),
            &mut states, &mut deletes);
        let WallHistoryEdit::StateChange { observed_operation, .. } = &states[0] else { panic!() };
        assert_eq!(&observed_operation[4..8], &0x3ee66666u32.to_le_bytes());
        assert_eq!(&observed_operation[8..12], &0x3f0ccccdu32.to_le_bytes());
        assert_eq!(deletes.len(), 1);
    }

    #[test]
    fn finish_recorded_writes_the_live_transaction_and_clears_the_original_owners() {
        use crate::resources::history::{EditType, History};
        let mut archivist = WallEraserArchivist::new();
        archivist.cache_if_original_wall(WallId(10), &snapshot(3.0), ErasedRange { low: 0.2, high: 0.8 });
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(12).unwrap();
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(10), WallState::new(curve(), 3.0, 2, 5, false));
        archivist.finish_recorded(&WallColorIds::new(), &HalftimberColorIds::new(), &mut history, &inner).unwrap();
        let edits = &history.pending().unwrap().edits;
        assert_eq!(edits.len(), 1);
        let EditType::Wall(compound) = &edits[0] else { panic!("必须写入墙历史") };
        assert_eq!(compound.groups.len(), 5);
        assert_eq!(compound.groups[0].len(), 1);
        assert!(archivist.map1.is_empty() && archivist.map2.is_empty() && archivist.map3.is_empty());
    }

    #[test]
    fn finish_recorded_panics_before_clearing_rewire_map_when_not_recording() {
        let mut archivist = WallEraserArchivist::new();
        archivist.cache_if_original_wall(WallId(10), &snapshot(3.0), ErasedRange { low: 0.2, high: 0.8 });
        let mut history = History::<()>::empty_glade();
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(10), WallState::new(curve(), 3.0, 2, 5, false));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = archivist.finish_recorded(&WallColorIds::new(), &HalftimberColorIds::new(), &mut history, &inner);
        })).is_err());
        // map2/map3 已经按原入口 drain；map1 的最终 clear 在 History::add 之后。
        assert!(!archivist.map1.is_empty());
        assert!(archivist.map2.is_empty() && archivist.map3.is_empty());
    }
}

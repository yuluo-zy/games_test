//! `resources::history` — 权威历史缓冲（S05，B 骨架对齐，WP13）。
//!
//! 范围（合同见 `contracts/history.md`）：
//!   - `History` 资源：edit_chain / snapshots / pending 桶 / cursor / history_start /
//!     nonce / count（布局 O：见各方法反编译）
//!   - `HistoryChainIndex` / `HistoryEditChain`（history_chain 子模块）
//!   - `EditType` 枚举、`MultiEdit`（40B）、`HistoryMutationToken`
//!   - snapshot 记录：`HistoryChainSnapshot` / `ResolvedHistoryChainSnapshot` /
//!     `HistorySnapshotId` / `MaybePendingSnapshotId` / `HistoryChainSnapshots`
//!   - `HistoryIdentity`（fastrand u64）
//!   - 判空谓词：`is_decorator_history_diff_empty` / `is_stairs_history_diff_empty`
//!
//! 撤销/重做游标逻辑内联进 `systems::history::history_process`（0x140af5b30），
//! 见 `src/systems/history/history_process.rs`。
//!
//! 未证字段一律 typed gap；不补造原 Rust 签名。

use bevy_ecs::resource::Resource;
use std::sync::{Arc, Mutex};

use crate::typed_gap::Gap;

pub mod history_chain;
pub use history_chain::{HistoryChainIndex, HistoryEditChain};

/// 历史编辑命令类型（O：PDB constructor 已证 Path/Cobblestone1..3/Garden/Water；
/// 其余为 starting-builds JSON 观察 tag，见 reconstruction/history/findings.md）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditType {
    Wall,
    WallNewSystem,
    Roof,
    DecoratorV5,
    Terrain,
    Tree,
    Color,
    Stairs,
    Path,
    Water,
    Garden,
    Cobblestone1,
    Cobblestone2,
    Cobblestone3,
    /// 未证 tag（开放枚举；观察 schema 不封闭）。
    Unknown,
}

impl EditType {
    /// 由外层命令 tag 投影（O：观察 JSON 13 类外层 tag）。
    pub fn from_observed_tag(tag: &str) -> Self {
        match tag {
            "Wall" => EditType::Wall,
            "WallNewSystem" => EditType::WallNewSystem,
            "Roof" => EditType::Roof,
            "DecoratorV5" => EditType::DecoratorV5,
            "Terrain" => EditType::Terrain,
            "Tree" => EditType::Tree,
            "Color" => EditType::Color,
            "Stairs" => EditType::Stairs,
            "Path" => EditType::Path,
            "Water" => EditType::Water,
            "Garden" => EditType::Garden,
            "Cobblestone1" => EditType::Cobblestone1,
            "Cobblestone2" => EditType::Cobblestone2,
            "Cobblestone3" => EditType::Cobblestone3,
            _ => EditType::Unknown,
        }
    }
}

/// 一次多编辑记录（O：40B；`edits` Vec + 三个辅助容器计数，空判定
/// `edits.len==0 && aux 计数==0`，见 finish_input_recording 0x140a86730）。
/// 辅助容器（decorators/stairs/…）字段 U。
#[derive(Clone, Debug)]
pub struct MultiEdit {
    pub edits: Vec<EditType>,
    /// O：三个非空计数（edits 外辅助容器）。
    pub observed_auxiliary_counts: [usize; 3],
    /// 辅助容器精确类型/负载。布局 U。
    pub aux: Gap,
}

impl Default for MultiEdit {
    fn default() -> Self {
        Self {
            edits: Vec::new(),
            observed_auxiliary_counts: [0; 3],
            aux: Gap,
        }
    }
}

impl MultiEdit {
    /// O：`iter`（编辑命令迭代）。
    pub fn iter(&self) -> std::slice::Iter<'_, EditType> {
        self.edits.iter()
    }
    /// O：`is_empty`（edits + 辅助计数均空）。
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty() && self.observed_auxiliary_counts == [0; 3]
    }
}

/// 历史链快照记录（O：16B = `{payload Arc@+0x00, chain_index i32@+0x08}`，
/// current_snapshot 0x140a84650 二分 0x10 步长）。payload 具体类型 U。
#[derive(Clone, Debug)]
pub struct HistoryChainSnapshot {
    /// O：Arc<Mutex<T>>；T（快照负载）U。
    pub payload: Arc<Mutex<Gap>>,
    pub chain_index: i32,
}

/// 已解析快照（O：`ResolvedHistoryChainSnapshot` TryFrom 0x140a84260 +
/// closest_resolved_snapshot 0x140a84a50；16B）。
#[derive(Clone, Debug)]
pub struct ResolvedHistoryChainSnapshot {
    pub payload: Arc<Mutex<Gap>>,
    pub chain_index: i32,
}

impl TryFrom<&HistoryChainSnapshot> for ResolvedHistoryChainSnapshot {
    type Error = Gap;
    fn try_from(value: &HistoryChainSnapshot) -> Result<Self, Self::Error> {
        Ok(Self {
            payload: value.payload.clone(),
            chain_index: value.chain_index,
        })
    }
}

/// 快照 id（O：`from_snapshot_blob` / `from_file_stem` / `file_name`；
/// HashSet<HistorySnapshotId> 键）。载体 U。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HistorySnapshotId(pub u64);

impl HistorySnapshotId {
    /// O：方法存在；blob 解析规则 U。
    pub fn from_snapshot_blob(_blob: &[u8]) -> Self {
        Self(0)
    }
    /// O：方法存在；文件名规则 U。
    pub fn file_name(&self) -> Gap {
        Gap
    }
    /// O：方法存在；文件 stem 解析规则 U。
    pub fn from_file_stem(_stem: &str) -> Self {
        Self(0)
    }
}

/// 待定/已分配快照 id 包装（O：`MaybePendingSnapshotId` Arc drop 出现；
/// `get` / `assign` / `assign_invalid`）。布局 U。
#[derive(Clone, Debug, Default)]
pub struct MaybePendingSnapshotId(pub Gap);

impl MaybePendingSnapshotId {
    pub fn get(&self) -> Option<HistorySnapshotId> {
        None
    }
    pub fn assign(&mut self, _id: HistorySnapshotId) {}
    pub fn assign_invalid(&mut self) {}
}

/// 快照集合（O：serde 类型，`HistoryChainSnapshots`；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct HistoryChainSnapshots(pub Vec<HistoryChainSnapshot>);

/// 历史身份（O：`generate` 0x140a84220 = fastrand u64；`mutate` 生成新 id）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct HistoryIdentity(pub u64);

impl HistoryIdentity {
    /// O：`generate` = `fastrand::u64(..)`（RangeInclusive<u64> 全范围）。
    pub fn generate() -> Self {
        Self(fastrand::u64(..))
    }
    /// O：`mutate` = 新随机 id。
    pub fn mutate(&mut self) {
        self.0 = fastrand::u64(..);
    }
}

/// 忘记变更守卫（O：Drop 时若 active 未消费则 panic
/// “History was mutated, but wasn't told about it”，观察模型已复现）。
#[must_use = "finish_potential_mutation must consume an active mutation token"]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryMutationToken {
    pub active: bool,
}

impl HistoryMutationToken {
    /// O：`i_promise_there_were_no_mutations`（内联进 finish_input_recording 区域）。
    pub fn i_promise_there_were_no_mutations(mut self) {
        self.active = false;
    }
}

impl Drop for HistoryMutationToken {
    fn drop(&mut self) {
        if self.active && !std::thread::panicking() {
            panic!("History was mutated, but wasn't told about it");
        }
    }
}

/// 历史资源（O：布局见方法反编译；字段顺序/完整 serde U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct History {
    /// O：+0x00 `edit_chain`（Vec<MultiEdit>，元素 0x28）。
    pub edit_chain: HistoryEditChain,
    /// O：+0x18 `snapshots`（Vec<HistoryChainSnapshot>，元素 0x10）。
    pub snapshots: Vec<HistoryChainSnapshot>,
    /// O：+0x30..+0x50 进行中的 `pending` MultiEdit 桶。
    pub pending: Option<MultiEdit>,
    /// O：+0x58 计数（nuke 时 clamp 到 min(自身, edit_chain.len)）。
    pub observed_count_58: usize,
    /// O：+0x60 `mutation_nonce`（fastrand）。
    pub mutation_nonce: u64,
    /// O：+0x68 `chain_index`（cursor，undo/redo 目标）。
    pub chain_index: i32,
    /// O：+0x6c `history_start`（undo 下界锚）。
    pub history_start: i32,
    /// O：+0x70（u32，序列化 `snapshot` 相关）。语义 U。
    pub tail: u32,
}

impl History {
    /// O：`new`/`new_from_params` 存在；起始参数 U。
    pub fn new_from_params(start: i32) -> Self {
        Self {
            edit_chain: HistoryEditChain::default(),
            snapshots: Vec::new(),
            pending: None,
            observed_count_58: 0,
            mutation_nonce: fastrand::u64(..),
            chain_index: -1,
            history_start: start,
            tail: 0,
        }
    }
    /// O：`is_recording`（pending 桶存在）。
    pub fn is_recording(&self) -> bool {
        self.pending.is_some()
    }
    /// O：`start_input_recording` 0x140a86270：AppMode 断言（replay 下 panic），
    /// 丢弃旧 pending 桶，分配新桶。
    pub fn start_input_recording(&mut self) {
        self.pending = Some(MultiEdit::default());
    }
    /// O：`try_start_input_recording` 0x140a86510：未录制才启动，返回是否启动。
    pub fn try_start_input_recording(&mut self) -> bool {
        if self.is_recording() {
            return false;
        }
        self.pending = Some(MultiEdit::default());
        true
    }
    /// O：`add` 0x140a84ec0：若 pending 为空且 cursor-len < -1 先 nuke_future_states；
    /// 向 pending 桶累计 edit（原为 MultiEdit 内层校验；此处保骨架入口）。
    pub fn add(&mut self, edit: EditType) {
        let nuke = match &self.pending {
            Some(p) => {
                p.edits.is_empty() && self.chain_index - (self.edit_chain.inner.len() as i32) < -1
            }
            None => panic!("Tried writing to History when there was no input recording"),
        };
        if nuke {
            self.nuke_future_states();
            self.pending = Some(MultiEdit::default());
        }
        self.pending.as_mut().unwrap().edits.push(edit);
    }
    /// O：`add_decorator_diff` 0x140a854a0（2348B；装饰 diff 专用累计，panic 串
    /// “Tried writing decorator history…” / “Stairs diff…”）。
    pub fn add_decorator_diff(&mut self, _diff: Gap) {
        // 装饰 diff 结构 U；保骨架入口，不臆造字段。
    }
    /// O：`finish_input_recording` 0x140a86730：取 pending；空则 drop 不提交；
    /// 否则 push edit_chain、cursor+1、断言 cursor==len-1、写 nonce、
    /// 追加快照记录。
    pub fn finish_input_recording(&mut self) {
        let Some(pending) = self.pending.take() else {
            panic!("finish_input_recording without active recording");
        };
        if pending.is_empty() {
            return;
        }
        self.edit_chain.push(pending);
        self.mutation_nonce = fastrand::u64(..);
        assert!(self.chain_index >= -1, "cursor below -1");
        self.chain_index += 1;
        assert_eq!(self.chain_index, self.edit_chain.inner.len() as i32 - 1);
        // O：追加 {MIN, 1, chain_index=cursor} 记录；payload 具体 U。
        self.snapshots.push(HistoryChainSnapshot {
            payload: Arc::new(Mutex::new(Gap)),
            chain_index: self.chain_index,
        });
    }
    /// O：`finish_potential_mutation` 0x140a86980：active 则清 snapshots + 写 nonce。
    pub fn finish_potential_mutation(&mut self, token: HistoryMutationToken) {
        if token.active {
            self.snapshots.clear();
            self.mutation_nonce = fastrand::u64(..);
        }
    }
    /// O：`nuke_future_states` 0x140a84c50：cursor != len-1 时截断/重置 + 写 nonce。
    pub fn nuke_future_states(&mut self) {
        if self.chain_index != self.edit_chain.inner.len() as i32 - 1 {
            if self.chain_index < 0 {
                *self = Self::new_from_params(self.history_start);
            } else {
                let retained = (self.chain_index as usize + 1).min(self.edit_chain.inner.len());
                self.edit_chain.truncate(retained);
                self.observed_count_58 = self.observed_count_58.min(retained);
                self.snapshots
                    .retain(|snapshot| snapshot.chain_index <= self.chain_index);
            }
        }
        self.mutation_nonce = fastrand::u64(..);
    }
    /// O：`insert_starting_edit` 0x140a844f0：cursor>-2 断言；cursor+1；首部插入；
    /// 写 nonce、count=0。
    pub fn insert_starting_edit(&mut self, edit: MultiEdit) {
        assert!(
            self.chain_index > -2,
            "insert_starting_edit before chain start"
        );
        self.chain_index += 1;
        self.edit_chain.insert_at_start(edit);
        self.mutation_nonce = fastrand::u64(..);
        self.observed_count_58 = 0;
    }
    /// O：`undo`（内联进 history_process）：cursor>start && cursor<len 时 cursor-1。
    pub fn undo(&mut self) {
        let cursor = self.chain_index;
        if cursor < 0 {
            return;
        }
        if cursor <= self.history_start {
            return;
        }
        if self.edit_chain.inner.len() as i32 <= cursor {
            return;
        }
        self.chain_index = cursor - 1;
    }
    /// O：`redo`（内联进 history_process）：cursor+1 < len 时 cursor+1。
    pub fn redo(&mut self) {
        assert!(self.chain_index >= -1);
        let target = self.chain_index + 1;
        if target < 0 {
            return;
        }
        if self.edit_chain.inner.len() as i32 <= target {
            return;
        }
        self.chain_index = target;
    }
    /// O：`pending_undo_count`（内联进 add；基于 cursor/start/len 计算）。
    pub fn pending_undo_count(&self) -> u32 {
        if self.chain_index < 0 {
            0
        } else {
            (self.chain_index - self.history_start).max(0) as u32
        }
    }
    /// O：`current_snapshot` 0x140a84650：二分找 chain_index<=cursor 最近记录 → Arc clone。
    pub fn current_snapshot(&self) -> Option<Arc<Mutex<Gap>>> {
        let idx = closest_snapshot_index(&self.snapshots, self.chain_index);
        self.snapshots
            .get(idx)
            .map(|snapshot| snapshot.payload.clone())
    }
    /// O：`set_current_snapshot` 0x140a84760：插入 {payload, cursor}（替换同 index）。
    pub fn set_current_snapshot(&mut self, payload: Arc<Mutex<Gap>>) {
        let idx = closest_snapshot_index(&self.snapshots, self.chain_index);
        if self
            .snapshots
            .get(idx)
            .is_some_and(|s| s.chain_index == self.chain_index)
        {
            self.snapshots[idx] = HistoryChainSnapshot {
                payload,
                chain_index: self.chain_index,
            };
        } else {
            self.snapshots.insert(
                idx,
                HistoryChainSnapshot {
                    payload,
                    chain_index: self.chain_index,
                },
            );
        }
    }
    /// O：`closest_resolved_snapshot` 0x140a84a50。
    pub fn closest_resolved_snapshot(&self) -> Option<ResolvedHistoryChainSnapshot> {
        let resolved: Vec<ResolvedHistoryChainSnapshot> = self
            .snapshots
            .iter()
            .filter_map(|s| ResolvedHistoryChainSnapshot::try_from(s).ok())
            .collect();
        let idx = closest_snapshot_index_any(&resolved, self.chain_index);
        resolved.get(idx).cloned()
    }
    /// O：`iter_resolved_snapshots` 0x140a84a40。
    pub fn iter_resolved_snapshots(
        &self,
    ) -> impl Iterator<Item = ResolvedHistoryChainSnapshot> + '_ {
        self.snapshots
            .iter()
            .filter_map(|s| ResolvedHistoryChainSnapshot::try_from(s).ok())
    }
    /// O：`remove_snapshot_record`（PDB 名；移除某快照记录）。
    pub fn remove_snapshot_record(&mut self, _id: HistorySnapshotId) {}
    /// O：`debug_set_history_start` 0x140a84c40 = 写 +0x6c。
    pub fn debug_set_history_start(&mut self, start: i32) {
        self.history_start = start;
    }
    /// O：`debug_display` 0x140a84ba0 = “History: {} / {}. Start at {}”（cursor/start）。
    pub fn debug_display(&self) -> String {
        format!(
            "History: {} / {}. Start at {}",
            self.chain_index,
            self.edit_chain.inner.len() as i32 - 1,
            self.history_start
        )
    }
    /// O：`edit_chain_mut`。
    pub fn edit_chain_mut(&mut self) -> &mut HistoryEditChain {
        &mut self.edit_chain
    }
}

/// 对已排序 `chain_index` 记录二分，返回 `<= cursor` 最近位置（O：current_snapshot
/// 0x140a84650 / closest_resolved_snapshot 0x140a84a50 的二分模式）。
fn closest_snapshot_index(snapshots: &[HistoryChainSnapshot], cursor: i32) -> usize {
    let mut lo = 0usize;
    let mut hi = snapshots.len();
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        if snapshots[mid].chain_index < cursor {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    // 回退到 <= cursor 最近者
    if lo > 0 && snapshots.get(lo).is_none_or(|s| s.chain_index > cursor) {
        lo -= 1;
    }
    lo.min(snapshots.len().saturating_sub(1))
}

/// 泛化版本（O：closest_resolved_snapshot 对 Vec<Resolved…> 二分）。
fn closest_snapshot_index_any(snapshots: &[ResolvedHistoryChainSnapshot], cursor: i32) -> usize {
    let mut lo = 0usize;
    let mut hi = snapshots.len();
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        if snapshots[mid].chain_index < cursor {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo > 0 && snapshots.get(lo).is_none_or(|s| s.chain_index > cursor) {
        lo -= 1;
    }
    lo.min(snapshots.len().saturating_sub(1))
}

/// O：`closest_history_snapshot`（free 函数；供加载端找最近快照）。
pub fn closest_history_snapshot(
    snapshots: &[HistoryChainSnapshot],
    cursor: i32,
) -> Option<&HistoryChainSnapshot> {
    snapshots.get(closest_snapshot_index(snapshots, cursor))
}

/// O：`is_decorator_history_diff_empty` 0x140a84290。
pub fn is_decorator_history_diff_empty(_diff: &Gap) -> bool {
    true
}

/// O：`is_stairs_history_diff_empty` 0x140a842a0（检查容器 +0x30 非空）。
pub fn is_stairs_history_diff_empty(_diff: &Gap) -> bool {
    true
}

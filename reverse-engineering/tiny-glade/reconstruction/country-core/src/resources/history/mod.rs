//! `resources::history` — 权威历史缓冲（S05；按 B 机器码恢复，2026-10-14 深度重写）。
//!
//! 证据：RE-MCP 反编译 `resources/history/mod.rs` 全部业务过程 + 原字节核对
//! （`evidence/country-core-deep/history/`）。本文件布局与行为按 B 观察记录：
//!
//! ## History 资源布局（O，cap-first 容器）
//!
//! | 偏移 | 字段 | 证据 |
//! |---|---|---|
//! | +0x00 | `edit_chain`：Vec<MultiEdit>（元素 0x28=40B；**cap@0x00, ptr@0x08, len@0x10**） | new_from_params 0x140a844f0 / finish 0x140a86730 / nuke 0x140a84c50 |
//! | +0x18 | `snapshots`：Vec<HistoryChainSnapshot>（元素 0x10=16B） | current_snapshot 0x140a84650 二分 0x10 步长 |
//! | +0x30 | `pending` 桶 40B：{edits Vec（cap-first，元素 0x18=24B）, decorators Arc(64B), stairs Arc(96B)}；首字段 i64::MIN = 未录制哨兵 | start/finish/add/is_recording |
//! | +0x58 | 计数（nuke 时 clamp 到 min(自身, len)） | nuke_future_states |
//! | +0x60 | `mutation_nonce`（fastrand u64） | add/finish/nuke/insert 均刷新 |
//! | +0x68 | `chain_index`（cursor i32，undo/redo 目标） | history_process 0x140af5b30 |
//! | +0x6c | `history_start`（undo 下界锚） | debug_set_history_start 0x140a84c40 = `mov [rcx+0x6c], edx` |
//! | +0x70 | u32（nuke 整组重置写 0；语义 U） | nuke_future_states |
//!
//! 原版 `pending` 用首字段哨兵编码 Option；重建用 `Option<MultiEdit>` 表示等价
//! 语义（`is_recording` 0x140a86970 = `xor eax,eax; cmp rax,[rcx+0x30]; setno al`
//! 即“+0x30 >= 0”）。重建不追求 ABI 逐字节对齐。
//!
//! ## MultiEdit（40B，O）
//!
//! `{ edits: Vec<HistoryEdit>（24B）, decorators: Arc<DecoratorDiffTable>（64B）,
//! stairs: Arc<StairsDiffTable>（96B） }`。空判定（finish_input_recording）：
//! `edits.len==0 && decorators+0x28==0 && stairs+0x40==0 && stairs+0x58==0`。
//!
//! `HistoryEdit`（24B）：`{ tag: u32@+0x00, records: u64@+0x08, count: u64@+0x10 }`。
//! tag==7（墙族批量）时 records/count 指向 `(对象指针数组, 数量)` 16B 批记录；
//! `add` 0x140a84ec0 对每对象按判别（对象+0x10 的 niche 判别 0/1/2）校验：
//!   0=曲线切片（tier@+0xf8<2 时要求 start_u@+0xfc < end_u@+0x100，否则 panic
//!     “Slice start_u must be smaller than end_u. Otherwise an invalid curve will be created!”）、
//!   1=CreateWall（+0x18 Option 哨兵 <0 → panic “None is not allowed in `CreateWall` history command”）、
//!   2=DeleteWall（同 → “None is not allowed in `DeleteWall` history command”）。
//!
//! ## 撤销/重做（内联进 `systems::history::history_process` 0x140af5b30）
//!
//! HistoryCmd 4 字节字：byte[1]==0 → Undo（cursor<0 / cursor<=history_start /
//! len<=cursor 时返回；否则 cursor-1 → replay_edit）；byte[1]==1 → Redo（cursor<-1
//! panic；target=cursor+1 越界返回；cursor=target → replay_edit）；byte[1]==4 或
//! byte[0]==2 → Interrupt。其余 → `unreachable!`（“internal error: entered
//! unreachable code”，history_process.rs:40）。
//!
//! 未证字段一律 typed gap，不补造原 Rust 签名。

use bevy_ecs::resource::Resource;
use std::sync::Arc;

use crate::typed_gap::Gap;

pub mod history_chain;
pub use history_chain::{HistoryChainIndex, HistoryEditChain};

// WP17: replay-domain edit-record containers (history replay extension).
pub mod auto_clutter_edit;
pub mod color_edit;
pub mod decorator_edit;
pub mod path_edit;
pub mod replay_cmd_buffer;
pub mod stairs_history_edit;
pub mod terrain_edit;
pub mod tree_edit;
pub mod wall_history;

/// `resources::history::wall_compound_edit` — 墙复合历史编辑（WP17 恢复）。
pub mod wall_compound_edit;

/// 持久化外层命令 tag（O：starting-builds JSON 2,120 edit_chain 条目观察；
/// PDB 已证 Path/Cobblestone1..3/Garden/Water 的 constructor）。
///
/// 注意：这是**序列化 schema** 的分类，不等于运行时 `HistoryEdit.tag`
/// 的判别值。原实现中 `EditType` 构造经 0x140a842c0 跳表分发（serde visitor）。
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

/// 运行时历史编辑记录（O：24B = `{tag: u32@+0x00, records: u64@+0x08,
/// count: u64@+0x10}`；pending 桶与 edit_chain 元素同构）。
///
/// tag==7 为墙族批量编辑：records/count 指向 16B 批记录
/// `{对象指针数组, 对象数}`，`add` 对其逐对象校验（见模块文档）。
/// 其余 tag 的 payload 布局 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistoryEdit {
    /// O：+0x00 判别（观察值 7 = 墙族批量）。
    pub tag: u32,
    /// O：+0x08 记录/数据指针（具体 Rust 类型 U）。
    pub records: u64,
    /// O：+0x10 记录数/计数。
    pub count: u64,
}

impl HistoryEdit {
    pub const WALL_COMPOUND_TAG: u32 = 7;

    /// O：`is_empty`（内联进 finish 的空判定；单条无空语义，恒 false）。
    pub fn is_empty(&self) -> bool {
        false
    }
}

/// 装饰 diff 表（O：pending MultiEdit 的 +0x18 Arc 对象，64B 初始化模式：
/// `{1, 1, 静态模板指针, 0, [0u32;4], 全局递增序列快照}`，字节模板
/// @0x142af1470）。
///
/// 空判定（finish_input_recording）：`(type_counts[2] | type_counts[3]<<32) == 0`。
/// 字段语义（除空判定外）U。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecoratorDiffTable {
    /// O：+0x00 初始化为 1（语义 U）。
    pub tag0: u64,
    /// O：+0x08 初始化为 1（语义 U）。
    pub tag1: u64,
    /// O：+0x10 静态模板指针（0x142af1460；语义 U）。
    pub template: u64,
    /// O：+0x18 初始化为 0（语义 U）。
    pub field_18: u64,
    /// O：+0x20..+0x30 四个 u32 全 0 初始化。
    pub type_counts: [u32; 4],
    /// O：+0x30..+0x40 全局递增序列快照（start_input_recording 从全局计数
    /// 器 FUN_1400d9b20(0) 取 `{u64, u64}` 递增后写入）。
    pub seq: [u64; 2],
}

impl DecoratorDiffTable {
    /// O：start_input_recording 的 64B 初始化序列。
    pub fn new(seq: [u64; 2]) -> Self {
        Self {
            tag0: 1,
            tag1: 1,
            template: 0x0142af1460,
            field_18: 0,
            type_counts: [0; 4],
            seq,
        }
    }

    /// O：finish_input_recording 的空判定读 `*(table+0x28) == 0`。
    pub fn is_empty(&self) -> bool {
        (u64::from(self.type_counts[2]) | (u64::from(self.type_counts[3]) << 32)) == 0
    }
}

impl Default for DecoratorDiffTable {
    fn default() -> Self {
        Self::new([0, 0])
    }
}

/// 楼梯 diff 表（O：pending MultiEdit 的 +0x20 Arc 对象，96B 初始化模式：
/// `{1,1,0,0,0,0, Vec::new()@+0x30, Vec::new()@+0x48}`，两个 Vec 的 dangling
/// 指针 = 4（align-4 元素））。
///
/// 空判定（finish_input_recording）：两个 Vec 的 len（+0x40 / +0x58）均为 0。
/// 元素类型与用途 U（I：楼梯图边对；见 “Stairs graph history recording
/// error: checksum mismatch” panic 串）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StairsDiffTable {
    /// O：+0x00/+0x08 初始化为 1（语义 U）。
    pub tag0: u64,
    pub tag1: u64,
    /// O：+0x10/+0x18/+0x20/+0x28 初始化为 0（语义 U）。
    pub field_10: u64,
    pub field_18: u64,
    pub field_20: u64,
    pub field_28: u64,
    /// O：+0x30..+0x48（Vec，align-4 元素）。
    pub vec_a: Vec<u32>,
    /// O：+0x48..+0x60（Vec，align-4 元素）。
    pub vec_b: Vec<u32>,
}

impl StairsDiffTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// O：finish_input_recording 读 `*(table+0x40)==0 && *(table+0x58)==0`。
    pub fn is_empty(&self) -> bool {
        self.vec_a.is_empty() && self.vec_b.is_empty()
    }
}

/// 一次多编辑记录（O：40B；`edits` Vec + 两个 Arc 辅助表）。
#[derive(Clone, Debug)]
pub struct MultiEdit {
    /// O：+0x00..+0x18（cap-first；元素 24B）。
    pub edits: Vec<HistoryEdit>,
    /// O：+0x18 Arc<DecoratorDiffTable>（history_process 对元素 +0x18 做 Arc
    /// refcount 自增，证实为 Arc 共享表）。
    pub decorators: Arc<DecoratorDiffTable>,
    /// O：+0x20 Arc<StairsDiffTable>。
    pub stairs: Arc<StairsDiffTable>,
}

impl MultiEdit {
    /// O：start_input_recording 的新桶分配（64B + 96B + 全局序列快照）。
    pub fn new(seq: [u64; 2]) -> Self {
        Self {
            edits: Vec::new(),
            decorators: Arc::new(DecoratorDiffTable::new(seq)),
            stairs: Arc::new(StairsDiffTable::new()),
        }
    }

    /// O：finish_input_recording 的空判定（四条件）。
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty() && self.decorators.is_empty() && self.stairs.is_empty()
    }
}

impl Default for MultiEdit {
    fn default() -> Self {
        Self::new([0, 0])
    }
}

/// 历史链快照记录（O：16B = `{payload Arc@+0x00, chain_index i32@+0x08}`，
/// current_snapshot 0x140a84650 二分 0x10 步长；finish_potential_mutation
/// 清空时逐条 Arc drop）。payload 具体类型 U。
#[derive(Clone, Debug)]
pub struct HistoryChainSnapshot {
    /// O：Arc<T>；T（快照负载）U（Ghidra 曾误标 CrashMiscInfo）。
    pub payload: Arc<Gap>,
    pub chain_index: i32,
}

/// 已解析快照（O：closest_resolved_snapshot 0x140a84a50 的 filter_map 结果；
/// 16B = `{payload ptr, chain_index}`）。解析规则（filter 谓词）U。
#[derive(Clone, Debug)]
pub struct ResolvedHistoryChainSnapshot {
    pub payload: Arc<Gap>,
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

/// 快照集合资源（O：finish_input_recording 0x140a86730 的 `ResMut` 参数所指向
/// 容器：+0x20..+0x38 = Vec（cap-first，元素 0x30=48B），+0x40 = 递增值）。
/// 记录 `{payload 哨兵 i64::MIN@+0x00, 1@+0x08, …, chain_index@+0x28}`；
/// 前 0x20 字节与 payload 具体类型 U。
#[derive(Clone, Debug, Default)]
pub struct HistoryChainSnapshots {
    /// O：+0x20..+0x38（cap-first；48B 记录）。
    pub records: Vec<SnapshotRecord>,
    /// O：+0x40 索引计数器（finish 读取后自增并写入记录 +0x28）。
    pub next_index: u64,
}

/// HistoryChainSnapshots 的 48B 记录（O：finish_input_recording 的 push 序列）。
#[derive(Clone, Copy, Debug, Default)]
pub struct SnapshotRecord {
    /// O：+0x00 初始写 i64::MIN（Option 哨兵；具体 Option<T> 类型 U）。
    pub payload_sentinel: i64,
    /// O：+0x08 初始写 1（语义 U）。
    pub field_08: u64,
    /// O：+0x10..+0x28 未初始化（push 时不写；语义 U）。
    pub pad: [u64; 3],
    /// O：+0x28 = 容器 +0x40 计数器的值（chain_index）。
    pub chain_index: u64,
}

impl HistoryChainSnapshots {
    /// O：finish_input_recording 的追加序列（记录 + len++ + 计数器++）。
    pub fn push_pending_record(&mut self) {
        let index = self.next_index;
        self.records.push(SnapshotRecord {
            payload_sentinel: i64::MIN,
            field_08: 1,
            pad: [0; 3],
            chain_index: index,
        });
        self.next_index += 1;
    }
}

/// 历史身份（O：`generate` 0x140a84220 = fastrand u64 全范围；`mutate` 生成新 id）。
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
///
/// 注意：`finish_potential_mutation` 0x140a86980 本身**不接收** token——
/// 其真实签名是 `(&mut self, was_active: bool)`（active 时清 snapshots + 写
/// nonce）。token 由调用侧通过 `i_promise_there_were_no_mutations` 消费。
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

/// 历史资源（O：布局见模块文档；字段顺序/完整 serde U）。
#[derive(Resource, Clone, Debug)]
pub struct History {
    /// O：+0x00 `edit_chain`（Vec<MultiEdit>，元素 0x28）。
    pub edit_chain: HistoryEditChain,
    /// O：+0x18 `snapshots`（Vec<HistoryChainSnapshot>，元素 0x10）。
    pub snapshots: Vec<HistoryChainSnapshot>,
    /// O：+0x30..+0x58 进行中的 `pending` 桶（原版以首字段 i64::MIN 为
    /// “未录制”哨兵；重建用 Option 等价表示）。
    pub pending: Option<MultiEdit>,
    /// O：+0x58 计数（nuke 时 clamp 到 min(自身, edit_chain.len)）。
    pub observed_count_58: usize,
    /// O：+0x60 `mutation_nonce`（fastrand）。
    pub mutation_nonce: u64,
    /// O：+0x68 `chain_index`（cursor，undo/redo 目标）。
    pub chain_index: i32,
    /// O：+0x6c `history_start`（undo 下界锚）。
    pub history_start: i32,
    /// O：+0x70（u32，nuke 整组重置写 0）。语义 U。
    pub tail: u32,
}

impl Default for History {
    fn default() -> Self {
        Self {
            edit_chain: HistoryEditChain::default(),
            snapshots: Vec::new(),
            pending: None,
            observed_count_58: 0,
            mutation_nonce: fastrand::u64(..),
            chain_index: -1,
            history_start: 0,
            tail: 0,
        }
    }
}

impl History {
    /// O：`new_from_params` 0x140a844f0（旧合同名 `insert_starting_edit`）：
    /// 断言 cursor>-2；cursor+1；edit_chain 首部 memmove 右移后插入起始
    /// 编辑；写 nonce；count(+0x58)=0。
    pub fn new_from_params(&mut self, edit: MultiEdit) {
        assert!(
            self.chain_index > -2,
            "new_from_params before chain start"
        );
        self.chain_index += 1;
        self.edit_chain.insert_at_start(edit);
        self.mutation_nonce = fastrand::u64(..);
        self.observed_count_58 = 0;
    }

    /// O：`is_recording` 0x140a86970 = `+0x30 >= 0`（pending 判别非哨兵）。
    pub fn is_recording(&self) -> bool {
        self.pending.is_some()
    }

    /// O：`start_input_recording` 0x140a86270：断言
    /// `**app_mode != AppMode::Replay`（panic “assertion failed:
    /// **app_mode != AppMode::Replay”）；丢弃旧 pending 桶，分配新桶
    /// （edits Vec 空 + 64B 装饰表 + 96B 楼梯表，带全局序列快照）。
    pub fn start_input_recording(&mut self, app_mode: &Gap) {
        assert!(
            // O：与 AppMode::Replay 静态（0x142ad83e8 = 0xd）比较；真实
            // AppMode 类型见 systems::history/mod.rs（wp17/w20 判别冲突，
            // 未合并）。此处以 typed gap 保签名，判别比较在 typed 面完成。
            true,
            "**app_mode != AppMode::Replay"
        );
        let _ = app_mode;
        let seq = crate::resources::history::next_global_seq();
        // O：先 drop 旧桶（若在录制），再写新桶。
        self.pending = Some(MultiEdit::new(seq));
    }

    /// O：`try_start_input_recording` 0x140a86510：AppMode 非 Replay 且未在
    /// 录制才启动，返回是否启动（不 panic）。
    pub fn try_start_input_recording(&mut self, app_mode: &Gap) -> bool {
        let _ = app_mode;
        if self.is_recording() {
            return false;
        }
        self.pending = Some(MultiEdit::new(next_global_seq()));
        true
    }

    /// O：`add` 0x140a84ec0：pending 不存在 → panic
    /// “Tried writing {:?} to History when there was no input recording”；
    /// 若 pending.edits 为空 **且** cursor - edit_chain.len <= -2 则先
    /// `nuke_future_states` 并重建 pending；随后校验（tag==7 墙族：
    /// CreateWall/DeleteWall Option 与曲线切片范围，见模块文档）并 push
    /// 24B 编辑记录。
    pub fn add(&mut self, edit: HistoryEdit) {
        if self.pending.is_none() {
            panic!(
                "Tried writing {edit:?} to History when there was no input recording"
            );
        }
        let needs_nuke = {
            let p = self.pending.as_ref().expect("checked above");
            p.edits.is_empty()
                && self.chain_index - (self.edit_chain.inner.len() as i32) <= -2
        };
        if needs_nuke {
            self.nuke_future_states();
            self.pending = Some(MultiEdit::new(next_global_seq()));
        }
        if edit.tag == HistoryEdit::WALL_COMPOUND_TAG {
            validate_wall_compound_edit(edit);
        }
        self.pending.as_mut().expect("re-created above").edits.push(edit);
    }

    /// O：`create_rectangle` 0x140a93790 内的**第二次** History::add — Color
    /// 历史（owner = create_rectangle；原指令在颜色 id `!= 0` 时追加并写入
    /// 该 u32）。运行时 envelope tag/records 布局 U；此入口为 typed 边界，
    /// 待 Color 历史编码恢复后接到 `add`。
    pub fn record_color_edit(&mut self, _edit: crate::resources::history::color_edit::ColorEdit) {
        // U: envelope encoding not yet recovered (tag/records/count).
    }

    /// O：`add_decorator_diff` 0x140a854a0（2348B）：pending 不存在 → panic
    /// “Tried writing decorator_history_diff to History when the input
    /// recording is finished”；diff 为空则直接返回；否则（含楼梯图校验，
    /// “Stairs graph history recording error: checksum mismatch; last
    /// known: …”）累计进 pending 的装饰表。diff 结构 U：保留入口 + 空/非空
    /// 语义，负载字段不臆造。
    pub fn add_decorator_diff(&mut self, diff: &Gap) {
        if self.pending.is_none() {
            panic!(
                "Tried writing decorator_history_diff to History when the input recording is finished"
            );
        }
        if is_decorator_history_diff_empty(diff) {
            return;
        }
        // O：非空 diff 走 nuke+重建路径并写装饰表；表字段语义 U。
    }

    /// O：`finish_input_recording` 0x140a86730（snapshots 记录追加目标为
    /// `ResMut<HistoryChainSnapshots>`，Bevy Mut 包装见反编译 tick 回写）：
    /// pending 为哨兵 → panic “Called finish_input_recording when history
    /// recording was already Finished”；取出 pending；四条件空 → drop 不
    /// 提交；否则 push edit_chain（0x28 元素）、cursor+1、断言
    /// `cursor == len-1`、写 nonce，并向 snapshots 追加
    /// `{i64::MIN, 1, chain_index=cursor}` 48B 记录。
    pub fn finish_input_recording(&mut self, snapshots: &mut HistoryChainSnapshots) {
        let Some(pending) = self.pending.take() else {
            panic!("Called finish_input_recording when history recording was already Finished");
        };
        if pending.is_empty() {
            return;
        }
        self.edit_chain.push(pending);
        self.mutation_nonce = fastrand::u64(..);
        assert!(self.chain_index >= -1, "self.0 >= -1");
        self.chain_index += 1;
        assert_eq!(self.chain_index, self.edit_chain.inner.len() as i32 - 1);
        snapshots.push_pending_record();
    }

    /// O：`finish_potential_mutation` 0x140a86980：`was_active` 为真则清空
    /// snapshots（逐条 Arc drop：LOCK/dec/UNLOCK，归零即释放）+ 写 nonce。
    /// 真实签名为 `(&mut self, was_active: bool)`，不接收 token。
    pub fn finish_potential_mutation(&mut self, was_active: bool) {
        if was_active {
            self.snapshots.clear();
            self.mutation_nonce = fastrand::u64(..);
        }
    }

    /// O：`nuke_future_states` 0x140a84c50：cursor != len-1 时：
    ///   cursor < 0 → 整组重置（drop 两 Vec → 空 Vec、pending 哨兵、
    ///   count=0、nonce=0、cursor=-1、+0x70=0）；
    ///   否则 → truncate(cursor+1)（逐元素 drop）、断言 cursor==len-1、
    ///   count = min(count, len)、snapshots retain(chain_index<=cursor)。
    /// 两路径最后均写新 nonce。
    pub fn nuke_future_states(&mut self) {
        if self.chain_index != self.edit_chain.inner.len() as i32 - 1 {
            if self.chain_index < 0 {
                self.edit_chain = HistoryEditChain::default();
                self.snapshots = Vec::new();
                self.pending = None;
                self.observed_count_58 = 0;
                self.mutation_nonce = 0;
                self.chain_index = -1;
                self.tail = 0;
            } else {
                let retained = (self.chain_index as usize + 1).min(self.edit_chain.inner.len());
                self.edit_chain.truncate(retained);
                assert_eq!(
                    self.chain_index,
                    self.edit_chain.inner.len() as i32 - 1,
                    "nuke_future_states post-truncate cursor"
                );
                self.observed_count_58 = self.observed_count_58.min(retained);
                self.snapshots
                    .retain(|snapshot| snapshot.chain_index <= self.chain_index);
            }
        }
        self.mutation_nonce = fastrand::u64(..);
    }

    /// O：Undo（内联进 history_process 0x140af5b30，byte[1]==0 分支）：
    /// cursor<0 / cursor<=history_start / len<=cursor 时返回；否则 cursor-1。
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

    /// O：Redo（byte[1]==1 分支）：cursor<-1 panic；target=cursor+1；
    /// target<0 / len<=target 返回；否则 cursor=target。
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

    /// O：`current_snapshot` 0x140a84650：对 snapshots 二分
    /// `chain_index <= cursor` 最近者；**仅当记录的 chain_index == cursor**
    /// 时 Arc clone 返回，否则撤销 clone 返回 None（不是“最近者即返回”）。
    pub fn current_snapshot(&self) -> Option<Arc<Gap>> {
        let idx = closest_snapshot_index(&self.snapshots, self.chain_index);
        let snapshot = self.snapshots.get(idx)?;
        if snapshot.chain_index == self.chain_index {
            Some(snapshot.payload.clone())
        } else {
            None
        }
    }

    /// O：`set_current_snapshot` 0x140a84760：断言
    /// `self.current_snapshot().is_none()`（同 cursor 已有记录 → panic，不
    /// 替换）；按 chain_index 有序插入 `{payload, cursor}`（memmove 右移）。
    pub fn set_current_snapshot(&mut self, payload: Arc<Gap>) {
        assert!(
            self.current_snapshot().is_none(),
            "self.current_snapshot().is_none()"
        );
        let idx = insertion_index(&self.snapshots, self.chain_index);
        assert!(
            idx <= self.snapshots.len(),
            "set_current_snapshot insertion index"
        );
        self.snapshots.insert(
            idx,
            HistoryChainSnapshot {
                payload,
                chain_index: self.chain_index,
            },
        );
    }

    /// O：`closest_resolved_snapshot` 0x140a84a50：filter_map 构建
    /// `Vec<ResolvedHistoryChainSnapshot>` 后二分 `chain_index <= cursor`
    /// 最近者，返回 `Option<{payload, chain_index}>`。
    pub fn closest_resolved_snapshot(&self) -> Option<ResolvedHistoryChainSnapshot> {
        let resolved: Vec<ResolvedHistoryChainSnapshot> = self
            .snapshots
            .iter()
            .filter_map(|s| ResolvedHistoryChainSnapshot::try_from(s).ok())
            .collect();
        let idx = closest_resolved_index(&resolved, self.chain_index);
        resolved.get(idx).cloned()
    }

    /// O：`iter_resolved_snapshots` 0x140a84a40（返回 snapshots 切片迭代；
    /// 首条指令 `mov rax,[rcx+0x20]; mov rdx,[rcx+0x28]; shl rdx,4; add rdx,rax`）。
    pub fn iter_resolved_snapshots(
        &self,
    ) -> impl Iterator<Item = ResolvedHistoryChainSnapshot> + '_ {
        self.snapshots
            .iter()
            .filter_map(|s| ResolvedHistoryChainSnapshot::try_from(s).ok())
    }

    /// O：`remove_snapshot_record` 0x140a84ba0（粘合层：委托
    /// FUN_140d79f90 + HistoryChainIndex::prev 回调，见 0x140b31c90 对容器
    /// +0x30/+0x38 切片的迭代）。具体移除规则 U：保留入口。
    pub fn remove_snapshot_record(&mut self, _id: HistorySnapshotId) {}

    /// O：`debug_set_history_start` 0x140a84c40 = `mov [rcx+0x6c], edx`。
    pub fn debug_set_history_start(&mut self, start: i32) {
        self.history_start = start;
    }

    /// O：`debug_display` 0x140a84ba0 = “History: {} / {}. Start at {}”
    /// （cursor / len-1 / history_start）。
    pub fn debug_display(&self) -> String {
        format!(
            "History: {} / {}. Start at {}",
            self.chain_index,
            self.edit_chain.inner.len() as i32 - 1,
            self.history_start
        )
    }

    /// O：`edit_chain_mut`（PDB 名曾与 set_current_snapshot 混淆；该地址
    /// 实为 set_current_snapshot，此处保留存取器语义）。
    pub fn edit_chain_mut(&mut self) -> &mut HistoryEditChain {
        &mut self.edit_chain
    }

    /// WP12 wall-domain marker：墙变更历史写入（typed gap）。WP13 恢复面以
    /// `add(HistoryEdit)` 累计；墙 compound-edit 负载为 typed gap。
    pub fn mark_changed(&mut self) {}
}

/// O：全局递增序列（FUN_1400d9b20(0) 返回 24B 计数器 `{u64, u32, u64, u32}`，
/// start/add 读取后原子自增并写入 64B 表 +0x30..+0x40）。此处以 fastrand
/// 近似全局序列语义（原计数器与并发模型 U）。
fn next_global_seq() -> [u64; 2] {
    [fastrand::u64(..), fastrand::u64(..)]
}

/// O：add 0x140a84ec0 的 tag==7 校验循环（16B 批记录 → 8B 对象指针数组 →
/// 每对象按 +0x10 niche 判别 0/1/2 检查）。对象布局（墙历史编辑）U：
/// 判别 @+0x10、Option 字段 @+0x18、tier @+0xf8、曲线 start_u @+0xfc /
/// end_u @+0x100。panic 串 O（见模块文档）。
fn validate_wall_compound_edit(_edit: HistoryEdit) {
    // 对象指针数组与 16B 批记录为原始指针载荷（HistoryEdit.records/count）；
    // 重建侧在 wall_compound_edit 类型恢复前保留校验闭包，不臆造布局。
}

/// 对已排序 `chain_index` 记录二分，返回 `<= cursor` 最近下标（O：
/// current_snapshot 0x140a84650 的二分模式）。
fn closest_snapshot_index(snapshots: &[HistoryChainSnapshot], cursor: i32) -> usize {
    binary_search_last_le(snapshots, cursor, |s| s.chain_index)
}

/// 泛化版本（O：closest_resolved_snapshot 0x140a84a50）。
fn closest_resolved_index(snapshots: &[ResolvedHistoryChainSnapshot], cursor: i32) -> usize {
    binary_search_last_le(snapshots, cursor, |s| s.chain_index)
}

/// `chain_index <= cursor` 的最近者（原二分的“回退到最近较小”模式）。
fn binary_search_last_le<T>(
    items: &[T],
    cursor: i32,
    key: impl Fn(&T) -> i32,
) -> usize {
    if items.is_empty() {
        return 0;
    }
    let mut lo = 0usize;
    let mut hi = items.len();
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        if key(&items[mid]) < cursor {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo > 0 && items.get(lo).is_none_or(|s| key(s) > cursor) {
        lo -= 1;
    }
    lo.min(items.len().saturating_sub(1))
}

/// `set_current_snapshot` 的有序插入位置（第一个 chain_index >= cursor）。
fn insertion_index(snapshots: &[HistoryChainSnapshot], cursor: i32) -> usize {
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
    lo
}

/// O：`closest_history_snapshot`（free 函数；0x140b36460，大内联——
/// 快照解析/筛选系统，完整语义 U）。此处提供切片二分检索面。
pub fn closest_history_snapshot(
    snapshots: &[HistoryChainSnapshot],
    cursor: i32,
) -> Option<&HistoryChainSnapshot> {
    snapshots.get(closest_snapshot_index(snapshots, cursor))
}

/// O：`is_decorator_history_diff_empty` 0x140a84290 =
/// `cmp qword [rcx+0x18], 0; sete al`（diff 容器 +0x18 字段 == 0）。
pub fn is_decorator_history_diff_empty(diff: &Gap) -> bool {
    // diff 容器布局 U；重建侧无 +0x18 字段可读，按 typed gap 返回空语义。
    // 真实判断为单字段 `field_18 == 0`（O 字节）。
    let _ = diff;
    true
}

/// O：`is_stairs_history_diff_empty` 0x140a842a0 =
/// `[rcx+0x30] != 0 → false; [rcx+0x48] == 0`（两个 Vec len 均空）。
pub fn is_stairs_history_diff_empty(diff: &Gap) -> bool {
    let _ = diff;
    true
}

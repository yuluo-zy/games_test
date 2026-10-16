//! `deferred_decorator_history.rs` — 延迟装饰历史（生命周期/历史链核心）。
//!
//! 本文件含两类机器码证据的**两个不同结构**（2026-10-16 fresh RE-MCP）：
//!
//! ① `DeferredDecoratorEditHistory`（PDB 名；历史链权威，O）——记录提交队列：
//!    `decorator_record_history` 0x140817850 排空面 = `{cap-first Vec<0x20B 记录>
//!    @+0x00, HashSet<u32> @+0x18, flag u8 @+0x48}`；`apply_decorator_op`
//!    0x141276e30 经 write_*_op_history（PDB 内联行 156/166/182）→ `add`（行 34，
//!    Vec push）+ `add_potentially_surviving_affected_decorator`（行 48）。
//!    0x20B 记录 = `{kind u64@0, u32@8, u32@0xc, u32@0x10, u32@0x14, u64@0x18}`；
//!    kind 3=change、4=place、5=remove（O），6=排空终止哨兵（读取面 O）。
//!    `decorator_record_history`：flag 非零 → 排空记录，逐条 `History::add`
//!    （`FUN_140a84ec0`，**tag 10 = DecoratorV5**，载荷 Arc<32B 记录>）→ 清空
//!    HashSet。**core 56B `DecoratorHistoryEdit` 为存档格式模型，运行时载荷 =
//!    本 32B 记录（偏差登记）**。
//!
//! ② UI 延迟操作队列（D2 batch 证据，O）——独立于历史链的结构：`{Vec<0x28B>
//!    @+0x20 {cap@+0x20, ptr@+0x28, len@+0x30}, 计数器@+0x40}`；0x28B 记录 =
//!    `{u64 seq@0, u8 kind@8, payload@9..0x24}`，kind 0x12=烟囱高度（ui_change_
//!    chimney_height 0x1407ae2e0）、0x21=移动（ui_move_decorator 0x1407a4850）、
//!    取消族 0x14125e560（8B 步长 id 推入）。**overlay 冲突登记**：D2 曾把该
//!    队列命名为 `DeferredDecoratorEditHistory`（与 ① 同名异构）；机器码
//!    （0x140817850 排空面 + 0x141276e30 push 面）证明 ① 是历史链权威布局，
//!    ② 归 UI 批次消费。D2 的 0x28B 记录类型保留为 [`DeferredDecoratorOp`]。
//!
//! core 对接：`History::add(EditType::DecoratorV5(Arc<DecoratorHistoryEdit>))`。
//! core 缺口登记：`History::add_decorator_diff` 0x140a854a0（4 参
//! `(&mut History, &DecoratorDiff 64B, &StairsDiffTable 96B, &RawTable remap)`，
//! 章节 O；本系统按 `EditType::DecoratorV5` + `History::add` 模型接入，
//! 与 deferred_terrain_history 同模式）。

use bevy_ecs::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;

use country_core::resources::history::decorator_edit::DecoratorHistoryEdit;
use country_core::resources::history::{EditType, History};

/// ① 延迟装饰历史记录（32B；布局 O，字段语义 U——见文件头）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeferredDecoratorRecord {
    /// +0x00：操作 kind（O：3=change/move、4=place、5=remove；6=排空终止）。
    pub kind: u64,
    /// +0x08：u32（U：候选 decorator id）。
    pub a: u32,
    /// +0x0c：u32（U）。
    pub b: u32,
    /// +0x10：u32（U）。
    pub c: u32,
    /// +0x14：u32（U）。
    pub d: u32,
    /// +0x18：u64（U）。
    pub e: u64,
}

impl DeferredDecoratorRecord {
    /// kind 常量（apply_decorator_op 0x141276e30 push 值，O）。
    pub const KIND_CHANGE: u64 = 3;
    pub const KIND_PLACE: u64 = 4;
    pub const KIND_REMOVE: u64 = 5;
    /// decorator_record_history 0x140817850 排空循环 break 哨兵（O 读取面）。
    pub const KIND_TERMINATE: u64 = 6;
}

/// ① 延迟装饰编辑历史资源（B `DeferredDecoratorEditHistory`；布局 O）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DeferredDecoratorEditHistory {
    /// +0x00：待提交记录（cap-first Vec，元素 0x20B）。
    pub records: Vec<DeferredDecoratorRecord>,
    /// +0x18：潜在存续受影响装饰 id 集合。
    pub potentially_surviving_affected_decorators: HashSet<u32>,
    /// +0x48：待处理标志（apply_decorator_op 置位；decorator_record_history 清零）。
    pub pending: bool,
}

impl DeferredDecoratorEditHistory {
    /// 行 34（内联，O）：追加一条延迟记录（Vec push）。
    #[inline]
    pub fn add(&mut self, record: DeferredDecoratorRecord) {
        self.records.push(record);
    }

    /// 行 40（内联，O）：潜在存续受影响装饰 id 迭代。
    #[inline]
    pub fn potentially_surviving_affected_decorators(&self) -> impl Iterator<Item = u32> + '_ {
        self.potentially_surviving_affected_decorators.iter().copied()
    }

    /// 行 48（内联，O）：登记潜在存续受影响装饰 id（HashSet insert）。
    #[inline]
    pub fn add_potentially_surviving_affected_decorator(&mut self, decorator_id: u32) {
        self.potentially_surviving_affected_decorators.insert(decorator_id);
    }
}

/// ② UI 延迟操作记录（0x28B；D2 batch 证据，O；独立于历史链）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeferredDecoratorOp {
    /// @+0：序列号（队列计数器旧值；O）。
    pub seq: u64,
    /// @+8：操作类型字节（0x12=烟囱高度、0x21=移动；O）。
    pub kind: u8,
    /// @+9..0x24：载荷（按 kind 分派；I）。
    pub payload: [u8; 27],
}

impl DeferredDecoratorOp {
    /// 烟囱高度操作（O：kind=0x12，ratio/pos 载荷；D2 证据）。
    pub fn chimney_height(seq: u64, ratio: f32, pos: (u32, u32), extra: f32) -> Self {
        let mut op = DeferredDecoratorOp {
            seq,
            kind: 0x12,
            payload: [0; 27],
        };
        op.payload[3..7].copy_from_slice(&ratio.to_bits().to_le_bytes());
        op.payload[15..23].copy_from_slice(&u64::from(pos.0).to_le_bytes());
        op.payload[19..23].copy_from_slice(&u64::from(pos.1).to_le_bytes());
        op.payload[23..27].copy_from_slice(&extra.to_bits().to_le_bytes());
        op
    }

    /// 移动操作（O：kind=0x21，位置/朝向载荷 I；D2 证据）。
    pub fn move_op(seq: u64, payload16: [u8; 16], extra: u32) -> Self {
        let mut op = DeferredDecoratorOp {
            seq,
            kind: 0x21,
            payload: [0; 27],
        };
        op.payload[0..16].copy_from_slice(&payload16);
        op.payload[19..23].copy_from_slice(&extra.to_le_bytes());
        op
    }
}

/// ② UI 延迟操作队列（D2 batch 布局 O；独立于历史链）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DeferredDecoratorOpQueue {
    /// 0x28B 操作队列（原 +0x20 区域）。
    pub ops: Vec<DeferredDecoratorOp>,
    /// +0x40：序列计数器。
    pub seq: u64,
}

impl DeferredDecoratorOpQueue {
    /// 0x28B 步长 push（O：写 seq/kind/payload + len/counter 自增）。
    pub fn push(&mut self, mut op: DeferredDecoratorOp) {
        op.seq = self.seq;
        self.ops.push(op);
        self.seq = self.seq.wrapping_add(1);
    }

    /// 8B 步长 push（O：0x14125e560 族——取消/移除 id 列表；语义 I）。
    pub fn push_u64(&mut self, value: u64) {
        let mut op = DeferredDecoratorOp {
            seq: self.seq,
            kind: 0,
            payload: [0; 27],
        };
        op.payload[0..8].copy_from_slice(&value.to_le_bytes());
        self.ops.push(op);
        self.seq = self.seq.wrapping_add(1);
    }

    /// 排空（宿主消费接线；I）。
    pub fn drain_ops(&mut self) -> Vec<DeferredDecoratorOp> {
        std::mem::take(&mut self.ops)
    }
}

/// 32B 记录 → core 存档模型 `DecoratorHistoryEdit` 的映射。
///
/// 模型适配登记：原生 HistoryEdit 载荷 = 32B 记录（tag 10），core
/// `DecoratorHistoryEdit{id, before, after}`（56B）为存档格式模型。映射仅取
/// 机器码证实的位置：`id` ← 记录 `a`（+0x08）。before/after 状态由 archivist
/// before/after 表消费方（extract_diff）持有；**不得把 @0 kind 当作 id**。
pub(crate) fn record_to_history_edit(record: &DeferredDecoratorRecord) -> DecoratorHistoryEdit {
    DecoratorHistoryEdit {
        id: u64::from(record.a),
        // U：before/after 状态语义待 archivist extract_diff 链取证。
        ..DecoratorHistoryEdit::default()
    }
}

/// `decorator_record_history` @0x140817850（O）。
///
/// 每帧：待处理标志非零 → 排空记录并逐条 `History::add(EditType::DecoratorV5)`；
/// 随后清空潜在存续集合。flag 清零在读取时完成。
pub fn decorator_record_history<S: Send + Sync + 'static>(
    mut history: ResMut<History<S>>,
    mut deferred: ResMut<DeferredDecoratorEditHistory>,
) {
    if !deferred.pending {
        return;
    }
    deferred.pending = false;
    let records = std::mem::take(&mut deferred.records);
    for record in &records {
        if record.kind == DeferredDecoratorRecord::KIND_TERMINATE {
            // O 读取面：kind==6 提前终止排空。
            break;
        }
        let edit = record_to_history_edit(record);
        if let Err(error) = history.add(EditType::DecoratorV5(Arc::new(edit))) {
            // 原生 FUN_140a84ec0 失败路径未显式记录（历史合同适配，同地形族）。
            log::warn!(target: LOG_TARGET, "decorator history add failed: {error:?}");
        }
    }
    deferred.potentially_surviving_affected_decorators.clear();
}

const LOG_TARGET: &str = "system_decorator::deferred_decorator_history";

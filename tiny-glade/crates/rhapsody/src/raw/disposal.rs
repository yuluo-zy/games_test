//! 命令缓冲销毁桶（原 `crates/rhapsody/src/raw/disposal.rs`）。
//!
//! 结构（机器码证实，见 evidence/.../raw-init/rne/disposal-lock-topology.md 及本轮 reports）：
//! - `CommandBufferDisposalBuckets = parking_lot::RwLock<HashMap<CommandBufferSeq, parking_lot::Mutex<DisposalBucket>>>`。
//!   外层不是 `Mutex` 而是 `RwLock`：`remove`(0x1418c4100)/`insert`(0x1418c4260) 对状态字做 CAS 0→8
//!   （`WRITER_BIT=8`），释放为 CAS 8→0；`dispose_cb_resource<T>`（如 0x1418c65a0）用 `lock cmpxchg [state], rax+0x10`
//!   取读锁（`ONE_READER=0x10`），经 hashbrown 探测（桶步长 0x100 = key 8B + `Mutex<DisposalBucket>` 0xf8B）
//!   找到桶后再锁桶内 Mutex（字节 CAS 0→1）并推入动作。
//! - 缺桶：`dispose_cb_resource` 不 panic，而是写一条日志环记录（kind 4）并把资源放进调用方给的回退桶
//!   （`take_out_dropped_waste` 的栈桶），随后由 `immediate_empty_bucket` 立即销毁；命中时写 kind 3 记录。
//! - `insert` 先替换后断言 `prev.is_none()`；`remove` 缺桶 panic。
//! - `SequentialDisposal{ VecDeque<u64>@0, last_retired@0x20 }`：`dispose_in_order`（0x1418c7ae0）见该类型。
//!
//! 偏差（保留）：`DisposalBucket` 原为 0xf0B（多 Vec/容量字段，8B 动作载荷）；这里是 `Vec<DestroyAction>`，
//! 因为 Rust 侧销毁动作是闭包。`Failure` 变体仅由 `try_*` 兼容入口返回；主入口与原版一样 panic。
use crate::log_ring;
use crate::raw::managed::tracking::Failure;
use parking_lot::{Mutex, RwLock};
use std::collections::{HashMap, VecDeque};

pub type DestroyAction = Box<dyn FnOnce() + Send>;
pub type CommandBufferSeq = u64;

/// 原 panic 文本（0x142d069c1 / 0x142d06a40）。
pub const REMOVE_MISSING_MSG: &str = "bucket removed prematurely or command buffer begin() never called?";
pub const INSERT_DUPLICATE_MSG: &str = "assertion failed: prev.is_none()";

/// 日志环记录 kind（`dispose_cb_resource` 的 3/4，`dispose_in_order` 的 0x1e）。
pub const LOG_KIND_BUCKET_HIT: u8 = 3;
pub const LOG_KIND_BUCKET_MISS: u8 = 4;
pub const LOG_KIND_RETIRED: u8 = 0x1e;

#[derive(Default)]
pub struct DisposalBucket {
    actions: Vec<DestroyAction>,
}
impl DisposalBucket {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, action: DestroyAction) {
        self.actions.push(action);
    }
    pub fn len(&self) -> usize {
        self.actions.len()
    }
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
    /// 执行桶内全部延迟销毁动作，返回动作数（原 0x140bf92f0 / `immediate_empty_bucket` 的执行体）。
    pub fn dispose(mut self) -> usize {
        let actions = std::mem::take(&mut self.actions);
        let count = actions.len();
        for action in actions {
            action();
        }
        count
    }
    pub fn into_actions(mut self) -> Vec<DestroyAction> {
        std::mem::take(&mut self.actions)
    }
}
impl Drop for DisposalBucket {
    fn drop(&mut self) {
        // 未取得完成证据时，Rust 容器析构不能提前销毁仍在 GPU 使用的对象。
        for action in self.actions.drain(..) {
            std::mem::forget(action);
        }
    }
}

#[derive(Default)]
pub struct CommandBufferDisposalBuckets {
    map: RwLock<HashMap<CommandBufferSeq, Mutex<DisposalBucket>>>,
}
impl CommandBufferDisposalBuckets {
    /// 原 `insert`（0x1418c4260）：先替换，再断言 `prev.is_none()`。被覆盖的桶不执行 vkDestroy。
    pub fn insert(&self, sequence: CommandBufferSeq, bucket: DisposalBucket) {
        assert!(self.replace(sequence, bucket).is_none(), "{INSERT_DUPLICATE_MSG}");
    }
    fn replace(&self, sequence: CommandBufferSeq, bucket: DisposalBucket) -> Option<DisposalBucket> {
        self.map
            .write()
            .insert(sequence, Mutex::new(bucket))
            .map(Mutex::into_inner)
    }
    /// `insert` 的非 panic 兼容入口：重复 seq 仍按原语义替换，但返回 `DuplicateBucket`。
    pub fn try_insert(&self, sequence: CommandBufferSeq, bucket: DisposalBucket) -> Result<(), Failure> {
        match self.replace(sequence, bucket) {
            None => Ok(()),
            Some(previous) => {
                drop(previous); // Drop 内 forget 动作
                Err(Failure::DuplicateBucket)
            }
        }
    }
    /// 原 `remove`（0x1418c4100）：缺桶 panic。
    pub fn remove(&self, sequence: CommandBufferSeq) -> DisposalBucket {
        self.try_remove(sequence)
            .unwrap_or_else(|_| panic!("{REMOVE_MISSING_MSG}"))
    }
    pub fn try_remove(&self, sequence: CommandBufferSeq) -> Result<DisposalBucket, Failure> {
        self.map
            .write()
            .remove(&sequence)
            .map(Mutex::into_inner)
            .ok_or(Failure::MissingBucket)
    }
    /// 命令开始：登记空桶（原 insert 调用点，重复 seq panic）。
    pub fn begin(&self, sequence: CommandBufferSeq) {
        self.insert(sequence, DisposalBucket::new());
    }
    /// 兼容入口：重复 seq 返回 `Failure::DuplicateBucket`。
    pub fn begin_command_bucket(&self, sequence: CommandBufferSeq) -> Result<(), Failure> {
        self.try_insert(sequence, DisposalBucket::new())
    }
    pub fn contains(&self, sequence: CommandBufferSeq) -> bool {
        self.map.read().contains_key(&sequence)
    }
    /// 原 `dispose_cb_resource<T>`（7 个单态，函数体相同）：读锁查桶；命中推入桶（日志 kind 3），
    /// 缺桶则推入调用方回退桶（日志 kind 4），回退桶随后由 `immediate_empty_bucket` 立即销毁。
    pub fn dispose_cb_resource<T: Send + 'static>(
        &self,
        sequence: CommandBufferSeq,
        resource: T,
        fallback: &mut DisposalBucket,
    ) {
        let action: DestroyAction = Box::new(move || drop(resource));
        let map = self.map.read();
        match map.get(&sequence) {
            Some(bucket) => {
                log_ring::record_event(LOG_KIND_BUCKET_HIT, sequence);
                bucket.lock().push(action);
            }
            None => {
                drop(map);
                log_ring::record_event(LOG_KIND_BUCKET_MISS, sequence);
                fallback.push(action);
            }
        }
    }
    /// 已有桶时追加动作；缺桶把动作还给调用方（`Result` 兼容路径用）。
    pub fn append_existing(&self, sequence: CommandBufferSeq, action: DestroyAction) -> Result<(), DestroyAction> {
        match self.map.read().get(&sequence) {
            Some(bucket) => {
                bucket.lock().push(action);
                Ok(())
            }
            None => Err(action),
        }
    }
    /// 取出并执行 `sequence` 桶（缺桶 panic，原 `remove` 语义）；返回动作数。
    pub fn drain_completed_sequence_or_panic(&self, sequence: CommandBufferSeq) -> usize {
        self.remove(sequence).dispose()
    }
    pub fn drain_completed_sequence(&self, sequence: CommandBufferSeq) -> Result<usize, Failure> {
        Ok(self.try_remove(sequence)?.dispose())
    }
    /// 原 `GraphicsDevice::immediate_empty_bucket`（0xbf92f0）：立即排空给定序号的桶；缺桶 `Failure`（兼容入口）。
    pub fn immediate_empty_bucket(&self, sequence: CommandBufferSeq) -> Result<usize, Failure> {
        self.drain_completed_sequence(sequence)
    }
    /// 立即执行回退桶（`take_out_dropped_waste` 的栈桶，原 0x140bf92f0）。
    pub fn empty_bucket(bucket: DisposalBucket) -> usize {
        bucket.dispose()
    }
    pub fn pending(&self) -> usize {
        self.map.read().values().map(|bucket| bucket.lock().len()).sum()
    }
    pub fn bucket_count(&self) -> usize {
        self.map.read().len()
    }
    /// 当前已注册的序号集合（升序，便于遍历排空）。
    pub fn pending_sequences(&self) -> Vec<CommandBufferSeq> {
        let mut keys: Vec<_> = self.map.read().keys().copied().collect();
        keys.sort_unstable();
        keys
    }
    pub fn drain_after_idle(&self) -> Vec<DestroyAction> {
        let map = std::mem::take(&mut *self.map.write());
        let mut out = Vec::new();
        let mut keys: Vec<_> = map.into_iter().collect();
        keys.sort_unstable_by_key(|(sequence, _)| *sequence);
        for (_, bucket) in keys {
            out.extend(bucket.into_inner().into_actions());
        }
        out
    }
}
impl Drop for CommandBufferDisposalBuckets {
    fn drop(&mut self) {
        for action in self.drain_after_idle() {
            std::mem::forget(action);
        }
    }
}

/// 原 `SequentialDisposal`（0x1418c7ae0 `dispose_in_order`）：
/// `{ pending: VecDeque<u64>@0x00, last_retired: u64@0x20 }`。
/// `pending[i]` 对应序号 `last_retired + 1 + i`，值 0 = 尚未完成，非 0 = 已完成（存序号本身）。
#[derive(Default, Debug)]
pub struct SequentialDisposal {
    pending: VecDeque<u64>,
    last_retired: u64,
}
impl SequentialDisposal {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn last_retired(&self) -> u64 {
        self.last_retired
    }
    pub fn waiting(&self) -> usize {
        self.pending.iter().filter(|&&slot| slot != 0).count()
    }
    /// 报告 `seq` 的命令已完成。`assert!(seq > last_retired)`；按序号连续推进，
    /// 每个新退役的序号（含之前乱序到达、现在连续了的）依次调用 `retire(seq)`
    /// （原版此处：写日志 kind 0x1e → `remove` 桶 → 执行桶 → 释放桶）。乱序重复 panic。
    pub fn dispose_in_order(&mut self, seq: u64, mut retire: impl FnMut(u64)) {
        assert!(seq > self.last_retired, "assertion failed: seq > self.last_retired");
        let idx = (seq - self.last_retired - 1) as usize;
        if idx == 0 {
            self.last_retired += 1;
            retire(seq);
            self.pending.pop_front();
            while let Some(&front) = self.pending.front() {
                if front == 0 {
                    break;
                }
                self.pending.pop_front();
                self.last_retired += 1;
                assert_eq!(self.last_retired, front);
                retire(front);
            }
        } else if idx >= self.pending.len() {
            self.pending.resize(idx, 0);
            self.pending.push_back(seq);
        } else {
            assert_eq!(self.pending[idx], 0, "序号重复完成");
            self.pending[idx] = seq;
        }
    }
    /// 兼容入口：用 `retire` 之外的方式只查询“是否已退役”。
    pub fn is_retired(&self, seq: u64) -> bool {
        seq <= self.last_retired
    }
}

/// 单桶销毁句柄兼容层；`dispose` 立即执行该序号桶。
pub struct DisposalBucketRef<'a>(&'a CommandBufferDisposalBuckets);
impl<'a> DisposalBucketRef<'a> {
    pub fn new(buckets: &'a CommandBufferDisposalBuckets) -> Self {
        Self(buckets)
    }
    pub fn dispose(&self, sequence: u64) -> Result<usize, Failure> {
        self.0.drain_completed_sequence(sequence)
    }
}

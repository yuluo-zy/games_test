//! `deferred_decorator_history.rs` — 延迟装饰历史（生命周期/历史链核心）。
//!
//! 延迟历史与音频事件是不同的 owner。
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
//! 烟囱 1407ae73c..790 的 0x28B 是 Bevy 音频事件记录，已由
//! PlayDecoratorSoundCmd 的 EventWriter 和 decorator_audio 消费。
//!
//! core 对接：`History::add(EditType::DecoratorV5(Arc<DecoratorHistoryEdit>))`。
//! core 缺口登记：`History::add_decorator_diff` 0x140a854a0（4 参
//! `(&mut History, &DecoratorDiff 64B, &StairsDiffTable 96B, &RawTable remap)`，
//! 章节 O；本系统按 `EditType::DecoratorV5` + `History::add` 模型接入，
//! 与 deferred_terrain_history 同模式）。

use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

use country_core::resources::history::decorator_edit::DecoratorV5Edit;
use country_core::resources::history::{EditType, History};
use country_core::resources::walls::decorator::DecoratorLocator;

/// ① 延迟装饰历史记录（**32B，O 布局 = `{kind u64 @0, f1 u64 @8, f2 u64
/// @0x10, f3 u64 @0x18}`**——decorator_record_history 0x140817850 以
/// `plVar10[0..3]` 读取，2026-10-17 reviewer-state 复核证实；旧 `{kind, a u32,
/// b u32, c u32, d u32, e u64}` 布局与二进制读面不符，已修）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeferredDecoratorRecord {
    /// +0x00：操作 kind（O：3=change/move、4=place、5=remove；6=排空终止）。
    pub kind: u64,
    /// +0x08：载荷 1（O：装饰 id 候选，u64 读面；f1=plVar10[1]）。
    pub f1: u64,
    /// +0x10：载荷 2（plVar10[2]；语义 U）。
    pub f2: u64,
    /// +0x18：载荷 3（plVar10[3]；语义 U）。
    pub f3: u64,
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
    /// +0x18：潜在存续受影响装饰定位集合（**HashMap<DecoratorLocator, ()>**，
    /// O：apply_decorator_op 0x141276e30 的 `DecoratorLocator::new` +
    /// `HashMap<DecoratorLocator,()>::insert`；decorator_record_history 排空
    /// 时清空）。
    pub potentially_surviving_affected_decorators: HashMap<DecoratorLocator, ()>,
    /// +0x48：待处理标志（apply_decorator_op 置位；decorator_record_history 清零）。
    pub pending: bool,
}

impl DeferredDecoratorEditHistory {
    /// 行 34（内联，O）：追加一条延迟记录（Vec push）。
    #[inline]
    pub fn add(&mut self, record: DeferredDecoratorRecord) {
        self.records.push(record);
    }

    /// 行 40（内联，O）：潜在存续受影响装饰定位迭代。
    #[inline]
    pub fn potentially_surviving_affected_decorators(&self) -> impl Iterator<Item = &DecoratorLocator> + '_ {
        self.potentially_surviving_affected_decorators.keys()
    }

    /// 行 48（内联，O）：登记潜在存续受影响装饰定位（
    /// `HashMap<DecoratorLocator,()>::insert`）。
    #[inline]
    pub fn add_potentially_surviving_affected_decorator(&mut self, locator: DecoratorLocator) {
        self.potentially_surviving_affected_decorators.insert(locator, ());
    }
}

/// Preserve all four native qwords copied by decorator_record_history140817850.
pub(crate) fn record_to_history_edit(record: &DeferredDecoratorRecord) -> DecoratorV5Edit {
    DecoratorV5Edit { kind: record.kind, fields: [record.f1, record.f2, record.f3] }
}

/// `decorator_record_history` @0x140817850（O）。
///
/// 每帧（**无论待处理标志**，O）：①读 +0x48 待处理标志并清零；②非零则
/// 逐记录（0x20B 步长，kind==6 提前终止）分配 48B 载荷 `{1,1,kind,f1,f2,f3}`
/// 调 `History::add`（kind 10）；③清空记录 Vec（len=0）；④清空
/// HashMap<DecoratorLocator,()>（control 字节 0xff 重置 + growth_left 重算）。
pub fn decorator_record_history<S: Send + Sync + 'static>(
    mut history: ResMut<History<S>>,
    mut deferred: ResMut<DeferredDecoratorEditHistory>,
) {
    // O：+0x48 标志读取即清零（无条件）。
    let pending = deferred.pending;
    deferred.pending = false;
    if pending {
        // Native zeroes Vec length before iterating, retaining its allocation.
        for record in deferred.records.drain(..) {
            if record.kind == DeferredDecoratorRecord::KIND_TERMINATE {
                // O 读取面：kind==6 提前终止排空。
                break;
            }
            let edit = record_to_history_edit(&record);
            if let Err(error) = history.add(EditType::DecoratorV5(Arc::new(edit))) {
                // 140a84ef0..ef7 detects an absent pending recording and
                // 140a8525b..2a0 enters panic_fmt; do not silently lose edits.
                if error == country_core::resources::history::transactions::HistoryTransactionError::RecordingNotStarted {
                    panic!("original History::add requires an active recording");
                }
                log::warn!(target: LOG_TARGET, "decorator history add failed: {error:?}");
            }
        }
    }
    // O：排空记录 Vec + 清空潜在存续集合（无条件，见文件头）。
    deferred.records.clear();
    deferred.potentially_surviving_affected_decorators.clear();
}

const LOG_TARGET: &str = "system_decorator::deferred_decorator_history";

#[cfg(test)]
mod missing_recording_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::history::MaybePendingSnapshotId;

    #[test]
    fn deferred_record_without_recording_panics() {
        let mut world = World::new();
        world.insert_resource(History::<MaybePendingSnapshotId>::empty_glade());
        let mut deferred = DeferredDecoratorEditHistory::default();
        deferred.add(DeferredDecoratorRecord { kind: DeferredDecoratorRecord::KIND_PLACE, ..Default::default() });
        deferred.pending = true;
        let locator = DecoratorLocator::new(
            country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Terrain, 42);
        deferred.add_potentially_surviving_affected_decorator(locator);
        let capacity = deferred.records.capacity();
        let pointer = deferred.records.as_ptr();
        world.insert_resource(deferred);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(decorator_record_history::<MaybePendingSnapshotId>).unwrap();
        }));
        assert!(result.is_err());
        let deferred = world.resource::<DeferredDecoratorEditHistory>();
        assert!(!deferred.pending);
        assert!(deferred.records.is_empty());
        assert_eq!(deferred.records.capacity(), capacity);
        assert_eq!(deferred.records.as_ptr(), pointer);
        assert!(deferred.potentially_surviving_affected_decorators.contains_key(&locator));
    }
}

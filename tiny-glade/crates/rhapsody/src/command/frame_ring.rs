//! 帧环：按 GPU 完成点循环复用有限个“帧槽”。
//!
//! # 设计
//!
//! 一帧的每队列资源（命令池、描述符池、临时缓冲等）都挂在某个帧槽上。槽位数量等于
//! `frames_in_flight`（1..=3），复用的唯一前提是“该槽上一代工作已经在所有相关队列上
//! 完成”——用 [`SyncSet`] 表达，对照 [`CompletedValues`] 判断，与 `Retirement` 同一套
//! 完成真相源，杜绝在飞池被提前 reset。
//!
//! 状态机：
//! ```text
//! Free ──try_acquire──▶ Reserved ──record──▶ InFlight(set)
//!   ▲                                             │ set 完成
//!   └─────────────────────────────────────────────┘
//! ```
//! `Reserved` 表示“本帧已取走但尚未提交”，此时被环序再次命中属于调用方时序错误，
//! [`FrameRing::try_acquire`] 返回 `None` 而不是错误地复用。

use crate::sync::{CompletedValues, SyncPoint, SyncSet};

/// 帧槽状态。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameSlotState {
    /// 从未使用，或上一代工作已完成、可立即复用。
    Free,
    /// 已交给当前帧、正在录制，尚未登记完成点。
    Reserved,
    /// 已提交，等待 `last_use` 在全部相关队列完成。
    InFlight(SyncSet),
}

/// 帧环登记错误：仅由调用方时序/越界误用触发。
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum FrameRingError {
    #[error("帧槽下标越界")]
    OutOfRange,
    #[error("帧槽不在录制态，不能登记完成点")]
    NotReserved,
}

/// 固定槽位数量的帧环。`N` 等于 `frames_in_flight`，编译期校验至少为 1。
pub struct FrameRing<const N: usize> {
    slots: [FrameSlotState; N],
    /// 下一次 `try_acquire` 要命中的槽位下标。
    cursor: usize,
}

impl<const N: usize> FrameRing<N> {
    pub fn new() -> Self {
        // 编译期不变量：帧环至少一个槽位。
        const { assert!(N >= 1) };
        Self {
            slots: [FrameSlotState::Free; N],
            cursor: 0,
        }
    }

    /// 取下一个可用槽位。
    ///
    /// 按环序命中目标槽：若它仍有未完成的在飞工作（或正被本帧占用），返回 `None`，
    /// 调用方应等待相关完成点后重试，而不是复位在飞池。命中后槽位进入 [`FrameSlotState::Reserved`]。
    pub fn try_acquire(&mut self, completed: &CompletedValues) -> Option<usize> {
        let index = self.cursor % N;
        match self.slots[index] {
            FrameSlotState::Free => {}
            FrameSlotState::Reserved => return None,
            FrameSlotState::InFlight(set) if set.is_reached(completed) => {}
            FrameSlotState::InFlight(_) => return None,
        }
        self.slots[index] = FrameSlotState::Reserved;
        self.cursor = self.cursor.wrapping_add(1);
        Some(index)
    }

    /// 记录一次成功提交：把 [`FrameSlotState::Reserved`] 槽推进为等待 `point` 的
    /// [`FrameSlotState::InFlight`]。只有录制态槽位可登记，否则返回错误。
    pub fn record(&mut self, index: usize, point: SyncPoint) -> Result<(), FrameRingError> {
        let slot = self
            .slots
            .get_mut(index)
            .ok_or(FrameRingError::OutOfRange)?;
        if *slot != FrameSlotState::Reserved {
            return Err(FrameRingError::NotReserved);
        }
        *slot = FrameSlotState::InFlight(SyncSet::from_point(point));
        Ok(())
    }

    /// 当前槽位状态（诊断用）。
    pub fn state(&self, index: usize) -> Option<FrameSlotState> {
        self.slots.get(index).copied()
    }

    /// 已取走（录制中或提交后等待完成）的槽位数量。
    pub fn busy_count(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| matches!(slot, FrameSlotState::Reserved | FrameSlotState::InFlight(_)))
            .count()
    }
}

impl<const N: usize> Default for FrameRing<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q() -> crate::sync::QueueId {
        crate::sync::QueueId::new(0).unwrap()
    }

    #[test]
    fn acquire_cycles_through_slots_then_blocks_on_unfinished_work() {
        let mut ring = FrameRing::<3>::new();
        let completed = CompletedValues::default();

        assert_eq!(ring.try_acquire(&completed), Some(0));
        assert_eq!(ring.try_acquire(&completed), Some(1));
        assert_eq!(ring.try_acquire(&completed), Some(2));
        // 三个槽都被本帧占用：第 4 次取用环回到槽 0，仍在 Reserved，必须等待。
        assert_eq!(ring.try_acquire(&completed), None);
    }

    #[test]
    fn recorded_slot_reuses_after_completion() {
        let mut ring = FrameRing::<2>::new();
        let mut completed = CompletedValues::default();

        assert_eq!(ring.try_acquire(&completed), Some(0));
        ring.record(
            0,
            SyncPoint {
                queue: q(),
                value: 1,
            },
        )
        .unwrap();
        assert_eq!(ring.try_acquire(&completed), Some(1));
        ring.record(
            1,
            SyncPoint {
                queue: q(),
                value: 2,
            },
        )
        .unwrap();

        // 槽 0 尚未完成，环回时不可复用。
        assert_eq!(ring.try_acquire(&completed), None);
        completed.raise(q(), 1);
        assert_eq!(ring.try_acquire(&completed), Some(0), "槽 0 完成后可复用");
    }

    #[test]
    fn record_requires_reserved_state_and_valid_index() {
        let mut ring = FrameRing::<2>::new();
        let point = SyncPoint {
            queue: q(),
            value: 1,
        };
        assert_eq!(ring.record(0, point), Err(FrameRingError::NotReserved));
        assert_eq!(ring.record(2, point), Err(FrameRingError::OutOfRange));
        assert!(ring.try_acquire(&CompletedValues::default()).is_some());
        assert!(ring.record(0, point).is_ok());
    }

    #[test]
    fn busy_count_tracks_reserved_and_inflight() {
        let mut ring = FrameRing::<2>::new();
        let completed = CompletedValues::default();
        assert_eq!(ring.busy_count(), 0);
        ring.try_acquire(&completed);
        assert_eq!(ring.busy_count(), 1);
        ring.record(
            0,
            SyncPoint {
                queue: q(),
                value: 1,
            },
        )
        .unwrap();
        assert_eq!(ring.busy_count(), 1);
        assert!(matches!(ring.state(0), Some(FrameSlotState::InFlight(_))));
    }
}

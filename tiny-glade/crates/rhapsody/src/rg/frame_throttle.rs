//! 帧在途节流：在途提交队列 `VecDeque<PendingSubmission>`。
//!
//! * `begin_frame`：`while pending.len() > 1 { let front = pop_front(); wait_submit_done(device, front) }`
//!   —— 新一帧开始前把在途提交排到只剩最近一个。
//! * `end_frame`：`push_back(本帧提交回执)`。
//! * `wait_gpu_queue_trim`：超时参数非零且 `len > 1` 时，对**队首**调
//!   `wait_duration_or_until_submit_done(device, front, timeout)`（不弹出）。
//!
//! 回执类型 `R` 由调用方决定。
use std::collections::VecDeque;

pub struct FrameThrottle<R> {
    pending: VecDeque<R>,
}

impl<R> Default for FrameThrottle<R> {
    fn default() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}

impl<R> FrameThrottle<R> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn in_flight(&self) -> usize {
        self.pending.len()
    }
    /// 在途"组"数：回执按提交顺序排列，`group` 连续相等的回执算同一组。
    /// 同一显示帧内的多张图共用一个节流槽，节流按组数而不是回执数。
    pub fn in_flight_groups<K: PartialEq>(&self, group: impl Fn(&R) -> K) -> usize {
        let mut count = 0;
        let mut last: Option<K> = None;
        for receipt in &self.pending {
            let key = group(receipt);
            if last.as_ref() != Some(&key) {
                count += 1;
                last = Some(key);
            }
        }
        count
    }
    /// 队首（最早提交）回执；不弹出。
    pub fn front(&self) -> Option<&R> {
        self.pending.front()
    }
    /// 弹出队首：只在已确认该提交完成后调用。
    pub fn pop_front(&mut self) -> Option<R> {
        self.pending.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    /// 按提交顺序遍历在途回执（只读）。
    pub fn iter(&self) -> impl Iterator<Item = &R> {
        self.pending.iter()
    }
    /// 新帧开始：按提交顺序等待并弹出，直到只剩最近一个。`wait` 返回错误时停止并保留剩余项。
    pub fn begin_frame<E>(&mut self, mut wait: impl FnMut(&R) -> Result<(), E>) -> Result<(), E> {
        while self.pending.len() > 1 {
            wait(self.pending.front().expect("len > 1"))?;
            self.pending.pop_front();
        }
        Ok(())
    }
    /// Reserve the in-flight entry before any irreversible GPU submission.
    pub fn reserve_frame(&mut self) -> crate::Result<()> {
        self.pending
            .try_reserve(1)
            .map_err(|_| crate::Error::OutOfHostMemory)
    }
    /// 本帧已提交：登记回执。
    pub fn end_frame(&mut self, receipt: R) {
        self.pending.push_back(receipt);
    }
    /// 带超时地等队首；`timeout_ns == 0` 或在途不足 2 个时不等。返回是否发起了等待。
    pub fn wait_gpu_queue_trim(&self, timeout_ns: u64, wait_until: impl FnOnce(&R, u64)) -> bool {
        if timeout_ns == 0 || self.pending.len() <= 1 {
            return false;
        }
        wait_until(self.pending.front().expect("len > 1"), timeout_ns);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begin_frame_drains_to_the_latest_one_in_submission_order() {
        let mut t = FrameThrottle::new();
        for i in 0..4 {
            t.end_frame(i);
        }
        let mut waited = Vec::new();
        t.begin_frame::<()>(|r| {
            waited.push(*r);
            Ok(())
        })
        .unwrap();
        assert_eq!(waited, [0, 1, 2]);
        assert_eq!(t.in_flight(), 1);
    }

    #[test]
    fn groups_count_consecutive_equal_keys() {
        let mut t = FrameThrottle::new();
        assert_eq!(t.in_flight_groups(|r: &u32| *r), 0);
        for r in [1u32, 1, 2, 2, 3] {
            t.end_frame(r);
        }
        assert_eq!(t.in_flight_groups(|r| *r), 3);
        assert_eq!(t.in_flight(), 5);
        // 每个回执自成一组时与旧的 in_flight 一致。
        let mut single = FrameThrottle::new();
        for r in 0u32..4 {
            single.end_frame(r);
        }
        assert_eq!(single.in_flight_groups(|r| *r), single.in_flight());
    }

    #[test]
    fn a_failed_wait_keeps_the_unwaited_receipts() {
        let mut t = FrameThrottle::new();
        for i in 0..3 {
            t.end_frame(i);
        }
        let r = t.begin_frame(|r| if *r == 1 { Err("lost") } else { Ok(()) });
        assert_eq!(r, Err("lost"));
        assert_eq!(t.in_flight(), 2);
    }

    #[test]
    fn trim_waits_on_the_front_only_when_more_than_one_is_in_flight() {
        let mut t = FrameThrottle::new();
        t.end_frame(10);
        assert!(!t.wait_gpu_queue_trim(5, |_, _| unreachable!()));
        t.end_frame(11);
        assert!(!t.wait_gpu_queue_trim(0, |_, _| unreachable!()));
        let mut seen = None;
        assert!(t.wait_gpu_queue_trim(5, |r, d| seen = Some((*r, d))));
        assert_eq!(seen, Some((10, 5)));
        assert_eq!(t.in_flight(), 2);
    }
}

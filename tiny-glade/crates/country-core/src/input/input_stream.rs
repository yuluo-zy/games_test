//! `input_stream.rs` — 输入录制环（B `input::input_stream`）。
//!
//! 证据（汇编逐条核对，`input/ALGORITHMS.md` §2）：
//! - `InputStream` 64 B = `sync_points: VecDeque`（元素 0x60 B）+ `frames: VecDeque`（元素 0xd0 B）；
//!   `sync_point_count` 0x140949170 / `frame_count` 0x140949180 分别是两者的 `len`；
//! - `add_frame` 0x140949190：`record` 且 `frames.len >= 9999` 时——有 `>= 2` 个同步点则 `drop_oldest_sync_point`，
//!   否则 `frames.len >= 49999` 才弹最老帧（文档文字写 50000，汇编是 `len - 0xc34f` 回绕比较，阈值 49999）；
//!   之后无条件在尾部追加；
//! - `on_history_sync` 0x1409494c0：`!record` 清空两个队列；`record` 且（同步点 `>= 9` 或帧数 `>= 10000`）丢最老同步点；
//!   两个分支之后都追加 `{state, frame_index = frames.len}`；
//! - `drop_oldest_sync_point` 0x140949660：弹出最老同步点，目标 = 它的 `frame_index`；**没有同步点时目标取 `frames.len`**
//!   （即弹空所有帧）；弹出 `target` 个最老帧，每个剩余同步点 `frame_index -= target`；
//! - `index` 0x140949970：越界 panic "Out of bounds access"。
//!
//! 帧与同步点状态的内容未恢复，所以 [`InputStream`] 对帧类型泛型；`InputTranslationState` 只保留 11 个 qword 的原始布局。
//! `try_load` 与整份 serde 形态（`sync_points`/`frames` 键）依赖未恢复的子结构，未做。
use std::collections::VecDeque;

/// 录制时帧数达到该值开始裁剪（O：0x140949190）。
pub const FRAME_PRUNE_THRESHOLD: usize = 9999;
/// 没有足够同步点时的帧数硬上限（O：`len - 49999` 回绕比较）。
pub const FRAME_HARD_CAP: usize = 49_999;
/// 同步点数量上限（O：`len >= 9` 丢最老）。
pub const MAX_SYNC_POINTS: usize = 9;
/// `on_history_sync` 的帧数上限（O：`len >= 10000`）。
pub const SYNC_FRAME_LIMIT: usize = 10_000;

/// 输入翻译状态：11 个 qword（O：同步点 `+0x00..+0x58`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputTranslationState(pub [u64; 11]);

/// 同步点（0x60 B）：翻译状态 + 它在帧环里的位置（`+0x58`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputStreamSyncPoint {
    pub state: InputTranslationState,
    pub frame_index: u64,
}

/// 录制环。
#[derive(Clone, Debug)]
pub struct InputStream<F> {
    sync_points: VecDeque<InputStreamSyncPoint>,
    frames: VecDeque<F>,
}

impl<F> Default for InputStream<F> {
    fn default() -> Self {
        Self {
            sync_points: VecDeque::new(),
            frames: VecDeque::new(),
        }
    }
}

impl<F> InputStream<F> {
    pub fn new() -> Self {
        Self::default()
    }

    /// 0x140949170。
    pub fn sync_point_count(&self) -> usize {
        self.sync_points.len()
    }

    /// 0x140949180。
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn sync_points(&self) -> &VecDeque<InputStreamSyncPoint> {
        &self.sync_points
    }

    /// 0x140949480。
    pub fn last_frame_mut(&mut self) -> Option<&mut F> {
        self.frames.back_mut()
    }

    /// `impl Index<usize>`（0x140949970）：越界 panic "Out of bounds access"。
    pub fn frame(&self, index: usize) -> &F {
        self.frames.get(index).expect("Out of bounds access")
    }

    /// `add_frame`（0x140949190）。
    pub fn add_frame(&mut self, frame: F, record: bool) {
        if record && self.frames.len() >= FRAME_PRUNE_THRESHOLD {
            if self.sync_points.len() >= 2 {
                self.drop_oldest_sync_point();
            } else if self.frames.len() >= FRAME_HARD_CAP {
                self.frames.pop_front();
            }
        }
        self.frames.push_back(frame);
    }

    /// `on_history_sync`（0x1409494c0）。
    pub fn on_history_sync(&mut self, state: InputTranslationState, record: bool) {
        if record {
            if self.sync_points.len() >= MAX_SYNC_POINTS || self.frames.len() >= SYNC_FRAME_LIMIT {
                self.drop_oldest_sync_point();
            }
        } else {
            self.frames.clear();
            self.sync_points.clear();
        }
        self.sync_points.push_back(InputStreamSyncPoint {
            state,
            frame_index: self.frames.len() as u64,
        });
    }

    /// `drop_oldest_sync_point`（0x140949660）。
    pub fn drop_oldest_sync_point(&mut self) {
        let target = match self.sync_points.pop_front() {
            Some(point) => point.frame_index,
            None => self.frames.len() as u64,
        };
        for _ in 0..target {
            self.frames
                .pop_front()
                .expect("assertion failed: self.frames.pop_front().is_some()");
        }
        for point in &mut self.sync_points {
            point.frame_index = point
                .frame_index
                .checked_sub(target)
                .expect("called `Option::unwrap()` on a `None` value");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(tag: u64) -> InputTranslationState {
        InputTranslationState([tag; 11])
    }

    fn filled(frames: usize) -> InputStream<usize> {
        let mut s = InputStream::new();
        for i in 0..frames {
            s.add_frame(i, false);
        }
        s
    }

    #[test]
    fn frames_are_not_pruned_when_not_recording() {
        let mut s = filled(60_000);
        s.add_frame(60_000, false);
        assert_eq!(s.frame_count(), 60_001);
    }

    #[test]
    fn recording_below_the_threshold_keeps_every_frame() {
        let mut s = filled(9_998);
        s.add_frame(0, true);
        assert_eq!(s.frame_count(), 9_999);
    }

    #[test]
    fn without_enough_sync_points_frames_are_kept_until_49999() {
        let mut s = filled(49_998);
        s.add_frame(1, true);
        assert_eq!(s.frame_count(), 49_999);
        s.add_frame(2, true);
        assert_eq!(s.frame_count(), 49_999, "one popped, one pushed");
        assert_eq!(*s.frame(0), 1);
    }

    #[test]
    fn two_sync_points_make_add_frame_drop_the_oldest_segment() {
        let mut s = filled(9_999);
        s.sync_points.push_back(InputStreamSyncPoint {
            state: state(1),
            frame_index: 100,
        });
        s.sync_points.push_back(InputStreamSyncPoint {
            state: state(2),
            frame_index: 500,
        });
        s.add_frame(77, true);
        assert_eq!(s.sync_point_count(), 1);
        assert_eq!(s.frame_count(), 9_999 - 100 + 1);
        assert_eq!(s.sync_points()[0].frame_index, 400);
        assert_eq!(*s.frame(0), 100);
    }

    #[test]
    fn history_sync_without_recording_clears_then_pushes_at_zero() {
        let mut s = filled(10);
        s.on_history_sync(state(1), true);
        s.on_history_sync(state(2), false);
        assert_eq!(s.frame_count(), 0);
        assert_eq!(s.sync_point_count(), 1);
        assert_eq!(
            s.sync_points()[0],
            InputStreamSyncPoint {
                state: state(2),
                frame_index: 0
            }
        );
    }

    #[test]
    fn history_sync_records_the_current_frame_count() {
        let mut s = filled(7);
        s.on_history_sync(state(9), true);
        assert_eq!(s.sync_points()[0].frame_index, 7);
        assert_eq!(s.sync_points()[0].state, state(9));
    }

    #[test]
    fn the_tenth_sync_point_evicts_the_oldest() {
        let mut s = filled(3);
        for i in 0..9 {
            s.on_history_sync(state(i), true);
        }
        assert_eq!(s.sync_point_count(), 9);
        s.on_history_sync(state(100), true);
        assert_eq!(s.sync_point_count(), 9);
        assert_eq!(s.sync_points()[0].state, state(1));
    }

    #[test]
    fn dropping_with_no_sync_points_empties_the_frames() {
        let mut s = filled(12_000);
        s.on_history_sync(state(1), true);
        assert_eq!(
            s.frame_count(),
            0,
            "frames >= 10000 and no sync point: target = frames.len"
        );
        assert_eq!(s.sync_points()[0].frame_index, 0);
    }

    #[test]
    fn surviving_sync_points_are_rebased() {
        let mut s = filled(1_000);
        s.on_history_sync(state(1), true);
        for i in 0..50 {
            s.add_frame(i, true);
        }
        s.on_history_sync(state(2), true);
        s.drop_oldest_sync_point();
        assert_eq!(s.frame_count(), 50);
        assert_eq!(s.sync_points()[0].frame_index, 50);
    }

    #[test]
    #[should_panic(expected = "Out of bounds access")]
    fn indexing_past_the_end_panics() {
        let s = filled(2);
        let _ = s.frame(2);
    }
}

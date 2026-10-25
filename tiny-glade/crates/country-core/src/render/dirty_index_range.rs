//! `DirtyIndexRange` — 原子闭区间脏窗口（layer A1）。
//!
//! O：空哨兵 `(start=usize::MAX, end=0)`；`start > end` 即空；`gpu_flush` 把区间当作闭区间使用
//! （`[start, end+1)`）；Debug 为 `impl$2::fmt` 0x141693b30（两个字段的派生 Debug）。
//! I：`add` 为内联代码、无独立符号；这里按“闭区间”语义实现（旧实现把 `range.end` 直接写入闭区间的
//! `end`，会多标一个元素，导致 `gpu_flush` 在窗口触及末尾时 `end+1 > len` 而 panic）。

use core::ops::Range;
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
pub struct DirtyIndexRange {
    /// 闭区间下界（O，偏移 0）。
    pub start: AtomicUsize,
    /// 闭区间上界（O，偏移 8）。
    pub end: AtomicUsize,
}

impl Clone for DirtyIndexRange {
    fn clone(&self) -> Self {
        Self {
            start: AtomicUsize::new(self.start.load(Ordering::Relaxed)),
            end: AtomicUsize::new(self.end.load(Ordering::Relaxed)),
        }
    }
}

impl Default for DirtyIndexRange {
    fn default() -> Self {
        Self::empty()
    }
}

impl DirtyIndexRange {
    /// 空窗口（O：(usize::MAX, 0)）。
    pub fn empty() -> Self {
        Self {
            start: AtomicUsize::new(usize::MAX),
            end: AtomicUsize::new(0),
        }
    }

    /// 并入半开区间 `range`；空区间忽略（I）。
    pub fn add(&self, range: Range<usize>) {
        if range.start >= range.end {
            return;
        }
        self.start.fetch_min(range.start, Ordering::Relaxed);
        self.end.fetch_max(range.end - 1, Ordering::Relaxed);
    }

    /// `start > end`（O：`gpu_flush` 的首个判断）。
    pub fn is_empty(&self) -> bool {
        self.start.load(Ordering::Relaxed) > self.end.load(Ordering::Relaxed)
    }

    /// 半开区间形式；空窗口为 `None`。
    pub fn to_range(&self) -> Option<Range<usize>> {
        if self.is_empty() {
            None
        } else {
            Some(self.start.load(Ordering::Relaxed)..self.end.load(Ordering::Relaxed) + 1)
        }
    }

    /// 重置为空哨兵（O：`gpu_flush` 末尾写 (usize::MAX, 0)）。
    pub fn clear(&self) {
        self.start.store(usize::MAX, Ordering::Relaxed);
        self.end.store(0, Ordering::Relaxed);
    }
}

impl core::fmt::Debug for DirtyIndexRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DirtyIndexRange")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sentinel() {
        let d = DirtyIndexRange::empty();
        assert!(d.is_empty());
        assert_eq!(d.to_range(), None);
    }

    #[test]
    fn add_is_inclusive_end_and_merges() {
        let d = DirtyIndexRange::empty();
        d.add(3..5);
        assert_eq!(d.to_range(), Some(3..5));
        d.add(1..2);
        d.add(7..8);
        assert_eq!(d.to_range(), Some(1..8));
        d.add(4..4); // 空区间忽略
        assert_eq!(d.to_range(), Some(1..8));
        d.clear();
        assert!(d.is_empty());
    }
}

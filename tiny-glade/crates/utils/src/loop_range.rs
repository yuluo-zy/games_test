//! Borrowed iteration over a logical range whose indices wrap around a period.
use std::ops::{Index, Range, Sub};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoopIdx {
    pub idx: i32,
    pub period: i32,
}
impl LoopIdx {
    pub fn raw_idx(self) -> i32 {
        self.idx
    }
    pub fn index(self) -> usize {
        assert!(self.period > 0, "loop period must be positive");
        self.idx.rem_euclid(self.period) as usize
    }
}
impl Sub for LoopIdx {
    type Output = i64;
    fn sub(self, rhs: Self) -> i64 {
        i64::from(self.idx) - i64::from(rhs.idx)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoopRange {
    pub start_idx: i32,
    pub end_idx: i32,
    pub period: i32,
}
impl LoopRange {
    pub fn new_loop(start: i32, end: i32, period: i32) -> Self {
        Self::validate(start, end, period, true);
        Self {
            start_idx: start,
            end_idx: end,
            period,
        }
    }
    pub fn new_line(start: i32, end: i32, period: i32) -> Self {
        Self::validate(start, end, period, false);
        Self {
            start_idx: start,
            end_idx: end,
            period,
        }
    }
    fn validate(start: i32, end: i32, period: i32, closed: bool) {
        assert!(period > 0, "loop period must be positive");
        let period = i64::from(period);
        let start = i64::from(start);
        let end = i64::from(end);
        let extra = i64::from(closed);
        assert!(start > -period && start < period + extra);
        assert!(end >= 0 && end <= period + 1);
        assert!(end >= start && end - start <= period + extra);
    }
    pub fn start(self) -> LoopIdx {
        LoopIdx {
            idx: self.start_idx,
            period: self.period,
        }
    }
    pub fn end(self) -> LoopIdx {
        LoopIdx {
            idx: self.end_idx,
            period: self.period,
        }
    }
    pub fn len(self) -> usize {
        (i64::from(self.end_idx) - i64::from(self.start_idx)).max(0) as usize
    }
    pub fn is_empty(self) -> bool {
        self.start_idx >= self.end_idx
    }
    /// Selects a relative, half-open range. Empty slices, including at the end,
    /// are valid; reversed or out-of-bounds ranges panic like ordinary slicing.
    pub fn subslice(self, range: Range<usize>) -> Self {
        assert!(range.start <= range.end, "slice range is reversed");
        assert!(range.end <= self.len(), "slice range is out of bounds");
        let start = i64::from(self.start_idx);
        Self {
            start_idx: (start + range.start as i64) as i32,
            end_idx: (start + range.end as i64) as i32,
            period: self.period,
        }
    }
    /// Projects logical indices onto borrowed storage without allocation.
    pub fn map_slice<T>(self, slice: &[T]) -> LoopSlice<'_, T> {
        LoopSlice { slice, range: self }
    }
}
impl Iterator for LoopRange {
    type Item = LoopIdx;
    fn next(&mut self) -> Option<LoopIdx> {
        if self.start_idx >= self.end_idx {
            return None;
        }
        let out = self.start();
        self.start_idx += 1;
        Some(out)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = LoopRange::len(*self);
        (n, Some(n))
    }
}
impl DoubleEndedIterator for LoopRange {
    fn next_back(&mut self) -> Option<LoopIdx> {
        if self.start_idx >= self.end_idx {
            return None;
        }
        self.end_idx -= 1;
        Some(self.end())
    }
}
impl ExactSizeIterator for LoopRange {}

/// A borrowed view of a logical loop range. Indexing is relative to the
/// remaining range, and only storage indices wrap.
#[derive(Debug)]
pub struct LoopSlice<'a, T> {
    slice: &'a [T],
    range: LoopRange,
}
impl<T> Clone for LoopSlice<'_, T> {
    fn clone(&self) -> Self {
        Self {
            slice: self.slice,
            range: self.range,
        }
    }
}
impl<T> LoopSlice<'_, T> {
    pub fn len(&self) -> usize {
        self.range.len()
    }
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
}
impl<'a, T> Iterator for LoopSlice<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|index| &self.slice[index.index()])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}
impl<T> DoubleEndedIterator for LoopSlice<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|index| &self.slice[index.index()])
    }
}
impl<T> ExactSizeIterator for LoopSlice<'_, T> {}
impl<T> Index<usize> for LoopSlice<'_, T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < self.len(), "loop slice index out of bounds");
        let logical = LoopIdx {
            idx: (i64::from(self.range.start_idx) + index as i64) as i32,
            period: self.range.period,
        };
        &self.slice[logical.index()]
    }
}

#[cfg(test)]
mod loop_slice_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/foundation-completion-2026-10-13/base-assets/loop_slice_tests.rs"
    ));
}

#[cfg(test)]
#[path = "oracle/loop_range_native_oracle.rs"]
mod native_oracle;

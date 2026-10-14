//! Logical indices may be negative. The third field is the period, established
//! by LoopIdx::index's signed Euclidean remainder, not an opaque context.
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
        self.idx.rem_euclid(self.period) as usize
    }
}
impl Sub for LoopIdx {
    type Output = i64;
    fn sub(self, rhs: Self) -> i64 {
        self.idx.wrapping_sub(rhs.idx) as i64
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
        assert!(start > period.wrapping_neg());
        assert!(start <= period);
        assert!(end >= 0);
        assert!(end <= period.wrapping_add(1));
        assert!((end.wrapping_sub(start) as u32) <= period.wrapping_add(1) as u32);
        Self {
            start_idx: start,
            end_idx: end,
            period,
        }
    }
    pub fn new_line(start: i32, end: i32, period: i32) -> Self {
        assert!(start > period.wrapping_neg());
        assert!(start < period);
        assert!(end >= 0);
        assert!(end <= period.wrapping_add(1));
        assert!((end.wrapping_sub(start) as u32) <= period as u32);
        Self {
            start_idx: start,
            end_idx: end,
            period,
        }
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
        self.end_idx.wrapping_sub(self.start_idx) as i64 as usize
    }
    pub fn is_empty(self) -> bool {
        self.start_idx == self.end_idx
    }
    /// B consumes/mutates its range argument, requires start < len, and does not
    /// reject start > end in this helper. Preserve those assertions separately.
    pub fn subslice(mut self, range: Range<usize>) -> Self {
        assert!(range.start < self.len());
        assert!(range.end <= self.len());
        self.end_idx = self
            .start_idx
            .wrapping_add(range.end as i32)
            .min(self.end_idx);
        self.start_idx = self.start_idx.wrapping_add(range.start as i32);
        self
    }
    /// Borrow the original slice without materializing a wrapped point list.
    /// B map_slice stores the slice pointer/length and this logical range.
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
        self.start_idx = self.start_idx.wrapping_add(1);
        Some(out)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.end_idx.saturating_sub(self.start_idx).max(0) as usize;
        (n, Some(n))
    }
}
impl DoubleEndedIterator for LoopRange {
    fn next_back(&mut self) -> Option<LoopIdx> {
        if self.start_idx >= self.end_idx {
            return None;
        }
        self.end_idx = self.end_idx.wrapping_sub(1);
        Some(self.end())
    }
}
impl ExactSizeIterator for LoopRange {}

/// Original loop/line range projected onto borrowed storage. Iteration advances
/// the logical range; indexing is relative to its current start and wraps by
/// the range period, then applies the ordinary source-slice bounds check.
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
        let logical = LoopIdx {
            idx: self.range.start_idx.wrapping_add(index as i32),
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
mod native_oracle {
    use super::*;
    #[test]
    fn original_b_instructions() {
        let cases: &[(i32, i32, u64)] = &[
            (-53534, 825, 91),
            (17684, 772, 700),
            (-88383, 771, 282),
            (-51384, 433, 143),
            (-15954, 616, 62),
            (55255, 306, 175),
            (2898, 60, 18),
            (42281, 27, 26),
            (76589, 188, 73),
            (-51327, 943, 538),
            (-13489, 104, 31),
            (-33910, 862, 570),
            (13100, 982, 334),
            (47782, 853, 14),
            (81820, 872, 724),
            (-63859, 310, 1),
            (-14404, 242, 116),
            (6415, 766, 287),
            (57988, 199, 79),
            (-76828, 307, 229),
            (-3857, 457, 256),
            (-90618, 891, 264),
            (-38979, 426, 213),
            (88966, 88, 86),
            (-68016, 155, 29),
            (52787, 124, 87),
            (-56451, 560, 109),
            (26123, 869, 53),
            (81077, 591, 110),
            (81202, 245, 107),
            (42950, 890, 230),
            (91856, 810, 326),
            (-16897, 17, 1),
            (-56232, 31, 2),
            (-20515, 173, 72),
            (14447, 678, 209),
            (22147, 994, 279),
            (-83566, 232, 186),
            (-36136, 90, 44),
            (70097, 651, 440),
            (45680, 507, 50),
            (-26568, 60, 12),
            (67977, 540, 477),
            (28703, 490, 283),
            (-79864, 380, 316),
            (-72074, 688, 166),
            (-17895, 594, 519),
            (64419, 67, 32),
            (29536, 522, 304),
            (-19833, 960, 327),
        ];
        for &(idx, period, expected) in cases {
            assert_eq!(LoopIdx { idx, period }.index() as u64, expected);
        }
    }
}

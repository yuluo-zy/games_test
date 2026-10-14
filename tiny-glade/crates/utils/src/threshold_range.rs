//! The three B specializations partition values into contiguous <= and > runs.
//! Callers decide whether to retain singleton runs. NaN begins the high branch
//! but terminates either ordered comparison, matching the original scans.
use std::ops::Range;
pub fn threshold_range(
    values: &[f32],
    threshold: f32,
    mut low: impl FnMut(Range<usize>),
    mut high: impl FnMut(Range<usize>),
) {
    let mut start = 0;
    while start < values.len() {
        let is_low = values[start] <= threshold;
        let mut end = start + 1;
        while end < values.len()
            && if is_low {
                values[end] <= threshold
            } else {
                threshold < values[end]
            }
        {
            end += 1;
        }
        if is_low {
            low(start..end)
        } else {
            high(start..end)
        };
        start = end;
    }
}

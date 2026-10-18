//! Repeated subtraction of intervals, retaining boundary points for geometry.
//! This is a clipping policy: emitted pieces have closed endpoints, and a cut
//! touching an existing endpoint does not remove that endpoint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub start: f32,
    pub end: f32,
    pub upper_exclusive: bool,
}
#[derive(Clone, Debug)]
pub struct SubtractiveBoolean1D {
    ranges: Vec<Interval>,
    scratch: Vec<Interval>,
}
impl SubtractiveBoolean1D {
    /// Creates a closed interval. Reversed or NaN bounds panic.
    pub fn new_range(start: f32, end: f32) -> Self {
        assert!(start <= end, "interval bounds must be ordered");
        Self {
            ranges: vec![Interval {
                start,
                end,
                upper_exclusive: false,
            }],
            scratch: Vec::new(),
        }
    }
    pub fn result(&self) -> &[Interval] {
        &self.ranges
    }
    /// Clips an ordered interval out of the result. Reversed or NaN bounds
    /// panic before changing the current result.
    pub fn subtract(&mut self, cut: Interval) {
        assert!(cut.start <= cut.end, "cut bounds must be ordered");
        // A half-open interval with coincident bounds contains no points.
        if self.ranges.is_empty() || (cut.upper_exclusive && cut.start == cut.end) {
            return;
        }
        let mut result = std::mem::take(&mut self.scratch);
        result.clear();
        // Disjoint ordered ranges allow a single cut to add at most one piece.
        result.reserve(self.ranges.len().saturating_add(1));
        for &range in &self.ranges {
            let emit = |result: &mut Vec<Interval>, start, end| {
                result.push(Interval {
                    start,
                    end,
                    upper_exclusive: false,
                });
            };
            if range.start == range.end {
                // Isolated points have no surviving edge to retain at a cut.
                let covered = range.start >= cut.start
                    && (range.start < cut.end || (!cut.upper_exclusive && range.start == cut.end));
                if !covered {
                    result.push(range);
                }
            } else if cut.end <= range.start || cut.start >= range.end {
                // Touching an edge does not create a new zero-length piece.
                result.push(range);
            } else if cut.start <= range.start && cut.end >= range.end {
                if cut.upper_exclusive && cut.end == range.end {
                    emit(&mut result, range.end, range.end);
                }
            } else {
                // The closure of each surviving edge keeps the new cut point.
                // Emit only positive-width pieces so repeated cuts are stable.
                if range.start < cut.start {
                    emit(&mut result, range.start, cut.start);
                }
                if range.end > cut.end {
                    emit(&mut result, cut.end, range.end);
                }
            }
        }
        std::mem::swap(&mut self.ranges, &mut result);
        result.clear();
        self.scratch = result;
    }
}
#[cfg(test)]
#[path = "oracle/subtractive_1d_boolean_tests.rs"]
mod tests;

#[cfg(test)]
// Fixture tuples retain the captured native input/output record shape.
#[allow(clippy::type_complexity)]
#[path = "oracle/subtractive_1d_boolean_native_oracle.rs"]
mod native_oracle;

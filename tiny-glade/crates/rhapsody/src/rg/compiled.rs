//! Symbolic batch boundaries used by the graph planner.
use std::ops::Range;

pub const NO_ACCESS: u32 = u32::MAX;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batch {
    pub pass_range: Range<usize>,
    pub export_barriers: bool,
}

#[derive(Debug, Default)]
pub struct BatchCompiler {
    pub batches: Vec<Batch>,
    pub current_range: Range<usize>,
    pub current_export_barriers: bool,
}

impl BatchCompiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush_batch(&mut self, boundary: usize) {
        if boundary <= self.current_range.start {
            return;
        }
        self.current_range.end = boundary;
        let pass_range = std::mem::take(&mut self.current_range);
        let export_barriers = std::mem::take(&mut self.current_export_barriers);
        self.batches.push(Batch {
            pass_range,
            export_barriers,
        });
        self.current_range.start = boundary;
    }

    pub fn finish(mut self, pass_count: usize) -> Vec<Batch> {
        self.flush_batch(pass_count);
        if let Some(last) = self.batches.last_mut() {
            last.export_barriers = true;
        }
        self.batches
    }
}

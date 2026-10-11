pub const GRID_SIDE: usize = 512;
pub const GRID_SAMPLES: usize = GRID_SIDE * GRID_SIDE;
pub const DIRTY_WORDS: usize = 64;

#[derive(Clone, Debug)]
pub struct HeightReadback {
    pub encoded: Vec<u16>,
    pub minimum_y: f32,
    pub maximum_y: f32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadbackChange {
    pub mask: [u64; DIRTY_WORDS],
    pub changed: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadbackError {
    InvalidSampleCount { expected: usize, actual: usize },
}
impl HeightReadback {
    pub fn new(encoded: Vec<u16>, minimum_y: f32, maximum_y: f32) -> Result<Self, ReadbackError> {
        if encoded.len() != GRID_SAMPLES {
            return Err(ReadbackError::InvalidSampleCount { expected: GRID_SAMPLES, actual: encoded.len() });
        }
        Ok(Self { encoded, minimum_y, maximum_y })
    }
    /// B copy_from_gpu's business diff/copy, excluding profiling/publisher internals.
    /// Each mask bit covers eight adjacent u16s on each of eight rows.
    pub fn copy_from_gpu(&mut self, source: &[u16]) -> Result<ReadbackChange, ReadbackError> {
        if source.len() != GRID_SAMPLES || self.encoded.len() != GRID_SAMPLES {
            let actual = if source.len() != GRID_SAMPLES { source.len() } else { self.encoded.len() };
            return Err(ReadbackError::InvalidSampleCount { expected: GRID_SAMPLES, actual });
        }
        let bounds_changed = self.minimum_y != -2.5 || self.maximum_y != 24.0;
        let mut change = ReadbackChange { mask: if bounds_changed { [u64::MAX; DIRTY_WORDS] } else { [0; DIRTY_WORDS] }, changed: bounds_changed };
        if bounds_changed {
            self.minimum_y = -2.5;
            self.maximum_y = 24.0;
        }
        for (index, (previous, new)) in self.encoded.iter_mut().zip(source).enumerate() {
            if *previous != *new {
                *previous = *new;
                let row = index / GRID_SIDE;
                let column = index % GRID_SIDE;
                change.mask[row / 8] |= 1_u64 << (column / 8);
                change.changed = true;
            }
        }
        Ok(change)
    }
}
impl ReadbackChange {
    pub fn cells(&self) -> impl Iterator<Item = usize> + '_ {
        self.mask.iter().enumerate().flat_map(|(row, word)| {
            let mut remaining = *word;
            std::iter::from_fn(move || {
                if remaining == 0 { return None; }
                let bit = remaining.trailing_zeros() as usize;
                remaining &= remaining - 1;
                Some(row * 64 + bit)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn changes_in_eight_by_eight_block_share_one_bit() {
        let mut grid = HeightReadback::new(vec![0; GRID_SAMPLES], -2.5, 24.0).unwrap();
        let mut source = grid.encoded.clone();
        source[7 * GRID_SIDE + 7] = 1;
        source[8 * GRID_SIDE + 8] = 2;
        let change = grid.copy_from_gpu(&source).unwrap();
        assert_eq!(change.cells().collect::<Vec<_>>(), [0, 65]);
        assert!(!grid.copy_from_gpu(&source).unwrap().changed);
    }
    #[test]
    fn changed_bounds_invalidate_all_cells() {
        let mut grid = HeightReadback::new(vec![0; GRID_SAMPLES], -2.5, 20.0).unwrap();
        let source = grid.encoded.clone();
        let change = grid.copy_from_gpu(&source).unwrap();
        assert_eq!(change.mask, [u64::MAX; 64]);
        assert_eq!(grid.maximum_y, 24.0);
    }
}

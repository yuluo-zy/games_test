//! Linear initial data for textures.

/// Splits linear data into consecutive slices of `step` bytes.
pub struct LinearInitialDataIter<'a> {
    data: &'a [u8],
    step: usize,
}

impl<'a> LinearInitialDataIter<'a> {
    pub fn new(data: &'a [u8], step: usize) -> Self {
        Self {
            data,
            step: step.max(1),
        }
    }
}

impl<'a> Iterator for LinearInitialDataIter<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        let take = self.step.min(self.data.len());
        let (head, rest) = self.data.split_at(take);
        self.data = rest;
        Some(head)
    }
}

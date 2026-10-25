//! Successful submission receipts. Physical state is prepared and published by
//! the actual raw command batch while its SyncCell submission gates are held.
use crate::{
    Error, Result,
    sync::{SyncPoint, SyncSet},
};
#[derive(Debug, Default)]
pub struct FrameTransaction {
    planned_batches: usize,
    published_batches: usize,
    receipts: smallvec::SmallVec<[SyncPoint; 8]>,
    completion: SyncSet,
}
impl FrameTransaction {
    pub(crate) fn prepare(compiled: &super::graph::FrameInstance) -> Result<Self> {
        let mut transaction = Self {
            planned_batches: compiled.plan.batches.len(),
            ..Self::default()
        };
        transaction
            .receipts
            .try_reserve_exact(transaction.planned_batches)
            .map_err(|_| Error::OutOfHostMemory)?;
        Ok(transaction)
    }
    pub(crate) fn publish(&mut self, point: SyncPoint) {
        self.published_batches += 1;
        self.receipts.push(point);
        self.completion.insert(point);
    }
    pub(crate) fn reserve_receipt(&mut self) -> Result<()> {
        self.receipts
            .try_reserve(1)
            .map_err(|_| Error::OutOfHostMemory)
    }
    pub(crate) fn publish_auxiliary(&mut self, point: SyncPoint) {
        self.receipts.push(point);
        self.completion.insert(point);
    }
    pub fn receipts(&self) -> &[SyncPoint] {
        &self.receipts
    }
    pub fn completion(&self) -> SyncSet {
        self.completion
    }
    pub fn submitted_batches(&self) -> usize {
        self.published_batches
    }
    pub fn planned_batches(&self) -> usize {
        self.planned_batches
    }
}

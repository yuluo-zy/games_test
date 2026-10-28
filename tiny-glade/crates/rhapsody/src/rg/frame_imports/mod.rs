//! Frame-scoped physical imports. Unrecorded graphs own declaration leases;
//! after recording, commands retain resources through real submission points.
use crate::tracking::identity::PhysicalIdentity;
use super::resource::{GraphId, ResourceHandle};
use crate::{Error, Result};
use crate::collections::temp_list::TempList;
use crate::accounting::{AllocatorStats, BudgetCategory, CpuMemoryReservation, shared_cpu_bytes};
use std::sync::{Arc, Weak};

#[derive(Clone, Copy, Debug)]
pub struct FrameImportConfig {
    pub max_imports: usize,
}
impl Default for FrameImportConfig {
    fn default() -> Self {
        Self { max_imports: 4096 }
    }
}

struct LeaseMemory {
    _memory: CpuMemoryReservation,
}
pub(crate) struct LeaseMarker {
    _memory: Arc<LeaseMemory>,
}
pub(crate) struct ImportLease {
    marker: Arc<LeaseMarker>,
    memory: Arc<LeaseMemory>,
}
impl ImportLease {
    pub(crate) fn marker(&self) -> Arc<LeaseMarker> {
        self.marker.clone()
    }
}

struct Entry {
    handle: ResourceHandle,
    token: u64,
    // Destroy the final Weak before returning the Arc allocation's CPU credit.
    marker: Weak<LeaseMarker>,
    _memory: Arc<LeaseMemory>,
}
pub(crate) struct FrameImports {
    entries: TempList<Entry>,
    /// (graph, physical identity) -> declared handle. `find` used to scan every
    /// entry of every in-flight graph; a graph imports the same texture table
    /// once per pass, so that scan ran thousands of times per frame.
    index: std::collections::HashMap<(GraphId, PhysicalIdentity), (ResourceHandle, Weak<LeaseMarker>)>,
    budget: Arc<AllocatorStats>,
}
impl FrameImports {
    pub(crate) fn new(config: FrameImportConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        if config.max_imports == 0 || config.max_imports > usize::from(u16::MAX) {
            return Err(Error::invalid_usage("invalid frame import limit"));
        }
        Ok(Self {
            entries: TempList::new(config.max_imports, budget.clone())?,
            index: Default::default(),
            budget,
        })
    }
    pub(crate) fn find(
        &self,
        graph: GraphId,
        identity: PhysicalIdentity,
    ) -> Option<ResourceHandle> {
        let (handle, marker) = self.index.get(&(graph, identity))?;
        (marker.strong_count() != 0).then_some(*handle)
    }
    pub(crate) fn reserve(&mut self) -> Result<ImportLease> {
        self.entries.reserve(self.entries.as_slice().len() + 1)?;
        // The allocation stays charged through the last graph/command callback
        // and registry reference, even when recording releases admission first.
        let memory = self.budget.reserve_cpu_bytes(
            shared_cpu_bytes::<LeaseMarker>()?
                .checked_add(shared_cpu_bytes::<LeaseMemory>()?)
                .and_then(|bytes| bytes.checked_add(std::mem::size_of::<Arc<LeaseMarker>>() as u64))
                .ok_or(Error::OutOfHostMemory)?,
            BudgetCategory::Live,
        )?;
        let memory = Arc::new(LeaseMemory { _memory: memory });
        Ok(ImportLease {
            marker: Arc::new(LeaseMarker {
                _memory: memory.clone(),
            }),
            memory,
        })
    }
    pub(crate) fn insert(
        &mut self,
        identity: PhysicalIdentity,
        token: u64,
        handle: ResourceHandle,
        lease: ImportLease,
    ) -> Result<()> {
        self.entries.add(Entry {
            handle,
            token,
            marker: Arc::downgrade(&lease.marker),
            _memory: lease.memory,
        })?;
        self.index.insert(
            (handle.graph_id(), identity),
            (handle, Arc::downgrade(&lease.marker)),
        );
        Ok(())
    }
    pub(crate) fn collect(
        &mut self,
        recorded: Option<(GraphId, &[u16])>,
        mut remove: impl FnMut(u64),
    ) {
        // Exports remain addressable by post-submit consumers until retirement.
        // Other declarations are only needed until recording has transferred
        // physical ownership to commands. Drop the index's Weak first.
        let needed = |handle: ResourceHandle| recorded.is_none_or(|(graph, exports)| {
            handle.graph_id() != graph
                || exports.iter().any(|index| u32::from(*index) == handle.raw())
        });
        self.index.retain(|_, (handle, marker)| {
            needed(*handle) && marker.strong_count() != 0
        });
        self.entries.retain(|entry| {
            if needed(entry.handle) && entry.marker.strong_count() != 0 {
                return true;
            }
            remove(entry.token);
            false
        });
    }
    pub(crate) fn len(&self) -> usize {
        self.entries.as_slice().len()
    }
}

#[cfg(test)]
mod tests;

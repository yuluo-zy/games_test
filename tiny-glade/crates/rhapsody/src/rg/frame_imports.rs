//! Frame-scoped physical imports. A graph owns its declaration lease; commands
//! independently retain actual resources through their real submission points.
use super::resource::{GraphId, ResourceHandle};
use crate::{
    Error, Result,
    device::BudgetCategory,
    raw::{
        allocator::{AllocatorStats, CpuMemoryReservation, shared_cpu_bytes},
        temp_list::TempList,
    },
};
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum PhysicalIdentity {
    Buffer(usize),
    Image(usize),
}
struct Entry {
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
        // The allocation's weak count and the graph's boxed Arc capture remain
        // charged until both graph retirement and registry removal occur.
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
    pub(crate) fn collect(&mut self, mut remove: impl FnMut(u64)) {
        // Drop the index's Weak first: the entry's Weak must be the last one
        // alive when its CPU credit is returned.
        self.index.retain(|_, (_, marker)| marker.strong_count() != 0);
        self.entries.retain(|entry| {
            if entry.marker.strong_count() != 0 {
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
mod tests {
    use super::*;

    #[test]
    fn dropped_graph_lease_is_collected_once_and_live_graphs_apply_backpressure() {
        let budget = Arc::new(AllocatorStats::default());
        let mut imports =
            FrameImports::new(FrameImportConfig { max_imports: 2 }, budget.clone()).unwrap();
        let graph = GraphId::next();
        let first = imports.reserve().unwrap();
        let first_graph_owner = first.marker();
        let first_handle = ResourceHandle::scoped(graph, 3);
        imports
            .insert(PhysicalIdentity::Buffer(17), 41, first_handle, first)
            .unwrap();
        assert_eq!(
            imports.find(graph, PhysicalIdentity::Buffer(17)),
            Some(first_handle)
        );
        assert_eq!(
            imports.find(GraphId::next(), PhysicalIdentity::Buffer(17)),
            None
        );
        let second = imports.reserve().unwrap();
        let second_graph_owner = second.marker();
        imports
            .insert(
                PhysicalIdentity::Buffer(18),
                42,
                ResourceHandle::scoped(graph, 4),
                second,
            )
            .unwrap();
        assert!(matches!(imports.reserve(), Err(Error::Timeout)));
        let charged = budget.cpu_memory_usage().total();
        let mut removed = Vec::new();
        imports.collect(|token| removed.push(token));
        assert!(removed.is_empty());
        drop(first_graph_owner);
        assert_eq!(budget.cpu_memory_usage().total(), charged);
        imports.collect(|token| removed.push(token));
        assert_eq!(removed, [41]);
        assert!(budget.cpu_memory_usage().total() < charged);
        assert_eq!(imports.find(graph, PhysicalIdentity::Buffer(17)), None);
        drop(second_graph_owner);
        imports.collect(|token| removed.push(token));
        imports.collect(|token| removed.push(token));
        assert_eq!(removed, [41, 42]);
        drop(imports);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn physical_identity_keeps_resource_kinds_distinct() {
        let budget = Arc::new(AllocatorStats::default());
        let mut imports = FrameImports::new(FrameImportConfig { max_imports: 2 }, budget).unwrap();
        let graph = GraphId::next();
        let buffer = imports.reserve().unwrap();
        let buffer_owner = buffer.marker();
        imports
            .insert(
                PhysicalIdentity::Buffer(17),
                1,
                ResourceHandle::scoped(graph, 0),
                buffer,
            )
            .unwrap();
        assert_eq!(imports.find(graph, PhysicalIdentity::Image(17)), None);
        let image = imports.reserve().unwrap();
        let image_owner = image.marker();
        imports
            .insert(
                PhysicalIdentity::Image(17),
                2,
                ResourceHandle::scoped(graph, 1),
                image,
            )
            .unwrap();
        assert_eq!(
            imports
                .find(graph, PhysicalIdentity::Buffer(17))
                .unwrap()
                .raw(),
            0
        );
        assert_eq!(
            imports
                .find(graph, PhysicalIdentity::Image(17))
                .unwrap()
                .raw(),
            1
        );
        drop((buffer_owner, image_owner));
        imports.collect(|_| {});
        assert_eq!(imports.len(), 0);
    }

    #[test]
    fn failed_admission_never_registers_an_import() {
        let budget = Arc::new(AllocatorStats::default());
        let mut imports =
            FrameImports::new(FrameImportConfig { max_imports: 1 }, budget.clone()).unwrap();
        budget.set_total_budget(1).unwrap();
        assert!(matches!(imports.reserve(), Err(Error::OutOfHostMemory)));
        assert_eq!(imports.len(), 0);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn an_unsubmitted_graph_outlives_its_session_without_returning_its_lease_credit() {
        let budget = Arc::new(AllocatorStats::default());
        let mut imports =
            FrameImports::new(FrameImportConfig { max_imports: 1 }, budget.clone()).unwrap();
        let lease = imports.reserve().unwrap();
        let graph_owner = lease.marker();
        imports
            .insert(
                PhysicalIdentity::Buffer(1),
                9,
                ResourceHandle::scoped(GraphId::next(), 0),
                lease,
            )
            .unwrap();
        drop(imports);
        assert_ne!(budget.cpu_memory_usage().total(), 0);
        drop(graph_owner);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
}

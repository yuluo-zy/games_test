//! Owned per-frame metadata, separate from temporary recording arenas and GPU
//! memory. A frame returns its tables only when its last CPU owner is dropped.
use super::{graph, plan::CompiledPlan};
use crate::{
    Error, Result,
    device::{BudgetAction, BudgetCategory},
    raw::allocator::{AllocatorStats, CpuMemoryReservation},
};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct FrameStorageConfig {
    pub max_frames: usize,
    pub max_total_bytes: usize,
    pub max_cached_bytes: usize,
}
impl Default for FrameStorageConfig {
    fn default() -> Self {
        Self {
            max_frames: 32,
            max_total_bytes: 4 << 20,
            max_cached_bytes: 1 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameStorageStats {
    pub checked_out: usize,
    pub cached_frames: usize,
    pub live_bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub capacity_growths: u64,
    pub capacity_stalls: u64,
}

#[derive(Default)]
pub(crate) struct FrameTables {
    pub registry: graph::Registry,
    pub physical_mapping: Vec<graph::PhysicalMapping>,
    pub pool_hits: Vec<u32>,
    pub history_validity: Vec<Option<bool>>,
}
fn bytes<T>(capacity: usize) -> usize {
    capacity.saturating_mul(std::mem::size_of::<T>())
}
fn desired<T>(values: &Vec<T>, len: usize) -> usize {
    bytes::<T>(values.capacity().max(len))
}
fn reserve<T>(values: &mut Vec<T>, len: usize) -> Result<()> {
    if values.capacity() < len {
        values
            .try_reserve_exact(len.saturating_sub(values.len()))
            .map_err(|_| Error::OutOfHostMemory)?;
    }
    Ok(())
}
impl FrameTables {
    fn capacity_bytes(&self) -> usize {
        self.desired_bytes(0, 0)
    }
    fn desired_bytes(&self, resources: usize, exports: usize) -> usize {
        desired(&self.registry.resources, resources)
            .saturating_add(desired(&self.registry.resource_initial_access, resources))
            .saturating_add(desired(&self.registry.backing_resources, resources))
            .saturating_add(desired(&self.registry.exported_logical_handles, exports))
            .saturating_add(desired(&self.physical_mapping, resources))
            .saturating_add(desired(&self.pool_hits, resources))
            .saturating_add(desired(&self.history_validity, resources))
    }
    pub(crate) fn reserve_plan(&mut self, plan: &CompiledPlan) -> Result<()> {
        let resources = plan.info.lifetimes.len();
        let exports = plan.definition.exports.len();
        reserve(&mut self.registry.resources, resources)?;
        reserve(&mut self.registry.resource_initial_access, resources)?;
        reserve(&mut self.registry.backing_resources, resources)?;
        reserve(&mut self.registry.exported_logical_handles, exports)?;
        reserve(&mut self.physical_mapping, resources)?;
        reserve(&mut self.pool_hits, resources)?;
        reserve(&mut self.history_validity, resources)?;
        Ok(())
    }
    fn clear(&mut self) {
        self.registry.resources.clear();
        self.registry.resource_initial_access.clear();
        self.registry.backing_resources.clear();
        self.registry.exported_logical_handles.clear();
        self.physical_mapping.clear();
        self.pool_hits.clear();
        self.history_validity.clear();
    }
}
struct CachedTables {
    // Free storage before crediting its exact capacity reservation.
    tables: FrameTables,
    reservation: CpuMemoryReservation,
    bytes: usize,
}
struct State {
    idle: Vec<CachedTables>,
    stats: FrameStorageStats,
    _metadata: CpuMemoryReservation,
}
struct Shared {
    config: FrameStorageConfig,
    budget: Arc<AllocatorStats>,
    state: Mutex<State>,
}
#[derive(Clone)]
pub struct FrameStoragePool {
    shared: Arc<Shared>,
}
impl FrameStoragePool {
    pub fn new(config: FrameStorageConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        if !(1..=256).contains(&config.max_frames)
            || config.max_total_bytes == 0
            || config.max_cached_bytes > config.max_total_bytes
        {
            return Err(Error::invalid_usage(
                "invalid bounded frame metadata configuration",
            ));
        }
        let mut idle = Vec::new();
        idle.try_reserve_exact(config.max_frames)
            .map_err(|_| Error::OutOfHostMemory)?;
        // Exact free-list capacity plus a conservative fixed owner envelope.
        // The latter includes Shared/State and the Arc counters/alignment; no
        // driver memory estimate is mixed into the GPU allocation ledger.
        let metadata_bytes = bytes::<CachedTables>(idle.capacity())
            .saturating_add(std::mem::size_of::<Shared>())
            .saturating_add(2 * std::mem::size_of::<usize>())
            .saturating_add(std::mem::align_of::<Shared>());
        if metadata_bytes >= config.max_total_bytes {
            return Err(Error::invalid_usage(
                "frame metadata limit cannot hold its free list",
            ));
        }
        let metadata = budget.reserve_cpu_bytes(metadata_bytes as u64, BudgetCategory::Live)?;
        Ok(Self {
            shared: Arc::new(Shared {
                config,
                budget,
                state: Mutex::new(State {
                    idle,
                    stats: FrameStorageStats {
                        metadata_bytes,
                        ..Default::default()
                    },
                    _metadata: metadata,
                }),
            }),
        })
    }
    pub fn config(&self) -> FrameStorageConfig {
        self.shared.config
    }
    pub fn stats(&self) -> FrameStorageStats {
        self.shared.state.lock().stats
    }
    pub fn trim_idle(&self, target_bytes: usize) -> usize {
        let mut state = self.shared.state.lock();
        let mut freed = 0;
        while state.stats.cached_bytes > target_bytes {
            let Some(tables) = state.idle.pop() else {
                break;
            };
            state.stats.cached_bytes -= tables.bytes;
            state.stats.cached_frames -= 1;
            freed += tables.bytes;
            drop(tables);
        }
        freed
    }
    pub(crate) fn checkout(&self, plan: &CompiledPlan) -> Result<FrameStorageLease> {
        let mut state = self.shared.state.lock();
        let mut cached = if let Some(cached) = state.idle.pop() {
            state.stats.cached_frames -= 1;
            state.stats.cached_bytes -= cached.bytes;
            cached
        } else {
            if state.stats.checked_out >= self.shared.config.max_frames {
                state.stats.capacity_stalls += 1;
                return Err(Error::Timeout);
            }
            CachedTables {
                tables: FrameTables::default(),
                reservation: self
                    .shared
                    .budget
                    .reserve_cpu_bytes(0, BudgetCategory::Live)?,
                bytes: 0,
            }
        };
        cached.reservation.reclassify(BudgetCategory::Live);
        state.stats.checked_out += 1;
        state.stats.live_bytes += cached.bytes;
        drop(state);
        let mut lease = FrameStorageLease {
            cached: Some(cached),
            shared: self.shared.clone(),
            reusable: false,
        };
        lease.reserve_plan(plan)?;
        Ok(lease)
    }
}
pub(crate) struct FrameStorageLease {
    cached: Option<CachedTables>,
    shared: Arc<Shared>,
    reusable: bool,
}
impl FrameStorageLease {
    fn reserve_plan(&mut self, plan: &CompiledPlan) -> Result<()> {
        let cached = self.cached.as_mut().expect("active frame tables");
        // No temporary count table: use the plan's immutable ranges directly.
        let required = cached
            .tables
            .desired_bytes(plan.info.lifetimes.len(), plan.definition.exports.len());
        let mut state = self.shared.state.lock();
        if state.stats.live_bytes
            + state.stats.cached_bytes
            + state.stats.metadata_bytes
            + required.saturating_sub(cached.bytes)
            > self.shared.config.max_total_bytes
        {
            state.stats.capacity_stalls += 1;
            return Err(Error::Timeout);
        }
        cached.reservation.resize(required as u64)?;
        let result = cached.tables.reserve_plan(plan);
        let actual = cached.tables.capacity_bytes();
        // try_reserve_exact guarantees at least the requested capacity. Charge
        // the observed capacity too, and reject an allocator's extra capacity.
        let budgeted = cached.reservation.resize(actual as u64);
        state.stats.live_bytes = state.stats.live_bytes - cached.bytes + actual;
        state.stats.capacity_growths += u64::from(actual > cached.bytes);
        cached.bytes = actual;
        result?;
        budgeted?;
        if state.stats.live_bytes + state.stats.cached_bytes + state.stats.metadata_bytes
            > self.shared.config.max_total_bytes
        {
            return Err(Error::Timeout);
        }
        self.reusable = true;
        Ok(())
    }
    pub(crate) fn take_tables(&mut self) -> FrameTables {
        std::mem::take(&mut self.cached.as_mut().expect("active frame tables").tables)
    }
    pub(crate) fn return_tables(&mut self, tables: FrameTables) {
        let cached = self.cached.as_mut().expect("active frame tables");
        let actual = tables.capacity_bytes();
        // Explicitly released table capacity can shrink retained storage. Public
        // caller mutations must never cause the recycler to cache extra bytes
        // which were not admitted by this owner.
        if actual > cached.bytes {
            self.reusable = false;
        } else {
            let mut state = self.shared.state.lock();
            state.stats.live_bytes = state.stats.live_bytes - cached.bytes + actual;
            cached
                .reservation
                .resize(actual as u64)
                .expect("shrinking CPU reservation");
            cached.bytes = actual;
        }
        cached.tables = tables;
    }
}
impl Drop for FrameStorageLease {
    fn drop(&mut self) {
        let mut cached = self.cached.take().expect("active frame tables");
        cached.tables.clear();
        let mut state = self.shared.state.lock();
        state.stats.checked_out -= 1;
        state.stats.live_bytes -= cached.bytes;
        if self.reusable
            && state.stats.cached_bytes.saturating_add(cached.bytes)
                <= self.shared.config.max_cached_bytes
            && !matches!(
                self.shared.budget.budget_action(),
                BudgetAction::EvictCache | BudgetAction::Backpressure
            )
        {
            cached.reservation.reclassify(BudgetCategory::Cache);
            state.stats.cached_bytes += cached.bytes;
            state.stats.cached_frames += 1;
            state.idle.push(cached);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn plan(resources: usize, passes: usize) -> CompiledPlan {
        let mut graph = graph::RenderGraph::new();
        let resources = (0..resources)
            .map(|index| {
                graph
                    .import_unowned(
                        format!("resource{index}"),
                        graph::Descriptor::Buffer(graph::BufferDesc {
                            word_at_00: 64,
                            word_at_08: 0,
                            bytes_at_0c_through_0e: [0; 3],
                        }),
                        index as u64 + 1,
                        0x25,
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
        for index in 0..passes {
            let mut pass = graph.add_pass(format!("pass{index}"));
            pass.side_effect();
            for resource in &resources {
                pass.add_resource_access(*resource, 0x25, 0).unwrap();
            }
            pass.render_with_result_context(|_| Ok(()));
        }
        if let Some(resource) = resources.first() {
            graph.export(*resource, 0x25).unwrap();
        }
        let (declaration, _, _) = graph.build().into_parts();
        super::super::plan::compile_definition(
            super::super::plan::GraphDefinition::from_declaration(
                &declaration,
                &mut super::super::vulkan::AccessTable::original(),
                Default::default(),
            )
            .unwrap(),
        )
        .unwrap()
    }
    fn config() -> FrameStorageConfig {
        FrameStorageConfig {
            max_frames: 2,
            max_total_bytes: 256 << 10,
            max_cached_bytes: 128 << 10,
        }
    }
    #[test]
    fn real_tables_reuse_beyond_inline_sizes_and_return_only_after_cpu_lease_drop() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = FrameStoragePool::new(config(), budget.clone()).unwrap();
        let plan = plan(32, 12);
        assert_eq!(plan.info.pass_access_ranges.len(), 12);
        assert_eq!(plan.info.lifetimes.len(), 32);
        let mut first = pool.checkout(&plan).unwrap();
        let tables = first.take_tables();
        let pointers = (
            tables.registry.resources.as_ptr(),
            tables.physical_mapping.as_ptr(),
        );
        let actual = tables.capacity_bytes();
        assert!(actual > 0);
        first.return_tables(tables);
        let second = pool.checkout(&plan).unwrap();
        assert!(matches!(pool.checkout(&plan), Err(Error::Timeout)));
        assert_eq!(pool.trim_idle(0), 0);
        assert_eq!(pool.stats().checked_out, 2);
        drop(first);
        for _ in 0..1000 {
            let mut reused = pool.checkout(&plan).unwrap();
            let tables = reused.take_tables();
            assert_eq!(
                (
                    tables.registry.resources.as_ptr(),
                    tables.physical_mapping.as_ptr()
                ),
                pointers
            );
            reused.return_tables(tables);
        }
        assert_eq!(pool.stats().capacity_growths, 2);
        assert_eq!(pool.stats().checked_out, 1);
        let metadata = pool.stats().metadata_bytes;
        assert_eq!(budget.cpu_memory_usage().live, (actual + metadata) as u64);
        assert_eq!(budget.cpu_memory_usage().cache, actual as u64);
        assert_eq!(pool.trim_idle(0), actual);
        assert_eq!(budget.cpu_memory_usage().cache, 0);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().live, (actual + metadata) as u64);
        drop(second);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
        assert!(budget.report().allocations.is_empty());
    }
    #[test]
    fn quota_and_local_capacity_failures_are_unpublished_and_preserve_categories() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = FrameStoragePool::new(config(), budget.clone()).unwrap();
        let metadata = pool.stats().metadata_bytes as u64;
        budget.set_total_budget(metadata + 1).unwrap();
        assert!(matches!(
            pool.checkout(&plan(32, 12)),
            Err(Error::OutOfHostMemory)
        ));
        assert_eq!(pool.stats().checked_out, 0);
        assert_eq!(budget.cpu_memory_usage().total(), metadata);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);

        let pool = FrameStoragePool::new(
            FrameStorageConfig {
                max_frames: 1,
                max_total_bytes: 4096,
                max_cached_bytes: 4096,
            },
            Arc::new(AllocatorStats::default()),
        )
        .unwrap();
        assert!(matches!(pool.checkout(&plan(128, 16)), Err(Error::Timeout)));
        assert_eq!(pool.stats().checked_out, 0);
        assert_eq!(pool.stats().capacity_stalls, 1);
    }
    #[test]
    fn shrinking_tables_and_idle_eviction_credit_only_released_capacity() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = FrameStoragePool::new(config(), budget.clone()).unwrap();
        let mut lease = pool.checkout(&plan(16, 9)).unwrap();
        let mut tables = lease.take_tables();
        let original = tables.capacity_bytes();
        // Release the actual allocation before reducing its capacity ticket.
        drop(std::mem::take(&mut tables.registry.backing_resources));
        let retained = tables.capacity_bytes();
        lease.return_tables(tables);
        assert!(retained < original);
        assert_eq!(pool.stats().live_bytes, retained);
        drop(lease);
        assert_eq!(pool.stats().cached_bytes, retained);
        assert_eq!(pool.trim_idle(0), retained);
        assert_eq!(
            budget.cpu_memory_usage().total(),
            pool.stats().metadata_bytes as u64
        );
    }
}

//! No-driver adapter for the production range planner and state table. Faults
//! are injected by the caller before `publish`, just like a failed queue call.
use rhapsody::{
    Error, Result,
    raw::allocator::AllocatorStats,
    resource::{
        OwnershipRange,
        range_state::{RangeOwner, RangeStateTable, StatePatch},
    },
    rg::{
        graph::{AccessDependency, FrameInstance},
        queue_schedule::QueueDomain,
        sync_plan::{SynchronizationConfig, SynchronizationPlanner, initial_native_state},
        vulkan::AccessTable,
    },
    sync::SyncPoint,
};
use std::{collections::BTreeMap, sync::Arc};

pub struct CpuRanges {
    pub states: BTreeMap<u64, RangeStateTable>,
    pub planner: Arc<SynchronizationPlanner>,
    budget: Arc<AllocatorStats>,
    tokens: Vec<Option<u64>>,
    receipts: Vec<Option<SyncPoint>>,
    patches: Vec<(u64, StatePatch)>,
    scratch: Vec<StatePatch>,
    prepared: Vec<u64>,
    pending: Option<usize>,
}
impl Default for CpuRanges {
    fn default() -> Self {
        let budget = Arc::new(AllocatorStats::default());
        Self {
            states: BTreeMap::new(),
            planner: Arc::new(
                SynchronizationPlanner::new(budget.clone(), SynchronizationConfig::default())
                    .unwrap(),
            ),
            budget,
            tokens: Vec::new(),
            receipts: Vec::new(),
            patches: Vec::new(),
            scratch: Vec::new(),
            prepared: Vec::new(),
            pending: None,
        }
    }
}
impl CpuRanges {
    pub fn bind(&mut self, compiled: &FrameInstance) -> Result<()> {
        self.cancel();
        let planner = Arc::get_mut(&mut self.planner).ok_or_else(|| {
            Error::invalid_usage("previous CPU recorder retained synchronization scratch")
        })?;
        planner.begin();
        self.tokens
            .resize(compiled.plan.range_program.resources.len(), None);
        self.tokens.fill(None);
        for mapping in &compiled.snapshot.physical_mapping {
            let Some(token) = mapping.token else { continue };
            let index = mapping.logical as usize;
            let domain = compiled.plan.range_program.resources[index];
            self.tokens[index] = Some(token);
            if let std::collections::btree_map::Entry::Vacant(entry) = self.states.entry(token) {
                let (scope, mut layout) =
                    AccessTable::original().native_semantics(mapping.initial_access);
                if matches!(domain.range, OwnershipRange::Buffer { .. }) {
                    layout = ash::vk::ImageLayout::UNDEFINED;
                }
                let context = compiled.plan.definition.context;
                let queue = rhapsody::sync::QueueId::new(context.queue_index as usize).unwrap();
                let initial = initial_native_state(scope, layout, queue, RangeOwner::Unclaimed);
                entry.insert(RangeStateTable::new(
                    domain.range,
                    domain.sharing,
                    initial,
                    Default::default(),
                    self.budget.clone(),
                )?);
            }
            let table = &self.states[&token];
            if table.domain() != domain.range {
                return Err(Error::invalid_usage(
                    "CPU resource extent changed without a new identity",
                ));
            }
            planner.add_initial(mapping.logical, table.cells())?;
        }
        let context = compiled.plan.definition.context;
        planner.bind(&compiled.plan.range_program, |family| {
            if let Some(domains) = context.queue_domains {
                domains
                    .into_iter()
                    .find(|domain| domain.family == family)
                    .ok_or_else(|| Error::unsupported("CPU fixture has no queue for owner family"))
            } else if context.queue_family == family {
                Ok(QueueDomain {
                    queue_index: context.queue_index,
                    family,
                })
            } else {
                Err(Error::unsupported(
                    "CPU fixture has no queue for owner family",
                ))
            }
        })?;
        self.receipts.resize(planner.bound().batches().len(), None);
        self.receipts.fill(None);
        Ok(())
    }
    pub fn prepare(&mut self, batch: usize) -> Result<()> {
        self.cancel();
        let bound = self.planner.bound();
        let domain = bound
            .batches()
            .get(batch)
            .ok_or_else(|| Error::invalid_usage("CPU batch is absent"))?
            .domain;
        let count = bound
            .patches()
            .iter()
            .filter(|patch| patch.batch == batch)
            .count();
        self.patches.clear();
        self.patches
            .try_reserve(count)
            .map_err(|_| Error::OutOfHostMemory)?;
        self.scratch
            .try_reserve(count)
            .map_err(|_| Error::OutOfHostMemory)?;
        self.prepared
            .try_reserve(count)
            .map_err(|_| Error::OutOfHostMemory)?;
        for patch in bound.patches().iter().filter(|patch| patch.batch == batch) {
            let token = self
                .tokens
                .get(patch.resource as usize)
                .copied()
                .flatten()
                .ok_or_else(|| Error::invalid_usage("CPU patch has no physical resource"))?;
            self.patches
                .push((token, patch.resolve(|source| self.receipts[source])?));
        }
        let queue = rhapsody::sync::QueueId::new(domain.queue_index as usize)
            .ok_or_else(|| Error::invalid_usage("CPU queue index is invalid"))?;
        for index in 0..self.patches.len() {
            let token = self.patches[index].0;
            if self.prepared.contains(&token) {
                continue;
            }
            self.scratch.clear();
            self.scratch.extend(
                self.patches
                    .iter()
                    .filter(|entry| entry.0 == token)
                    .map(|entry| entry.1),
            );
            if let Err(error) =
                self.states
                    .get_mut(&token)
                    .unwrap()
                    .prepare(&self.scratch, queue, domain.family)
            {
                self.cancel();
                return Err(error);
            }
            self.prepared.push(token);
        }
        self.pending = Some(batch);
        Ok(())
    }
    pub fn publish(&mut self, point: SyncPoint) {
        for token in self.prepared.drain(..) {
            self.states.get_mut(&token).unwrap().publish(point);
        }
        if let Some(batch) = self.pending.take() {
            self.receipts[batch] = Some(point);
        }
    }
    fn cancel(&mut self) {
        for token in self.prepared.drain(..) {
            self.states.get_mut(&token).unwrap().cancel_prepared();
        }
        self.pending = None;
    }
}

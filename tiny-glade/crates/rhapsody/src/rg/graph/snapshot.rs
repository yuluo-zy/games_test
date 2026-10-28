//! Resource snapshots and per-frame instances of a built graph.
use crate::rg::compiled as batch_state;
use crate::rg::resource_info::LifetimeEvent;
use crate::rg::resource_info::ResourceAccessRecord;
use crate::rg::resource_info::ResourceInfo;
use crate::rg::resource_info::ResourceLifetimeRecord;
use std::ops::Range;
use super::Registry;
use super::declaration::GraphDeclaration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalMapping {
    pub logical: u32,
    pub physical: u16,
    pub reused: bool,
    pub initial_access: u8,
    pub payload_tag: u8,
    pub token: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotStatic {
    pub pass_order: Vec<String>,
    pub pass_ranges: Vec<Range<usize>>,
    pub accesses: Vec<ResourceAccessRecord>,
    pub lifetimes: Vec<ResourceLifetimeRecord>,
    pub image_usage: Vec<u32>,
    pub buffer_usage: Vec<u32>,
    pub lifetime_events: Vec<LifetimeEvent>,
    pub batches: Vec<batch_state::Batch>,
    pub imports: Vec<u32>,
    pub exports: Vec<u16>,
}

impl SnapshotStatic {
    pub(crate) fn from_plan(
        definition: &super::super::plan::GraphDefinition,
        retained: &[usize],
        info: &ResourceInfo,
        batches: &[batch_state::Batch],
    ) -> Self {
        let mut exports = info.exported_indices.iter().copied().collect::<Vec<_>>();
        exports.sort_unstable();
        Self {
            pass_order: retained
                .iter()
                .map(|&index| definition.passes[index].name.to_string())
                .collect(),
            pass_ranges: info
                .pass_access_ranges
                .iter()
                .map(|(start, end)| start.0 as usize..end.0 as usize)
                .collect(),
            accesses: info.accesses.clone(),
            lifetimes: info.lifetimes.clone(),
            image_usage: info.image_usage_flags.clone(),
            buffer_usage: info.buffer_usage_flags.clone(),
            lifetime_events: info.lifetime_events.clone(),
            batches: batches.to_vec(),
            imports: definition
                .resources
                .iter()
                .enumerate()
                .filter_map(|(index, resource)| resource.is_external().then_some(index as u32))
                .collect(),
            exports,
        }
    }
    pub(crate) fn estimated_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self
                .pass_order
                .iter()
                .map(|name| std::mem::size_of::<String>() + name.capacity())
                .sum::<usize>()
            + self.pass_ranges.capacity() * std::mem::size_of::<Range<usize>>()
            + self.accesses.capacity() * std::mem::size_of::<ResourceAccessRecord>()
            + self.lifetimes.capacity() * std::mem::size_of::<ResourceLifetimeRecord>()
            + (self.image_usage.capacity() + self.buffer_usage.capacity() + self.imports.capacity())
                * std::mem::size_of::<u32>()
            + self.lifetime_events.capacity() * std::mem::size_of::<LifetimeEvent>()
            + self.batches.capacity() * std::mem::size_of::<batch_state::Batch>()
            + self.exports.capacity() * std::mem::size_of::<u16>()
    }
}

/// Immutable plan data is shared by frame receipts. Physical resource mappings
/// remain owned by the individual frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub shared: std::sync::Arc<SnapshotStatic>,
    pub physical_mapping: Vec<PhysicalMapping>,
    pub pool_hits: Vec<u32>,
}

impl std::ops::Deref for Snapshot {
    type Target = SnapshotStatic;
    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}

impl std::ops::DerefMut for Snapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        std::sync::Arc::make_mut(&mut self.shared)
    }
}

pub struct FrameInstance {
    pub plan: std::sync::Arc<super::super::plan::CompiledPlan>,
    pub declarations: GraphDeclaration,
    pub info: std::sync::Arc<ResourceInfo>,
    pub(crate) tables: FrameInstanceTables,
    pub(super) storage: Option<super::super::frame_storage::FrameStorageLease>,
}

/// Read-only dynamic frame metadata. Capacity growth is admitted before the
/// frame is published; public callers cannot mutate or move pooled vectors.
pub struct FrameInstanceTables {
    pub registry: Registry,
    pub snapshot: Snapshot,
    pub(crate) history_validity: Option<Vec<Option<bool>>>,
}

impl std::ops::Deref for FrameInstance {
    type Target = FrameInstanceTables;
    fn deref(&self) -> &Self::Target {
        &self.tables
    }
}

impl Drop for FrameInstance {
    fn drop(&mut self) {
        if let Some(mut storage) = self.storage.take() {
            storage.return_tables(super::super::frame_storage::FrameTables {
                registry: std::mem::take(&mut self.tables.registry),
                physical_mapping: std::mem::take(&mut self.tables.snapshot.physical_mapping),
                pool_hits: std::mem::take(&mut self.tables.snapshot.pool_hits),
                history_validity: self.tables.history_validity.take().unwrap_or_default(),
            });
        }
    }
}

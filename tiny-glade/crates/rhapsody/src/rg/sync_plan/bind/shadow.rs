//! Per-resource shadow state tracking the last access of each range cell.
use crate::Result;
use ash::vk;
use crate::rg::sync_plan::{AliasEdge, BoundBatch, BoundPatch, BoundTransition, BoundWait, InitialState};
use crate::Error;
use crate::rg::sync_plan::DependencyRef;
use crate::tracking::range_state::NativeRange;
use crate::tracking::range_state::RangeCell;
use crate::tracking::range_state::RangeOwner;
use crate::tracking::range_state::RangeStateConfig;
use super::SynchronizationPlanner;
use super::frontier::PendingFrontier;
use super::frontier::PendingOwner;

macro_rules! push {
    ($name:ident,$field:ident,$ty:ty,$limit:ident) => {
        pub(super) fn $name(&mut self, value: $ty) -> Result<()> {
            let target = &mut self.bound.$field;
            let config = RangeStateConfig {
                max_cells: self.config.$limit,
                max_cpu_bytes: self.config.max_cpu_bytes,
            };
            let needed = target
                .values
                .len()
                .checked_add(1)
                .ok_or(Error::OutOfHostMemory)?;
            let capacity = needed
                .max(target.values.capacity().saturating_mul(2))
                .min(config.max_cells);
            if needed > config.max_cells {
                return Err(Error::Timeout);
            }
            if needed > target.values.capacity() {
                let own = target.bytes();
                let total = self.known_cpu_bytes();
                let target = &mut self.bound.$field;
                target.grow(capacity, total - own, config, &self.budget)?;
            }
            self.bound.$field.values.push(value);
            Ok(())
        }
    };
}

pub(super) fn shadow_bind_check() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("RHAPSODY_BIND_SHADOW").is_some_and(|v| v == "1"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ShadowCell {
    pub(super) resource: u32,
    pub(super) range: NativeRange,
    pub(super) layout: vk::ImageLayout,
    pub(super) layout_known: bool,
    pub(super) writer: PendingFrontier,
    pub(super) readers: PendingFrontier,
    pub(super) owner: PendingOwner,
    pub(super) revision: u64,
    /// Not yet touched by this binding; alias edges apply on the first touch.
    pub(super) fresh: bool,
    /// Index of the `initial` row this cell (or its ancestor) came from.
    pub(super) initial_slot: u32,
}

impl ShadowCell {
    fn new(resource: u32, cell: RangeCell, initial_slot: u32) -> Self {
        let state = cell.state;
        Self {
            resource,
            range: cell.range,
            layout: state.layout,
            layout_known: state.layout_known,
            writer: PendingFrontier::from_native(state.writer),
            readers: PendingFrontier::from_native(state.readers),
            owner: match state.owner {
                RangeOwner::Unclaimed => PendingOwner::Unclaimed,
                RangeOwner::Owned(family) => PendingOwner::Owned(family),
                RangeOwner::Released { transfer, point } => PendingOwner::Released {
                    transfer,
                    release: DependencyRef::Point(point),
                },
            },
            revision: state.revision,
            fresh: true,
            initial_slot,
        }
    }
    pub(super) fn increment(&mut self) -> Result<()> {
        self.revision = self
            .revision
            .checked_add(1)
            .filter(|revision| *revision != u64::MAX)
            .ok_or_else(|| Error::unsupported("native range revision exhausted"))?;
        Ok(())
    }
}

/// CPU-only binding scratch and the read-only native projection share one
/// bounded owner. GPU completion never extends this CPU lease: commands copy
/// only their admitted StatePatches and retain the physical raw resources.
/// An outstanding diagnostic borrow prevents a caller from beginning a frame.
/// Per-resource view of the flat shadow array, so a use only visits the cells
/// of its own resource instead of every cell of the frame.
#[derive(Default)]
pub(super) struct ResourceShadow {
    /// Indices into `shadow.values`, ascending (the order the flat scan used).
    pub(super) cells: smallvec::SmallVec<[u32; 4]>,
    /// bit0: some alias edge has this resource as `next`; bit1: as `previous`.
    pub(super) alias_flags: u8,
    /// Every `previous` of an alias edge whose `next` is this resource.
    pub(super) alias_previous: smallvec::SmallVec<[u32; 2]>,
}

impl SynchronizationPlanner {
    /// The per-resource index is sized by resource number before `bind`
    /// validates it against the program; refuse numbers no graph can have
    /// instead of allocating a table for them.
    fn check_resource(&self, resource: u32) -> Result<()> {
        if resource as usize >= self.config.max_cells {
            return Err(Error::invalid_usage("native initial resource is absent"));
        }
        Ok(())
    }

    pub(super) fn resource_shadow(&mut self, resource: u32) -> Result<&mut ResourceShadow> {
        let index = resource as usize;
        if self.by_resource.values.len() <= index {
            let capacity = (index + 1).max(self.by_resource.values.capacity().saturating_mul(2)).min(self.config.max_cells);
            let total = self.known_cpu_bytes() - self.by_resource.bytes();
            self.by_resource.grow(capacity, total, RangeStateConfig { max_cells: self.config.max_cells, max_cpu_bytes: self.config.max_cpu_bytes }, &self.budget)?;
            self.by_resource.values.resize_with(index + 1, Default::default);
        }
        Ok(&mut self.by_resource.values[index])
    }

    // Inline adjacency can spill geometrically. Reserve a conservative bound
    // from the already admitted flat row capacities before any spill occurs.
    fn reserve_index_spill(&mut self, rows: usize) -> Result<()> {
        let needed = rows.saturating_mul(4 * size_of::<u32>());
        if needed <= self.index_spill_bytes { return Ok(()); }
        let bytes = needed.max(self.index_spill_bytes.saturating_mul(2));
        if bytes <= self.index_spill_bytes { return Ok(()); }
        if self.known_cpu_bytes().saturating_sub(self.index_spill_bytes).saturating_add(bytes) > self.config.max_cpu_bytes {
            return Err(Error::exhausted("native synchronization index capacity exceeded"));
        }
        self._index_spill_memory.resize(bytes as u64)?;
        self.index_spill_bytes = bytes;
        Ok(())
    }

    pub fn add_initial(&mut self, resource: u32, cells: &[RangeCell]) -> Result<()> {
        self.check_resource(resource)?;
        for &cell in cells {
            let slot = self.bound.initial.values.len() as u32;
            self.push_initial(InitialState { resource, cell })?;
            self.push_shadow(ShadowCell::new(resource, cell, slot))?;
        }
        Ok(())
    }

    /// Declare that `next` reuses the backing memory of `previous` in this
    /// frame. Call after [`Self::add_initial`] and before [`Self::bind`].
    pub fn add_alias(&mut self, previous: u32, next: u32) -> Result<()> {
        if previous == next {
            return Err(Error::invalid_usage("a resource cannot alias itself"));
        }
        self.check_resource(previous)?;
        self.check_resource(next)?;
        self.reserve_index_spill(self.bound.aliases.values.len() + self.shadow.values.len() + 1)?;
        self.push_alias(AliasEdge { previous, next })?;
        self.resource_shadow(next)?.alias_flags |= 1;
        self.resource_shadow(next)?.alias_previous.push(previous);
        self.resource_shadow(previous)?.alias_flags |= 2;
        Ok(())
    }

    /// First touch of an alias member: its contents are discarded and every
    /// access to the members it reuses becomes its synthetic previous writer.
    pub(super) fn discard_alias_contents(&self, mut current: ShadowCell) -> ShadowCell {
        let order = &self.bound.order.values;
        let later = |a, b| Self::later(order, a, b);
        let mut writer = current.writer;
        writer.merge(current.readers, later);
        if let Some(entry) = self.by_resource.values.get(current.resource as usize) {
            for &previous in &entry.alias_previous {
                let Some(other) = self.by_resource.values.get(previous as usize) else {
                    continue;
                };
                for &index in &other.cells {
                    let cell = &self.shadow.values[index as usize];
                    writer.merge(cell.writer, later);
                    writer.merge(cell.readers, later);
                }
            }
        }
        current.writer = writer;
        current.readers = PendingFrontier::EMPTY;
        current.layout = vk::ImageLayout::UNDEFINED;
        current.layout_known = true;
        current
    }

    /// An image range with known UNDEFINED layout holds no defined contents.
    pub(super) fn contents_discarded(cell: &ShadowCell) -> bool {
        matches!(cell.range, NativeRange::Image { .. })
            && cell.layout_known
            && cell.layout == vk::ImageLayout::UNDEFINED
    }
}

impl SynchronizationPlanner {
    pub(super) fn push_batch(&mut self, value: BoundBatch) -> Result<usize> {
        let index = self.bound.batches.values.len();
        self.push_batch_row(value)?;
        Ok(index)
    }
    push!(push_initial, initial, InitialState, max_cells);
    push!(push_alias, aliases, AliasEdge, max_cells);
    push!(push_batch_row, batches, BoundBatch, max_batches);
    push!(push_order, order, usize, max_batches);
    push!(push_transition, transitions, BoundTransition, max_rows);
    push!(push_patch, patches, BoundPatch, max_rows);
    push!(push_wait, waits, BoundWait, max_rows);

    pub(super) fn push_patch_slot(&mut self, slot: u32, value: BoundPatch) -> Result<()> {
        let needed = self.patch_slots.values.len() + 1;
        if self.patch_slots.values.capacity() < needed {
            let capacity = needed.max(self.patch_slots.values.capacity().saturating_mul(2)).min(self.config.max_rows);
            let total = self.known_cpu_bytes() - self.patch_slots.bytes();
            self.patch_slots.grow(capacity, total, RangeStateConfig { max_cells: self.config.max_rows, max_cpu_bytes: self.config.max_cpu_bytes }, &self.budget)?;
        }
        self.push_patch(value)?;
        self.patch_slots.values.push(slot);
        Ok(())
    }

    pub(super) fn push_shadow(&mut self, value: ShadowCell) -> Result<()> {
        let target = &mut self.shadow;
        let config = RangeStateConfig {
            max_cells: self.config.max_cells,
            max_cpu_bytes: self.config.max_cpu_bytes,
        };
        let needed = target
            .values
            .len()
            .checked_add(1)
            .ok_or(Error::OutOfHostMemory)?;
        if needed > config.max_cells {
            return Err(Error::Timeout);
        }
        let capacity = needed
            .max(target.values.capacity().saturating_mul(2))
            .min(config.max_cells);
        if needed > target.values.capacity() {
            let own = target.bytes();
            let total = self.known_cpu_bytes();
            self.shadow
                .grow(capacity, total - own, config, &self.budget)?;
        }
        self.reserve_index_spill(self.bound.aliases.values.len() + self.shadow.values.len() + 1)?;
        let target = &mut self.shadow;
        let index = target.values.len() as u32;
        let resource = value.resource;
        target.values.push(value);
        self.resource_shadow(resource)?.cells.push(index);
        Ok(())
    }
}

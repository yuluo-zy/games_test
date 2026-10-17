//! Reusable CPU projections of the actual physical resource registry.
use super::*;
use crate::{
    Error, Result,
    device::{BudgetCategory, DeviceGeneration},
    raw::allocator::{AllocatorStats, CpuMemoryReservation, shared_cpu_bytes},
};

#[derive(Clone, Copy, Debug)]
pub struct ManagedProjectionConfig {
    pub max_snapshots: usize,
    pub max_resources: usize,
    pub max_bytes: usize,
}
impl Default for ManagedProjectionConfig {
    fn default() -> Self {
        Self {
            max_snapshots: 32,
            max_resources: u16::MAX as usize,
            max_bytes: 32 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ManagedProjectionStats {
    pub snapshots: usize,
    pub bytes: usize,
    pub capacity_growths: u64,
    pub managed_allocations: u64,
    pub reused_rows: u64,
    pub capacity_stalls: u64,
}
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum PhysicalIdentity {
    Buffer(usize),
    Image(usize),
}
struct Resolved {
    identity: PhysicalIdentity,
    physical: u16,
    owned: bool,
    descriptor: Descriptor,
    resource: super::super::vulkan::GpuResource,
}
type PhysicalIndex = (PhysicalIdentity, u16);
#[derive(Clone)]
enum ManagedRow {
    Buffer(ManagedBuffer),
    Image(ManagedTexture),
}
fn managed(resource: &RegistryResource<ManagedTexture, ManagedBuffer, ()>) -> Option<ManagedRow> {
    match resource {
        RegistryResource::OwnedBuffer(value) | RegistryResource::UnownedBuffer(value) => {
            Some(ManagedRow::Buffer(value.handle.clone()))
        }
        RegistryResource::OwnedTexture(value) | RegistryResource::UnownedTexture(value) => {
            Some(ManagedRow::Image(value.handle.clone()))
        }
        _ => None,
    }
}
fn clear_registry(registry: &mut ManagedBindingRegistry) {
    registry.resources.clear();
    registry.resource_initial_access.clear();
    registry.exported_logical_handles.clear();
    registry.backing_resources.clear();
}
/// This ticket includes both registry buffers, lookup/scratch backing, and the
/// Arc payload/header. The real vectors precede the ticket in destruction order.
pub(super) struct ProjectionState {
    staging: ManagedBindingRegistry,
    index: Vec<PhysicalIndex>,
    next_index: Vec<PhysicalIndex>,
    resolved: Vec<Resolved>,
    bytes: usize,
    growths: u64,
    allocations: u64,
    reused: u64,
    memory: CpuMemoryReservation,
}
impl ProjectionState {
    fn new(budget: &Arc<AllocatorStats>) -> Result<Self> {
        let bytes = shared_cpu_bytes::<ManagedPassResources>()?;
        Ok(Self {
            staging: Default::default(),
            index: Vec::new(),
            next_index: Vec::new(),
            resolved: Vec::new(),
            bytes: usize::try_from(bytes).map_err(|_| Error::OutOfHostMemory)?,
            growths: 0,
            allocations: 0,
            reused: 0,
            memory: budget.reserve_cpu_bytes(bytes, BudgetCategory::Live)?,
        })
    }
    fn clear(&mut self, current: &mut ManagedBindingRegistry) {
        self.index.clear();
        self.next_index.clear();
        self.resolved.clear();
        clear_registry(current);
        clear_registry(&mut self.staging);
        self.memory.reclassify(BudgetCategory::Cache);
    }
}
/// Grow using a replacement allocation. Both backings are charged until the
/// actual old Vec has been freed; any rejected growth leaves it untouched.
fn reserve<T>(
    values: &mut Vec<T>,
    needed: usize,
    maximum: usize,
    state: &mut Reservation<'_>,
) -> Result<()> {
    if needed > maximum {
        return Err(Error::Timeout);
    }
    if values.capacity() >= needed {
        return Ok(());
    }
    let capacity = needed.max(values.capacity().saturating_mul(2)).min(maximum);
    let bytes = |capacity: usize| {
        capacity
            .checked_mul(std::mem::size_of::<T>())
            .ok_or(Error::OutOfHostMemory)
    };
    let previous = *state.bytes;
    let peak = previous
        .checked_add(bytes(capacity)?)
        .ok_or(Error::OutOfHostMemory)?;
    if peak > state.limit {
        return Err(Error::Timeout);
    }
    state.memory.resize(peak as u64)?;
    let mut replacement = Vec::new();
    if replacement.try_reserve_exact(capacity).is_err() {
        state
            .memory
            .resize(previous as u64)
            .expect("shrinking host reservation");
        return Err(Error::OutOfHostMemory);
    }
    let actual = bytes(replacement.capacity())?;
    let observed_peak = previous.checked_add(actual).ok_or(Error::OutOfHostMemory)?;
    let admitted = if observed_peak > state.limit || replacement.capacity() > maximum {
        Err(Error::Timeout)
    } else {
        state.memory.resize(observed_peak as u64)
    };
    if let Err(error) = admitted {
        drop(replacement);
        state
            .memory
            .resize(previous as u64)
            .expect("shrinking host reservation");
        return Err(error);
    }
    let old_bytes = bytes(values.capacity())?;
    replacement.append(values);
    let old = std::mem::replace(values, replacement);
    drop(old);
    *state.bytes = previous - old_bytes + actual;
    state
        .memory
        .resize(*state.bytes as u64)
        .expect("shrinking host reservation");
    *state.growths += 1;
    Ok(())
}
struct Reservation<'a> {
    memory: &'a mut CpuMemoryReservation,
    bytes: &'a mut usize,
    growths: &'a mut u64,
    limit: usize,
}
impl ManagedPassResources {
    fn empty(stores: &ManagedResources, budget: &Arc<AllocatorStats>) -> Result<Self> {
        Ok(Self {
            registry: Default::default(),
            buffers: stores.buffers.clone(),
            textures: stores.textures.clone(),
            projection: ProjectionState::new(budget)?,
        })
    }
    fn refill<Other>(
        &mut self,
        device: &Arc<Device>,
        source: &ResourceRegistry<u64, u64, Other>,
        config: ManagedProjectionConfig,
        limit: usize,
        mut resolve: impl FnMut(u64) -> Option<super::super::vulkan::GpuResource>,
    ) -> Result<()> {
        let result = self.refill_inner(device, source, config, limit, &mut resolve);
        self.projection.resolved.clear();
        if result.is_err() {
            self.projection.next_index.clear();
            clear_registry(&mut self.projection.staging);
            self.projection.memory.reclassify(BudgetCategory::Cache);
        }
        result
    }
    fn refill_inner<Other>(
        &mut self,
        device: &Arc<Device>,
        source: &ResourceRegistry<u64, u64, Other>,
        config: ManagedProjectionConfig,
        limit: usize,
        resolve: &mut impl FnMut(u64) -> Option<super::super::vulkan::GpuResource>,
    ) -> Result<()> {
        if source.resources.len() != source.resource_initial_access.len() {
            return Err(Error::invalid_usage(
                "resource projection has mismatched logical tables",
            ));
        }
        let logical = source.resources.len();
        let physical = source.backing_resources.len();
        let exports = source.exported_logical_handles.len();
        if [logical, physical, exports]
            .into_iter()
            .any(|len| len > config.max_resources)
        {
            return Err(Error::Timeout);
        }
        let state = &mut self.projection;
        state.memory.reclassify(BudgetCategory::Live);
        let mut reservation = Reservation {
            memory: &mut state.memory,
            bytes: &mut state.bytes,
            growths: &mut state.growths,
            limit,
        };
        reserve(
            &mut state.staging.resources,
            logical,
            config.max_resources,
            &mut reservation,
        )?;
        reserve(
            &mut state.staging.resource_initial_access,
            logical,
            config.max_resources,
            &mut reservation,
        )?;
        reserve(
            &mut state.staging.exported_logical_handles,
            exports,
            config.max_resources,
            &mut reservation,
        )?;
        reserve(
            &mut state.staging.backing_resources,
            physical,
            config.max_resources,
            &mut reservation,
        )?;
        reserve(
            &mut state.next_index,
            physical,
            config.max_resources,
            &mut reservation,
        )?;
        reserve(
            &mut state.resolved,
            physical,
            config.max_resources,
            &mut reservation,
        )?;
        state.next_index.clear();
        state.resolved.clear();
        clear_registry(&mut state.staging);
        for (index, resource) in source.backing_resources.iter().enumerate() {
            let (token, owned, descriptor) = match resource {
                RegistryResource::OwnedBuffer(value) => {
                    (value.handle, true, Descriptor::Buffer(value.desc))
                }
                RegistryResource::UnownedBuffer(value) => {
                    (value.handle, false, Descriptor::Buffer(value.desc))
                }
                RegistryResource::OwnedTexture(value) => {
                    (value.handle, true, Descriptor::Image(value.desc))
                }
                RegistryResource::UnownedTexture(value) => {
                    (value.handle, false, Descriptor::Image(value.desc))
                }
                RegistryResource::Empty => continue,
                RegistryResource::Other(_) => {
                    return Err(Error::invalid_usage("unsupported projected resource type"));
                }
            };
            let resource = resolve(token)
                .ok_or_else(|| Error::invalid_usage("projection physical resource is missing"))?;
            let identity = match (&resource, descriptor) {
                (super::super::vulkan::GpuResource::Buffer(raw), Descriptor::Buffer(_))
                    if Arc::ptr_eq(device, &raw.device) =>
                {
                    PhysicalIdentity::Buffer(Arc::as_ptr(raw) as usize)
                }
                (super::super::vulkan::GpuResource::Image(raw), Descriptor::Image(_))
                    if Arc::ptr_eq(device, &raw.device) =>
                {
                    PhysicalIdentity::Image(Arc::as_ptr(raw) as usize)
                }
                _ => {
                    return Err(Error::invalid_usage(
                        "projected resource has wrong type or device",
                    ));
                }
            };
            state.resolved.push(Resolved {
                identity,
                physical: index as u16,
                owned,
                descriptor,
                resource,
            });
        }
        state.resolved.sort_unstable_by_key(|row| row.identity);
        state.staging.resources.extend_from_slice(&source.resources);
        state
            .staging
            .resource_initial_access
            .extend_from_slice(&source.resource_initial_access);
        state
            .staging
            .exported_logical_handles
            .extend_from_slice(&source.exported_logical_handles);
        state
            .staging
            .backing_resources
            .resize_with(physical, || RegistryResource::Empty);
        let mut preceding: Option<PhysicalIndex> = None;
        for row in &state.resolved {
            let handle = if let Some((_, index)) =
                preceding.filter(|(identity, _)| *identity == row.identity)
            {
                state.reused += 1;
                managed(&state.staging.backing_resources[index as usize])
                    .expect("resolved preceding row")
            } else if let Ok(index) = state
                .index
                .binary_search_by_key(&row.identity, |(identity, _)| *identity)
            {
                state.reused += 1;
                managed(&self.registry.backing_resources[state.index[index].1 as usize])
                    .expect("indexed physical row")
            } else {
                let created = match &row.resource {
                    super::super::vulkan::GpuResource::Buffer(raw) => {
                        ManagedRow::Buffer(self.buffers.allocate_shared(raw.clone())?)
                    }
                    super::super::vulkan::GpuResource::Image(raw) => {
                        ManagedRow::Image(self.textures.allocate_shared(raw.clone())?)
                    }
                };
                state.allocations += 1;
                created
            };
            if preceding.is_none_or(|(identity, _)| identity != row.identity) {
                state.next_index.push((row.identity, row.physical));
                preceding = Some((row.identity, row.physical));
            }
            state.staging.backing_resources[row.physical as usize] =
                match (handle, row.descriptor, row.owned) {
                    (ManagedRow::Buffer(handle), Descriptor::Buffer(desc), true) => {
                        RegistryResource::OwnedBuffer(RegistryBuffer { handle, desc })
                    }
                    (ManagedRow::Buffer(handle), Descriptor::Buffer(desc), false) => {
                        RegistryResource::UnownedBuffer(RegistryBuffer { handle, desc })
                    }
                    (ManagedRow::Image(handle), Descriptor::Image(desc), true) => {
                        RegistryResource::OwnedTexture(RegistryTexture { handle, desc })
                    }
                    (ManagedRow::Image(handle), Descriptor::Image(desc), false) => {
                        RegistryResource::UnownedTexture(RegistryTexture { handle, desc })
                    }
                    _ => {
                        return Err(Error::invalid_usage(
                            "projection identity changed its resource type",
                        ));
                    }
                };
        }
        std::mem::swap(&mut self.registry, &mut state.staging);
        std::mem::swap(&mut state.index, &mut state.next_index);
        state.next_index.clear();
        clear_registry(&mut state.staging);
        Ok(())
    }
}
struct Slot {
    value: Arc<ManagedPassResources>,
    last_used: u64,
}
pub(crate) struct ManagedProjectionPool {
    slots: Vec<Slot>,
    config: ManagedProjectionConfig,
    generation: DeviceGeneration,
    budget: Arc<AllocatorStats>,
    metadata_bytes: usize,
    clock: u64,
    stalls: u64,
    retired_growths: u64,
    retired_allocations: u64,
    retired_reused: u64,
    _memory: CpuMemoryReservation,
}
impl ManagedProjectionPool {
    pub(crate) fn new(
        generation: DeviceGeneration,
        budget: Arc<AllocatorStats>,
        config: ManagedProjectionConfig,
    ) -> Result<Self> {
        if !(1..=256).contains(&config.max_snapshots)
            || !(1..=u16::MAX as usize).contains(&config.max_resources)
        {
            return Err(Error::invalid_usage(
                "invalid projection snapshot/resource limit",
            ));
        }
        let wanted = config
            .max_snapshots
            .checked_mul(std::mem::size_of::<Slot>())
            .ok_or(Error::OutOfHostMemory)?;
        if config.max_bytes <= wanted + shared_cpu_bytes::<ManagedPassResources>()? as usize {
            return Err(Error::invalid_usage(
                "projection byte limit cannot hold a snapshot",
            ));
        }
        let mut memory = budget.reserve_cpu_bytes(wanted as u64, BudgetCategory::Cache)?;
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(config.max_snapshots)
            .map_err(|_| Error::OutOfHostMemory)?;
        let metadata_bytes = slots
            .capacity()
            .checked_mul(std::mem::size_of::<Slot>())
            .ok_or(Error::OutOfHostMemory)?;
        memory.resize(metadata_bytes as u64)?;
        Ok(Self {
            slots,
            config,
            generation,
            budget,
            metadata_bytes,
            clock: 0,
            stalls: 0,
            retired_growths: 0,
            retired_allocations: 0,
            retired_reused: 0,
            _memory: memory,
        })
    }
    pub(crate) fn stats(&self) -> ManagedProjectionStats {
        self.slots.iter().fold(
            ManagedProjectionStats {
                snapshots: self.slots.len(),
                bytes: self.metadata_bytes,
                capacity_stalls: self.stalls,
                capacity_growths: self.retired_growths,
                managed_allocations: self.retired_allocations,
                reused_rows: self.retired_reused,
            },
            |mut stats, slot| {
                stats.bytes += slot.value.projection.bytes;
                stats.capacity_growths += slot.value.projection.growths;
                stats.managed_allocations += slot.value.projection.allocations;
                stats.reused_rows += slot.value.projection.reused;
                stats
            },
        )
    }
    fn remove_slot(&mut self, index: usize) {
        let slot = self.slots.remove(index);
        self.retired_growths += slot.value.projection.growths;
        self.retired_allocations += slot.value.projection.allocations;
        self.retired_reused += slot.value.projection.reused;
        drop(slot);
    }
    pub(crate) fn trim_idle(&mut self) {
        for index in (0..self.slots.len()).rev() {
            if Arc::get_mut(&mut self.slots[index].value).is_some() {
                self.remove_slot(index);
            }
        }
    }
    pub(crate) fn clear_idle(&mut self) {
        for slot in &mut self.slots {
            if let Some(value) = Arc::get_mut(&mut slot.value) {
                value.projection.clear(&mut value.registry);
            }
        }
    }
    pub(crate) fn checkout<Other>(
        &mut self,
        device: &Arc<Device>,
        source: &ResourceRegistry<u64, u64, Other>,
        stores: &ManagedResources,
        resolve: impl FnMut(u64) -> Option<super::super::vulkan::GpuResource>,
    ) -> Result<Arc<ManagedPassResources>> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if device.generation() != self.generation {
            return Err(Error::invalid_usage(
                "projection pool belongs to another device",
            ));
        }
        let selected = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| {
                Arc::strong_count(&slot.value) == 1 && Arc::weak_count(&slot.value) == 0
            })
            .max_by_key(|(_, slot)| slot.last_used)
            .map(|(index, _)| index);
        let mut index = if let Some(index) = selected {
            index
        } else {
            if self.slots.len() >= self.config.max_snapshots {
                self.stalls += 1;
                return Err(Error::Timeout);
            }
            let bytes = self
                .stats()
                .bytes
                .checked_add(shared_cpu_bytes::<ManagedPassResources>()? as usize)
                .ok_or(Error::OutOfHostMemory)?;
            if bytes > self.config.max_bytes {
                self.stalls += 1;
                return Err(Error::Timeout);
            }
            self.slots.push(Slot {
                value: Arc::new(ManagedPassResources::empty(stores, &self.budget)?),
                last_used: 0,
            });
            self.slots.len() - 1
        };
        // Idle surplus snapshots are no longer needed after external owners
        // release them. Free their actual backing before admitting a new shape.
        for other in (0..self.slots.len()).rev() {
            if other != index && Arc::get_mut(&mut self.slots[other].value).is_some() {
                self.remove_slot(other);
                index -= usize::from(other < index);
            }
        }
        let others = self.metadata_bytes
            + self
                .slots
                .iter()
                .enumerate()
                .filter(|(other, _)| *other != index)
                .map(|(_, slot)| slot.value.projection.bytes)
                .sum::<usize>();
        let available = self.config.max_bytes.saturating_sub(others);
        let value = Arc::get_mut(&mut self.slots[index].value).ok_or(Error::Timeout)?;
        if value.buffers.identity() != stores.buffers.identity()
            || value.textures.identity() != stores.textures.identity()
        {
            return Err(Error::invalid_usage("projection stores changed"));
        }
        if let Err(error) = value.refill(device, source, self.config, available, resolve) {
            self.stalls += u64::from(matches!(error, Error::Timeout));
            return Err(error);
        }
        self.clock = self.clock.saturating_add(1);
        self.slots[index].last_used = self.clock;
        Ok(self.slots[index].value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_growth_charges_real_capacity_and_releases_its_replaced_backing() {
        let stats = Arc::new(AllocatorStats::default());
        let mut memory = stats.reserve_cpu_bytes(16, BudgetCategory::Live).unwrap();
        let mut bytes = 16;
        let mut growths = 0;
        let mut values = Vec::<u64>::new();
        for needed in [3, 7, 7] {
            let mut state = Reservation {
                memory: &mut memory,
                bytes: &mut bytes,
                growths: &mut growths,
                limit: 4096,
            };
            reserve(&mut values, needed, 16, &mut state).unwrap();
            assert_eq!(bytes, 16 + values.capacity() * size_of::<u64>());
            assert_eq!(stats.cpu_memory_usage().total(), bytes as u64);
            values.resize(needed, 9);
        }
        assert_eq!(growths, 2);
        assert!(values.iter().all(|value| *value == 9));
        drop(values);
        drop(memory);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn rejected_vector_growth_preserves_existing_storage_and_global_accounting() {
        let stats = Arc::new(AllocatorStats::default());
        let mut memory = stats.reserve_cpu_bytes(16, BudgetCategory::Cache).unwrap();
        let mut bytes = 16;
        let mut growths = 0;
        let mut values = Vec::<u64>::new();
        let mut state = Reservation {
            memory: &mut memory,
            bytes: &mut bytes,
            growths: &mut growths,
            limit: 4096,
        };
        reserve(&mut values, 2, 16, &mut state).unwrap();
        values.extend([11, 12]);
        let pointer = values.as_ptr();
        let old_usage = stats.cpu_memory_usage();
        stats.set_total_budget(old_usage.total() + 1).unwrap();
        assert!(matches!(
            reserve(&mut values, 8, 16, &mut state),
            Err(Error::OutOfHostMemory)
        ));
        assert_eq!(values, [11, 12]);
        assert_eq!(values.as_ptr(), pointer);
        assert_eq!(stats.cpu_memory_usage(), old_usage);
        assert_eq!(*state.growths, 1);
        assert!(matches!(
            reserve(&mut values, 17, 16, &mut state),
            Err(Error::Timeout)
        ));
        assert_eq!(stats.cpu_memory_usage(), old_usage);
    }
}

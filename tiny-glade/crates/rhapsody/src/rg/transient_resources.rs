//! Physical graph-local alias preparation. Alias sets (shared backings and the
//! resources bound to them) are cached per physical plan and lent to one frame
//! at a time, so distinct in-flight frame instances never overlap memory.
use super::{
    graph,
    plan::CompiledPlan,
    transient_memory::{self, MemoryRequirement, PhysicalMemoryPlan},
    vulkan::{GpuResource, ResourceSpec},
};
use crate::raw::{
    allocator::MemoryBlock,
    buffer,
    device::Device,
    texture,
    transient_backing::{SharedBacking, TransientBackingCounters},
};
use ash::vk;
use std::{
    collections::BTreeMap,
    sync::Arc,
};

/// Unaliased dependency storage reused across frames.
#[derive(Default)]
pub(crate) struct PreparedTransients {
    pub plan: PhysicalMemoryPlan,
    // Declared after the plan so the actual edge storage is freed before its credit.
    edge_memory: Option<crate::raw::allocator::CpuMemoryReservation>,
}
impl PreparedTransients {
    /// Preserve only the budgeted CPU edge storage. Physical candidates from a
    /// failed frame are dropped; published resources remain owned by that frame.
    pub(super) fn prepare_unaliased(
        &mut self,
        dependencies: &[(usize, usize)],
        budget: &Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<()> {
        let (edges, memory) = if self.edge_memory.is_some() {
            (
                std::mem::take(&mut self.plan.dependency_edges),
                self.edge_memory.take(),
            )
        } else {
            (Vec::new(), None)
        };
        *self = Self {
            plan: PhysicalMemoryPlan {
                dependency_edges: edges,
                ..Default::default()
            },
            edge_memory: memory,
            ..Default::default()
        };
        if dependencies.len() > self.plan.dependency_edges.capacity() {
            let bytes = std::alloc::Layout::array::<(usize, usize)>(dependencies.len())
                .map_err(|_| crate::Error::OutOfHostMemory)?
                .size();
            let mut memory =
                budget.reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Live)?;
            let mut replacement = Vec::new();
            replacement
                .try_reserve_exact(dependencies.len())
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            let actual = replacement
                .capacity()
                .checked_mul(std::mem::size_of::<(usize, usize)>())
                .ok_or(crate::Error::OutOfHostMemory)?;
            memory.resize(actual as u64)?;
            let old = std::mem::replace(&mut self.plan.dependency_edges, replacement);
            let old_memory = self.edge_memory.replace(memory);
            drop(old);
            drop(old_memory);
        }
        self.plan.dependency_edges.clear();
        self.plan.dependency_edges.extend_from_slice(dependencies);
        if let Some(memory) = &mut self.edge_memory {
            memory.reclassify(crate::device::BudgetCategory::Cache);
        }
        Ok(())
    }
    pub(super) fn trim_unaliased(&mut self) {
        if self.edge_memory.is_some() {
            let old = std::mem::take(&mut self.plan.dependency_edges);
            drop(old);
            self.edge_memory.take();
        }
    }
}
enum Handle {
    Buffer(vk::Buffer, buffer::BufferDesc),
    Image(vk::Image, texture::TextureDesc),
}
struct Unbound {
    device: Arc<Device>,
    handle: Option<Handle>,
    storage: Option<crate::resource::tracked::ResourceStorage>,
    fact: MemoryRequirement,
}
impl Drop for Unbound {
    fn drop(&mut self) {
        unsafe {
            match self.handle.take() {
                Some(Handle::Buffer(handle, _)) => self.device.device.destroy_buffer(handle, None),
                Some(Handle::Image(handle, _)) => self.device.device.destroy_image(handle, None),
                None => {}
            }
        }
    }
}
impl Unbound {
    fn create(
        device: Arc<Device>,
        id: u32,
        spec: ResourceSpec,
        uses: Vec<usize>,
    ) -> crate::Result<Self> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.allocator_stats)?;
        let sharing = spec.sharing();
        sharing.validate_device(&device)?;
        let (handle, requirements, dedicated, compatibility, flags, memory) = match spec {
            ResourceSpec::Buffer(desc) | ResourceSpec::ConcurrentBuffer(desc, _) => {
                if desc.size == 0 {
                    return Err(crate::Error::invalid_usage("zero-sized transient buffer"));
                }
                let address = desc
                    .usage
                    .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS);
                if address && !device.capabilities.buffer_device_address {
                    return Err(crate::Error::unsupported(
                        "buffer device address is disabled",
                    ));
                }
                let info = vk::BufferCreateInfo::default()
                    .size(desc.size)
                    .usage(desc.usage)
                    .sharing_mode(if sharing.is_concurrent() {
                        vk::SharingMode::CONCURRENT
                    } else {
                        vk::SharingMode::EXCLUSIVE
                    })
                    .queue_family_indices(sharing.families());
                let memory = device.allocator_stats.reserve_cpu_bytes(
                    crate::raw::allocator::shared_cpu_bytes::<buffer::Buffer>()?,
                    crate::device::BudgetCategory::Live,
                )?;
                let handle = crate::raw::error::observe_vulkan(&device, unsafe {
                    device.device.create_buffer(&info, None)
                })?;
                let mut dedicated = vk::MemoryDedicatedRequirements::default();
                let mut requirements = vk::MemoryRequirements2::default().push_next(&mut dedicated);
                unsafe {
                    device.device.get_buffer_memory_requirements2(
                        &vk::BufferMemoryRequirementsInfo2::default().buffer(handle),
                        &mut requirements,
                    );
                }
                let mut req = requirements.memory_requirements;
                req.alignment =
                    buffer::buffer_alignment(req.alignment, desc.alignment, desc.usage.as_raw());
                (
                    Handle::Buffer(handle, desc),
                    req,
                    dedicated,
                    0,
                    if address {
                        vk::MemoryAllocateFlags::DEVICE_ADDRESS
                    } else {
                        vk::MemoryAllocateFlags::empty()
                    },
                    memory,
                )
            }
            ResourceSpec::Image(desc) | ResourceSpec::ConcurrentImage(desc, _) => {
                desc.validate()?;
                let info = texture::get_image_create_info(&desc)?.flags(
                    desc.flags
                        | vk::ImageCreateFlags::ALIAS
                        | if desc.kind.is_cube() {
                            vk::ImageCreateFlags::CUBE_COMPATIBLE
                        } else {
                            vk::ImageCreateFlags::empty()
                        },
                );
                let info = info
                    .sharing_mode(if sharing.is_concurrent() {
                        vk::SharingMode::CONCURRENT
                    } else {
                        vk::SharingMode::EXCLUSIVE
                    })
                    .queue_family_indices(sharing.families());
                let memory = device.allocator_stats.reserve_cpu_bytes(
                    crate::raw::allocator::shared_cpu_bytes::<texture::Texture>()?,
                    crate::device::BudgetCategory::Live,
                )?;
                let handle = crate::raw::error::observe_vulkan(&device, unsafe {
                    device.device.create_image(&info, None)
                })?;
                let mut dedicated = vk::MemoryDedicatedRequirements::default();
                let mut requirements = vk::MemoryRequirements2::default().push_next(&mut dedicated);
                unsafe {
                    device.device.get_image_memory_requirements2(
                        &vk::ImageMemoryRequirementsInfo2::default().image(handle),
                        &mut requirements,
                    );
                }
                (
                    Handle::Image(handle, desc),
                    requirements.memory_requirements,
                    dedicated,
                    1,
                    vk::MemoryAllocateFlags::empty(),
                    memory,
                )
            }
        };
        Ok(Self {
            device,
            handle: Some(handle),
            storage: Some(crate::resource::tracked::ResourceStorage {
                object: memory,
                synchronization: sync_storage,
            }),
            fact: MemoryRequirement {
                sharing,
                resource: id,
                compatibility,
                size: requirements.size,
                alignment: requirements.alignment,
                memory_type_bits: requirements.memory_type_bits,
                allocation_flags: flags.as_raw(),
                requires_dedicated: dedicated.requires_dedicated_allocation != 0,
                prefers_dedicated: dedicated.prefers_dedicated_allocation != 0,
                uses,
            },
        })
    }
    fn bind(mut self, backing: Arc<SharedBacking>) -> crate::Result<GpuResource> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let range = match self.handle.as_ref().expect("unbound resource") {
            Handle::Image(_, desc) => Some(texture::Texture::ownership_range(desc)?),
            Handle::Buffer(_, _) => None,
        };
        let allocation = MemoryBlock::from_shared(self.device.clone(), backing);
        unsafe {
            match self.handle.as_ref().expect("unbound resource") {
                Handle::Buffer(handle, _) => crate::raw::error::observe_vulkan(
                    &self.device,
                    self.device.device.bind_buffer_memory(
                        *handle,
                        allocation.handle,
                        allocation.offset,
                    ),
                )?,
                Handle::Image(handle, _) => crate::raw::error::observe_vulkan(
                    &self.device,
                    self.device.device.bind_image_memory(
                        *handle,
                        allocation.handle,
                        allocation.offset,
                    ),
                )?,
            }
            Ok(match self.handle.take().expect("bound resource") {
                Handle::Buffer(handle, desc) => {
                    GpuResource::Buffer(Arc::new(buffer::Buffer::from_bound_transient(
                        self.device.clone(),
                        desc,
                        handle,
                        allocation,
                        self.fact.sharing,
                        self.storage.take().expect("unbound metadata reservation"),
                    )))
                }
                Handle::Image(handle, desc) => {
                    GpuResource::Image(Arc::new(texture::Texture::from_bound_transient(
                        self.device.clone(),
                        desc,
                        handle,
                        allocation,
                        self.fact.sharing,
                        range.expect("validated image range"),
                        self.storage.take().expect("unbound metadata reservation"),
                    )))
                }
            })
        }
    }
}

/// Identity of one physical alias plan: the aliasing candidates (logical
/// index, creation parameters including derived usage, sharing, scheduled
/// uses) and the dependency proof. Equal keys produce equal memory plans on one
/// device, so the plan and its bound members can be reused.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct AliasKey {
    entries: Vec<KeyEntry>,
    uses: Vec<usize>,
    edges: Vec<(usize, usize)>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpecKey {
    Buffer {
        size: u64,
        usage: vk::BufferUsageFlags,
        alignment: u16,
    },
    Image(texture::TextureDesc),
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct KeyEntry {
    index: u32,
    spec: SpecKey,
    sharing: crate::resource::ResourceSharing,
    uses: std::ops::Range<usize>,
}
impl AliasKey {
    fn clear(&mut self) {
        self.entries.clear();
        self.uses.clear();
        self.edges.clear();
    }
}
/// Where a lent alias member returns once its frame retires.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AliasOrigin {
    set: u64,
    member: usize,
}
/// One physical alias plan with its shared backings and bound members. A set
/// is lent to exactly one frame at a time: members come back only after that
/// frame retired on the GPU (or never reached it), so memory still in flight is
/// never handed to another frame.
struct AliasSet {
    id: u64,
    key: AliasKey,
    plan: Arc<PhysicalMemoryPlan>,
    edges: Vec<(u32, u32)>,
    members: Vec<(u32, Option<GpuResource>)>,
    lent: usize,
    last_used: u64,
}
/// A frame's view of a lent alias set.
pub(crate) struct LentAliases {
    pub set: u64,
    pub resources: BTreeMap<String, (GpuResource, AliasOrigin)>,
}
/// Cache of physical alias plans keyed by [`AliasKey`]. A cache hit performs no
/// Vulkan object creation, memory query or allocation.
#[derive(Default)]
pub(crate) struct AliasCache {
    sets: Vec<AliasSet>,
    next_id: u64,
    clock: u64,
    key: AliasKey,
    pairs: Vec<(u32, usize)>,
}
/// Retired sets kept for reuse (lent sets are never evicted). Two frames in
/// flight with one or two alternating graphs fit without reallocation.
const MAX_AVAILABLE_SETS: usize = 8;
impl AliasCache {
    pub(crate) fn plan(&self, set: u64) -> Option<&PhysicalMemoryPlan> {
        self.sets
            .iter()
            .find(|entry| entry.id == set)
            .map(|entry| &*entry.plan)
    }
    /// Alias edges `(previous, next)` by logical resource index, including the
    /// wrap-around edge from the last member back to the first: when a set is
    /// reused, its first member overwrites memory last used by the final member.
    pub(crate) fn edges(&self, set: u64) -> &[(u32, u32)] {
        self.sets
            .iter()
            .find(|entry| entry.id == set)
            .map_or(&[], |entry| &entry.edges)
    }
    /// Take back a member lent by [`Self::lend`]. The caller guarantees the GPU
    /// no longer uses it.
    pub(crate) fn give_back(&mut self, origin: AliasOrigin, resource: GpuResource) {
        if let Some(set) = self.sets.iter_mut().find(|set| set.id == origin.set)
            && let Some((_, slot)) = set.members.get_mut(origin.member)
            && slot.is_none()
        {
            *slot = Some(resource);
            set.lent -= 1;
        }
    }
    /// Drop every retired set (memory pressure).
    pub(crate) fn trim(&mut self) {
        self.sets.retain(|set| set.lent != 0);
    }
    fn build_key(
        &mut self,
        specifications: &BTreeMap<String, ResourceSpec>,
        plan: &CompiledPlan,
        graph: &graph::GraphDeclaration,
    ) {
        self.pairs.clear();
        for &pass in &plan.retained_passes {
            self.pairs.extend(
                plan.definition.passes[pass]
                    .accesses
                    .iter()
                    .map(|access| (access.resource_index, pass)),
            );
        }
        self.pairs.sort_unstable();
        self.pairs.dedup();
        let key = &mut self.key;
        key.clear();
        key.edges.extend_from_slice(&plan.dependency_edges);
        let mut cursor = 0;
        for (index, declaration) in graph.resources.iter().enumerate() {
            let first = cursor
                + self.pairs[cursor..].partition_point(|&(resource, _)| (resource as usize) < index);
            let last = first
                + self.pairs[first..].partition_point(|&(resource, _)| resource as usize == index);
            cursor = last;
            if first == last
                || !matches!(declaration.source, graph::OriginSource::Create)
                || plan.info.exported_indices.contains(&(index as u16))
            {
                continue;
            }
            let Some(spec) = specifications.get(&declaration.name).copied() else {
                continue;
            };
            let spec_key = match spec {
                ResourceSpec::Buffer(desc) | ResourceSpec::ConcurrentBuffer(desc, _) => {
                    if desc.memory_location != buffer::MemoryLocation::GpuOnly {
                        continue;
                    }
                    SpecKey::Buffer {
                        size: desc.size,
                        usage: desc.usage
                            | vk::BufferUsageFlags::from_raw(plan.info.buffer_usage_flags[index]),
                        alignment: desc.alignment,
                    }
                }
                ResourceSpec::Image(mut desc) | ResourceSpec::ConcurrentImage(mut desc, _) => {
                    // Sparse, protected, disjoint, external/modifier images need a
                    // different binding contract. Ordinary optimal images only.
                    let allowed = vk::ImageCreateFlags::CUBE_COMPATIBLE
                        | vk::ImageCreateFlags::MUTABLE_FORMAT
                        | vk::ImageCreateFlags::ALIAS;
                    if !(desc.flags & !allowed).is_empty() {
                        continue;
                    }
                    desc.usage |= vk::ImageUsageFlags::from_raw(plan.info.image_usage_flags[index]);
                    SpecKey::Image(desc)
                }
            };
            let start = key.uses.len();
            key.uses
                .extend(self.pairs[first..last].iter().map(|&(_, pass)| pass));
            key.entries.push(KeyEntry {
                index: index as u32,
                spec: spec_key,
                sharing: spec.sharing(),
                uses: start..key.uses.len(),
            });
        }
    }
    /// Lend the physical alias set for this frame, reusing a retired set with
    /// the same key or creating one. Returns `None` when nothing can alias.
    pub(crate) fn lend(
        &mut self,
        device: &Arc<Device>,
        specifications: &BTreeMap<String, ResourceSpec>,
        plan: &CompiledPlan,
        graph: &graph::GraphDeclaration,
        counters: Arc<TransientBackingCounters>,
    ) -> crate::Result<Option<LentAliases>> {
        self.clock += 1;
        self.build_key(specifications, plan, graph);
        if self.key.entries.is_empty() {
            return Ok(None);
        }
        let position = match self
            .sets
            .iter()
            .position(|set| set.lent == 0 && set.key == self.key)
        {
            Some(position) => position,
            None => {
                let set = self.create(device, plan, graph, counters)?;
                self.evict();
                self.sets.push(set);
                self.sets.len() - 1
            }
        };
        let set = &mut self.sets[position];
        set.last_used = self.clock;
        let mut resources = BTreeMap::new();
        for (member, (index, slot)) in set.members.iter_mut().enumerate() {
            let resource = slot.take().expect("retired alias set owns every member");
            resources.insert(
                graph.resources[*index as usize].name.clone(),
                (resource, AliasOrigin { set: set.id, member }),
            );
        }
        set.lent = set.members.len();
        Ok(Some(LentAliases {
            set: set.id,
            resources,
        }))
    }
    fn evict(&mut self) {
        while self.sets.iter().filter(|set| set.lent == 0).count() >= MAX_AVAILABLE_SETS {
            let oldest = self
                .sets
                .iter()
                .enumerate()
                .filter(|(_, set)| set.lent == 0)
                .min_by_key(|(_, set)| set.last_used)
                .map(|(position, _)| position)
                .expect("counted available set");
            self.sets.swap_remove(oldest);
        }
    }
    fn create(
        &mut self,
        device: &Arc<Device>,
        plan: &CompiledPlan,
        graph: &graph::GraphDeclaration,
        counters: Arc<TransientBackingCounters>,
    ) -> crate::Result<AliasSet> {
        let mut candidates = BTreeMap::new();
        for entry in &self.key.entries {
            let spec = match entry.spec {
                SpecKey::Buffer {
                    size,
                    usage,
                    alignment,
                } => {
                    let mut desc = buffer::BufferDesc::new_gpu_only(size, usage);
                    desc.alignment = alignment;
                    match entry.sharing {
                        crate::resource::ResourceSharing::Exclusive => ResourceSpec::Buffer(desc),
                        crate::resource::ResourceSharing::Concurrent(families) => {
                            ResourceSpec::ConcurrentBuffer(desc, families)
                        }
                    }
                }
                SpecKey::Image(desc) => match entry.sharing {
                    crate::resource::ResourceSharing::Exclusive => ResourceSpec::Image(desc),
                    crate::resource::ResourceSharing::Concurrent(families) => {
                        ResourceSpec::ConcurrentImage(desc, families)
                    }
                },
            };
            let uses = self.key.uses[entry.uses.clone()].to_vec();
            candidates.insert(
                entry.index,
                Unbound::create(device.clone(), entry.index, spec, uses)?,
            );
        }
        let facts = candidates
            .values()
            .map(|candidate| candidate.fact.clone())
            .collect::<Vec<_>>();
        let mut physical =
            transient_memory::plan_memory_requirements(&facts, &plan.dependency_edges)?;
        if super::validation_enabled() {
            super::transient_memory_check::check_memory_plan(&physical, &plan.dependency_edges)?;
        }
        let mut members = Vec::new();
        let mut edges = Vec::new();
        for backing in &mut physical.backings {
            if backing.resources.len() < 2 {
                continue;
            }
            let owner = SharedBacking::allocate(
                device,
                vk::MemoryRequirements {
                    size: backing.size,
                    alignment: backing.alignment,
                    memory_type_bits: backing.memory_type_bits,
                },
                vk::MemoryAllocateFlags::from_raw(backing.allocation_flags),
                candidates[&backing.resources[0]].fact.compatibility == 0,
                counters.clone(),
            )?;
            backing.allocation_id = Some(owner.accounting_id);
            backing.memory_type = Some(owner.memory_type);
            let mut order = backing.resources.clone();
            order.sort_by_key(|id| {
                *candidates[id]
                    .fact
                    .uses
                    .iter()
                    .min()
                    .expect("used resource")
            });
            for pair in order.windows(2) {
                edges.push((pair[0], pair[1]));
            }
            edges.push((*order.last().expect("shared backing"), order[0]));
            for id in order {
                let resource = candidates
                    .remove(&id)
                    .expect("planned candidate")
                    .bind(owner.clone())?;
                debug_assert!(
                    (id as usize) < graph.resources.len(),
                    "alias member is a declared resource"
                );
                members.push((id, Some(resource)));
            }
        }
        let id = self.next_id;
        self.next_id += 1;
        Ok(AliasSet {
            id,
            key: self.key.clone(),
            plan: Arc::new(physical),
            edges,
            members,
            lent: 0,
            last_used: self.clock,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_unaliased_edges_reuse_accounted_storage_and_trim() {
        let budget = Arc::new(crate::raw::allocator::AllocatorStats::default());
        let mut prepared = PreparedTransients::default();
        let edges = [(0, 1), (1, 2), (2, 3)];
        prepared.prepare_unaliased(&edges, &budget).unwrap();
        let address = prepared.plan.dependency_edges.as_ptr();
        let bytes = (prepared.plan.dependency_edges.capacity()
            * std::mem::size_of::<(usize, usize)>()) as u64;
        budget.set_total_budget(bytes).unwrap();
        for _ in 0..1000 {
            prepared.prepare_unaliased(&edges, &budget).unwrap();
            assert_eq!(prepared.plan.dependency_edges.as_ptr(), address);
            assert_eq!(prepared.plan.dependency_edges, edges);
            assert_eq!(budget.cpu_memory_usage().cache, bytes);
        }
        prepared.trim_unaliased();
        assert_eq!(prepared.plan.dependency_edges.capacity(), 0);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn unaliased_edge_growth_oom_preserves_the_existing_credit_and_plan() {
        let budget = Arc::new(crate::raw::allocator::AllocatorStats::default());
        let mut prepared = PreparedTransients::default();
        prepared.prepare_unaliased(&[(0, 1)], &budget).unwrap();
        let address = prepared.plan.dependency_edges.as_ptr();
        let bytes = budget.cpu_memory_usage().total();
        budget.set_total_budget(bytes).unwrap();
        assert!(matches!(
            prepared.prepare_unaliased(&[(0, 1), (1, 2)], &budget),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(prepared.plan.dependency_edges, [(0, 1)]);
        assert_eq!(prepared.plan.dependency_edges.as_ptr(), address);
        assert_eq!(budget.cpu_memory_usage().total(), bytes);
        drop(prepared);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
}

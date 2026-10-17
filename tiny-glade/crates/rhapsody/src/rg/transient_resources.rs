//! Physical graph-local alias preparation. Every invocation owns fresh backing
//! allocations, so distinct in-flight frame instances never overlap memory.
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
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Default)]
pub(crate) struct PreparedTransients {
    pub resources: BTreeMap<String, GpuResource>,
    pub switches: BTreeMap<String, Vec<u8>>,
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

pub(crate) fn prepare(
    device: &Arc<Device>,
    specifications: &BTreeMap<String, ResourceSpec>,
    plan: &CompiledPlan,
    graph: &graph::GraphDeclaration,
    counters: Arc<TransientBackingCounters>,
) -> crate::Result<PreparedTransients> {
    let mut candidates = BTreeMap::new();
    for (index, declaration) in graph.resources.iter().enumerate() {
        if !matches!(declaration.source, graph::OriginSource::Create)
            || plan.info.exported_indices.contains(&(index as u16))
        {
            continue;
        }
        let uses = plan
            .retained_passes
            .iter()
            .copied()
            .filter(|&pass| {
                plan.definition.passes[pass]
                    .accesses
                    .iter()
                    .any(|access| access.resource_index as usize == index)
            })
            .collect::<Vec<_>>();
        if uses.is_empty() {
            continue;
        }
        let Some(mut spec) = specifications.get(&declaration.name).copied() else {
            continue;
        };
        match &mut spec {
            ResourceSpec::Buffer(desc) | ResourceSpec::ConcurrentBuffer(desc, _) => {
                if desc.memory_location != buffer::MemoryLocation::GpuOnly {
                    continue;
                }
                desc.usage |= vk::BufferUsageFlags::from_raw(plan.info.buffer_usage_flags[index]);
            }
            ResourceSpec::Image(desc) | ResourceSpec::ConcurrentImage(desc, _) => {
                // Sparse, protected, disjoint, external/modifier images need a
                // different binding contract. Ordinary optimal images only.
                let allowed = vk::ImageCreateFlags::CUBE_COMPATIBLE
                    | vk::ImageCreateFlags::MUTABLE_FORMAT
                    | vk::ImageCreateFlags::ALIAS;
                if !(desc.flags & !allowed).is_empty() {
                    continue;
                }
                desc.usage |= vk::ImageUsageFlags::from_raw(plan.info.image_usage_flags[index]);
            }
        }
        candidates.insert(
            index as u32,
            Unbound::create(device.clone(), index as u32, spec, uses)?,
        );
    }
    let facts = candidates
        .values()
        .map(|candidate| candidate.fact.clone())
        .collect::<Vec<_>>();
    let physical = transient_memory::plan_memory_requirements(&facts, &plan.dependency_edges)?;
    super::transient_memory_check::check_memory_plan(&physical, &plan.dependency_edges)?;
    let mut result = PreparedTransients {
        plan: physical,
        ..Default::default()
    };
    for backing in &mut result.plan.backings {
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
        let mut members = backing.resources.clone();
        members.sort_by_key(|id| {
            *candidates[id]
                .fact
                .uses
                .iter()
                .min()
                .expect("used resource")
        });
        let mut previous = None;
        for id in members {
            let name = graph.resources[id as usize].name.clone();
            let resource = candidates
                .remove(&id)
                .expect("planned candidate")
                .bind(owner.clone())?;
            if let Some(previous) = previous {
                let codes = plan
                    .retained_passes
                    .iter()
                    .flat_map(|&pass| plan.definition.passes[pass].accesses.iter())
                    .filter(|access| access.resource_index == previous)
                    .map(|access| access.access_code)
                    .collect::<BTreeSet<_>>();
                result
                    .switches
                    .insert(name.clone(), codes.into_iter().collect());
            }
            previous = Some(id);
            result.resources.insert(name, resource);
        }
    }
    Ok(result)
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

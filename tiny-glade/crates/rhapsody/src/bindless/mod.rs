//! A renderer-owned update-after-bind descriptor table. A lease is immutable:
//! changing a resource allocates a different slot. Its slot returns to the free
//! list only after every registered GPU use completes. See the Vulkan
//! [descriptor binding rules](https://docs.vulkan.org/spec/latest/chapters/descriptorsets.html).
use crate::profile::{RecordCount, RecordPhase, RecordSpan, record_count};
use crate::{Error, Result, device::{Device, FeatureSet}};
use crate::tracking::GpuUse;
use crate::command::descriptor_set::{LayoutBinding, LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSet, VulkanDescriptorSetLayout};
use ash::vk;
use parking_lot::Mutex;
use std::sync::Arc;
mod parameters;
pub use parameters::{
    BindlessParameters, BindlessResource, BindlessShaderPackage,
    PreparedBindlessParameters,
};
mod kind;
mod slots;
mod residency;
mod lease;
mod stats;
mod descriptors;
mod bind;
mod pages;
pub use kind::{DescriptorKind, BindlessConfig, BindlessIndex};
use slots::{Slot, Slots};
use residency::ResidentLeases;
pub use lease::BindlessLease;
pub(crate) use lease::Owner;
use lease::LeaseSlot;
pub use stats::{BindlessTableStats, BindlessStats, BindlessOccupancy, BindlessTableOccupancy};
pub use pages::{BindlessPagePool, GroupReservation, GroupPreparationError, GroupPreparationResult, BindlessPageCursor};

pub struct BindlessHeap {
    pub layout: Arc<VulkanDescriptorSetLayout>,
    pub(crate) set: Arc<VulkanDescriptorSet>,
    pub(crate) device: Arc<Device>,
    slots: Mutex<Slots>,
    /// Reuse descriptors while a prepared group or submitted command owns the
    /// lease. The index cache must not keep its own descriptors/resources alive.
    resident: Mutex<Option<ResidentLeases>>,
    // Covers slot storage and one live lease payload per slot. Leases retain
    // this page, so the capacity charge survives every CPU/GPU lease owner.
    // Declared after slots: release credit only after the actual CPU buffers.
    _slot_memory: crate::accounting::CpuMemoryReservation,
}
impl GpuUse for BindlessHeap {
    fn visit_uses(&self, visit: &mut crate::tracking::UseVisitor<'_>) -> Result<()> {
        self.set.visit_uses(visit)
    }
}
impl BindlessHeap {
    pub fn new(device: Arc<Device>, config: BindlessConfig) -> Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let counts = config.counts();
        if counts.contains(&0)
            || counts
                .iter()
                .try_fold(0u32, |sum, count| sum.checked_add(*count))
                .is_none_or(|total| total > 1 << 20)
        {
            return Err(Error::invalid_usage(
                "bindless table counts must be nonzero and bounded",
            ));
        }
        let required = crate::device::policy::BINDLESS_REQUIRED;
        device.caps().require_all_enabled(
            required
                .into_iter()
                .fold(FeatureSet::EMPTY, |set, feature| set.with(feature)),
        )?;
        let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
        unsafe {
            device
                .core
                .instance
                .instance
                .get_physical_device_properties2(device.core.physical, &mut properties);
        }
        let limits = [
            indexing
                .max_descriptor_set_update_after_bind_sampled_images
                .min(indexing.max_per_stage_descriptor_update_after_bind_sampled_images),
            indexing
                .max_descriptor_set_update_after_bind_storage_images
                .min(indexing.max_per_stage_descriptor_update_after_bind_storage_images),
            indexing
                .max_descriptor_set_update_after_bind_storage_buffers
                .min(indexing.max_per_stage_descriptor_update_after_bind_storage_buffers),
            indexing
                .max_descriptor_set_update_after_bind_samplers
                .min(indexing.max_per_stage_descriptor_update_after_bind_samplers),
        ];
        let total: u32 = counts.iter().sum();
        if counts
            .iter()
            .zip(limits)
            .any(|(count, limit)| *count > limit)
            || total > indexing.max_per_stage_update_after_bind_resources
            || total > indexing.max_update_after_bind_descriptors_in_all_pools
        {
            return Err(Error::unsupported(
                "bindless table exceeds enabled descriptor limits",
            ));
        }
        let host_bytes = crate::accounting::shared_cpu_bytes::<Self>()?
            + u64::from(total) * ((std::mem::size_of::<Slot>() + std::mem::size_of::<u32>()) as u64
                + crate::accounting::shared_cpu_bytes::<BindlessLease>()?);
        let mut slot_memory = device
            .memory
            .stats
            .reserve_cpu_bytes(host_bytes, crate::accounting::BudgetCategory::Live)?;
        let slots = Slots::new(counts)?;
        slot_memory.resize(slots.host_bytes()?)?;
        let flags = vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
            | vk::DescriptorBindingFlags::PARTIALLY_BOUND
            | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING;
        let plan = LayoutPlan {
            flags: vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw(),
            binding_flags: vec![flags.as_raw(); 4],
            bindings: DescriptorKind::ALL
                .into_iter()
                .map(|kind| LayoutBinding {
                    binding: kind as u32,
                    descriptor_type: kind.ty().as_raw(),
                    descriptor_count: counts[kind as usize],
                    stage_flags: vk::ShaderStageFlags::ALL.as_raw(),
                    immutable_sampler: None,
                })
                .collect(),
        };
        let layout = VulkanDescriptorSetLayout::new(device.clone(), &plan)?;
        let pool = VulkanDescriptorPool::new(device.clone(), &plan, 1)?;
        let set = pool.allocate_shared(layout.clone())?;
        Ok(Arc::new(Self {
            layout,
            set,
            device,
            slots: Mutex::new(slots),
            resident: Mutex::new(None),
            _slot_memory: slot_memory,
        }))
    }
    /// 建立共享同一真实布局的新物理页；槽、描述符池和集合各自独立。
    /// 本入口只提供页原语，完整绑定组准入与页选择仍由调用方完成。
    pub fn new_page(&self) -> Result<Arc<Self>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let counts: [u32; 4] = {
            let slots = self.slots.lock();
            // 各表容量只在 new 中建立且受 1<<20 总数限制，之后不会调整。
            std::array::from_fn(|kind| slots.entries[kind].len() as u32)
        };
        let plan = self.layout.plan();
        let flags = vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
            | vk::DescriptorBindingFlags::PARTIALLY_BOUND
            | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING;
        // layout 是公开 owner，仍验证其实际设备和结构，不能依赖外部未替换它。
        if !Arc::ptr_eq(&self.device, self.layout.device())
            || plan.flags != vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw()
            || plan.bindings.len() != 4
            || plan.binding_flags.len() != 4
            || plan
                .binding_flags
                .iter()
                .any(|value| *value != flags.as_raw())
            || plan
                .bindings
                .iter()
                .zip(DescriptorKind::ALL)
                .any(|(binding, kind)| {
                    binding.binding != kind as u32
                        || binding.descriptor_type != kind.ty().as_raw()
                        || binding.descriptor_count != counts[kind as usize]
                        || binding.stage_flags != vk::ShaderStageFlags::ALL.as_raw()
                        || binding.immutable_sampler.is_some()
                })
        {
            return Err(Error::invalid_usage(
                "bindless page layout differs from its tables",
            ));
        }
        let total: u32 = counts.iter().sum();
        // 与 new 相同，先准入并分配实际槽存储，再进入不可回滚的 native 创建。
        let host_bytes = crate::accounting::shared_cpu_bytes::<Self>()?
            + u64::from(total) * ((std::mem::size_of::<Slot>() + std::mem::size_of::<u32>()) as u64
                + crate::accounting::shared_cpu_bytes::<BindlessLease>()?);
        let mut slot_memory = self
            .device
            .memory
            .stats
            .reserve_cpu_bytes(host_bytes, crate::accounting::BudgetCategory::Live)?;
        let slots = Slots::new(counts)?;
        slot_memory.resize(slots.host_bytes()?)?;
        // 复用已实际创建成功的布局，不重复查询，也不复制模板页的占用或 generation。
        let pool = VulkanDescriptorPool::new(self.device.clone(), plan, 1)?;
        let set = pool.allocate_shared(self.layout.clone())?;
        Ok(Arc::new(Self {
            layout: self.layout.clone(),
            set,
            device: self.device.clone(),
            slots: Mutex::new(slots),
            resident: Mutex::new(None),
            _slot_memory: slot_memory,
        }))
    }
    fn reserve(self: &Arc<Self>, kind: DescriptorKind, owner: Owner) -> Result<Arc<BindlessLease>> {
        let _trace = RecordSpan::new(RecordPhase::DescriptorReserve);
        record_count(RecordCount::DescriptorReserveCalls, 1);

        let allocation = self.slots.lock().allocate(kind);
        let index = match allocation {
            Ok(index) => index,
            Err(Error::Exhausted(_)) => {
                self.trim_resident();
                self.collect()?;
                self.slots.lock().allocate(kind)?
            }
            Err(error) => return Err(error),
        };
        Ok(Arc::new(BindlessLease {
            owner,
            slot: LeaseSlot { heap: self.clone(), index },
        }))
    }
}

#[cfg(test)]
mod tests;


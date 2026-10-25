//! Descriptor set layouts, pools and sets: CPU layout/pool planning and the
//! Vulkan objects with their write, retention and retirement rules.
use crate::Result;
use ash::vk;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Binding {
    pub tag: u8,
    pub name: String,
    pub runtime: bool,
    pub count: u32,
    pub sampler: u64,
}

// Vulkan objects.
use crate::raw::binding::{DescriptorSetBinding, ResourceOwner};
use crate::raw::device::Device;
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

pub struct VulkanDescriptorSetLayout {
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::DescriptorSetLayout,
    pub(crate) plan: LayoutPlan,
    _samplers: Vec<Arc<crate::raw::sampler::Sampler>>,
    _memory: super::allocator::CpuMemoryReservation,
}

impl VulkanDescriptorSetLayout {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::DescriptorSetLayout {
        self.handle
    }
}

impl VulkanDescriptorSetLayout {
    pub fn plan(&self) -> &LayoutPlan {
        &self.plan
    }

    pub fn new(device: Arc<Device>, plan: &LayoutPlan) -> Result<Arc<Self>> {
        Self::new_with_samplers(device, plan, Vec::new())
    }

    /// 不可变 sampler 需保持到布局销毁；接收真实 Arc 对象而非仅保存整数句柄。
    pub fn new_with_samplers(
        device: Arc<Device>,
        plan: &LayoutPlan,
        retained: Vec<Arc<crate::raw::sampler::Sampler>>,
    ) -> Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        // 不可变采样器数组必须保持到 vkCreateDescriptorSetLayout 返回。
        use ash::vk::Handle;
        if plan.binding_flags.len() != plan.bindings.len()
            || plan.bindings.iter().any(|b| b.descriptor_count == 0)
            || plan.bindings.iter().enumerate().any(|(index, binding)| {
                plan.bindings[..index]
                    .iter()
                    .any(|prior| prior.binding == binding.binding)
            })
        {
            return Err(crate::Error::invalid_usage("描述符布局、写入或设备不匹配"));
        }
        for binding in &plan.bindings {
            if let Some(handle) = binding.immutable_sampler
                && !retained
                    .iter()
                    .any(|s| s.handle.as_raw() == handle && Arc::ptr_eq(&device, &s.device))
            {
                return Err(crate::Error::invalid_usage("描述符布局、写入或设备不匹配"));
            }
        }
        let retained_bytes = super::allocator::cpu_vec_bytes::<Arc<crate::raw::sampler::Sampler>>(
            retained.capacity(),
        )?;
        let plan_bytes = |bindings, flags| -> Result<u64> {
            super::allocator::cpu_vec_bytes::<LayoutBinding>(bindings)?
                .checked_add(super::allocator::cpu_vec_bytes::<u32>(flags)?)
                .and_then(|bytes| bytes.checked_add(retained_bytes))
                .ok_or(crate::Error::OutOfHostMemory)
        };
        let mut memory = super::allocator::reserve_shared_cpu_metadata::<Self>(
            &device.memory.stats,
            plan_bytes(plan.bindings.len(), plan.binding_flags.len())?,
        )?;
        let mut owned_plan = LayoutPlan {
            flags: plan.flags,
            binding_flags: Vec::new(),
            bindings: Vec::new(),
        };
        owned_plan
            .bindings
            .try_reserve_exact(plan.bindings.len())
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        owned_plan
            .binding_flags
            .try_reserve_exact(plan.binding_flags.len())
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        owned_plan.bindings.extend_from_slice(&plan.bindings);
        owned_plan
            .binding_flags
            .extend_from_slice(&plan.binding_flags);
        memory.resize(
            super::allocator::shared_cpu_bytes::<Self>()?
                .checked_add(plan_bytes(
                    owned_plan.bindings.capacity(),
                    owned_plan.binding_flags.capacity(),
                )?)
                .ok_or(crate::Error::OutOfHostMemory)?,
        )?;
        let sampler_count = plan.bindings.iter().try_fold(0usize, |count, binding| {
            count
                .checked_add(if binding.immutable_sampler.is_some() {
                    binding.descriptor_count as usize
                } else {
                    0
                })
                .ok_or(crate::Error::OutOfHostMemory)
        })?;
        let samplers = super::submit::SubmitScratch::<[vk::Sampler; 8]>::try_collect(
            sampler_count,
            &device.memory.stats,
            plan.bindings
                .iter()
                .filter_map(|binding| {
                    binding
                        .immutable_sampler
                        .map(|sampler| (sampler, binding.descriptor_count))
                })
                .flat_map(|(sampler, count)| {
                    std::iter::repeat_n(vk::Sampler::from_raw(sampler), count as usize)
                })
                .map(Ok),
        )?;
        let mut sampler_offset = 0;
        let bindings =
            super::submit::SubmitScratch::<[vk::DescriptorSetLayoutBinding<'_>; 8]>::try_collect(
                plan.bindings.len(),
                &device.memory.stats,
                plan.bindings.iter().map(|b| {
                    let value = vk::DescriptorSetLayoutBinding::default()
                        .binding(b.binding)
                        .descriptor_type(vk::DescriptorType::from_raw(b.descriptor_type))
                        .descriptor_count(b.descriptor_count)
                        .stage_flags(vk::ShaderStageFlags::from_raw(b.stage_flags));
                    Ok(if b.immutable_sampler.is_none() {
                        value
                    } else {
                        let start = sampler_offset;
                        sampler_offset += b.descriptor_count as usize;
                        value.immutable_samplers(&samplers[start..sampler_offset])
                    })
                }),
            )?;
        let flags = super::submit::SubmitScratch::<[vk::DescriptorBindingFlags; 8]>::try_collect(
            plan.binding_flags.len(),
            &device.memory.stats,
            plan.binding_flags
                .iter()
                .map(|f| Ok(vk::DescriptorBindingFlags::from_raw(*f))),
        )?;
        let mut flags_info =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
        let info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::from_raw(plan.flags))
            .bindings(&bindings)
            .push_next(&mut flags_info);
        // maxPerSetDescriptors is a guaranteed supported size, not a hard
        // maximum. Larger layouts require an affirmative implementation query.
        // Query every layout so native packing constraints are checked before
        // vkCreateDescriptorSetLayout (VUID-...-09582).
        let mut support = vk::DescriptorSetLayoutSupport::default();
        unsafe {
            device
                .core
                .device
                .get_descriptor_set_layout_support(&info, &mut support);
        }
        if support.supported == vk::FALSE {
            let mut maintenance = vk::PhysicalDeviceMaintenance3Properties::default();
            let mut properties =
                vk::PhysicalDeviceProperties2::default().push_next(&mut maintenance);
            unsafe {
                device
                    .core
                    .instance
                    .instance
                    .get_physical_device_properties2(device.core.physical, &mut properties);
            }
            let requested: u64 = plan
                .bindings
                .iter()
                .map(|binding| u64::from(binding.descriptor_count))
                .sum();
            return Err(crate::Error::unsupported(format!(
                "descriptor layout with {requested} descriptors is unsupported (guaranteed maxPerSetDescriptors={})",
                maintenance.max_per_set_descriptors,
            )));
        }
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let handle = crate::error::observe_vulkan(&device, unsafe {
            device.core.device.create_descriptor_set_layout(&info, None)
        })?;
        Ok(Arc::new(Self {
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            handle,
            plan: owned_plan,
            _samplers: retained,
            _memory: memory,
        }))
    }
}
impl Drop for VulkanDescriptorSetLayout {
    fn drop(&mut self) {
        self.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::DescriptorSetLayout(self.handle),
        );
    }
}

pub struct VulkanDescriptorPool {
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    // Fields drop in declaration order. Release pool storage before the last
    // device owner; deferred copies are drained inside Device::drop.
    allocation: Arc<DescriptorPoolAllocation>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::DescriptorPool,
    free_individual_sets: bool,
    live_sets: AtomicUsize,
    _memory: super::allocator::CpuMemoryReservation,
}

impl VulkanDescriptorPool {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::DescriptorPool {
        self.handle
    }
}
/// Raw pool ownership without an Arc<Device> back-reference.
pub struct DescriptorPoolAllocation {
    _descriptor_quota: Option<super::allocator::DescriptorReservation>,
    device: ash::Device,
    handle: vk::DescriptorPool,
    lock: Mutex<()>,
    _memory: super::allocator::CpuMemoryReservation,
}
impl DescriptorPoolAllocation {
    pub(crate) fn free(&self, handle: vk::DescriptorSet) {
        let _guard = self.lock.lock();
        unsafe {
            let _ = self.device.free_descriptor_sets(self.handle, &[handle]);
        }
    }
}
impl Drop for DescriptorPoolAllocation {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_pool(self.handle, None);
        }
    }
}
impl VulkanDescriptorPool {
    pub fn new(device: Arc<Device>, plan: &LayoutPlan, max_sets: u32) -> Result<Arc<Self>> {
        Self::create(device, plan, max_sets, true)
    }

    /// Transient pool: reset as a whole instead of freeing individual sets.
    pub fn new_transient(
        device: Arc<Device>,
        plan: &LayoutPlan,
        max_sets: u32,
    ) -> Result<Arc<Self>> {
        Self::create(device, plan, max_sets, false)
    }

    fn create(
        device: Arc<Device>,
        plan: &LayoutPlan,
        max_sets: u32,
        free_individual_sets: bool,
    ) -> Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if max_sets == 0 {
            return Err(crate::Error::invalid_usage("描述符布局、写入或设备不匹配"));
        }
        let unique = plan.bindings.iter().enumerate().filter(|(index, binding)| {
            !plan.bindings[..*index]
                .iter()
                .any(|previous| previous.descriptor_type == binding.descriptor_type)
        });
        let sizes = super::submit::SubmitScratch::<[vk::DescriptorPoolSize; 8]>::try_collect(
            unique.clone().count(),
            &device.memory.stats,
            unique.map(|(_, binding)| {
                let count = plan
                    .bindings
                    .iter()
                    .filter(|value| value.descriptor_type == binding.descriptor_type)
                    .try_fold(0u32, |total, value| {
                        value
                            .descriptor_count
                            .checked_mul(max_sets)
                            .and_then(|count| total.checked_add(count))
                            .ok_or_else(|| crate::Error::invalid_usage("描述符池容量溢出"))
                    })?;
                Ok(vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::from_raw(binding.descriptor_type),
                    descriptor_count: count,
                })
            }),
        )?;
        let mut flags = if free_individual_sets {
            vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET
        } else {
            vk::DescriptorPoolCreateFlags::empty()
        };
        if plan.flags & vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw() != 0 {
            flags |= vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND;
        }
        // Include pools waiting in Retirement in the device-wide descriptor
        // quota. This allocation owner deliberately contains no Arc<Device>.
        let descriptor_quota = if flags.contains(vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND) {
            let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
            let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
            unsafe {
                device
                    .core
                    .instance
                    .instance
                    .get_physical_device_properties2(device.core.physical, &mut properties);
            }
            let count = sizes
                .iter()
                .map(|size| u64::from(size.descriptor_count))
                .sum();
            Some(device.memory.stats.reserve_update_after_bind_descriptors(
                count,
                u64::from(indexing.max_update_after_bind_descriptors_in_all_pools),
            )?)
        } else {
            None
        };
        let info = vk::DescriptorPoolCreateInfo::default()
            .flags(flags)
            .max_sets(max_sets)
            .pool_sizes(&sizes);
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let memory =
            super::allocator::reserve_shared_cpu_metadata::<Self>(&device.memory.stats, 0)?;
        let allocation_memory = super::allocator::reserve_shared_cpu_metadata::<
            DescriptorPoolAllocation,
        >(&device.memory.stats, 0)?;
        let handle = crate::error::observe_vulkan(&device, unsafe {
            device.core.device.create_descriptor_pool(&info, None)
        })?;
        let allocation = Arc::new(DescriptorPoolAllocation {
            _descriptor_quota: descriptor_quota,
            device: device.core.device.clone(),
            handle,
            lock: Mutex::new(()),
            _memory: allocation_memory,
        });
        Ok(Arc::new(Self {
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            handle,
            allocation,
            free_individual_sets,
            live_sets: AtomicUsize::new(0),
            _memory: memory,
        }))
    }
    pub fn allocate(
        self: &Arc<Self>,
        layout: Arc<VulkanDescriptorSetLayout>,
    ) -> Result<VulkanDescriptorSet> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if !Arc::ptr_eq(&self.device, &layout.device) {
            return Err(crate::Error::invalid_usage("描述符布局、写入或设备不匹配"));
        }
        let max_resources = layout
            .plan
            .bindings
            .iter()
            .try_fold(0usize, |total, binding| {
                total
                    .checked_add(binding.descriptor_count as usize)
                    .ok_or_else(|| {
                        crate::Error::invalid_usage("descriptor resource table capacity overflows")
                    })
            })?
            .max(1);
        let resources =
            super::temp_list::TempList::new(max_resources, self.device.memory.stats.clone())?;
        let _guard = self.allocation.lock.lock();
        let layouts = [layout.handle];
        let variable_count = layout
            .plan
            .bindings
            .iter()
            .zip(&layout.plan.binding_flags)
            .find(|(_, flags)| {
                **flags & vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw() != 0
            })
            .map(|(binding, _)| binding.descriptor_count);
        let counts = [variable_count.unwrap_or(0)];
        let mut variable = vk::DescriptorSetVariableDescriptorCountAllocateInfo::default()
            .descriptor_counts(&counts);
        let mut info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.handle)
            .set_layouts(&layouts);
        if variable_count.is_some() {
            info = info.push_next(&mut variable);
        }
        let sync_storage =
            crate::resource::SyncCell::reserve_arc_storage(&self.device.memory.stats)?;
        let handle = crate::error::observe_vulkan(&self.device, unsafe {
            self.device.core.device.allocate_descriptor_sets(&info)
        })?[0];
        self.live_sets.fetch_add(1, Ordering::Relaxed);
        Ok(VulkanDescriptorSet {
            last_use: crate::resource::SyncCell::for_device(self.device.generation())
                .into_reserved_arc(sync_storage),
            handle,
            layout,
            pool: self.clone(),
            resources,
            _memory: None,
        })
    }

    /// Renderer-owned shared set storage is admitted before native allocation;
    /// by-value public sets do not charge a caller's optional Arc wrapper.
    pub(crate) fn allocate_shared(
        self: &Arc<Self>,
        layout: Arc<VulkanDescriptorSetLayout>,
    ) -> Result<Arc<VulkanDescriptorSet>> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let memory = super::allocator::reserve_shared_cpu_metadata::<VulkanDescriptorSet>(
            &self.device.memory.stats,
            0,
        )?;
        let mut set = self.allocate(layout)?;
        set._memory = Some(memory);
        Ok(Arc::new(set))
    }

    /// 仅空闲池调用：调用方必须已经释放所有已提交集合的 GPU 持有引用。
    pub fn reset(&self) -> Result<()> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if self.free_individual_sets {
            return Err(crate::Error::invalid_usage(
                "persistent descriptor pools cannot be reset",
            ));
        }
        let _reservation = self.last_use.lock_submission();
        let _guard = self.allocation.lock.lock();
        if self.live_sets.load(Ordering::Relaxed) != 0 {
            return Err(vk::Result::NOT_READY.into());
        }
        if !self
            .device
            .queues
            .timelines
            .is_reached(&self.last_use.snapshot())?
        {
            return Err(vk::Result::NOT_READY.into());
        }
        crate::error::observe_vulkan(&self.device, unsafe {
            self.device
                .core
                .device
                .reset_descriptor_pool(self.handle, vk::DescriptorPoolResetFlags::empty())
        })
    }
}
impl VulkanDescriptorPool {
    /// Sets allocated from this pool that are still alive on the CPU.
    pub fn live_set_count(&self) -> usize {
        self.live_sets.load(Ordering::Relaxed)
    }
    /// Whether the pool resets as a whole instead of freeing individual sets.
    pub fn is_transient(&self) -> bool {
        !self.free_individual_sets
    }
}
impl Drop for VulkanDescriptorPool {
    fn drop(&mut self) {
        self.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::DescriptorPool(self.allocation.clone()),
        );
    }
}

/// 持有实际 pool/layout 与写入资源；命令登记的 last_use 决定安全退役。
pub struct VulkanDescriptorSet {
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub(crate) handle: vk::DescriptorSet,
    pub layout: Arc<VulkanDescriptorSetLayout>,
    pub pool: Arc<VulkanDescriptorPool>,
    resources: super::temp_list::TempList<DescriptorOwners>,
    _memory: Option<super::allocator::CpuMemoryReservation>,
}

impl VulkanDescriptorSet {
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::DescriptorSet {
        self.handle
    }
}
struct DescriptorOwners {
    binding: u32,
    element: u32,
    owners: super::submit::SubmitScratch<[ResourceOwner; 4]>,
}
impl VulkanDescriptorSet {
    /// # Safety
    /// 更新时必须满足 Vulkan 的宿主/GPU 同步条件，绑定对象必须属于本设备。
    pub unsafe fn update(
        &mut self,
        binding: u32,
        array_element: u32,
        value: &DescriptorSetBinding,
    ) -> Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let slot = self
            .layout
            .plan
            .bindings
            .iter()
            .find(|b| b.binding == binding)
            .ok_or_else(|| crate::Error::invalid_usage("描述符布局、写入或设备不匹配"))?;
        if array_element >= slot.descriptor_count
            || slot.descriptor_type != value.descriptor_type().as_raw()
        {
            return Err(crate::Error::invalid_usage("描述符布局、写入或设备不匹配"));
        }
        let prior = self
            .resources
            .as_slice()
            .iter()
            .position(|entry| entry.binding == binding && entry.element == array_element);
        if prior.is_none() {
            self.resources.reserve(
                self.resources
                    .as_slice()
                    .len()
                    .checked_add(1)
                    .ok_or(crate::Error::OutOfHostMemory)?,
            )?;
        }
        let owner_refs = value.owner_refs();
        let prepared = DescriptorOwners {
            binding,
            element: array_element,
            owners: super::submit::SubmitScratch::try_collect(
                owner_refs.len(),
                &self.pool.device.memory.stats,
                owner_refs.iter().cloned().map(Ok),
            )?,
        };
        unsafe {
            value.write(&self.pool.device, self.handle, binding, array_element)?;
        }
        if let Some(index) = prior {
            *self
                .resources
                .get_mut(index)
                .expect("validated descriptor owner row") = prepared;
        } else {
            self.resources.add_reserved(prepared);
        }
        Ok(())
    }
}
impl Drop for VulkanDescriptorSet {
    fn drop(&mut self) {
        {
            let _guard = self.pool.allocation.lock.lock();
            self.pool.live_sets.fetch_sub(1, Ordering::Relaxed);
        }
        if !self.pool.free_individual_sets {
            // 瞬时 set 随整池 reset 回收，不能单独 free，更不能重用仍在录制的 set。
            return;
        }
        self.pool.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::DescriptorSet(self.handle, self.pool.allocation.clone()),
        );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutBinding {
    pub binding: u32,
    pub descriptor_type: i32,
    pub descriptor_count: u32,
    pub stage_flags: u32,
    pub immutable_sampler: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutPlan {
    pub flags: u32,
    pub binding_flags: Vec<u32>,
    pub bindings: Vec<LayoutBinding>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    ZeroDescriptorCount,
    UnknownReflectionTag(u8),
    Vulkan(i32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "描述符布局无效：{self:?}")
    }
}
impl std::error::Error for Error {}
impl From<Error> for crate::Error {
    fn from(source: Error) -> Self {
        Self::invalid_usage_with("描述符布局无效", source)
    }
}

pub fn runtime_descriptor_count(max_descriptor_set_update_after_bind_sampled_images: u32) -> u32 {
    max_descriptor_set_update_after_bind_sampled_images
        .saturating_sub(32)
        .min(8192)
}

pub fn descriptor_set_layout(
    bindings: &[Binding],
    first_binding: u64,
    stage_flags: vk::ShaderStageFlags,
    device_limit: u32,
) -> Result<LayoutPlan> {
    if first_binding > u64::from(u32::MAX)
        || (!bindings.is_empty()
            && first_binding
                .checked_add(bindings.len() as u64 - 1)
                .is_none_or(|last| last > u64::from(u32::MAX)))
    {
        return Err(crate::Error::invalid_usage("描述符 binding 编号溢出"));
    }
    let mut result = LayoutPlan {
        flags: 0,
        binding_flags: vec![0; bindings.len()],
        bindings: Vec::with_capacity(bindings.len()),
    };
    for (index, binding) in bindings.iter().enumerate() {
        let count = if binding.tag == 5 { 1 } else if binding.runtime {
            runtime_descriptor_count(device_limit)
        } else {
            binding.count
        };
        if count == 0 {
            return Err(Error::ZeroDescriptorCount.into());
        }
        if count != 1 {
            result.binding_flags[index] = vk::DescriptorBindingFlags::PARTIALLY_BOUND.as_raw();
        }
        let descriptor_type = match binding.tag {
            0 => {
                if binding.name.ends_with("_dyn") {
                    vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
                } else {
                    vk::DescriptorType::STORAGE_BUFFER
                }
            }
            1 => {
                if binding.name.ends_with("_dyn") {
                    vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
                } else {
                    vk::DescriptorType::UNIFORM_BUFFER
                }
            }
            2 => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            3 => vk::DescriptorType::STORAGE_IMAGE,
            4 => {
                if binding.runtime {
                    result.binding_flags[index] = 15;
                    result.flags |=
                        vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw();
                }
                vk::DescriptorType::SAMPLED_IMAGE
            }
            5 => vk::DescriptorType::SAMPLER,
            other => return Err(Error::UnknownReflectionTag(other).into()),
        };
        result.bindings.push(LayoutBinding {
            binding: (first_binding + index as u64) as u32,
            descriptor_type: descriptor_type.as_raw(),
            descriptor_count: if binding.tag == 5 { 1 } else { count },
            stage_flags: stage_flags.as_raw(),
            immutable_sampler: (binding.tag == 5).then_some(binding.sampler),
        });
    }
    Ok(result)
}




/// Maximum descriptor sets represented by the public shader interface schema.
pub const MAX_DESCRIPTOR_SETS: usize = 4;

impl crate::resource::GpuUse for VulkanDescriptorSetLayout {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        for sampler in &self._samplers {
            sampler.visit_uses(visitor)?;
        }

        Ok(())
    }
}

impl crate::resource::GpuUse for VulkanDescriptorPool {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        crate::resource::tracked::visit_cells(visitor, [&self.last_use])
    }
}

impl crate::resource::GpuUse for VulkanDescriptorSet {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        self.pool.visit_uses(visitor)?;
        self.layout.visit_uses(visitor)?;
        for entry in self.resources.as_slice() {
            for owner in entry.owners.iter() {
                owner.visit_uses(visitor)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod descriptor_pool_tests {
    use super::*;

    #[test]
    fn layout_rejects_invalid_input_as_usage_error_with_details() {
        let binding = Binding {
            tag: 0,
            name: "buffer".into(),
            runtime: false,
            count: 0,
            sampler: 0,
        };
        let error = descriptor_set_layout(
            std::slice::from_ref(&binding),
            0,
            vk::ShaderStageFlags::COMPUTE,
            0,
        )
        .unwrap_err();
        assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
        assert!(matches!(
            std::error::Error::source(&error).and_then(|source| source.downcast_ref::<Error>()),
            Some(Error::ZeroDescriptorCount)
        ));
        let valid = Binding {
            count: 1,
            ..binding
        };
        let error = descriptor_set_layout(
            &[valid.clone(), valid],
            u64::from(u32::MAX),
            vk::ShaderStageFlags::COMPUTE,
            0,
        )
        .unwrap_err();
        assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
    }
}
#[path = "descriptor_pool_ring.rs"]
mod pool_ring;
pub use pool_ring::{DescriptorPoolRing, DescriptorPoolRingConfig, DescriptorPoolRingStats};

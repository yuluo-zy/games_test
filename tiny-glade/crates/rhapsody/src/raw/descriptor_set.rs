//! 原 descriptor layout/pool CPU 规则及新增 Vulkan 对象生产、写入、回收候选接口。
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

// 以下为可运行 Vulkan 候选路径；上方原 CPU layout/pool 规则保持独立证据范围。
use crate::raw::binding::{DescriptorSetBinding, ResourceOwner};
use crate::raw::device::VulkanDevice;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};

pub struct VulkanDescriptorSetLayout {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::DescriptorSetLayout,
    pub plan: LayoutPlan,
    _samplers: Vec<Arc<crate::raw::sampler::Sampler>>,
}

impl VulkanDescriptorSetLayout {
    pub fn new(device: Arc<VulkanDevice>, plan: &LayoutPlan) -> Result<Arc<Self>, vk::Result> {
        Self::new_with_samplers(device, plan, Vec::new())
    }

    /// 不可变 sampler 需保持到布局销毁；接收真实 Arc 对象而非仅保存整数句柄。
    pub fn new_with_samplers(
        device: Arc<VulkanDevice>,
        plan: &LayoutPlan,
        retained: Vec<Arc<crate::raw::sampler::Sampler>>,
    ) -> Result<Arc<Self>, vk::Result> {
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
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        for binding in &plan.bindings {
            if let Some(handle) = binding.immutable_sampler
                && !retained
                    .iter()
                    .any(|s| s.handle.as_raw() == handle && Arc::ptr_eq(&device, &s.device))
            {
                return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
            }
        }
        let samplers: Vec<Vec<vk::Sampler>> = plan
            .bindings
            .iter()
            .map(|b| {
                b.immutable_sampler
                    .map(|s| vec![vk::Sampler::from_raw(s); b.descriptor_count as usize])
                    .unwrap_or_default()
            })
            .collect();
        let bindings: Vec<_> = plan
            .bindings
            .iter()
            .zip(&samplers)
            .map(|(b, s)| {
                let value = vk::DescriptorSetLayoutBinding::default()
                    .binding(b.binding)
                    .descriptor_type(vk::DescriptorType::from_raw(b.descriptor_type))
                    .descriptor_count(b.descriptor_count)
                    .stage_flags(vk::ShaderStageFlags::from_raw(b.stage_flags));
                if s.is_empty() {
                    value
                } else {
                    value.immutable_samplers(s)
                }
            })
            .collect();
        let flags: Vec<_> = plan
            .binding_flags
            .iter()
            .map(|f| vk::DescriptorBindingFlags::from_raw(*f))
            .collect();
        let mut flags_info =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
        let info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::from_raw(plan.flags))
            .bindings(&bindings)
            .push_next(&mut flags_info);
        let handle = unsafe { device.device.create_descriptor_set_layout(&info, None) }?;
        Ok(Arc::new(Self {
            device,
            handle,
            plan: plan.clone(),
            _samplers: retained,
        }))
    }
}
impl Drop for VulkanDescriptorSetLayout {
    fn drop(&mut self) {
        self.device
            .bury(super::graveyard::Doomed::DescriptorSetLayout(self.handle));
    }
}

pub struct VulkanDescriptorPool {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::DescriptorPool,
    lock: Mutex<()>,
}
impl VulkanDescriptorPool {
    pub fn new(
        device: Arc<VulkanDevice>,
        plan: &LayoutPlan,
        max_sets: u32,
    ) -> Result<Arc<Self>, vk::Result> {
        Self::create(device, plan, max_sets, true)
    }

    /// 瞬时池整池 reset，不逐 set 释放；原分配器的空闲栈保存池句柄。
    fn new_transient(
        device: Arc<VulkanDevice>,
        plan: &LayoutPlan,
        max_sets: u32,
    ) -> Result<Arc<Self>, vk::Result> {
        Self::create(device, plan, max_sets, false)
    }

    fn create(
        device: Arc<VulkanDevice>,
        plan: &LayoutPlan,
        max_sets: u32,
        free_individual_sets: bool,
    ) -> Result<Arc<Self>, vk::Result> {
        if max_sets == 0 {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let mut counts = BTreeMap::<i32, u32>::new();
        for binding in &plan.bindings {
            let count = binding
                .descriptor_count
                .checked_mul(max_sets)
                .ok_or(vk::Result::ERROR_OUT_OF_POOL_MEMORY)?;
            let total = counts.entry(binding.descriptor_type).or_default();
            *total = total
                .checked_add(count)
                .ok_or(vk::Result::ERROR_OUT_OF_POOL_MEMORY)?;
        }
        let sizes: Vec<_> = counts
            .into_iter()
            .map(|(ty, count)| vk::DescriptorPoolSize {
                ty: vk::DescriptorType::from_raw(ty),
                descriptor_count: count,
            })
            .collect();
        let mut flags = if free_individual_sets {
            vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET
        } else {
            vk::DescriptorPoolCreateFlags::empty()
        };
        if plan.flags & vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw() != 0 {
            flags |= vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND;
        }
        let info = vk::DescriptorPoolCreateInfo::default()
            .flags(flags)
            .max_sets(max_sets)
            .pool_sizes(&sizes);
        let handle = unsafe { device.device.create_descriptor_pool(&info, None) }?;
        Ok(Arc::new(Self {
            device,
            handle,
            lock: Mutex::new(()),
        }))
    }
    pub fn allocate(
        self: &Arc<Self>,
        layout: Arc<VulkanDescriptorSetLayout>,
    ) -> Result<VulkanDescriptorSet, vk::Result> {
        if !Arc::ptr_eq(&self.device, &layout.device) {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let _guard = self.lock.lock();
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
        let handle = unsafe { self.device.device.allocate_descriptor_sets(&info) }?[0];
        Ok(VulkanDescriptorSet {
            handle,
            layout,
            pool: self.clone(),
            resources: BTreeMap::new(),
            transient_lease: None,
        })
    }

    /// 仅空闲池调用：调用方必须已经释放所有已提交集合的 GPU 持有引用。
    fn reset(&self) -> Result<(), vk::Result> {
        let _guard = self.lock.lock();
        unsafe {
            self.device
                .device
                .reset_descriptor_pool(self.handle, vk::DescriptorPoolResetFlags::empty())
        }
    }
}
impl Drop for VulkanDescriptorPool {
    fn drop(&mut self) {
        self.device
            .bury(super::graveyard::Doomed::DescriptorPool(self.handle));
    }
}

/// pool/layout/所有写入资源均被持有。候选回收先 wait_idle，后续统一接原 managed 队列。
pub struct VulkanDescriptorSet {
    pub handle: vk::DescriptorSet,
    pub layout: Arc<VulkanDescriptorSetLayout>,
    pub pool: Arc<VulkanDescriptorPool>,
    resources: BTreeMap<(u32, u32), Vec<ResourceOwner>>,
    // 与录制者的 Arc<Set> 同寿命；最后引用释放后才允许池再次进入空闲来源。
    transient_lease: Option<Arc<DescriptorPoolLease<Arc<VulkanDescriptorPool>>>>,
}
impl VulkanDescriptorSet {
    /// # Safety
    /// 更新时必须满足 Vulkan 的宿主/GPU 同步条件，绑定对象必须属于本设备。
    pub unsafe fn update(
        &mut self,
        binding: u32,
        array_element: u32,
        value: &DescriptorSetBinding,
    ) -> Result<(), vk::Result> {
        let slot = self
            .layout
            .plan
            .bindings
            .iter()
            .find(|b| b.binding == binding)
            .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
        if array_element >= slot.descriptor_count
            || slot.descriptor_type != value.descriptor_type().as_raw()
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        unsafe {
            value.write(&self.pool.device, self.handle, binding, array_element);
        }
        self.resources
            .insert((binding, array_element), value.owners());
        Ok(())
    }
}
impl Drop for VulkanDescriptorSet {
    fn drop(&mut self) {
        if self.transient_lease.is_some() {
            // 瞬时 set 随整池 reset 回收，不能单独 free，更不能重用仍在录制的 set。
            return;
        }
        let _guard = self.pool.lock.lock();
        unsafe {
            let _ = self.pool.device.wait_idle();
            let _ = self
                .pool
                .device
                .device
                .free_descriptor_sets(self.pool.handle, &[self.handle]);
        }
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
) -> Result<LayoutPlan, Error> {
    let mut result = LayoutPlan {
        flags: 0,
        binding_flags: vec![0; bindings.len()],
        bindings: Vec::with_capacity(bindings.len()),
    };
    for (index, binding) in bindings.iter().enumerate() {
        let count = if binding.runtime {
            runtime_descriptor_count(device_limit)
        } else {
            binding.count
        };
        if count == 0 {
            return Err(Error::ZeroDescriptorCount);
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
            other => return Err(Error::UnknownReflectionTag(other)),
        };
        result.bindings.push(LayoutBinding {
            binding: first_binding.wrapping_add(index as u64) as u32,
            descriptor_type: descriptor_type.as_raw(),
            descriptor_count: if binding.tag == 5 { 1 } else { count },
            stage_flags: stage_flags.as_raw(),
            immutable_sampler: (binding.tag == 5).then_some(binding.sampler),
        });
    }
    Ok(result)
}

impl LayoutPlan {
    pub fn with_create_info<R>(
        &self,
        call: impl FnOnce(&vk::DescriptorSetLayoutCreateInfo<'_>) -> R,
    ) -> R {
        use ash::vk::Handle;
        let samplers: Vec<vk::Sampler> = self
            .bindings
            .iter()
            .map(|b| vk::Sampler::from_raw(b.immutable_sampler.unwrap_or(0)))
            .collect();
        let bindings: Vec<_> = self
            .bindings
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let mut result = vk::DescriptorSetLayoutBinding::default()
                    .binding(b.binding)
                    .descriptor_type(vk::DescriptorType::from_raw(b.descriptor_type))
                    .descriptor_count(b.descriptor_count)
                    .stage_flags(vk::ShaderStageFlags::from_raw(b.stage_flags));
                if b.immutable_sampler.is_some() {
                    result.p_immutable_samplers = &samplers[i];
                }
                result
            })
            .collect();
        let flags: Vec<_> = self
            .binding_flags
            .iter()
            .map(|&x| vk::DescriptorBindingFlags::from_raw(x))
            .collect();
        let mut extension =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
        let info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::from_raw(self.flags))
            .bindings(&bindings)
            .push_next(&mut extension);
        call(&info)
    }
}

pub fn with_compute_pipeline_layout<R>(
    layouts: &[vk::DescriptorSetLayout],
    push_constant_bytes: u32,
    call: impl FnOnce(&vk::PipelineLayoutCreateInfo<'_>) -> R,
) -> R {
    let range = vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::COMPUTE)
        .offset(0)
        .size(push_constant_bytes);
    let ranges = if push_constant_bytes == 0 {
        &[][..]
    } else {
        std::slice::from_ref(&range)
    };
    call(
        &vk::PipelineLayoutCreateInfo::default()
            .set_layouts(layouts)
            .push_constant_ranges(ranges),
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentPoolPlan {
    pub flags: u32,
    pub max_sets: u32,
    pub sizes: Vec<(i32, u32)>,
    pub variable_descriptor_count: u32,
}

pub fn persistent_pool_plan(layout: &LayoutPlan, device_limit: u32) -> PersistentPoolPlan {
    let mut sizes: Vec<(i32, u32)> = Vec::new();
    for binding in &layout.bindings {
        if let Some((_, count)) = sizes
            .iter_mut()
            .find(|(ty, _)| *ty == binding.descriptor_type)
        {
            *count = count.wrapping_add(binding.descriptor_count);
        } else {
            sizes.push((binding.descriptor_type, binding.descriptor_count));
        }
    }
    PersistentPoolPlan {
        flags: vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND.as_raw(),
        max_sets: 1,
        sizes,
        variable_descriptor_count: runtime_descriptor_count(device_limit),
    }
}

impl PersistentPoolPlan {
    pub fn with_pool_create_info<R>(
        &self,
        call: impl FnOnce(&vk::DescriptorPoolCreateInfo<'_>) -> R,
    ) -> R {
        let sizes: Vec<_> = self
            .sizes
            .iter()
            .map(|&(ty, count)| vk::DescriptorPoolSize {
                ty: vk::DescriptorType::from_raw(ty),
                descriptor_count: count,
            })
            .collect();
        call(
            &vk::DescriptorPoolCreateInfo::default()
                .flags(vk::DescriptorPoolCreateFlags::from_raw(self.flags))
                .max_sets(self.max_sets)
                .pool_sizes(&sizes),
        )
    }

    pub fn with_allocate_info<R>(
        &self,
        pool: vk::DescriptorPool,
        layout: vk::DescriptorSetLayout,
        call: impl FnOnce(&vk::DescriptorSetAllocateInfo<'_>) -> R,
    ) -> R {
        let counts = [self.variable_descriptor_count];
        let layouts = [layout];
        let mut extension = vk::DescriptorSetVariableDescriptorCountAllocateInfo::default()
            .descriptor_counts(&counts);
        call(
            &vk::DescriptorSetAllocateInfo::default()
                .descriptor_pool(pool)
                .set_layouts(&layouts)
                .push_next(&mut extension),
        )
    }
}

/// 原版每个 `DescriptorSetState` 的瞬时池容量：`vkCreateDescriptorPool.maxSets = 100`
/// （0x140be5a40 的 CreateInfo `+0x14 = 0x64`），池尺寸 = 每种描述符类型 `Σ binding.descriptorCount × 100`
/// （构造闭包 0x1418b3a00 的 `imul rdi,[entry-8],0x64` 与按类型线性累加）；flags 恒 0。
pub const DESCRIPTOR_POOL_MAX_SETS: u32 = 100;
/// `create_descriptor_set_layouts`（0x140be60a0）的 `assert!(set_count <= MAX_DESCRIPTOR_SETS)`，常量 4。
pub const MAX_DESCRIPTOR_SETS: usize = 4;

/// 原 0x140be5a40：栈顶池分配不同 set，耗尽池移交调用方，完成后归还共享来源。
/// 不以写入内容查缓存；同内容的两个活跃请求仍得到不同 set。
pub struct DescriptorSetState {
    pub layout: Arc<VulkanDescriptorSetLayout>,
    sets_per_pool: u32,
    state: Mutex<DescriptorPoolQueue<Arc<VulkanDescriptorPool>>>,
}

struct DescriptorPoolSource<P> {
    returned: Mutex<Vec<P>>,
}

struct DescriptorPoolLease<P> {
    pool: Option<P>,
    source: Weak<DescriptorPoolSource<P>>,
}
impl<P> Drop for DescriptorPoolLease<P> {
    fn drop(&mut self) {
        if let Some(source) = self.source.upgrade() {
            source.returned.lock().push(self.pool.take().unwrap());
        }
    }
}

/// 调用方应把耗尽池持有到提交完成；仍有 Arc<Set> 时此引用释放也不会提前 reset。
pub struct RetiredDescriptorPool {
    _lease: Arc<DescriptorPoolLease<Arc<VulkanDescriptorPool>>>,
}

/// 过期描述符池的回收堆；`reclaim_descriptor_pools` 一次性取回全部，由上层决定
/// 归还空闲池还是销毁。原 `DescriptorPoolDiscardPile` 的候选表示。
#[derive(Default)]
pub struct DescriptorPoolDiscardPile {
    retired: Vec<RetiredDescriptorPool>,
}
impl DescriptorPoolDiscardPile {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, pool: RetiredDescriptorPool) {
        self.retired.push(pool);
    }
    pub fn is_empty(&self) -> bool {
        self.retired.is_empty()
    }
    pub fn len(&self) -> usize {
        self.retired.len()
    }
    /// 原入口：把当前堆里的回收池转交给 `into`，本堆清空。
    pub fn reclaim_descriptor_pools(&mut self, into: &mut Vec<RetiredDescriptorPool>) {
        into.append(&mut self.retired);
    }
}

struct DescriptorPoolQueue<P> {
    pools: Vec<Arc<DescriptorPoolLease<P>>>,
    source: Arc<DescriptorPoolSource<P>>,
}
impl<P> Default for DescriptorPoolQueue<P> {
    fn default() -> Self {
        Self {
            pools: Vec::new(),
            source: Arc::new(DescriptorPoolSource {
                returned: Mutex::new(Vec::new()),
            }),
        }
    }
}
impl<P> DescriptorPoolQueue<P> {
    // 与真实 Vulkan 入口共用整个循环；测试仅替换 API，不复刻分配器实现。
    fn allocate<T>(
        &mut self,
        retired: &mut Vec<Arc<DescriptorPoolLease<P>>>,
        mut create: impl FnMut() -> Result<P, vk::Result>,
        mut reset: impl FnMut(&P) -> Result<(), vk::Result>,
        mut allocate: impl FnMut(&P) -> Result<T, vk::Result>,
    ) -> Result<(T, Arc<DescriptorPoolLease<P>>), vk::Result> {
        loop {
            if self.pools.is_empty() {
                let returned = std::mem::take(&mut *self.source.returned.lock());
                for pool in returned {
                    // reset 只对来源返还的整池执行，新建池无需 reset。
                    reset(&pool)?;
                    self.pools.push(Arc::new(DescriptorPoolLease {
                        pool: Some(pool),
                        source: Arc::downgrade(&self.source),
                    }));
                }
                if self.pools.is_empty() {
                    self.pools.push(Arc::new(DescriptorPoolLease {
                        pool: Some(create()?),
                        source: Arc::downgrade(&self.source),
                    }));
                }
            }
            let lease = self.pools.last().unwrap();
            match allocate(lease.pool.as_ref().unwrap()) {
                Ok(set) => return Ok((set, lease.clone())),
                Err(vk::Result::ERROR_OUT_OF_POOL_MEMORY | vk::Result::ERROR_FRAGMENTED_POOL) => {
                    // 先移出栈，再由调用方保留；不可当场 reset 仍被 GPU 使用的池。
                    retired.push(self.pools.pop().unwrap());
                }
                Err(error) => return Err(error),
            }
        }
    }
}
#[derive(Clone)]
pub struct DescriptorWrite {
    pub binding: u32,
    pub array_element: u32,
    pub value: DescriptorSetBinding,
}
impl DescriptorSetState {
    /// 原版构造（`create_descriptor_set_states` 闭包 0x1418b3a00）：每池 100 个 set。
    pub fn for_layout(layout: Arc<VulkanDescriptorSetLayout>) -> Result<Self, vk::Result> {
        Self::new(layout, DESCRIPTOR_POOL_MAX_SETS)
    }
    pub fn new(
        layout: Arc<VulkanDescriptorSetLayout>,
        sets_per_pool: u32,
    ) -> Result<Self, vk::Result> {
        if sets_per_pool == 0 {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        Ok(Self {
            layout,
            sets_per_pool,
            state: Mutex::new(DescriptorPoolQueue::default()),
        })
    }
    /// # Safety
    /// 所有写入句柄必须属于布局设备，资源的实际大小/用途/图像布局必须符合声明。
    /// 返回集合不可再更新；录制者必须持有返回的 Arc 至 GPU 完成。
    pub unsafe fn allocate_descriptor_set(
        &self,
        writes: &[DescriptorWrite],
    ) -> Result<Arc<VulkanDescriptorSet>, vk::Result> {
        let mut retired = Vec::new();
        // 兼容入口：活跃 set 本身持有池租约。需要原 caller 回收向量时使用下方入口。
        unsafe { self.allocate_descriptor_set_with_retired(writes, &mut retired) }
    }

    /// # Safety
    /// 与 allocate_descriptor_set 相同；retired 向量及返回的 Arc<Set> 必须由提交所有者
    /// 持有至 GPU 完成。此处不制造额外 wait_idle，也不隐式丢弃 caller 的耗尽池。
    pub unsafe fn allocate_descriptor_set_with_retired(
        &self,
        writes: &[DescriptorWrite],
        retired: &mut Vec<RetiredDescriptorPool>,
    ) -> Result<Arc<VulkanDescriptorSet>, vk::Result> {
        self.validate_writes(writes)?;
        let mut state = self.state.lock();
        let mut retired_leases = Vec::new();
        let result = state.allocate(
            &mut retired_leases,
            || {
                VulkanDescriptorPool::new_transient(
                    self.layout.device.clone(),
                    &self.layout.plan,
                    self.sets_per_pool,
                )
            },
            |pool| pool.reset(),
            |pool| pool.allocate(self.layout.clone()),
        );
        // 包括失败路径：前面的耗尽池已移出栈，调用方仍必须收到它们。
        retired.extend(
            retired_leases
                .into_iter()
                .map(|lease| RetiredDescriptorPool { _lease: lease }),
        );
        let (mut set, lease) = result?;
        set.transient_lease = Some(lease);
        for write in writes {
            unsafe {
                set.update(write.binding, write.array_element, &write.value)?;
            }
        }
        Ok(Arc::new(set))
    }
    fn validate_writes(&self, writes: &[DescriptorWrite]) -> Result<(), vk::Result> {
        let mut ordered = BTreeMap::new();
        for write in writes {
            let slot = self
                .layout
                .plan
                .bindings
                .iter()
                .find(|b| b.binding == write.binding)
                .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
            if write.array_element >= slot.descriptor_count
                || slot.descriptor_type != write.value.descriptor_type().as_raw()
                || ordered
                    .insert((write.binding, write.array_element), &write.value)
                    .is_some()
            {
                return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
            }
        }
        // 未启用 PARTIALLY_BOUND 的每个元素都必须有内容；不可变 sampler 无需更新。
        for (index, slot) in self.layout.plan.bindings.iter().enumerate() {
            if slot.immutable_sampler.is_none() && self.layout.plan.binding_flags[index] & 4 == 0 {
                for element in 0..slot.descriptor_count {
                    if !ordered.contains_key(&(slot.binding, element)) {
                        return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
                    }
                }
            }
        }
        for value in ordered.into_values() {
            if let DescriptorSetBinding::Buffer { info, .. } = value
                && (info.buffer == vk::Buffer::null() || info.range == 0)
            {
                return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
            }
        }
        Ok(())
    }
    /// 移出当前池；仍被 set/command 持有的池只能在最后租约释放后归还来源。
    pub fn invalidate(&self) {
        let mut state = self.state.lock();
        state.pools.clear();
    }
    /// 兼容旧观测接口；内容缓存已经移除，始终为 0。
    pub fn cached_set_count(&self) -> usize {
        0
    }
    pub fn free_pool_count(&self) -> usize {
        let state = self.state.lock();
        state.source.returned.lock().len()
    }
}

#[cfg(test)]
mod descriptor_pool_tests {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[test]
    fn exhausted_and_fragmented_pools_retry_without_reusing_sets() {
        let mut queue = DescriptorPoolQueue::<u32>::default();
        let mut retired = Vec::new();
        let next_pool = Cell::new(0);
        let events = RefCell::new(Vec::new());
        let (set, lease) = queue
            .allocate(
                &mut retired,
                || {
                    let id = next_pool.get() + 1;
                    next_pool.set(id);
                    events.borrow_mut().push(format!("create:{id}"));
                    Ok(id)
                },
                |pool| {
                    events.borrow_mut().push(format!("reset:{pool}"));
                    Ok(())
                },
                |pool| {
                    events.borrow_mut().push(format!("allocate:{pool}"));
                    match pool {
                        1 => Err(vk::Result::ERROR_OUT_OF_POOL_MEMORY),
                        2 => Err(vk::Result::ERROR_FRAGMENTED_POOL),
                        _ => Ok(301),
                    }
                },
            )
            .unwrap();
        assert_eq!(set, 301);
        assert_eq!(retired.len(), 2);
        assert_eq!(*lease.pool.as_ref().unwrap(), 3);
        assert_eq!(
            *events.borrow(),
            [
                "create:1",
                "allocate:1",
                "create:2",
                "allocate:2",
                "create:3",
                "allocate:3"
            ]
        );
        let (second, _) = queue
            .allocate(
                &mut retired,
                || panic!("现有池仍可分配"),
                |_| panic!("活跃池不能 reset"),
                |_| Ok(302),
            )
            .unwrap();
        assert_ne!(set, second);
    }

    #[test]
    fn caller_and_set_owners_delay_pool_return_then_refill_is_lifo() {
        let mut queue = DescriptorPoolQueue::<u32>::default();
        let mut retired = Vec::new();
        let (_, first_owner) = queue
            .allocate(&mut retired, || Ok(1), |_| Ok(()), |_| Ok(11))
            .unwrap();
        let (_, second_owner) = queue
            .allocate(
                &mut retired,
                || Ok(2),
                |_| Ok(()),
                |pool| {
                    if *pool == 1 {
                        Err(vk::Result::ERROR_OUT_OF_POOL_MEMORY)
                    } else {
                        Ok(22)
                    }
                },
            )
            .unwrap();
        retired.clear();
        assert!(queue.source.returned.lock().is_empty());
        drop(first_owner);
        assert_eq!(*queue.source.returned.lock(), [1]);
        queue.pools.clear();
        // 对应调用方提交完成后释放最后集合；否则第二个池仍不能进入来源。
        drop(second_owner);
        assert_eq!(*queue.source.returned.lock(), [1, 2]);
        let resets = RefCell::new(Vec::new());
        let (set, _) = queue
            .allocate(
                &mut retired,
                || panic!("来源已有整批空闲池"),
                |pool| {
                    resets.borrow_mut().push(*pool);
                    Ok(())
                },
                |pool| Ok(*pool * 100),
            )
            .unwrap();
        assert_eq!(*resets.borrow(), [1, 2]);
        assert_eq!(set, 200);
    }

    #[test]
    fn other_vulkan_error_keeps_current_pool_and_does_not_retry() {
        let mut queue = DescriptorPoolQueue::<u32>::default();
        let mut retired = Vec::new();
        let result = queue.allocate::<u32>(
            &mut retired,
            || Ok(1),
            |_| Ok(()),
            |_| Err(vk::Result::ERROR_DEVICE_LOST),
        );
        assert!(matches!(result, Err(vk::Result::ERROR_DEVICE_LOST)));
        assert!(retired.is_empty());
        let (set, _) = queue
            .allocate(
                &mut retired,
                || panic!("非容量错误没有弹池"),
                |_| panic!("不 reset"),
                |_| Ok(12),
            )
            .unwrap();
        assert_eq!(set, 12);
    }

    #[test]
    fn refill_failure_preserves_already_retired_pool_for_caller() {
        let mut queue = DescriptorPoolQueue::<u32>::default();
        let mut retired = Vec::new();
        let first = Cell::new(true);
        let result = queue.allocate::<u32>(
            &mut retired,
            || {
                if first.replace(false) {
                    Ok(1)
                } else {
                    Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY)
                }
            },
            |_| Ok(()),
            |_| Err(vk::Result::ERROR_OUT_OF_POOL_MEMORY),
        );
        assert!(matches!(
            result,
            Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY)
        ));
        assert_eq!(retired.len(), 1);
        assert!(queue.source.returned.lock().is_empty());
    }
}

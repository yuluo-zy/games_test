// 候选实现迁移来源：reconstruction/render/work-packages/device-counters-r6/src/lib.rs.
// 这是行为重建，不是原作者源码。
use ash::vk;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// 记录实际启用的功能，不能以驱动支持代替设备已启用。
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceCapabilities {
    pub buffer_device_address: bool,
    pub wide_lines: bool,
    pub runtime_descriptor_array: bool,
    pub descriptor_binding_partially_bound: bool,
    pub descriptor_binding_variable_descriptor_count: bool,
    pub sampled_image_update_after_bind: bool,
    pub storage_buffer_update_after_bind: bool,
    /// 实际启用的采样图片数组非一致索引，不能由 runtime array 推断。
    pub shader_sampled_image_array_non_uniform_indexing: bool,
    /// runtime sampled-image 布局 flags15 所需的实际启用特性。
    pub descriptor_binding_update_unused_while_pending: bool,
}

/// 实际 Vulkan 设备适配。原作 GraphicsDevice 的完整布局不使用此候选布局替代。
pub struct VulkanDevice {
    pub device: ash::Device,
    pub physical: vk::PhysicalDevice,
    pub memory: vk::PhysicalDeviceMemoryProperties,
    pub queue: vk::Queue,
    pub queue_family: u32,
    pub instance: Arc<super::instance::VulkanInstance>,
    pub queue_lock: Mutex<()>,
    pub capabilities: DeviceCapabilities,
    pub additional_queues: Vec<DeviceQueue>,
    pub allocator_stats: Arc<super::allocator::AllocatorStats>,
    pub graveyard: super::graveyard::Graveyard,
    /// 启用子分配时的 gpu-allocator 适配；None 表示每资源一块专用内存。
    #[cfg(feature = "candidate-suballocation")]
    pub suballocator: Option<super::suballoc::SubAllocator>,
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceQueue {
    pub handle: vk::Queue,
    pub family: u32,
    pub index: u32,
}

impl VulkanDevice {
    #[cfg(not(feature = "candidate-suballocation"))]
    pub fn suballocation_enabled(&self) -> bool {
        false
    }
    #[cfg(not(feature = "candidate-suballocation"))]
    pub fn memory_block_count(&self) -> usize {
        self.allocator_stats.report().allocations.len()
    }
    /// 原 runtime descriptor 上限来自该实际 properties2 字段；减32和8192截断由布局规则执行。
    pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
        let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
        unsafe {
            self.instance
                .instance
                .get_physical_device_properties2(self.physical, &mut properties);
        }
        indexing.max_descriptor_set_update_after_bind_sampled_images
    }
    /// 接收调用者选择的队列族与显式特性；默认配置由 init 模块提供。
    pub fn create(
        instance: Arc<super::instance::VulkanInstance>,
        physical: vk::PhysicalDevice,
        queue_family: u32,
        info: &vk::DeviceCreateInfo<'_>,
    ) -> Result<Arc<Self>, vk::Result> {
        unsafe {
            let mut capabilities = DeviceCapabilities::default();
            if !info.p_enabled_features.is_null() {
                capabilities.wide_lines = (*info.p_enabled_features).wide_lines != 0;
            }
            let mut feature = info.p_next.cast::<vk::BaseInStructure<'_>>();
            while !feature.is_null() {
                match (*feature).s_type {
                    vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => {
                        let enabled = &*feature.cast::<vk::PhysicalDeviceVulkan12Features<'_>>();
                        capabilities.buffer_device_address = enabled.buffer_device_address != 0;
                        capabilities.runtime_descriptor_array =
                            enabled.runtime_descriptor_array != 0;
                        capabilities.descriptor_binding_partially_bound =
                            enabled.descriptor_binding_partially_bound != 0;
                        capabilities.descriptor_binding_variable_descriptor_count =
                            enabled.descriptor_binding_variable_descriptor_count != 0;
                        capabilities.sampled_image_update_after_bind =
                            enabled.descriptor_binding_sampled_image_update_after_bind != 0;
                        capabilities.storage_buffer_update_after_bind =
                            enabled.descriptor_binding_storage_buffer_update_after_bind != 0;
                        capabilities.shader_sampled_image_array_non_uniform_indexing =
                            enabled.shader_sampled_image_array_non_uniform_indexing != 0;
                        capabilities.descriptor_binding_update_unused_while_pending =
                            enabled.descriptor_binding_update_unused_while_pending != 0;
                    }
                    vk::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => {
                        let enabled =
                            &*feature.cast::<vk::PhysicalDeviceDescriptorIndexingFeatures<'_>>();
                        capabilities.runtime_descriptor_array =
                            enabled.runtime_descriptor_array != 0;
                        capabilities.descriptor_binding_partially_bound =
                            enabled.descriptor_binding_partially_bound != 0;
                        capabilities.descriptor_binding_variable_descriptor_count =
                            enabled.descriptor_binding_variable_descriptor_count != 0;
                        capabilities.sampled_image_update_after_bind =
                            enabled.descriptor_binding_sampled_image_update_after_bind != 0;
                        capabilities.storage_buffer_update_after_bind =
                            enabled.descriptor_binding_storage_buffer_update_after_bind != 0;
                        capabilities.shader_sampled_image_array_non_uniform_indexing =
                            enabled.shader_sampled_image_array_non_uniform_indexing != 0;
                        capabilities.descriptor_binding_update_unused_while_pending =
                            enabled.descriptor_binding_update_unused_while_pending != 0;
                    }
                    vk::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => {
                        capabilities.buffer_device_address =
                            (*feature.cast::<vk::PhysicalDeviceBufferDeviceAddressFeatures<'_>>())
                                .buffer_device_address
                                != 0;
                    }
                    _ => {}
                }
                feature = (*feature).p_next;
            }
            let device = instance.instance.create_device(physical, info, None)?;
            let queue = device.get_device_queue(queue_family, 0);
            let memory = instance
                .instance
                .get_physical_device_memory_properties(physical);
            let requested = std::slice::from_raw_parts(
                info.p_queue_create_infos,
                info.queue_create_info_count as usize,
            );
            let mut additional_queues = Vec::new();
            for request in requested {
                for index in 0..request.queue_count {
                    if request.queue_family_index == queue_family && index == 0 {
                        continue;
                    }
                    additional_queues.push(DeviceQueue {
                        handle: device.get_device_queue(request.queue_family_index, index),
                        family: request.queue_family_index,
                        index,
                    });
                }
            }
            #[cfg(feature = "candidate-suballocation")]
            let suballocator = match super::suballoc::default_allocation_mode() {
                super::suballoc::AllocationMode::Dedicated => None,
                super::suballoc::AllocationMode::Suballocated => Some(
                    super::suballoc::SubAllocator::new(
                        &instance.instance,
                        &device,
                        physical,
                        capabilities.buffer_device_address,
                    )
                    .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?,
                ),
            };
            Ok(Arc::new(Self {
                device,
                physical,
                memory,
                queue,
                queue_family,
                instance,
                queue_lock: Mutex::new(()),
                capabilities,
                additional_queues,
                allocator_stats: Arc::new(super::allocator::AllocatorStats::default()),
                graveyard: Default::default(),
                #[cfg(feature = "candidate-suballocation")]
                suballocator,
            }))
        }
    }

    pub fn wait_idle(&self) -> Result<(), vk::Result> {
        self.wait_idle_and_collect().map(|_| ())
    }
}

impl Drop for VulkanDevice {
    fn drop(&mut self) {
        // 所有拥有的 Vulkan 子对象都持有设备 Arc，因此会先于设备销毁；延迟场在此排空。
        self.drain_graveyard_for_drop();
        // 子分配器的内存块必须先于设备销毁释放。
        #[cfg(feature = "candidate-suballocation")]
        if let Some(sub) = &self.suballocator {
            sub.shutdown();
        }
        unsafe {
            let _ = self.device.device_wait_idle();
            self.device.destroy_device(None);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueIdentity {
    Primary,
    Auxiliary(u32),
}

impl QueueIdentity {
    pub fn counter_slot(self) -> usize {
        match self {
            Self::Primary => 0,
            Self::Auxiliary(index) => index.wrapping_add(1) as usize,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueFamily {
    pub flags: u32,
    pub queue_count: u32,
    pub family_index: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueuePlan {
    pub primary_family: u32,
    pub auxiliary_family: Option<u32>,
    pub auxiliary_count: u32,
}

impl QueuePlan {
    pub fn select(families: &[QueueFamily]) -> Option<Self> {
        let primary = families.iter().find(|family| family.flags & 1 != 0)?;
        let auxiliary = families.iter().find(|family| family.flags & 3 == 2);
        Some(Self {
            primary_family: primary.family_index,
            auxiliary_family: auxiliary.map(|family| family.family_index),
            auxiliary_count: auxiliary.map_or(0, |family| family.queue_count),
        })
    }

    pub fn counters(&self) -> DeviceCounters {
        DeviceCounters::new(self.auxiliary_count)
    }
}

pub struct DeviceCounters {
    slots: Box<[AtomicU64]>,
}

impl DeviceCounters {
    pub fn new(auxiliary_count: u32) -> Self {
        let slots = (0..u64::from(auxiliary_count) + 1)
            .map(|_| AtomicU64::new(1))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { slots }
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn begin_sequence(&self, queue: QueueIdentity) -> Option<u64> {
        self.slots
            .get(queue.counter_slot())
            .map(|counter| counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn next_value(&self, queue: QueueIdentity) -> Option<u64> {
        self.slots
            .get(queue.counter_slot())
            .map(|counter| counter.load(Ordering::Relaxed))
    }
}

pub fn raw_begin_projection(
    counters: &mut [u64],
    queue_tag: u32,
    auxiliary_index: u32,
) -> Option<u64> {
    let slot = if queue_tag & 1 == 0 {
        0
    } else {
        auxiliary_index.wrapping_add(1) as usize
    };
    counters.get_mut(slot).map(|counter| {
        let sequence = *counter;
        *counter = counter.wrapping_add(1);
        sequence
    })
}

#[derive(Default)]
pub struct BucketKeyWitness {
    live: BTreeMap<u64, QueueIdentity>,
}

impl BucketKeyWitness {
    pub fn insert(&mut self, sequence: u64, queue: QueueIdentity) -> Option<QueueIdentity> {
        self.live.insert(sequence, queue)
    }

    pub fn remove(&mut self, sequence: u64) -> Option<QueueIdentity> {
        self.live.remove(&sequence)
    }
}

/// 原 `GraphicsDevice` 的候选包装：持有真实 Vulkan 设备、属性快照与内置资源，
/// 把原方法名委托到现有 raw 类型。字段布局不声称等于原 ABI。
pub struct GraphicsDevice {
    pub inner: Arc<VulkanDevice>,
    properties: vk::PhysicalDeviceProperties,
    builtin_samplers: Vec<Arc<super::sampler::Sampler>>,
    seq: super::command_buffer::CommandBufferSeqTracker,
    disposal: Mutex<super::disposal::CommandBufferDisposalBuckets>,
}
impl GraphicsDevice {
    /// 从已选物理设备与队列族构造；默认启用 BDA 与 descriptor indexing 特性。
    pub fn create(
        instance: Arc<super::instance::VulkanInstance>,
        physical: vk::PhysicalDevice,
        queue_family: u32,
    ) -> Result<Arc<Self>, vk::Result> {
        let mut features2 = vk::PhysicalDeviceFeatures2::default();
        let mut vk12 = vk::PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true)
            .runtime_descriptor_array(true)
            .descriptor_binding_partially_bound(true)
            .descriptor_binding_variable_descriptor_count(true)
            .descriptor_binding_sampled_image_update_after_bind(true)
            .descriptor_binding_storage_buffer_update_after_bind(true);
        features2 = features2.push_next(&mut vk12);
        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&[1.0]);
        let extension_names = [
            "VK_KHR_swapchain",
            "VK_KHR_timeline_semaphore",
            "VK_EXT_descriptor_indexing",
            "VK_KHR_buffer_device_address",
        ]
        .map(std::ffi::CString::new)
        .map(Result::unwrap);
        let extensions = extension_names
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<_>>();
        let queue_infos = [queue_info];
        let info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&extensions)
            .push_next(&mut features2);
        let device = VulkanDevice::create(instance, physical, queue_family, &info)?;
        Ok(Self::from_device(device))
    }
    /// 从已构造设备包装。
    pub fn from_device(device: Arc<VulkanDevice>) -> Arc<Self> {
        let properties = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        };
        Arc::new(Self {
            inner: device,
            properties,
            builtin_samplers: Vec::new(),
            seq: super::command_buffer::CommandBufferSeqTracker::new(),
            disposal: Mutex::new(super::disposal::CommandBufferDisposalBuckets::default()),
        })
    }
    pub fn device_uuid(&self) -> [u8; 16] {
        self.properties.pipeline_cache_uuid
    }
    pub fn limits(&self) -> &vk::PhysicalDeviceLimits {
        &self.properties.limits
    }
    /// 子组大小下限；需要 `subgroup_size_control` 特性时读取 properties2，否则用已知较小值。
    pub fn min_subgroup_size(&self) -> u32 {
        // subgroup_size_control 未显式启用时不读 properties2 字段；候选返回通用下限。
        1
    }
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.inner.physical
    }
    pub fn queue_family_index(&self) -> u32 {
        self.inner.queue_family
    }
    pub fn debug_utils(&self) -> super::debug::DebugUtils {
        super::debug::DebugUtils::new(&self.inner.instance.instance, self.inner.clone())
    }
    /// 原 bindless 运行时描述符上限。
    pub fn max_bindless_descriptor_count(&self) -> u32 {
        self.inner
            .max_descriptor_set_update_after_bind_sampled_images()
    }
    /// 本构建无硬件光追；返回 None 表示该路径不可用。
    pub fn rt(&self) -> Option<()> {
        None
    }
    /// 设备完成并排空已注册的销毁桶。
    pub fn wait_idle_and_recycle_resources(&mut self) -> Result<(), vk::Result> {
        self.inner.wait_idle()?;
        let mut disposal = self.disposal.lock();
        let keys = disposal.pending_sequences();
        for key in keys {
            let _ = disposal.immediate_empty_bucket(key);
        }
        Ok(())
    }
    pub fn allocate_command_buffer_seq(&mut self) -> u64 {
        self.seq.track()
    }
    pub fn command_buffer_seq(&self) -> &super::command_buffer::CommandBufferSeqTracker {
        &self.seq
    }
    pub fn disposal(
        &self,
    ) -> parking_lot::MutexGuard<'_, super::disposal::CommandBufferDisposalBuckets> {
        self.disposal.lock()
    }
    /// 创建可映射（HOST_VISIBLE）缓冲。
    pub fn create_mapped_buffer(
        &self,
        size: u64,
        usage: vk::BufferUsageFlags,
    ) -> super::error::Result<super::buffer::Buffer> {
        super::buffer::Buffer::new(
            self.inner.clone(),
            &super::buffer::BufferDesc::new_cpu_to_gpu(size, usage),
        )
    }
    pub fn create_buffer_impl(
        &self,
        desc: &super::buffer::BufferDesc,
    ) -> super::error::Result<super::buffer::Buffer> {
        super::buffer::Buffer::new(self.inner.clone(), desc)
    }
    /// 创建 GPU 分析输出缓冲。
    pub fn create_gpu_profiler_output_buffer(
        &self,
        size: u64,
    ) -> super::error::Result<super::buffer::Buffer> {
        super::buffer::Buffer::new(
            self.inner.clone(),
            &super::buffer::BufferDesc::new_gpu_to_cpu(
                size,
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
    }
    /// 初始化纹理数据：经暂存缓冲 + setup 命令缓冲同步上传（见 `raw::setup`）。
    pub fn init_texture_data(
        &self,
        texture: &super::texture::Texture,
        bytes: &[u8],
    ) -> super::error::Result<()> {
        super::setup::init_texture_data(&self.inner, texture, bytes)
    }
    pub fn update_texture(
        &self,
        texture: &super::texture::Texture,
        bytes: &[u8],
    ) -> super::error::Result<()> {
        super::setup::update_texture(&self.inner, texture, bytes)
    }
    /// 创建一组内置采样器（最近邻/线性/深度比较）。
    pub fn create_samplers(&mut self) -> Result<(), vk::Result> {
        let device = self.inner.clone();
        let nearest = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::NEAREST)
            .min_filter(vk::Filter::NEAREST)
            .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT);
        let linear = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT);
        let depth = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .compare_enable(true)
            .compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);
        self.builtin_samplers = vec![
            Arc::new(
                super::sampler::Sampler::new(device.clone(), &nearest)
                    .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?,
            ),
            Arc::new(
                super::sampler::Sampler::new(device.clone(), &linear)
                    .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?,
            ),
            Arc::new(
                super::sampler::Sampler::new(device, &depth)
                    .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?,
            ),
        ];
        Ok(())
    }
    /// 返回内置采样器之一；未创建时返回 None。
    pub fn get_built_in_sampler(&self, index: usize) -> Option<Arc<super::sampler::Sampler>> {
        self.builtin_samplers.get(index).cloned()
    }
    /// 查询设备是否支持给定扩展名。
    pub fn query_device_extension_support(&self, names: &[&str]) -> Vec<bool> {
        let available = match unsafe {
            self.inner
                .instance
                .instance
                .enumerate_device_extension_properties(self.inner.physical)
        } {
            Ok(extensions) => extensions
                .iter()
                .map(|e| {
                    unsafe { std::ffi::CStr::from_ptr(e.extension_name.as_ptr()) }
                        .to_string_lossy()
                        .into_owned()
                })
                .collect::<Vec<_>>(),
            Err(_) => Vec::new(),
        };
        names
            .iter()
            .map(|name| available.iter().any(|have| have == name))
            .collect()
    }
    /// 校验物理设备需求（内存/队列族/特性）；候选只做最基础的堆检查。
    pub fn validate_physical_device_requirements(&self, min_device_local_bytes: u64) -> bool {
        let memory = unsafe {
            self.inner
                .instance
                .instance
                .get_physical_device_memory_properties(self.inner.physical)
        };
        let device_local = memory.memory_heaps[..memory.memory_heap_count as usize]
            .iter()
            .filter(|heap| heap.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL))
            .map(|heap| heap.size)
            .sum::<u64>();
        device_local >= min_device_local_bytes
    }
    /// 全局分配器包装：执行 `body` 并附带分配器统计。候选仅返回统计快照。
    pub fn with_global_allocator<R>(
        &self,
        body: impl FnOnce(&super::allocator::AllocatorStats) -> R,
    ) -> R {
        body(&self.inner.allocator_stats)
    }
    /// 下发一次管线屏障（memory/buffer/image 三类数组按 scope 汇总）。
    pub fn issue_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
        scope: &super::barrier::SynchronizationScope,
    ) {
        unsafe {
            self.inner.device.cmd_pipeline_barrier(
                command,
                scope.source_stages,
                scope.destination_stages,
                vk::DependencyFlags::empty(),
                &scope.memory,
                &scope.buffers,
                &scope.images,
            )
        }
    }
    /// 下发后立即清空 scope，供下一批复用。
    pub fn issue_and_reset_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
        scope: &mut super::barrier::SynchronizationScope,
    ) {
        self.issue_pipeline_barrier(command, scope);
        scope.memory.clear();
        scope.buffers.clear();
        scope.images.clear();
    }
    /// 全同步屏障（所有命令→所有命令）。
    pub fn issue_full_pipeline_barrier(&self, command: vk::CommandBuffer) {
        unsafe {
            self.inner.device.cmd_pipeline_barrier(
                command,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[],
            )
        }
    }
    /// 等待单个 fence 完成。
    pub fn wait_submit_done(&self, fence: vk::Fence, timeout: u64) -> Result<(), vk::Result> {
        unsafe { self.inner.device.wait_for_fences(&[fence], true, timeout) }
    }
    /// 有界等待直到提交完成或超时；返回 true 表示完成。
    pub fn wait_duration_or_until_submit_done(
        &self,
        fence: vk::Fence,
        timeout: u64,
    ) -> Result<bool, vk::Result> {
        match unsafe { self.inner.device.wait_for_fences(&[fence], true, timeout) } {
            Ok(()) => Ok(true),
            Err(vk::Result::TIMEOUT) => Ok(false),
            Err(error) => Err(error),
        }
    }
    /// 分配持久 descriptor set（candidate：按 writes 直接分配）。
    pub fn create_persistent_descriptor_set(
        &self,
        state: &super::descriptor_set::DescriptorSetState,
        writes: &[super::descriptor_set::DescriptorWrite],
    ) -> Result<std::sync::Arc<super::descriptor_set::VulkanDescriptorSet>, vk::Result> {
        unsafe { state.allocate_descriptor_set(writes) }
    }
    /// 在 setup 命令缓冲上录制并同步执行一次回调；结尾自动补全量内存屏障。
    pub fn with_setup_cb<R>(
        &self,
        body: impl FnOnce(vk::CommandBuffer) -> R,
    ) -> super::error::Result<R> {
        super::setup::with_setup_cb(&self.inner, body)
    }
}

/// 队列句柄的候选包装；把提交引擎方法按队列暴露。
#[derive(Clone)]
pub struct Queue {
    pub handle: vk::Queue,
    pub family: u32,
    pub index: u32,
}
impl Queue {
    pub fn with_raw(handle: vk::Queue, family: u32, index: u32) -> Self {
        Self {
            handle,
            family,
            index,
        }
    }
    pub fn update_pending_submissions(&self, _engine: &mut super::submit::SubmissionEngine) {
        // 队列的待处理推进统一由提交引擎按 timeline 处理。
    }
    pub fn reclaim_command_buffer_resources(
        &self,
        engine: &mut super::submit::SubmissionEngine,
        completed: u64,
    ) -> Vec<vk::CommandBuffer> {
        engine.reclaim_command_buffer_resources_all_queues(completed)
    }
}

/// 原自由入口：完整创建 GraphicsDevice。
pub fn create_raw_device(
    instance: Arc<super::instance::VulkanInstance>,
    physical: vk::PhysicalDevice,
    queue_family: u32,
) -> Result<Arc<GraphicsDevice>, vk::Result> {
    GraphicsDevice::create(instance, physical, queue_family)
}

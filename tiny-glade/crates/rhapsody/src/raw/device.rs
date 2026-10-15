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
    /// 创建时实际启用的设备扩展名（`vkCreateDevice` 的 `ppEnabledExtensionNames`）。
    pub enabled_extensions: Vec<String>,
    pub additional_queues: Vec<DeviceQueue>,
    pub allocator_stats: Arc<super::allocator::AllocatorStats>,
    pub graveyard: super::graveyard::Graveyard,
    /// 退役时间线：每次主队列提交末尾 signal 到该次的 epoch，供延迟销毁非阻塞判断完成。
    pub(crate) retire_timeline: vk::Semaphore,
    /// 原 `device+0xab0`：累计 `vkUpdateDescriptorSets` 调用次数。四个写入点（`lock inc`）：
    /// 0x1416a26aa（update_persistent_descriptor_set）与 0x1416a56e5 / 0x1416a5ce8 / 0x1416a63f2
    /// （三个 bind_descriptor_set 单态，每次调用一条 vkUpdateDescriptorSets 后 +1）；初始化为 0
    /// （0x140bf631d）。全 .text 扫描未找到读取者（`xchg`/`xadd`/`mov r,[dev+0xab0]` 均无），
    /// 语义按纯统计计数处理；无读取点故不清零。
    pub descriptor_update_count: AtomicU64,
    /// 启用子分配时的 gpu-allocator 适配；None 表示每资源一块专用内存。
    #[cfg(feature = "suballocation")]
    pub suballocator: Option<super::suballoc::SubAllocator>,
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceQueue {
    pub handle: vk::Queue,
    pub family: u32,
    pub index: u32,
}

impl VulkanDevice {
    #[cfg(not(feature = "suballocation"))]
    pub fn suballocation_enabled(&self) -> bool {
        false
    }
    #[cfg(not(feature = "suballocation"))]
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
            let enabled_extensions = if info.pp_enabled_extension_names.is_null() {
                Vec::new()
            } else {
                std::slice::from_raw_parts(
                    info.pp_enabled_extension_names,
                    info.enabled_extension_count as usize,
                )
                .iter()
                .map(|name| std::ffi::CStr::from_ptr(*name).to_string_lossy().into_owned())
                .collect()
            };
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
            #[cfg(feature = "suballocation")]
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
            let mut timeline_type = vk::SemaphoreTypeCreateInfo::default()
                .semaphore_type(vk::SemaphoreType::TIMELINE)
                .initial_value(0);
            let retire_timeline = device.create_semaphore(
                &vk::SemaphoreCreateInfo::default().push_next(&mut timeline_type),
                None,
            )?;
            Ok(Arc::new(Self {
                retire_timeline,
                descriptor_update_count: AtomicU64::new(0),
                device,
                physical,
                memory,
                queue,
                queue_family,
                instance,
                queue_lock: Mutex::new(()),
                capabilities,
                enabled_extensions,
                additional_queues,
                allocator_stats: Arc::new(super::allocator::AllocatorStats::default()),
                graveyard: Default::default(),
                #[cfg(feature = "suballocation")]
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
        #[cfg(feature = "suballocation")]
        if let Some(sub) = &self.suballocator {
            sub.shutdown();
        }
        unsafe {
            let _ = self.device.device_wait_idle();
            self.device.destroy_semaphore(self.retire_timeline, None);
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
///
/// 原版 `GraphicsDevice` 真实字段布局（机器码取证，**仅作记录，本结构不照搬**）：
///
/// | 偏移 | 内容 | 证据 |
/// |---|---|---|
/// | `+0x1a` | u8：VK_EXT_memory_budget 门控（`refresh_memory_budget` 入口） | 0x140bf8f80 |
/// | `+0x20..+0x3f` | **`Option<QueueFamilyInfo>`（32B）= 第二（compute-only）队列族**：`+0x20` u32 tag（1=Some，读者只测 bit0）、`+0x24..+0x3b` 族属性记录（flags/queueCount/timestampValidBits/minImageTransferGranularity）、`+0x3c` u32 族索引。**写入点已证实**：`GraphicsDevice::create` 0x140bf2e10..0x140bf2e48 对物理设备族属性数组（0x1c 步长，`+0x658`→`+0x38/+0x40`）找首个 `flags & 3 == 2`（compute 且非 graphics），拷 0x1c 记录到栈 `[rbp+0x2fa4..0x2fc0)` 并置 `[rbp+0x2fa0]=1`（无则 0）；尾段 0x140bf6388..0x140bf639a 以两条 `movups` 把该 32B 整块写入 `dev+0x20/+0x30`。读者 0x140bfc750 | 0x140bf2560 / 0x140bfc750 |
/// | `+0x40/+0x44` | `Option<u32>` min_subgroup_size（标志/值） | 0x140bf8fa0 |
/// | `+0x78` | ash `Device` 函数表（`+0x98` vkDeviceWaitIdle、`+0x298` vkCreateCommandPool、`+0x2b0` vkAllocateCommandBuffers、`+0x2c0/+0x2c8/+0x2d0` begin/end/reset、`+0x3e0` vkCmdPipelineBarrier；`+0x88` vkQueueSubmit、`+0x370` vkCmdCopyBuffer、`+0x4f0` vkGetSemaphoreCounterValue、`+0x4f8` vkWaitSemaphores、`+0x640` VkDevice；**wave3 已用 ash 0.38.0 `DeviceFnV1_0/1_1/1_2` 字段偏移逐项核对**：`+0x78` 起是 `ash::Device` 本体——V1_0@+0x78（120 项）、V1_1@+0x438、V1_2@+0x4b8、V1_3@+0x520、句柄@+0x640（=ash 内相对偏移 `0x5c8`，故 `0x140bf0b90` 以 ash `Device` 为参数时见到 `+0x478`/`+0x5c8`，即 `dev+0x4f0`/`dev+0x640`；旧表把 `+0x478` 当 dev 绝对偏移是笔误）） | 0x140bed050/0x140bed230/0x140bf0b90/0x140bfd30d/0x140bfcab3 |
/// | `+0x188` | `HashMap<RenderPassDesc, RenderPass>` | create_render_pass |
/// | `+0x5c8` | 时间线语义用的 VkDevice 副本（`vkGetSemaphoreCounterValue` 第一参） | 0x140bf0b90 |
/// | `+0x640` | `VkDevice` | 0x140bed050 / 0x140bf8980 |
/// | `+0x648` | 托管资源上下文（`ManagedResources`；`+0xa50` 即 owning_device 代号） | create_buffer 校验 |
/// | `+0x658` | 物理设备包装指针（`+0x10` = PhysicalDevice、`+0x38/+0x40` = 队列族属性数组 0x1c 步长、`+0x690` = pipeline_cache_uuid） | 0x140bf84c0/0x140bef4a0 |
/// | `+0x668` | 每队列命令序号数组（`begin` 取序号） | 0x140bed230 |
/// | `+0x6d0` | 队列操作分派（`Queue::with_raw`） | 0x140c03d70 |
/// | `+0x718` | 提交引擎自旋锁（parking_lot 字节锁） | 0x140bf8e60 |
/// | `+0x720` | `SubmissionEngine`（图形 Queue@+0x730，Vec<Queue>@+0x7e8/+0x7f0，stride 0xb0） | 0x140bf8e60 |
/// | `+0x800` | 提交读写锁（`RwLock<()>`：`wait_idle` 取写锁 0→8，`queue_submit` 取读锁） | 0x140bf8980 |
/// | `+0x870/+0x8b0` | debug_utils 命名回调 / 其上下文（空 = 未启用） | 0x140bf8960/0x140bef4a0 |
/// | `+0x9b0` | `CommandBufferPool` 字节锁；`+0x9c0` 数组，`+0x9c8` 长度（0x18B 元素 = `Vec<CommandBuffer>`） | 0x140bef4a0/0x140bed4c0 |
/// | `+0x9d0` | 回收用第二把锁（`reclaim_command_buffer_resources` 初始化 `pcStack_110`） | 0x140bf0cc0 |
/// | `+0x9f0/+0x9f8..+0xa4f` | setup CB 锁 / 内联记录（`get_setup_cb_submit_info` 读） | 0x140bf84e0 |
/// | `+0xa00..+0xa4f` | setup CB 槽载荷（句柄 `+0xa40`；**不是** wait/signal 模板，wave2 更正）；创建尾段初值 `{+0x9f0 锁=0, +0x9f8 状态=4(None)}` | 0x140bf2560 / 0x140bf84e0 |
/// | `+0xa50` | owning_device 代号（u16） | 0x140bed050 |
/// | `+0xad4` | 主队列族索引（u32）；`+0xaba` 全屏障开关 | 0x140bfc750/0x140bee090 |
pub struct GraphicsDevice {
    pub inner: Arc<VulkanDevice>,
    properties: vk::PhysicalDeviceProperties,
    /// `+0x40/+0x44`：`Option<u32>` 最小子组大小（`VK_EXT_subgroup_size_control` 可用时才有值）。
    min_subgroup_size: Option<u32>,
    builtin_samplers: Vec<Arc<super::sampler::Sampler>>,
    seq: super::command_buffer::CommandBufferSeqTracker,
    /// `+0xa50`：命令缓冲销毁桶表（`RwLock<HashMap>` 自带内部锁，不再套外层 Mutex）。
    /// `Arc` 仅用于与设备内的 `ManagedResources` 共享（原版 `ManagedResources` 经 `&GraphicsDevice` 参数访问）。
    disposal: Arc<super::disposal::CommandBufferDisposalBuckets>,
    /// `+0x9b0/+0x9c0/+0x9c8`：按队列族索引的命令缓冲空闲链。
    command_pool: super::command_buffer::CommandBufferPool,
    /// `+0x718/+0x720`：提交引擎（由图执行器挂接；未挂接时 `wait_idle_and_recycle_resources` 只做前两步）。
    submission: Mutex<Option<super::submit::SubmissionEngine>>,
    /// 设备创建尾段（0x140bf2560 段 E）建的管线缓存（空初始数据）。
    pipeline_cache: Option<super::shader::PipelineCache>,
    /// 设备创建尾段（段 G）建的 4 字节 “crash tracking buffer”。
    crash_tracking_buffer: Option<super::buffer::Buffer>,
    /// 物理设备包装（`+0x658`）；由 init 路径挂接，供 `refresh_memory_budget` 委托。
    physical_info: std::sync::OnceLock<Arc<super::physical_device::PhysicalDevice>>,
    /// 设备持有的托管资源槽位（原版句柄所指的存储）。
    pub resources: super::managed::resources::ManagedResources,
    /// 原 `device+0x188` 的 `HashMap<RenderPassDesc, RenderPass>`：渲染通道按描述缓存，
    /// 缓存本身持有一个句柄引用，设备存活期内不退役。
    pub(crate) render_pass_cache: Mutex<
        std::collections::HashMap<
            super::render_pass::RenderPassDesc,
            super::managed::render_pass::ManagedRenderPass,
        >,
    >,
    /// 原 `dev+0x9f0..0xa4f` 的 setup 命令缓冲槽（`Mutex<Option<CB>>`，创建尾段写 `{0,4}`）。
    /// `queue_submit`（0x140c04e20）取走它并附一个 ALL_COMMANDS 全局屏障；这里只持有槽，
    /// 真实提交路径仍走 `VulkanCommandBuffer::submit_to`（见 reports/wave2-queue-submit.md §4）。
    pub setup_slot: Mutex<super::submit::SetupCommandSlot>,
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
        // "Created a Vulkan device"（0x142b160a0，原版 trace 级，门控 `*0x14362c3a8 > 4`）。
        log::trace!(target: "rhapsody::raw::device", "Created a Vulkan device");
        Self::finish_creation(device)
    }
    /// 从已构造设备包装（不跑创建尾段：不建管线缓存与 crash tracking buffer）。
    pub fn from_device(device: Arc<VulkanDevice>) -> Arc<Self> {
        Self::build(device, None, None)
    }
    /// `GraphicsDevice::create`（0x140bf2560）尾段 D–G，顺序按反编译：
    /// D 分配器初始化日志（memblock 大小 / 内存属性 / 预算 / `Initialized gpu-allocator`）→
    /// G `crash tracking buffer`（4 字节）→ E `vkCreatePipelineCache`（空初始数据）→
    /// `Backend device fully initialized.`。
    pub(crate) fn finish_creation(device: Arc<VulkanDevice>) -> Result<Arc<Self>, vk::Result> {
        Self::log_allocator_initialization(&device);
        let crash = super::buffer::Buffer::new(
            device.clone(),
            &super::buffer::BufferDesc::new_gpu_to_cpu(4, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .map_err(|error| {
            log::error!(target: "rhapsody::raw::device", "crash tracking buffer: {error:?}");
            vk::Result::ERROR_INITIALIZATION_FAILED
        })?;
        let cache = super::shader::PipelineCache::new(device.clone(), &[]).map_err(|error| {
            log::error!(target: "rhapsody::raw::device", "vkCreatePipelineCache: {error:?}");
            error
        })?;
        let this = Self::build(device, Some(cache), Some(crash));
        log::trace!(target: "rhapsody::raw::device", "Backend device fully initialized.");
        Ok(this)
    }
    /// 段 D 的日志：`device_memblock_size: {}MB, host_memblock_size: {}MB`（debug，`> 3`）、
    /// `Device memory properties: {:?}`、`Device memory budget: {:?}`（debug）、`Initialized gpu-allocator`（trace）。
    ///
    /// ⚠️ 原版 memblock 大小由 0x140c00530（按堆求和）与某字段 `>>1` 的除数换算、向上取整到 4 MiB，
    /// 其输入字段 `+0x224` 未能还原；这里记录的是本重建**实际使用**的 gpu-allocator 默认块大小，
    /// 不是原版公式的结果。
    fn log_allocator_initialization(device: &Arc<VulkanDevice>) {
        const DEFAULT_DEVICE_MEMBLOCK_MB: u64 = 256;
        const DEFAULT_HOST_MEMBLOCK_MB: u64 = 64;
        log::debug!(
            target: "rhapsody::raw::device",
            "device_memblock_size: {DEFAULT_DEVICE_MEMBLOCK_MB}MB, host_memblock_size: {DEFAULT_HOST_MEMBLOCK_MB}MB"
        );
        let heap_count = device.memory.memory_heap_count as usize;
        log::debug!(
            target: "rhapsody::raw::device",
            "Device memory properties: {:?}",
            &device.memory.memory_heaps[..heap_count]
        );
        if device.enabled_extensions.iter().any(|e| e == "VK_EXT_memory_budget") {
            let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
            let mut properties = vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut budget);
            unsafe {
                device
                    .instance
                    .instance
                    .get_physical_device_memory_properties2(device.physical, &mut properties);
            }
            log::debug!(
                target: "rhapsody::raw::device",
                "Device memory budget: budget={:?} usage={:?}",
                &budget.heap_budget[..heap_count],
                &budget.heap_usage[..heap_count]
            );
        }
        log::trace!(target: "rhapsody::raw::device", "Initialized gpu-allocator");
    }
    fn build(
        device: Arc<VulkanDevice>,
        pipeline_cache: Option<super::shader::PipelineCache>,
        crash_tracking_buffer: Option<super::buffer::Buffer>,
    ) -> Arc<Self> {
        let properties = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        };
        let family_count = unsafe {
            device
                .instance
                .instance
                .get_physical_device_queue_family_properties(device.physical)
        }
        .len();
        let min_subgroup_size = Self::query_min_subgroup_size(&device);
        let disposal = Arc::new(super::disposal::CommandBufferDisposalBuckets::default());
        let mut resources =
            super::managed::resources::ManagedResources::with_device_buckets(disposal.clone());
        resources.set_device(device.clone());
        let mut graphics_device = Self {
            resources,
            inner: device,
            properties,
            min_subgroup_size,
            builtin_samplers: Vec::new(),
            seq: super::command_buffer::CommandBufferSeqTracker::new(),
            disposal,
            command_pool: super::command_buffer::CommandBufferPool::new(family_count),
            submission: Mutex::new(None),
            pipeline_cache,
            crash_tracking_buffer,
            physical_info: std::sync::OnceLock::new(),
            render_pass_cache: Mutex::new(Default::default()),
            setup_slot: Mutex::new(Default::default()),
        };
        // 原版在 `GraphicsDevice::create` 内联创建 13 个内置采样器，失败 `expect("create_sampler")`；
        // 这里失败只记日志（旧的最小设备可能缺 anisotropy），之后按名字取会 panic `Sampler not found`。
        if let Err(error) = graphics_device.create_samplers() {
            log::error!(target: "rhapsody::raw::device", "create_sampler: {error:?}");
        }
        Arc::new(graphics_device)
    }
    /// `VK_EXT_subgroup_size_control` 已启用时读 `min_subgroup_size`，否则 `None`（`+0x40` 标志为 0）。
    fn query_min_subgroup_size(device: &VulkanDevice) -> Option<u32> {
        if !device.enabled_extensions.iter().any(|e| e == "VK_EXT_subgroup_size_control") {
            return None;
        }
        let mut control = vk::PhysicalDeviceSubgroupSizeControlProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut control);
        unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties2(device.physical, &mut properties);
        }
        Some(control.min_subgroup_size)
    }
    /// 0x140bf84c0：从物理设备包装 `+0x690` 取 16 字节（`pipeline_cache_uuid`）。
    pub fn device_uuid(&self) -> [u8; 16] {
        self.properties.pipeline_cache_uuid
    }
    pub fn limits(&self) -> &vk::PhysicalDeviceLimits {
        &self.properties.limits
    }
    /// 0x140bf8fa0：`[dev+0x40]` 标志 / `[dev+0x44]` 值 = `Option<u32>`；
    /// `VK_EXT_subgroup_size_control` 未启用时为 `None`。
    pub fn min_subgroup_size_option(&self) -> Option<u32> {
        self.min_subgroup_size
    }
    /// 旧候选接口（数值版）：有值取原值，否则保持历史行为返回通用下限 1。
    pub fn min_subgroup_size(&self) -> u32 {
        self.min_subgroup_size.unwrap_or(1)
    }
    /// 0x140bf8950：返回物理设备包装内的 `PhysicalDevice`（`*(dev+0x658)+0x10`）；
    /// 本重建里是裸句柄，包装见 [`Self::physical_device_info`]。
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.inner.physical
    }
    /// 物理设备包装；由 init 路径经 [`Self::attach_physical_device_info`] 挂接。
    pub fn physical_device_info(&self) -> Option<&Arc<super::physical_device::PhysicalDevice>> {
        self.physical_info.get()
    }
    /// 挂接物理设备包装（只能设置一次）。
    pub fn attach_physical_device_info(
        &self,
        info: Arc<super::physical_device::PhysicalDevice>,
    ) -> Result<(), Arc<super::physical_device::PhysicalDevice>> {
        self.physical_info.set(info)
    }
    /// 0x140bf8f80：`device+0x1a` 门控（VK_EXT_memory_budget 已启用）→ 尾调 raw-init 的滞回算法
    /// （0x140c01620，即 `PhysicalDevice::refresh_memory_budget`）；门关则直接返回。
    /// 未挂接物理设备包装时无法委托，返回 `Ok(None)`。
    pub fn refresh_memory_budget(
        &self,
    ) -> super::error::Result<Option<vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static>>> {
        if !self.memory_budget_enabled() {
            return Ok(None);
        }
        match self.physical_info.get() {
            Some(physical) => physical.refresh_memory_budget(),
            None => Ok(None),
        }
    }
    /// `+0x1a`：创建时是否启用了 `VK_EXT_memory_budget`。
    pub fn memory_budget_enabled(&self) -> bool {
        self.inner
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_memory_budget")
    }
    /// 0x140bfc750 `queue_family_index(kind)`：`kind` bit0 清 → `dev+0xad4` 主族；
    /// bit0 置 → `dev+0x3c` 第二族，但要求 `dev+0x20` bit0（= `Option<QueueFamilyInfo>` 的 Some 标签，存在第二族），
    /// 否则 panic（`unreachable`，0x142b16c60）。`dev+0x20..+0x3f` 的写入点已在 `create` 内证实（见结构表）：
    /// 首个 `flags & 3 == 2` 的族（与 [`QueuePlan::select`] 一致）。本重建的“第二族” = 首个附加队列的族。
    pub fn queue_family_index_for(&self, kind: u8) -> u32 {
        if kind & 1 == 0 {
            return self.inner.queue_family;
        }
        self.inner
            .additional_queues
            .iter()
            .map(|queue| queue.family)
            .find(|family| *family != self.inner.queue_family)
            .expect("internal error: entered unreachable code")
    }
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index_for(0)
    }
    /// 0x140bf8960：`dev+0x870` 非空才有（`Option<&DebugUtils>`）；本重建以实例是否启用
    /// `VK_EXT_debug_utils` 为准。
    pub fn debug_utils_option(&self) -> Option<super::debug::DebugUtils> {
        self.inner
            .instance
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_debug_utils")
            .then(|| self.debug_utils())
    }
    pub fn debug_utils(&self) -> super::debug::DebugUtils {
        super::debug::DebugUtils::new(&self.inner.instance.instance, self.inner.clone())
    }
    /// 创建尾段建的管线缓存；`from_device` 路径没有。
    pub fn pipeline_cache(&self) -> Option<&super::shader::PipelineCache> {
        self.pipeline_cache.as_ref()
    }
    /// 创建尾段建的 4 字节 “crash tracking buffer”；`from_device` 路径没有。
    pub fn crash_tracking_buffer(&self) -> Option<&super::buffer::Buffer> {
        self.crash_tracking_buffer.as_ref()
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
    /// 0x140bf8980 `wait_idle`：日志 `Waiting for the device to be idle...`（debug）+ 提交锁握手 →
    /// `vkDeviceWaitIdle` → 解锁。原版锁是 `RwLock<()>`（`+0x800`，写锁 0→8）；本重建的
    /// `queue_lock` 互斥量在 `wait_idle_and_collect` 内取得，排他语义不弱于原版。
    pub fn wait_idle(&self) -> Result<(), vk::Result> {
        log::debug!(target: "rhapsody::raw::device", "Waiting for the device to be idle...");
        self.inner.wait_idle()
    }
    /// 挂接提交引擎（原 `+0x720`）。
    pub fn attach_submission_engine(&self, engine: super::submit::SubmissionEngine) {
        *self.submission.lock() = Some(engine);
    }
    /// 在提交引擎锁（原 `+0x718`）内访问引擎。
    pub fn with_submission_engine<R>(
        &self,
        body: impl FnOnce(&mut super::submit::SubmissionEngine) -> R,
    ) -> Option<R> {
        self.submission.lock().as_mut().map(body)
    }
    /// 0x140bf8e60：`wait_idle()` → 退役排水（0x1418c0a70）→ 锁提交引擎 → 推进图形/辅助队列时间线
    /// （0x1418c3300，`vkGetSemaphoreCounterValue`）→ 回收图形 Queue 与每个辅助 Queue 的命令缓冲 → 解锁。
    /// 原版末尾对全局 parking_lot 哈希表（0x1436308f0）的遍历是 `parking_lot` 解锁慢路径的 `unpark`，
    /// 属依赖库实现，不复现（见报告 U#8 闭合）。
    pub fn wait_idle_and_recycle_resources(&self) -> Result<(), vk::Result> {
        self.wait_idle()?;
        // 原 0x1418c0a70 `take_out_dropped_waste(&ManagedResources, &GraphicsDevice)`；
        // 桶本身只由 `retire_in_order`（命令完成序号）排空，不在此整体清空。
        self.resources.take_out_dropped_waste_on(self);
        let device = self.inner.clone();
        if let Some(engine) = self.submission.lock().as_mut() {
            engine.update_queue_pending_submissions_with(|queue| unsafe {
                device
                    .device
                    .get_semaphore_counter_value(queue.timeline_semaphore)
                    .unwrap_or(0)
            });
            engine.reclaim_command_buffer_resources_all_queues(0);
        }
        Ok(())
    }
    /// 0x140bef4a0 `create_command_buffer(family, name)`：持池锁，从 `pool[family]` 空闲链 LIFO 取；
    /// 空则新建（trace 日志 `Allocating a new command buffer`，0x142b15c50）；
    /// 取得后若启用了 debug_utils 则命名。越界族索引 panic（与原版一致）。
    pub fn create_command_buffer(
        &self,
        family: u32,
        name: &str,
    ) -> Result<super::command_buffer::VulkanCommandBuffer, vk::Result> {
        let command = match self.command_pool.take(family) {
            Some(command) => command,
            None => {
                log::trace!(target: "rhapsody::raw::command_buffer", "Allocating a new command buffer");
                super::command_buffer::VulkanCommandPool::new_for_family(self.inner.clone(), family)?
                    .allocate()?
            }
        };
        if let Some(utils) = self.debug_utils_option() {
            let _ = unsafe { utils.set_object_name(command.handle, name) };
        }
        Ok(command)
    }
    /// 回收路径把已完成的命令缓冲压回其族空闲链（`reclaim_command_buffer_resources` 尾段）。
    ///
    /// # Safety
    /// 覆盖该命令缓冲全部提交的时间线必须已完成。
    pub unsafe fn recycle_command_buffer(
        &self,
        command: super::command_buffer::VulkanCommandBuffer,
    ) -> Result<(), vk::Result> {
        unsafe { self.command_pool.recycle_completed(command) }
    }
    /// 空闲链数量（诊断/测试）。
    pub fn free_command_buffers(&self, family: u32) -> usize {
        self.command_pool.free_count(family)
    }
    pub fn allocate_command_buffer_seq(&mut self) -> u64 {
        self.seq.track()
    }
    pub fn command_buffer_seq(&self) -> &super::command_buffer::CommandBufferSeqTracker {
        &self.seq
    }
    /// `+0xa50` 销毁桶表（内部 RwLock，直接借用即可）。
    pub fn disposal(&self) -> &super::disposal::CommandBufferDisposalBuckets {
        &self.disposal
    }
    /// 原 `take_out_dropped_waste(&ManagedResources, &GraphicsDevice)` 的设备侧入口。
    pub fn take_out_dropped_waste(&self) -> usize {
        self.resources.take_out_dropped_waste_on(self)
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
        // 原版（0x140bf5208..0x140bf5823 + 0x142135f90）：13 个 `(name, VkSamplerCreateInfo)` 按
        // 固定顺序逐个 vkCreateSampler 后收进 HashMap<name, Sampler>；失败 `expect("create_sampler")`。
        // 这里保留创建顺序的 Vec，名字到下标由 BUILTIN_SAMPLER_SPECS 决定。
        let mut samplers = Vec::with_capacity(super::sampler::BUILTIN_SAMPLER_SPECS.len());
        for spec in super::sampler::BUILTIN_SAMPLER_SPECS.iter() {
            let info = spec.create_info();
            let sampler = super::sampler::Sampler::new(device.clone(), &info)
                .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?;
            samplers.push(Arc::new(sampler));
        }
        self.builtin_samplers = samplers;
        Ok(())
    }
    /// 按创建顺序下标返回内置采样器（0..13，顺序见 `BUILTIN_SAMPLER_SPECS`）；未创建时返回 None。
    pub fn get_built_in_sampler(&self, index: usize) -> Option<Arc<super::sampler::Sampler>> {
        self.builtin_samplers.get(index).cloned()
    }
    /// 按名字查找内置采样器；未知名字或尚未创建返回 None。
    pub fn try_get_built_in_sampler_by_name(
        &self,
        name: &str,
    ) -> Option<Arc<super::sampler::Sampler>> {
        let index = super::sampler::BUILTIN_SAMPLER_SPECS
            .iter()
            .position(|s| s.name == name)?;
        self.get_built_in_sampler(index)
    }
    /// 原 `get_built_in_sampler`（0x140bfc5b0）：缺失时 `panic!("Sampler not found: {name}")`。
    pub fn get_built_in_sampler_by_name(&self, name: &str) -> Arc<super::sampler::Sampler> {
        match self.try_get_built_in_sampler_by_name(name) {
            Some(sampler) => sampler,
            None => panic!("Sampler not found: {name}"),
        }
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
    /// 0x140bf0b90：以 `vkGetSemaphoreCounterValue` 的结果 `counter` 推进该队列的时间线环
    /// （`id` 为引擎内的队列标识；队首 `<= counter` 的值弹出，`counter` 存为队列 `+0x80`）。
    pub fn update_pending_submissions(
        &self,
        engine: &mut super::submit::SubmissionEngine,
        id: super::submit::QueueId,
        counter: u64,
    ) {
        engine.update_pending_submissions(id, counter);
    }
    /// 0x140bf0cc0：回收该队列已完成（`<= +0x80`）的命令缓冲句柄。
    pub fn reclaim_command_buffer_resources(
        &self,
        engine: &mut super::submit::SubmissionEngine,
        id: super::submit::QueueId,
    ) -> Vec<vk::CommandBuffer> {
        engine.reclaim_queue(id)
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

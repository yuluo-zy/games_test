// 候选实现迁移来源：reconstruction/render/work-packages/device-counters-r6/src/lib.rs.
// 这是行为重建，不是原作者源码。
use ash::vk;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

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
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceQueue {
    pub handle: vk::Queue,
    pub family: u32,
    pub index: u32,
}

impl VulkanDevice {
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
            }))
        }
    }

    pub fn wait_idle(&self) -> Result<(), vk::Result> {
        let _lock = self.queue_lock.lock().unwrap_or_else(|e| e.into_inner());
        unsafe { self.device.device_wait_idle() }
    }
}

impl Drop for VulkanDevice {
    fn drop(&mut self) {
        unsafe {
            // 所有拥有的 Vulkan 子对象都持有设备 Arc，因此会先于设备销毁。
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

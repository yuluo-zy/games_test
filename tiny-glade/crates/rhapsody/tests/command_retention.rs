//! Actual command metadata admission and per-cell CPU ownership. No synthetic
//! driver failures or fabricated device completion points are injected.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    Error, ErrorKind,
    device::{Device, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        descriptor_set::{
            LayoutBinding, LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSetLayout,
        },
        instance::{InstanceBuilder, ValidationMessenger},
        sampler::Sampler,
        semaphore::TimelineSemaphore,
    },
    resource::{GpuUse, SyncCell, UseVisitor},
    sync::QueueId,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
fn fixture() -> Option<(Arc<Device>, ValidationMessenger)> {
    gpu_test_support::initialize(
        "command retention admission",
        (|| -> rhapsody::Result<_> {
            let mut instance = InstanceBuilder::default();
            instance.debug_markers();
            instance
                .layers
                .push(c"VK_LAYER_KHRONOS_validation".to_owned());
            let device = Device::new(
                instance.build()?,
                None,
                &DeviceConfig {
                    validation: true,
                    ..DeviceConfig::headless()
                },
            )?;
            let messages = ValidationMessenger::new(device.instance())?;
            Ok((device, messages))
        })(),
    )
}
fn clean(device: &Device, messages: &ValidationMessenger) {
    device.try_collect_all_completed().unwrap();
    assert!(
        messages.collector.errors().is_empty(),
        "{:?}",
        messages.collector.errors()
    );
    assert!(
        messages.collector.warnings().is_empty(),
        "{:?}",
        messages.collector.warnings()
    );
}
struct Owner(Arc<AtomicUsize>);
impl Drop for Owner {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}
impl GpuUse for Owner {
    fn visit_uses(&self, _: &mut UseVisitor<'_>) -> rhapsody::Result<()> {
        Ok(())
    }
}
#[test]
fn owner_capacity_is_bounded_and_failed_recording_recovers_only_by_reset() {
    let Some((device, messages)) = fixture() else {
        return;
    };
    let pool = VulkanCommandPool::new_for_family_with_cpu_limit(
        device.clone(),
        device.queue_family_index(),
        2,
    )
    .unwrap();
    let before = device.allocator_stats().cpu_memory_usage().total();
    let dropped = Arc::new(AtomicUsize::new(0));
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    command
        .retain_object(Arc::new(Owner(dropped.clone())))
        .unwrap();
    command
        .retain_object(Arc::new(Owner(dropped.clone())))
        .unwrap();
    let queue = QueueId::new(0).unwrap();
    let submitted = device.timelines().submitted(queue).unwrap();
    assert!(matches!(
        command.retain_object(Arc::new(Owner(dropped.clone()))),
        Err(Error::Timeout)
    ));
    assert!(matches!(command.end(), Err(Error::Timeout)));
    assert!(matches!(command.end(), Err(Error::Timeout)));
    assert_eq!(device.timelines().submitted(queue).unwrap(), submitted);
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
    command.reset().unwrap();
    assert_eq!(dropped.load(Ordering::Relaxed), 3);
    command.begin().unwrap();
    command
        .retain_object(Arc::new(Owner(dropped.clone())))
        .unwrap();
    command.end().unwrap();
    let pending = command.submit().unwrap();
    pending.wait(10_000_000_000).unwrap();
    assert_eq!(pending.point.value, submitted + 1);
    drop(pending);
    assert_eq!(dropped.load(Ordering::Relaxed), 4);
    pool.trim_cpu_scratch();
    assert_eq!(device.allocator_stats().cpu_memory_usage().total(), before);
    clean(&device, &messages);
}
#[test]
fn owner_storage_oom_keeps_its_category_and_publishes_no_queue_point() {
    let Some((device, messages)) = fixture() else {
        return;
    };
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    let before = device.allocator_stats().cpu_memory_usage().total();
    let submitted = device
        .timelines()
        .submitted(QueueId::new(0).unwrap())
        .unwrap();
    device
        .allocator_stats()
        .set_total_budget(device.allocator_stats().budget_usage().total() + 1)
        .unwrap();
    let dropped = Arc::new(AtomicUsize::new(0));
    let result = command.retain_object(Arc::new(Owner(dropped.clone())));
    device.allocator_stats().set_total_budget(u64::MAX).unwrap();
    assert!(matches!(result, Err(Error::OutOfHostMemory)));
    assert!(matches!(command.end(), Err(Error::OutOfHostMemory)));
    assert_eq!(device.allocator_stats().cpu_memory_usage().total(), before);
    assert_eq!(
        device
            .timelines()
            .submitted(QueueId::new(0).unwrap())
            .unwrap(),
        submitted
    );
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
    command.reset().unwrap();
    command.begin().unwrap();
    command.end().unwrap();
    let pending = command.submit().unwrap();
    pending.wait(10_000_000_000).unwrap();
    drop(pending);
    clean(&device, &messages);
}
struct Reject(Arc<Sampler>);
struct Pair([Arc<Sampler>; 2]);
impl GpuUse for Pair {
    fn visit_uses(&self, visitor: &mut UseVisitor<'_>) -> rhapsody::Result<()> {
        for sampler in &self.0 {
            sampler.visit_uses(visitor)?;
        }
        Ok(())
    }
}
#[test]
fn nested_resource_cells_have_their_own_capacity_gate() {
    let Some((device, messages)) = fixture() else {
        return;
    };
    let pool = VulkanCommandPool::new_for_family_with_cpu_limit(
        device.clone(),
        device.queue_family_index(),
        2,
    )
    .unwrap();
    let pair = Arc::new(Pair([
        Arc::new(Sampler::new(device.clone(), &vk::SamplerCreateInfo::default()).unwrap()),
        Arc::new(Sampler::new(device.clone(), &vk::SamplerCreateInfo::default()).unwrap()),
    ]));
    let before = device.allocator_stats().cpu_memory_usage().total();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    let submitted = device
        .timelines()
        .submitted(QueueId::new(0).unwrap())
        .unwrap();
    // One owner fits, but its two distinct cells plus the pool exceed the limit.
    assert!(matches!(
        command.retain_object(pair.clone()),
        Err(Error::Timeout)
    ));
    assert!(matches!(command.end(), Err(Error::Timeout)));
    assert_eq!(
        device
            .timelines()
            .submitted(QueueId::new(0).unwrap())
            .unwrap(),
        submitted
    );
    pair.visit_uses(&mut |cell| {
        assert!(cell.snapshot().is_empty());
        Ok(())
    })
    .unwrap();
    drop(command);
    assert_eq!(device.allocator_stats().cpu_memory_usage().total(), before);
    clean(&device, &messages);
}
impl GpuUse for Reject {
    fn visit_uses(&self, visitor: &mut UseVisitor<'_>) -> rhapsody::Result<()> {
        self.0.visit_uses(visitor)?;
        Err(Error::unsupported("intentional CPU visitor rejection"))
    }
}
#[test]
fn nested_visitor_rejection_is_sticky_and_cell_credit_follows_its_last_owner() {
    let Some((device, messages)) = fixture() else {
        return;
    };
    let sampler =
        Arc::new(Sampler::new(device.clone(), &vk::SamplerCreateInfo::default()).unwrap());
    let mut cell: Option<Arc<SyncCell>> = None;
    sampler
        .visit_uses(&mut |value| {
            cell = Some(value.clone());
            Ok(())
        })
        .unwrap();
    let cell = cell.unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    let submitted = device
        .timelines()
        .submitted(QueueId::new(0).unwrap())
        .unwrap();
    assert_eq!(
        command
            .retain_object(Arc::new(Reject(sampler.clone())))
            .unwrap_err()
            .kind(),
        ErrorKind::Unsupported
    );
    assert_eq!(command.end().unwrap_err().kind(), ErrorKind::Unsupported);
    assert_eq!(
        device
            .timelines()
            .submitted(QueueId::new(0).unwrap())
            .unwrap(),
        submitted
    );
    assert!(cell.snapshot().is_empty());
    drop(command);
    drop(sampler);
    clean(&device, &messages);
    let before = device.allocator_stats().cpu_memory_usage().total();
    let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
        .extend(std::alloc::Layout::new::<SyncCell>())
        .unwrap();
    let bytes = layout.pad_to_align().size() as u64;
    assert!(before >= bytes);
    drop(cell);
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total() + bytes,
        before
    );
    clean(&device, &messages);
}

#[test]
fn ignored_invalid_native_range_cannot_end_or_publish_a_submission() {
    let Some((device, messages)) = fixture() else {
        return;
    };
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(32, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap(),
    );
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    let queue = QueueId::new(0).unwrap();
    let submitted = device.timelines().submitted(queue).unwrap();
    // SAFETY: no native memory operation is recorded. The deliberately invalid
    // retention request must poison recording even if its error is ignored.
    let _ignored = unsafe { command.retain_opaque_buffer_range(buffer, 31, 2) };
    assert_eq!(command.end().unwrap_err().kind(), ErrorKind::InvalidUsage);
    assert_eq!(device.timelines().submitted(queue).unwrap(), submitted);
    command.reset().unwrap();
    command.begin().unwrap();
    command.end().unwrap();
    let pending = command.submit().unwrap();
    pending.wait(10_000_000_000).unwrap();
    assert_eq!(pending.point.value, submitted + 1);
    drop(pending);
    clean(&device, &messages);
}

#[test]
fn nine_external_waits_and_signals_keep_last_owners_and_charge_actual_spill() {
    let Some((device, messages)) = fixture() else {
        return;
    };
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let semaphores: Vec<_> = (0..9)
        .map(|_| TimelineSemaphore::new(device.clone(), 0).unwrap())
        .collect();
    let weak: Vec<_> = semaphores.iter().map(Arc::downgrade).collect();
    let waits: Vec<_> = semaphores
        .iter()
        .map(|semaphore| (semaphore.clone(), 0, vk::PipelineStageFlags::ALL_COMMANDS))
        .collect();
    let signals: Vec<_> = semaphores
        .iter()
        .map(|semaphore| (semaphore.clone(), 1))
        .collect();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    command.end().unwrap();
    let before = device.allocator_stats().cpu_memory_usage().total();
    let pending = command
        .submit_to(
            rhapsody::raw::device::DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &waits,
            &signals,
        )
        .unwrap();
    pool.trim_cpu_scratch();
    // Nested Vulkan arrays are synchronous scratch. Only the eighteen actual
    // owner Arcs spill into the pending receipt after the queue call returns.
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total(),
        before + (18 * std::mem::size_of::<Arc<TimelineSemaphore>>()) as u64
    );
    drop(waits);
    drop(signals);
    drop(semaphores);
    assert!(weak.iter().all(|semaphore| semaphore.upgrade().is_some()));
    pending.wait(10_000_000_000).unwrap();
    for semaphore in &weak {
        assert_eq!(semaphore.upgrade().unwrap().completed().unwrap(), 1);
    }
    drop(pending);
    assert!(weak.iter().all(|semaphore| semaphore.upgrade().is_none()));
    clean(&device, &messages);
}

#[test]
fn external_owner_and_nested_array_oom_each_leave_real_queue_and_semaphores_unchanged() {
    let Some((device, messages)) = fixture() else {
        return;
    };
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let semaphores: Vec<_> = (0..9)
        .map(|_| TimelineSemaphore::new(device.clone(), 0).unwrap())
        .collect();
    let signals: Vec<_> = semaphores
        .iter()
        .map(|semaphore| (semaphore.clone(), 1))
        .collect();
    let queue = QueueId::new(0).unwrap();
    let submitted = device.timelines().submitted(queue).unwrap();
    let owners_bytes = (9 * std::mem::size_of::<Arc<TimelineSemaphore>>()) as u64;
    // First deny the owner spill; then admit it exactly and deny the separate
    // nested Vulkan arrays. Both are pre-submit admission failures.
    for spare in [1, owners_bytes] {
        let mut command = pool.allocate().unwrap();
        command.begin().unwrap();
        command.end().unwrap();
        let before = device.allocator_stats().budget_usage().total();
        device
            .allocator_stats()
            .set_total_budget(before + spare)
            .unwrap();
        let result = command.submit_to(
            rhapsody::raw::device::DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[],
            &signals,
        );
        device.allocator_stats().set_total_budget(u64::MAX).unwrap();
        assert!(matches!(result, Err(Error::OutOfHostMemory)));
        assert_eq!(device.timelines().submitted(queue).unwrap(), submitted);
        for semaphore in &semaphores {
            assert_eq!(semaphore.completed().unwrap(), 0);
            semaphore
                .visit_uses(&mut |cell| {
                    assert!(cell.snapshot().is_empty());
                    Ok(())
                })
                .unwrap();
        }
    }
    clean(&device, &messages);
}

#[test]
fn descriptor_owner_row_oom_precedes_native_update_and_preserves_retry() {
    use rhapsody::raw::binding::DescriptorSetBinding;
    let Some((device, messages)) = fixture() else {
        return;
    };
    let plan = LayoutPlan {
        flags: 0,
        binding_flags: vec![0],
        bindings: vec![LayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER.as_raw(),
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE.as_raw(),
            immutable_sampler: None,
        }],
    };
    let layout = VulkanDescriptorSetLayout::new(device.clone(), &plan).unwrap();
    let pool = VulkanDescriptorPool::new(device.clone(), &plan, 1).unwrap();
    let mut set = pool.allocate(layout).unwrap();
    let owner = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(32, vk::BufferUsageFlags::STORAGE_BUFFER),
        )
        .unwrap(),
    );
    let weak = Arc::downgrade(&owner);
    let binding = DescriptorSetBinding::Buffer {
        ty: vk::DescriptorType::STORAGE_BUFFER,
        info: vk::DescriptorBufferInfo::default()
            .buffer(owner.handle)
            .range(32),
        owner,
    };
    let before = device.allocator_stats().cpu_memory_usage().total();
    let updates = device.descriptor_update_count();
    device
        .allocator_stats()
        .set_total_budget(device.allocator_stats().budget_usage().total() + 1)
        .unwrap();
    // SAFETY: the set is unsubmitted and this owned buffer matches its layout.
    let result = unsafe { set.update(0, 0, &binding) };
    device.allocator_stats().set_total_budget(u64::MAX).unwrap();
    assert!(matches!(result, Err(Error::OutOfHostMemory)));
    assert_eq!(device.descriptor_update_count(), updates);
    assert_eq!(device.allocator_stats().cpu_memory_usage().total(), before);
    // SAFETY: the same idle set is now allowed to admit its actual owner row.
    unsafe { set.update(0, 0, &binding) }.unwrap();
    assert_eq!(device.descriptor_update_count(), updates + 1);
    drop(binding);
    assert!(weak.upgrade().is_some());
    drop(set);
    assert!(weak.upgrade().is_none());
    clean(&device, &messages);
}

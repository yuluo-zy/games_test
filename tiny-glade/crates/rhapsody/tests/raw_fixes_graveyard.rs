//! 真实 GPU 回归：提交失败不占时间线值，未完成工作不能提前回收。
mod gpu_test_support;
use ash::vk;
use rhapsody::device::graveyard::Doomed;
use rhapsody::device::selection::create_headless_device;
use rhapsody::sync::SyncSet;

#[test]
fn failed_submit_does_not_advance_completion_past_in_flight_work() {
    let Ok(device) = create_headless_device() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    let q = device.queue_id(device.primary_queue()).unwrap();
    let mut ty = vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::TIMELINE);
    let gate = unsafe {
        device
            .raw_device()
            .create_semaphore(&vk::SemaphoreCreateInfo::default().push_next(&mut ty), None)
    }
    .unwrap();
    let (point, ()) = device
        .timelines()
        .submit(q, |signal| {
            let waits = [gate];
            let stages = [vk::PipelineStageFlags::ALL_COMMANDS];
            let signals = [signal.semaphore];
            let values = [signal.value];
            let mut timeline = vk::TimelineSemaphoreSubmitInfo::default()
                .wait_semaphore_values(&[1])
                .signal_semaphore_values(&values);
            let info = vk::SubmitInfo::default()
                .wait_semaphores(&waits)
                .wait_dst_stage_mask(&stages)
                .signal_semaphores(&signals)
                .push_next(&mut timeline);
            unsafe {
                device
                    .raw_device()
                    .queue_submit(device.primary_queue(), &[info], vk::Fence::null())
            }
            .map_err(Into::into)
        })
        .unwrap();
    let failed = device.timelines().submit(q, |_| {
        Err::<(), _>(vk::Result::ERROR_OUT_OF_HOST_MEMORY.into())
    });
    assert_eq!(failed.unwrap_err().kind(), rhapsody::ErrorKind::OutOfMemory);
    let before = device.timelines().submitted(q).unwrap();
    let sampler = unsafe {
        device
            .raw_device()
            .create_sampler(&vk::SamplerCreateInfo::default(), None)
    }
    .unwrap();
    device.retire(point.into(), Doomed::Sampler(sampler));
    let collected = device.collect_garbage();
    let explicitly_collected = device.try_collect_all_completed().unwrap();
    let pending = device.retirement_stats().pending;
    // 先开闸再断言，测试失败也不会使设备 Drop 等待被本线程阻塞的 GPU。
    unsafe {
        device
            .raw_device()
            .signal_semaphore(&vk::SemaphoreSignalInfo::default().semaphore(gate).value(1))
    }
    .unwrap();
    device
        .timelines()
        .wait(&SyncSet::from_point(point), 10_000_000_000)
        .unwrap();
    device.collect_garbage();
    unsafe { device.raw_device().destroy_semaphore(gate, None) };
    assert_eq!(before, point.value);
    assert_eq!(collected, 0);
    assert_eq!(
        explicitly_collected, 0,
        "explicit draining still respects GPU dependencies"
    );
    assert_eq!(pending, 1);
    assert_eq!(device.retirement_stats().pending, 0);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

#[test]
fn unused_objects_are_destroyed_without_idle() {
    let Ok(device) = create_headless_device() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    for _ in 0..128 {
        let sampler = unsafe {
            device
                .raw_device()
                .create_sampler(&vk::SamplerCreateInfo::default(), None)
        }
        .unwrap();
        device.retire(SyncSet::EMPTY, Doomed::Sampler(sampler));
    }
    assert_eq!(device.retirement_stats().pending, 0);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

#[test]
fn dropping_a_submission_defers_used_resources_without_blocking_or_retiring_unused_ones() {
    use rhapsody::command::command_buffer::VulkanCommandPool;
    use rhapsody::device::DeviceQueue;
    use rhapsody::resource::buffer::{Buffer, BufferDesc};
    use rhapsody::resource::sampler::Sampler;
    use rhapsody::device::semaphore::TimelineSemaphore;
    use std::{
        sync::{Arc, mpsc},
        time::Duration,
    };
    let Ok(device) = create_headless_device() else {
        eprintln!("SKIP: pending resource retirement requires Vulkan");
        return;
    };
    let gate = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let weak_buffer = Arc::downgrade(&buffer);
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    command.retain_opaque(buffer.clone()).unwrap();
    unsafe {
        device
            .raw_device()
            .cmd_fill_buffer(command.raw_handle(), buffer.raw_handle(), 0, 16, 0);
    }
    command.end().unwrap();
    let queue = DeviceQueue {
        handle: device.primary_queue(),
        family: device.queue_family_index(),
        index: 0,
    };
    let pending = command
        .submit_to(
            queue,
            &[(gate.clone(), 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    drop(buffer);
    drop(pool);
    let (tx, rx) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        drop(pending);
        let _ = tx.send(());
    });
    let did_not_block = rx.recv_timeout(Duration::from_secs(1)).is_ok();
    let deferred = device.retirement_stats().pending;
    let released_cpu_owner = weak_buffer.upgrade().is_none();
    let unused = Sampler::new(device.clone(), &vk::SamplerCreateInfo::default()).unwrap();
    drop(unused);
    let after_unused = device.retirement_stats().pending;
    // Always release the gate before assertions, including on a blocking-drop regression.
    unsafe {
        device.raw_device().signal_semaphore(
            &vk::SemaphoreSignalInfo::default()
                .semaphore(gate.raw_handle())
                .value(1),
        )
    }
    .unwrap();
    worker.join().unwrap();
    device.wait(&point.into(), 10_000_000_000).unwrap();
    device.try_collect_garbage().unwrap();
    assert!(
        did_not_block,
        "dropping the receipt must not wait for the GPU"
    );
    assert!(released_cpu_owner);
    assert!(
        deferred >= 2,
        "command allocation and buffer must remain retired while the gate is closed"
    );
    assert_eq!(
        after_unused, deferred,
        "an unused sampler has no dependency on this submission"
    );
    assert_eq!(device.retirement_stats().pending, 0);
}

#[test]
fn completed_submitted_drops_obey_ticks_and_explicit_drain_ignores_the_quota() {
    use rhapsody::device::{
        Device, DeviceConfig, QueueClass, ReclamationBudget, ReclamationConfig,
    };
    let budget = ReclamationBudget {
        max_scans: 8,
        max_destroys: 3,
    };
    let config = DeviceConfig {
        validation: true,
        reclamation: ReclamationConfig {
            normal: budget,
            pressure: budget,
        },
        ..DeviceConfig::headless()
    };
    let Some(device) =
        gpu_test_support::initialize("bounded native reclamation", Device::new_headless(&config))
    else {
        return;
    };
    assert_eq!(device.reclamation_config(), config.reclamation);
    let queue = device.queue(QueueClass::Graphics).unwrap();
    let point = unsafe { device.submit(queue, &[]) }.unwrap();
    device.wait(&point.into(), 10_000_000_000).unwrap();
    for _ in 0..161 {
        let sampler = unsafe {
            device
                .raw_device()
                .create_sampler(&vk::SamplerCreateInfo::default(), None)
        }
        .unwrap();
        device.retire(point.into(), Doomed::Sampler(sampler));
    }
    assert_eq!(
        device.retirement_stats().pending,
        161,
        "already-completed submitted drops must not bypass maintenance"
    );
    assert_eq!(device.try_collect_garbage().unwrap(), 3);
    assert_eq!(device.retirement_stats().pending, 158);
    assert_eq!(device.try_collect_garbage().unwrap(), 3);
    assert_eq!(device.retirement_stats().pending, 155);
    assert_eq!(device.try_collect_all_completed().unwrap(), 155);
    assert_eq!(device.retirement_stats().pending, 0);
    assert_eq!(device.try_collect_all_completed().unwrap(), 0);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

//! 真实 GPU 回归：提交失败不占时间线值，未完成工作不能提前回收。
use ash::vk;
use rhapsody::raw::{graveyard::Doomed, init::create_headless_device};
use rhapsody::sync::SyncSet;

#[test]
fn failed_submit_does_not_advance_completion_past_in_flight_work() {
    let Ok(device) = create_headless_device() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    let q = device.queue_id(device.queue).unwrap();
    let mut ty = vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::TIMELINE);
    let gate = unsafe {
        device
            .device
            .create_semaphore(&vk::SemaphoreCreateInfo::default().push_next(&mut ty), None)
    }
    .unwrap();
    let (point, ()) = device
        .timelines
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
                    .device
                    .queue_submit(device.queue, &[info], vk::Fence::null())
            }
            .map_err(Into::into)
        })
        .unwrap();
    let failed = device.timelines.submit(q, |_| {
        Err::<(), _>(vk::Result::ERROR_OUT_OF_HOST_MEMORY.into())
    });
    assert_eq!(failed.unwrap_err().kind(), rhapsody::ErrorKind::OutOfMemory);
    let before = device.timelines.submitted(q).unwrap();
    let sampler = unsafe {
        device
            .device
            .create_sampler(&vk::SamplerCreateInfo::default(), None)
    }
    .unwrap();
    device.bury(Doomed::Sampler(sampler));
    let collected = device.collect_garbage();
    let pending = device.garbage_stats().pending;
    // 先开闸再断言，测试失败也不会使设备 Drop 等待被本线程阻塞的 GPU。
    unsafe {
        device
            .device
            .signal_semaphore(&vk::SemaphoreSignalInfo::default().semaphore(gate).value(1))
    }
    .unwrap();
    device
        .timelines
        .wait(&SyncSet::from_point(point), 10_000_000_000)
        .unwrap();
    device.collect_garbage();
    unsafe { device.device.destroy_semaphore(gate, None) };
    assert_eq!(before, point.value);
    assert_eq!(collected, 0);
    assert_eq!(pending, 1);
    assert_eq!(device.garbage_stats().pending, 0);
    assert_eq!(device.garbage_stats().idle_waits, 0);
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
                .device
                .create_sampler(&vk::SamplerCreateInfo::default(), None)
        }
        .unwrap();
        device.bury(Doomed::Sampler(sampler));
    }
    assert_eq!(device.garbage_stats().pending, 0);
    assert_eq!(device.garbage_stats().idle_waits, 0);
}

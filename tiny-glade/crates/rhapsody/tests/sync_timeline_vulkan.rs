//! 真实设备上验证 Timelines：提交值、失败不占用、等待与查询。
mod gpu_test_support;
use ash::vk;
use rhapsody::error::ErrorKind;
use rhapsody::sync::{QueueId, SyncSet, timeline::Timelines};

fn gpu() -> Option<std::sync::Arc<rhapsody::raw::device::Device>> {
    gpu_test_support::initialize(
        "timeline production headless device",
        rhapsody::raw::init::create_headless_device(),
    )
}

#[test]
fn timelines_on_a_real_queue() {
    let Some(gpu) = gpu() else {
        eprintln!("跳过：无 Vulkan 设备");
        return;
    };
    let mut kind = vk::SemaphoreTypeCreateInfo::default()
        .semaphore_type(vk::SemaphoreType::TIMELINE)
        .initial_value(0);
    let semaphore = unsafe {
        gpu.device.create_semaphore(
            &vk::SemaphoreCreateInfo::default().push_next(&mut kind),
            None,
        )
    }
    .unwrap();
    let timelines = Timelines::new(gpu.device.clone(), &[semaphore]).unwrap();
    let q0 = QueueId::new(0).unwrap();

    let submit = |fail: bool| {
        timelines.submit(q0, |signal| {
            if fail {
                return Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY.into());
            }
            let values = [signal.value];
            let semaphores = [signal.semaphore];
            let mut timeline_info =
                vk::TimelineSemaphoreSubmitInfo::default().signal_semaphore_values(&values);
            let info = [vk::SubmitInfo::default()
                .signal_semaphores(&semaphores)
                .push_next(&mut timeline_info)];
            unsafe { gpu.device.queue_submit(gpu.queue, &info, vk::Fence::null()) }?;
            Ok(())
        })
    };

    let mut points = SyncSet::EMPTY;
    for _ in 0..50 {
        let (point, ()) = submit(false).unwrap();
        points.insert(point);
    }
    assert_eq!(submit(true).unwrap_err().kind(), ErrorKind::OutOfMemory);
    let (point, ()) = submit(false).unwrap();
    assert_eq!(point.value, 51, "失败的提交没有占用值");
    points.insert(point);

    timelines.wait(&points, 5_000_000_000).unwrap();
    assert!(timelines.is_reached(&points).unwrap());
    assert_eq!(timelines.poll().unwrap().get(q0), 51);
    // 设备上的真实计数器与提交值一致（没有因失败而跳值）。
    assert_eq!(
        unsafe { gpu.device.get_semaphore_counter_value(semaphore) }.unwrap(),
        51
    );

    let ahead = SyncSet::from_point(rhapsody::sync::SyncPoint {
        queue: q0,
        value: 52,
    });
    assert_eq!(
        timelines.wait(&ahead, u64::MAX).unwrap_err().kind(),
        ErrorKind::InvalidUsage
    );

    drop(timelines);
    gpu.wait_idle().unwrap();
    unsafe {
        gpu.device.destroy_semaphore(semaphore, None);
    }
}

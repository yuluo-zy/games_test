//! 延迟销毁场修复回归：提交失败不得把退役时间线推过在途工作；非主队列提交后入场仍能自动回收。
use ash::vk;
use rhapsody::raw::{
    graveyard::{AUTO_COLLECT_THRESHOLD, Doomed},
    init::create_headless_device,
};
use std::time::Duration;

fn timeline(device: &ash::Device) -> vk::Semaphore {
    let mut kind = vk::SemaphoreTypeCreateInfo::default()
        .semaphore_type(vk::SemaphoreType::TIMELINE)
        .initial_value(0);
    let info = vk::SemaphoreCreateInfo::default().push_next(&mut kind);
    unsafe { device.create_semaphore(&info, None) }.expect("timeline")
}

/// epoch 1 的提交被闸门信号量挡在 GPU 上；epoch 2 提交失败时不能抢先把完成线推到 2。
#[test]
fn failed_submit_does_not_advance_completion_past_in_flight_work() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    let gate = timeline(&device.device);
    {
        let _guard = device.queue_lock.lock();
        let retire = device
            .note_submit_on(device.queue)
            .expect("主队列在时间线上");
        let waits = [gate];
        let wait_values = [1u64];
        let stages = [vk::PipelineStageFlags::ALL_COMMANDS];
        let signals = [retire.0];
        let signal_values = [retire.1];
        let mut values = vk::TimelineSemaphoreSubmitInfo::default()
            .wait_semaphore_values(&wait_values)
            .signal_semaphore_values(&signal_values);
        let submit = [vk::SubmitInfo::default()
            .wait_semaphores(&waits)
            .wait_dst_stage_mask(&stages)
            .signal_semaphores(&signals)
            .push_next(&mut values)];
        unsafe {
            device
                .device
                .queue_submit(device.queue, &submit, vk::Fence::null())
        }
        .expect("submit");
        let failed = device.note_submit_on(device.queue);
        let host = device.device.clone();
        let opener = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(300));
            let info = vk::SemaphoreSignalInfo::default().semaphore(gate).value(1);
            unsafe { host.signal_semaphore(&info) }.expect("host signal");
        });
        device.note_submit_failed(failed);
        // 返回时 epoch 1 的工作必须已经完成（闸门已开），否则完成线超前于在途提交。
        let opened = unsafe { device.device.get_semaphore_counter_value(gate) }.unwrap();
        opener.join().unwrap();
        assert_eq!(
            opened, 1,
            "note_submit_failed 在在途提交完成前推进了退役时间线"
        );
        // 之后的等待不能卡在失败的 epoch 上。
        device.idle_if_in_flight_locked().expect("idle");
    }
    assert!(device.garbage_stats().completed_epoch >= 2);
    device.wait_idle().unwrap();
    unsafe { device.device.destroy_semaphore(gate, None) };
}

/// 有非主队列提交（未被 idle 确认）后，入场达到阈值仍需回收，不能无界增长。
#[test]
fn bury_collects_after_foreign_submit() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    {
        let _guard = device.queue_lock.lock();
        // 非主队列句柄：只登记 epoch，不真正提交（不在退役时间线上）。
        assert!(device.note_submit_on(vk::Queue::null()).is_none());
    }
    let info = vk::SamplerCreateInfo::default();
    for _ in 0..AUTO_COLLECT_THRESHOLD * 2 {
        let sampler = unsafe { device.device.create_sampler(&info, None) }.unwrap();
        device.bury(Doomed::Sampler(sampler));
    }
    let stats = device.garbage_stats();
    assert!(
        stats.pending < AUTO_COLLECT_THRESHOLD,
        "非主队列提交后入场从不回收：pending = {}",
        stats.pending
    );
}

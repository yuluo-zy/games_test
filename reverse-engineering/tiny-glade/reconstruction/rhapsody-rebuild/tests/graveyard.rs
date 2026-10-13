//! 延迟销毁场：用真实设备验证“无在飞工作就地释放 / 有在飞工作延后 / 摊销回收 / 设备销毁排空”。
//! 无可用 Vulkan 设备（如 CI）时跳过；本机可用 `VK_ICD_FILENAMES=.../lvp_icd.json` 走 lavapipe。
use ash::vk;
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    device::VulkanDevice,
    graveyard::AUTO_COLLECT_THRESHOLD,
    init::create_headless_device,
};
use std::sync::Arc;

fn device() -> Option<Arc<VulkanDevice>> {
    match create_headless_device() {
        Ok(device) => Some(device),
        Err(error) => {
            eprintln!("跳过：无可用 Vulkan 设备（{error:?}）");
            None
        }
    }
}
fn buffer(device: &Arc<VulkanDevice>) -> Buffer {
    Buffer::new(
        Arc::clone(device),
        &BufferDesc::new_gpu_only(4096, vk::BufferUsageFlags::STORAGE_BUFFER),
    )
    .unwrap()
}
/// 模拟“有一次尚未确认完成的提交”：真实提交路径在 queue_lock 内递增 epoch。
fn pretend_submit(device: &VulkanDevice) {
    let _guard = device.queue_lock.lock();
    device.note_submit();
}
fn live_allocations(device: &VulkanDevice) -> usize {
    device.allocator_stats.report().allocations.len()
}

#[test]
fn quiescent_drop_releases_immediately_without_waiting() {
    let Some(device) = device() else { return };
    let b = buffer(&device);
    assert_eq!(live_allocations(&device), 1);
    drop(b);
    assert_eq!(live_allocations(&device), 0);
    assert_eq!(device.garbage_stats().pending, 0);
}

#[test]
fn drop_after_unconfirmed_submit_is_deferred_until_collect() {
    let Some(device) = device() else { return };
    let b = buffer(&device);
    pretend_submit(&device);
    drop(b);
    // 显存与句柄都还在：GPU 可能仍在使用。
    assert_eq!(device.garbage_stats().pending, 1);
    assert_eq!(live_allocations(&device), 1);
    assert_eq!(device.collect_garbage(), 1);
    assert_eq!(device.garbage_stats().pending, 0);
    assert_eq!(live_allocations(&device), 0);
    let stats = device.garbage_stats();
    assert_eq!(stats.completed_epoch, stats.submit_epoch);
}

#[test]
fn wait_idle_confirms_completion_so_later_drops_are_immediate() {
    let Some(device) = device() else { return };
    let early = buffer(&device);
    pretend_submit(&device);
    drop(early);
    assert_eq!(device.garbage_stats().pending, 1);
    device.wait_idle().unwrap();
    assert_eq!(device.garbage_stats().pending, 0);
    drop(buffer(&device));
    assert_eq!(device.garbage_stats().pending, 0);
    assert_eq!(live_allocations(&device), 0);
}

#[test]
fn many_deferred_drops_share_one_idle_wait() {
    let Some(device) = device() else { return };
    let buffers: Vec<_> = (0..AUTO_COLLECT_THRESHOLD).map(|_| buffer(&device)).collect();
    pretend_submit(&device);
    let before = device.garbage_stats().completed_epoch;
    drop(buffers);
    // 达到阈值时自动回收一次：只推进一次完成线，而不是每个资源一次 idle。
    let after = device.garbage_stats();
    assert_eq!(after.pending, 0);
    assert_eq!(after.completed_epoch, before + 1);
    assert_eq!(live_allocations(&device), 0);
}

#[test]
fn device_drop_drains_pending_entries() {
    let Some(device) = device() else { return };
    let b = buffer(&device);
    pretend_submit(&device);
    drop(b);
    assert_eq!(device.garbage_stats().pending, 1);
    // 设备 Drop 排空延迟场；释放登记不一致会在此 panic。
    drop(device);
}

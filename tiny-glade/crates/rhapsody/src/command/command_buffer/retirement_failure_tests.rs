use super::*;
use crate::device::Device;
fn device() -> Option<Arc<Device>> {
    match Device::new_headless(&crate::device::DeviceConfig::headless()) {
        Ok(device) => Some(device),
        Err(error) => {
            assert!(
                !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                "{error}"
            );
            None
        }
    }
}
#[test]
fn failed_adopt_consumes_detached_ownership_and_releases_its_credit() {
    let Some(device) = device() else { return };
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let baseline = device.allocator_stats().cpu_memory_usage().total();
    let command = pool.allocate().unwrap();
    let (handle, _, retirement) = command.detach();
    device
        .allocator_stats()
        .set_host_budget(device.allocator_stats().cpu_memory_usage().total())
        .unwrap();
    assert!(matches!(
        pool.adopt(handle, retirement),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(pool.live_command_count(), 0);
    pool.reset_completed(&mut []).unwrap();
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total(),
        baseline
    );
    device.allocator_stats().set_host_budget(0).unwrap();
}
#[test]
fn failed_native_wrap_uses_its_ticket_even_while_another_command_records() {
    for busy in [false, true] {
        let Some(device) = device() else { return };
        let pool = VulkanCommandPool::new(device.clone()).unwrap();
        let baseline = device.allocator_stats().cpu_memory_usage().total();
        let mut recording = busy.then(|| pool.allocate().unwrap());
        if let Some(command) = &mut recording {
            command.begin().unwrap();
        }
        let used = device.allocator_stats().cpu_memory_usage().total();
        let node_bytes = std::mem::size_of::<
            crate::retire::Node<crate::device::graveyard::Doomed>,
        >() as u64;
        // Admit exactly the two retirement nodes, then fail the wrapper's
        // first use-cell vector AFTER the actual native allocation.
        device
            .allocator_stats()
            .set_host_budget(used + 2 * node_bytes)
            .unwrap();
        assert!(matches!(
            pool.allocate(),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(device.retirement_stats().pending, usize::from(busy));
        device.allocator_stats().set_host_budget(0).unwrap();
        if let Some(command) = &mut recording {
            command.end().unwrap();
        }
        drop(recording);
        device.try_collect_all_completed().unwrap();
        pool.reset_completed(&mut []).unwrap();
        assert_eq!(
            device.allocator_stats().cpu_memory_usage().total(),
            baseline
        );
    }
}

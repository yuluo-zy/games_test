//! 显存预算快照：原作 `refresh_memory_budget` 经 arc-swap 发布快照，`*_memory_budget_bytes` 只读快照。
//! 无可用 Vulkan 实例时跳过；本机可用 `VK_ICD_FILENAMES=.../lvp_icd.json` 走 lavapipe。
use rhapsody::device::selection::GraphicsInstance;
use std::sync::Arc;

#[test]
fn refresh_publishes_a_snapshot_that_getters_read_without_requerying() {
    let Ok(graphics) = GraphicsInstance::new() else {
        eprintln!("跳过：无可用 Vulkan 实例");
        return;
    };
    let Some(physical) = graphics.devices.first() else {
        eprintln!("跳过：无物理设备");
        return;
    };
    assert!(
        physical.cached_performance_info().is_none(),
        "刷新前没有快照"
    );
    let fresh = physical.performance_info().unwrap();
    let cached = physical
        .cached_performance_info()
        .expect("刷新必须发布快照");
    assert_eq!(*cached, fresh);
    // getter 读快照：同一份 Arc 被反复读取，不会换成新分配的快照。
    let again = physical.cached_performance_info().unwrap();
    assert!(Arc::ptr_eq(&cached, &again));
    assert_eq!(
        physical.device_local_memory_budget_bytes().unwrap(),
        cached.device_local_budget
    );
    assert_eq!(
        physical.host_memory_budget_bytes().unwrap(),
        cached.host_budget
    );
    assert!(Arc::ptr_eq(
        &cached,
        &physical.cached_performance_info().unwrap()
    ));
    // 再次刷新才会换新快照。
    physical.performance_info().unwrap();
    assert!(!Arc::ptr_eq(
        &cached,
        &physical.cached_performance_info().unwrap()
    ));
}

#[test]
fn getter_on_a_fresh_device_refreshes_once_then_reads_the_snapshot() {
    let Ok(graphics) = GraphicsInstance::new() else {
        return;
    };
    let Some(physical) = graphics.devices.first() else {
        return;
    };
    let first = physical.device_local_memory_budget_bytes().unwrap();
    let published = physical
        .cached_performance_info()
        .expect("首次读取应发布快照");
    assert_eq!(first, published.device_local_budget);
    physical.host_memory_budget_bytes().unwrap();
    assert!(Arc::ptr_eq(
        &published,
        &physical.cached_performance_info().unwrap()
    ));
}

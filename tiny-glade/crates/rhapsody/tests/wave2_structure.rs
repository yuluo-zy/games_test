//! 第二批结构对齐：销毁桶归设备、`track` 非法状态 panic、物理设备包装挂接。
//! 需要 Vulkan 的用例在无实例时跳过（本机用 lavapipe）。
use rhapsody::raw::device::GraphicsDevice;
use rhapsody::raw::init::{DeviceSelectionPolicy, GraphicsInstance};
use rhapsody::raw::managed::resources::ResourceManager;
use rhapsody::raw::managed::tracking::CommandState;
use rhapsody::raw::physical_device::PhysicalDevice;
use std::sync::Arc;

fn headless() -> Option<(GraphicsInstance, Arc<GraphicsDevice>)> {
    let graphics = GraphicsInstance::new().ok()?;
    let policy = DeviceSelectionPolicy::default();
    let available = graphics.available_for_surface(None, policy).ok()?;
    let candidate = available.available_devices().into_iter().next()?;
    let device = available
        .create_graphics_device_by_uuid(&graphics, candidate.device_uuid)
        .ok()?;
    Some((graphics, device))
}

#[test]
fn graphics_device_owns_the_destruction_bucket_table() {
    let Some((_graphics, device)) = headless() else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    // 设备内的 ManagedResources 与设备共享同一张表：经任一侧登记，另一侧可见。
    device.resources.begin_command_bucket(7).unwrap();
    assert!(device.disposal().contains(7));
    device.disposal().insert(8, Default::default());
    assert_eq!(
        device.resources.begin_command_bucket(8),
        Err(rhapsody::raw::managed::tracking::Failure::DuplicateBucket)
    );
    // 真实签名 take_out_dropped_waste(&ManagedResources, &GraphicsDevice)：无待排水项时为 0。
    assert_eq!(device.resources.take_out_dropped_waste_on(&device), 0);
    assert_eq!(device.take_out_dropped_waste(), 0);
    // 清理，避免 Drop 里 forget 的动作被误认为泄漏（桶为空）。
    assert!(device.disposal().remove(7).is_empty());
    assert!(device.disposal().remove(8).is_empty());
}

#[test]
fn dropped_resource_goes_to_the_device_bucket_and_retires_with_the_sequence() {
    let Some((_graphics, device)) = headless() else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    let freed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    struct Probe(Arc<std::sync::atomic::AtomicUsize>);
    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    // 命令序号 1 的桶在设备上；`ManagedResources::retire_in_order` 排空的就是设备桶里的动作。
    device.resources.begin_command_bucket(1).unwrap();
    let probe = Probe(freed.clone());
    device
        .disposal()
        .append_existing(1, Box::new(move || drop(probe)))
        .map_err(|_| ())
        .unwrap();
    assert_eq!(device.resources.pending_destroy(), 1);
    assert_eq!(freed.load(std::sync::atomic::Ordering::SeqCst), 0);
    assert_eq!(device.resources.retire_in_order(1), 1);
    assert_eq!(freed.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(!device.disposal().contains(1));
}

#[test]
fn owning_device_id_is_per_resource_table_and_shared_by_its_stores() {
    use rhapsody::raw::managed::resources::ManagedResources;
    let (a, b) = (ManagedResources::new(), ManagedResources::new());
    assert_ne!(a.owning_device_id(), b.owning_device_id());
}

#[test]
fn create_graphics_device_attaches_the_physical_device_wrapper() {
    let Some((graphics, device)) = headless() else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    let attached: &Arc<PhysicalDevice> = device
        .physical_device_info()
        .expect("create_graphics_device 必须挂接物理设备包装");
    assert!(graphics.devices.iter().any(|p| Arc::ptr_eq(p, attached)));
    assert_eq!(attached.handle, device.physical_device());
    // 第二次挂接被拒（原版只设置一次）。
    assert!(
        device
            .attach_physical_device_info(attached.clone())
            .is_err()
    );
    // refresh_memory_budget 经包装委托：扩展门开时给出快照，门关时 Ok(None)。
    let budget = device.refresh_memory_budget().unwrap();
    if device
        .inner
        .enabled_extensions
        .iter()
        .any(|e| e == "VK_EXT_memory_budget")
    {
        assert!(budget.is_some());
        assert!(
            attached.cached_performance_info().is_some(),
            "委托必须发布快照"
        );
    } else {
        assert!(budget.is_none());
    }
}

#[test]
fn track_panics_on_non_recording_state_but_try_track_reports_err() {
    let store = ResourceManager::new();
    let handle = store.allocate(1_u8);
    let bad = CommandState {
        tag: 3,
        sequence: 4,
    };
    assert!(handle.try_track(bad).is_err());
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handle.track(bad)));
    let message = *caught
        .expect_err("原版 track 在状态非 Recording 时 panic")
        .downcast::<String>()
        .unwrap();
    assert!(
        message.starts_with("Can't use a resource for a command buffer in state "),
        "{message}"
    );
    assert_eq!(handle.tracked_sequence(), 0);
    handle.track(CommandState {
        tag: 1,
        sequence: 4,
    });
    assert_eq!(handle.tracked_sequence(), 4);
}

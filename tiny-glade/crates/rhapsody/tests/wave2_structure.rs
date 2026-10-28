//! Typed weak stores and the attached physical-device configuration.
//! 需要 Vulkan 的用例在无实例时跳过（本机用 lavapipe）。
mod gpu_test_support;
use rhapsody::device::Device;
use rhapsody::device::selection::{DeviceSelectionPolicy, GraphicsInstance};
use rhapsody::native::physical_device::PhysicalDevice;
use std::sync::Arc;

fn headless() -> Option<(GraphicsInstance, Arc<Device>)> {
    let result = (|| -> rhapsody::Result<_> {
        let graphics = GraphicsInstance::new()?;
        let available = graphics.available_for_surface(None, DeviceSelectionPolicy::default())?;
        let candidate = available
            .available_devices()
            .into_iter()
            .next()
            .ok_or_else(|| rhapsody::Error::unsupported("no headless candidate"))?;
        let device = Device::new(&graphics.instance, None, &rhapsody::device::DeviceConfig {
            preferred_device: rhapsody::device::DevicePreference::ByUuid(candidate.device_uuid),
            ..rhapsody::device::DeviceConfig::headless()
        })?;
        Ok((graphics, device))
    })();
    gpu_test_support::initialize("managed device tables", result)
}

#[test]
fn device_stores_release_unused_resources_without_sequence_buckets() {
    let Some((_graphics, device)) = headless() else {
        eprintln!("SKIP: Vulkan device unavailable");
        return;
    };
    let buffer = device
        .create_buffer(
            &rhapsody::resource::buffer::BufferDesc::new_gpu_to_cpu(
                16,
                ash::vk::BufferUsageFlags::TRANSFER_DST,
            ),
            "unused",
            (),
        )
        .unwrap();
    let key = buffer.key();
    let weak = Arc::downgrade(&buffer.shared());
    drop(buffer);
    assert!(device.resources().unwrap().buffers.get(key).is_none());
    assert!(weak.upgrade().is_none());
}

#[test]
fn device_tables_have_independent_typed_store_identities() {
    use rhapsody::resource::managed::resources::ManagedResources;
    let (a, b) = (
        ManagedResources::new(Default::default()).unwrap(),
        ManagedResources::new(Default::default()).unwrap(),
    );
    assert_ne!(a.buffers.identity(), a.textures.identity());
    assert_ne!(a.buffers.identity(), b.buffers.identity());
    assert_eq!(a.buffers.identity(), a.buffers.clone().identity());
}

#[test]
fn configured_creation_owns_the_selected_physical_device_metadata() {
    let Some((graphics, device)) = headless() else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    let attached: &Arc<PhysicalDevice> = device.physical_device_info();
    assert!(graphics.devices.iter().any(|p| p.raw_handle() == attached.raw_handle()));
    assert!(Arc::ptr_eq(&graphics.instance, attached.instance()));
    assert_eq!(attached.raw_handle(), device.physical_device());
    // refresh_memory_budget 经包装委托：扩展门开时给出快照，门关时 Ok(None)。
    let budget = device.refresh_memory_budget().unwrap();
    if device
        .enabled_extensions()
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

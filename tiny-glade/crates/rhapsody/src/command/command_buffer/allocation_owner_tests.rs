use super::*;
use crate::device::Device;
#[test]
fn detached_commands_and_attached_wrappers_share_pool_ownership() {
    let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert_ne!(
                std::env::var_os("RHAPSODY_REQUIRE_VULKAN").as_deref(),
                Some(std::ffi::OsStr::new("1")),
                "required GPU initialization failed: {error}"
            );
            return;
        }
    };
    let first = VulkanCommandPool::new(device.clone()).unwrap();
    let command = first.allocate().unwrap();
    let (handle, _, retirement) = command.detach();
    let second = VulkanCommandPool::attach(device, first.family, first.allocation()).unwrap();
    assert!(Arc::ptr_eq(&first.last_use, &second.last_use));
    assert_eq!(first.live_command_count(), 0);
    assert_eq!(
        second.reset_completed(&mut []).unwrap_err().kind(),
        crate::ErrorKind::InvalidUsage
    );
    let mut command = second.adopt(handle, retirement).unwrap();
    command.begin().unwrap();
    second.reset_completed(&mut [&mut command]).unwrap();
    assert!(!command.is_recording());
    assert_eq!(
        first.reset_completed(&mut []).unwrap_err().kind(),
        crate::ErrorKind::InvalidUsage
    );
    drop(command);
    first.reset_completed(&mut []).unwrap();
}

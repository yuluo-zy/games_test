use super::*;
#[test]
fn pinned_allocator_errors_preserve_fatal_categories() {
    assert!(matches!(
        allocation_error(gpu_allocator::AllocationError::OutOfMemory),
        Error::OutOfDeviceMemory
    ));
    for (code, expected) in [
        ("ERROR_DEVICE_LOST", crate::ErrorKind::DeviceLost),
        ("ERROR_OUT_OF_HOST_MEMORY", crate::ErrorKind::OutOfMemory),
    ] {
        let error = gpu_allocator::AllocationError::Internal(format!(
            "Unexpected error in vkAllocateMemory: {code}"
        ));
        assert_eq!(allocation_error(error).kind(), expected);
    }
}

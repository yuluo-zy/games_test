//! Test-only initialization policy. GPU validation jobs set
//! RHAPSODY_REQUIRE_VULKAN=1 so unavailable/broken initialization cannot look like
//! a passing GPU test. Developer machines may still explicitly report SKIP.
#![allow(dead_code)] // each test binary uses a different subset of the shared helpers
pub fn initialize<T, E: std::fmt::Display>(stage: &str, result: Result<T, E>) -> Option<T> {
    let required = std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1");
    initialize_with_requirement(required, stage, result)
}

pub fn initialize_with_requirement<T, E: std::fmt::Display>(
    required: bool,
    stage: &str,
    result: Result<T, E>,
) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            assert!(
                !required,
                "RHAPSODY_REQUIRE_VULKAN=1: GPU initialization failed at {stage}: {error}"
            );
            eprintln!("SKIP Vulkan initialization at {stage}: {error}");
            None
        }
    }
}

/// Drive production admission and pump when a native test needs initial data.
#[allow(dead_code)]
pub fn submit_upload(
    manager: &mut rhapsody::transfer::TransferManager,
    target: std::sync::Arc<rhapsody::resource::buffer::Buffer>,
    offset: u64,
    bytes: &[u8],
) -> rhapsody::Result<rhapsody::transfer::UploadTicket> {
    use rhapsody::transfer::{UploadKey, UploadOptions, UploadPriority};
    let accepted = manager.enqueue_upload(target, offset, bytes, UploadOptions {
        key: UploadKey(u64::MAX), epoch: 0, priority: UploadPriority::High,
    })?;
    manager.flush_uploads(1)?;
    accepted.try_submitted()?.ok_or(rhapsody::Error::Timeout)
}

use super::*;
use crate::device::Device;
use ash::vk;
use std::sync::Arc;
use crate::resource::buffer;
use std::sync::atomic::{AtomicUsize, Ordering};

fn session() -> Option<VulkanGraphSession> {
    let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                "strict graph rejection GPU initialization failed: {error}");
            eprintln!("SKIP graph rejection GPU initialization: {error}");
            return None;
        }
    };
    Some(VulkanGraphSession::new(VulkanGraphBackend::new(device, AccessTable::original()).unwrap()))
}

#[test]
fn rejection_precedes_uninitialized_read_and_compile_allocation_then_next_frame_recovers() {
    let Some(mut session) = session() else { return };
    let device = session.backend.device().clone();
    let desc = buffer::BufferDesc::new_gpu_only(64,
        vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST);
    let imported = Arc::new(buffer::Buffer::new(device.clone(), &desc).unwrap());
    let imported_weak = Arc::downgrade(&imported);
    let mut graph = session.begin_frame().unwrap();
    session.import_frame_buffer(&mut graph, imported, 0x34).unwrap();
    let named = session.named_buffer(&mut graph, "rejection-history", desc).unwrap();
    assert!(!named.history_valid);
    // Named storage is created during declaration and may stay cached;
    // rejecting compilation must not create any additional named owners.
    let named_before_compile = session.named_resource_count();
    assert_eq!(named_before_compile, 1);
    let executed = Arc::new(AtomicUsize::new(0));
    let callback_owner = Arc::new(());
    let callback_weak = Arc::downgrade(&callback_owner);
    let calls = executed.clone();
    let mut reader = graph.add_pass("uninitialized downstream consumer").with_side_effect();
    reader.add_resource_access(named.buffer.raw(), 0x30, 0).unwrap();
    reader.render_with_result_context(move |_| {
        let _retain = &callback_owner;
        calls.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    let retired = Arc::new(AtomicUsize::new(0));
    let calls = retired.clone();
    graph.on_retire(move || { calls.fetch_add(1, Ordering::Relaxed); });
    graph.on_retire(|| panic!("injected retirement failure must not replace OOM"));
    graph.reject(crate::Error::OutOfHostMemory);
    graph.reject(crate::Error::invalid_usage("later producer failure"));
    let submitted = device.retirement_stats().submitted;
    assert!(matches!(session.execute_frame(graph.build()), Err(crate::Error::OutOfHostMemory)));
    assert_eq!(executed.load(Ordering::Relaxed), 0);
    assert_eq!(retired.load(Ordering::Relaxed), 1);
    assert!(callback_weak.upgrade().is_none());
    assert!(imported_weak.upgrade().is_none());
    assert_eq!(session.frame_import_count(), 0);
    assert_eq!(session.pending_frame_count(), 0);
    assert_eq!(session.named_resource_count(), named_before_compile);
    assert!(session.active_prepared.is_none());
    assert!(session.backend.receipt_storage.is_none());
    assert_eq!(device.retirement_stats().submitted, submitted);
    assert!(!session.is_poisoned());
    let mut next = session.begin_frame().unwrap();
    let named = session.named_buffer(&mut next, "rejection-history", desc).unwrap();
    assert!(!named.history_valid);
    session.cancel_frame(next.build()).unwrap();
    let next = session.begin_frame().unwrap();
    let prepared = session.compile_frame(next.build()).unwrap();
    session.cancel_prepared(prepared).unwrap();
}

#[test]
fn cancelling_rejected_graph_retires_once_and_does_not_poison_next_frame() {
    let Some(mut session) = session() else { return };
    let retired = Arc::new(AtomicUsize::new(0));
    let mut graph = session.begin_frame().unwrap();
    let calls = retired.clone();
    graph.on_retire(move || { calls.fetch_add(1, Ordering::Relaxed); });
    graph.reject(crate::Error::OutOfHostMemory);
    session.cancel_frame(graph.build()).unwrap();
    assert_eq!(retired.load(Ordering::Relaxed), 1);
    let graph = session.begin_frame().unwrap();
    let prepared = session.compile_frame(graph.build()).unwrap();
    session.cancel_prepared(prepared).unwrap();
    assert_eq!(retired.load(Ordering::Relaxed), 1);
}

#[test]
fn rejected_device_lost_is_terminal_and_retirement_failure_cannot_replace_it() {
    let Some(mut session) = session() else { return };
    let mut graph = session.begin_frame().unwrap();
    graph.on_retire(|| panic!("cleanup cannot mask device loss"));
    graph.reject(crate::Error::DeviceLost);
    assert!(matches!(session.compile_frame(graph.build()), Err(crate::Error::DeviceLost)));
    assert!(session.is_poisoned());
    assert_eq!(session.pending_frame_count(), 0);
    assert!(session.active_prepared.is_none());
}

//! Regression for the original compile-before-execute boundary. Run with
//! RHAPSODY_REQUIRE_VULKAN=1 so absence of actual GPU execution is a failure.
#[path = "support/mod.rs"]
mod test_support;
use test_support::BridgeTestDriver as _;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use country_core::{asset_libraries::shader_library::AssetShaderLibrary,
    render::{ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice},
        rg_helpers::{rg_boundary::UploadContentVersion, rg_buffer_copy_from_slice_u32}}};
use tiny_glade::render_bridge::CoreRenderBridge;

fn bridge() -> Option<CoreRenderBridge> {
    let result = rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())
        .and_then(|device| CoreRenderBridge::new(device, ".", Arc::new(AssetShaderLibrary::new())));
    match result {
        Ok(bridge) => Some(bridge),
        Err(error) => {
            assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                "strict GPU initialization failed: {error}");
            eprintln!("SKIP compile-before-submit GPU initialization: {error}");
            None
        }
    }
}

#[test]
fn compiled_graph_is_unsubmitted_and_drop_cancels_then_the_next_frame_really_writes() {
    let Some(mut bridge) = bridge() else { return; };
    let calls = Arc::new(AtomicUsize::new(0));
    let retired = Arc::new(AtomicUsize::new(0));
    let calls_in_pass = calls.clone();
    let retired_in_pass = retired.clone();
    let (_, built) = bridge.build_frame(|frame| frame.with_graph(|graph| {
        let mut pass = graph.add_pass("compile cancellation boundary");
        pass.side_effect();
        pass.render_with_result_context(move |_| { calls_in_pass.fetch_add(1, Ordering::SeqCst); Ok(()) });
        graph.on_retire(move || { retired_in_pass.fetch_add(1, Ordering::SeqCst); });
        Ok(())
    })).unwrap();
    let compiled = bridge.compile_built(built).unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 0, "compile must never invoke a render callback");
    assert_eq!(retired.load(Ordering::SeqCst), 0);
    assert_eq!(bridge.with_session(|session| session.pending_frame_count()), 0,
        "compile must never publish a GPU submission");
    drop(compiled);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(retired.load(Ordering::SeqCst), 1, "abandoned compiled graph retires exactly once");

    let buffers = bridge.hosts().buffers.clone();
    let target = buffers.create_buffer(&BufferDesc::new(8, 0x22, 2), "separate compile target", None).unwrap();
    let retired_in_pass = retired.clone();
    let (_, built) = bridge.build_frame(|frame| {
        let id = frame.import_buffer(&target, 0)?;
        frame.with_compute(|compute| rg_buffer_copy_from_slice_u32(compute, id.raw(), 8,
            &[0x12345678, 0x90abcdef], UploadContentVersion::Unknown))?;
        frame.with_graph(|graph| {
            graph.export(id, 0x26)?;
            graph.on_retire(move || { retired_in_pass.fetch_add(1, Ordering::SeqCst); });
            Ok(())
        })
    }).unwrap();
    let compiled = bridge.compile_built(built).unwrap();
    assert_eq!(bridge.with_session(|session| session.pending_frame_count()), 0);
    let receipt = bridge.execute_after_uploads(compiled).unwrap();
    assert!(!receipt.receipts.is_empty(), "execution must contain real Vulkan receipts");
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let expected = [0x12345678_u32.to_ne_bytes(), 0x90abcdef_u32.to_ne_bytes()].concat();
    assert_eq!(&buffers.mapped_slice_mut(&target).unwrap()[..8], expected);
    assert_eq!(retired.load(Ordering::SeqCst), 2);
    assert!(buffers.release(&target));
}

#[test]
fn executing_an_admitted_graph_does_not_flush_the_next_frames_uploads() {
    let Some(mut bridge) = bridge() else { return; };
    let (_, built) = bridge.build_frame(|frame| frame.with_graph(|graph| {
        let mut pass = graph.add_pass("already admitted presentation work");
        pass.side_effect();
        pass.render_with_result_context(|_| Ok(()));
        Ok(())
    })).unwrap();
    let compiled = bridge.compile_built(built).unwrap();
    let buffers = bridge.hosts().buffers.clone();
    let next = buffers.create_buffer(&BufferDesc::new(16, 0x23, 2), "next frame upload", None).unwrap();
    buffers.update_buffer_bytes(&next, &[9, 8, 7, 6], &[0..4], 0).unwrap();
    let before = buffers.queued_upload_stats().unwrap();
    assert!(before.queued_jobs > 0);

    // Intentionally call the production executor directly: this graph's owner
    // has already finished admission; the queued write belongs to its successor.
    let receipt = bridge.execute_compiled(compiled).unwrap();
    assert!(!receipt.receipts.is_empty());
    let after = buffers.queued_upload_stats().unwrap();
    assert_eq!((after.queued_jobs, after.queued_bytes), (before.queued_jobs, before.queued_bytes));
    bridge.wait_submits_done(10_000_000_000).unwrap();
    test_support::until_ready(|| buffers.poll_pending_uploads()).unwrap();
    let device = buffers.graphics_device();
    let before_empty_poll = device.timeline_poll_stats();
    assert!(buffers.poll_pending_uploads().unwrap());
    assert_eq!(device.timeline_poll_stats(), before_empty_poll,
        "an empty owned queue must not poll GPU timelines");
    let mut observed = [0; 4];
    buffers.managed_buffer(&next).unwrap().read_bytes(0, &mut observed).unwrap();
    assert_eq!(observed, [9, 8, 7, 6]);
    assert!(buffers.release(&next));
}

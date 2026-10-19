//! The render-graph resource pool and transient alias cache are evictable:
//! memory pressure releases idle (GPU-complete) pooled resources, least
//! recently used first, and never anything lent to a frame.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{
    device::{AllocationStrategy, Device, DeviceConfig},
    raw::buffer::{Buffer, BufferDesc},
    rg::{
        executor::PassContext,
        graph::BuiltGraph,
        vulkan::{AccessTable, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;

const MIB: u64 = 1 << 20;
const WRITE: u8 = 0x34;
const TIMEOUT: u64 = 10_000_000_000;

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "graph cache eviction",
        // Dedicated allocations make the budget arithmetic exact.
        Device::new_headless(&DeviceConfig {
            allocation: AllocationStrategy::Dedicated,
            ..DeviceConfig::headless()
        }),
    )
}

/// One frame whose only resource is an unaliased GPU-only scratch buffer: it
/// is pooled when the frame completes.
fn scratch_frame(session: &mut VulkanGraphSession, mib: u64, name: &str) -> BuiltGraph {
    let mut graph = session.begin_frame().unwrap();
    let id = session
        .backend
        .declare(
            &mut graph,
            name,
            ResourceSpec::Buffer(BufferDesc::new_gpu_only(
                mib * MIB,
                vk::BufferUsageFlags::TRANSFER_DST,
            )),
        )
        .unwrap();
    let index = id.raw();
    let device = session.backend.device().clone();
    let mut pass = graph.add_pass("fill").with_side_effect();
    pass.add_resource_access(id, WRITE, 0).unwrap();
    pass.render_with_result_context(move |ctx: PassContext<'_>| {
        let managed = ctx
            .managed
            .ok_or_else(|| rhapsody::Error::invalid_usage("missing managed resources"))?;
        let buffer = managed
            .buffer_for(index)
            .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;
        unsafe {
            device.raw_device().cmd_fill_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                buffer.raw(),
                0,
                vk::WHOLE_SIZE,
                7,
            );
        }
        Ok(())
    });
    graph.build()
}

fn big_buffer(device: &Arc<Device>, mib: u64) -> rhapsody::Result<Buffer> {
    Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(mib * MIB, vk::BufferUsageFlags::STORAGE_BUFFER),
    )
}

fn session(device: &Arc<Device>) -> VulkanGraphSession {
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    VulkanGraphSession::new(backend)
}

#[test]
fn pressure_evicts_idle_pooled_resources_only_and_the_allocation_then_succeeds() {
    let Some(device) = device() else {
        return;
    };
    let mut session = session(&device);
    let built = scratch_frame(&mut session, 6, "scratch");
    session.execute_frame(built).unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
    let parked = session.cache_stats();
    assert_eq!(parked.parked_resources, 1, "{parked:?}");
    assert!(parked.parked_bytes >= 6 * MIB, "{parked:?}");
    assert_eq!(device.evictables().len(), 1, "registered with the device");

    // A frame that reuses the pooled resource has it lent: nothing is idle.
    let built = scratch_frame(&mut session, 6, "scratch");
    let prepared = session.compile_frame(built).unwrap();
    assert_eq!(
        session.cache_stats().parked_resources,
        0,
        "the pooled resource is lent to the compiled frame"
    );
    let used = device.allocator_stats().device_memory_usage().total();
    device
        .allocator_stats()
        .set_device_budget(used + 2 * MIB)
        .unwrap();
    assert_eq!(
        device.evict_idle_caches(u64::MAX),
        0,
        "nothing idle to free"
    );
    assert!(
        big_buffer(&device, 8).is_err(),
        "the lent resource must not be evicted to make room"
    );
    // The lent frame still executes on its intact resource.
    session.execute_prepared(prepared).unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
    assert_eq!(session.cache_stats().parked_resources, 1, "parked again");

    // Idle and GPU-complete now: pressure releases it and the allocation fits.
    let big = big_buffer(&device, 8);
    assert!(big.is_ok(), "eviction must make room: {:?}", big.err());
    let after = session.cache_stats();
    assert_eq!(after.parked_resources, 0, "{after:?}");
    assert!(device.evictables().stats().1 >= 6 * MIB);
    drop(big);

    // The session forgets the evicted pool entry and creates a fresh resource.
    device.allocator_stats().set_device_budget(0).unwrap();
    let built = scratch_frame(&mut session, 6, "scratch");
    session.execute_frame(built).unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
    assert_eq!(session.cache_stats().parked_resources, 1);
    device.wait_idle().unwrap();
}

#[test]
fn eviction_is_least_recently_used_first_and_stops_at_the_target() {
    let Some(device) = device() else {
        return;
    };
    let mut session = session(&device);
    // Two differently sized scratch buffers pooled by two frames; the
    // smaller (older) one is parked first.
    let built = scratch_frame(&mut session, 3, "older");
    session.execute_frame(built).unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
    let built = scratch_frame(&mut session, 5, "newer");
    session.execute_frame(built).unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
    let parked = session.cache_stats();
    assert_eq!(parked.parked_resources, 2, "{parked:?}");
    // Asking for a little frees only the least recently used entry.
    let before = device.allocator_stats().device_memory_usage().total();
    let released = device.evict_idle_caches(MIB);
    assert!(released >= 3 * MIB && released < 8 * MIB, "{released}");
    let freed = before - device.allocator_stats().device_memory_usage().total();
    assert!(
        freed >= 3 * MIB,
        "the memory is really freed, not only counted: {freed}"
    );
    let after = session.cache_stats();
    assert_eq!(after.parked_resources, 1, "{after:?}");
    assert!(
        after.parked_bytes >= 5 * MIB,
        "the newer one stays: {after:?}"
    );
    assert!(device.evict_idle_caches(u64::MAX) >= 5 * MIB);
    assert_eq!(session.cache_stats().parked_resources, 0);
    device.wait_idle().unwrap();
}

#[test]
fn caches_unregister_with_the_session_and_hold_no_device_cycle() {
    let Some(device) = device() else {
        return;
    };
    let weak = Arc::downgrade(&device);
    {
        let mut session = session(&device);
        let built = scratch_frame(&mut session, 2, "scratch");
        session.execute_frame(built).unwrap();
        session.wait_submits_done(TIMEOUT).unwrap();
        assert_eq!(device.evictables().len(), 1);
        assert_eq!(session.cache_stats().parked_resources, 1);
        // Pressure with the session alive is fine; dropping it releases the
        // parked resource and the registration.
    }
    assert_eq!(device.evictables().len(), 0, "the weak entry is gone");
    assert_eq!(device.evict_idle_caches(u64::MAX), 0);
    drop(device);
    assert!(weak.upgrade().is_none(), "no Arc cycle through the caches");
}

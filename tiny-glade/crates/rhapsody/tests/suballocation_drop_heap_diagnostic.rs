//! Actual default small Buffer Drop/reclamation diagnostic, not a CPU benchmark.
//! Uses the existing test-only TLS CountingAllocator<System>; production engine
//! allocator, gpu-allocator version, retirement behavior and assertions are unchanged.
#![cfg(feature = "suballocation")]
mod allocation_counter;
mod gpu_test_support;
use allocation_counter::Sample;
use ash::vk;
use rhapsody::device::{AllocationStrategy, Device, DeviceConfig, GpuMemoryUsage};
use rhapsody::command::command_buffer::VulkanCommandPool;
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use serde_json::json;
use std::sync::Arc;

const BYTES: u64 = 64 * 1024;
const COUNT: usize = 128;
const WAIT_NS: u64 = 10_000_000_000;

#[derive(Clone, Copy)]
enum Case {
    Default,
    DedicatedControl,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Default => "actual_default_suballocated",
            Self::DedicatedControl => "dedicated_control",
        }
    }
    fn device(self) -> Arc<Device> {
        let mut config = DeviceConfig::headless();
        assert_eq!(
            config.allocation,
            AllocationStrategy::Suballocated,
            "this diagnostic must exercise the actual default, not replace it"
        );
        if matches!(self, Self::DedicatedControl) {
            config.allocation = AllocationStrategy::Dedicated;
        }
        // Initialization failure is a failure, never a silent passing SKIP.
        let device = gpu_test_support::initialize_with_requirement(
            true,
            self.label(),
            Device::new_headless(&config),
        )
        .expect("required Vulkan initialization");
        assert_eq!(
            device.suballocation_enabled(),
            matches!(self, Self::Default)
        );
        let empty = device.memory_usage_snapshot();
        assert_eq!(
            empty.resource_range_bytes, 0,
            "fresh-device proof excludes unrelated ranges"
        );
        assert_eq!(empty.suballocated_used_bytes, 0);
        assert_eq!(empty.dedicated_bytes, 0);
        device
    }
}
fn small_buffer(device: &Arc<Device>, case: Case) -> Arc<Buffer> {
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(BYTES, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .expect("real small buffer creation"),
    );
    // Existing public diagnostic hook delegates to MemoryBlock::host_range:
    // direct SubAllocation is (offset,size); Dedicated is (0,WHOLE_SIZE).
    // This reads allocation metadata only, never maps or reads GPU bytes.
    let (offset, size) = buffer.host_range_for_test();
    match case {
        Case::Default => {
            assert_ne!(
                size,
                vk::WHOLE_SIZE,
                "actual small Buffer fell back to dedicated"
            );
            assert!(size >= BYTES && offset.checked_add(size).is_some());
        }
        Case::DedicatedControl => assert_eq!((offset, size), (0, vk::WHOLE_SIZE)),
    }
    buffer
}
/// Public production accounting, not a new Buffer getter: on this fresh device
/// every range is one of these actual Buffers. Equality of all live range bytes
/// with suballocated bytes, with zero dedicated bytes, rules out a fallback.
fn prove_backing(case: Case, usage: GpuMemoryUsage) {
    assert!(usage.resource_range_bytes > 0);
    match case {
        Case::Default => {
            assert_eq!(
                usage.resource_range_bytes, usage.suballocated_used_bytes,
                "all actual Buffer ranges must use the dependency suballocator"
            );
            assert_eq!(
                usage.dedicated_bytes, 0,
                "dedicated fallback is not this diagnostic"
            );
            assert!(usage.suballocator_reserved_bytes >= usage.suballocated_used_bytes);
            assert!(usage.native_memory_blocks > 0);
        }
        Case::DedicatedControl => {
            assert_eq!(usage.suballocated_used_bytes, 0);
            assert_eq!(usage.suballocator_reserved_bytes, 0);
            assert_eq!(usage.resource_range_bytes, usage.dedicated_bytes);
        }
    }
}
fn report(
    case: Case,
    phase: &str,
    objects: usize,
    sample: Sample,
    before: GpuMemoryUsage,
    after: GpuMemoryUsage,
    retirement_before: usize,
    retirement_after: usize,
    collected: Option<usize>,
) {
    // Serialization/logging and all snapshots occur outside measured().
    let prefix = (sample.allocations.min(32)) as usize;
    println!(
        "SUBALLOCATION_DROP_RUST_HEAP {}",
        json!({
            "schema":"suballocation-drop-rust-heap-v1","diagnostic_only":true,
            "case":case.label(),"phase":phase,"buffer_count":objects,"requested_buffer_bytes":BYTES,
            "counter_scope":"calling-thread Rust global alloc/alloc_zeroed/realloc attempts and requested bytes",
            "test_global_allocator":"existing allocation_counter::CountingAllocator<System>",
            "rust_allocation_calls":sample.allocations,"rust_requested_bytes":sample.allocated_bytes,
            "allocation_size_prefix":&sample.allocation_sizes[..prefix],
            "allocation_size_prefix_truncated":sample.allocations>32,
            "memory_before":before,"memory_after":after,
            "retirement_pending_before":retirement_before,"retirement_pending_after":retirement_after,
            "collected_native_items":collected,
            "not_measured":["C/driver heap", "other-thread allocations", "deallocation calls", "peak/live Rust heap", "thread CPU", "frame CPU", "production mimalloc overhead"],
            "timing_reported":false,"performance_acceptance":false,
        })
    );
}
fn finish(device: &Arc<Device>) {
    device
        .try_collect_all_completed()
        .expect("final completed cleanup");
    assert_eq!(device.retirement_stats().pending, 0);
    assert_eq!(device.retirement_stats().idle_waits, 0);
    assert_eq!(device.memory_usage_snapshot().resource_range_bytes, 0);
    device.trim_unused_suballocations();
    assert_eq!(
        device.memory_usage_snapshot().application_vk_memory_bytes,
        0
    );
}

#[test]
fn diagnose_unused_small_buffer_drop_default_and_dedicated_control() {
    for case in [Case::Default, Case::DedicatedControl] {
        let device = case.device();
        // Keep a second real owner alive so the default free returns one range,
        // rather than measuring final-shard teardown as if it were the common case.
        let anchor = small_buffer(&device, case);
        let target = small_buffer(&device, case);
        let before = device.memory_usage_snapshot();
        prove_backing(case, before);
        let pending_before = device.retirement_stats().pending;
        assert_eq!(pending_before, 0);
        let (_, sample) = allocation_counter::measured(|| drop(target));
        let after = device.memory_usage_snapshot();
        let pending_after = device.retirement_stats().pending;
        report(
            case,
            "unused_drop_with_live_anchor",
            1,
            sample,
            before,
            after,
            pending_before,
            pending_after,
            None,
        );
        // No assumed allocator-call count. The raw observation may be nonzero.
        assert!(after.resource_range_bytes < before.resource_range_bytes);
        assert_eq!(
            pending_after, 0,
            "never-submitted resource is destroyed immediately today"
        );
        prove_backing(case, after);
        drop(anchor);
        finish(&device);
    }
}

#[test]
fn diagnose_fragmented_completed_reclaim_default_and_dedicated_control() {
    for case in [Case::Default, Case::DedicatedControl] {
        let device = case.device();
        let mut anchors = Vec::with_capacity(COUNT);
        let mut targets = Vec::with_capacity(COUNT / 4);
        for _ in 0..COUNT {
            anchors.push(small_buffer(&device, case));
        }
        let all_live = device.memory_usage_snapshot();
        prove_backing(case, all_live);
        if matches!(case, Case::Default) {
            assert_eq!(
                all_live.native_memory_blocks, 1,
                "controlled fragmentation requires these actual ranges in one native block"
            );
            anchors.sort_unstable_by_key(|buffer| buffer.host_range_for_test().0);
            for pair in anchors.windows(2) {
                let (offset, size) = pair[0].host_range_for_test();
                assert!(
                    offset + size <= pair[1].host_range_for_test().0,
                    "the actual suballocated ranges must not overlap"
                );
            }
        }
        // Free every eighth +3 range before counting. +1/+5 targets remain
        // separated by live anchors, so reclamation creates additional holes.
        // This exercises fragmentation; it does not guarantee a HashSet growth.
        let mut survivors = Vec::with_capacity(COUNT);
        for (i, buffer) in anchors.into_iter().enumerate() {
            match i % 8 {
                3 => drop(buffer),
                1 | 5 => targets.push(buffer),
                _ => survivors.push(buffer),
            }
        }
        assert_eq!(targets.len(), COUNT / 4);
        assert_eq!(survivors.len(), COUNT * 5 / 8);
        let before_submit = device.memory_usage_snapshot();
        prove_backing(case, before_submit);
        let pool = VulkanCommandPool::new(device.clone()).expect("command pool");
        let mut command = pool.allocate().expect("command allocation");
        command.begin().expect("begin real command");
        for buffer in &targets {
            command
                .retain_opaque(buffer.clone())
                .expect("retain real target owner");
            // SAFETY: new native targets are aligned, TRANSFER_DST, live, and
            // retained/tracked by the same real submitted command until completion.
            unsafe {
                device.raw_device().cmd_fill_buffer(
                    command.raw_handle(),
                    buffer.raw_handle(),
                    0,
                    BYTES,
                    7,
                );
            }
        }
        command.end().expect("end real command");
        let pending = command.submit().expect("real native submission");
        let point = pending.point;
        pending
            .wait(WAIT_NS)
            .expect("real timeline completion before measuring reclaim");
        assert!(pending.is_complete().expect("completed point query"));
        // No artificial gate, paused GPU or wait is part of either counted phase.
        drop(targets);
        let before = device.memory_usage_snapshot();
        let queued_before = device.retirement_stats().pending;
        let (_, drop_sample) = allocation_counter::measured(|| drop(pending));
        let queued_after_drop = device.retirement_stats().pending;
        let after_drop = device.memory_usage_snapshot();
        report(
            case,
            "completed_submission_drop_enqueues_reserved_payloads",
            COUNT / 4,
            drop_sample,
            before,
            after_drop,
            queued_before,
            queued_after_drop,
            None,
        );
        assert!(
            queued_after_drop >= COUNT / 4 + 1,
            "buffer and command payloads must really be queued"
        );
        assert_eq!(
            after_drop.resource_range_bytes, before.resource_range_bytes,
            "no target range may be silently reclaimed outside the measured collector"
        );
        assert!(
            device.retirement_stats().completed[point.queue.index()] >= point.value,
            "the actual GPU point completed before this counted phase"
        );
        let (collected, collect_sample) = allocation_counter::measured(|| {
            device
                .try_collect_all_completed()
                .expect("completed reclamation")
        });
        let after_collect = device.memory_usage_snapshot();
        let queued_after_collect = device.retirement_stats().pending;
        report(
            case,
            "fragmented_completed_reclaim_with_live_anchors",
            COUNT / 4,
            collect_sample,
            after_drop,
            after_collect,
            queued_after_drop,
            queued_after_collect,
            Some(collected),
        );
        assert!(collected >= COUNT / 4 + 1);
        assert_eq!(queued_after_collect, 0);
        assert!(after_collect.resource_range_bytes < after_drop.resource_range_bytes);
        prove_backing(case, after_collect);
        drop(survivors);
        drop(pool);
        finish(&device);
    }
}

//! Actual Rust/System allocations in the production VulkanGraphSession path.
//! Construction, begin_frame and GPU completion are measured separately. Native
//! driver allocations and other threads are outside this counter's scope.
mod allocation_counter;
use allocation_counter::{Sample, measured};
use ash::vk::{self, Handle};
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        instance::{InstanceBuilder, ValidationMessenger},
    },
    rg::{
        graph::{self, RenderGraph},
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

const BYTES: u64 = 64;
const HOST_READ: u8 = 0x26;
const COPY_READ: u8 = 0x25;
const COPY_WRITE: u8 = 0x34;
const PASS_NAMES: [&str; 8] = [
    "copy0", "copy1", "copy2", "copy3", "copy4", "copy5", "copy6", "copy7",
];

fn build(
    mut graph: RenderGraph,
    buffers: &[Arc<Buffer>; 2],
    recorded: &Arc<AtomicU64>,
) -> graph::BuiltGraph {
    let handles = [
        graph
            .import_unowned(
                "input",
                ResourceSpec::Buffer(buffers[0].desc).descriptor(),
                11,
                HOST_READ,
            )
            .unwrap(),
        graph
            .import_unowned(
                "output",
                ResourceSpec::Buffer(buffers[1].desc).descriptor(),
                12,
                HOST_READ,
            )
            .unwrap(),
    ];
    // Ping-pong creates real producer/consumer dependencies. No side-effect flag
    // bypasses pure-pass culling; exporting the two outputs retains all 8 copies.
    for (index, name) in PASS_NAMES.iter().enumerate() {
        let source = index % 2;
        let destination = 1 - source;
        let mut pass = graph.add_pass(*name);
        pass.add_resource_access(handles[source], COPY_READ, 0)
            .unwrap();
        pass.add_resource_access(handles[destination], COPY_WRITE, 1)
            .unwrap();
        let device = buffers[source].device.clone();
        let source = buffers[source].handle;
        let destination = buffers[destination].handle;
        let recorded = recorded.clone();
        pass.render_with_result_context(move |context| {
            // SAFETY: these exact same-device buffers are owned graph imports,
            // declared with full matching copy ranges. The production executor
            // retains their RawResource owners and records their real SyncCells.
            unsafe {
                device.raw_device().cmd_copy_buffer(
                    vk::CommandBuffer::from_raw(context.command.handle()),
                    source,
                    destination,
                    &[vk::BufferCopy {
                        src_offset: 0,
                        dst_offset: 0,
                        size: BYTES,
                    }],
                );
            }
            recorded.fetch_add(1, Ordering::Relaxed);
            Ok(())
        });
    }
    for handle in handles {
        graph.export(handle, HOST_READ).unwrap();
    }
    graph.build()
}
fn summary(samples: &[Sample]) -> serde_json::Value {
    let mut durations = samples
        .iter()
        .map(|sample| sample.elapsed_ns)
        .collect::<Vec<_>>();
    durations.sort_unstable();
    serde_json::json!({
        "sampled_frames": samples.len(),
        "mean_allocations": samples.iter().map(|s|s.allocations).sum::<u64>() as f64 / samples.len() as f64,
        "max_allocations": samples.iter().map(|s|s.allocations).max(),
        "mean_requested_bytes": samples.iter().map(|s|s.allocated_bytes).sum::<u64>() / samples.len() as u64,
        "first_sample_sizes":samples[0].allocation_sizes,
        "p95_ns":durations[(durations.len()-1)*95/100],
        "p99_ns":durations[(durations.len()-1)*99/100],
    })
}
#[test]
#[ignore = "explicit strict GPU allocation measurement; run this binary alone"]
fn real_vulkan_session_reports_framework_execution_allocations() {
    let mut instance = InstanceBuilder::default();
    instance.debug_markers();
    instance
        .layers
        .push(std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap());
    let device = Device::new(
        instance.build().unwrap(),
        None,
        &DeviceConfig {
            validation: true,
            ..DeviceConfig::headless()
        },
    )
    .expect("strict Vulkan Session allocation fixture requires a working Vulkan device");
    let messenger = ValidationMessenger::new(device.instance()).unwrap();
    let desc = BufferDesc::new_gpu_to_cpu(
        BYTES,
        vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
    );
    let buffers = [
        Arc::new(Buffer::new(device.clone(), &desc).unwrap()),
        Arc::new(Buffer::new(device.clone(), &desc).unwrap()),
    ];
    let expected: [u8; BYTES as usize] = std::array::from_fn(|index| index as u8 ^ 0x5a);
    buffers[0].write_bytes(0, &expected).unwrap();
    buffers[1].write_bytes(0, &[0; BYTES as usize]).unwrap();
    let mut backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    for (index, buffer) in buffers.iter().enumerate() {
        backend
            .bind_import_named(
                if index == 0 { "input" } else { "output" },
                index as u64 + 11,
                GpuResource::Buffer(buffer.clone()),
            )
            .unwrap();
    }
    let mut session = VulkanGraphSession::new(backend);
    session.chain.parallel = rhapsody::rg::parallel::ParallelRecording::SERIAL;
    let recorded = Arc::new(AtomicU64::new(0));
    for _ in 0..300 {
        let graph = build(session.begin_frame().unwrap(), &buffers, &recorded);
        let receipt = session.execute_frame(graph).unwrap();
        assert_eq!(receipt.stats.culled_passes, 0);
        session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
    }
    #[cfg(unix)]
    if std::env::var_os("RHAPSODY_TRACE_CPU_ALLOCATIONS").is_some() {
        let graph = build(session.begin_frame().unwrap(), &buffers, &recorded);
        allocation_counter::allocation_trace::START.with(|start| {
            start.set(
                std::env::var("RHAPSODY_TRACE_ALLOCATION_START")
                    .ok()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0),
            );
        });
        let completion = std::env::var_os("RHAPSODY_TRACE_COMPLETION").is_some();
        let sample = if completion {
            let receipt = session.execute_frame(graph).unwrap();
            allocation_counter::allocation_trace::ENABLED.with(|enabled| enabled.set(true));
            let ((), sample) =
                measured(|| session.wait_frame(receipt.frame, 10_000_000_000).unwrap());
            allocation_counter::allocation_trace::ENABLED.with(|enabled| enabled.set(false));
            sample
        } else {
            allocation_counter::allocation_trace::ENABLED.with(|enabled| enabled.set(true));
            let (receipt, sample) = measured(|| session.execute_frame(graph).unwrap());
            allocation_counter::allocation_trace::ENABLED.with(|enabled| enabled.set(false));
            session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
            sample
        };
        println!(
            "ALLOCATION_TRACE_PHASE {}",
            if completion { "completion" } else { "execute" }
        );
        allocation_counter::allocation_trace::report(&sample);
        allocation_counter::allocation_trace::START.with(|start| start.set(0));
    }
    let mut begin = Vec::with_capacity(128);
    let mut construction = Vec::with_capacity(128);
    let mut execution = Vec::with_capacity(128);
    let mut completion = Vec::with_capacity(128);
    for _ in 0..128 {
        let (graph, sample) = measured(|| session.begin_frame().unwrap());
        begin.push(sample);
        let (built, sample) = measured(|| build(graph, &buffers, &recorded));
        construction.push(sample);
        let (receipt, sample) = measured(|| session.execute_frame(built).unwrap());
        execution.push(sample);
        assert!(receipt.stats.plan_cache_hit);
        assert_eq!(receipt.stats.culled_passes, 0);
        let ((), sample) = measured(|| session.wait_frame(receipt.frame, 10_000_000_000).unwrap());
        completion.push(sample);
        drop(receipt);
    }
    // Exercise the actual pre-submit admission boundary after measuring the
    // steady state. Retaining CPU receipts must not overwrite old snapshots or
    // create a GPU submission when every configured slot is still borrowed.
    session
        .configure_receipt_storage(rhapsody::rg::receipt_storage::ReceiptStorageConfig {
            max_receipts: 2,
            max_total_bytes: 256 << 10,
            max_cached_bytes: 128 << 10,
        })
        .unwrap();
    let graph = build(session.begin_frame().unwrap(), &buffers, &recorded);
    let first = session.execute_frame(graph).unwrap();
    session.wait_frame(first.frame, 10_000_000_000).unwrap();
    let retained_clone = first.clone();
    drop(first);
    let original_mapping = retained_clone.snapshot.physical_mapping.clone();
    let graph = build(session.begin_frame().unwrap(), &buffers, &recorded);
    let second = session.execute_frame(graph).unwrap();
    session.wait_frame(second.frame, 10_000_000_000).unwrap();
    let before_rejection = recorded.load(Ordering::Relaxed);
    let retired = Arc::new(AtomicU64::new(0));
    let mut graph = session.begin_frame().unwrap();
    let retired_callback = retired.clone();
    graph.on_retire(move || {
        retired_callback.fetch_add(1, Ordering::Relaxed);
    });
    let graph = build(graph, &buffers, &recorded);
    assert!(matches!(
        session.execute_frame(graph),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(retired.load(Ordering::Relaxed), 1);
    assert_eq!(recorded.load(Ordering::Relaxed), before_rejection);
    assert_eq!(session.pending_frame_count(), 0);
    assert!(session.backend.submissions.is_empty());
    assert_eq!(retained_clone.snapshot.physical_mapping, original_mapping);
    drop(retained_clone);
    let graph = build(session.begin_frame().unwrap(), &buffers, &recorded);
    let recovered = session.execute_frame(graph).unwrap();
    session.wait_frame(recovered.frame, 10_000_000_000).unwrap();
    assert_eq!(recorded.load(Ordering::Relaxed), before_rejection + 8);
    drop((second, recovered));
    assert_eq!(session.receipt_storage_statistics().borrowed, 0);
    // A CPU owner-lease admission failure must also stop before the first
    // callback and before any queue call, then permit a normal retry.
    session
        .backend
        .configure_owner_storage(rhapsody::raw::temp_list_pool::TempListPoolConfig {
            max_items: 1,
            ..Default::default()
        })
        .unwrap();
    let before_owner_failure = recorded.load(Ordering::Relaxed);
    let retired_owner_failure = Arc::new(AtomicU64::new(0));
    let mut graph = session.begin_frame().unwrap();
    let retired_callback = retired_owner_failure.clone();
    graph.on_retire(move || {
        retired_callback.fetch_add(1, Ordering::Relaxed);
    });
    let graph = build(graph, &buffers, &recorded);
    assert!(matches!(
        session.execute_frame(graph),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(recorded.load(Ordering::Relaxed), before_owner_failure);
    assert_eq!(retired_owner_failure.load(Ordering::Relaxed), 1);
    assert!(session.backend.submissions.is_empty());
    assert_eq!(session.pending_frame_count(), 0);
    assert_eq!(session.backend.owner_storage_statistics().checked_out, 0);
    session
        .backend
        .configure_owner_storage(Default::default())
        .unwrap();
    let graph = build(session.begin_frame().unwrap(), &buffers, &recorded);
    let recovered = session.execute_frame(graph).unwrap();
    session.wait_frame(recovered.frame, 10_000_000_000).unwrap();
    assert_eq!(recorded.load(Ordering::Relaxed), before_owner_failure + 8);
    drop(recovered);
    for buffer in &buffers {
        let mut actual = [0; BYTES as usize];
        buffer.read_bytes(0, &mut actual).unwrap();
        assert_eq!(actual, expected);
    }
    assert_eq!(recorded.load(Ordering::Relaxed) % 8, 0);
    assert!(recorded.load(Ordering::Relaxed) >= (300 + 128) * 8);
    assert!(
        messenger.collector.errors().is_empty(),
        "{:?}",
        messenger.collector.errors()
    );
    assert!(
        messenger.collector.warnings().is_empty(),
        "{:?}",
        messenger.collector.warnings()
    );
    let report = serde_json::json!({
        "scope":"actual serial VulkanGraphSession; current-thread Rust/System allocations; native driver/other threads excluded; callback emits 8 fixed native buffer copies with no allocations",
        "warmup_frames":300,"resource_count":2,"retained_passes":8,
        "native_submit_api": if session.backend.device.caps().is_enabled(rhapsody::device::Feature::Synchronization2) {"synchronization2"} else {"legacy"},"pixel_or_byte_reference":"both 64-byte GPU copy results exact",
        "begin_frame":summary(&begin),"graph_construction":summary(&construction),
        "framework_execute":summary(&execution),"completion_and_wait":summary(&completion),
        "validation_errors":0,"validation_warnings":0,
        "receipt_admission":"two retained receipt slots reject before recording/submission; retire once; release and recover",
        "owner_admission":"one owner slot rejects before callback/submission; retire once; CPU lease returned; configured limit restored and GPU copy recovered",
    });
    println!("{report}");
    if let Some(path) = std::env::var_os("RHAPSODY_GPU_ALLOCATION_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    if std::env::var_os("RHAPSODY_REQUIRE_ZERO_FRAME_ALLOCATIONS").is_some() {
        assert!(
            execution.iter().all(|sample| sample.allocations == 0),
            "actual Vulkan framework execute still allocates"
        );
        assert!(
            begin.iter().all(|sample| sample.allocations == 0),
            "actual Session begin_frame still allocates"
        );
        assert!(
            completion.iter().all(|sample| sample.allocations == 0),
            "actual Session completion/wait still allocates"
        );
    }
}

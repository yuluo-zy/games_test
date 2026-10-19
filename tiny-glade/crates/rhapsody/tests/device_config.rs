//! 设备配置实际接线：能力、提交入口与帧节流；无 Vulkan 时明确报告跳过。
mod gpu_test_support;
use ash::vk;
use rhapsody::rg::{
    graph::RenderGraph,
    parallel::ParallelRecording,
    vulkan::{AccessTable, VulkanGraphBackend, VulkanGraphSession},
};
use rhapsody::{
    device::{
        AllocationStrategy, Device, DeviceConfig, Feature, QueueClass, RecordingPolicy,
        SubmitBatch, Wait,
    },
    sync::{SyncPoint, SyncSet},
};

fn create(config: &DeviceConfig) -> Option<std::sync::Arc<Device>> {
    gpu_test_support::initialize("configured headless device", Device::new_headless(config))
}

#[test]
fn configured_device_reports_actual_enabled_capabilities_and_allocation_mode() {
    let config = DeviceConfig {
        allocation: AllocationStrategy::Dedicated,
        recording: RecordingPolicy::Adaptive,
        ..DeviceConfig::headless()
    };
    let Some(device) = create(&config) else {
        return;
    };
    assert!(device.caps().is_consistent());
    assert!(device.caps().is_enabled(Feature::TimelineSemaphore));
    assert_eq!(
        device.caps().is_enabled(Feature::BufferDeviceAddress),
        device.capabilities().buffer_device_address
    );
    assert_eq!(device.policy().allocation, AllocationStrategy::Dedicated);
    assert!(!device.suballocation_enabled());
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let session = VulkanGraphSession::new(backend);
    assert_eq!(
        session.chain.parallel.strategy,
        ParallelRecording::ADAPTIVE.strategy
    );
}

#[test]
fn default_device_records_graph_sessions_adaptively() {
    let Some(device) = create(&DeviceConfig::headless()) else {
        return;
    };
    assert_eq!(device.policy().recording, RecordingPolicy::Adaptive);
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let session = VulkanGraphSession::new(backend);
    assert_eq!(session.chain.parallel, ParallelRecording::default());
    assert_eq!(session.chain.parallel, ParallelRecording::ADAPTIVE);
}

#[test]
fn device_submission_rejects_future_waits_without_advancing_timeline() {
    let Some(device) = create(&DeviceConfig::headless()) else {
        return;
    };
    let queue = device.queue(QueueClass::Graphics).unwrap();
    // 空批次只 signal 设备时间线，没有外部句柄或资源生命期要求。
    let first = unsafe { device.submit(queue, &[]) }.unwrap();
    let waits = [Wait::Timeline {
        point: SyncPoint {
            queue,
            value: first.value + 1,
        },
        stage: vk::PipelineStageFlags::ALL_COMMANDS,
    }];
    let batch = SubmitBatch {
        waits: &waits,
        commands: &[],
        signals: &[],
    };
    let error = unsafe { device.submit(queue, &[batch]) }.unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert_eq!(device.timelines().submitted(queue).unwrap(), first.value);
    let waits = [Wait::Timeline {
        point: first,
        stage: vk::PipelineStageFlags::ALL_COMMANDS,
    }];
    let batch = SubmitBatch {
        waits: &waits,
        commands: &[],
        signals: &[],
    };
    let second = unsafe { device.submit(queue, &[batch]) }.unwrap();
    assert_eq!(second.value, first.value + 1);
    device
        .wait(&SyncSet::from_point(second), 10_000_000_000)
        .unwrap();
    assert!(device.poll().unwrap().get(queue) >= second.value);
}

#[test]
fn configured_frames_in_flight_controls_the_session() {
    for limit in [1, 3] {
        let Some(device) = create(&DeviceConfig {
            frames_in_flight: limit,
            ..DeviceConfig::headless()
        }) else {
            return;
        };
        let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
        let mut session = VulkanGraphSession::new(backend);
        for _ in 0..limit {
            let mut graph = session.begin_frame().unwrap();
            graph
                .add_pass("empty command")
                .with_side_effect()
                .render(|_| true);
            session.execute_frame(graph.build()).unwrap();
        }
        assert_eq!(session.pending_frame_count(), limit as usize);
        assert!(session.execute_frame(RenderGraph::new().build()).is_err());
        let mut graph = session.begin_frame().unwrap();
        assert_eq!(session.pending_frame_count(), limit as usize - 1);
        graph
            .add_pass("next frame")
            .with_side_effect()
            .render(|_| true);
        session.execute_frame(graph.build()).unwrap();
        session.wait_submits_done(10_000_000_000).unwrap();
        assert_eq!(session.pending_frame_count(), 0);
    }
}

#[test]
fn an_independent_compute_queue_submits_while_the_graphics_queue_lock_is_held() {
    use rhapsody::device::{
        DevicePreference, FamilyInfo, QueueConfig, QueuePreference, plan_queues,
    };
    use std::{sync::mpsc, thread, time::Duration};

    let Some(probe) = create(&DeviceConfig::headless()) else {
        return;
    };
    let config = DeviceConfig {
        queues: QueueConfig {
            async_compute: QueuePreference::Require,
            require_present: false,
            ..Default::default()
        },
        preferred_device: DevicePreference::ByUuid(probe.device_uuid()),
        ..DeviceConfig::headless()
    };
    let families = unsafe {
        probe
            .instance()
            .instance
            .get_physical_device_queue_family_properties(probe.physical_device())
    }
    .iter()
    .map(|family| FamilyInfo::from_properties(family, false))
    .collect::<Vec<_>>();
    match plan_queues(&families, &config.queues) {
        Ok(_) => {}
        Err(rhapsody::Error::Unsupported(reason)) => {
            eprintln!(
                "SKIP independent queue lock isolation: actual device has no independent compute queue: {reason}"
            );
            return;
        }
        Err(error) => panic!("independent queue planning failed: {error}"),
    }
    drop(probe);
    let device = Device::new_headless(&config).expect(
        "Require compute creation must succeed after the same GPU's queue plan was validated",
    );
    let graphics = device.queue(QueueClass::Graphics).unwrap();
    let compute = device.queue(QueueClass::Compute).unwrap();
    assert_ne!(graphics, compute, "Require must not silently alias compute");
    assert_ne!(
        device.queue_plan().physical(QueueClass::Graphics),
        device.queue_plan().physical(QueueClass::Compute),
        "the test requires two actual Vulkan queues"
    );

    let (locked_tx, locked_rx) = mpsc::sync_channel(1);
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let (done_tx, done_rx) = mpsc::sync_channel(1);
    let holding_device = device.clone();
    let holder = thread::spawn(move || {
        holding_device.timelines().with_queue(graphics, || {
            locked_tx
                .send(())
                .expect("main thread waits for graphics lock");
            release_rx
                .recv_timeout(Duration::from_secs(30))
                .map_err(|_| rhapsody::Error::Timeout)?;
            Ok(())
        })
    });
    let locked = locked_rx.recv_timeout(Duration::from_secs(5));
    let mut submitter = None;
    let completed_before_release = if locked.is_ok() {
        let submitting_device = device.clone();
        let worker = thread::Builder::new()
            .name("independent-compute-submit".into())
            .spawn(move || {
                // This real empty submit has no external command or semaphore
                // lifetimes. Its own queue timeline proves GPU completion.
                let result = unsafe { submitting_device.submit(compute, &[]) }.and_then(|point| {
                    submitting_device.wait(&point.into(), 10_000_000_000)?;
                    Ok(point)
                });
                let _ = done_tx.send(result);
            });
        let started = worker.is_ok();
        submitter = Some(worker);
        started.then(|| done_rx.recv_timeout(Duration::from_secs(5)))
    } else {
        None
    };

    // Always release and join before asserting. A global-lock regression must
    // fail the test, rather than leave a worker holding graphics indefinitely.
    let _ = release_tx.send(());
    let holder_result = holder.join();
    let submitter_result = submitter.map(|worker| worker.map(|worker| worker.join()));
    assert!(
        locked.is_ok(),
        "graphics lock was not acquired in time: {locked:?}"
    );
    holder_result
        .expect("graphics lock holder panicked")
        .unwrap();
    submitter_result
        .expect("compute submitter was started")
        .expect("compute submitter thread creation failed")
        .expect("compute submitter panicked");
    let point = completed_before_release
        .expect("graphics lock was acquired")
        .expect("independent compute work was blocked while the graphics queue lock was held")
        .expect("independent compute submit/wait failed");
    assert_eq!(point.queue, compute);
    assert!(device.poll().unwrap().get(compute) >= point.value);
}

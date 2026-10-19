//! Real Vulkan frame-pool reuse, including a deterministically pending submit.
mod gpu_test_support;

use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig, RecordingPolicy},
    raw::{command_buffer::VulkanCommandPool, semaphore::TimelineSemaphore},
    rg::{
        parallel::ParallelRecording,
        vulkan::{AccessTable, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "frame command pools",
        Device::new_headless(&DeviceConfig::headless()),
    )
}

struct Owner(Arc<AtomicUsize>);
impl Drop for Owner {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}
impl rhapsody::resource::GpuUse for Owner {
    fn visit_uses(&self, _: &mut rhapsody::resource::UseVisitor<'_>) -> rhapsody::Result<()> {
        Ok(())
    }
}

#[test]
fn whole_pool_reset_requires_every_live_command_and_preserves_owners_on_failure() {
    let Some(device) = device() else {
        return;
    };
    let pool = VulkanCommandPool::new(device).unwrap();
    let mut first = pool.allocate().unwrap();
    let mut second = pool.allocate().unwrap();
    let dropped = Arc::new(AtomicUsize::new(0));
    first
        .retain_object(Arc::new(Owner(dropped.clone())))
        .unwrap();
    first.begin().unwrap();
    first.end().unwrap();
    second.begin().unwrap();
    assert_eq!(
        pool.reset_completed(&mut [&mut first]).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert!(first.is_executable());
    assert!(second.is_recording());
    assert_eq!(dropped.load(Ordering::Relaxed), 0);
    assert_eq!(pool.reset_count(), 0);
    pool.reset_completed(&mut [&mut first, &mut second])
        .unwrap();
    assert!(!first.is_executable());
    assert!(!second.is_recording());
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
    assert_eq!(pool.reset_count(), 1);
    let pending = pool.allocate().unwrap();
    std::thread::spawn(move || drop(pending)).join().unwrap();
    assert_eq!(
        pool.pending_free_count(),
        0,
        "a whole-pool reset ends every recording scope"
    );
    first.begin().unwrap();
    first.end().unwrap();
    second.begin().unwrap();
    second.end().unwrap();
}

struct Unblock(Arc<TimelineSemaphore>, bool);
impl Unblock {
    fn signal(&mut self) -> rhapsody::Result<()> {
        if !self.1 {
            unsafe {
                self.0.device.raw_device().signal_semaphore(
                    &vk::SemaphoreSignalInfo::default()
                        .semaphore(self.0.handle)
                        .value(1),
                )
            }?;
            self.1 = true;
        }
        Ok(())
    }
}
impl Drop for Unblock {
    fn drop(&mut self) {
        let _ = self.signal();
    }
}

#[test]
fn dropped_pending_commands_do_not_allow_early_whole_pool_reset() {
    let Some(device) = device() else {
        return;
    };
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let blocker = TimelineSemaphore::new(device.clone(), 0).unwrap();
    // Always release the GPU wait, even when an assertion fails.
    let mut unblock = Unblock(blocker.clone(), false);
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    command.end().unwrap();
    let queue = rhapsody::raw::device::DeviceQueue {
        handle: device.primary_queue(),
        family: device.queue_family_index(),
        index: 0,
    };
    let pending = command
        .submit_to(
            queue,
            &[(blocker, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    drop(pending);
    // CPU validation has ended even though the actual GPU wait is blocked.
    // Its cached Vec may be returned without proving GPU pool completion.
    assert!(pool.cpu_scratch_capacity_bytes() > 0);
    assert_eq!(pool.cpu_scratch_growth_count(), 1);
    pool.trim_cpu_scratch();
    assert_eq!(pool.cpu_scratch_capacity_bytes(), 0);
    assert_eq!(pool.live_command_count(), 0);
    assert!(matches!(
        pool.reset_completed(&mut []),
        Err(rhapsody::Error::Vulkan(vk::Result::NOT_READY))
    ));
    assert_eq!(pool.reset_count(), 0);
    unblock.signal().unwrap();
    device
        .timelines()
        .wait(&point.into(), 10_000_000_000)
        .unwrap();
    pool.reset_completed(&mut []).unwrap();
    assert_eq!(pool.reset_count(), 1);
    device.try_collect_garbage().unwrap();
}

#[test]
fn production_session_uses_bounded_frame_worker_pools_and_whole_pool_reuse() {
    let config = DeviceConfig {
        frames_in_flight: 2,
        recording: RecordingPolicy::Adaptive,
        ..DeviceConfig::headless()
    };
    let Some(device) = gpu_test_support::initialize(
        "parallel frame command pools",
        Device::new_headless(&config),
    ) else {
        return;
    };
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    session.chain.parallel = ParallelRecording::EVERY_PASS;
    for _ in 0..12 {
        let mut graph = session.begin_frame().unwrap();
        for name in ["one", "two", "three", "four"] {
            graph
                .add_pass(name)
                .with_side_effect()
                .render_with_result_context(|context| {
                    let cpu = context.cpu.expect("actual worker owns a CPU arena lease");
                    let values = cpu.slice_with(128, |index| index + context.pass_index)?;
                    assert_eq!(values[127], 127 + context.pass_index);
                    Ok(())
                });
        }
        // Populate every worker in both real frame slots before measuring
        // reuse. Actual CPU timings can otherwise warm fewer PGO chunks, then
        // legitimately add another worker when the callback cost changes.
        session.chain.pgo.previous.reset();
        session
            .chain
            .pgo
            .previous
            .extend(["one", "two", "three", "four"].map(|name| {
                rhapsody::rg::executor::PassPgoDuration::new(
                    rhapsody::rg::executor::PassPgoNameId::from_name(name),
                    1,
                )
            }));
        session.execute_frame(graph.build()).unwrap();
        assert_eq!(session.backend.cpu_arena_statistics().checked_out, 0);
    }
    session.wait_submits_done(10_000_000_000).unwrap();
    let warm_count = session.command_count();
    assert!(
        session.backend.command_pool_count() > 2,
        "recording workers must use distinct actual pools"
    );
    // One pool for the main commands plus one per chunk (4), per frame slot (2):
    // a chunk never shares the main command's pool while both are recording.
    assert!(session.backend.command_pool_count() <= 10);
    assert!(session.backend.command_pool_reset_count() > 0);
    assert!(
        session.backend.cpu_arena_statistics().peak_checked_out >= 2,
        "the frame arena stays borrowed while each actual worker checks out its own arena"
    );
    for _ in 0..32 {
        let mut graph = session.begin_frame().unwrap();
        for name in ["one", "two", "three", "four"] {
            graph.add_pass(name).with_side_effect().render(|_| true);
        }
        session.execute_frame(graph.build()).unwrap();
    }
    session.wait_submits_done(10_000_000_000).unwrap();
    assert_eq!(session.command_count(), warm_count);
}

#[test]
fn production_callbacks_reuse_cpu_arenas_after_recording_completes() {
    let Some(device) = device() else {
        return;
    };
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    for _ in 0..12 {
        let mut graph = session.begin_frame().unwrap();
        graph
            .add_pass("frame CPU scratch")
            .with_side_effect()
            .render_with_result_context(move |context| {
                let cpu = context.cpu.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("production CPU frame arena missing")
                })?;
                let temporary = cpu.slice_with(128, |index| index as u32)?;
                assert_eq!(temporary[127], 127);
                Ok(())
            });
        session.execute_frame(graph.build()).unwrap();
        assert_eq!(session.backend.cpu_arena_statistics().checked_out, 0);
        session.wait_submits_done(10_000_000_000).unwrap();
    }
    assert!(session.backend.command_pool_reset_count() > 0);
    assert_eq!(
        session.backend.cpu_arena_statistics().backing_allocations,
        1
    );
}

#[test]
fn frees_from_other_threads_are_deferred_while_the_pool_records() {
    let Some(device) = device() else {
        return;
    };
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut first = pool.allocate().unwrap();
    let mut second = pool.allocate().unwrap();
    let idle = pool.allocate().unwrap();
    first.begin().unwrap();
    second.begin().unwrap();
    // A never-submitted buffer is freed by retirement immediately; while the
    // pool records on this thread, a free from another thread is deferred.
    std::thread::spawn(move || drop(idle)).join().unwrap();
    assert_eq!(pool.pending_free_count(), 1);
    first.end().unwrap();
    assert_eq!(pool.pending_free_count(), 1, "second still records");
    // Dropping a recording buffer also ends its recording scope.
    drop(second);
    assert_eq!(pool.pending_free_count(), 0);
    first.reset().unwrap();
    first.begin().unwrap();
    first.end().unwrap();
    assert_eq!(pool.live_command_count(), 1);
}

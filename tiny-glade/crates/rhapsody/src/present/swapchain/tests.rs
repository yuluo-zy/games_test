use super::*;
use super::frame_sync::FrameSlotState;
use crate::present::swapchain::policy::check_retired_generation_budget;
use crate::present::swapchain::progress::PendingFence;
use crate::present::swapchain::wsi::WsiFenceOperations;
use crate::present::swapchain::wsi::present_was_enqueued;
use crate::present::swapchain::wsi::suboptimal_requires_recreate;
use crate::present::swapchain::wsi::wsi_call;
use crate::error::ErrorKind;
use ash::vk::Handle;
use std::cell::{Cell, RefCell};

fn certain(serial: u64) -> PendingFence {
    PendingFence {
        serial,
        certain: true,
    }
}
fn uncertain(serial: u64) -> PendingFence {
    PendingFence {
        serial,
        certain: false,
    }
}

fn created_state() -> SwapchainInner {
    let mut state = SwapchainInner::new(SwapchainDesc {
        extent: vk::Extent2D {
            width: 4,
            height: 2,
        },
        vsync: true,
        usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
    });
    state.state = SwapchainState::Created;
    state.stale = false;
    state
}

#[test]
fn fullscreen_focus_loss_blocks_new_acquire_but_allows_owned_frame_to_present() {
    let mut state = created_state();
    state.fullscreen.status.requested = WindowMode::Exclusive { monitor: 42 };
    state.fullscreen.monitor = Some(42);
    state.fullscreen.status.acquired = true;
    state.acquire_with(|| Ok((0, false))).unwrap();
    state.fullscreen.status.focused = false;
    assert!(!state.can_replace());
    assert!(matches!(
        state.acquire_with(|| panic!("unfocused acquire")),
        Err(Error::Vulkan(vk::Result::NOT_READY))
    ));
    let mut progress = PresentProgress::default();
    assert!(
        !state
            .finish_present(&mut progress, true, Ok(false))
            .unwrap()
    );
    state.return_acquisition(true);
    assert!(state.can_replace());
    state.fullscreen.status.focused = true;
    assert_eq!(state.acquire_with(|| Ok((1, false))).unwrap(), (1, false));
}

#[test]
fn windowed_and_borderless_focus_loss_keeps_acquiring() {
    for mode in [WindowMode::Windowed, WindowMode::Borderless] {
        let mut state = created_state();
        state.fullscreen.status.requested = mode;
        state.fullscreen.status.focused = false;
        assert_eq!(state.acquire_with(|| Ok((1, false))).unwrap(), (1, false));
        state.return_acquisition(true);
    }
    // Exclusive requested but fallen back to ordinary presentation.
    let mut state = created_state();
    state.fullscreen.status.requested = WindowMode::Exclusive { monitor: 42 };
    state.fullscreen.status.fallback = Some(FullscreenFallback::Unsupported);
    state.fullscreen.status.focused = false;
    assert!(state.acquire_with(|| Ok((0, false))).is_ok());
}

#[test]
fn fullscreen_present_loss_latches_fallback_and_keeps_wait_consumption_proof() {
    let mut state = created_state();
    state.fullscreen.status.requested = WindowMode::Exclusive { monitor: 42 };
    state.fullscreen.monitor = Some(42);
    state.fullscreen.status.acquired = true;
    let mut progress = PresentProgress::default();
    assert!(
        !state
            .finish_present(
                &mut progress,
                true,
                Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
            )
            .unwrap()
    );
    assert!(state.stale);
    assert!(!state.fullscreen.status.acquired);
    assert_eq!(
        state.fullscreen.status.fallback,
        Some(FullscreenFallback::ExclusiveLost)
    );
    assert_eq!(progress.issued, 1);
    assert_eq!(progress.completed, 0);
}

#[test]
fn fullscreen_minimized_and_released_generations_cannot_acquire() {
    let mut state = created_state();
    state.desc.extent.width = 0;
    assert!(state.acquire_with(|| panic!("minimized acquire")).is_err());
    state.desc.extent.width = 4;
    state.fullscreen.monitor = Some(42);
    assert!(matches!(
        state.acquire_with(|| panic!("released exclusive acquire")),
        Err(Error::SurfaceOutOfDate)
    ));
}

#[derive(Debug, PartialEq, Eq)]
enum FenceCall {
    Wait(vk::Fence, u64),
    Reset(vk::Fence),
    Status(vk::Fence),
}
#[derive(Default)]
struct FaultFences {
    calls: RefCell<Vec<FenceCall>>,
    fail_at: Cell<Option<(usize, vk::Result)>>,
    ready: Cell<bool>,
}
impl FaultFences {
    fn call(&self, operation: FenceCall) -> Result<()> {
        self.calls.borrow_mut().push(operation);
        if let Some((index, error)) = self.fail_at.get()
            && self.calls.borrow().len() == index
        {
            self.fail_at.set(None);
            return Err(error.into());
        }
        Ok(())
    }
}
impl WsiFenceOperations for FaultFences {
    fn wait(&self, fence: vk::Fence, timeout: u64) -> Result<()> {
        self.call(FenceCall::Wait(fence, timeout))
    }
    fn reset(&self, fence: vk::Fence) -> Result<()> {
        self.call(FenceCall::Reset(fence))
    }
    fn signaled(&self, fence: vk::Fence) -> Result<bool> {
        self.call(FenceCall::Status(fence))?;
        Ok(self.ready.get())
    }
}

#[test]
fn production_policy_requires_provable_shutdown_but_acquire_only_does_not() {
    assert_eq!(SwapchainPolicy::default(), SwapchainPolicy::Strict);
    assert_eq!(
        SwapchainPolicy::Strict.validate(false).unwrap_err().kind(),
        ErrorKind::Unsupported
    );
    assert!(SwapchainPolicy::Strict.validate(true).is_ok());
    assert!(SwapchainPolicy::AcquireOnly.validate(false).is_ok());
    assert_eq!(
        SwapchainPolicy::AcquireOnly
            .allow_present()
            .unwrap_err()
            .kind(),
        ErrorKind::Unsupported
    );
}

#[test]
fn live_acquire_state_publishes_nothing_on_failure_and_can_retry() {
    for error in [
        vk::Result::NOT_READY,
        vk::Result::TIMEOUT,
        vk::Result::ERROR_OUT_OF_HOST_MEMORY,
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
        vk::Result::ERROR_DEVICE_LOST,
    ] {
        let mut state = created_state();
        let result = state.acquire_with(|| Err(error.into()));
        assert_eq!(result.unwrap_err().kind(), Error::from(error).kind());
        assert_eq!(state.active_acquisitions, 0);
        assert_eq!(state.next_slot, 0);
        assert!(!state.stale);
        if error == vk::Result::ERROR_DEVICE_LOST {
            // The runtime's wsi_call observes terminal loss before lease
            // cleanup; this state object alone does not own that runtime.
            continue;
        }
        assert_eq!(state.acquire_with(|| Ok((1, false))).unwrap(), (1, false));
        assert_eq!(state.active_acquisitions, 1);
        state.return_acquisition(true);
    }
    let mut state = created_state();
    assert!(state.acquire_with(|| Err(Error::SurfaceOutOfDate)).is_err());
    assert!(state.stale);
    assert_eq!(state.active_acquisitions, 0);
    assert!(
        state
            .acquire_with(|| panic!("stale generations must not call Vulkan"))
            .is_err()
    );
    state.stale = false;
    state.closing = true;
    assert!(
        state
            .acquire_with(|| panic!("closing generations must not call Vulkan"))
            .is_err()
    );
}

#[test]
fn live_slot_failure_matrix_never_resets_before_the_actual_waits_complete() {
    let acquired = vk::Fence::from_raw(11);
    let submitted = vk::Fence::from_raw(12);
    for failure in 1..=3 {
        let operations = FaultFences::default();
        operations.fail_at.set(Some((failure, vk::Result::TIMEOUT)));
        let mut state = FrameSlotState {
            submitted: true,
            acquired: true,
            ..Default::default()
        };
        assert_eq!(
            state
                .reserve_with(&operations, acquired, submitted, 7)
                .unwrap_err()
                .kind(),
            ErrorKind::Timeout
        );
        assert!(!state.reserved);
        assert_eq!(state.submitted, failure == 1);
        assert_eq!(state.acquired, failure <= 2);
        assert_eq!(operations.calls.borrow().len(), failure);
        if failure <= 2 {
            assert!(
                !operations
                    .calls
                    .borrow()
                    .iter()
                    .any(|call| matches!(call, FenceCall::Reset(_)))
            );
        }
        state
            .reserve_with(&operations, acquired, submitted, 9)
            .unwrap();
        assert!(state.reserved);
        assert!(!state.submitted);
        assert!(!state.acquired);
        let calls = operations.calls.borrow().len();
        assert!(
            state
                .reserve_with(&operations, acquired, submitted, 9)
                .is_err()
        );
        assert_eq!(
            operations.calls.borrow().len(),
            calls,
            "a reserved slot cannot mutate Vulkan state"
        );
    }
}

#[test]
fn live_present_fence_timeout_preserves_serial_and_prevents_reset() {
    let fence = vk::Fence::from_raw(17);
    let operations = FaultFences::default();
    operations.fail_at.set(Some((1, vk::Result::TIMEOUT)));
    let mut progress = PresentProgress::default();
    progress.record_result(Ok(false), true);
    assert!(progress.prepare_with(&operations, fence, 7).is_err());
    assert_eq!(progress.pending_fence, Some(certain(1)));
    assert!(!progress.all_presentations_complete());
    assert_eq!(*operations.calls.borrow(), [FenceCall::Wait(fence, 7)]);
    progress.prepare_with(&operations, fence, 9).unwrap();
    assert_eq!(progress.pending_fence, None);
    assert!(progress.all_presentations_complete());
    assert_eq!(
        *operations.calls.borrow(),
        [
            FenceCall::Wait(fence, 7),
            FenceCall::Wait(fence, 9),
            FenceCall::Reset(fence)
        ]
    );
}

#[test]
fn live_reclaim_poll_does_not_publish_completion_on_status_or_wait_failure() {
    let fence = vk::Fence::from_raw(19);
    for failure in 1..=2 {
        let operations = FaultFences::default();
        operations.ready.set(true);
        operations
            .fail_at
            .set(Some((failure, vk::Result::ERROR_OUT_OF_HOST_MEMORY)));
        let mut progress = PresentProgress::default();
        progress.record_result(Ok(false), true);
        assert_eq!(
            progress.poll_with(&operations, fence).unwrap_err().kind(),
            ErrorKind::OutOfMemory
        );
        assert_eq!(progress.pending_fence, Some(certain(1)));
        assert!(!progress.all_presentations_complete());
        progress.poll_with(&operations, fence).unwrap();
        assert!(progress.all_presentations_complete());
        assert_eq!(progress.pending_fence, None);
        assert!(
            !operations
                .calls
                .borrow()
                .iter()
                .any(|call| matches!(call, FenceCall::Reset(_)))
        );
    }
}

#[test]
fn live_shutdown_rejects_unproven_unextended_present_without_gpu_wait_shortcut() {
    let operations = FaultFences::default();
    let mut progress = PresentProgress::default();
    progress.record_result(Ok(false), false);
    assert_eq!(
        progress
            .finish_shutdown_with(&operations, vk::Fence::null(), 7)
            .unwrap_err()
            .kind(),
        ErrorKind::Unsupported
    );
    assert!(!progress.all_presentations_complete());
    assert!(
        operations.calls.borrow().is_empty(),
        "there is no invented queue-idle proof"
    );
    // The production newer-generation protocol calls this only after a
    // real completed present in the replacement generation.
    progress.generation_completed();
    progress
        .finish_shutdown_with(&operations, vk::Fence::null(), 7)
        .unwrap();
    assert!(progress.all_presentations_complete());
}

#[test]
fn live_shutdown_timeout_retains_pending_fence_for_a_successful_retry() {
    let fence = vk::Fence::from_raw(23);
    let operations = FaultFences::default();
    operations.fail_at.set(Some((1, vk::Result::TIMEOUT)));
    let mut progress = PresentProgress::default();
    progress.record_result(Ok(false), true);
    assert_eq!(
        progress
            .finish_shutdown_with(&operations, fence, 7)
            .unwrap_err()
            .kind(),
        ErrorKind::Timeout
    );
    assert_eq!(progress.pending_fence, Some(certain(1)));
    assert!(!progress.all_presentations_complete());
    progress
        .finish_shutdown_with(&operations, fence, 9)
        .unwrap();
    assert!(progress.all_presentations_complete());
    assert_eq!(progress.pending_fence, None);
    assert_eq!(
        *operations.calls.borrow(),
        [FenceCall::Wait(fence, 7), FenceCall::Wait(fence, 9)]
    );
}

#[test]
fn live_present_error_matrix_retains_enqueued_operations_only() {
    for (error, enqueued) in [
        (vk::Result::ERROR_OUT_OF_DATE_KHR, true),
        (vk::Result::ERROR_SURFACE_LOST_KHR, true),
        (vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT, true),
        (vk::Result::ERROR_OUT_OF_HOST_MEMORY, false),
        (vk::Result::ERROR_OUT_OF_DEVICE_MEMORY, false),
        (vk::Result::ERROR_DEVICE_LOST, false),
    ] {
        let mut inner = created_state();
        let mut progress = PresentProgress::default();
        let result = inner.finish_present(&mut progress, true, Err(error));
        assert_eq!(inner.native_present_successes, 0);
        assert!(inner.stale);
        assert_eq!(progress.issued, u64::from(enqueued));
        // Rejected presents enqueue their wait, but their fence is not
        // guaranteed to signal: it is polled, never waited on.
        assert_eq!(progress.pending_fence, enqueued.then_some(uncertain(1)));
        assert_eq!(progress.all_presentations_complete(), !enqueued);
        if matches!(
            error,
            vk::Result::ERROR_OUT_OF_DATE_KHR
                | vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
        ) {
            assert!(!result.unwrap());
        } else {
            assert_eq!(result.unwrap_err().kind(), Error::from(error).kind());
        }
    }
}

#[test]
fn native_present_success_count_excludes_recoverable_errors_and_retains_successes() {
    let mut inner = created_state();
    let mut progress = PresentProgress::default();
    for error in [
        vk::Result::ERROR_OUT_OF_DATE_KHR,
        vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
    ] {
        assert!(!inner.finish_present(&mut progress, false, Err(error)).unwrap());
        assert!(inner.any_presented, "保留原可恢复结果的记账");
        assert_eq!(inner.native_present_successes, 0);
    }
    assert!(!inner.finish_present(&mut progress, false, Ok(false)).unwrap());
    assert_eq!(inner.native_present_successes, 1);
    assert!(inner.finish_present(&mut progress, false, Ok(true)).unwrap());
    assert_eq!(inner.native_present_successes, 2);
    assert!(inner.finish_present(&mut progress, false, Err(vk::Result::ERROR_DEVICE_LOST)).is_err());
    assert_eq!(inner.native_present_successes, 2);
}

struct TestTimelineApi;
impl crate::sync::timeline::TimelineApi for TestTimelineApi {
    fn counter_value(&self, _: vk::Semaphore) -> Result<u64> {
        Ok(0)
    }
    fn wait_all(&self, _: &[vk::Semaphore], _: &[u64], _: u64) -> Result<bool> {
        Ok(true)
    }
}

#[test]
fn rapid_resize_generation_policy_applies_bounded_backpressure_until_collection() {
    let mut ancestors = 0;
    let mut accepted = 0;
    for _ in 0..100 {
        if check_retired_generation_budget(ancestors).is_ok() {
            ancestors += 1;
            accepted += 1;
        }
        assert!(ancestors <= MAX_RETIRED_GENERATIONS);
    }
    assert_eq!(accepted, MAX_RETIRED_GENERATIONS);
    assert_eq!(
        check_retired_generation_budget(ancestors)
            .unwrap_err()
            .kind(),
        ErrorKind::Exhausted
    );
    // The current chain remains owned on rejection; completion collection
    // can release ancestors and admit a later replacement without a reset.
    ancestors -= 1;
    assert!(check_retired_generation_budget(ancestors).is_ok());
    ancestors += 1;
    assert!(check_retired_generation_budget(ancestors).is_err());
    ancestors = 0;
    assert!(check_retired_generation_budget(ancestors).is_ok());
}

#[test]
fn wsi_device_loss_is_sticky_before_cleanup_can_submit_again() {
    use crate::sync::{QueueId, timeline::Timelines};
    use std::cell::Cell;
    for operation in [
        "acquire",
        "fence wait",
        "fence status",
        "fence reset",
        "release",
        "creation",
    ] {
        let timelines = Timelines::new(TestTimelineApi, &[vk::Semaphore::null()]).unwrap();
        let calls = Cell::new(0);
        let error = wsi_call(&timelines, || {
            calls.set(calls.get() + 1);
            Err::<(), _>(vk::Result::ERROR_DEVICE_LOST)
        })
        .unwrap_err();
        assert!(error.is_device_lost(), "{operation}");
        assert!(timelines.is_device_lost(), "{operation}");
        assert!(
            wsi_call(&timelines, || {
                calls.set(calls.get() + 1);
                Ok::<_, vk::Result>(())
            })
            .unwrap_err()
            .is_device_lost()
        );
        assert_eq!(calls.get(), 1, "known-lost cleanup must not call Vulkan");
        let result: Result<(crate::sync::SyncPoint, ())> = timelines
            .submit(QueueId::new(0).unwrap(), |_| {
                panic!("a WSI loss must block subsequent queue submission")
            });
        assert!(result.unwrap_err().is_device_lost());
    }
}

#[test]
fn recoverable_wsi_errors_keep_their_category_and_allow_retry() {
    use crate::sync::timeline::Timelines;
    let timelines = Timelines::new(TestTimelineApi, &[vk::Semaphore::null()]).unwrap();
    for error in [
        vk::Result::TIMEOUT,
        vk::Result::ERROR_OUT_OF_HOST_MEMORY,
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
        vk::Result::ERROR_OUT_OF_DATE_KHR,
        vk::Result::ERROR_SURFACE_LOST_KHR,
    ] {
        let observed = wsi_call(&timelines, || Err::<(), _>(error)).unwrap_err();
        assert_eq!(observed.kind(), Error::from(error).kind());
        assert!(!timelines.is_device_lost());
        assert!(wsi_call(&timelines, || Ok::<_, vk::Result>(())).is_ok());
    }
    assert!(
        wsi_call(&timelines, || Err::<(), _>(Error::DeviceLost))
            .unwrap_err()
            .is_device_lost()
    );
    assert!(
        timelines.is_device_lost(),
        "internal rescue command errors are observed too"
    );
}

#[test]
fn acquire_suboptimal_keeps_image_usable_and_only_out_of_date_rebuilds() {
    let mut state = created_state();
    assert_eq!(state.acquire_with(|| Ok((2, true))).unwrap(), (2, true));
    state.return_acquisition(true);
    assert!(!state.stale);
    for (error, kind) in [
        (vk::Result::NOT_READY, ErrorKind::Vulkan),
        (vk::Result::TIMEOUT, ErrorKind::Timeout),
    ] {
        assert_eq!(
            state.acquire_with(|| Err(error.into())).unwrap_err().kind(),
            kind
        );
        assert!(!state.stale);
    }
    assert_eq!(
        state
            .acquire_with(|| Err(vk::Result::ERROR_OUT_OF_DATE_KHR.into()))
            .unwrap_err()
            .kind(),
        ErrorKind::SurfaceOutOfDate
    );
    assert!(state.stale);
}

#[test]
fn swapchain_state_lives_in_one_rwlock() {
    let desc = SwapchainDesc {
        extent: vk::Extent2D {
            width: 4,
            height: 2,
        },
        vsync: true,
        usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
    };
    let lock: RwLock<SwapchainInner> = RwLock::new(SwapchainInner::new(desc));
    {
        let (r1, r2) = (lock.read(), lock.read());
        assert_eq!(r1.desc, r2.desc);
        assert!(lock.try_write().is_none(), "有读者时写锁不可得");
    }
    let mut w = lock.write();
    assert_eq!(w.state, SwapchainState::NotCreated);
    assert!(w.stale, "未创建视为陈旧");
    assert!(lock.try_read().is_none(), "写者独占");
    w.state = SwapchainState::Created;
}

#[test]
fn present_progress_requires_acquisition_completion_for_the_same_image() {
    let mut a = PresentProgress::default();
    let mut b = PresentProgress::default();
    let first = a.issue();
    let other = b.issue();
    b.acquired(other);
    assert!(!a.completed(first));
    a.acquired(first);
    assert!(a.completed(first));
    let second = a.issue();
    assert!(!a.completed(second));
    a.acquired(first);
    assert!(!a.completed(second));
}

#[test]
fn present_failure_matrix_distinguishes_enqueued_waits_from_rejected_submits() {
    for error in [
        vk::Result::ERROR_OUT_OF_DATE_KHR,
        vk::Result::ERROR_SURFACE_LOST_KHR,
        vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
    ] {
        assert!(present_was_enqueued(Err(error)));
    }
    for error in [
        vk::Result::ERROR_OUT_OF_HOST_MEMORY,
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
        vk::Result::ERROR_DEVICE_LOST,
    ] {
        assert!(!present_was_enqueued(Err(error)));
    }
    assert!(present_was_enqueued(Ok(false)));
    assert!(present_was_enqueued(Ok(true)));
}

#[test]
fn generation_replacement_waits_for_every_acquisition_to_return() {
    let mut state = SwapchainInner::new(SwapchainDesc {
        extent: vk::Extent2D {
            width: 4,
            height: 2,
        },
        vsync: true,
        usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
    });
    state.stale = false;
    state.acquired();
    state.acquired();
    assert!(!state.can_replace());
    state.return_acquisition(true);
    assert!(
        !state.can_replace(),
        "the second lease may still submit an old present"
    );
    assert!(!state.stale);
    state.return_acquisition(false);
    assert!(state.can_replace());
    assert!(
        state.stale,
        "an abandoned raw acquisition requires retirement"
    );
}

#[test]
fn rejected_present_does_not_publish_a_fence_or_completion_serial() {
    let mut state = PresentProgress::default();
    for error in [
        vk::Result::ERROR_OUT_OF_HOST_MEMORY,
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
        vk::Result::ERROR_DEVICE_LOST,
    ] {
        state.record_result(Err(error), true);
        assert_eq!(state.issued, 0);
        assert_eq!(state.completed, 0);
        assert_eq!(state.pending_fence, None);
    }
    state.record_result(Ok(false), true);
    assert_eq!(state.issued, 1);
    assert_eq!(state.pending_fence, Some(certain(1)));
    assert!(!state.completed(1));
    state.acquired(1);
    state.pending_fence = None;
    state.record_result(Err(vk::Result::ERROR_OUT_OF_DATE_KHR), true);
    assert_eq!(state.issued, 2);
    assert_eq!(state.pending_fence, Some(uncertain(2)));
    assert!(!state.completed(2));
}

#[test]
fn rejected_present_cleanup_keeps_its_submit_point_after_wait_timeout() {
    use crate::sync::{CompletedValues, QueueId, SyncPoint};
    let queue = QueueId::new(0).unwrap();
    let other_queue = QueueId::new(1).unwrap();
    let mut state = PresentProgress::default();
    let render = SyncPoint { queue, value: 4 };
    state.record_gpu_submission(render);
    state.record_result(Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY), true);
    let mut completed = CompletedValues::default();
    completed.raise(queue, render.value);
    assert!(state.retirement_ready(&completed));
    // The cleanup queue submit succeeds, but its bounded wait times out.
    // Completion of render/present alone must no longer permit retirement.
    let cleanup = SyncPoint { queue, value: 5 };
    state.record_gpu_submission(cleanup);
    assert!(!state.retirement_ready(&completed));
    completed.raise(other_queue, u64::MAX);
    assert!(!state.retirement_ready(&completed));
    assert_eq!(state.gpu_use.get(queue), cleanup.value);
    completed.raise(queue, cleanup.value);
    assert!(state.retirement_ready(&completed));
}

#[test]
fn recovery_render_submission_also_blocks_retirement_until_gpu_completion() {
    use crate::sync::{CompletedValues, QueueId, SyncPoint};
    let queue = QueueId::new(0).unwrap();
    let point = SyncPoint { queue, value: 7 };
    let mut state = PresentProgress::default();
    state.record_gpu_submission(point);
    state.record_result(Ok(false), false);
    // A present/acquire completion proof cannot erase a queued GPU use.
    state.acquired(state.issued);
    let mut completed = CompletedValues::default();
    assert!(!state.retirement_ready(&completed));
    completed.raise(queue, point.value);
    assert!(state.retirement_ready(&completed));
}

#[test]
fn unextended_present_completion_requires_reacquisition_proof() {
    let mut state = PresentProgress::default();
    state.record_result(Ok(false), false);
    state.record_result(Err(vk::Result::ERROR_SURFACE_LOST_KHR), false);
    assert_eq!(state.issued, 2);
    assert_eq!(state.pending_fence, None);
    assert!(!state.completed(1));
    state.acquired(1);
    assert!(state.completed(1));
    assert!(!state.completed(2));
}

#[test]
fn wsi_result_policy_classifies_nonfatal_codes() {
    assert_eq!(present_result(Ok(false)).unwrap(), (false, false));
    // SUBOPTIMAL alone does not retire; note_suboptimal decides.
    assert_eq!(present_result(Ok(true)).unwrap(), (true, false));
    assert_eq!(
        present_result(Err(vk::Result::ERROR_OUT_OF_DATE_KHR)).unwrap(),
        (false, true)
    );
    assert_eq!(
        present_result(Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)).unwrap(),
        (false, true)
    );
    assert_eq!(
        present_result(Err(vk::Result::ERROR_SURFACE_LOST_KHR))
            .unwrap_err()
            .kind(),
        ErrorKind::SurfaceLost
    );
}

#[test]
fn rejected_present_fence_is_polled_never_waited() {
    let fence = vk::Fence::from_raw(29);
    for error in [
        vk::Result::ERROR_OUT_OF_DATE_KHR,
        vk::Result::ERROR_SURFACE_LOST_KHR,
        vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
    ] {
        // Unsignaled: reuse is refused without blocking.
        let operations = FaultFences::default();
        let mut progress = PresentProgress::default();
        progress.record_result(Err(error), true);
        assert_eq!(
            progress
                .prepare_with(&operations, fence, 7)
                .unwrap_err()
                .kind(),
            ErrorKind::SurfaceOutOfDate
        );
        assert_eq!(*operations.calls.borrow(), [FenceCall::Status(fence)]);
        assert_eq!(progress.pending_fence, Some(uncertain(1)));
        // Teardown relies on the render timeline proof, never a wait.
        progress
            .finish_shutdown_with(&operations, fence, 7)
            .unwrap();
        assert!(progress.all_presentations_complete());
        assert!(
            !operations
                .calls
                .borrow()
                .iter()
                .any(|call| matches!(call, FenceCall::Wait(..))),
            "{error:?}"
        );
        // Signaled: consumed and reusable.
        let operations = FaultFences::default();
        operations.ready.set(true);
        let mut progress = PresentProgress::default();
        progress.record_result(Err(error), true);
        progress.prepare_with(&operations, fence, 7).unwrap();
        assert!(progress.all_presentations_complete());
        assert_eq!(
            *operations.calls.borrow(),
            [FenceCall::Status(fence), FenceCall::Reset(fence)]
        );
    }
}

#[test]
fn persistent_suboptimal_recreates_only_on_surface_change() {
    let extent = vk::Extent2D {
        width: 64,
        height: 32,
    };
    let identity = vk::SurfaceTransformFlagsKHR::IDENTITY;
    let caps = |width, transform| vk::SurfaceCapabilitiesKHR {
        current_extent: vk::Extent2D { width, height: 32 },
        current_transform: transform,
        ..Default::default()
    };
    assert!(!suboptimal_requires_recreate(
        &caps(64, identity),
        extent,
        identity
    ));
    assert!(suboptimal_requires_recreate(
        &caps(65, identity),
        extent,
        identity
    ));
    assert!(suboptimal_requires_recreate(
        &caps(64, vk::SurfaceTransformFlagsKHR::ROTATE_90),
        extent,
        identity
    ));
    // Window-defined extent: only an explicit resize retires the chain.
    assert!(!suboptimal_requires_recreate(
        &caps(u32::MAX, identity),
        extent,
        identity
    ));
}

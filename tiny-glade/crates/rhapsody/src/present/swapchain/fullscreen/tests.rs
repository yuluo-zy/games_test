use super::*;

fn exclusive() -> FullscreenState {
    let mut state = FullscreenState::default();
    state.status.requested = WindowMode::Exclusive { monitor: 42 };
    state.replacement(true)
}

#[test]
fn fullscreen_acquire_release_are_idempotent_and_retired_handles_are_never_called() {
    let mut state = exclusive();
    assert!(!state.acquire(|| Ok(())).unwrap());
    assert!(state.status.acquired);
    state.acquire(|| panic!("duplicate acquire")).unwrap();
    state.release(|| Ok(())).unwrap();
    state.release(|| panic!("duplicate release")).unwrap();
    state.retired = true;
    state.acquire(|| panic!("retired acquire")).unwrap();
    state.status.acquired = true;
    state.release(|| panic!("retired release")).unwrap();
}

#[test]
fn fullscreen_denial_downgrades_once_and_does_not_swallow_oom_or_device_loss() {
    let mut state = exclusive();
    assert!(
        state
            .acquire(|| Err(vk::Result::ERROR_INITIALIZATION_FAILED.into()))
            .unwrap()
    );
    let mut fallback = state.replacement(true);
    assert_eq!(fallback.monitor, None);
    assert_eq!(
        fallback.status.fallback,
        Some(FullscreenFallback::AcquisitionFailed)
    );
    fallback
        .acquire(|| panic!("fallback must not acquire"))
        .unwrap();
    for error in [
        Error::OutOfHostMemory,
        Error::OutOfDeviceMemory,
        Error::DeviceLost,
        Error::SurfaceLost,
    ] {
        let mut state = exclusive();
        let expected = error.kind();
        assert_eq!(state.acquire(|| Err(error)).unwrap_err().kind(), expected);
        assert_eq!(state.status.fallback, None);
        assert!(!state.status.acquired);
    }
}

#[test]
fn fullscreen_release_failure_preserves_ownership_for_retry() {
    let mut state = exclusive();
    state.acquire(|| Ok(())).unwrap();
    assert!(state.release(|| Err(Error::OutOfHostMemory)).is_err());
    assert!(state.status.acquired);
    state.release(|| Ok(())).unwrap();
    assert!(!state.status.acquired);
}

#[test]
fn fullscreen_unfocused_and_unsupported_paths_do_not_call_extension() {
    let mut state = exclusive();
    state.status.focused = false;
    state.acquire(|| panic!("unfocused acquire")).unwrap();
    let mut unsupported = state.replacement(false);
    assert_eq!(unsupported.monitor, None);
    assert_eq!(
        unsupported.status.fallback,
        Some(FullscreenFallback::Unsupported)
    );
    unsupported
        .acquire(|| panic!("unsupported extension"))
        .unwrap();
}

#[test]
fn fullscreen_native_loss_preserves_requested_mode_and_latches_fallback() {
    let mut state = exclusive();
    state.acquire(|| Ok(())).unwrap();
    state.lost();
    assert!(!state.status.acquired);
    let next = state.replacement(true);
    assert_eq!(next.status.requested, WindowMode::Exclusive { monitor: 42 });
    assert_eq!(next.monitor, None);
    assert_eq!(
        next.status.fallback,
        Some(FullscreenFallback::ExclusiveLost)
    );
}

#[test]
fn fullscreen_only_a_real_focus_restoration_retries_latched_fallback() {
    let mut state = exclusive();
    state.lost();
    assert!(!state.set_focused(true));
    assert_eq!(
        state.status.fallback,
        Some(FullscreenFallback::ExclusiveLost)
    );
    assert!(!state.set_focused(false));
    let mut next = state.replacement(true);
    assert!(!next.status.focused);
    assert!(next.set_focused(true));
    assert_eq!(next.status.fallback, None);
    let mut next = next.replacement(true);
    assert_eq!(next.monitor, Some(42));
    next.acquire(|| Ok(())).unwrap();
    assert!(next.status.acquired);
}

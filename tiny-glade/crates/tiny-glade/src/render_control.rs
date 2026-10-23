//! Bounded, coalesced desired window state. Native operations stay on the
//! resident render owner; posting never reclaims a frame or waits on the GPU.
use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct WindowControls {
    pub resize: Option<[u32; 2]>,
    pub suspended: Option<bool>,
    pub focused: Option<bool>,
    pub vsync: Option<bool>,
    pub stale: bool,
    pub cancel_preparation: bool,
}
impl WindowControls {
    fn is_empty(self) -> bool { self == Self::default() }
}
#[derive(Default)]
struct State {
    requested: u64,
    desired: WindowControls,
    pending: WindowControls,
}
#[derive(Default)]
pub(crate) struct WindowControlMailbox {
    state: Mutex<State>,
    applied: AtomicU64,
    error: Mutex<Option<anyhow::Error>>,
}
impl WindowControlMailbox {
    pub fn update(&self, update: impl FnOnce(&mut WindowControls)) {
        let mut state = self.state.lock();
        let before = state.desired;
        update(&mut state.desired);
        let after = state.desired;
        if before == after { return; }
        state.requested = state.requested.checked_add(1).expect("window control generation exhausted");
        if before.resize != after.resize { state.pending.resize = after.resize; }
        if before.suspended != after.suspended { state.pending.suspended = after.suspended; }
        if before.focused != after.focused { state.pending.focused = after.focused; }
        if before.vsync != after.vsync { state.pending.vsync = after.vsync; }
    }
    pub fn invalidate_surface(&self) {
        let mut state = self.state.lock();
        if !state.pending.stale {
            state.requested = state.requested.checked_add(1).expect("window control generation exhausted");
            state.pending.stale = true;
        }
    }
    pub fn cancel_preparation(&self) {
        let mut state = self.state.lock();
        if !state.pending.cancel_preparation {
            state.requested = state.requested.checked_add(1).expect("window control generation exhausted");
            state.pending.cancel_preparation = true;
        }
    }
    pub fn take(&self) -> Option<(u64, WindowControls)> {
        let mut state = self.state.lock();
        if state.pending.is_empty() { return None; }
        Some((state.requested, std::mem::take(&mut state.pending)))
    }
    pub fn complete(&self, generation: u64) { self.applied.store(generation, Ordering::Release); }
    pub fn fail(&self, error: anyhow::Error) {
        let mut first = self.error.lock();
        if first.is_none() { *first = Some(error); }
    }
    pub fn take_error(&self) -> Option<anyhow::Error> { self.error.lock().take() }
    pub fn progress(&self) -> (u64, u64) {
        (self.state.lock().requested, self.applied.load(Ordering::Acquire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coalesces_to_latest_state_and_acknowledges_only_applied_generations() {
        let controls = WindowControlMailbox::default();
        for x in 0..10_000 {
            controls.update(|state| { state.resize = Some([x, 720]); state.focused = Some(x%2 == 0); });
        }
        controls.update(|state| state.suspended = Some(true));
        let (generation, last) = controls.take().unwrap();
        assert_eq!(last.resize, Some([9_999, 720]));
        assert_eq!(last.focused, Some(false));
        assert_eq!(last.suspended, Some(true));
        assert_eq!(controls.progress(), (generation, 0));
        assert!(controls.take().is_none());
        controls.update(|state| state.suspended = Some(false));
        controls.complete(generation);
        let (next, resume) = controls.take().unwrap();
        assert!(next > generation);
        assert_eq!(resume.suspended, Some(false));
        assert_eq!(resume.resize, None);
        controls.complete(next);
        controls.update(|state| state.suspended = Some(false));
        assert!(controls.take().is_none(), "duplicate desired state must not rebuild the swapchain");
        assert_eq!(controls.progress(), (next, next));
    }
    #[test]
    fn repeated_surface_invalidation_survives_a_previous_acknowledgement() {
        let controls = WindowControlMailbox::default();
        for _ in 0..2 {
            controls.invalidate_surface();
            let (generation, pending) = controls.take().unwrap();
            assert!(pending.stale);
            controls.complete(generation);
        }
        assert_eq!(controls.progress(), (2, 2));
    }
    #[test]
    fn reports_the_first_worker_error_without_overwriting_it() {
        let controls = WindowControlMailbox::default();
        controls.fail(anyhow::anyhow!("resize"));
        controls.fail(anyhow::anyhow!("focus"));
        assert_eq!(controls.take_error().unwrap().to_string(), "resize");
        assert!(controls.take_error().is_none());
    }
}

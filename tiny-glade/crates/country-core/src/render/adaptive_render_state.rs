//! Runtime VRAM recovery state, shared by startup, App handoff and rendering.
//! Native main_loop 1404dce76..1404dd1dd: reduce immediately, retry after
//! 250ms, double the retry after recovery/renewed pressure, then cool down.
//! Native render 1404e90e3 marks OOM as tag 2, which never auto-restores.
use std::time::{Duration, Instant};

#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdaptiveRenderState {
    pub enabled: bool,
    phase: AdaptivePhase,
    requested: Option<Option<u32>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AdaptivePhase {
    Initial,
    Waiting { interval: Duration, deadline: Instant },
    OutOfMemory,
    Cooldown { interval: Duration, deadline: Instant },
}

impl AdaptiveRenderState {
    pub fn at_startup(ignore_vram_budget: bool) -> Self {
        Self { enabled: !ignore_vram_budget, phase: AdaptivePhase::Initial, requested: None }
    }
    pub fn native_tag(&self) -> u64 {
        match self.phase { AdaptivePhase::Initial => 0, AdaptivePhase::Waiting { .. } => 1,
            AdaptivePhase::OutOfMemory => 2, AdaptivePhase::Cooldown { .. } => 3 }
    }
    pub fn mark_out_of_memory(&mut self) { self.phase = AdaptivePhase::OutOfMemory; }
    /// 原设置页显式修改缩放后重启预算状态；不改变用户开关和请求值。
    /// 与逐帧request_scale不同，即使数值相同也必须清除旧OOM状态。
    pub fn reset_after_manual_scale_change(&mut self) { self.phase = AdaptivePhase::Initial; }
    /// Track explicit settings changes without resetting a retained OOM limit
    /// when the loading World hands its resources to the gameplay World.
    pub fn request_scale(&mut self, requested: Option<f32>) -> bool {
        let requested = requested.map(f32::to_bits);
        let changed = self.requested.is_some_and(|previous| previous != requested);
        self.requested = Some(requested);
        if changed { self.phase = AdaptivePhase::Initial; }
        changed
    }

    pub fn update(&mut self, now: Instant, current: f32, requested: f32, maximum: f32) -> f32 {
        let maximum = if self.enabled { maximum } else { f32::MAX };
        if current > maximum {
            let interval = match self.phase {
                AdaptivePhase::Initial => Duration::from_millis(250),
                AdaptivePhase::Waiting { interval, .. } => interval,
                AdaptivePhase::OutOfMemory => return maximum,
                AdaptivePhase::Cooldown { interval, .. } => interval.checked_mul(2).expect("adaptive retry duration overflow"),
            };
            self.phase = AdaptivePhase::Waiting { interval, deadline: now + interval };
            return maximum;
        }
        match self.phase {
            AdaptivePhase::Waiting { interval, deadline } if now >= deadline && maximum >= requested => {
                let interval = interval.checked_mul(2).expect("adaptive retry duration overflow");
                self.phase = AdaptivePhase::Cooldown { interval, deadline: now + interval.max(Duration::from_secs(2)) };
                requested
            }
            AdaptivePhase::Cooldown { deadline, .. } if now >= deadline => {
                self.phase = AdaptivePhase::Initial;
                current
            }
            _ => current,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pressure_recovers_only_after_deadline_and_uses_exponential_backoff() {
        let mut state = AdaptiveRenderState::at_startup(false);
        let now = Instant::now();
        assert_eq!(state.update(now, 1.0, 1.0, 0.7), 0.7);
        assert_eq!(state.native_tag(), 1);
        assert_eq!(state.update(now + Duration::from_millis(249), 0.7, 1.0, 2.0), 0.7);
        assert_eq!(state.update(now + Duration::from_millis(250), 0.7, 1.0, 0.9), 0.7);
        assert_eq!(state.update(now + Duration::from_millis(300), 0.7, 1.0, 2.0), 1.0);
        assert_eq!(state.native_tag(), 3);
        assert_eq!(state.update(now + Duration::from_millis(400), 1.0, 1.0, 0.6), 0.6);
        assert_eq!(state.update(now + Duration::from_millis(1399), 0.6, 1.0, 2.0), 0.6);
        assert_eq!(state.update(now + Duration::from_millis(1400), 0.6, 1.0, 2.0), 1.0);
        assert_eq!(state.update(now + Duration::from_millis(3400), 1.0, 1.0, 2.0), 1.0);
        assert_eq!(state.native_tag(), 0);
    }
    #[test]
    fn manual_target_is_budget_limited_and_oom_never_automatically_recovers() {
        let mut state = AdaptiveRenderState::at_startup(false);
        let now = Instant::now();
        assert_eq!(state.update(now, 1.5, 1.5, 0.8), 0.8);
        state.mark_out_of_memory();
        assert_eq!(state.update(now + Duration::from_secs(60), 0.8, 1.5, 3.0), 0.8);
        assert_eq!(state.update(now + Duration::from_secs(61), 0.8, 1.5, 0.5), 0.5);
        assert_eq!(state.native_tag(), 2);
        let mut ignored = AdaptiveRenderState::at_startup(true);
        assert_eq!(ignored.update(now, 1.5, 1.5, 0.0), 1.5);
        assert_eq!(ignored.native_tag(), 0);
    }
}

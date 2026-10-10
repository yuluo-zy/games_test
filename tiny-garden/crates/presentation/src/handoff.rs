//! 所有展示类型共用的交接时钟：冷管线预热、淡化、主导拾取姿态和结束回收。
//! 类型负责构件和材质，公共状态不接收领域对象或网格身份。
#[derive(Default)]
pub(crate) struct CoherentHandoff {
    pub started: bool,
    pub switched: bool,
    pub age: f32,
    pub elapsed: f32,
}
pub(crate) struct HandoffStep {
    pub alpha: f32,
    pub started_now: bool,
    pub switched_now: bool,
    pub complete: bool,
}
impl CoherentHandoff {
    pub fn pipeline_ready(&self, dt: f32, enabled: bool, ready: bool) -> bool {
        ready && (!enabled || self.age + dt > 0.05)
    }
    pub fn advance(
        &mut self,
        dt: f32,
        duration: f32,
        ready: bool,
        elapsed: Option<f32>,
    ) -> Option<HandoffStep> {
        self.age += dt;
        if !self.started && !ready {
            return None;
        }
        let started_now = !self.started;
        self.started = true;
        self.elapsed = elapsed.unwrap_or(self.elapsed + dt);
        let t = (self.elapsed / duration.max(0.001)).clamp(0., 1.);
        let alpha = t * t * (3. - 2. * t);
        let switched_now = !self.switched && alpha >= 0.5;
        self.switched |= switched_now;
        Some(HandoffStep {
            alpha,
            started_now,
            switched_now,
            complete: t >= 1.,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_clock_keeps_pick_switch_and_completion_in_sync() {
        let mut a = CoherentHandoff::default();
        let mut b = CoherentHandoff::default();
        assert!(a.advance(0.016, 0.16, false, None).is_none());
        let first = a.advance(0.016, 0.16, true, Some(0.08)).unwrap();
        let second = b.advance(0.016, 0.16, true, Some(0.08)).unwrap();
        assert_eq!(first.alpha, second.alpha);
        assert!(first.switched_now && second.switched_now);
        assert!(a.advance(0.016, 0.16, false, Some(0.16)).unwrap().complete);
    }
}

//! Opt-in submission phase tracing (`RHAPSODY_SUBMIT_TRACE=1`).

/// 仅在 RHAPSODY_SUBMIT_TRACE=1 时启用的提交分段观测，开关每进程读取一次。
/// 禁用时不取时间、不分配；启用时所有样本也放在固定栈数组中。
pub(crate) struct SubmitTrace {
    active: Option<SubmitTraceActive>,
}
struct SubmitTraceActive {
    scope: &'static str,
    start: std::time::Instant,
    last: std::time::Instant,
    phases: [(&'static str, u64); 12],
    phase_count: usize,
    counts: [(&'static str, usize); 6],
    count_count: usize,
}
impl SubmitTrace {
    pub(crate) fn configured() -> bool {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ENABLED.get_or_init(|| {
            std::env::var_os("RHAPSODY_SUBMIT_TRACE").is_some_and(|value| value == "1")
        })
    }
    pub(crate) fn new(scope: &'static str) -> Self {
        Self {
            active: Self::configured().then(|| {
                let now = std::time::Instant::now();
                SubmitTraceActive {
                    scope,
                    start: now,
                    last: now,
                    phases: [("", 0); 12],
                    phase_count: 0,
                    counts: [("", 0); 6],
                    count_count: 0,
                }
            }),
        }
    }
    pub(crate) fn enabled(&self) -> bool {
        self.active.is_some()
    }
    pub(crate) fn mark(&mut self, phase: &'static str) {
        let Some(active) = &mut self.active else {
            return;
        };
        let now = std::time::Instant::now();
        if let Some(slot) = active.phases.get_mut(active.phase_count) {
            *slot = (
                phase,
                now.duration_since(active.last)
                    .as_nanos()
                    .min(u64::MAX as u128) as u64,
            );
            active.phase_count += 1;
        }
        active.last = now;
    }
    pub(crate) fn count(&mut self, name: &'static str, value: usize) {
        let Some(active) = &mut self.active else {
            return;
        };
        if let Some(slot) = active.counts.get_mut(active.count_count) {
            *slot = (name, value);
            active.count_count += 1;
        }
    }
}
impl Drop for SubmitTrace {
    fn drop(&mut self) {
        let Some(active) = &self.active else {
            return;
        };
        use std::io::Write as _;
        // 诊断输出失败不得覆盖 Vulkan 错误，也不能使原清理路径 panic。
        let _ = writeln!(
            std::io::stderr().lock(),
            "RHAPSODY_SUBMIT_TRACE scope={} total_ns={} phases_ns={:?} counts={:?}",
            active.scope,
            active.start.elapsed().as_nanos(),
            &active.phases[..active.phase_count],
            &active.counts[..active.count_count]
        );
    }
}

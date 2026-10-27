//! CPU-side recording profile: per-thread scopes, spans and counters for the phases of
//! recording a pass or command. Disabled unless a scope is active; all state is
//! thread-local, so it never contends.
pub(crate) mod command_workload;
pub use command_workload::CommandWorkload;

// 仅作 CPU 录制观测；字段均为 inclusive，不能相加当作帧墙钟。
#[derive(Clone, Copy)]
#[repr(usize)]
pub(crate) enum RecordPhase {
    Requirements,
    CompleteGroup,
    GroupLockWait,
    RequireSlots,
    DescriptorReserve,
    UpdateDriver,
    ManagedValidate,
    Retain,
    NativeBind,
    NativePush,
    Finish,
    WithPush,
    FinishConstants,
    FinishLease,
}
const RECORD_PHASES: usize = 14;
const RECORD_PHASE_NAMES: [&str; RECORD_PHASES] = [
    "Requirements",
    "CompleteGroup",
    "GroupLockWait",
    "RequireSlots",
    "DescriptorReserve",
    "UpdateDriver",
    "ManagedValidate",
    "Retain",
    "NativeBind",
    "NativePush",
    "Finish",
    "WithPush",
    "FinishConstants",
    "FinishLease",
];
#[derive(Clone, Copy)]
#[repr(usize)]
pub(crate) enum RecordCount {
    RequirementResources,
    RequirementUnique,
    GroupAttempts,
    CurrentPageReuse,
    PageSearches,
    PagesCreated,
    InsufficientRetries,
    SampledImageWrites,
    StorageImageWrites,
    StorageBufferWrites,
    SamplerWrites,
    BindingLeaseInputs,
    PassLeaseChecks,
    CoverageAccesses,
    RetainSuccesses,
    LocalCacheHits,
    LocalCacheMisses,
    RequirementsCalls,
    CompleteGroupCalls,
    DescriptorReserveCalls,
    NativeBindCalls,
    NativePushCalls,
    FinishCalls,
    WithPushCalls,
    BindingChecks,
}
const RECORD_COUNTS: usize = 25;
const RECORD_COUNT_NAMES: [&str; RECORD_COUNTS] = [
    "RequirementResources",
    "RequirementUnique",
    "GroupAttempts",
    "CurrentPageReuse",
    "PageSearches",
    "PagesCreated",
    "InsufficientRetries",
    "SampledImageWrites",
    "StorageImageWrites",
    "StorageBufferWrites",
    "SamplerWrites",
    "BindingLeaseInputs",
    "PassLeaseChecks",
    "CoverageAccesses",
    "RetainSuccesses",
    "LocalCacheHits",
    "LocalCacheMisses",
    "RequirementsCalls",
    "CompleteGroupCalls",
    "DescriptorReserveCalls",
    "NativeBindCalls",
    "NativePushCalls",
    "FinishCalls",
    "WithPushCalls",
    "BindingChecks",
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RecordNumbers {
    phases: [u64; RECORD_PHASES],
    counts: [u64; RECORD_COUNTS],
}
impl RecordNumbers {
    const EMPTY: Self = Self {
        phases: [0; RECORD_PHASES],
        counts: [0; RECORD_COUNTS],
    };
}
thread_local! {
    // 热路径关闭时只读这个常量初始化的整数，不碰统计、时钟或全局原子。
    static RECORD_ACTIVE: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static RECORD_NUMBERS: std::cell::Cell<RecordNumbers> = const { std::cell::Cell::new(RecordNumbers::EMPTY) };
}
struct RecordScopeActive<'a> {
    id: u64,
    previous_id: u64,
    previous_numbers: RecordNumbers,
    start: std::time::Instant,
    graph: u64,
    pass: usize,
    command: u64,
    name: &'a str,
    callback_ok: Option<bool>,
    #[cfg(feature = "cpu-probe")]
    native_workload: Option<CommandWorkload>,
    emit: bool,
}
pub(crate) struct RecordScope<'a> {
    active: Option<RecordScopeActive<'a>>,
    _not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}
impl<'a> RecordScope<'a> {
    pub(crate) fn new(graph: u64, pass: usize, command: u64, name: &'a str) -> Self {
        Self::begin(
            crate::trace::SubmitTrace::configured(),
            graph,
            pass,
            command,
            name,
            true,
        )
    }
    fn begin(
        enabled: bool,
        graph: u64,
        pass: usize,
        command: u64,
        name: &'a str,
        emit: bool,
    ) -> Self {
        let active = enabled
            .then(|| {
                use std::sync::atomic::{AtomicU64, Ordering};
                static NEXT: AtomicU64 = AtomicU64::new(1);
                // 耗尽后停用观测，禁止诊断 ID 环绕成关闭值或重复身份。
                let id = NEXT
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
                        old.checked_add(1)
                    })
                    .ok()?;
                let previous_numbers =
                    RECORD_NUMBERS.with(|numbers| numbers.replace(RecordNumbers::EMPTY));
                let previous_id = RECORD_ACTIVE.with(|active| active.replace(id));
                Some(RecordScopeActive {
                    id,
                    previous_id,
                    previous_numbers,
                    start: std::time::Instant::now(),
                    graph,
                    pass,
                    command,
                    name,
                    callback_ok: None,
                    #[cfg(feature = "cpu-probe")]
                    native_workload: None,
                    emit,
                })
            })
            .flatten();
        Self {
            active,
            _not_send: std::marker::PhantomData,
        }
    }
    #[cfg(any(not(feature = "cpu-probe"), test))]
    pub(crate) fn complete(&mut self, ok: bool) {
        if let Some(active) = &mut self.active {
            active.callback_ok = Some(ok);
        }
    }
    /// Existing trace-only event: successful recording is not GPU submission.
    /// Off keeps no workload copy and emits no additional event or allocation.
    #[cfg(feature = "cpu-probe")]
    pub(crate) fn complete_with_workload(&mut self, ok: bool, workload: &std::cell::Cell<CommandWorkload>) {
        if let Some(active) = &mut self.active {
            let mut workload = workload.get();
            if !ok { workload.invalidate(); }
            active.callback_ok = Some(ok);
            active.native_workload = Some(workload);
        }
    }
}
impl Drop for RecordScope<'_> {
    fn drop(&mut self) {
        let Some(active) = &self.active else { return };
        let total_ns = active.start.elapsed().as_nanos();
        let numbers = RECORD_NUMBERS.with(|numbers| numbers.replace(active.previous_numbers));
        RECORD_ACTIVE.with(|id| id.set(active.previous_id));
        // 嵌套/错误/panic 均先恢复父 scope，再做可能阻塞的诊断 I/O。
        if active.emit {
            use std::io::Write as _;
            let phases: [(&str, u64); RECORD_PHASES] =
                std::array::from_fn(|i| (RECORD_PHASE_NAMES[i], numbers.phases[i]));
            let counts: [(&str, u64); RECORD_COUNTS] =
                std::array::from_fn(|i| (RECORD_COUNT_NAMES[i], numbers.counts[i]));
            #[cfg(feature = "cpu-probe")]
            let _ = writeln!(
                std::io::stderr().lock(),
                "RHAPSODY_RECORD_TRACE id={} graph={} pass={} command={} name={:?} callback_ok={:?} unwind={} total_ns={} inclusive_phases_ns={:?} counts={:?} native_workload={:?}",
                active.id,
                active.graph,
                active.pass,
                active.command,
                active.name,
                active.callback_ok,
                std::thread::panicking(),
                total_ns,
                phases,
                counts,
                active.native_workload
            );
            #[cfg(not(feature = "cpu-probe"))]
            let _ = writeln!(
                std::io::stderr().lock(),
                "RHAPSODY_RECORD_TRACE id={} graph={} pass={} command={} name={:?} callback_ok={:?} unwind={} total_ns={} inclusive_phases_ns={:?} counts={:?}",
                active.id,
                active.graph,
                active.pass,
                active.command,
                active.name,
                active.callback_ok,
                std::thread::panicking(),
                total_ns,
                phases,
                counts
            );
        }
    }
}
struct RecordSpanActive {
    id: u64,
    phase: RecordPhase,
    start: std::time::Instant,
}
pub(crate) struct RecordSpan {
    active: Option<RecordSpanActive>,
    _not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}
impl RecordSpan {
    #[inline]
    pub(crate) fn new(phase: RecordPhase) -> Self {
        let id = RECORD_ACTIVE.with(std::cell::Cell::get);
        Self {
            active: (id != 0).then(|| RecordSpanActive {
                id,
                phase,
                start: std::time::Instant::now(),
            }),
            _not_send: std::marker::PhantomData,
        }
    }
}
impl Drop for RecordSpan {
    #[inline]
    fn drop(&mut self) {
        let Some(active) = &self.active else { return };
        if RECORD_ACTIVE.with(std::cell::Cell::get) != active.id {
            return;
        }
        let elapsed = active.start.elapsed().as_nanos().min(u64::MAX as u128) as u64;
        RECORD_NUMBERS.with(|numbers| {
            let mut next = numbers.get();
            next.phases[active.phase as usize] =
                next.phases[active.phase as usize].saturating_add(elapsed);
            numbers.set(next);
        });
    }
}
#[inline]
pub(crate) fn record_count(field: RecordCount, count: usize) {
    if RECORD_ACTIVE.with(std::cell::Cell::get) == 0 {
        return;
    }
    RECORD_NUMBERS.with(|numbers| {
        let mut next = numbers.get();
        next.counts[field as usize] = next.counts[field as usize].saturating_add(count as u64);
        numbers.set(next);
    });
}
/// 紧循环只更新两个局部整数，退出时才访问一次 TLS 统计。
pub(crate) struct RecordTally {
    id: u64,
    fields: [RecordCount; 2],
    counts: [std::cell::Cell<u64>; 2],
    _not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}
impl RecordTally {
    pub(crate) fn new(fields: [RecordCount; 2]) -> Self {
        Self {
            id: RECORD_ACTIVE.with(std::cell::Cell::get),
            fields,
            counts: [std::cell::Cell::new(0), std::cell::Cell::new(0)],
            _not_send: std::marker::PhantomData,
        }
    }
    #[inline]
    pub(crate) fn add(&self, field: usize) {
        if self.id != 0 {
            self.counts[field].set(self.counts[field].get().saturating_add(1));
        }
    }
}
impl Drop for RecordTally {
    fn drop(&mut self) {
        if self.id == 0 || RECORD_ACTIVE.with(std::cell::Cell::get) != self.id {
            return;
        }
        RECORD_NUMBERS.with(|numbers| {
            let mut next = numbers.get();
            for (field, count) in self.fields.into_iter().zip(&self.counts) {
                next.counts[field as usize] =
                    next.counts[field as usize].saturating_add(count.get());
            }
            numbers.set(next);
        });
    }
}

#[cfg(test)]
mod record_trace_tests;

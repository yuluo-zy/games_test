//! CPU upload scheduling. Payloads and task envelopes have independent, bounded
//! host ownership; publishing a GPU point never needs a new allocation.
use crate::{
    Error, Result,
    device::{BudgetAction, BudgetCategory},
    raw::allocator::{AllocatorStats, CpuMemoryReservation, shared_cpu_bytes},
    sync::SyncPoint,
};
use parking_lot::Mutex;
use std::sync::{
    Arc, OnceLock, Weak,
    atomic::{AtomicUsize, Ordering},
};

// Diagnostic identity is unique only inside this process. Never compare it
// across binary runs; run identity remains the caller's report responsibility.
#[cfg(any(test, feature = "cpu-probe"))]
static NEXT_MANAGER_PROBE_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
#[cfg(any(test, feature = "cpu-probe"))]
fn next_manager_probe_id() -> u64 {
    NEXT_MANAGER_PROBE_ID.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| next.checked_add(1)).unwrap_or(0)
}

#[derive(Clone, Copy, Debug)]
pub struct UploadQueueConfig {
    pub max_jobs: usize,
    pub max_keys: usize,
    /// Includes completed task envelopes still held by callers.
    pub max_tickets: usize,
    pub max_regions_per_job: usize,
    /// Known host capacity, including temporary replacement payloads and all
    /// outstanding ticket envelopes. GPU staging has its own TransferConfig.
    pub max_cpu_bytes: usize,
}
impl Default for UploadQueueConfig {
    fn default() -> Self {
        Self {
            max_jobs: 64,
            max_keys: 128,
            max_tickets: 128,
            max_regions_per_job: 64,
            max_cpu_bytes: 32 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UploadKey(pub u64);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum UploadPriority {
    Background,
    #[default]
    Normal,
    High,
}
#[derive(Clone, Copy, Debug)]
pub struct UploadOptions {
    pub key: UploadKey,
    pub epoch: u64,
    pub priority: UploadPriority,
}

/// Pending replacements may supersede older replacements of the same key.
/// Ordered deltas are never merged or implicitly cancelled: each accepted
/// interval remains a separate FIFO job, including across frame epochs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UploadMode {
    Replacement,
    OrderedDelta,
}

/// Failed admission returns the caller's allocation unchanged for retry.
#[derive(Debug)]
pub struct OwnedUploadError {
    pub error: Error,
    pub payload: Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UploadCancellation {
    Superseded,
    Explicit,
    ManagerDropped,
}
#[derive(Clone, Debug)]
pub enum UploadStatus {
    Pending,
    Submitted(SyncPoint),
    Cancelled(UploadCancellation),
    Failed(Arc<UploadFailure>),
}
/// Shared failure retains its own preallocated CPU envelope even after the task
/// slot is reused or the manager is dropped.
pub struct UploadFailure {
    error: OnceLock<Error>,
    _memory: Reservation,
}
impl UploadFailure {
    pub fn error(&self) -> &Error {
        self.error
            .get()
            .expect("failure is published only after initialization")
    }
    pub fn to_error(self: &Arc<Self>) -> Error {
        match self.error() {
            Error::DeviceLost => Error::DeviceLost,
            Error::OutOfDeviceMemory => Error::OutOfDeviceMemory,
            Error::OutOfHostMemory => Error::OutOfHostMemory,
            Error::SurfaceOutOfDate => Error::SurfaceOutOfDate,
            Error::SurfaceLost => Error::SurfaceLost,
            Error::Timeout => Error::Timeout,
            Error::Exhausted(what) => Error::Exhausted(what.clone()),
            Error::Unsupported(what) => Error::Unsupported(what.clone()),
            Error::Vulkan(code) => Error::Vulkan(*code),
            Error::InvalidUsage { what, .. } => Error::InvalidUsage {
                what: what.clone(),
                source: Some(Box::new(self.clone())),
            },
        }
    }
}
impl std::fmt::Debug for UploadFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.error(), f)
    }
}
impl std::fmt::Display for UploadFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.error(), f)
    }
}
impl std::error::Error for UploadFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.error())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UploadQueueStats {
    pub queued_jobs: usize,
    pub queued_bytes: usize,
    pub known_cpu_bytes: usize,
    pub task_slots: usize,
    pub active_keys: usize,
    pub merged_requests: u64,
    pub cancelled_jobs: u64,
    pub submitted_jobs: u64,
    pub deferred_pumps: u64,
    pub evicted_ticket_slots: u64,
}
struct LocalBudget {
    used: AtomicUsize,
    limit: usize,
    stats: Arc<AllocatorStats>,
    _envelope: CpuMemoryReservation,
}
struct Reservation {
    local: Arc<LocalBudget>,
    bytes: usize,
    global: CpuMemoryReservation,
}
impl Reservation {
    fn new(local: &Arc<LocalBudget>, bytes: usize) -> Result<Self> {
        local
            .used
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(bytes).filter(|next| *next <= local.limit)
            })
            .map_err(|_| Error::Timeout)?;
        match local
            .stats
            .reserve_cpu_bytes(bytes as u64, BudgetCategory::Live)
        {
            Ok(global) => Ok(Self {
                local: local.clone(),
                bytes,
                global,
            }),
            Err(error) => {
                local.used.fetch_sub(bytes, Ordering::AcqRel);
                Err(error)
            }
        }
    }
    fn resize(&mut self, bytes: usize) -> Result<()> {
        if bytes > self.bytes {
            let extra = bytes - self.bytes;
            self.local
                .used
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                    used.checked_add(extra)
                        .filter(|next| *next <= self.local.limit)
                })
                .map_err(|_| Error::Timeout)?;
            if let Err(error) = self.global.resize(bytes as u64) {
                self.local.used.fetch_sub(extra, Ordering::AcqRel);
                return Err(error);
            }
        } else {
            self.global.resize(bytes as u64)?;
            self.local
                .used
                .fetch_sub(self.bytes - bytes, Ordering::AcqRel);
        }
        self.bytes = bytes;
        Ok(())
    }
}
impl Drop for Reservation {
    fn drop(&mut self) {
        self.local.used.fetch_sub(self.bytes, Ordering::AcqRel);
    }
}
pub(super) struct Task {
    pub status: Mutex<UploadStatus>,
    failure: Arc<UploadFailure>,
    _memory: Reservation,
}
struct Payload {
    data: Vec<u8>,
    _memory: Reservation,
}
impl Payload {
    fn new(local: &Arc<LocalBudget>, bytes: usize) -> Result<Self> {
        let mut memory = Reservation::new(local, bytes)?;
        let mut data = Vec::new();
        data.try_reserve_exact(bytes)
            .map_err(|_| Error::OutOfHostMemory)?;
        memory.resize(data.capacity())?;
        data.resize(bytes, 0);
        Ok(Self {
            data,
            _memory: memory,
        })
    }
    /// A payload holding a copy of `bytes`, written once: no zero-fill pass
    /// before the copy that overwrites it.
    fn from_slice(local: &Arc<LocalBudget>, bytes: &[u8]) -> Result<Self> {
        let mut memory = Reservation::new(local, bytes.len())?;
        let mut data = Vec::new();
        data.try_reserve_exact(bytes.len())
            .map_err(|_| Error::OutOfHostMemory)?;
        memory.resize(data.capacity())?;
        data.extend_from_slice(bytes);
        Ok(Self {
            data,
            _memory: memory,
        })
    }
}
pub(super) struct Segment {
    pub offset: u64,
    payload: Payload,
}
impl Segment {
    pub fn bytes(&self) -> &[u8] {
        &self.payload.data
    }
    fn end(&self) -> u64 {
        self.offset + self.payload.data.len() as u64
    }
}
pub(super) struct Job<T> {
    pub target: Arc<T>,
    pub options: UploadOptions,
    pub task: Arc<Task>,
    pub segments: Vec<Segment>,
    sequence: u64,
    mode: UploadMode,
    // Segment backing drops before its capacity credit.
    _memory: Reservation,
}
impl<T> Job<T> {
    pub fn bytes(&self) -> usize {
        self.segments.iter().map(|s| s.bytes().len()).sum()
    }
}
struct Epoch<T> {
    target: Weak<T>,
    key: UploadKey,
    epoch: u64,
}
pub(super) struct UploadQueue<T> {
    jobs: Vec<Job<T>>,
    epochs: Vec<Epoch<T>>,
    tasks: Vec<Arc<Task>>,
    config: UploadQueueConfig,
    chunk_bytes: usize,
    sequence: u64,
    minimum_epoch: u64,
    counters: UploadQueueStats,
    #[cfg(any(test, feature = "cpu-probe"))]
    manager_probe_id: u64,
    #[cfg(any(test, feature = "cpu-probe"))]
    native_submitted_jobs: u64,
    #[cfg(any(test, feature = "cpu-probe"))]
    submitted_bytes: u64,
    #[cfg(any(test, feature = "cpu-probe"))]
    submission_counters_overflowed: bool,
    _memory: Reservation,
    local: Arc<LocalBudget>,
}
fn reserve<T>(capacity: usize) -> Result<Vec<T>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| Error::OutOfHostMemory)?;
    Ok(values)
}
impl<T> UploadQueue<T> {
    pub fn new(
        stats: Arc<AllocatorStats>,
        config: UploadQueueConfig,
        chunk_bytes: usize,
    ) -> Result<Self> {
        if config.max_jobs == 0
            || config.max_jobs > 256
            || config.max_keys == 0
            || config.max_keys > 1024
            || config.max_tickets < config.max_jobs
            || config.max_tickets > 1024
            || config.max_regions_per_job == 0
            || config.max_regions_per_job > 256
            || chunk_bytes == 0
        {
            return Err(Error::invalid_usage(
                "invalid bounded upload queue configuration",
            ));
        }
        let envelope = shared_cpu_bytes::<LocalBudget>()?;
        if envelope > config.max_cpu_bytes as u64 {
            return Err(Error::Timeout);
        }
        let memory = stats.reserve_cpu_bytes(envelope, BudgetCategory::Live)?;
        let local = Arc::new(LocalBudget {
            used: AtomicUsize::new(envelope as usize),
            limit: config.max_cpu_bytes,
            stats,
            _envelope: memory,
        });
        let estimate = config.max_jobs * size_of::<Job<T>>()
            + config.max_keys * size_of::<Epoch<T>>()
            + config.max_tickets * size_of::<Arc<Task>>();
        let mut memory = Reservation::new(&local, estimate)?;
        let jobs = reserve(config.max_jobs)?;
        let epochs = reserve(config.max_keys)?;
        let tasks = reserve(config.max_tickets)?;
        memory.resize(
            jobs.capacity() * size_of::<Job<T>>()
                + epochs.capacity() * size_of::<Epoch<T>>()
                + tasks.capacity() * size_of::<Arc<Task>>(),
        )?;
        Ok(Self {
            jobs,
            epochs,
            tasks,
            config,
            chunk_bytes,
            sequence: 0,
            minimum_epoch: 0,
            counters: UploadQueueStats::default(),
            #[cfg(any(test, feature = "cpu-probe"))]
            manager_probe_id: next_manager_probe_id(),
            #[cfg(any(test, feature = "cpu-probe"))]
            native_submitted_jobs: 0,
            #[cfg(any(test, feature = "cpu-probe"))]
            submitted_bytes: 0,
            #[cfg(any(test, feature = "cpu-probe"))]
            submission_counters_overflowed: false,
            _memory: memory,
            local,
        })
    }
    fn same_key(job: &Job<T>, target: &Arc<T>, key: UploadKey) -> bool {
        Arc::ptr_eq(&job.target, target) && job.options.key == key
    }
    fn fresh(&mut self, target: &Arc<T>, options: UploadOptions) -> Result<Option<usize>> {
        self.epochs.retain(|entry| entry.target.strong_count() != 0);
        let existing = self.epochs.iter().position(|entry| {
            entry.target.as_ptr() == Arc::as_ptr(target) && entry.key == options.key
        });
        if options.epoch < self.minimum_epoch
            || existing.is_some_and(|i| options.epoch < self.epochs[i].epoch)
        {
            return Err(Error::invalid_usage(
                "upload epoch is older than the accepted key epoch",
            ));
        }
        if existing.is_none() && self.epochs.len() == self.config.max_keys {
            return Err(Error::Timeout);
        }
        Ok(existing)
    }
    fn new_failure(local: &Arc<LocalBudget>) -> Result<Arc<UploadFailure>> {
        let memory = Reservation::new(local, shared_cpu_bytes::<UploadFailure>()? as usize)?;
        Ok(Arc::new(UploadFailure {
            error: OnceLock::new(),
            _memory: memory,
        }))
    }
    fn task(&mut self) -> Result<Arc<Task>> {
        if let Some(task) = self
            .tasks
            .iter_mut()
            .find(|task| Arc::strong_count(task) == 1 && Arc::weak_count(task) == 0)
        {
            let task_mut = Arc::get_mut(task).expect("exclusive task slot");
            // Dropping an old private status cannot affect any escaped failure.
            // Its own Arc retains the separate real envelope and budget.
            *task_mut.status.get_mut() = UploadStatus::Pending;
            if let Some(failure) = Arc::get_mut(&mut task_mut.failure) {
                failure.error = OnceLock::new();
            } else {
                task_mut.failure = Self::new_failure(&self.local)?;
            }
            return Ok(task.clone());
        }
        if self.tasks.len() == self.config.max_tickets {
            return Err(Error::Timeout);
        }
        let memory = Reservation::new(&self.local, shared_cpu_bytes::<Task>()? as usize)?;
        let failure = Self::new_failure(&self.local)?;
        let task = Arc::new(Task {
            status: Mutex::new(UploadStatus::Pending),
            failure,
            _memory: memory,
        });
        self.tasks.push(task.clone());
        Ok(task)
    }
    /// All changes are admitted before cancellation/new-epoch publication. A
    /// rejected enqueue preserves previously accepted payload and epoch.
    pub fn enqueue(
        &mut self,
        target: Arc<T>,
        offset: u64,
        bytes: &[u8],
        options: UploadOptions,
    ) -> Result<Arc<Task>> {
        if bytes.is_empty()
            || bytes.len() > self.chunk_bytes
            || offset.checked_add(bytes.len() as u64).is_none()
        {
            return Err(Error::invalid_usage(
                "queued upload interval exceeds one chunk",
            ));
        }
        let epoch_index = self.fresh(&target, options)?;
        if let Some(index) = self
            .jobs
            .iter()
            .rposition(|job| Arc::ptr_eq(&job.target, &target))
            .filter(|&index| {
                self.jobs[index].options.key == options.key
                    && self.jobs[index].options.epoch == options.epoch
                    && self.jobs[index].mode == UploadMode::Replacement
            })
        {
            let job = &mut self.jobs[index];
            let end = offset + bytes.len() as u64;
            let mut start = offset;
            let mut finish = end;
            // Segments are sorted/disjoint. A connecting request consumes all
            // touched intervals; gaps outside the request are never fabricated.
            for segment in &job.segments {
                if segment.offset <= finish && segment.end() >= start {
                    start = start.min(segment.offset);
                    finish = finish.max(segment.end());
                }
            }
            let replaced: usize = job
                .segments
                .iter()
                .filter(|s| s.offset <= finish && s.end() >= start)
                .map(|s| s.bytes().len())
                .sum();
            let union = usize::try_from(finish - start).map_err(|_| Error::OutOfHostMemory)?;
            let old_count = job
                .segments
                .iter()
                .filter(|s| s.offset <= finish && s.end() >= start)
                .count();
            if job.bytes() - replaced + union <= self.chunk_bytes
                && job.segments.len() - old_count < self.config.max_regions_per_job
            {
                let mut payload = Payload::new(&self.local, union)?;
                for segment in &job.segments {
                    if segment.offset <= finish && segment.end() >= start {
                        let at = (segment.offset - start) as usize;
                        payload.data[at..at + segment.bytes().len()]
                            .copy_from_slice(segment.bytes());
                    }
                }
                let at = (offset - start) as usize;
                payload.data[at..at + bytes.len()].copy_from_slice(bytes);
                job.segments
                    .retain(|s| !(s.offset <= finish && s.end() >= start));
                job.segments.push(Segment {
                    offset: start,
                    payload,
                });
                job.segments.sort_unstable_by_key(|s| s.offset);
                job.options.priority = job.options.priority.max(options.priority);
                self.counters.merged_requests += 1;
                return Ok(job.task.clone());
            }
        }
        // A new epoch may replace old jobs even at the job-count limit.
        let superseded = self
            .jobs
            .iter()
            .filter(|job| {
                Self::same_key(job, &target, options.key)
                    && job.options.epoch < options.epoch
                    && job.mode == UploadMode::Replacement
            })
            .count();
        if self.jobs.len() - superseded == self.config.max_jobs {
            return Err(Error::Timeout);
        }
        let sequence = self
            .sequence
            .checked_add(1)
            .ok_or_else(|| Error::invalid_usage("upload FIFO exhausted"))?;
        let payload = Payload::from_slice(&self.local, bytes)?;
        let mut memory = Reservation::new(
            &self.local,
            self.config.max_regions_per_job * size_of::<Segment>(),
        )?;
        let mut segments = reserve(self.config.max_regions_per_job)?;
        memory.resize(segments.capacity() * size_of::<Segment>())?;
        segments.push(Segment { offset, payload });
        let task = self.task()?;
        self.cancel_matching(
            |job| {
                Self::same_key(job, &target, options.key)
                    && job.options.epoch < options.epoch
                    && job.mode == UploadMode::Replacement
            },
            UploadCancellation::Superseded,
        );
        if let Some(index) = epoch_index {
            self.epochs[index].epoch = options.epoch;
        } else {
            self.epochs.push(Epoch {
                target: Arc::downgrade(&target),
                key: options.key,
                epoch: options.epoch,
            });
        }
        self.jobs.push(Job {
            target,
            options,
            task: task.clone(),
            segments,
            sequence,
            mode: UploadMode::Replacement,
            _memory: memory,
        });
        self.sequence = sequence;
        Ok(task)
    }

    /// Take ownership without copying or coalescing the payload. All fallible
    /// admission precedes publication; a rejected request cannot advance the
    /// key epoch, cancel an accepted job, or lose the caller's bytes.
    pub fn enqueue_owned(
        &mut self,
        target: Arc<T>,
        offset: u64,
        bytes: Vec<u8>,
        options: UploadOptions,
        mode: UploadMode,
    ) -> core::result::Result<Arc<Task>, OwnedUploadError> {
        let admitted = (|| {
            if bytes.is_empty()
                || bytes.len() > self.chunk_bytes
                || offset.checked_add(bytes.len() as u64).is_none()
            {
                return Err(Error::invalid_usage(
                    "queued upload interval exceeds one chunk",
                ));
            }
            let epoch_index = self.fresh(&target, options)?;
            let supersedes = |job: &Job<T>| {
                mode == UploadMode::Replacement
                    && job.mode == UploadMode::Replacement
                    && Self::same_key(job, &target, options.key)
                    && job.options.epoch < options.epoch
            };
            let superseded = self.jobs.iter().filter(|job| supersedes(job)).count();
            if self.jobs.len() - superseded >= self.config.max_jobs {
                return Err(Error::Timeout);
            }
            let sequence = self
                .sequence
                .checked_add(1)
                .ok_or_else(|| Error::invalid_usage("upload FIFO exhausted"))?;
            // Capacity, not length: retained spare storage consumes budget too.
            let payload_memory = Reservation::new(&self.local, bytes.capacity())?;
            let mut memory = Reservation::new(&self.local, size_of::<Segment>())?;
            let segments = reserve::<Segment>(1)?;
            memory.resize(segments.capacity() * size_of::<Segment>())?;
            let task = self.task()?;
            Ok((
                epoch_index,
                sequence,
                payload_memory,
                memory,
                segments,
                task,
            ))
        })();
        let (epoch_index, sequence, payload_memory, memory, mut segments, task) = match admitted {
            Ok(admitted) => admitted,
            Err(error) => {
                return Err(OwnedUploadError {
                    error,
                    payload: bytes,
                });
            }
        };
        segments.push(Segment {
            offset,
            payload: Payload {
                data: bytes,
                _memory: payload_memory,
            },
        });
        if mode == UploadMode::Replacement {
            self.cancel_matching(
                |job| {
                    job.mode == UploadMode::Replacement
                        && Self::same_key(job, &target, options.key)
                        && job.options.epoch < options.epoch
                },
                UploadCancellation::Superseded,
            );
        }
        if let Some(index) = epoch_index {
            self.epochs[index].epoch = options.epoch;
        } else {
            self.epochs.push(Epoch {
                target: Arc::downgrade(&target),
                key: options.key,
                epoch: options.epoch,
            });
        }
        self.jobs.push(Job {
            target,
            options,
            task: task.clone(),
            segments,
            sequence,
            mode,
            _memory: memory,
        });
        self.sequence = sequence;
        Ok(task)
    }
    fn cancel_matching(
        &mut self,
        matches: impl Fn(&Job<T>) -> bool,
        reason: UploadCancellation,
    ) -> usize {
        let mut cancelled = 0;
        self.jobs.retain(|job| {
            if !matches(job) {
                return true;
            }
            *job.task.status.lock() = UploadStatus::Cancelled(reason);
            cancelled += 1;
            false
        });
        self.counters.cancelled_jobs += cancelled as u64;
        cancelled
    }
    pub fn cancel_before(&mut self, epoch: u64) -> usize {
        self.minimum_epoch = self.minimum_epoch.max(epoch);
        self.cancel_matching(
            |job| job.options.epoch < epoch,
            UploadCancellation::Explicit,
        )
    }
    pub fn cancel_key_before(&mut self, target: &Arc<T>, key: UploadKey, epoch: u64) -> usize {
        if let Some(entry) = self
            .epochs
            .iter_mut()
            .find(|entry| entry.target.as_ptr() == Arc::as_ptr(target) && entry.key == key)
        {
            entry.epoch = entry.epoch.max(epoch);
        }
        self.cancel_matching(
            |job| Self::same_key(job, target, key) && job.options.epoch < epoch,
            UploadCancellation::Explicit,
        )
    }
    /// Highest eligible priority, FIFO for ties. A later update of the same
    /// physical target never overtakes an earlier accepted update, even under
    /// another logical key (overlapping destinations must keep write order).
    pub fn next(&mut self, pressure: BudgetAction) -> Option<usize> {
        let best = self
            .jobs
            .iter()
            .enumerate()
            .filter(|(i, job)| {
                (pressure == BudgetAction::None || job.options.priority == UploadPriority::High)
                    && !self.jobs[..*i]
                        .iter()
                        .any(|earlier| Arc::ptr_eq(&earlier.target, &job.target))
            })
            .max_by_key(|(_, job)| (job.options.priority, std::cmp::Reverse(job.sequence)))
            .map(|(i, _)| i);
        if best.is_none() && !self.jobs.is_empty() {
            self.counters.deferred_pumps += 1;
        }
        best
    }
    /// Pack heads of distinct physical targets, at the same eligible priority.
    /// A later write to one target stays queued until its earlier submission.
    pub fn next_batch(
        &mut self,
        pressure: BudgetAction,
        byte_limit: usize,
        region_limit: usize,
    ) -> smallvec::SmallVec<[usize; super::MAX_UPLOAD_TARGETS]> {
        let mut selected = smallvec::SmallVec::new();
        let Some(first) = self.next(pressure) else {
            return selected;
        };
        let head = &self.jobs[first];
        let priority = head.options.priority;
        let mut bytes = head.bytes();
        let mut regions = head.segments.len();
        selected.push(first);
        for (index, job) in self.jobs.iter().enumerate().skip(first + 1) {
            if selected.len() == super::MAX_UPLOAD_TARGETS {
                break;
            }
            if job.options.priority != priority
                || self.jobs[..index]
                    .iter()
                    .any(|earlier| Arc::ptr_eq(&earlier.target, &job.target))
            {
                continue;
            }
            let next_bytes = job.bytes();
            if next_bytes > byte_limit.saturating_sub(bytes)
                || job.segments.len() > region_limit.saturating_sub(regions)
            {
                continue;
            }
            bytes += next_bytes;
            regions += job.segments.len();
            selected.push(index);
        }
        selected
    }
    pub fn cancel_task(&mut self, task: &Arc<Task>) -> bool {
        self.cancel_matching(
            |job| Arc::ptr_eq(&job.task, task),
            UploadCancellation::Explicit,
        ) != 0
    }
    /// Only CPU-exclusive envelopes can be evicted. Escaped tickets and failure
    /// diagnostics keep their own reservations; pending jobs are never evicted.
    pub fn trim_idle(&mut self) -> usize {
        let before = self.tasks.len();
        self.tasks
            .retain(|task| Arc::strong_count(task) != 1 || Arc::weak_count(task) != 0);
        self.epochs.retain(|entry| entry.target.strong_count() != 0);
        let removed = before - self.tasks.len();
        self.counters.evicted_ticket_slots += removed as u64;
        removed
    }
    pub fn job(&self, index: usize) -> &Job<T> {
        &self.jobs[index]
    }
    pub fn publish(&mut self, index: usize, point: SyncPoint) {
        *self.jobs[index].task.status.lock() = UploadStatus::Submitted(point);
        self.jobs.remove(index);
        self.counters.submitted_jobs += 1;
    }
    /// Record exactly one native-submit proof, before any fallible acquire
    /// registration or ticket publication. `None` means no native submission.
    /// Repeating a real GPU copy on retry counts again, even for the same ticket.
    #[cfg(any(test, feature = "cpu-probe"))]
    pub(super) fn record_native_submission(&mut self, jobs: usize, payload: Option<u64>) {
        let Some(payload) = payload else { return; };
        if let Some(jobs) = u64::try_from(jobs).ok()
            .and_then(|jobs| self.native_submitted_jobs.checked_add(jobs))
        {
            self.native_submitted_jobs = jobs;
        } else {
            self.submission_counters_overflowed = true;
        }
        if !self.submission_counters_overflowed {
            if let Some(bytes) = self.submitted_bytes.checked_add(payload) {
                self.submitted_bytes = bytes;
            } else {
                // Diagnostics cannot roll back a real native submission.
                self.submission_counters_overflowed = true;
            }
        }
    }
    /// O(1) cumulative native-submit counters. No pending-job traversal.
    #[cfg(any(test, feature = "cpu-probe"))]
    fn submitted_snapshot(&self) -> Result<(u64, u64)> {
        if self.manager_probe_id == 0 {
            Err(Error::invalid_usage("upload manager probe identity exhausted"))
        } else if self.submission_counters_overflowed {
            Err(Error::invalid_usage("upload submission counters overflowed"))
        } else {
            Ok((self.native_submitted_jobs, self.submitted_bytes))
        }
    }
    fn failure(task: &Arc<Task>, error: Error) -> Arc<UploadFailure> {
        task.failure
            .error
            .set(error)
            .expect("pending task has a private uninitialized failure envelope");
        task.failure.clone()
    }
    pub fn fail_all(&mut self, error: Error) -> Arc<UploadFailure> {
        let failure = Self::failure(&self.jobs[0].task, error);
        for job in &self.jobs {
            *job.task.status.lock() = UploadStatus::Failed(failure.clone());
        }
        self.jobs.clear();
        failure
    }
    /// The selected command was never submitted. Every ticket of that command
    /// observes the same retained failure; unrelated queued jobs stay pending.
    pub fn fail_batch(&mut self, indices: &[usize], error: Error) -> Arc<UploadFailure> {
        let failure = Self::failure(&self.jobs[indices[0]].task, error);
        for &index in indices.iter().rev() {
            *self.jobs[index].task.status.lock() = UploadStatus::Failed(failure.clone());
            self.jobs.remove(index);
        }
        failure
    }
    pub fn defer(&mut self) {
        self.counters.deferred_pumps += 1;
    }
    /// Number of queued jobs. `stats()` also sums every segment's bytes, which
    /// callers that only ask "is anything queued?" do not need.
    pub fn queued_jobs(&self) -> usize {
        self.jobs.len()
    }
    pub(super) fn fixture_weak_history_count(&self, target: &Arc<T>) -> usize {
        self.epochs.iter().filter(|entry| entry.target.as_ptr() == Arc::as_ptr(target)).count()
    }

    pub fn stats(&self) -> UploadQueueStats {
        UploadQueueStats {
            queued_jobs: self.jobs.len(),
            queued_bytes: self.jobs.iter().map(Job::bytes).sum(),
            known_cpu_bytes: self.local.used.load(Ordering::Acquire),
            task_slots: self.tasks.len(),
            active_keys: self.epochs.len(),
            ..self.counters
        }
    }
}
impl<T> Drop for UploadQueue<T> {
    fn drop(&mut self) {
        for job in &self.jobs {
            *job.task.status.lock() = UploadStatus::Cancelled(UploadCancellation::ManagerDropped);
        }
    }
}

#[cfg(feature = "cpu-probe")]
impl super::TransferManager {
    /// `(process_local_manager_id, submitted_jobs, submitted_payload_bytes)`.
    /// Actually submitted queued CPU-buffer H2D job executions and effective
    /// payload bytes, cumulative for this manager's lifetime. A later acquire
    /// registration or publication error preserves the native prefix. A real
    /// repeated copy on retry counts again; this is not a unique-ticket or
    /// command count. Several jobs may share one point. Direct/texture/setup/D2D
    /// paths are outside this scope. Admission, deferral and cancellation do
    /// not count. A later flush error does not erase its successful prefix.
    ///
    /// O(1), with no pending queue traversal or extra lock. Sticky overflow is
    /// an explicit invalid sample. Callers must establish a fresh baseline
    /// across manager/App replacement rather than wrapping a negative delta.
    /// The nonzero manager token is unique only within this process, never a
    /// cross-run or cross-binary signature. Exhausted tokens invalidate samples.
    pub fn upload_submitted_snapshot(&self) -> crate::Result<(u64, u64, u64)> {
        let (jobs, bytes) = self.queued.submitted_snapshot()?;
        Ok((self.queued.manager_probe_id, jobs, bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::QueueId;
    fn options(key: u64, epoch: u64, priority: UploadPriority) -> UploadOptions {
        UploadOptions {
            key: UploadKey(key),
            epoch,
            priority,
        }
    }
    fn queue(chunk: usize) -> UploadQueue<()> {
        UploadQueue::new(
            Arc::new(AllocatorStats::default()),
            UploadQueueConfig::default(),
            chunk,
        )
        .unwrap()
    }
    #[test]
    fn batches_respect_payload_region_target_bounds_and_physical_fifo() {
        let mut queue = UploadQueue::new(
            Arc::new(AllocatorStats::default()),
            UploadQueueConfig {
                max_jobs: 128,
                ..Default::default()
            },
            1024,
        )
        .unwrap();
        let first = Arc::new(());
        for epoch in 1..=2 {
            queue
                .enqueue_owned(
                    first.clone(),
                    0,
                    vec![epoch as u8; 4],
                    options(epoch, epoch, UploadPriority::High),
                    UploadMode::OrderedDelta,
                )
                .unwrap();
        }
        for key in 3..103 {
            queue
                .enqueue_owned(
                    Arc::new(()),
                    0,
                    vec![1; 4],
                    options(key, 1, UploadPriority::High),
                    UploadMode::OrderedDelta,
                )
                .unwrap();
        }
        let bytes = queue.next_batch(BudgetAction::None, 64, 256);
        assert_eq!(bytes.len(), 16);
        assert_eq!(&bytes[..3], &[0, 2, 3]);
        let regions = queue.next_batch(BudgetAction::None, 1024, 3);
        assert_eq!(&regions[..], &[0, 2, 3]);
        let targets = queue.next_batch(BudgetAction::None, 1024, 256);
        assert_eq!(targets.len(), super::super::MAX_UPLOAD_TARGETS);
        assert!(
            !targets.contains(&1),
            "the second write to the same owner stays pending"
        );
    }
    #[test]
    fn unsubmitted_batch_failure_keeps_unrelated_tickets_pending() {
        let mut queue = queue(64);
        let tickets = (0..3)
            .map(|key| {
                queue
                    .enqueue_owned(
                        Arc::new(()),
                        0,
                        vec![1; 4],
                        options(key, 1, UploadPriority::Normal),
                        UploadMode::OrderedDelta,
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let failed = queue.fail_batch(&[0, 2], Error::OutOfDeviceMemory);
        for index in [0, 2] {
            let status = tickets[index].status.lock();
            assert!(matches!(&*status, UploadStatus::Failed(error) if Arc::ptr_eq(error, &failed)));
        }
        assert!(matches!(*tickets[1].status.lock(), UploadStatus::Pending));
        assert_eq!(queue.queued_jobs(), 1);
        assert!(Arc::ptr_eq(&queue.jobs[0].task, &tickets[1]));
    }
    // Synthetic values test CPU accounting/publication only; production records
    // the proof at command.submit_to success, before fallible acquire registration.
    fn point(value: u64) -> SyncPoint {
        SyncPoint {
            queue: QueueId::new(0).unwrap(),
            value,
        }
    }
    fn record_and_publish(queue: &mut UploadQueue<()>, index: usize, point: SyncPoint) {
        let bytes = queue.job(index).bytes() as u64;
        queue.record_native_submission(1, Some(bytes));
        queue.publish(index, point);
    }
    #[test]
    fn submitted_snapshot_native_proof_survives_post_submit_acquire_error_without_publication() {
        let mut queue = queue(64);
        let ticket = queue.enqueue_owned(Arc::new(()), 0, vec![1; 4],
            options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        // Failure before native submission produces no proof and no work.
        queue.record_native_submission(1, None);
        assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
        // Model native success followed by acquire registration failure: the
        // original protocol leaves this accepted job pending, but GPU work ran.
        let submitted_payload = Some(queue.job(0).bytes() as u64);
        let post_submit_result: Result<()> = Err(Error::OutOfHostMemory);
        queue.record_native_submission(1, submitted_payload);
        assert!(post_submit_result.is_err());
        assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
        assert_eq!(queue.counters.submitted_jobs, 0);
        assert!(matches!(*ticket.status.lock(), UploadStatus::Pending));
        // The retry submits the same payload again: two physical executions,
        // one published ticket. Publication itself must not count a third copy.
        queue.record_native_submission(1, submitted_payload);
        queue.publish(0, point(2));
        assert_eq!(queue.submitted_snapshot().unwrap(), (2, 8));
        assert_eq!(queue.counters.submitted_jobs, 1);
        assert!(matches!(*ticket.status.lock(), UploadStatus::Submitted(p) if p == point(2)));
    }
    #[test]
    fn submitted_snapshot_manager_identity_changes_on_rebuild_and_exhaustion_is_invalid() {
        let first_id = {
            let queue = queue(64);
            assert!(queue.manager_probe_id != 0);
            assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
            queue.manager_probe_id
        };
        let mut replacement = queue(64);
        assert!(replacement.manager_probe_id != 0);
        assert_ne!(replacement.manager_probe_id, first_id);
        // Simulate identity allocation exhaustion without mutating the global
        // token source shared by parallel tests or the production queue protocol.
        replacement.manager_probe_id = 0;
        assert!(matches!(replacement.submitted_snapshot(), Err(Error::InvalidUsage { .. })));
        let ticket = replacement.enqueue_owned(Arc::new(()), 0, vec![1; 4],
            options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        record_and_publish(&mut replacement, 0, point(1));
        assert!(matches!(*ticket.status.lock(), UploadStatus::Submitted(p) if p == point(1)));
        assert!(replacement.submitted_snapshot().is_err());
    }
    #[test]
    fn submitted_snapshot_keeps_successful_prefix_after_failure_cancel_and_refusal() {
        let mut queue = queue(64);
        let first = queue.enqueue_owned(Arc::new(()), 0, vec![1; 4],
            options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        let failed = queue.enqueue_owned(Arc::new(()), 0, vec![2; 8],
            options(2, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        let cancelled = queue.enqueue_owned(Arc::new(()), 0, vec![3; 12],
            options(3, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
        queue.defer();
        assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
        record_and_publish(&mut queue, 0, point(1));
        assert!(matches!(*first.status.lock(), UploadStatus::Submitted(p) if p == point(1)));
        assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
        queue.fail_batch(&[0], Error::OutOfDeviceMemory);
        assert!(matches!(*failed.status.lock(), UploadStatus::Failed(_)));
        assert!(queue.cancel_task(&cancelled));
        assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
        let mut oversized = Vec::with_capacity(queue.config.max_cpu_bytes);
        oversized.extend_from_slice(&[7; 4]);
        let pointer = oversized.as_ptr();
        let refused = queue.enqueue_owned(Arc::new(()), 0, oversized,
            options(4, 1, UploadPriority::Normal), UploadMode::OrderedDelta).err().expect("real CPU budget refusal");
        assert_eq!(refused.payload.as_ptr(), pointer);
        assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
        queue.enqueue_owned(Arc::new(()), 0, vec![8; 6],
            options(5, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
        record_and_publish(&mut queue, 0, point(2));
        assert_eq!(queue.submitted_snapshot().unwrap(), (2, 10));
    }
    #[test]
    fn submitted_snapshot_counts_effective_regions_not_gaps_or_merged_request_bytes() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let opts = options(1, 1, UploadPriority::Normal);
        queue.enqueue(target.clone(), 0, &[1; 4], opts).unwrap();
        queue.enqueue(target.clone(), 8, &[2; 4], opts).unwrap();
        assert_eq!(queue.job(0).bytes(), 8); // Four-byte gap is not a payload.
        record_and_publish(&mut queue, 0, point(1));
        assert_eq!(queue.submitted_snapshot().unwrap(), (1, 8));
        queue.enqueue(target.clone(), 0, &[1; 4], opts).unwrap();
        queue.enqueue(target.clone(), 8, &[2; 4], opts).unwrap();
        queue.enqueue(target, 3, &[3; 6], opts).unwrap();
        assert_eq!(queue.job(0).bytes(), 12); // Request lengths sum to14, overlap is copied once.
        record_and_publish(&mut queue, 0, point(2));
        assert_eq!(queue.submitted_snapshot().unwrap(), (2, 20));
    }
    #[test]
    fn submitted_snapshot_counts_jobs_sharing_a_success_point_independently() {
        let mut queue = queue(64);
        for key in 0..2 {
            queue.enqueue_owned(Arc::new(()), 0, vec![key as u8; 4],
                options(key, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        }
        record_and_publish(&mut queue, 1, point(7));
        record_and_publish(&mut queue, 0, point(7));
        assert_eq!(queue.submitted_snapshot().unwrap(), (2, 8));
    }
    #[test]
    fn submitted_snapshot_overflow_is_sticky_without_reverting_real_publication() {
        for (jobs, bytes) in [(u64::MAX, 0), (0, u64::MAX - 3)] {
            let mut queue = queue(64);
            queue.native_submitted_jobs = jobs;
            queue.submitted_bytes = bytes;
            let ticket = queue.enqueue_owned(Arc::new(()), 0, vec![1; 4],
                options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
            record_and_publish(&mut queue, 0, point(1));
            assert!(matches!(*ticket.status.lock(), UploadStatus::Submitted(p) if p == point(1)));
            assert_eq!(queue.queued_jobs(), 0);
            assert!(matches!(queue.submitted_snapshot(), Err(Error::InvalidUsage { .. })));
            assert_eq!((queue.native_submitted_jobs, queue.submitted_bytes), (jobs.saturating_add(1), bytes));
            queue.enqueue_owned(Arc::new(()), 0, vec![2; 1],
                options(2, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
            record_and_publish(&mut queue, 0, point(2));
            assert!(queue.submitted_snapshot().is_err());
            assert_eq!((queue.native_submitted_jobs, queue.submitted_bytes), (jobs.saturating_add(2), bytes));
        }
        let mut queue = queue(64);
        queue.native_submitted_jobs = u64::MAX - 1;
        queue.submitted_bytes = u64::MAX - 4;
        queue.enqueue_owned(Arc::new(()), 0, vec![3; 4],
            options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        record_and_publish(&mut queue, 0, point(1));
        assert_eq!(queue.submitted_snapshot().unwrap(), (u64::MAX, u64::MAX));
    }
    #[test]
    fn owned_deltas_preserve_all_epochs_and_the_original_allocation() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let bytes = vec![1, 2, 3, 4];
        let pointer = bytes.as_ptr();
        let first = queue
            .enqueue_owned(
                target.clone(),
                0,
                bytes,
                options(1, 1, UploadPriority::Normal),
                UploadMode::OrderedDelta,
            )
            .unwrap();
        queue
            .enqueue_owned(
                target.clone(),
                8,
                vec![5; 4],
                options(1, 2, UploadPriority::High),
                UploadMode::OrderedDelta,
            )
            .unwrap();
        // Even the legacy replacing API must not implicitly cancel a delta.
        queue
            .enqueue(target, 16, &[9; 4], options(1, 3, UploadPriority::High))
            .unwrap();
        assert_eq!(queue.queued_jobs(), 3);
        assert_eq!(queue.job(0).segments[0].bytes().as_ptr(), pointer);
        assert!(matches!(*first.status.lock(), UploadStatus::Pending));
        assert_eq!(queue.next(BudgetAction::None), Some(0));
        queue.publish(0, point(1));
        assert_eq!(queue.job(0).segments[0].offset, 8);
        queue.publish(0, point(2));
        assert_eq!(queue.job(0).segments[0].offset, 16);
    }

    #[test]
    fn owned_admission_failure_returns_storage_and_does_not_advance_epoch() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let mut bytes = Vec::with_capacity(queue.config.max_cpu_bytes);
        bytes.extend_from_slice(&[7; 4]);
        let pointer = bytes.as_ptr();
        let before = queue.stats().known_cpu_bytes;
        let error = queue
            .enqueue_owned(
                target.clone(),
                0,
                bytes,
                options(1, 9, UploadPriority::Normal),
                UploadMode::OrderedDelta,
            )
            .err()
            .expect("budget rejection");
        assert_eq!(error.payload.as_ptr(), pointer);
        assert_eq!(error.payload, [7; 4]);
        assert_eq!(queue.queued_jobs(), 0);
        assert_eq!(queue.stats().known_cpu_bytes, before);
        queue
            .enqueue_owned(
                target,
                0,
                vec![1; 4],
                options(1, 1, UploadPriority::Normal),
                UploadMode::OrderedDelta,
            )
            .unwrap();
    }

    #[test]
    fn owned_deltas_in_the_same_epoch_keep_separate_completion_tickets() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let opts = options(1, 1, UploadPriority::Normal);
        let first = queue
            .enqueue_owned(
                target.clone(),
                0,
                vec![1; 4],
                opts,
                UploadMode::OrderedDelta,
            )
            .unwrap();
        let second = queue
            .enqueue_owned(target, 0, vec![2; 4], opts, UploadMode::OrderedDelta)
            .unwrap();
        assert!(!Arc::ptr_eq(&first, &second));
        queue.publish(0, point(1));
        assert!(matches!(*first.status.lock(), UploadStatus::Submitted(_)));
        assert!(matches!(*second.status.lock(), UploadStatus::Pending));
    }
    #[test]
    fn dirty_regions_coalesce_latest_overlap_and_never_copy_a_gap() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let opts = options(1, 1, UploadPriority::Normal);
        let a = queue.enqueue(target.clone(), 0, &[1; 4], opts).unwrap();
        let b = queue.enqueue(target.clone(), 8, &[2; 4], opts).unwrap();
        assert!(Arc::ptr_eq(&a, &b));
        assert_eq!(queue.job(0).segments.len(), 2);
        assert_eq!(queue.job(0).bytes(), 8, "the untouched gap stays absent");
        queue.enqueue(target, 3, &[3; 6], opts).unwrap();
        let job = queue.job(0);
        assert_eq!(job.segments.len(), 1);
        assert_eq!(
            job.segments[0].bytes(),
            &[1, 1, 1, 3, 3, 3, 3, 3, 3, 2, 2, 2]
        );
        queue.publish(0, point(7));
        assert!(matches!(*a.status.lock(), UploadStatus::Submitted(p) if p == point(7)));
        assert!(matches!(*b.status.lock(), UploadStatus::Submitted(p) if p == point(7)));
        assert_eq!(queue.stats().merged_requests, 2);
    }
    #[test]
    fn priority_is_fifo_and_never_reorders_the_same_physical_destination() {
        let mut queue = queue(4);
        let a = Arc::new(());
        let b = Arc::new(());
        let c = Arc::new(());
        queue
            .enqueue(
                a.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Background),
            )
            .unwrap();
        queue
            .enqueue(a, 0, &[2; 4], options(2, 1, UploadPriority::High))
            .unwrap();
        queue
            .enqueue(b, 0, &[3; 4], options(3, 1, UploadPriority::High))
            .unwrap();
        queue
            .enqueue(c, 0, &[4; 4], options(4, 1, UploadPriority::High))
            .unwrap();
        assert_eq!(queue.next(BudgetAction::Backpressure), Some(2));
        queue.publish(2, point(1));
        assert_eq!(queue.next(BudgetAction::EvictCache), Some(2));
        queue.publish(2, point(2));
        assert_eq!(queue.next(BudgetAction::Backpressure), None);
        assert_eq!(queue.next(BudgetAction::None), Some(0));
        queue.publish(0, point(3));
        assert_eq!(queue.next(BudgetAction::Backpressure), Some(0));
    }
    #[test]
    fn intervening_logical_key_prevents_coalescing_an_earlier_write() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let old = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Normal),
            )
            .unwrap();
        queue
            .enqueue(
                target.clone(),
                0,
                &[2; 4],
                options(2, 1, UploadPriority::Normal),
            )
            .unwrap();
        let newest = queue
            .enqueue(target, 0, &[3; 4], options(1, 1, UploadPriority::High))
            .unwrap();
        assert!(!Arc::ptr_eq(&old, &newest));
        assert_eq!(queue.stats().queued_jobs, 3);
        assert_eq!(queue.next(BudgetAction::None), Some(0));
    }
    #[test]
    fn newer_epochs_cancel_pending_but_not_submitted_and_reject_late_old_requests() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let old = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Normal),
            )
            .unwrap();
        let current = queue
            .enqueue(
                target.clone(),
                0,
                &[2; 4],
                options(1, 2, UploadPriority::Normal),
            )
            .unwrap();
        assert!(matches!(
            *old.status.lock(),
            UploadStatus::Cancelled(UploadCancellation::Superseded)
        ));
        assert!(matches!(
            queue.enqueue(
                target.clone(),
                0,
                &[3; 4],
                options(1, 1, UploadPriority::Normal)
            ),
            Err(Error::InvalidUsage { .. })
        ));
        queue.publish(0, point(9));
        queue.cancel_key_before(&target, UploadKey(1), 3);
        assert!(matches!(*current.status.lock(), UploadStatus::Submitted(p) if p == point(9)));
        assert!(
            queue
                .enqueue(target, 0, &[4; 4], options(1, 2, UploadPriority::Normal))
                .is_err()
        );
    }
    #[test]
    fn failed_admission_preserves_accepted_bytes_epoch_and_budget() {
        let stats = Arc::new(AllocatorStats::default());
        let mut queue = UploadQueue::new(stats.clone(), UploadQueueConfig::default(), 64).unwrap();
        let target = Arc::new(());
        let old = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Normal),
            )
            .unwrap();
        let used = stats.budget_usage().total();
        stats.set_total_budget(used).unwrap();
        assert!(matches!(
            queue.enqueue(
                target.clone(),
                0,
                &[2; 8],
                options(1, 2, UploadPriority::Normal)
            ),
            Err(Error::OutOfHostMemory)
        ));
        assert_eq!(stats.budget_usage().total(), used);
        assert!(matches!(*old.status.lock(), UploadStatus::Pending));
        assert_eq!(queue.job(0).segments[0].bytes(), &[1; 4]);
        stats.set_total_budget(0).unwrap();
        assert!(
            queue
                .enqueue(target, 0, &[3; 4], options(1, 1, UploadPriority::Normal))
                .is_ok()
        );
    }
    #[test]
    fn escaped_ticket_envelopes_backpressure_reuse_and_keep_real_cpu_credit() {
        let stats = Arc::new(AllocatorStats::default());
        let config = UploadQueueConfig {
            max_jobs: 1,
            max_tickets: 1,
            ..Default::default()
        };
        let mut queue = UploadQueue::new(stats.clone(), config, 64).unwrap();
        let target = Arc::new(());
        let held = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::High),
            )
            .unwrap();
        queue.publish(0, point(1));
        assert!(matches!(
            queue.enqueue(
                target.clone(),
                0,
                &[2; 4],
                options(1, 2, UploadPriority::High)
            ),
            Err(Error::Timeout)
        ));
        drop(held);
        let pending = queue
            .enqueue(target, 0, &[2; 4], options(1, 2, UploadPriority::High))
            .unwrap();
        assert_eq!(queue.stats().task_slots, 1);
        drop(queue);
        assert!(matches!(
            *pending.status.lock(),
            UploadStatus::Cancelled(UploadCancellation::ManagerDropped)
        ));
        assert!(stats.cpu_memory_usage().total() > 0);
        drop(pending);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn live_key_directory_is_bounded_and_dead_buffer_identity_can_be_reclaimed() {
        let config = UploadQueueConfig {
            max_keys: 1,
            ..Default::default()
        };
        let mut queue = UploadQueue::new(Arc::new(AllocatorStats::default()), config, 64).unwrap();
        let target = Arc::new(());
        queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Normal),
            )
            .unwrap();
        queue.publish(0, point(1));
        assert!(matches!(
            queue.enqueue(
                Arc::new(()),
                0,
                &[2; 4],
                options(2, 1, UploadPriority::Normal)
            ),
            Err(Error::Timeout)
        ));
        drop(target);
        for epoch in 1..1001 {
            let target = Arc::new(());
            queue
                .enqueue(
                    target,
                    0,
                    &[3; 4],
                    options(2, epoch, UploadPriority::Normal),
                )
                .unwrap();
            queue.publish(0, point(epoch + 1));
            assert_eq!(queue.stats().active_keys, 1);
        }
    }
    #[test]
    fn failure_has_no_completion_point_and_explicit_expiry_cannot_be_undone() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let failed = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Normal),
            )
            .unwrap();
        queue.fail_batch(&[0], Error::OutOfDeviceMemory);
        assert!(
            matches!(&*failed.status.lock(), UploadStatus::Failed(error) if matches!(error.error(), Error::OutOfDeviceMemory))
        );
        let cancelled = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 2, UploadPriority::Normal),
            )
            .unwrap();
        assert_eq!(queue.cancel_before(3), 1);
        assert!(matches!(
            *cancelled.status.lock(),
            UploadStatus::Cancelled(UploadCancellation::Explicit)
        ));
        assert!(
            queue
                .enqueue(target, 0, &[1; 4], options(1, 2, UploadPriority::Normal))
                .is_err()
        );
        assert_eq!(queue.stats().submitted_jobs, 0);
    }
    #[test]
    fn escaped_failure_diagnostic_keeps_its_own_budget_after_task_reuse_and_manager_drop() {
        let stats = Arc::new(AllocatorStats::default());
        let mut queue = UploadQueue::new(stats.clone(), UploadQueueConfig::default(), 64).unwrap();
        let target = Arc::new(());
        let ticket = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Normal),
            )
            .unwrap();
        let failure = queue.fail_batch(
            &[0],
            Error::invalid_usage("injected structured upload failure"),
        );
        let original = failure.to_error();
        drop(ticket);
        queue
            .enqueue(target, 0, &[2; 4], options(1, 2, UploadPriority::Normal))
            .unwrap();
        drop(queue);
        drop(failure);
        assert!(
            stats.cpu_memory_usage().total() > 0,
            "the escaped structured source still owns its failure envelope"
        );
        assert!(std::error::Error::source(&original).is_some());
        drop(original);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn pressure_trim_only_releases_cpu_exclusive_envelopes_and_preserves_live_epoch_history() {
        let mut queue = queue(64);
        let target = Arc::new(());
        let held = queue
            .enqueue(
                target.clone(),
                0,
                &[1; 4],
                options(1, 1, UploadPriority::Normal),
            )
            .unwrap();
        queue.publish(0, point(1));
        assert_eq!(queue.trim_idle(), 0);
        drop(held);
        let before = queue.stats().known_cpu_bytes;
        assert_eq!(queue.trim_idle(), 1);
        assert!(queue.stats().known_cpu_bytes < before);
        assert_eq!(queue.stats().active_keys, 1);
        assert!(
            queue
                .enqueue(target, 0, &[1; 4], options(1, 0, UploadPriority::Normal))
                .is_err()
        );
    }
}

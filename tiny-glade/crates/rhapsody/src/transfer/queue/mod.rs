//! CPU upload scheduling. Payloads and task envelopes have independent, bounded
//! host ownership; publishing a GPU point never needs a new allocation.
use crate::{Error, Result, sync::SyncPoint};
use crate::accounting::{AllocatorStats, BudgetAction, BudgetCategory, CpuMemoryReservation, shared_cpu_bytes};
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
mod tests;

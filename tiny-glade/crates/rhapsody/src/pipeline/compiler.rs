//! The asynchronous pipeline compiler and its worker.
use crate::Error;
use crate::Result;
use parking_lot::Condvar;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::ffi::CString;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use super::request::Cached;
use super::request::CompilePriority;
use super::request::CompilerConfig;
use super::request::CompilerStats;
use super::request::Completion;
use super::request::Job;
use super::request::PipelinePoll;
use super::request::PipelineTicket;
use super::request::Queued;
use super::request::RequestOptions;
use super::request::Shared;
use super::request::State;
use super::shader::ComputePipelineDesc;
use super::shader::PipelineCache;
use super::shader::PipelineLayout;
use super::shader::RasterPipelineDesc;

pub struct AsyncPipelineCompiler {
    pub(super) shared: Arc<Shared>,
    pub(super) workers: Vec<JoinHandle<()>>,
}

impl AsyncPipelineCompiler {
    pub fn new(cache: Arc<PipelineCache>, config: CompilerConfig) -> Result<Self> {
        config.validate()?;
        let shared = Arc::new(Shared {
            cache,
            config,
            available: Condvar::new(),
            state: Mutex::new(State {
                priority_burst: 0,
                cancelled: 0,
                queue: VecDeque::new(),
                cached: BTreeMap::new(),
                lru: VecDeque::new(),
                pending: 0,
                request_bytes: 0,
                cache_bytes: 0,
                stopping: false,
                peak_pending: 0,
                peak_request_bytes: 0,
                compiled: 0,
                failed: 0,
                merged: 0,
            }),
        });
        let evictable: Arc<dyn crate::accounting::Evictable> = shared.clone();
        shared.cache.device.register_evictable(
            Arc::downgrade(&evictable),
            crate::accounting::EvictionPriority::Expensive,
        );
        let mut compiler = Self {
            shared,
            workers: Vec::new(),
        };
        for index in 0..config.workers {
            let shared = compiler.shared.clone();
            let worker = thread::Builder::new()
                .name(format!("rhapsody-pipeline-{index}"))
                .spawn(move || worker(shared))
                .map_err(|error| Error::invalid_usage_with("start pipeline worker", error))?;
            compiler.workers.push(worker);
        }
        Ok(compiler)
    }
    pub fn request_compute(&self, desc: ComputePipelineDesc) -> Result<PipelineTicket> {
        self.request_compute_with_options(desc, RequestOptions::default())
    }
    pub fn request_compute_with_options(
        &self,
        desc: ComputePipelineDesc,
        options: RequestOptions,
    ) -> Result<PipelineTicket> {
        self.request(Job::Compute(desc), options)
    }
    pub fn request_raster(&self, desc: RasterPipelineDesc) -> Result<PipelineTicket> {
        self.request_raster_with_options(desc, RequestOptions::default())
    }
    pub fn request_raster_with_options(
        &self,
        desc: RasterPipelineDesc,
        options: RequestOptions,
    ) -> Result<PipelineTicket> {
        self.request(Job::Raster(Box::new(desc)), options)
    }
    /// Shader module creation and pipeline creation both run on the worker.
    pub fn request_compute_spirv(
        &self,
        bytes: Vec<u8>,
        entry: CString,
        layout: Arc<PipelineLayout>,
    ) -> Result<PipelineTicket> {
        self.request_compute_spirv_with_options(bytes, entry, layout, RequestOptions::default())
    }
    pub fn request_compute_spirv_with_options(
        &self,
        bytes: Vec<u8>,
        entry: CString,
        layout: Arc<PipelineLayout>,
        options: RequestOptions,
    ) -> Result<PipelineTicket> {
        self.request(
            Job::ComputeSpirv {
                bytes: Arc::new(bytes),
                entry,
                layout,
            },
            options,
        )
    }
    /// Own the request and return a stable ticket without caller-side retry or
    /// sleep. Count/byte refusal is explicit allocation pressure, never an
    /// invitation to redeclare a frame. Ready pipelines keep the bounded LRU.

    pub fn defer_raster(&self, desc: RasterPipelineDesc) -> Result<PipelineTicket> {
        self.request_inner(Job::Raster(Box::new(desc)), RequestOptions::default(), true)
    }
    pub fn defer_compute_spirv(
        &self,
        bytes: Vec<u8>,
        entry: CString,
        layout: Arc<PipelineLayout>,
    ) -> Result<PipelineTicket> {
        self.request_inner(
            Job::ComputeSpirv {
                bytes: Arc::new(bytes),
                entry,
                layout,
            },
            RequestOptions::default(),
            true,
        )
    }
    fn request(&self, job: Job, options: RequestOptions) -> Result<PipelineTicket> {
        self.request_inner(job, options, false)
    }
    fn request_inner(
        &self,
        job: Job,
        options: RequestOptions,
        deferred: bool,
    ) -> Result<PipelineTicket> {
        let refused = || {
            if deferred {
                Error::OutOfHostMemory
            } else {
                Error::Timeout
            }
        };
        if self.shared.cache.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !Arc::ptr_eq(job.device(), &self.shared.cache.device) {
            return Err(Error::invalid_usage(
                "pipeline request belongs to another device",
            ));
        }
        let bytes = job.retained_bytes();
        if bytes > self.shared.config.max_request_bytes {
            return Err(refused());
        }
        let mut state = self.shared.state.lock();
        if state.stopping {
            return Err(Error::invalid_usage("pipeline compiler is shutting down"));
        }
        if let Some((key, cached)) = state.cached.iter().find(|(key, _)| job.matches_key(key)) {
            let key = key.clone();
            let completion = cached.completion.clone();
            if let Some(queued) = state.queue.iter_mut().find(|queued| queued.key == key) {
                queued.options.priority = queued.options.priority.max(options.priority);
                queued.options.epoch = queued.options.epoch.max(options.epoch);
            }
            state.merged += 1;
            if let Some(index) = state.lru.iter().position(|old| old == &key) {
                state.lru.remove(index);
            }
            state.lru.push_back(key);
            return Ok(PipelineTicket {
                completion,
                compiler: Arc::downgrade(&self.shared),
            });
        }
        if options.priority == CompilePriority::Background
            && self.shared.cache.device.memory.stats.budget_action()
                != crate::accounting::BudgetAction::None
        {
            return Err(refused());
        }
        let pending_limit = if deferred {
            self.shared.config.max_deferred_pending
        } else {
            self.shared.config.max_pending
        };
        if state.pending >= pending_limit
            || state
                .request_bytes
                .checked_add(bytes)
                .is_none_or(|total| total > self.shared.config.max_request_bytes)
        {
            return Err(refused());
        }
        let key_limit = if deferred {
            self.shared
                .config
                .max_cached_requests
                .saturating_add(self.shared.config.max_deferred_pending)
        } else {
            self.shared.config.max_cached_requests
        };
        while state.cached.len() >= key_limit
            || state
                .cache_bytes
                .checked_add(bytes)
                .is_none_or(|total| total > self.shared.config.max_request_bytes)
        {
            let position = state.lru.iter().position(|key| {
                state.cached.get(key).is_some_and(|entry| {
                    !matches!(*entry.completion.status.lock(), PipelinePoll::Pending)
                })
            });
            let Some(position) = position else {
                return Err(refused());
            };
            let victim = state.lru.remove(position).expect("bounded LRU position");
            if let Some(entry) = state.cached.remove(&victim) {
                state.cache_bytes -= entry.bytes;
            }
        }
        let reservation = self
            .shared
            .cache
            .device
            .memory
            .stats
            .reserve_cpu_bytes(bytes as u64, crate::accounting::BudgetCategory::Live)?;
        let key = job.key();
        let completion = Arc::new(Completion {
            status: Mutex::new(PipelinePoll::Pending),
            changed: Condvar::new(),
        });
        state.cached.insert(
            key.clone(),
            Cached {
                reservation,
                completion: completion.clone(),
                bytes,
            },
        );
        state.lru.push_back(key.clone());
        state.queue.push_back(Queued {
            options,
            key,
            job,
            completion: completion.clone(),
            bytes,
        });
        state.pending += 1;
        state.request_bytes += bytes;
        state.cache_bytes += bytes;
        state.peak_pending = state.peak_pending.max(state.pending);
        state.peak_request_bytes = state.peak_request_bytes.max(state.request_bytes);
        self.shared.available.notify_one();
        Ok(PipelineTicket {
            completion,
            compiler: Arc::downgrade(&self.shared),
        })
    }
    pub fn stats(&self) -> CompilerStats {
        let state = self.shared.state.lock();
        CompilerStats {
            cancelled: state.cancelled,
            pending: state.pending,
            request_bytes: state.request_bytes,
            cached_requests: state.cached.len(),
            cached_key_bytes: state.cache_bytes,
            peak_pending: state.peak_pending,
            peak_request_bytes: state.peak_request_bytes,
            compiled: state.compiled,
            failed: state.failed,
            merged: state.merged,
        }
    }
    /// Cancel stale queued work. Active Vulkan calls finish normally; critical
    /// requests and a key joined by a newer epoch are retained.
    pub fn cancel_queued_before(&self, epoch: u64) -> usize {
        let mut state = self.shared.state.lock();
        let mut index = 0;
        let mut count = 0;
        while index < state.queue.len() {
            let options = state.queue[index].options;
            if options.epoch < epoch && options.priority != CompilePriority::Critical {
                let queued = state
                    .queue
                    .remove(index)
                    .expect("queued cancellation index");
                cancel_queued(&mut state, queued);
                count += 1;
            } else {
                index += 1;
            }
        }
        count
    }
}

pub(super) fn cancel_queued(state: &mut State, queued: Queued) {
    let Queued {
        key,
        job,
        completion,
        bytes,
        ..
    } = queued;
    let removed = state.cached.remove(&key);
    if let Some(entry) = &removed {
        state.cache_bytes -= entry.bytes;
    }
    state.lru.retain(|old| old != &key);
    state.pending -= 1;
    state.request_bytes -= bytes;
    state.cancelled += 1;
    // Budget credit follows the last payload owner, before cancellation becomes
    // observable to a caller that may immediately retry the same request.
    drop(job);
    drop(key);
    drop(removed);
    *completion.status.lock() =
        PipelinePoll::Failed(Arc::new(Error::invalid_usage("pipeline request cancelled")));
    completion.changed.notify_all();
}

pub(super) fn worker(shared: Arc<Shared>) {
    loop {
        let queued = {
            let mut state = shared.state.lock();
            while state.queue.is_empty() && !state.stopping {
                shared.available.wait(&mut state);
            }
            if state.stopping && state.queue.is_empty() {
                return;
            }
            let index = if state.priority_burst >= 8 {
                state.priority_burst = 0;
                0
            } else {
                state.priority_burst += 1;
                state
                    .queue
                    .iter()
                    .enumerate()
                    .max_by_key(|(index, queued)| {
                        (queued.options.priority, std::cmp::Reverse(*index))
                    })
                    .map_or(0, |(index, _)| index)
            };
            state.queue.remove(index).expect("worker queue predicate")
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if shared.cache.device.is_device_lost() {
                Err(Error::DeviceLost)
            } else {
                queued.job.compile(&shared.cache)
            }
        }))
        .unwrap_or_else(|_| {
            Err(Error::invalid_usage(
                "background pipeline compiler panicked",
            ))
        });
        if result.as_ref().is_err_and(|error| error.is_device_lost()) {
            shared.cache.device.queues.timelines.mark_device_lost();
        }
        // Remove failed keys before publishing failure. The queued descriptor
        // retains its original Arc identities through this critical section,
        // including failed creation, so raw Vulkan handles cannot suffer ABA.
        let failed = result.is_err();
        let mut state = shared.state.lock();
        let removed = if failed {
            let removed = state.cached.remove(&queued.key);
            if let Some(entry) = &removed {
                state.cache_bytes -= entry.bytes;
            }
            state.lru.retain(|key| key != &queued.key);
            removed
        } else {
            if let Some(entry) = state.cached.get_mut(&queued.key) {
                entry
                    .reservation
                    .reclassify(crate::accounting::BudgetCategory::Cache);
            }
            None
        };
        state.pending -= 1;
        state.request_bytes -= queued.bytes;
        if failed { state.failed += 1; } else { state.compiled += 1; }
        let Queued {
            job,
            key,
            completion,
            ..
        } = queued;
        drop(job);
        drop(key);
        drop(removed);
        *completion.status.lock() = match result {
            Ok(pipeline) => PipelinePoll::Ready(pipeline),
            Err(error) => PipelinePoll::Failed(Arc::new(error)),
        };
        completion.changed.notify_all();
        // Pending keys are retained for deduplication. Once work completes,
        // trim back to the configured completed-request cache bound even when
        // a deferred burst temporarily held more pending keys.
        while state.cached.len() > shared.config.max_cached_requests {
            let Some(index) = state.lru.iter().position(|key| {
                state.cached.get(key).is_some_and(|entry| {
                    !matches!(*entry.completion.status.lock(), PipelinePoll::Pending)
                })
            }) else {
                break;
            };
            let key = state.lru.remove(index).expect("completed LRU entry");
            if let Some(entry) = state.cached.remove(&key) {
                state.cache_bytes -= entry.bytes;
            }
        }
    }
}

pub(super) fn evict_completed_requests(state: &mut State, target_bytes: u64) -> u64 {
    let mut released = 0u64;
    let mut index = 0;
    while index < state.lru.len() && released < target_bytes {
        let idle = state.cached.get(&state.lru[index]).is_some_and(|entry| {
            Arc::strong_count(&entry.completion) == 1
                && !matches!(*entry.completion.status.lock(), PipelinePoll::Pending)
        });
        if !idle {
            index += 1;
            continue;
        }
        let key = state.lru.remove(index).expect("bounded LRU position");
        if let Some(entry) = state.cached.remove(&key) {
            state.cache_bytes -= entry.bytes;
            released += entry.bytes as u64;
        }
    }
    released
}

/// Under memory pressure, completed requests that no ticket references are
/// dropped from the request cache in LRU order. Pipelines still referenced by
/// callers or recorded commands stay alive through their own owners.
impl crate::accounting::Evictable for Shared {
    fn evict_idle(&self, _completed: &crate::sync::CompletedValues, target_bytes: u64) -> u64 {
        let Some(mut state) = self.state.try_lock() else {
            return 0;
        };
        evict_completed_requests(&mut state, target_bytes)
    }
}

impl Drop for AsyncPipelineCompiler {
    fn drop(&mut self) {
        {
            let mut state = self.shared.state.lock();
            state.stopping = true;
            while let Some(queued) = state.queue.pop_front() {
                cancel_queued(&mut state, queued);
            }
        }
        self.shared.available.notify_all();
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}

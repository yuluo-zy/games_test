//! Bounded background pipeline creation, persistent driver data and explicit
//! publication at a frame boundary. The compiler is owned by the renderer,
//! never by Device, so cached pipelines cannot form a Device ownership cycle.

pub use crate::raw::shader::{
    ComputePipelineDesc, DynamicRenderingFormats, PipelineCache, PipelineCacheData, PipelineLayout,
    RasterPipelineDesc, RasterPipelineOpts, ShaderLibrary, ShaderModule, VulkanPipeline,
    get_vk_blend_mode, reflect_spirv_entries, vertex_input_from_reflection,
};
use crate::{Error, Result, raw::device::Device};
use ash::vk::Handle;
use parking_lot::{Condvar, Mutex};
use std::{
    collections::{BTreeMap, VecDeque},
    ffi::CString,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
pub struct CompilerConfig {
    pub workers: usize,
    /// Includes queued and actively compiling jobs.
    pub max_pending: usize,
    /// Total queued + active requests accepted by the deferred frame API.
    /// Worker concurrency is still `workers`; bytes share max_request_bytes.
    pub max_deferred_pending: usize,
    pub max_request_bytes: usize,
    pub max_cached_requests: usize,
}
impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            workers: 1,
            max_pending: 64,
            max_deferred_pending: 1024,
            max_request_bytes: 16 << 20,
            max_cached_requests: 256,
        }
    }
}
impl CompilerConfig {
    fn validate(self) -> Result<()> {
        if self.workers == 0
            || self.workers > 16
            || self.max_pending == 0
            || self.max_deferred_pending == 0
            || self.max_request_bytes == 0
            || self.max_cached_requests == 0
        {
            return Err(Error::invalid_usage(
                "invalid bounded pipeline compiler configuration",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompilePriority {
    Background,
    #[default]
    Normal,
    Critical,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct RequestOptions {
    pub priority: CompilePriority,
    pub epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum KeyPayload {
    Compute(u64, u64, Vec<u8>),
    Raster(Vec<u64>),
    ComputeSpirv(u64, Arc<Vec<u8>>, Vec<u8>),
}
type Key = Arc<KeyPayload>;
enum Job {
    Compute(ComputePipelineDesc),
    Raster(Box<RasterPipelineDesc>),
    ComputeSpirv {
        bytes: Arc<Vec<u8>>,
        entry: CString,
        layout: Arc<PipelineLayout>,
    },
}
impl Job {
    /// Compare borrowed input before admission. A full queue or budget rejection
    /// must not first duplicate a shader payload or a large raster key.
    fn matches_key(&self, key: &KeyPayload) -> bool {
        match (self, key) {
            (Self::Compute(d), KeyPayload::Compute(shader, layout, entry)) => {
                d.shader.handle.as_raw() == *shader
                    && d.layout.handle.as_raw() == *layout
                    && d.entry.as_bytes() == entry
            }
            (Self::Raster(d), KeyPayload::Raster(key)) => {
                crate::raw::shader::raster_pipeline_key_matches(d, key)
            }
            (
                Self::ComputeSpirv {
                    bytes,
                    entry,
                    layout,
                },
                KeyPayload::ComputeSpirv(l, b, e),
            ) => layout.handle.as_raw() == *l && bytes == b && entry.as_bytes() == e,
            _ => false,
        }
    }
    fn key(&self) -> Key {
        Arc::new(match self {
            Self::Compute(d) => KeyPayload::Compute(
                d.shader.handle.as_raw(),
                d.layout.handle.as_raw(),
                d.entry.as_bytes().to_vec(),
            ),
            Self::Raster(d) => KeyPayload::Raster(crate::raw::shader::raster_pipeline_key(d)),
            Self::ComputeSpirv {
                bytes,
                entry,
                layout,
            } => KeyPayload::ComputeSpirv(
                layout.handle.as_raw(),
                bytes.clone(),
                entry.as_bytes().to_vec(),
            ),
        })
    }
    fn retained_bytes(&self) -> usize {
        match self {
            Self::Compute(d) => {
                256usize.saturating_add(d.entry.as_bytes_with_nul().len().saturating_mul(2))
            }
            Self::Raster(d) => {
                512usize
                    .saturating_add(
                        crate::raw::shader::raster_pipeline_key_len(d).saturating_mul(8),
                    )
                    .saturating_add(d.vertex_bindings.capacity().saturating_mul(
                        std::mem::size_of::<ash::vk::VertexInputBindingDescription>(),
                    ))
                    .saturating_add(d.vertex_attributes.capacity().saturating_mul(
                        std::mem::size_of::<ash::vk::VertexInputAttributeDescription>(),
                    ))
                    .saturating_add(d.blend_attachments.capacity().saturating_mul(
                        std::mem::size_of::<ash::vk::PipelineColorBlendAttachmentState>(),
                    ))
                    .saturating_add(
                        d.dynamic_rendering
                            .as_ref()
                            .map_or(0, |formats| formats.colors.capacity().saturating_mul(4)),
                    )
                    .saturating_add(d.entry.as_bytes_with_nul().len())
                    .saturating_add(
                        d.fragment_entry
                            .as_ref()
                            .map_or(0, |entry| entry.as_bytes_with_nul().len()),
                    )
            }
            Self::ComputeSpirv { bytes, entry, .. } => 256usize
                .saturating_add(bytes.capacity())
                .saturating_add(entry.as_bytes().len().saturating_mul(2)),
        }
    }
    fn device(&self) -> &Arc<Device> {
        match self {
            Self::Compute(d) => &d.layout.device,
            Self::Raster(d) => &d.layout.device,
            Self::ComputeSpirv { layout, .. } => &layout.device,
        }
    }
    fn compile(&self, cache: &PipelineCache) -> Result<Arc<VulkanPipeline>> {
        match self {
            Self::Compute(desc) => VulkanPipeline::create_compute_pipeline(cache, desc.clone()),
            Self::Raster(desc) => VulkanPipeline::create_raster_pipeline(cache, (**desc).clone()),
            Self::ComputeSpirv {
                bytes,
                entry,
                layout,
            } => {
                // Validate the SPIR-V entry and stage before giving a hot-reload
                // candidate to Vulkan. Broken edits never replace the old one.
                let words = ash::util::read_spv(&mut std::io::Cursor::new(bytes.as_slice()))
                    .map_err(|_| Error::invalid_usage("invalid hot-reload SPIR-V"))?;
                crate::raw::shader_interface::validate_spirv_layout(
                    &words,
                    entry,
                    ash::vk::ShaderStageFlags::COMPUTE,
                    layout,
                )?;
                let shader = ShaderModule::from_spirv(cache.device.clone(), bytes.as_slice())?;
                VulkanPipeline::create_compute_pipeline(
                    cache,
                    ComputePipelineDesc {
                        shader,
                        entry: entry.clone(),
                        layout: layout.clone(),
                    },
                )
            }
        }
    }
}

struct Completion {
    status: Mutex<PipelinePoll>,
    changed: Condvar,
}
#[derive(Clone)]
pub struct PipelineTicket {
    completion: Arc<Completion>,
    compiler: std::sync::Weak<Shared>,
}
#[derive(Clone)]
pub enum PipelinePoll {
    Pending,
    Ready(Arc<VulkanPipeline>),
    Failed(Arc<Error>),
}
impl PipelineTicket {
    /// Nonblocking production access. Pending work applies explicit Timeout
    /// backpressure; terminal driver errors retain their original categories.
    pub fn try_get(&self) -> Result<Arc<VulkanPipeline>> {
        match self.poll() {
            PipelinePoll::Pending => Err(Error::Timeout),
            PipelinePoll::Ready(pipeline) => Ok(pipeline),
            PipelinePoll::Failed(error) => Err(error.into()),
        }
    }
    pub fn poll(&self) -> PipelinePoll {
        self.completion.status.lock().clone()
    }
    fn cancel_unshared_queued(&self) {
        let Some(shared) = self.compiler.upgrade() else {
            return;
        };
        let mut state = shared.state.lock();
        // Cache + queue + this pending owner. Other tickets keep a merged
        // request alive; an active native compilation always finishes normally.
        if Arc::strong_count(&self.completion) != 3 {
            return;
        }
        if let Some(index) = state
            .queue
            .iter()
            .position(|job| Arc::ptr_eq(&job.completion, &self.completion))
        {
            let queued = state.queue.remove(index).expect("located queued pipeline");
            cancel_queued(&mut state, queued);
        }
    }
    /// Intended for startup/prewarming and tests. A render frame should poll
    /// and explicitly keep its fallback or skip when the ticket is pending.
    pub fn wait(&self, timeout: Duration) -> std::result::Result<Arc<VulkanPipeline>, Arc<Error>> {
        let deadline = Instant::now().checked_add(timeout);
        let mut status = self.completion.status.lock();
        loop {
            match &*status {
                PipelinePoll::Ready(pipeline) => return Ok(pipeline.clone()),
                PipelinePoll::Failed(error) => return Err(error.clone()),
                PipelinePoll::Pending => {
                    let remaining = deadline
                        .and_then(|deadline| deadline.checked_duration_since(Instant::now()))
                        .ok_or_else(|| Arc::new(Error::Timeout))?;
                    if self
                        .completion
                        .changed
                        .wait_for(&mut status, remaining)
                        .timed_out()
                        && matches!(*status, PipelinePoll::Pending)
                    {
                        return Err(Arc::new(Error::Timeout));
                    }
                }
            }
        }
    }
    pub fn same_request(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.completion, &other.completion)
    }
}
struct Queued {
    options: RequestOptions,
    key: Key,
    job: Job,
    completion: Arc<Completion>,
    bytes: usize,
}
struct Cached {
    reservation: crate::raw::allocator::CpuMemoryReservation,
    completion: Arc<Completion>,
    bytes: usize,
}
struct State {
    priority_burst: usize,
    cancelled: u64,
    queue: VecDeque<Queued>,
    cached: BTreeMap<Key, Cached>,
    lru: VecDeque<Key>,
    pending: usize,
    request_bytes: usize,
    cache_bytes: usize,
    stopping: bool,
    peak_pending: usize,
    peak_request_bytes: usize,
    compiled: u64,
    failed: u64,
    merged: u64,
}
struct Shared {
    state: Mutex<State>,
    available: Condvar,
    cache: Arc<PipelineCache>,
    config: CompilerConfig,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct CompilerStats {
    pub cancelled: u64,
    pub pending: usize,
    pub request_bytes: usize,
    pub cached_requests: usize,
    pub cached_key_bytes: usize,
    pub peak_pending: usize,
    pub peak_request_bytes: usize,
    /// Successfully created pipelines. Failed attempts are counted separately.
    pub compiled: u64,
    pub failed: u64,
    pub merged: u64,
}
pub struct AsyncPipelineCompiler {
    shared: Arc<Shared>,
    workers: Vec<JoinHandle<()>>,
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
        let evictable: Arc<dyn crate::device::Evictable> = shared.clone();
        shared.cache.device.register_evictable(
            Arc::downgrade(&evictable),
            crate::device::EvictionPriority::Expensive,
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
                != crate::device::BudgetAction::None
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
            .reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Live)?;
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
fn cancel_queued(state: &mut State, queued: Queued) {
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
fn worker(shared: Arc<Shared>) {
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
                    .reclassify(crate::device::BudgetCategory::Cache);
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
/// Under memory pressure, completed requests that no ticket references are
/// dropped from the request cache in LRU order. Pipelines still referenced by
/// callers or recorded commands stay alive through their own owners.
impl crate::device::Evictable for Shared {
    fn evict_idle(&self, _completed: &crate::sync::CompletedValues, target_bytes: u64) -> u64 {
        let Some(mut state) = self.state.try_lock() else {
            return 0;
        };
        evict_completed_requests(&mut state, target_bytes)
    }
}
fn evict_completed_requests(state: &mut State, target_bytes: u64) -> u64 {
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

/// Existing draws retain Arc owners and their SyncCell dependencies after a
/// successful publication. Failed/pending replacements keep the old pipeline.
pub struct HotPipeline {
    current: Arc<VulkanPipeline>,
    pending: Option<PipelineTicket>,
    last_error: Option<Arc<Error>>,
    revision: u64,
}
impl HotPipeline {
    pub fn new(current: Arc<VulkanPipeline>) -> Self {
        Self {
            current,
            pending: None,
            last_error: None,
            revision: 0,
        }
    }
    pub fn stage(&mut self, ticket: PipelineTicket) {
        if self
            .pending
            .as_ref()
            .is_some_and(|old| Arc::ptr_eq(&old.completion, &ticket.completion))
        {
            return;
        }
        if let Some(previous) = self.pending.replace(ticket) {
            previous.cancel_unshared_queued();
        }
    }
    pub fn current(&self) -> Arc<VulkanPipeline> {
        self.current.clone()
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn last_error(&self) -> Option<&Arc<Error>> {
        self.last_error.as_ref()
    }
    pub fn publish_at_frame_boundary(&mut self) -> bool {
        if self.current.device.is_device_lost() {
            self.pending = None;
            self.last_error = Some(Arc::new(Error::DeviceLost));
            return false;
        }
        let Some(ticket) = &self.pending else {
            return false;
        };
        match ticket.poll() {
            PipelinePoll::Pending => false,
            PipelinePoll::Failed(error) => {
                self.last_error = Some(error);
                self.pending = None;
                false
            }
            PipelinePoll::Ready(next) => {
                let compatible = Arc::ptr_eq(&next.device, &self.current.device)
                    && next.layout.bindless_abi() == self.current.layout.bindless_abi()
                    && next.point == self.current.point
                    && next.layout.sets.len() == self.current.layout.sets.len()
                    && next
                        .layout
                        .sets
                        .iter()
                        .zip(&self.current.layout.sets)
                        .all(|(a, b)| a.plan == b.plan)
                    && next.layout.push_constants.len() == self.current.layout.push_constants.len()
                    && next
                        .layout
                        .push_constants
                        .iter()
                        .zip(&self.current.layout.push_constants)
                        .all(|(a, b)| {
                            a.offset == b.offset
                                && a.size == b.size
                                && a.stage_flags == b.stage_flags
                        })
                    && next.raster_target_signature() == self.current.raster_target_signature()
                    && next.has_same_shader_interface(&self.current);
                self.pending = None;
                if !compatible {
                    self.last_error = Some(Arc::new(Error::invalid_usage(
                        "hot-reload pipeline interface/target changed",
                    )));
                    return false;
                }
                self.current = next;
                self.last_error = None;
                self.revision += 1;
                true
            }
        }
    }
}

const CACHE_MAGIC: &[u8; 8] = b"RHPC0001";
const MAX_DISK_BYTES: u64 = crate::raw::shader::MAX_PIPELINE_CACHE_BYTES;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskCacheStatus {
    Missing,
    Loaded,
    Rejected,
}
pub struct PersistentPipelineCache {
    pub cache: Arc<PipelineCache>,
    path: PathBuf,
    identity: [u8; 32],
    pub status: DiskCacheStatus,
}
fn identity(device: &Device) -> [u8; 32] {
    let properties = device.properties();
    let mut bytes = [0; 32];
    for (index, value) in [
        properties.vendor_id,
        properties.device_id,
        properties.driver_version,
        properties.api_version,
    ]
    .into_iter()
    .enumerate()
    {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes[16..].copy_from_slice(&properties.pipeline_cache_uuid);
    bytes
}
fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}
#[cfg(test)]
fn encode(identity: &[u8], data: &[u8]) -> Vec<u8> {
    let mut bytes = CACHE_MAGIC.to_vec();
    bytes.extend(identity);
    bytes.extend((data.len() as u64).to_le_bytes());
    bytes.extend(checksum(data).to_le_bytes());
    bytes.extend(data);
    bytes
}
fn decode<'a>(bytes: &'a [u8], identity: &[u8]) -> Option<&'a [u8]> {
    let header = 8 + identity.len() + 16;
    if bytes.len() < header
        || &bytes[..8] != CACHE_MAGIC
        || &bytes[8..8 + identity.len()] != identity
    {
        return None;
    }
    let offset = 8 + identity.len();
    let size = u64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
    if size > MAX_DISK_BYTES || usize::try_from(size).ok()? != bytes.len() - header {
        return None;
    }
    let expected = u64::from_le_bytes(bytes[offset + 8..header].try_into().ok()?);
    let data = &bytes[header..];
    (checksum(data) == expected).then_some(data)
}
impl PersistentPipelineCache {
    pub fn open(device: Arc<Device>, path: impl Into<PathBuf>, capacity: usize) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let path = path.into();
        let identity = identity(&device);
        let mut bytes = None;
        let mut status = match fs::File::open(&path) {
            Ok(mut file) => {
                let length = file.metadata().ok().map(|metadata| metadata.len());
                let loaded =
                    if let Some(length) = length.filter(|&length| length <= MAX_DISK_BYTES + 128) {
                        let mut payload =
                            PipelineCacheData::zeroed(&device.memory.stats, length as usize)?;
                        let mut trailing = [0];
                        if file.read_exact(payload.as_mut_slice()).is_ok()
                            && file.read(&mut trailing).is_ok_and(|read| read == 0)
                            && decode(&payload, &identity).is_some()
                        {
                            bytes = Some(payload);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                if loaded {
                    DiskCacheStatus::Loaded
                } else {
                    DiskCacheStatus::Rejected
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => DiskCacheStatus::Missing,
            Err(_) => DiskCacheStatus::Rejected,
        };
        let mut data = if status == DiskCacheStatus::Loaded {
            decode(bytes.as_deref().unwrap_or_default(), &identity).unwrap_or_default()
        } else {
            &[]
        };
        if !data.is_empty() && !crate::raw::shader::valid_pipeline_cache_data(&device, data) {
            status = DiskCacheStatus::Rejected;
            data = &[];
        }
        let cache = match PipelineCache::with_capacity(device.clone(), data, capacity) {
            Ok(cache) => cache,
            Err(error) if !data.is_empty() && matches!(error, Error::Vulkan(_)) => {
                status = DiskCacheStatus::Rejected;
                PipelineCache::with_capacity(device, &[], capacity)?
            }
            Err(error) => return Err(error),
        };
        Ok(Self {
            cache: Arc::new(cache),
            path,
            identity,
            status,
        })
    }
    pub fn save(&self) -> Result<()> {
        let data = self.cache.data()?;
        if data.len() as u64 > MAX_DISK_BYTES {
            return Err(Error::invalid_usage(
                "pipeline disk cache exceeds its byte limit",
            ));
        }
        // Stream the fixed envelope and the accounted driver bytes. Building a
        // second 64 MiB copy would double peak host memory during save.
        let mut header = [0; 56];
        header[..8].copy_from_slice(CACHE_MAGIC);
        header[8..40].copy_from_slice(&self.identity);
        header[40..48].copy_from_slice(&(data.len() as u64).to_le_bytes());
        header[48..].copy_from_slice(&checksum(&data).to_le_bytes());
        atomic_write_parts(&self.path, &[&header, &data])
    }
}
fn atomic_write_parts(path: &Path, parts: &[&[u8]]) -> Result<()> {
    static NEXT_FILE: AtomicU64 = AtomicU64::new(1);
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(format!(
        ".{}.{}.tmp",
        std::process::id(),
        NEXT_FILE.fetch_add(1, Ordering::Relaxed)
    ));
    let temporary = PathBuf::from(temporary);
    atomic_write_temporary(path, &temporary, parts)
}
fn atomic_write_temporary(path: &Path, temporary: &Path, parts: &[&[u8]]) -> Result<()> {
    let mut created = false;
    let result = (|| {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(temporary)?;
        created = true;
        for part in parts {
            file.write_all(part)?;
        }
        file.sync_all()?;
        drop(file);
        fs::rename(temporary, path)
    })();
    if result.is_err() && created {
        let _ = fs::remove_file(temporary);
    }
    result.map_err(|error| Error::invalid_usage_with("save persistent pipeline cache", error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_evicts_only_completed_unreferenced_requests_in_lru_order() {
        let stats = Arc::new(crate::raw::allocator::AllocatorStats::default());
        let mut state = State {
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
        };
        let mut tickets = Vec::new();
        // (status, ticket alive) in LRU order.
        let entries = [
            (PipelinePoll::Failed(Arc::new(Error::OutOfHostMemory)), false),
            (PipelinePoll::Pending, false),
            (PipelinePoll::Failed(Arc::new(Error::OutOfHostMemory)), true),
            (PipelinePoll::Failed(Arc::new(Error::OutOfHostMemory)), false),
        ];
        for (index, (status, referenced)) in entries.into_iter().enumerate() {
            let key: Key = Arc::new(KeyPayload::Raster(vec![index as u64]));
            let completion = Arc::new(Completion {
                status: Mutex::new(status),
                changed: Condvar::new(),
            });
            if referenced {
                tickets.push(completion.clone());
            }
            state.cached.insert(
                key.clone(),
                Cached {
                    reservation: stats
                        .reserve_cpu_bytes(10, crate::device::BudgetCategory::Cache)
                        .unwrap(),
                    completion,
                    bytes: 10,
                },
            );
            state.lru.push_back(key);
            state.cache_bytes += 10;
        }
        assert_eq!(evict_completed_requests(&mut state, 5), 10);
        assert_eq!(state.cached.len(), 3);
        assert_eq!(evict_completed_requests(&mut state, 100), 10);
        let remaining: Vec<_> = state
            .lru
            .iter()
            .map(|key| match &**key {
                KeyPayload::Raster(values) => values[0],
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(
            remaining,
            [1, 2],
            "pending and ticket-referenced requests stay"
        );
        assert_eq!(state.cache_bytes, 20);
        assert_eq!(stats.cpu_memory_usage().cache, 20);
        drop(tickets);
    }
    #[test]
    fn atomic_cache_write_does_not_remove_an_unowned_temporary_collision() {
        let directory = std::env::temp_dir().join(format!(
            "rhapsody-cache-collision-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        let cache = directory.join("cache");
        let temporary = directory.join("occupied.tmp");
        fs::write(&cache, b"previous cache").unwrap();
        fs::write(&temporary, b"another writer owns this").unwrap();
        assert!(atomic_write_temporary(&cache, &temporary, &[b"new cache"]).is_err());
        assert_eq!(fs::read(&temporary).unwrap(), b"another writer owns this");
        assert_eq!(fs::read(&cache).unwrap(), b"previous cache");
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn disk_envelope_rejects_corruption_stale_identity_and_truncation() {
        let identity = vec![7; 32];
        let bytes = encode(&identity, b"driver payload");
        assert_eq!(decode(&bytes, &identity), Some(&b"driver payload"[..]));
        let mut corrupt = bytes.clone();
        *corrupt.last_mut().unwrap() ^= 1;
        assert!(decode(&corrupt, &identity).is_none());
        assert!(decode(&bytes, &[8; 32]).is_none());
        for length in 0..bytes.len() {
            assert!(decode(&bytes[..length], &identity).is_none());
        }
    }
    #[test]
    fn deferred_queue_owns_bounded_pending_requests_and_merges_before_refusing() {
        let device =
            match crate::device::Device::new_headless(&crate::device::DeviceConfig::headless()) {
                Ok(device) => device,
                Err(error) => {
                    assert!(
                        std::env::var_os("RHAPSODY_REQUIRE_VULKAN").is_none(),
                        "{error}"
                    );
                    return;
                }
            };
        let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
        let mut compiler = AsyncPipelineCompiler::new(
            Arc::new(PipelineCache::new(device, &[]).unwrap()),
            CompilerConfig {
                max_pending: 1,
                max_deferred_pending: 3,
                max_cached_requests: 1,
                ..Default::default()
            },
        )
        .unwrap();
        // Stop idle workers before enqueueing to deterministically exercise a
        // saturated queue, independent of native driver compilation speed.
        compiler.shared.state.lock().stopping = true;
        compiler.shared.available.notify_all();
        for worker in compiler.workers.drain(..) {
            worker.join().unwrap();
        }
        compiler.shared.state.lock().stopping = false;
        let mut tickets = Vec::new();
        for value in 0..3 {
            tickets.push(
                compiler
                    .defer_compute_spirv(vec![value; 4], c"main".to_owned(), layout.clone())
                    .unwrap(),
            );
        }
        assert_eq!(compiler.stats().pending, 3);
        let merged = compiler
            .defer_compute_spirv(vec![0; 4], c"main".to_owned(), layout.clone())
            .unwrap();
        assert!(merged.same_request(&tickets[0]));
        let before = compiler.stats();
        assert!(matches!(
            compiler.defer_compute_spirv(vec![3; 4], c"main".to_owned(), layout.clone()),
            Err(Error::OutOfHostMemory)
        ));
        assert!(matches!(
            compiler.request_compute_spirv(vec![3; 4], c"main".to_owned(), layout),
            Err(Error::Timeout)
        ));
        let after = compiler.stats();
        assert_eq!(
            (
                before.pending,
                before.request_bytes,
                before.cached_key_bytes
            ),
            (after.pending, after.request_bytes, after.cached_key_bytes)
        );
        assert!(
            tickets
                .iter()
                .all(|ticket| matches!(ticket.poll(), PipelinePoll::Pending))
        );
        let shared = compiler.shared.clone();
        compiler
            .workers
            .push(std::thread::spawn(move || worker(shared)));
        for ticket in &tickets {
            assert!(
                ticket.wait(Duration::from_secs(10)).is_err(),
                "invalid SPIR-V must retain its terminal error"
            );
        }
        // Failed jobs leave the request table; no permanently full queue remains.
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while compiler.stats().pending != 0 {
            assert!(std::time::Instant::now() < deadline);
            std::thread::yield_now();
        }
        assert_eq!(compiler.stats().request_bytes, 0);
        assert!(compiler.stats().cached_requests <= 1);
    }

    #[test]
    fn worker_and_memory_limits_must_be_bounded() {
        assert!(CompilerConfig::default().validate().is_ok());
        for config in [
            CompilerConfig {
                workers: 0,
                ..Default::default()
            },
            CompilerConfig {
                workers: 17,
                ..Default::default()
            },
            CompilerConfig {
                max_pending: 0,
                ..Default::default()
            },
            CompilerConfig {
                max_deferred_pending: 0,
                ..Default::default()
            },
            CompilerConfig {
                max_request_bytes: 0,
                ..Default::default()
            },
            CompilerConfig {
                max_cached_requests: 0,
                ..Default::default()
            },
        ] {
            assert!(config.validate().is_err());
        }
    }
}

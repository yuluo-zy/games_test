//! Compile requests, tickets, priorities and compiler state.
use ash::vk::Handle;
use crate::Error;
use crate::Result;
use crate::device::Device;
use parking_lot::Condvar;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::ffi::CString;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use super::compiler::cancel_queued;
use super::shader::ComputePipelineDesc;
use super::shader::PipelineCache;
use super::shader::PipelineLayout;
use super::shader::RasterPipelineDesc;
use super::shader::ShaderModule;
use super::shader::VulkanPipeline;

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
    pub(super) fn validate(self) -> Result<()> {
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
pub(super) enum KeyPayload {
    Compute(u64, u64, Vec<u8>),
    Raster(Vec<u64>),
    ComputeSpirv(u64, Arc<Vec<u8>>, Vec<u8>),
}

pub(super) type Key = Arc<KeyPayload>;

pub(super) enum Job {
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
    pub(super) fn matches_key(&self, key: &KeyPayload) -> bool {
        match (self, key) {
            (Self::Compute(d), KeyPayload::Compute(shader, layout, entry)) => {
                d.shader.handle.as_raw() == *shader
                    && d.layout.handle.as_raw() == *layout
                    && d.entry.as_bytes() == entry
            }
            (Self::Raster(d), KeyPayload::Raster(key)) => {
                crate::pipeline::shader::raster_pipeline_key_matches(d, key)
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
    pub(super) fn key(&self) -> Key {
        Arc::new(match self {
            Self::Compute(d) => KeyPayload::Compute(
                d.shader.handle.as_raw(),
                d.layout.handle.as_raw(),
                d.entry.as_bytes().to_vec(),
            ),
            Self::Raster(d) => KeyPayload::Raster(crate::pipeline::shader::raster_pipeline_key(d)),
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
    pub(super) fn retained_bytes(&self) -> usize {
        match self {
            Self::Compute(d) => {
                256usize.saturating_add(d.entry.as_bytes_with_nul().len().saturating_mul(2))
            }
            Self::Raster(d) => {
                512usize
                    .saturating_add(
                        crate::pipeline::shader::raster_pipeline_key_len(d).saturating_mul(8),
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
    pub(super) fn device(&self) -> &Arc<Device> {
        match self {
            Self::Compute(d) => &d.layout.device,
            Self::Raster(d) => &d.layout.device,
            Self::ComputeSpirv { layout, .. } => &layout.device,
        }
    }
    pub(super) fn compile(&self, cache: &PipelineCache) -> Result<Arc<VulkanPipeline>> {
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
                crate::pipeline::shader_interface::validate_spirv_layout(
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

pub(super) struct Completion {
    pub(super) status: Mutex<PipelinePoll>,
    pub(super) changed: Condvar,
}

#[derive(Clone)]
pub struct PipelineTicket {
    pub(super) completion: Arc<Completion>,
    pub(super) compiler: std::sync::Weak<Shared>,
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
    pub(super) fn cancel_unshared_queued(&self) {
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

#[derive(Clone)]
pub enum PipelinePoll {
    Pending,
    Ready(Arc<VulkanPipeline>),
    Failed(Arc<Error>),
}

pub(super) struct Queued {
    pub(super) options: RequestOptions,
    pub(super) key: Key,
    pub(super) job: Job,
    pub(super) completion: Arc<Completion>,
    pub(super) bytes: usize,
}

pub(super) struct Cached {
    pub(super) reservation: crate::accounting::CpuMemoryReservation,
    pub(super) completion: Arc<Completion>,
    pub(super) bytes: usize,
}

pub(super) struct State {
    pub(super) priority_burst: usize,
    pub(super) cancelled: u64,
    pub(super) queue: VecDeque<Queued>,
    pub(super) cached: BTreeMap<Key, Cached>,
    pub(super) lru: VecDeque<Key>,
    pub(super) pending: usize,
    pub(super) request_bytes: usize,
    pub(super) cache_bytes: usize,
    pub(super) stopping: bool,
    pub(super) peak_pending: usize,
    pub(super) peak_request_bytes: usize,
    pub(super) compiled: u64,
    pub(super) failed: u64,
    pub(super) merged: u64,
}

pub(super) struct Shared {
    pub(super) state: Mutex<State>,
    pub(super) available: Condvar,
    pub(super) cache: Arc<PipelineCache>,
    pub(super) config: CompilerConfig,
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

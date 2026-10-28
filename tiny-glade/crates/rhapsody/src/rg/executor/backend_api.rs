//! The backend trait and the types it exchanges with the executor.
use ash::vk;
use crate::command::CpuArenaPool;
use crate::command::recording;
use crate::native::submit;

#[derive(Clone, Copy, Debug)]
pub struct GraphTimelineWait {
    pub semaphore: vk::Semaphore,
    pub value: u64,
    pub stage: vk::PipelineStageFlags,
}

/// 外部生产的批次同步与图命令分别生成，避免调用者手填CPU recording identities。
#[derive(Clone, Debug, Default)]
pub struct GraphBatchSync {
    pub waits: submit::SubmitArray<GraphTimelineWait>,
    pub signals: submit::SubmitArray<crate::sync::timeline::SubmitSignal>,
}

impl GraphBatchSync {
    pub(super) fn valid(&self) -> bool {
        self.waits.iter().enumerate().all(|(index, wait)| {
            wait.semaphore != vk::Semaphore::null()
                && !wait.stage.is_empty()
                && !self.waits[..index]
                    .iter()
                    .any(|previous| previous.semaphore == wait.semaphore)
        }) && self.signals.iter().enumerate().all(|(index, signal)| {
            signal.semaphore != vk::Semaphore::null()
                && !self.signals[..index]
                    .iter()
                    .any(|previous| previous.semaphore == signal.semaphore)
        })
    }
}

/// 并行块内发出的一条 pass 前屏障；`token` 为物理资源 token。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkTransition {
    pub token: u64,
    pub transition: super::super::sync_plan::BoundTransition,
}

/// Exclusive right to record one chunk's command buffer during a parallel
/// section, standing for the external synchronization Vulkan requires of the
/// `VkCommandPool` that allocated it.
///
/// A backend creates leases only from disjoint `&mut` borrows of its
/// per-worker pools ([`ExecutionBackend::chunk_recording`]), so two live
/// leases can never stand for one pool; the borrow also outlives the parallel
/// section, so nothing else (allocation, reset, a second recording) touches
/// that pool meanwhile. A lease is `Send` (handed to one worker) but not
/// `Sync`: only the worker holding it records through it, and every
/// [`ChunkRecorder`] recording call requires it mutably.
///
/// ```compile_fail
/// # use rhapsody::rg::executor::ChunkLease;
/// let mut pool = ();
/// // A second lease of the same pool cannot coexist with the first.
/// let first = unsafe { ChunkLease::new(&mut pool, 1) };
/// let second = unsafe { ChunkLease::new(&mut pool, 2) };
/// drop((first, second));
/// ```
///
/// ```compile_fail
/// # use rhapsody::rg::executor::ChunkLease;
/// fn shared<T: Sync>(_: &T) {}
/// let mut pool = ();
/// shared(&unsafe { ChunkLease::new(&mut pool, 1) });
/// ```
pub struct ChunkLease<'a> {
    command: u64,
    _pool: std::marker::PhantomData<&'a mut ()>,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}

impl<'a> ChunkLease<'a> {
    /// # Safety
    /// `command` was allocated from the pool `pool` stands for, and no other
    /// command of that pool records while the lease lives. `pool` must be the
    /// backend's unique owner of that pool (its `&mut` borrow is what makes
    /// distinct leases name distinct pools).
    pub unsafe fn new<P: ?Sized>(pool: &'a mut P, command: u64) -> Self {
        let _ = pool;
        Self {
            command,
            _pool: std::marker::PhantomData,
            _not_sync: std::marker::PhantomData,
        }
    }
    /// The leased command buffer handle.
    pub fn command(&self) -> u64 {
        self.command
    }
}

/// A backend's parallel recording view with one [`ChunkLease`] per chunk.
pub type ChunkRecording<'a> = (
    Box<dyn ChunkRecorder + 'a>,
    smallvec::SmallVec<[ChunkLease<'a>; 8]>,
);

/// 后端在并行段暴露的只读录制视图：多个块同时调用，每次只写入调用方租约独占的命令缓冲。
pub trait ChunkRecorder: Sync {
    /// # Safety
    /// The command must be exclusively recording, and the pair must have been
    /// reserved for this command by this backend in the current CPU recording scope.
    unsafe fn write_pass_timestamp(
        &self,
        _lease: &mut ChunkLease<'_>,
        _pair: super::super::profiling::GpuPassTimestamp,
        _end: bool,
    ) {
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::collections::temp_list_pool::TempListPool<crate::command::binding::RetainedResource>>
    {
        None
    }
    fn cpu_arena_pool(&self) -> Option<&CpuArenaPool> {
        None
    }
    /// Emit one pass's scope. Native/OOM errors retain their public categories.
    fn record_transitions(
        &self,
        lease: &mut ChunkLease<'_>,
        transitions: &[ChunkTransition],
        owners: &mut super::super::owner_storage::ResourceOwners,
    ) -> crate::Result<()>;
}

#[derive(Clone, Copy, Debug)]
pub struct QueuePrelude {
    pub domain: super::super::queue_schedule::QueueDomain,
    pub tag: u64,
}

pub trait ExecutionBackend: crate::rg::graph::ResourceBackend {
    /// Reserve before recording workers run. Disabled profiling returns None
    /// and creates no query objects, owners or temporary heap allocations.
    fn reserve_pass_timestamp(
        &mut self,
        _command: u64,
        _pass: usize,
        _name: &crate::resource::ResourceName,
    ) -> Option<super::super::profiling::GpuPassTimestamp> {
        None
    }
    /// # Safety
    /// The command must be exclusively recording, and the pair must have been
    /// reserved for this command by this backend in the current CPU recording scope.
    unsafe fn write_pass_timestamp(
        &self,
        _command: u64,
        _pair: super::super::profiling::GpuPassTimestamp,
        _end: bool,
    ) {
    }
    fn requires_graph_completion(&self, _batch: usize, _total: usize) -> bool {
        false
    }
    fn select_queue_domain(
        &mut self,
        domain: super::super::queue_schedule::QueueDomain,
    ) -> crate::Result<()> {
        let context = self.plan_context();
        if context.queue_domain(crate::device::QueueClass::Graphics) != domain {
            return Err(crate::Error::unsupported(
                "backend does not support the requested queue domain",
            ));
        }
        Ok(())
    }
    fn next_queue_prelude(
        &mut self,
        _compiled: &crate::rg::graph::FrameInstance,
        _batch: usize,
    ) -> crate::Result<Option<QueuePrelude>> {
        Ok(None)
    }
    fn record_queue_prelude(&mut self, _command: u64, _prelude: QueuePrelude) -> crate::Result<()> {
        Ok(())
    }
    fn select_main_batch(
        &mut self,
        _compiled: &crate::rg::graph::FrameInstance,
        _batch: usize,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn timeline_wait(&self, _point: crate::sync::SyncPoint) -> crate::Result<GraphTimelineWait> {
        Err(crate::Error::unsupported(
            "backend has no device-owned queue timeline",
        ))
    }

    fn cpu_arena_pool(&self) -> Option<&CpuArenaPool> {
        None
    }
    fn plan_context(&self) -> super::super::plan::PlanContext {
        let (queue, family) = self.graph_queue();
        super::super::plan::PlanContext {
            queue_family: family,
            queue_index: match queue {
                recording::Queue::Primary => 0,
                recording::Queue::Auxiliary(i) => i + 1,
            },
            ..Default::default()
        }
    }
    /// Shared preflight for sessions and direct execution. Runs before allocation,
    /// recording, or submission; production backends repair partial-frame imports
    /// and validate physical identity, ownership and synchronization here.
    fn prepare_declaration(
        &mut self,
        _declaration: &crate::rg::graph::GraphDeclaration,
    ) -> crate::Result<()> {
        Ok(())
    }
    /// The same bounded projection drives worker chunks and ownership preludes.
    fn synchronization_plan(&self) -> std::sync::Arc<super::super::sync_plan::SynchronizationPlanner>;
    /// Attach pre-reserved physical state deltas to the actual raw batch before
    /// submission. Batch references resolve only to successful actual receipts.
    fn prepare_batch_submission(&mut self, commands: &[u64], batch: usize) -> crate::Result<()>;
    /// 包括 Pool 命中与导入资源；运行回调前重新发布真实物理映射。
    fn prepare_compiled_resources(&mut self, _compiled: &crate::rg::graph::FrameInstance) -> crate::Result<()> {
        Ok(())
    }
    /// 物理对象与 generation 由本次后端解析；未提供时 callback 明确看到 None。
    fn managed_pass_resources(
        &self,
    ) -> Option<std::sync::Arc<crate::rg::pass_bindings::ManagedPassResources>> {
        None
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::collections::temp_list_pool::TempListPool<crate::command::binding::RetainedResource>>
    {
        None
    }
    fn retain_command_owners(
        &mut self,
        _handle: u64,
        owners: &mut super::super::owner_storage::ResourceOwners,
    ) -> crate::Result<()> {
        if owners.is_empty() { Ok(()) } else { Err(crate::Error::unsupported("backend cannot retain resource owners")) }
    }
    fn graph_queue(&self) -> (recording::Queue, u32) {
        (recording::Queue::Primary, 0)
    }
    fn batch_synchronization(&self, _index: usize, _total: usize) -> GraphBatchSync {
        GraphBatchSync::default()
    }
    /// 设备和命令创建由后端提供；真实实现见 rg/vulkan.rs。
    fn command_handle(&mut self) -> crate::Result<u64>;
    /// 并行录制时第 `chunk` 块使用的命令；真实后端须保证不同块、以及块与主命令来自不同命令池。
    fn chunk_command_handle(&mut self, _chunk: usize) -> crate::Result<u64> {
        self.command_handle()
    }
    fn begin_command(&mut self, handle: u64) -> crate::Result<()>;
    fn end_command(&mut self, handle: vk::CommandBuffer) -> crate::Result<()>;
    /// CPU 后端可只记录投影；实际后端必须发出 Vulkan 屏障。
    fn transition(
        &mut self,
        _command: u64,
        _token: u64,
        _transition: &super::super::sync_plan::BoundTransition,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn flush_barriers(&mut self, _command: u64) -> crate::Result<()> {
        Ok(())
    }
    fn submit(
        &mut self,
        queue: vk::Queue,
        prepared: &[submit::PreparedSubmit],
    ) -> crate::Result<crate::sync::SyncPoint>;
    /// The parallel recording view plus one lease per chunk command, in the
    /// order of `commands` (each from [`Self::chunk_command_handle`] and
    /// begun). Leases must come from disjoint `&mut` borrows of the pools
    /// that allocated those commands, none shared with a command that stays
    /// recording during the parallel section (see `ChunkLease`).
    fn chunk_recording(&mut self, _commands: &[u64]) -> crate::Result<ChunkRecording<'_>> {
        Err(crate::Error::unsupported(
            "backend does not support parallel recording",
        ))
    }
    /// 失败路径：已分配或已开始、但不会提交的命令交还后端（它从未进入 GPU）。默认不做任何事。
    fn abandon_command(&mut self, _handle: u64) {}
    /// Record the device's pending ownership acquires for this batch's queue
    /// family at the start of the batch's first (just begun) command.
    /// Returns how many were recorded.
    fn drain_pending_acquires(&mut self, _command: u64) -> crate::Result<u32> {
        Ok(0)
    }
}

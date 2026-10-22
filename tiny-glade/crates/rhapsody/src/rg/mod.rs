internal_mod!(
    access_ledger,
    attachments,
    binding,
    cache,
    compiled,
    executor,
    frame,
    frame_imports,
    frame_storage,
    frame_throttle,
    graph,
    import_export,
    owner_storage,
    parallel,
    pass_api,
    pass_bindings,
    pass_builder,
    plan,
    plan_check,
    profiling,
    queue_schedule,
    receipt_storage,
    recorder,
    resource,
    resource_contracts,
    resource_info,
    resource_pool,
    resource_registry,
    result_cache,
    subgraph,
    sync_plan,
    temporal,
    transient_memory,
    transient_memory_check,
    util,
    vulkan,
    frame_loop,
    transaction,
    indirect_count,
);

// Curated render-graph surface; the submodules are implementation detail.
pub use crate::raw::transient_backing::TransientBackingCounters;
pub use attachments::{
    Attachment, AttachmentCache, PreparedAttachments, RenderingPolicy,
    contract::{
        AttachmentArray, AttachmentKey, DepthAccess, RenderPassAttachmentDesc, RenderPassDesc,
        RenderPassKey,
    },
};
pub use cache::CacheReason;
pub use executor::{
    Error as ExecutionError, ExecutionBackend, ExecutionChain, ExecutionStats, GraphExecution,
    PassContext,
};
pub use frame_loop::{FrameLoop, FrameOutcome, FrameStatus};
pub use graph::{
    BufferDesc, BuildError, BuiltGraph, Descriptor, GraphDeclaration, OriginSource,
    PassDeclaration, Pool, RenderGraph, ResourceBackend, ResourceDeclaration, Snapshot,
};
pub use indirect_count::{IndexedCountAdapter, IndexedCountRequest};
pub use pass_api::{AshCommandBackend, CommandBackend, RenderPassApi};
pub use pass_bindings::ManagedPassResources;
pub use pass_builder::PassBuilder;
pub use plan::{GraphDefinition, PlanContext, compile_definition};
pub use plan_check::check_plan;
pub use profiling::{GpuCommandTiming, GpuTimingConfig, GpuTimingStats};
pub use resource::{AccessRange, GraphId, ResourceContractError, ResourceHandle, ResourceOrigin};
pub use resource_contracts::validate_contracts;
pub use result_cache::{
    ContentDependency, DependencyId, ResultDependencies, ResultNodeId, ResultRequest,
};
pub use temporal::{NamedBuffer, NamedTexture, TemporalTexturePair};
pub use vulkan::{
    AccessTable, CompletedNamedResources, GpuResource, PreparedSessionFrame, PresentPolicy,
    PresentTarget, ResourceSpec, SessionFrameReceipt, SwapchainContents, VulkanGraphBackend,
    DisplayFrameId, VulkanGraphSession,
};

mod graph_caches;
pub mod token_map;
mod transient_resources;
pub use graph_caches::GraphCacheStats;

static PLAN_VALIDATION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Enable the independent plan verifiers (`plan_check`,
/// `transient_memory_check`) on every plan-cache miss in builds where they are
/// not already on. Debug builds and the `validation` feature always run them.
pub fn set_plan_validation(enabled: bool) {
    PLAN_VALIDATION.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

/// Whether plan compilation re-verifies its output with the independent checkers.
pub fn validation_enabled() -> bool {
    cfg!(any(debug_assertions, feature = "validation"))
        || PLAN_VALIDATION.load(std::sync::atomic::Ordering::Relaxed)
}

use crate::rg::resource_registry::{ResourcePool, ResourceRegistry};

mod declaration;
mod validate;
mod error;
mod backend;
mod snapshot;
mod instantiate;
mod declare;
mod build;
pub use declaration::{Descriptor, OriginSource, ResourceDeclaration, ExportableResource, PassDeclaration, GraphDeclaration, DeclarationShape, AccessDependency, RenderCallback, RetireCallback, MAX_PASSES};
pub use error::BuildError;
pub use backend::{BackendFailure, ResourceBackend, BackendEvent, EventBackend, Population, PopulationFailure};
pub use snapshot::{PhysicalMapping, SnapshotStatic, Snapshot, FrameInstance, FrameInstanceTables};
pub use instantiate::{compile_graph, instantiate_plan, reclaim_failed_resources, payload_projection};
pub(crate) use instantiate::instantiate_validated_plan;
pub use build::BuiltGraph;
pub type Registry = ResourceRegistry<u64, u64, ()>;
pub type Pool = ResourcePool<u64, u64>;

/// 一帧的渲染图声明：资源、pass、回调与导出。
pub struct RenderGraph {
    pub(crate) graph_id: super::resource::GraphId,
    // Committed-pass versions, indexed by logical resource. None preserves an
    // overflow error even if a later invalid explicit write resets its number.
    pub(crate) current_versions: Vec<Option<u32>>,
    pub(crate) declaration: GraphDeclaration,
    pub(crate) callbacks: Vec<RenderCallback>,
    retire_callbacks: Vec<RetireCallback>,
    rejection: Option<crate::Error>,
    /// Logical indices of imports declared outside `import_frame_resource`
    /// (named/temporal/swapchain); frame imports are found through the session's
    /// identity map, so only these need a physical-identity scan.
    pub(crate) foreign_imports: Vec<u32>,
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self {
            graph_id: super::resource::GraphId::next(),
            current_versions: Vec::new(),
            declaration: GraphDeclaration::default(),
            callbacks: Vec::new(),
            retire_callbacks: Vec::new(),
            rejection: None,
            foreign_imports: Vec::new(),
        }
    }
}

impl RenderGraph {
    pub fn new() -> Self {
        Self::default()
    }
    /// An empty graph whose vectors already have room for a frame of `shape`.
    /// Reservation is best effort: failure only means the vectors grow on demand.
    pub fn with_shape(shape: DeclarationShape) -> Self {
        let mut graph = Self::default();
        let (passes, resources, exports) =
            (shape.passes as usize, shape.resources as usize, shape.exports as usize);
        let _ = graph.declaration.passes.try_reserve_exact(passes);
        let _ = graph.callbacks.try_reserve_exact(passes);
        let _ = graph.declaration.resources.try_reserve_exact(resources);
        let _ = graph.current_versions.try_reserve_exact(resources);
        let _ = graph.declaration.exports.try_reserve_exact(exports);
        graph
    }

}

#[cfg(test)]
mod declaration_rejection_tests;

#[cfg(test)]
mod validation_allocation_tests;

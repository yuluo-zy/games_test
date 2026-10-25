//! 跨帧状态将图声明、资源池与时序资源放在一个拥有者中。
//! 此候选表示不宣称恢复原 RenderGraphState 的内存布局。
use super::{
    graph::{
        self, AccessDependency, BuiltGraph, CompiledGraph, Pool, RenderGraph, ResourceBackend,
    },
    temporal::TemporalRenderGraphState,
};
#[derive(Default)]
pub struct RenderGraphState {
    pub pool: Pool,
    pub temporal: TemporalRenderGraphState,
    pub frame: u64,
}
impl RenderGraphState {
    pub fn begin_frame(&mut self) -> RenderGraph {
        self.frame = self.frame.wrapping_add(1);
        RenderGraph::new()
    }
    pub fn clear(&mut self) {
        self.pool.clear();
        self.temporal.clear();
    }
    /// 导出发生在 population 之前；后续失败不会回滚成旧的 raw 历史。
    pub fn compile_frame(
        &mut self,
        mut graph: RenderGraph,
        backend: &mut impl ResourceBackend,
        dependency: &mut impl AccessDependency,
    ) -> Result<CompiledFrame, FrameCompileError> {
        self.temporal
            .export_temporal(&mut graph)
            .map_err(FrameCompileError::Temporal)?;
        let built = graph.build();
        let (declaration, callbacks, retire) = built.into_parts();
        let compiled = graph::compile_graph(declaration, &mut self.pool, backend, dependency)
            .map_err(FrameCompileError::Population)?;
        self.temporal
            .validate_compiled_exports(&compiled)
            .map_err(FrameCompileError::Temporal)?;
        Ok(CompiledFrame {
            compiled,
            callbacks,
            retire,
        })
    }
}

pub struct CompiledFrame {
    pub compiled: CompiledGraph,
    pub callbacks: Vec<graph::RenderCallback>,
    pub retire: Vec<graph::RetireCallback>,
}

#[derive(Debug)]
pub enum FrameCompileError {
    Temporal(super::temporal::TemporalError),
    Population(graph::PopulationFailure),
}

/// 原 compile 前段的 Option 所有权合同，不恢复标准容器或 panic ABI。
pub struct PersistentCompileState {
    pub graph: Option<RenderGraph>,
    pub temporal: Option<TemporalRenderGraphState>,
}
pub struct DetachedCompileState {
    pub graph: RenderGraph,
    pub temporal: TemporalRenderGraphState,
}
pub enum TakeFailure {
    GraphMissing,
    TemporalMissing { detached_graph: Box<RenderGraph> },
}
impl PersistentCompileState {
    /// 原 0x140166510..0x140166617：先取 graph，再检查 temporal；失败不回滚。
    pub fn take_for_compile(&mut self) -> Result<DetachedCompileState, TakeFailure> {
        let graph = self.graph.take().ok_or(TakeFailure::GraphMissing)?;
        let Some(temporal) = self.temporal.take() else {
            return Err(TakeFailure::TemporalMissing {
                detached_graph: Box::new(graph),
            });
        };
        Ok(DetachedCompileState { graph, temporal })
    }
    /// 原 0x140166d2a..0x140166e19：先安装导出状态，再 drop 旧状态。
    pub fn restore_temporal(&mut self, exported: TemporalRenderGraphState) {
        let previous = self.temporal.replace(exported);
        drop(previous);
    }
    /// 外部物理拥有者适配：仍先安装新值，再释放旧状态的缓存引用。
    pub fn restore_temporal_owned(
        &mut self,
        exported: TemporalRenderGraphState,
        resources: &mut impl super::temporal::TemporalResourceOwner,
    ) {
        if let Some(mut previous) = self.temporal.replace(exported) {
            previous.release_superseded(self.temporal.as_ref().unwrap(), resources);
        }
    }
}

impl DetachedCompileState {
    pub fn export_and_restore(
        mut self,
        owner: &mut PersistentCompileState,
    ) -> Result<BuiltGraph, super::temporal::TemporalError> {
        self.temporal.export_temporal(&mut self.graph)?;
        owner.restore_temporal(self.temporal);
        Ok(self.graph.build())
    }
    pub fn export_and_restore_owned(
        mut self,
        owner: &mut PersistentCompileState,
        resources: &mut impl super::temporal::TemporalResourceOwner,
    ) -> Result<BuiltGraph, super::temporal::TemporalError> {
        self.temporal.export_temporal(&mut self.graph)?;
        owner.restore_temporal_owned(self.temporal, resources);
        Ok(self.graph.build())
    }
}

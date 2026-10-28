//! Turning a declaration into a built graph.
use super::RenderGraph;
use super::declaration::GraphDeclaration;
use super::declaration::RenderCallback;
use super::declaration::RetireCallback;

pub struct BuiltGraph {
    graph_id: super::super::resource::GraphId,
    pub declaration: GraphDeclaration,
    pub callbacks: Vec<RenderCallback>,
    retire_callbacks: Vec<RetireCallback>,
    rejection: Option<crate::Error>,
}

impl BuiltGraph {
    pub(crate) fn into_graph(self) -> RenderGraph {
        // BuiltGraph.declaration is public and may have been edited. Rebuild
        // once at this boundary rather than trusting a stale private index.
        let mut current_versions = vec![Some(0); self.declaration.resources.len()];
        for pass in &self.declaration.passes {
            RenderGraph::advance_versions(&mut current_versions, &pass.accesses);
        }
        RenderGraph {
            graph_id: self.graph_id,
            current_versions,
            declaration: self.declaration,
            callbacks: self.callbacks,
            retire_callbacks: self.retire_callbacks,
            rejection: self.rejection,
            foreign_imports: Vec::new(),
        }
    }

    pub fn graph_id(&self) -> super::super::resource::GraphId {
        self.graph_id
    }
    pub(crate) fn take_rejection(&mut self) -> Option<crate::Error> {
        self.rejection.take()
    }
    /// Transfer declaration, callbacks, retirement and its first rejection.
    /// Execution owners must reject the last item before validating/planning
    /// or allocating resources. Cancellation still owns retirement callbacks.
    pub fn into_parts(self) -> (
        GraphDeclaration,
        Vec<RenderCallback>,
        Vec<RetireCallback>,
        Option<crate::Error>,
    ) {
        (self.declaration, self.callbacks, self.retire_callbacks, self.rejection)
    }
}

impl RenderGraph {
    pub fn build(self) -> BuiltGraph {
        BuiltGraph {
            graph_id: self.graph_id,
            declaration: self.declaration,
            callbacks: self.callbacks,
            retire_callbacks: self.retire_callbacks,
            rejection: self.rejection,
        }
    }
}

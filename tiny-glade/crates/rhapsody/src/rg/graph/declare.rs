//! Graph construction API: resources, passes, exports and planned declarations.
use crate::rg::resource_info::AccessDeclaration;
use crate::rg::resource_info::ExportDeclaration;
use super::RenderGraph;
use super::declaration::GraphDeclaration;
use super::declaration::MAX_PASSES;
use super::declaration::OriginSource;
use super::declaration::PassDeclaration;
use super::declaration::RenderCallback;
use super::declaration::ResourceDeclaration;
use super::error::BuildError;

impl RenderGraph {
    pub fn create_raw_resource(
        &mut self,
        name: impl Into<crate::resource::ResourceName>,
        origin: super::super::resource::ResourceOrigin,
    ) -> Result<super::super::resource::ResourceHandle, BuildError> {
        // 复用记录采用 u16 物理索引，最大值保留给无映射状态。
        if self.declaration.resources.len() >= usize::from(u16::MAX) {
            return Err(BuildError::ResourceLimit);
        }
        let handle = super::super::resource::ResourceHandle::scoped(
            self.graph_id,
            self.declaration.resources.len() as u32,
        );
        if origin.source != OriginSource::Create {
            self.foreign_imports.push(handle.raw());
        }
        self.current_versions.push(Some(0));
        self.declaration.resources.push(ResourceDeclaration {
            sharing: crate::tracking::ResourceSharing::Exclusive,
            name: name.into(),
            descriptor: origin.descriptor,
            source: origin.source,
        });
        Ok(handle)
    }

    /// Reject this declaration before planning, allocation or render callbacks.
    /// The first producer failure owns its original diagnostic and category.
    pub fn reject(&mut self, error: crate::Error) {
        if self.rejection.is_none() {
            self.rejection = Some(error);
        }
    }

    pub(crate) fn check_handle(
        &self,
        handle: super::super::resource::ResourceHandle,
    ) -> Result<(), BuildError> {
        if (handle.raw() as usize) >= self.declaration.resources.len() {
            return Err(BuildError::InvalidHandle(handle.raw()));
        }
        if handle.graph_id() != self.graph_id {
            return Err(BuildError::WrongGraph {
                expected: self.graph_id,
                actual: handle.graph_id(),
            });
        }
        Ok(())
    }

    /// Bind a stored logical index to this graph after checking its declaration.
    pub fn resource_handle(
        &self,
        index: u32,
    ) -> Result<super::super::resource::ResourceHandle, BuildError> {
        let handle = super::super::resource::ResourceHandle::scoped(self.graph_id, index);
        self.check_handle(handle)?;
        Ok(handle)
    }

    pub(crate) fn advance_versions(versions: &mut [Option<u32>], accesses: &[AccessDeclaration]) {
        use super::super::resource::AccessVersion;
        for access in accesses {
            // Malformed replay declarations are rejected by compilation. Do
            // not let an invalid index alter any valid resource's version.
            let Some(slot) = versions.get_mut(access.resource_index as usize) else { continue };
            let Some(version) = *slot else { continue };
            *slot = match access.version {
                AccessVersion::Write { current, .. } => Some(current),
                AccessVersion::Automatic if crate::native::access_table::is_write(access.access_code) => version.checked_add(1),
                _ => Some(version),
            };
        }
    }

    pub fn graph_id(&self) -> super::super::resource::GraphId {
        self.graph_id
    }

    pub fn declaration(&self) -> &GraphDeclaration {
        &self.declaration
    }

    /// 不检查 pass 上限，保持既有调用面；超限会在 `PassBuilder::add_resource_access` 返回
    /// [`BuildError::PassLimit`]，需要在拿到 builder 前就得到错误时用 [`Self::try_add_pass`]。
    pub fn add_pass(
        &mut self,
        name: impl Into<crate::resource::ResourceName>,
    ) -> super::super::pass_builder::PassBuilder<'_> {
        let graph_id = self.graph_id;
        super::super::pass_builder::PassBuilder {
            graph: self,
            declaration: PassDeclaration {
                queue_class: crate::device::QueueClass::Graphics,
                graph_id: Some(graph_id),
                name: name.into(),
                accesses: vec![],
                side_effect: false,
            },
        }
    }

    /// 带 pass 上限检查的 `add_pass`：已有 `MAX_PASSES` 个 pass 时返回 [`BuildError::PassLimit`]。
    pub fn try_add_pass(
        &mut self,
        name: impl Into<crate::resource::ResourceName>,
    ) -> Result<super::super::pass_builder::PassBuilder<'_>, BuildError> {
        self.check_pass_capacity()?;
        Ok(self.add_pass(name))
    }

    pub(crate) fn check_pass_capacity(&self) -> Result<(), BuildError> {
        if self.declaration.passes.len() >= MAX_PASSES {
            Err(BuildError::PassLimit)
        } else {
            Ok(())
        }
    }

    pub fn export(
        &mut self,
        handle: super::super::resource::ResourceHandle,
        access_code: u8,
    ) -> Result<(), BuildError> {
        self.export_range(handle, access_code, super::super::resource::AccessRange::Whole)
    }

    /// Publish the boundary state of exactly this initialized range.
    pub fn export_range(
        &mut self,
        handle: super::super::resource::ResourceHandle,
        access_code: u8,
        range: super::super::resource::AccessRange,
    ) -> Result<(), BuildError> {
        self.check_handle(handle)?;
        let resource = &self.declaration.resources[handle.raw() as usize];
        range
            .validate(resource.descriptor)
            .map_err(|error| super::super::resource_contracts::range_error(handle.raw(), error))?;
        self.declaration.exports.push(ExportDeclaration {
            tag: resource.descriptor.kind_code(),
            resource_index: handle.raw(),
            access_code,
            range,
        });
        Ok(())
    }

    pub fn batch_per_pass(&mut self, enabled: bool) {
        self.declaration.batch_per_pass = enabled;
    }

    /// Retained-graph replay: declare the planned `Create` resource at the next
    /// logical index. Imports and named resources are declared by the session
    /// APIs in the same order, so every index matches the plan.
    pub fn declare_planned_create(
        &mut self,
        definition: &super::super::plan::ResourceDefinition,
    ) -> Result<super::super::resource::ResourceHandle, BuildError> {
        if definition.origin != super::super::plan::ResourceOriginKind::Create {
            return Err(BuildError::InvalidHandle(self.declaration.resources.len() as u32));
        }
        let handle = self.create_raw_resource(
            definition.name.clone(),
            super::super::resource::ResourceOrigin {
                descriptor: definition.descriptor,
                source: OriginSource::Create,
            },
        )?;
        self.declaration.resources[handle.raw() as usize].sharing = definition.sharing;
        Ok(handle)
    }

    /// Retained-graph replay: take the plan's pass structure (and its recorders)
    /// instead of declaring each pass. Every resource must already have been
    /// replayed in plan order with the planned name, descriptor, sharing and
    /// origin kind, so the result equals a freshly declared frame; a mismatch
    /// leaves the graph untouched and tells the host to rebuild.
    pub fn adopt_planned_passes(
        &mut self,
        definition: &super::super::plan::GraphDefinition,
        callbacks: Vec<RenderCallback>,
    ) -> Result<(), BuildError> {
        use super::super::plan::ResourceOriginKind;
        let resources = &self.declaration.resources;
        let matches = self.declaration.passes.is_empty()
            && self.callbacks.is_empty()
            && callbacks.len() == definition.passes.len()
            && resources.len() == definition.resources.len()
            && resources.iter().zip(&definition.resources).all(|(declared, planned)| {
                declared.name == planned.name
                    && declared.descriptor == planned.descriptor
                    && declared.sharing == planned.sharing
                    && match declared.source {
                        OriginSource::Create => planned.origin == ResourceOriginKind::Create,
                        OriginSource::ImportOwned { .. } => {
                            planned.origin == ResourceOriginKind::ImportOwned
                        }
                        OriginSource::ImportUnowned { .. } => {
                            planned.origin == ResourceOriginKind::ImportUnowned
                        }
                    }
            });
        if !matches {
            return Err(BuildError::InvalidHandle(u32::MAX));
        }
        self.declaration.subgraphs = definition.subgraphs.clone();
        self.declaration.passes = definition.passes.clone();
        for pass in &mut self.declaration.passes {
            pass.graph_id = Some(self.graph_id);
            Self::advance_versions(&mut self.current_versions, &pass.accesses);
        }
        self.declaration.batch_per_pass = definition.batch_per_pass;
        for &index in &definition.gated_resources {
            if let Err(at) = self.declaration.gated_resources.binary_search(&index) {
                self.declaration.gated_resources.insert(at, index);
            }
        }
        self.callbacks = callbacks;
        Ok(())
    }

    /// Begin a new batch immediately before the first pass that uses
    /// `resource` (a no-op when it is already used by the first pass). Passes
    /// ahead of it then do not share a batch with the resource's first use, so
    /// an external wait attached to that batch (the swapchain acquire) never
    /// delays them. The boundary is part of the plan identity.
    pub fn gate_first_use(
        &mut self,
        resource: super::super::resource::ResourceHandle,
    ) -> crate::Result<()> {
        self.check_handle(resource)?;
        let index = resource.raw();
        if let Err(at) = self.declaration.gated_resources.binary_search(&index) {
            self.declaration
                .gated_resources
                .try_reserve(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            self.declaration.gated_resources.insert(at, index);
        }
        Ok(())
    }

    pub fn on_retire(&mut self, callback: impl FnOnce() + Send + 'static) {
        self.retire_callbacks.push(Box::new(callback));
    }

    pub(crate) fn reserve_retirement_callback(&mut self) -> crate::Result<()> {
        self.retire_callbacks
            .try_reserve(1)
            .map_err(|_| crate::Error::OutOfHostMemory)
    }
}

//! Per-frame resource imports.
use crate::resource::buffer;
use crate::resource::texture;
use crate::rg::graph;
use std::sync::Arc;
use super::VulkanGraphSession;
use super::super::resource_spec::GpuResource;
use super::super::resource_spec::validate_import_descriptor;

impl VulkanGraphSession {
    pub fn configure_frame_imports(
        &mut self,
        config: crate::rg::frame_imports::FrameImportConfig,
    ) -> crate::Result<()> {
        self.collect_frame_imports();
        if self.frame_import_count() != 0 {
            return Err(crate::Error::invalid_usage(
                "frame imports are still owned by live graphs",
            ));
        }
        self.frame_imports = Some(crate::rg::frame_imports::FrameImports::new(
            config,
            self.backend.device.memory.stats.clone(),
        )?);
        self.frame_import_config = config;
        Ok(())
    }

    /// Import an actual buffer for this graph without exposing a host token.
    /// Non-exported declaration bindings are released on cancellation or after recording;
    /// submitted commands independently own the physical resource until ready.
    pub fn import_frame_buffer(
        &mut self,
        graph: &mut graph::RenderGraph,
        buffer: Arc<buffer::Buffer>,
        initial_access: u8,
    ) -> crate::Result<crate::rg::resource::ResourceHandle> {
        self.import_frame_resource(graph, GpuResource::Buffer(buffer), initial_access, false)
    }

    /// Frame-scoped texture import with the same cancellation and retirement
    /// ownership as `import_frame_buffer`. Existing physical declarations are reused.
    pub fn import_frame_texture(
        &mut self,
        graph: &mut graph::RenderGraph,
        texture: Arc<texture::Texture>,
        initial_access: u8,
    ) -> crate::Result<crate::rg::resource::ResourceHandle> {
        self.import_frame_resource(graph, GpuResource::Image(texture), initial_access, false)
    }

    /// B compile-present transfers its main output into the second graph with
    /// access 0x22, while the acquired swapchain image remains borrowed.
    pub fn import_frame_texture_owned(
        &mut self,
        graph: &mut graph::RenderGraph,
        texture: Arc<texture::Texture>,
        initial_access: u8,
    ) -> crate::Result<crate::rg::resource::ResourceHandle> {
        self.import_frame_resource(graph, GpuResource::Image(texture), initial_access, true)
    }

    /// `Some(result)` when declaration `index` already is this physical resource.
    fn existing_import(
        &self,
        graph: &graph::RenderGraph,
        index: usize,
        resource: &GpuResource,
        owned: bool,
        sharing: crate::tracking::ResourceSharing,
        initial_access: u8,
    ) -> Option<crate::Result<crate::rg::resource::ResourceHandle>> {
        let declaration = graph.declaration.resources.get(index)?;
        let (token, previous_access) = match declaration.source {
            graph::OriginSource::ImportOwned { token, initial_access }
            | graph::OriginSource::ImportUnowned { token, initial_access } => (token, initial_access),
            graph::OriginSource::Create => return None,
        };
        let matches = match (self.backend.resources.get(&token), resource) {
            (Some(GpuResource::Buffer(existing)), GpuResource::Buffer(buffer)) => {
                Arc::ptr_eq(existing, buffer)
            }
            (Some(GpuResource::Image(existing)), GpuResource::Image(image)) => {
                Arc::ptr_eq(existing, image)
            }
            _ => false,
        };
        if !matches {
            return None;
        }
        Some((|| {
            if previous_access != initial_access {
                return Err(crate::Error::invalid_usage(format!(
                    "frame import initial access mismatch: resource={} index={index} existing={previous_access:#04x} requested={initial_access:#04x}", declaration.name
                )));
            }
            if owned != matches!(declaration.source, graph::OriginSource::ImportOwned { .. }) {
                return Err(crate::Error::invalid_usage(
                    "frame import ownership differs from its existing declaration",
                ));
            }
            validate_import_descriptor(declaration.descriptor, resource.descriptor())?;
            if declaration.sharing != sharing {
                return Err(crate::Error::invalid_usage("frame import sharing mismatch"));
            }
            graph.resource_handle(index as u32).map_err(Into::into)
        })())
    }

    pub(in crate::rg::vulkan) fn import_frame_resource(
        &mut self,
        graph: &mut graph::RenderGraph,
        resource: GpuResource,
        initial_access: u8,
        owned: bool,
    ) -> crate::Result<crate::rg::resource::ResourceHandle> {
        use crate::tracking::identity::PhysicalIdentity;
        let (identity, sharing, name) = match &resource {
            GpuResource::Buffer(buffer) => (
                PhysicalIdentity::Buffer(Arc::as_ptr(buffer) as usize),
                buffer.sharing(),
                "frame-buffer",
            ),
            GpuResource::Image(image) => (
                PhysicalIdentity::Image(Arc::as_ptr(image) as usize),
                image.sharing(),
                "frame-image",
            ),
        };
        let descriptor = resource.descriptor();
        resource.check_device(&self.backend.device)?;
        if usize::from(initial_access) >= crate::native::access_table::ACCESS_CODE_COUNT {
            return Err(crate::Error::invalid_usage("invalid frame import access"));
        }
        if self.is_poisoned() {
            return Err(crate::Error::DeviceLost);
        }
        // Fast path: this graph already imported this physical resource through
        // a frame import (one hash lookup instead of scanning every declaration).
        if let Some(existing) = self
            .frame_imports
            .as_ref()
            .and_then(|imports| imports.find(graph.graph_id(), identity))
            && let Some(found) =
                self.existing_import(graph, existing.raw() as usize, &resource, owned, sharing, initial_access)
        {
            return found;
        }
        // Other typed hosts may already have imported the same physical resource.
        // A second logical declaration would split its dependency history.
        for &index in &graph.foreign_imports {
            if let Some(found) =
                self.existing_import(graph, index as usize, &resource, owned, sharing, initial_access)
            {
                return found;
            }
        }
        if self.frame_imports.is_none() {
            self.frame_imports = Some(crate::rg::frame_imports::FrameImports::new(
                self.frame_import_config,
                self.backend.device.memory.stats.clone(),
            )?);
        }
        if self
            .frame_imports
            .as_ref()
            .expect("frame imports initialized")
            .find(graph.graph_id(), identity)
            .is_some()
        {
            return Err(crate::Error::invalid_usage(
                "frame import was invalidated before graph execution",
            ));
        }
        // Collection walks every live import, so run it only when the registry
        // is full (a per-import collect made a frame's imports quadratic).
        let reserved = self
            .frame_imports
            .as_mut()
            .expect("frame imports initialized")
            .reserve();
        let lease = match reserved {
            Ok(lease) => lease,
            Err(crate::Error::Exhausted(_)) => {
                // Only unrecorded graphs consume declaration capacity. Reclaim
                // abandoned graphs without polling or waiting for GPU work.
                self.collect_frame_imports();
                self.frame_imports
                    .as_mut()
                    .expect("frame imports initialized")
                    .reserve()?
            }
            Err(error) => return Err(error),
        };
        let imports = self
            .frame_imports
            .as_mut()
            .expect("frame imports initialized");
        graph.reserve_retirement_callback()?;
        let marker = lease.marker();
        let token = self.backend.bind_import(resource)?;
        let imported = if owned {
            graph.import_owned_with_sharing(
                format!("{name}-{}", graph.declaration.resources.len()),
                descriptor,
                token,
                initial_access,
                sharing,
            )
        } else {
            graph.import_unowned_with_sharing(
                format!("{name}-{}", graph.declaration.resources.len()),
                descriptor,
                token,
                initial_access,
                sharing,
            )
        };
        let handle = match imported {
            Ok(handle) => {
                // Found through `frame_imports` from now on, not by the foreign scan.
                graph.foreign_imports.pop();
                handle
            }
            Err(error) => {
                self.backend.resources.remove(&token);
                self.backend.import_families.remove(&token);
                return Err(error.into());
            }
        };
        if let Err(error) = imports.insert(identity, token, handle, lease) {
            self.backend.resources.remove(&token);
            self.backend.import_families.remove(&token);
            return Err(error);
        }
        graph.on_retire(move || drop(marker));
        Ok(handle)
    }

    pub fn frame_import_count(&self) -> usize {
        self.frame_imports
            .as_ref()
            .map_or(0, |imports| imports.len())
    }

    pub(super) fn collect_frame_imports(&mut self) {
        self.collect_recorded_imports(None);
    }

    pub(super) fn collect_recorded_imports(&mut self, recorded: Option<(crate::rg::resource::GraphId, &[u16])>) {
        let Some(imports) = &mut self.frame_imports else {
            return;
        };
        let backend = &mut self.backend;
        let mut removed_any = false;
        imports.collect(recorded, |token| {
            removed_any = true;
            backend.resources.remove(&token);
            backend.import_families.remove(&token);
            backend.history_validity.remove(&token);
        });
        if removed_any {
            // Declaration tokens expired; the bounded idle CPU projections
            // retain only physical owners and cannot revive old graph handles.
            let mut projection = backend.caches.projection();
            projection.snapshot.take();
            projection.pool.park_idle();
        }
    }

    /// 释放单个导入资源（`bind_import*` 登记的 token），不需要整设备空闲，也不清别的缓存。
    ///
    /// 只拒绝“仍被在飞帧引用”的 token：那种资源 GPU 可能还在用，必须等对应帧 `wait_frame` 之后。
    /// 返回是否真的移除了东西（未登记的 token 返回 `Ok(false)`）。
    /// 以前只能靠 `collect_after_completion`（要求在飞为空 + 设备级 `wait_idle`）才会放掉导入资源，
    /// 每帧新建并导入的资源因此会一直堆到排空点。
    pub fn release_import(&mut self, token: u64) -> Result<bool, crate::Error> {
        if self
            .named
            .as_ref()
            .is_some_and(|named| named.state.owns_token(token))
        {
            return Err(crate::Error::invalid_usage(
                "named imports remain owned by their session",
            ));
        }
        if self.result_cache.owns_token(token) {
            return Err(crate::Error::invalid_usage(
                "result-cache imports remain owned until cache eviction",
            ));
        }
        if self.result_import_in_flight(token) {
            return Err(crate::Error::invalid_usage(format!(
                "token {token} 仍被在飞帧引用，不能释放"
            )));
        }
        self.backend.clear_pass_resources();
        let removed = self.backend.resources.remove(&token).is_some();
        self.backend.import_families.remove(&token);
        self.backend.collect_completed_managed()?;
        Ok(removed)
    }

    pub(crate) fn result_import_in_flight(&self, token: u64) -> bool {
        self.pending.iter().any(|frame| {
            frame
                .execution
                .compiled
                .snapshot
                .physical_mapping
                .iter()
                .any(|mapping| mapping.token == Some(token))
        })
    }
}

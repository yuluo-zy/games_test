//! Named and temporal resources, pool eviction and cache statistics.
use crate::resource::buffer;
use crate::resource::texture;
use crate::rg::graph;
use std::sync::Arc;
use super::VulkanGraphSession;
use super::super::resource_spec::GpuResource;
use super::super::resource_spec::ResourceSpec;

impl VulkanGraphSession {
    /// A persistent named slot; contents are valid only after complete publication.
    pub fn named_texture(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
    ) -> crate::Result<crate::rg::temporal::NamedTexture> {
        self.named_texture_with_sharing(
            graph,
            name,
            desc,
            crate::tracking::ResourceSharing::Exclusive,
        )
    }

    pub fn named_texture_with_sharing(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
        sharing: crate::tracking::ResourceSharing,
    ) -> crate::Result<crate::rg::temporal::NamedTexture> {
        let lookup = self.request_named(
            graph,
            &format!("slot/{}:{name}", name.len()),
            ResourceSpec::Image(desc).with_sharing(sharing),
            false,
        )?;
        Ok(crate::rg::temporal::NamedTexture {
            texture: graph.texture_id(lookup.handle)?,
            history_valid: lookup.reused,
        })
    }

    pub fn named_buffer(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
    ) -> crate::Result<crate::rg::temporal::NamedBuffer> {
        self.named_buffer_with_sharing(
            graph,
            name,
            desc,
            crate::tracking::ResourceSharing::Exclusive,
        )
    }

    /// Like [`Self::named_buffer`], but a newly created (or resized) buffer is
    /// zero-filled, so reading it before the first write is defined.
    /// `history_valid` still reports whether earlier frames wrote it.
    pub fn named_buffer_zeroed(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
    ) -> crate::Result<crate::rg::temporal::NamedBuffer> {
        self.named_buffer_with_options(
            graph,
            name,
            desc,
            crate::tracking::ResourceSharing::Exclusive,
            true,
        )
    }

    pub fn named_buffer_with_sharing(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
        sharing: crate::tracking::ResourceSharing,
    ) -> crate::Result<crate::rg::temporal::NamedBuffer> {
        self.named_buffer_with_options(graph, name, desc, sharing, false)
    }

    /// A session-owned buffer that persists across frames. New contents are
    /// undefined (a read before the first write is rejected) unless
    /// `zero_initialized` requests a zero-filled allocation.
    pub fn named_buffer_with_options(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: buffer::BufferDesc,
        sharing: crate::tracking::ResourceSharing,
        zero_initialized: bool,
    ) -> crate::Result<crate::rg::temporal::NamedBuffer> {
        let lookup = self.request_named(
            graph,
            &format!("slot/{}:{name}", name.len()),
            ResourceSpec::Buffer(desc).with_sharing(sharing),
            zero_initialized,
        )?;
        Ok(crate::rg::temporal::NamedBuffer {
            buffer: graph.buffer_id(lookup.handle)?,
            history_valid: lookup.reused,
        })
    }

    /// Request the session-owned current/previous texture pair. No physical token escapes.
    pub fn temporal_texture(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
    ) -> crate::Result<crate::rg::temporal::TemporalTexturePair> {
        self.temporal_texture_with_sharing(
            graph,
            name,
            desc,
            crate::tracking::ResourceSharing::Exclusive,
        )
    }

    pub fn temporal_texture_with_sharing(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        desc: texture::TextureDesc,
        sharing: crate::tracking::ResourceSharing,
    ) -> crate::Result<crate::rg::temporal::TemporalTexturePair> {
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.validate_graph(graph)?;
        let key = format!("pair/{}:{name}", name.len());
        named.ensure_capacity(&format!("{key}:0"), 2)?;
        let names = [format!("{key}:0"), format!("{key}:1")];
        let previous = names.each_ref().map(|name| {
            named
                .resources
                .register(name, ResourceSpec::Image(desc).with_sharing(sharing))
        });
        let pair = match named.state.get_or_create_owned_pair(
            graph,
            &key,
            ResourceSpec::Image(desc).descriptor(),
            &mut named.resources,
        ) {
            Ok(pair) => pair,
            Err(error) => {
                for (name, previous) in names.iter().zip(previous) {
                    named.resources.restore_specification(name, previous);
                }
                return Err(error.into());
            }
        };
        for (token, valid) in named.state.requested_validity() {
            self.backend.history_validity.insert(token, valid);
        }
        Ok(crate::rg::temporal::TemporalTexturePair::from_lookups(
            graph, pair,
        )?)
    }

    fn request_named(
        &mut self,
        graph: &mut graph::RenderGraph,
        key: &str,
        spec: ResourceSpec,
        zero_initialized: bool,
    ) -> crate::Result<crate::rg::temporal::TemporalLookup> {
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.validate_graph(graph)?;
        named.ensure_capacity(key, 1)?;
        let previous = named.resources.register(key, spec);
        named.resources.set_zero_initialized(key, zero_initialized);
        let lookup = match named.state.get_or_create_owned(
            graph,
            key,
            spec.descriptor(),
            &mut named.resources,
        ) {
            Ok(lookup) => lookup,
            Err(error) => {
                named.resources.restore_specification(key, previous);
                return Err(error.into());
            }
        };
        self.backend
            .history_validity
            .insert(lookup.resource.token, lookup.reused);
        Ok(lookup)
    }

    /// Invalidate history without waiting for unrelated GPU work or destroying allocations.
    pub fn reset_temporal(&mut self, name: &str) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame owns temporal references",
            ));
        }
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.state.reset(&format!("slot/{}:{name}", name.len()))?;
        named.state.reset(&format!("pair/{}:{name}", name.len()))?;
        Ok(())
    }

    /// Original OOM recovery clears only reusable idle allocations. Pending
    /// frames retain their own physical owners until actual GPU completion.
    pub fn clear_idle_resource_pool(&mut self) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame still owns resource-pool allocations",
            ));
        }
        let (textures, buffers) = self.pool.drain();
        for token in textures
            .into_iter()
            .map(|resource| resource.handle)
            .chain(buffers.into_iter().map(|resource| resource.handle))
        {
            self.backend.discard_pooled(token);
        }
        Ok(())
    }

    /// B render 0x1404e8... replaces the two temporal tables on OOM recovery.
    /// clear() retires their ownership; collect_named_retired keeps in-flight
    /// frames alive instead of treating table removal as GPU completion.
    pub fn reset_all_temporals(&mut self) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame still owns temporal references",
            ));
        }
        let named = self
            .named
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("named frame is already executing"))?;
        named.state.clear();
        // 将状态表的退役令牌移交给物理 owner；在飞帧仍由后端按 pending 保活。
        named.state.release_retired(&mut named.resources);
        self.collect_named_retired();
        Ok(())
    }

    pub fn named_resource_count(&self) -> usize {
        self.named.as_ref().map_or(0, |named| named.resources.len())
    }

    /// Inspect a named owner through its graph-scoped typed ID, never a physical identifier.
    pub fn named_texture_resource(
        &self,
        graph: &graph::RenderGraph,
        id: crate::rg::resource::TextureId,
    ) -> crate::Result<Arc<texture::Texture>> {
        graph.check_handle(id.raw())?;
        let (graph::OriginSource::ImportOwned { token, .. }
        | graph::OriginSource::ImportUnowned { token, .. }) =
            graph.declaration.resources[id.raw().raw() as usize].source
        else {
            return Err(crate::Error::invalid_usage("texture is not a named import"));
        };
        match self
            .named
            .as_ref()
            .and_then(|named| named.resources.resource(token))
        {
            Some(GpuResource::Image(texture)) => Ok(texture),
            _ => Err(crate::Error::invalid_usage(
                "texture is not owned by this session",
            )),
        }
    }

    pub(super) fn collect_named_retired(&mut self) {
        let Some(named) = &mut self.named else {
            return;
        };
        if named.resources.retired.is_empty() {
            return;
        }
        let mut removed_any = false;
        named.resources.retired.retain(|token| {
            if self.pending.iter().any(|frame| {
                frame
                    .execution
                    .compiled
                    .snapshot
                    .physical_mapping
                    .iter()
                    .any(|mapping| mapping.token == Some(*token))
            }) {
                return true;
            }
            removed_any = true;
            self.backend.resources.remove(token);
            self.backend.import_families.remove(token);
            self.backend.history_validity.remove(token);
            false
        });
        // Invalidating the pass-resource projection once is enough; doing it
        // per token also forced a full rebuild for each retired resource.
        if removed_any {
            self.backend.clear_pass_resources();
        }
    }

    /// Drop pool entries whose parked resource the device's eviction already
    /// released (see `rg::graph_caches`).
    pub(super) fn forget_evicted_pool_entries(&mut self) {
        let evicted = self.backend.caches.take_evicted();
        if !evicted.is_empty() {
            // Tokens come from one process-wide counter shared by textures
            // and buffers.
            self.pool.forget_handles(&evicted, &evicted);
        }
    }

    pub(super) fn evict_idle_resources_under_pressure(&mut self) -> crate::Result<()> {
        let _ = self.backend.device.poll()?;
        if self.backend.device.memory.stats.budget_action()
            == crate::accounting::BudgetAction::EvictCache
        {
            if let Some(storage) = &mut self.backend.receipt_storage {
                storage.trim_idle(0);
            }
            let mut projection = self.backend.caches.projection();
            projection.snapshot.take();
            projection.pool.trim_idle();
            drop(projection);
            let (textures, buffers) = self.pool.drain();
            for token in textures
                .into_iter()
                .map(|r| r.handle)
                .chain(buffers.into_iter().map(|r| r.handle))
            {
                self.backend.discard_pooled(token);
            }
            self.backend.device.try_collect_garbage()?;
        }
        Ok(())
    }

    /// Idle pooled resources and retired alias sets that memory pressure may
    /// release.
    pub fn cache_stats(&self) -> crate::rg::graph_caches::GraphCacheStats {
        self.backend.caches.stats()
    }

    /// Physical resources the session owns, including idle pooled ones
    /// parked in the evictable cache.
    pub fn resource_count(&self) -> usize {
        self.backend.resources.len() + self.backend.caches.parked_count()
    }

    pub fn command_count(&self) -> usize {
        self.backend.commands.len()
    }
}

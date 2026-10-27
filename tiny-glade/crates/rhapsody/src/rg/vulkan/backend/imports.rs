//! Resource declaration and import binding.
use crate::rg::graph;
use crate::rg::resource::ResourceOrigin;
use std::sync::Arc;
use super::ActiveAlias;
use super::VulkanGraphBackend;
use super::super::GENERATED_TOKEN_LIMIT;
use super::super::resource_spec::GpuResource;
use super::super::resource_spec::ResourceSpec;
use super::super::try_allocate_token;

impl graph::ResourceBackend for VulkanGraphBackend {
    fn frame_storage_pool(&self) -> Option<&crate::rg::frame_storage::FrameStoragePool> {
        Some(&self.frame_storage)
    }

    fn history_validity(&self, token: u64) -> Option<bool> {
        self.history_validity.get(&token).copied()
    }

    fn device_generation(&self) -> Option<crate::sync::DeviceGeneration> {
        Some(self.device.generation())
    }
    fn prepare_transient_resources(
        &mut self,
        plan: &crate::rg::plan::CompiledPlan,
        graph: &graph::GraphDeclaration,
    ) -> crate::Result<()> {
        self.prepare_queue_plan(plan, graph)?;
        self.validate_batch_sync(plan.batches.len())?;
        // Drop any unused candidates from an earlier failed population before
        // allocating this frame. Already-published tokens belong to that frame.
        if self.transient_aliasing {
            self.return_unconsumed_aliases();
            self.active_alias = None;
            let (lent, evicted) = self.caches.alias().lend(
                &self.device,
                &self.specifications,
                plan,
                graph,
                &self.transient_counters,
            )?;
            // The cache guard ended at the statement boundary. Native owners
            // removed by LRU eviction are retired without holding its mutex.
            drop(evicted);
            self.active_alias = lent.as_ref().map(|lent| ActiveAlias {
                plan: lent.plan.clone(),
                edges: lent.edges.clone(),
            });
            self.lent_aliases = lent;
        } else {
            self.prepared_transients
                .prepare_unaliased(&plan.dependency_edges, &self.device.memory.stats)?;
        }
        Ok(())
    }
    fn has_prepared_resource(&self, name: &str) -> bool {
        self.lent_aliases
            .as_ref()
            .is_some_and(|lent| lent.resources.contains_key(&crate::resource::ResourceName::new(name)))
    }
    fn is_transient_alias(&self, token: u64) -> bool {
        self.transient_alias_tokens.contains_key(&token)
    }
    fn claim_pooled(&mut self, token: u64, sharing: crate::tracking::ResourceSharing) -> bool {
        if let Some(resource) = self.resources.get(&token) {
            return resource.sharing_mode() == sharing;
        }
        // Parked idle resource: take it back for this frame. A token the
        // evictor already released is not found and is refused.
        match self.caches.claim(token, sharing) {
            Some((resource, family)) => {
                self.resources.insert(token, resource);
                if let Some(family) = family {
                    self.import_families.insert(token, family);
                }
                true
            }
            None => false,
        }
    }
    fn park_pooled(&mut self, token: u64) {
        if self.transient_alias_tokens.contains_key(&token) {
            return;
        }
        if let Some(resource) = self.resources.remove(&token) {
            let family = self.import_families.remove(&token);
            self.caches.park(token, resource, family);
        }
    }
    fn create_without_initial_data(
        &mut self,
        descriptor: graph::Descriptor,
        name: &str,
    ) -> Result<u64, graph::BackendFailure> {
        let prepared = self
            .lent_aliases
            .as_mut()
            .and_then(|lent| lent.resources.remove(&crate::resource::ResourceName::new(name)));
        let origin = prepared.as_ref().map(|(_, origin)| *origin);
        let resource = if let Some((resource, _)) = prepared {
            resource
        } else {
            self.specifications
                .get(&crate::resource::ResourceName::new(name))
                .copied()
                .ok_or_else(|| crate::Error::invalid_usage(format!("缺少资源规格 {name}")))?
                .create_resource(self.device.clone(), descriptor.usage())?
        };
        let token = match self.bind_import_owned_by(resource.clone(), self.family) {
            Ok(token) => token,
            Err(error) => {
                if let Some(origin) = origin {
                    self.caches
                        .alias()
                        .give_back(origin, resource, &self.device.completed());
                }
                return Err(error);
            }
        };
        if let Some(origin) = origin {
            self.transient_alias_tokens.insert(token, origin);
        }
        Ok(token)
    }
    fn set_debug_name(&mut self, _token: u64, _descriptor: graph::Descriptor, _name: &str) {}
    fn release_created_resource(&mut self, token: u64) {
        // 只在 GPU 不会再用到该资源时调用（见 graph::reclaim_failed_resources）；命令拥有者仍持有的 Arc 不受影响。
        let resource = self.resources.remove(&token);
        if let (Some(origin), Some(resource)) =
            (self.transient_alias_tokens.remove(&token), resource)
        {
            // Completion is re-checked: a member of a partially submitted
            // frame rejoins its set only after its GPU work finished.
            self.caches
                .alias()
                .give_back(origin, resource, &self.device.completed());
        }
        self.import_families.remove(&token);
    }
}

impl VulkanGraphBackend {
    pub fn declare(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<crate::rg::resource::ResourceHandle, graph::BuildError> {
        let descriptor = spec.descriptor();
        if self.specifications.contains_key(&crate::resource::ResourceName::new(name)) {
            return Err(graph::BuildError::DuplicateResourceName(name.into()));
        }
        let handle = graph.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor,
                source: graph::OriginSource::Create,
            },
        )?;
        graph.declaration.resources[handle.raw() as usize].sharing = spec.sharing();
        self.specifications.insert(name.into(), spec);
        Ok(handle)
    }

    /// Change a not-yet-instantiated declaration and its allocation specification
    /// together. Imported objects retain the sharing mode chosen at creation.
    pub fn set_declared_sharing(
        &mut self,
        built: &mut graph::BuiltGraph,
        name: &str,
        sharing: crate::tracking::ResourceSharing,
    ) -> crate::Result<()> {
        self.device.validate_sharing(sharing)?;
        let declaration = built
            .declaration
            .resources
            .iter_mut()
            .find(|resource| resource.name == name)
            .ok_or_else(|| {
                crate::Error::invalid_usage("sharing override names an unknown resource")
            })?;
        if !matches!(declaration.source, graph::OriginSource::Create) {
            return Err(crate::Error::invalid_usage(
                "sharing cannot be changed after physical resource creation",
            ));
        }
        let specification = self.specifications.get_mut(&crate::resource::ResourceName::new(name)).ok_or_else(|| {
            crate::Error::invalid_usage("sharing override has no allocation specification")
        })?;
        *specification = specification.with_sharing(sharing);
        declaration.sharing = sharing;
        Ok(())
    }

    pub fn resource_clone(&self, token: u64) -> Option<GpuResource> {
        self.resources.get(&token).cloned()
    }

    pub(in crate::rg::vulkan) fn collect_completed_managed(&self) -> crate::Result<()> {
        self.device.try_collect_garbage().map(|_| ())
    }

    /// 跨帧沿用 token 和真实 Arc，不以 token 猜测 Vulkan 句柄或 generation。
    pub fn bind_import_named(
        &mut self,
        name: &str,
        token: u64,
        resource: GpuResource,
    ) -> crate::Result<()> {
        self.bind_import_named_owned_by(name, token, resource, self.device.queues.primary_family)
    }

    /// 已有独占资源须声明真实归属；不为跨队列族资源伪造 release/acquire。
    pub fn bind_import_named_owned_by(
        &mut self,
        _name: &str,
        token: u64,
        resource: GpuResource,
        owner_family: u32,
    ) -> crate::Result<()> {
        if !(GENERATED_TOKEN_LIMIT..u64::MAX).contains(&token) {
            return Err(crate::Error::invalid_usage("显式导入 token 必须使用宿主高位域"));
        }
        self.bind_existing_token(token, resource, owner_family)
    }

    // Only allocator-issued internal owners can rebind the generated domain.
    // External imports enter through the disjoint host domain above.
    pub(in crate::rg::vulkan) fn bind_existing_token(
        &mut self,
        token: u64,
        resource: GpuResource,
        owner_family: u32,
    ) -> crate::Result<()> {
        resource.check_device(&self.device)?;
        if token == 0 || token == u64::MAX {
            return Err(crate::Error::invalid_usage("跨帧资源 token 无效"));
        }
        if let Some(old) = self.resources.get(&token) {
            let same = match (old, &resource) {
                (GpuResource::Buffer(a), GpuResource::Buffer(b)) => Arc::ptr_eq(a, b),
                (GpuResource::Image(a), GpuResource::Image(b)) => Arc::ptr_eq(a, b),
                _ => false,
            };
            if !same {
                return Err(crate::Error::invalid_usage("token 已经属于其他物理资源"));
            }
        }
        self.resources.insert(token, resource);
        self.import_families.insert(token, owner_family);
        Ok(())
    }

    pub fn bind_import(&mut self, resource: GpuResource) -> crate::Result<u64> {
        self.bind_import_owned_by(resource, self.device.queues.primary_family)
    }

    pub fn bind_import_owned_by(
        &mut self,
        resource: GpuResource,
        owner_family: u32,
    ) -> crate::Result<u64> {
        resource.check_device(&self.device)?;
        let token =
            try_allocate_token().ok_or_else(|| crate::Error::unsupported("资源 token 空间耗尽"))?;
        self.resources.insert(token, resource);
        self.import_families.insert(token, owner_family);
        Ok(token)
    }
}

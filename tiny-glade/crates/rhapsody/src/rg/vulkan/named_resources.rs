//! Session-owned named and temporal GPU resources.
use ash::vk;
use crate::device::Device;
use crate::resource::buffer;
use std::collections::BTreeMap;
use std::sync::Arc;
use super::backend::VulkanGraphBackend;
use super::resource_spec::GpuResource;
use super::resource_spec::ResourceSpec;
use crate::rg::graph;
use super::try_allocate_token;

pub(super) struct NamedResources {
    pub(super) state: crate::rg::temporal::TemporalRenderGraphState,
    pub(super) resources: TemporalGpuResources,
    pub(super) graph_id: Option<crate::rg::resource::GraphId>,
}

impl NamedResources {
    pub(super) fn new(device: Arc<Device>) -> Self {
        Self {
            state: Default::default(),
            resources: TemporalGpuResources::new(device),
            graph_id: None,
        }
    }
    pub(super) fn validate_graph(&self, graph: &graph::RenderGraph) -> crate::Result<()> {
        if self.graph_id != Some(graph.graph_id()) {
            return Err(crate::Error::invalid_usage(
                "named resource request requires the session's current frame graph",
            ));
        }
        Ok(())
    }
    pub(super) fn ensure_capacity(&self, key: &str, additional: usize) -> crate::Result<()> {
        if !self.resources.specifications.contains_key(&crate::resource::ResourceName::new(key))
            && self
                .resources
                .specifications
                .len()
                .saturating_add(additional)
                > 1024
        {
            return Err(crate::Error::unsupported(
                "session named-resource capacity exceeded",
            ));
        }
        Ok(())
    }
}

/// 帧缓存持有实际资源；每帧后端导入 Arc 后，resize 可安全释放旧缓存引用。
pub(super) struct TemporalGpuResources {
    pub(crate) device: Arc<Device>,
    specifications: BTreeMap<crate::resource::ResourceName, ResourceSpec>,
    /// Keys whose buffer is zero-filled when it is (re)created.
    zero_initialized: std::collections::BTreeSet<String>,
    /// Tokens whose complete allocation was uploaded at creation.
    defined: std::collections::BTreeSet<u64>,
    pub(super) resources: BTreeMap<u64, GpuResource>,
    pub(super) retired: Vec<u64>,
}

impl TemporalGpuResources {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
            specifications: BTreeMap::new(),
            zero_initialized: Default::default(),
            defined: Default::default(),
            resources: BTreeMap::new(),
            retired: Vec::new(),
        }
    }
    pub(super) fn register(&mut self, name: &str, spec: ResourceSpec) -> Option<ResourceSpec> {
        self.specifications.insert(name.into(), spec)
    }
    pub(super) fn set_zero_initialized(&mut self, name: &str, zeroed: bool) {
        if zeroed {
            if !self.zero_initialized.contains(name) {
                self.zero_initialized.insert(name.into());
            }
        } else {
            self.zero_initialized.remove(name);
        }
    }
    pub(super) fn restore_specification(&mut self, name: &str, previous: Option<ResourceSpec>) {
        if let Some(previous) = previous {
            self.specifications.insert(name.into(), previous);
        } else {
            self.specifications.remove(&crate::resource::ResourceName::new(name));
        }
    }
    pub fn resource(&self, token: u64) -> Option<GpuResource> {
        self.resources.get(&token).cloned()
    }
    pub fn import_into(
        &self,
        backend: &mut VulkanGraphBackend,
        _name: &str,
        token: u64,
    ) -> crate::Result<()> {
        let resource = self
            .resource(token)
            .ok_or_else(|| crate::Error::invalid_usage("跨帧物理资源缺失"))?;
        let family = backend
            .import_families
            .get(&token)
            .copied()
            .unwrap_or(backend.device.queues.primary_family);
        backend.bind_existing_token(token, resource, family)
    }
    pub fn len(&self) -> usize {
        self.resources.len()
    }
}

impl crate::rg::temporal::TemporalResourceOwner for TemporalGpuResources {
    fn create_temporal(
        &mut self,
        name: &str,
        descriptor: graph::Descriptor,
    ) -> Result<u64, graph::BuildError> {
        let spec = self.specifications.get(&crate::resource::ResourceName::new(name)).copied().ok_or_else(|| {
            graph::BuildError::ResourceCreationFailed(format!("没有{name}的物理资源规格"))
        })?;
        if spec.descriptor() != descriptor {
            return Err(graph::BuildError::ResourceCreationFailed(
                "跨帧声明与物理规格不一致".into(),
            ));
        }
        // Contents of a new named resource are undefined (history_valid is
        // false and a read before the first write is rejected) unless the
        // caller asked for a zero-filled buffer, which costs one upload.
        let zeroed = self.zero_initialized.contains(name);
        let resource = match spec {
            ResourceSpec::Buffer(desc) | ResourceSpec::ConcurrentBuffer(desc, _) if zeroed => {
                let size = usize::try_from(desc.size)
                    .map_err(|_| graph::BuildError::from(crate::Error::OutOfHostMemory))?;
                let mut initial = Vec::new();
                initial
                    .try_reserve_exact(size)
                    .map_err(|_| graph::BuildError::from(crate::Error::OutOfHostMemory))?;
                initial.resize(size, 0u8);
                let object = match spec.sharing() {
                    crate::tracking::ResourceSharing::Exclusive => self
                        .device
                        .create_buffer(&desc, name, Some(initial.as_slice()))
                        .map_err(graph::BuildError::from)?,
                    crate::tracking::ResourceSharing::Concurrent(families) => {
                        let mut physical_desc = desc;
                        physical_desc.usage |= vk::BufferUsageFlags::TRANSFER_DST;
                        let raw = buffer::Buffer::new_concurrent(
                            self.device.clone(),
                            &physical_desc,
                            families.as_slice(),
                        )
                        .map_err(graph::BuildError::from)?;
                        let managed = self
                            .device
                            .resources()?
                            .buffers
                            .allocate(raw)
                            .map_err(graph::BuildError::from)?;
                        self.device.set_buffer_debug_name(&managed, name);
                        if size != 0 {
                            self.device
                                .update_buffer_bytes(
                                    &managed,
                                    &initial,
                                    std::slice::from_ref(&(0..size)),
                                    0,
                                )
                                .map_err(graph::BuildError::from)?;
                        }
                        managed
                    }
                };
                GpuResource::Buffer(object.shared())
            }
            _ => spec
                .create_resource(self.device.clone(), 0)
                .map_err(graph::BuildError::from)?,
        };
        let token = try_allocate_token()
            .ok_or_else(|| graph::BuildError::ResourceCreationFailed("跨帧token耗尽".into()))?;
        self.resources.insert(token, resource);
        if zeroed
            && matches!(
                spec,
                ResourceSpec::Buffer(_) | ResourceSpec::ConcurrentBuffer(..)
            )
        {
            self.defined.insert(token);
        }
        Ok(token)
    }
    fn initial_contents_defined(&self, token: u64) -> bool {
        self.defined.contains(&token)
    }
    fn temporal_sharing(&self, name: &str) -> crate::tracking::ResourceSharing {
        self.specifications
            .get(&crate::resource::ResourceName::new(name))
            .map(|spec| spec.sharing())
            .unwrap_or_default()
    }
    fn release_temporal(&mut self, token: u64) {
        self.resources.remove(&token);
        self.defined.remove(&token);
        self.retired.push(token);
    }
}

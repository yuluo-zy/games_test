//! 构图 Ref 绑定到真实 managed 资源解析；复用原逻辑→物理 registry 和描述符消费接口。
use super::{
    binding::{self, PassResourceSlot, ResolvedBinding},
    graph::{BuildError, Descriptor, GraphDeclaration, OriginSource, PassDeclaration},
    resource::ResourceHandle,
    resource_registry::{
        Buffer as RegistryBuffer, RegistryResource, ResourceRegistry, ResourceReuseInfo,
        Texture as RegistryTexture,
    },
};
use crate::raw::{
    binding::DescriptorSetBinding,
    buffer::Buffer,
    device::VulkanDevice,
    managed::{
        buffer::ManagedBuffer,
        resources::{ManagedResources, ResourceKey, ResourceManager},
        texture::ManagedTexture,
        tracking::{CommandState, Failure},
    },
    texture::{Texture, TextureViewDesc},
};
use ash::vk;
use std::{
    collections::{BTreeMap, HashSet},
    fmt,
    sync::{Arc, Mutex},
};
#[derive(Debug)]
pub enum PassBindingError {
    Build(BuildError),
    MissingLogical,
    MissingPhysical,
    WrongResourceKind,
    UndeclaredAccess,
    WrongDevice,
    MissingManagedGeneration,
    InvalidRange,
    InvalidView,
    Tracking(Failure),
    Descriptor(binding::BindingError),
    Vulkan(vk::Result),
    Resource(String),
}
impl From<BuildError> for PassBindingError {
    fn from(value: BuildError) -> Self {
        Self::Build(value)
    }
}
impl From<vk::Result> for PassBindingError {
    fn from(value: vk::Result) -> Self {
        Self::Vulkan(value)
    }
}
impl fmt::Display for PassBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "资源绑定失败：{self:?}")
    }
}
impl std::error::Error for PassBindingError {}

#[derive(Clone, Copy)]
pub struct PassBinding {
    pub slot: PassResourceSlot,
    pub view: Option<TextureViewDesc>,
    pub expected_key: Option<ResourceKey>,
}
impl PassBinding {
    pub fn buffer(resource: ResourceHandle, offset: u64, size: u64, uniform: bool) -> Self {
        Self {
            slot: if uniform {
                PassResourceSlot::UniformBuffer {
                    resource,
                    offset,
                    size,
                }
            } else {
                PassResourceSlot::StorageBuffer {
                    resource,
                    offset,
                    size,
                }
            },
            view: None,
            expected_key: None,
        }
    }
    pub fn image(
        resource: ResourceHandle,
        view: TextureViewDesc,
        layout: vk::ImageLayout,
        storage: bool,
    ) -> Self {
        Self {
            slot: if storage {
                PassResourceSlot::StorageImage { resource, layout }
            } else {
                PassResourceSlot::SampledImage { resource, layout }
            },
            view: Some(view),
            expected_key: None,
        }
    }
    pub fn with_expected_generation(mut self, key: ResourceKey) -> Self {
        self.expected_key = Some(key);
        self
    }
    pub fn resource(&self) -> ResourceHandle {
        match self.slot {
            PassResourceSlot::UniformBuffer { resource, .. }
            | PassResourceSlot::StorageBuffer { resource, .. }
            | PassResourceSlot::SampledImage { resource, .. }
            | PassResourceSlot::StorageImage { resource, .. } => resource,
        }
    }
}
pub struct BoundPassBindings {
    pub pass: PassDeclaration,
    pub bindings: BTreeMap<String, Vec<PassBinding>>,
}
pub enum ManagedPassResource {
    Buffer(ManagedBuffer),
    Image(ManagedTexture),
}
pub type ManagedBindingRegistry = ResourceRegistry<ManagedTexture, ManagedBuffer, ()>;
pub struct ManagedPassResources {
    pub registry: ManagedBindingRegistry,
    device: Arc<VulkanDevice>,
    buffers: ResourceManager<Buffer>,
    textures: ResourceManager<Texture>,
    views: Mutex<BTreeMap<ViewKey, crate::raw::managed::texture::ManagedTextureView>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ViewKey {
    resource: ResourceKey,
    view_type: i32,
    format: i32,
    aspect: u32,
    mip: u32,
    levels: u32,
    layer: u32,
    layers: u32,
}
impl ManagedPassResources {
    /// 直接复用编译/池分配产生的映射；token 只经真实物理资源 provider 解析，不猜 Vulkan 句柄。
    pub fn from_registry<Other>(
        device: Arc<VulkanDevice>,
        source: &ResourceRegistry<u64, u64, Other>,
        stores: &ManagedResources,
        mut resolve: impl FnMut(u64) -> Option<super::vulkan::GpuResource>,
    ) -> Result<Self, PassBindingError> {
        let mut buffers: BTreeMap<u64, ManagedBuffer> = BTreeMap::new();
        let mut textures: BTreeMap<u64, ManagedTexture> = BTreeMap::new();
        let mut physical = Vec::new();
        for resource in &source.backing_resources {
            let value = match resource {
                RegistryResource::OwnedBuffer(buffer) | RegistryResource::UnownedBuffer(buffer) => {
                    let handle = if let Some(handle) = buffers.get(&buffer.handle) {
                        handle.clone()
                    } else {
                        let Some(super::vulkan::GpuResource::Buffer(raw)) = resolve(buffer.handle)
                        else {
                            return Err(PassBindingError::MissingPhysical);
                        };
                        if !Arc::ptr_eq(&device, &raw.device) {
                            return Err(PassBindingError::WrongDevice);
                        }
                        let managed = stores.buffers.allocate_shared(raw);
                        buffers.insert(buffer.handle, managed.clone());
                        managed
                    };
                    let value = RegistryBuffer {
                        handle,
                        desc: buffer.desc,
                    };
                    if matches!(resource, RegistryResource::UnownedBuffer(_)) {
                        RegistryResource::UnownedBuffer(value)
                    } else {
                        RegistryResource::OwnedBuffer(value)
                    }
                }
                RegistryResource::OwnedTexture(texture)
                | RegistryResource::UnownedTexture(texture) => {
                    let handle = if let Some(handle) = textures.get(&texture.handle) {
                        handle.clone()
                    } else {
                        let Some(super::vulkan::GpuResource::Image(raw)) = resolve(texture.handle)
                        else {
                            return Err(PassBindingError::MissingPhysical);
                        };
                        if !Arc::ptr_eq(&device, &raw.device) {
                            return Err(PassBindingError::WrongDevice);
                        }
                        let managed = stores.textures.allocate_shared(raw);
                        textures.insert(texture.handle, managed.clone());
                        managed
                    };
                    let value = RegistryTexture {
                        handle,
                        desc: texture.desc,
                    };
                    if matches!(resource, RegistryResource::UnownedTexture(_)) {
                        RegistryResource::UnownedTexture(value)
                    } else {
                        RegistryResource::OwnedTexture(value)
                    }
                }
                RegistryResource::Empty => RegistryResource::Empty,
                RegistryResource::Other(_) => return Err(PassBindingError::WrongResourceKind),
            };
            physical.push(value);
        }
        Ok(Self {
            registry: ResourceRegistry {
                resources: source.resources.clone(),
                resource_initial_access: source.resource_initial_access.clone(),
                backing_resources: physical,
                exported_logical_handles: source.exported_logical_handles.clone(),
            },
            device,
            buffers: stores.buffers.clone(),
            textures: stores.textures.clone(),
            views: Mutex::new(BTreeMap::new()),
        })
    }
    /// 私有物理资源由 factory 生产，映射和所有权标签不由 callback 手填。
    /// 可直接接已有资源创建/导入 factory，避免另写 Vulkan 分配器。
    pub fn populate(
        device: Arc<VulkanDevice>,
        graph: &GraphDeclaration,
        stores: &ManagedResources,
        mut factory: impl FnMut(
            &super::graph::ResourceDeclaration,
        ) -> Result<ManagedPassResource, PassBindingError>,
    ) -> Result<Self, PassBindingError> {
        let mut registry = ResourceRegistry {
            resources: Vec::new(),
            resource_initial_access: Vec::new(),
            backing_resources: Vec::new(),
            exported_logical_handles: HashSet::new(),
        };
        for declaration in &graph.resources {
            let initial = match declaration.source {
                OriginSource::Create => 0,
                OriginSource::ImportOwned { initial_access, .. }
                | OriginSource::ImportUnowned { initial_access, .. } => initial_access,
            };
            let resource = match (declaration.descriptor, factory(declaration)?) {
                (Descriptor::Buffer(desc), ManagedPassResource::Buffer(handle)) => {
                    if !Arc::ptr_eq(&device, &handle.device)
                        || stores.buffers.get(handle.key()).is_none()
                    {
                        return Err(PassBindingError::WrongDevice);
                    }
                    let value = RegistryBuffer { handle, desc };
                    if matches!(declaration.source, OriginSource::ImportUnowned { .. }) {
                        RegistryResource::UnownedBuffer(value)
                    } else {
                        RegistryResource::OwnedBuffer(value)
                    }
                }
                (Descriptor::Image(desc), ManagedPassResource::Image(handle)) => {
                    if !Arc::ptr_eq(&device, &handle.device)
                        || stores.textures.get(handle.key()).is_none()
                    {
                        return Err(PassBindingError::WrongDevice);
                    }
                    let value = RegistryTexture { handle, desc };
                    if matches!(declaration.source, OriginSource::ImportUnowned { .. }) {
                        RegistryResource::UnownedTexture(value)
                    } else {
                        RegistryResource::OwnedTexture(value)
                    }
                }
                _ => return Err(PassBindingError::WrongResourceKind),
            };
            let index = u16::try_from(registry.backing_resources.len())
                .map_err(|_| PassBindingError::MissingPhysical)?;
            if index == u16::MAX {
                return Err(PassBindingError::MissingPhysical);
            }
            registry.resources.push(ResourceReuseInfo {
                backing_index: index,
                reused: false,
            });
            registry.resource_initial_access.push(initial);
            registry.backing_resources.push(resource);
        }
        Ok(Self {
            registry,
            device,
            buffers: stores.buffers.clone(),
            textures: stores.textures.clone(),
            views: Mutex::new(BTreeMap::new()),
        })
    }
    pub fn resolve(
        &self,
        pass: &PassDeclaration,
        binding: PassBinding,
        domain: u32,
        command: CommandState,
    ) -> Result<DescriptorSetBinding, PassBindingError> {
        let mut resolved = Some(self.resolve_raw(pass, binding, domain, command)?);
        binding::rg_binding_to_descriptor_binding(binding.slot, |_| resolved.take())
            .map_err(PassBindingError::Descriptor)
    }
    fn resolve_raw(
        &self,
        pass: &PassDeclaration,
        binding: PassBinding,
        domain: u32,
        command: CommandState,
    ) -> Result<ResolvedBinding, PassBindingError> {
        if command.tag != 1 {
            return Err(PassBindingError::Tracking(Failure::InvalidCommandState));
        }
        let resource = binding.resource();
        if !pass
            .accesses
            .iter()
            .any(|a| a.resource_index == resource.raw())
        {
            return Err(PassBindingError::UndeclaredAccess);
        }
        let mapping = self
            .registry
            .resources
            .get(resource.raw() as usize)
            .ok_or(PassBindingError::MissingLogical)?;
        let physical = self
            .registry
            .backing_resources
            .get(mapping.backing_index as usize)
            .ok_or(PassBindingError::MissingPhysical)?;
        let resolved = match (binding.slot, physical) {
            (
                PassResourceSlot::UniformBuffer { offset, size, .. }
                | PassResourceSlot::StorageBuffer { offset, size, .. },
                RegistryResource::OwnedBuffer(buffer) | RegistryResource::UnownedBuffer(buffer),
            ) => {
                if binding
                    .expected_key
                    .is_some_and(|key| key != buffer.handle.key())
                {
                    return Err(PassBindingError::MissingManagedGeneration);
                }
                let managed = self
                    .buffers
                    .get(buffer.handle.key())
                    .ok_or(PassBindingError::MissingManagedGeneration)?;
                if !Arc::ptr_eq(&self.device, &managed.device) {
                    return Err(PassBindingError::WrongDevice);
                }
                let size = if size == vk::WHOLE_SIZE {
                    managed
                        .size
                        .checked_sub(offset)
                        .ok_or(PassBindingError::InvalidRange)?
                } else {
                    size
                };
                if size == 0
                    || offset
                        .checked_add(size)
                        .is_none_or(|end| end > managed.size)
                {
                    return Err(PassBindingError::InvalidRange);
                }
                let usage = if matches!(binding.slot, PassResourceSlot::UniformBuffer { .. }) {
                    vk::BufferUsageFlags::UNIFORM_BUFFER
                } else {
                    vk::BufferUsageFlags::STORAGE_BUFFER
                };
                if !managed.usage.contains(usage) {
                    return Err(PassBindingError::WrongResourceKind);
                }
                managed
                    .track_on(domain, command)
                    .map_err(PassBindingError::Tracking)?;
                ResolvedBinding::Buffer {
                    buffer: managed.handle,
                    owner: Arc::new(managed),
                }
            }
            (
                PassResourceSlot::SampledImage { layout, .. }
                | PassResourceSlot::StorageImage { layout, .. },
                RegistryResource::OwnedTexture(texture) | RegistryResource::UnownedTexture(texture),
            ) => {
                if binding
                    .expected_key
                    .is_some_and(|key| key != texture.handle.key())
                {
                    return Err(PassBindingError::MissingManagedGeneration);
                }
                let managed = self
                    .textures
                    .get(texture.handle.key())
                    .ok_or(PassBindingError::MissingManagedGeneration)?;
                if !Arc::ptr_eq(&self.device, &managed.device) {
                    return Err(PassBindingError::WrongDevice);
                }
                let storage = matches!(binding.slot, PassResourceSlot::StorageImage { .. });
                if storage && layout != vk::ImageLayout::GENERAL {
                    return Err(PassBindingError::InvalidView);
                }
                if !storage
                    && ![
                        vk::ImageLayout::GENERAL,
                        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                        vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
                    ]
                    .contains(&layout)
                {
                    return Err(PassBindingError::InvalidView);
                }
                let usage = if storage {
                    vk::ImageUsageFlags::STORAGE
                } else {
                    vk::ImageUsageFlags::SAMPLED
                };
                if !managed.desc.usage.contains(usage) {
                    return Err(PassBindingError::WrongResourceKind);
                }
                let desc = binding.view.ok_or(PassBindingError::InvalidView)?;
                if desc.format != managed.desc.format
                    || !crate::raw::format::vk_format_valid_aspect_flags(managed.desc.format)
                        .contains(desc.aspect)
                    || desc.aspect.is_empty()
                {
                    return Err(PassBindingError::InvalidView);
                }
                let key = ViewKey {
                    resource: managed.key(),
                    view_type: desc.view_type.as_raw(),
                    format: desc.format.as_raw(),
                    aspect: desc.aspect.as_raw(),
                    mip: desc.base_mip_level,
                    levels: desc.level_count,
                    layer: desc.base_layer,
                    layers: desc.layer_count,
                };
                let mut cache = self.views.lock().unwrap_or_else(|error| error.into_inner());
                let view = match cache.get(&key) {
                    Some(value) => value.clone(),
                    None => {
                        let value = managed
                            .image_view(&desc)
                            .map_err(|error| PassBindingError::Resource(error.to_string()))?;
                        cache.insert(key, value.clone());
                        value
                    }
                };
                view.track_on(domain, command)
                    .map_err(PassBindingError::Tracking)?;
                ResolvedBinding::Image {
                    view: view.raw(),
                    owner: Arc::new(view),
                }
            }
            _ => return Err(PassBindingError::WrongResourceKind),
        };
        Ok(resolved)
    }
    /// generation/subresource 是键的一部分；失效后已经写出的 descriptor 继续拥有旧视图。
    pub fn invalidate_views(&self) {
        self.views
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .clear();
    }
    pub fn view_count(&self) -> usize {
        self.views
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .len()
    }
    pub fn descriptor_writes(
        &self,
        plan: &BoundPassBindings,
        entry: &crate::raw::shader::ReflectedEntry,
        set: u32,
        domain: u32,
        command: CommandState,
    ) -> Result<Vec<crate::raw::descriptor_set::DescriptorWrite>, PassBindingError> {
        let mut slots: BTreeMap<String, Vec<PassResourceSlot>> = BTreeMap::new();
        for descriptor in entry
            .descriptors
            .iter()
            .filter(|descriptor| descriptor.set == set && descriptor.descriptor_type != "Sampler")
        {
            let mut normalized = Vec::new();
            for value in plan
                .bindings
                .get(&descriptor.name)
                .ok_or(PassBindingError::MissingLogical)?
            {
                let compatible = match value.slot {
                    PassResourceSlot::UniformBuffer { .. } => {
                        descriptor.descriptor_type.starts_with("UniformBuffer")
                    }
                    PassResourceSlot::StorageBuffer { .. } => {
                        descriptor.descriptor_type.starts_with("StorageBuffer")
                    }
                    PassResourceSlot::SampledImage { .. } => {
                        descriptor.descriptor_type.starts_with("SampledImage")
                    }
                    PassResourceSlot::StorageImage { .. } => {
                        descriptor.descriptor_type.starts_with("StorageImage")
                    }
                };
                if !compatible {
                    return Err(PassBindingError::WrongResourceKind);
                }
                let mut slot = value.slot;
                if let PassResourceSlot::UniformBuffer { offset, size, .. }
                | PassResourceSlot::StorageBuffer { offset, size, .. } = &mut slot
                {
                    if *size == vk::WHOLE_SIZE {
                        let mapping = self
                            .registry
                            .resources
                            .get(value.resource().raw() as usize)
                            .ok_or(PassBindingError::MissingLogical)?;
                        let physical = self
                            .registry
                            .backing_resources
                            .get(mapping.backing_index as usize)
                            .ok_or(PassBindingError::MissingPhysical)?;
                        let buffer = match physical {
                            RegistryResource::OwnedBuffer(buffer)
                            | RegistryResource::UnownedBuffer(buffer) => buffer,
                            _ => return Err(PassBindingError::WrongResourceKind),
                        };
                        *size = buffer
                            .handle
                            .size
                            .checked_sub(*offset)
                            .ok_or(PassBindingError::InvalidRange)?;
                    }
                }
                normalized.push(slot);
            }
            slots.insert(descriptor.name.clone(), normalized);
        }
        let mut selections: std::collections::VecDeque<_> = entry
            .descriptors
            .iter()
            .filter(|descriptor| descriptor.set == set && descriptor.descriptor_type != "Sampler")
            .flat_map(|descriptor| {
                plan.bindings
                    .get(&descriptor.name)
                    .into_iter()
                    .flatten()
                    .copied()
            })
            .collect();
        let mut failure = None;
        let writes =
            binding::create_descriptor_writes(entry, set, &plan.pass, &slots, |resource| {
                let value = selections.pop_front()?;
                if value.resource() != resource {
                    failure = Some(PassBindingError::MissingLogical);
                    return None;
                }
                match self.resolve_raw(&plan.pass, value, domain, command) {
                    Ok(value) => Some(value),
                    Err(error) => {
                        failure = Some(error);
                        None
                    }
                }
            });
        if let Some(error) = failure {
            return Err(error);
        }
        writes.map_err(PassBindingError::Descriptor)
    }
}

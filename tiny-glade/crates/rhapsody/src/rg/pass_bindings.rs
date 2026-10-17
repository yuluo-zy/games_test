//! 构图 Ref 绑定到真实 managed 资源解析；复用原逻辑→物理 registry 和描述符消费接口。
use super::{
    binding::PassResourceSlot,
    graph::{BuildError, Descriptor, PassDeclaration},
    resource::ResourceHandle,
    resource_registry::{
        Buffer as RegistryBuffer, RegistryResource, ResourceRegistry, Texture as RegistryTexture,
    },
};
use crate::raw::{
    buffer::Buffer,
    device::Device,
    managed::{
        buffer::ManagedBuffer,
        resources::{ManagedResources, ResourceManager},
        texture::ManagedTexture,
    },
    texture::{Texture, TextureViewDesc},
};
use ash::vk;
use std::{fmt, sync::Arc};
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
    Vulkan(vk::Result),
    Resource(String),
}
impl From<PassBindingError> for crate::Error {
    fn from(value: PassBindingError) -> Self {
        match value {
            PassBindingError::Build(error) => error.into(),
            PassBindingError::Vulkan(error) => error.into(),
            error => Self::invalid_usage_with("graph resource binding", error),
        }
    }
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
        }
    }
    /// Validate a callback binding against the graph scope and declared ranges
    /// before touching device objects or descriptor allocators.
    pub fn validate_declaration(
        self,
        pass: &PassDeclaration,
        descriptor: Descriptor,
    ) -> Result<(), PassBindingError> {
        let resource = self.resource();
        if let Some(expected) = pass.graph_id
            && resource.graph_id() != expected
        {
            return Err(PassBindingError::Build(BuildError::WrongGraph {
                expected,
                actual: resource.graph_id(),
            }));
        }
        let bound_range = match self.slot {
            PassResourceSlot::UniformBuffer { offset, size, .. }
            | PassResourceSlot::StorageBuffer { offset, size, .. } => {
                let Descriptor::Buffer(desc) = descriptor else {
                    return Err(PassBindingError::WrongResourceKind);
                };
                let size = if size == vk::WHOLE_SIZE {
                    desc.word_at_00
                        .checked_sub(offset)
                        .ok_or(PassBindingError::InvalidRange)?
                } else {
                    size
                };
                super::resource::AccessRange::Buffer { offset, size }
            }
            PassResourceSlot::SampledImage { .. } | PassResourceSlot::StorageImage { .. } => {
                let view = self.view.ok_or(PassBindingError::InvalidView)?;
                super::resource::AccessRange::Texture {
                    aspect_mask: view.aspect.as_raw(),
                    base_mip_level: view.base_mip_level,
                    level_count: view.level_count,
                    base_array_layer: view.base_layer,
                    layer_count: view.layer_count,
                }
            }
        };
        bound_range
            .validate(descriptor)
            .map_err(|_| PassBindingError::InvalidRange)?;
        let declared_ranges: Vec<_> = pass
            .accesses
            .iter()
            .filter(|access| access.resource_index == resource.raw())
            .map(|access| access.range)
            .collect();
        if !super::resource_contracts::covers(&declared_ranges, bound_range, descriptor)
            .map_err(|_| PassBindingError::InvalidRange)?
        {
            return Err(PassBindingError::UndeclaredAccess);
        }
        Ok(())
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
mod projection;
pub(crate) use projection::ManagedProjectionPool;
pub use projection::{ManagedProjectionConfig, ManagedProjectionStats};

pub type ManagedBindingRegistry = ResourceRegistry<ManagedTexture, ManagedBuffer, ()>;
pub struct ManagedPassResources {
    pub registry: ManagedBindingRegistry,
    buffers: ResourceManager<Buffer>,
    textures: ResourceManager<Texture>,
    projection: projection::ProjectionState,
}
impl ManagedPassResources {
    /// 逻辑资源 → 物理托管缓冲（pass 回调里直接发命令的路径用，如 `fill_buffer`/`copy_buffer`）。
    pub fn buffer_for(
        &self,
        resource: u32,
    ) -> Result<crate::raw::managed::buffer::ManagedBuffer, PassBindingError> {
        let mapping = self
            .registry
            .resources
            .get(resource as usize)
            .ok_or(PassBindingError::MissingLogical)?;
        match self
            .registry
            .backing_resources
            .get(mapping.backing_index as usize)
            .ok_or(PassBindingError::MissingPhysical)?
        {
            RegistryResource::OwnedBuffer(b) | RegistryResource::UnownedBuffer(b) => self
                .buffers
                .get(b.handle.key())
                .ok_or(PassBindingError::MissingManagedGeneration),
            _ => Err(PassBindingError::WrongResourceKind),
        }
    }
    /// 逻辑资源 → 物理托管图像。
    pub fn texture_for(
        &self,
        resource: u32,
    ) -> Result<crate::raw::managed::texture::ManagedTexture, PassBindingError> {
        let mapping = self
            .registry
            .resources
            .get(resource as usize)
            .ok_or(PassBindingError::MissingLogical)?;
        match self
            .registry
            .backing_resources
            .get(mapping.backing_index as usize)
            .ok_or(PassBindingError::MissingPhysical)?
        {
            RegistryResource::OwnedTexture(t) | RegistryResource::UnownedTexture(t) => self
                .textures
                .get(t.handle.key())
                .ok_or(PassBindingError::MissingManagedGeneration),
            _ => Err(PassBindingError::WrongResourceKind),
        }
    }
}

impl ManagedPassResources {
    /// Retain only resources declared by this pass. Barriers retain their own
    /// operands in the backend, including export-only transitions.
    pub(crate) fn retain_pass_resources(
        &self,
        pass: &PassDeclaration,
        owners: &mut super::owner_storage::ResourceOwners,
    ) -> crate::Result<()> {
        for access in &pass.accesses {
            let mapping = &self.registry.resources[access.resource_index as usize];
            match &self.registry.backing_resources[mapping.backing_index as usize] {
                RegistryResource::OwnedBuffer(buffer) | RegistryResource::UnownedBuffer(buffer) => {
                    owners.add_recorded(buffer.handle.shared())?
                }
                RegistryResource::OwnedTexture(texture)
                | RegistryResource::UnownedTexture(texture) => {
                    owners.add_recorded(texture.handle.shared())?
                }
                _ => {}
            }
        }
        Ok(())
    }
}

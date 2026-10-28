//! Resource, access-semantics and import definitions for a plan.
use super::super::graph::Descriptor;
use super::super::graph::OriginSource;
use super::super::resource_info::OriginKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceOriginKind {
    Create,
    ImportOwned,
    ImportUnowned,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceDefinition {
    pub sharing: crate::tracking::ResourceSharing,
    pub name: crate::resource::ResourceName,
    pub descriptor: Descriptor,
    pub origin: ResourceOriginKind,
}

impl ResourceDefinition {
    pub fn is_external(&self) -> bool {
        self.origin != ResourceOriginKind::Create
    }
    pub(super) fn info_origin(&self) -> OriginKind {
        match self.origin {
            ResourceOriginKind::Create => OriginKind::Tag0 {
                descriptor_code: if self.descriptor.kind_code() == 1 {
                    7
                } else {
                    0
                },
                image_creation_flags: if self.descriptor.kind_code() == 0 {
                    self.descriptor.usage()
                } else {
                    0
                },
                buffer_creation_flags: if self.descriptor.kind_code() == 1 {
                    self.descriptor.usage()
                } else {
                    0
                },
            },
            ResourceOriginKind::ImportOwned => OriginKind::Tag2 {
                type_code: self.descriptor.kind_code(),
            },
            ResourceOriginKind::ImportUnowned => OriginKind::Tag1 {
                type_code: self.descriptor.kind_code(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccessSemantics {
    pub code: u8,
    /// The policy's declared access; it alone derives resource usage flags.
    pub declared_access: ash::vk::AccessFlags,
    /// The native synchronization scope (possibly wider than declared).
    pub scope: crate::native::AccessScope,
    pub layout: ash::vk::ImageLayout,
}

/// All policy values baked into planning are part of the complete cache key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PlanContext {
    pub queue_domains: Option<[super::super::queue_schedule::QueueDomain; 3]>,
    pub device_generation: u64,
    pub queue_family: u32,
    pub queue_index: u32,
    pub policy_signature: u64,
    /// The enabled feature controls whether depth and stencil layouts may vary.
    pub separate_depth_stencil_layouts: bool,
}

/// One imported resource bound for a frame of a retained plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportBinding {
    /// Logical resource index in the planned declaration order.
    pub logical: u32,
    /// Must be `ImportOwned`/`ImportUnowned`, matching the planned origin.
    pub source: OriginSource,
}

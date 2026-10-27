//! Descriptor kinds, configuration and bindless indices.
use ash::vk;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DescriptorKind {
    SampledImage = 0,
    StorageImage = 1,
    StorageBuffer = 2,
    Sampler = 3,
}

impl DescriptorKind {
    pub(super) const ALL: [Self; 4] = [
        Self::SampledImage,
        Self::StorageImage,
        Self::StorageBuffer,
        Self::Sampler,
    ];
    pub(super) fn ty(self) -> vk::DescriptorType {
        match self {
            Self::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
            Self::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
            Self::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
            Self::Sampler => vk::DescriptorType::SAMPLER,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BindlessConfig {
    pub sampled_images: u32,
    pub storage_images: u32,
    pub storage_buffers: u32,
    pub samplers: u32,
}

impl Default for BindlessConfig {
    fn default() -> Self {
        Self {
            sampled_images: 256,
            storage_images: 256,
            storage_buffers: 256,
            samplers: 64,
        }
    }
}

impl BindlessConfig {
    pub(super) fn counts(self) -> [u32; 4] {
        [
            self.sampled_images,
            self.storage_images,
            self.storage_buffers,
            self.samplers,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BindlessIndex {
    pub kind: DescriptorKind,
    pub index: u32,
    pub generation: u64,
}

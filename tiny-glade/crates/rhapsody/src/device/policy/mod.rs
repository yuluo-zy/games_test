//! Device policy reflects implemented paths and actually enabled capabilities.
//! Production descriptors use the global heap; missing indexing features are
//! Unsupported. Descriptor buffer remains an optional future experiment.
use crate::device::caps::{DeviceCaps, Feature, FeatureSet};
use crate::error::Result;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DescriptorStrategy {
    #[default]
    Bindless,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AllocationStrategy {
    #[default]
    Suballocated,
    Dedicated,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PolicyPreferences {
    pub allocation: AllocationStrategy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionPolicy {
    pub descriptor: DescriptorStrategy,
    pub allocation: AllocationStrategy,
}

/// bindless 依赖的整组 descriptor indexing 能力（缺一不可）。
pub(crate) const BINDLESS_REQUIRED: [Feature; 13] = [
    Feature::DescriptorIndexing,
    Feature::ShaderStorageBufferArrayDynamicIndexing,
    Feature::ShaderSampledImageArrayDynamicIndexing,
    Feature::ShaderStorageImageArrayDynamicIndexing,
    Feature::RuntimeDescriptorArray,
    Feature::DescriptorBindingPartiallyBound,
    Feature::SampledImageUpdateAfterBind,
    Feature::StorageBufferUpdateAfterBind,
    Feature::StorageImageUpdateAfterBind,
    Feature::ShaderSampledImageArrayNonUniformIndexing,
    Feature::ShaderStorageImageArrayNonUniformIndexing,
    Feature::ShaderStorageBufferArrayNonUniformIndexing,
    Feature::DescriptorBindingUpdateUnusedWhilePending,
];

pub(crate) fn bindless_features() -> FeatureSet {
    BINDLESS_REQUIRED
        .iter()
        .fold(FeatureSet::EMPTY, |set, &feature| set.with(feature))
}

pub fn plan_policy(caps: &DeviceCaps, preferences: &PolicyPreferences) -> Result<ExecutionPolicy> {
    caps.require_all_enabled(bindless_features())?;
    Ok(ExecutionPolicy {
        descriptor: DescriptorStrategy::Bindless,
        allocation: preferences.allocation,
    })
}

#[cfg(test)]
mod tests;

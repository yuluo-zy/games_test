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
mod tests {
    use super::*;
    use crate::error::ErrorKind;

    #[test]
    fn every_indexing_feature_must_be_actually_enabled() {
        let required = bindless_features();
        for feature in BINDLESS_REQUIRED {
            let caps = DeviceCaps::new(
                required,
                required.difference(FeatureSet::EMPTY.with(feature)),
            );
            assert_eq!(
                plan_policy(&caps, &PolicyPreferences::default())
                    .unwrap_err()
                    .kind(),
                ErrorKind::Unsupported
            );
        }
    }

    #[test]
    fn unimplemented_optional_capabilities_do_not_select_another_renderer() {
        let enabled = bindless_features().with(Feature::DescriptorBuffer);
        let policy = plan_policy(
            &DeviceCaps::new(enabled, enabled),
            &PolicyPreferences::default(),
        )
        .unwrap();
        assert_eq!(policy.descriptor, DescriptorStrategy::Bindless);
    }

    #[test]
    fn policy_carries_allocation_preferences() {
        let enabled = bindless_features();
        let policy = plan_policy(
            &DeviceCaps::new(enabled, enabled),
            &PolicyPreferences {
                allocation: AllocationStrategy::Dedicated,
            },
        )
        .unwrap();
        assert_eq!(policy.allocation, AllocationStrategy::Dedicated);
    }
}

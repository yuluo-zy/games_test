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

/// How render-graph batches are recorded. This is the single definition of
/// the default for devices and graph sessions alike; the chunking parameters
/// for each variant live in `rg::parallel::ParallelRecording::from`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RecordingPolicy {
    /// One command buffer per batch, always on the calling thread.
    Serial,
    /// Serial for the first frame and for light frames (no statistics or a
    /// cheap previous frame); otherwise balanced parallel chunking driven by
    /// the previous frame's per-pass recording cost.
    #[default]
    Adaptive,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AllocationStrategy {
    #[default]
    Suballocated,
    Dedicated,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PolicyPreferences {
    pub recording: RecordingPolicy,
    pub allocation: AllocationStrategy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionPolicy {
    pub descriptor: DescriptorStrategy,
    pub recording: RecordingPolicy,
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
        recording: preferences.recording,
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
    fn adaptive_recording_is_the_one_default_for_devices_and_graphs() {
        assert_eq!(RecordingPolicy::default(), RecordingPolicy::Adaptive);
        assert_eq!(
            PolicyPreferences::default().recording,
            RecordingPolicy::Adaptive
        );
        assert_eq!(
            crate::device::DeviceConfig::default().recording,
            RecordingPolicy::Adaptive
        );
        assert_eq!(
            crate::rg::parallel::ParallelRecording::default(),
            crate::rg::parallel::ParallelRecording::from(RecordingPolicy::Adaptive)
        );
        assert_eq!(
            crate::rg::parallel::ParallelRecording::from(RecordingPolicy::Adaptive),
            crate::rg::parallel::ParallelRecording::ADAPTIVE
        );
    }

    #[test]
    fn policy_carries_recording_and_allocation_preferences() {
        let enabled = bindless_features();
        let policy = plan_policy(
            &DeviceCaps::new(enabled, enabled),
            &PolicyPreferences {
                recording: RecordingPolicy::Serial,
                allocation: AllocationStrategy::Dedicated,
            },
        )
        .unwrap();
        assert_eq!(policy.recording, RecordingPolicy::Serial);
        assert_eq!(policy.allocation, AllocationStrategy::Dedicated);
    }
}

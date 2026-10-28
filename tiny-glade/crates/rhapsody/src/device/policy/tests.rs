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

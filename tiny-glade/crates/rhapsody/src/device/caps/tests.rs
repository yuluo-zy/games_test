use super::*;
use crate::error::ErrorKind;

#[test]
fn feature_bits_are_distinct_and_within_u64() {
    let mut seen = 0u64;
    for feature in Feature::ALL {
        let bit = feature.bit();
        assert_ne!(bit & seen, bit, "{feature:?} 的位与已有能力重叠");
        seen |= bit;
    }
}

#[test]
fn feature_set_operations_match_bit_semantics() {
    let a = FeatureSet::EMPTY
        .with(Feature::TimelineSemaphore)
        .with(Feature::DynamicRendering);
    let b = FeatureSet::EMPTY
        .with(Feature::DynamicRendering)
        .with(Feature::BufferDeviceAddress);

    assert_eq!(a.len(), 2);
    assert!(!a.is_empty());
    assert!(a.contains(Feature::TimelineSemaphore));
    assert_eq!(
        a.union(b),
        FeatureSet::from_bits(
            Feature::TimelineSemaphore.bit()
                | Feature::DynamicRendering.bit()
                | Feature::BufferDeviceAddress.bit()
        )
    );
    assert_eq!(
        a.intersection(b),
        FeatureSet::EMPTY.with(Feature::DynamicRendering)
    );
    assert_eq!(
        a.difference(b),
        FeatureSet::EMPTY.with(Feature::TimelineSemaphore)
    );
    assert!(a.union(b).is_superset(b));
    assert!(!a.is_superset(b));
}

#[test]
fn iter_yields_exactly_the_set_bits() {
    let set = FeatureSet::EMPTY
        .with(Feature::WideLines)
        .with(Feature::MeshShader);
    // `Feature` 的 `Ord` 按判别值排序（MeshShader=9 < WideLines=13）。
    let mut iterated = set.iter().collect::<Vec<_>>();
    iterated.sort_unstable();
    assert_eq!(iterated, [Feature::MeshShader, Feature::WideLines]);
}

#[test]
fn caps_separate_supported_from_enabled() {
    let supported = FeatureSet::EMPTY
        .with(Feature::DescriptorIndexing)
        .with(Feature::DescriptorBuffer);
    let enabled = FeatureSet::EMPTY.with(Feature::DescriptorIndexing);
    let caps = DeviceCaps::new(supported, enabled);

    assert!(caps.is_consistent());
    assert!(caps.is_supported(Feature::DescriptorBuffer));
    assert!(!caps.is_enabled(Feature::DescriptorBuffer), "支持 ≠ 已启用");
    assert!(caps.require_supported(Feature::DescriptorBuffer).is_ok());
    assert_eq!(
        caps.require_enabled(Feature::DescriptorBuffer)
            .unwrap_err()
            .kind(),
        ErrorKind::Unsupported
    );
}

#[test]
fn inconsistent_caps_are_detected() {
    let caps = DeviceCaps::new(
        FeatureSet::EMPTY.with(Feature::TimelineSemaphore),
        FeatureSet::EMPTY.with(Feature::MeshShader),
    );
    assert!(!caps.is_consistent());
}

#[test]
fn require_all_enabled_lists_missing_features() {
    let caps = DeviceCaps::new(
        FeatureSet::EMPTY.with(Feature::DescriptorIndexing),
        FeatureSet::EMPTY.with(Feature::DescriptorIndexing),
    );
    let required = FeatureSet::EMPTY
        .with(Feature::DescriptorIndexing)
        .with(Feature::DescriptorBuffer);
    let error = caps.require_all_enabled(required).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert!(error.to_string().contains("DescriptorBuffer"));
}

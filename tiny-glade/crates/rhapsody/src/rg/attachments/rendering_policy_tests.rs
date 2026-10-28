use super::*;
#[test]
fn auto_uses_only_enabled_feature_with_explicit_layouts() {
    assert!(RenderingPolicy::Auto.select_dynamic(false, true).is_err());
    assert!(RenderingPolicy::Auto.select_dynamic(true, false).is_err());
    assert!(RenderingPolicy::Auto.select_dynamic(true, true).unwrap());
}
#[test]
fn required_dynamic_rejects_unsupported_or_implicit_transition() {
    for (enabled, compatible) in [(false, true), (true, false), (false, false)] {
        assert_eq!(
            RenderingPolicy::RequireDynamic
                .select_dynamic(enabled, compatible)
                .unwrap_err()
                .kind(),
            crate::ErrorKind::Unsupported
        );
    }
    assert!(
        RenderingPolicy::RequireDynamic
            .select_dynamic(true, true)
            .unwrap()
    );
}

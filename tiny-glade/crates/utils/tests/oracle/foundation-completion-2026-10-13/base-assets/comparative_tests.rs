use super::*;
#[test]
fn original_static_region_and_lerp_policy() {
    let line=ZeroBoundedComparativeSpace {upper_bound:10.0};
    assert!(!line.regions_intersect(2.0,1.0,4.0,1.0));
    assert!(line.regions_intersect(2.0,1.0,3.99,1.0));
    let circle=CircularComparativeSpace {period:10.0};
    assert!(circle.regions_intersect(0.25,0.5,9.75,0.5));
    assert_eq!(circle.lerp_clamp(9.0,1.0,0.5),0.0);
    assert_eq!(line.lerp_clamp(9.0,1.0,2.0),0.0);
    assert_eq!(line.classify_side(-0.0,0.0),ComparativeSide::NonNegative);
    assert_eq!(line.classify_side(f32::NAN,0.0),ComparativeSide::NonNegative);
}

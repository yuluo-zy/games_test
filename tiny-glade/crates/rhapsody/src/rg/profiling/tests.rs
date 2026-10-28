use super::*;
#[test]
fn timestamp_wrap_and_period_are_applied_without_treating_missing_as_zero() {
    assert_eq!(duration_ns([250, 4], 8, 2.0), Some(20));
    assert_eq!(duration_ns([10, 10], 64, 1.0), Some(0));
    assert_eq!(duration_ns([1, 3], 0, 1.0), None);
    assert_eq!(duration_ns([1, 3], 64, f32::NAN), None);
    assert_eq!(duration_ns([1, 3], 64, 0.0), None);
}

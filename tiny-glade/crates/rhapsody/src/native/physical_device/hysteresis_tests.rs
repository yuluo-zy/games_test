use super::clamp_budget_hysteresis as clamp;

#[test]
fn eighth_window_with_unit_floor() {
    assert_eq!(clamp(800, 10_000), 900);
    assert_eq!(clamp(800, 0), 700);
    assert_eq!(clamp(800, 850), 850);
    // old < 8: delta 取 1，而不是 old/8 == 0。
    assert_eq!(clamp(5, 100), 6);
    assert_eq!(clamp(5, 0), 4);
    assert_eq!(clamp(0, 100), 1);
    assert_eq!(clamp(u64::MAX, 0), u64::MAX - (u64::MAX >> 3));
}

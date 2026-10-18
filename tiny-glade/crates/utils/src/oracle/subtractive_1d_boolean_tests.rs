use super::*;
#[test]
fn endpoint_flag_distinguishes_full_coverage() {
    let mut closed = SubtractiveBoolean1D::new_range(0.0, 1.0);
    closed.subtract(Interval {
        start: 0.0,
        end: 1.0,
        upper_exclusive: false,
    });
    assert!(closed.result().is_empty());
    let mut open = SubtractiveBoolean1D::new_range(0.0, 1.0);
    open.subtract(Interval {
        start: 0.0,
        end: 1.0,
        upper_exclusive: true,
    });
    assert_eq!(
        open.result(),
        [Interval {
            start: 1.0,
            end: 1.0,
            upper_exclusive: false
        }]
    );
}

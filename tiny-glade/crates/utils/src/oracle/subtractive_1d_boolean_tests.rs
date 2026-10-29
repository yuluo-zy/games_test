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
        [Interval { start: 0.0, end: 0.0, upper_exclusive: false }, Interval {
            start: 1.0,
            end: 1.0,
            upper_exclusive: false
        }]
    );
}

#[test]
fn original_unordered_and_reversed_bounds() {
    let cases: &[(f32, f32, &[(f32, f32)])] = &[
        (f32::NAN, f32::NAN, &[(0.0, 1.0)]),
        (f32::NAN, 0.5, &[(0.5, 1.0)]),
        (0.5, f32::NAN, &[(0.0, 0.5)]),
        (0.75, 0.25, &[(0.0, 0.75), (0.25, 1.0)]),
    ];
    for &(start, end, expected) in cases {
        let mut value = SubtractiveBoolean1D::new_range(0.0, 1.0);
        value.subtract(Interval { start, end, upper_exclusive: false });
        assert_eq!(value.result().iter().map(|v| (v.start, v.end)).collect::<Vec<_>>(), expected);
    }
    let value = SubtractiveBoolean1D::new_range(f32::NAN, 1.0);
    assert!(value.result()[0].start.is_nan());
}

//! Deterministic reference checks for clipping and periodic arithmetic.
use utils::{
    comparative_space::{CircularComparativeSpace, ComparativeSpace},
    subtractive_1d_boolean::{Interval, SubtractiveBoolean1D},
};

fn interval(start: f32, end: f32, upper_exclusive: bool) -> Interval {
    Interval {
        start,
        end,
        upper_exclusive,
    }
}

#[test]
fn clipping_preserves_original_boundary_records() {
    let cases: &[(f32, f32, bool, &[(f32, f32)])] = &[
        (0.0, 1.0, true, &[(0.0, 0.0), (1.0, 1.0)]),
        (0.0, 0.0, false, &[(0.0, 0.0), (0.0, 1.0)]),
        (1.0, 1.0, false, &[(0.0, 1.0), (1.0, 1.0)]),
    ];
    for &(start, end, exclusive, expected) in cases {
        let mut value = SubtractiveBoolean1D::new_range(0.0, 1.0);
        value.subtract(interval(start, end, exclusive));
        let expected: Vec<_> = expected.iter().map(|&(a,b)| interval(a,b,false)).collect();
        assert_eq!(value.result(), expected);
    }
}

#[test]
fn original_four_cut_sequences_match_native_records() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m06-boolean-sequence.json"
    ))).unwrap();
    for case in fixtures.as_array().unwrap() {
        let args = case["args"].as_array().unwrap();
        let value = |i: usize| args[i].as_str().unwrap().parse::<f32>().unwrap();
        let mut result = SubtractiveBoolean1D::new_range(value(0), value(1));
        for i in (2..args.len()).step_by(3) {
            result.subtract(interval(value(i), value(i+1), args[i+2].as_str().unwrap() == "1"));
        }
        let records = case["native"].as_array().unwrap();
        let expected_count = records[0].as_str().unwrap().strip_prefix("count=").unwrap().parse::<usize>().unwrap();
        assert_eq!(result.result().len(), expected_count, "case {}", case["case"]);
        for (actual, expected) in result.result().iter().zip(&records[1..]) {
            let hex = expected.as_str().unwrap();
            let bytes: Vec<_> = hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect();
            assert_eq!(actual.start.to_bits().to_le_bytes(), bytes[0..4], "case {}", case["case"]);
            assert_eq!(actual.end.to_bits().to_le_bytes(), bytes[4..8], "case {}", case["case"]);
            assert_eq!(u8::from(actual.upper_exclusive), bytes[8], "case {}", case["case"]);
        }
    }
}

#[test]
fn clipping_interior_membership_matches_reference() {
    for seed in [0x21e4_5a1d, 0xcba8_3142, 0xace0_77ef] {
        let rng = fastrand::Rng::with_seed(seed);
        for _ in 0..2_000 {
            let start = rng.i32(-2..66) as f32;
            let end = start + rng.u32(0..5) as f32;
            let mut value = SubtractiveBoolean1D::new_range(0.0, 64.0);
            value.subtract(interval(start, end, rng.bool()));
            for i in 0..64 {
                let probe = i as f32 + 0.5;
                let observed = value.result().iter().any(|r| r.start <= probe && probe <= r.end);
                assert_eq!(observed, !(start <= probe && probe < end), "seed={seed} probe={probe}");
            }
        }
    }
}

fn circular_reference(a: f32, b: f32, period: f32) -> f32 {
    let period = f64::from(period);
    // Each f32 value is exactly representable in f64. Reduce before subtracting
    // so widely separated exponents do not erase the smaller operand's phase.
    let difference = (f64::from(a) % period - f64::from(b) % period) % period;
    let centered = if difference < -period / 2.0 {
        difference + period
    } else if difference >= period / 2.0 {
        difference - period
    } else {
        difference
    };
    centered as f32
}

#[test]
fn circular_displacement_matches_wide_reference_for_extreme_finite_bits() {
    let rng = fastrand::Rng::with_seed(0xc14c_0a27);
    let check = |a: f32, b: f32, period: f32| {
        let space = CircularComparativeSpace { period };
        let expected = circular_reference(a, b, period);
        let actual = space.subtract(a, b);
        // The common projected path has one f32 addition/subtraction rounding.
        let tolerance = f64::from(period) * 2.0 * f64::from(f32::EPSILON);
        assert!(
            (f64::from(actual) - f64::from(expected)).abs() <= tolerance,
            "a={a}, b={b}, period={period}, actual={actual}, expected={expected}"
        );
        assert!(actual.is_finite());
        assert_eq!(space.distance(a, b), actual.abs());
        let opposite = space.subtract(b, a);
        let half = f64::from(period) / 2.0;
        // Both orientations intentionally select the negative half-turn.
        if (f64::from(actual).abs() - half).abs() > tolerance {
            assert!((f64::from(actual) + f64::from(opposite)).abs() <= tolerance);
        }
    };
    for period in [
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        0.25,
        11.0,
        65_536.0,
        f32::MAX,
    ] {
        for a in [-f32::MAX, -1e20, -22.0, -0.0, 0.0, 33.0, 1e20, f32::MAX] {
            for b in [-f32::MAX, -1e20, -1.0, 0.0, 1.0, 1e20, f32::MAX] {
                check(a, b, period);
            }
        }
    }
    for _ in 0..30_000 {
        let a = f32::from_bits(rng.u32(..));
        let b = f32::from_bits(rng.u32(..));
        let period = f32::from_bits(rng.u32(..) & 0x7fff_ffff);
        if a.is_finite() && b.is_finite() && period.is_finite() && period > 0.0 {
            check(a, b, period);
        }
    }
    assert_eq!(
        CircularComparativeSpace { period: 11.0 }.subtract(f32::MAX, -f32::MAX),
        -4.0
    );
}

#[test]
fn circular_small_integer_phases_are_invariant_under_multiple_turns() {
    for period in [2.0, 11.0, 16.0, 256.0] {
        let space = CircularComparativeSpace { period };
        for a in -16..=16 {
            for b in -16..=16 {
                let expected = space.subtract(a as f32, b as f32);
                for turns in [-4096.0, -17.0, 0.0, 19.0, 4096.0] {
                    assert_eq!(
                        space.subtract(a as f32 + turns * period, b as f32),
                        expected
                    );
                    assert_eq!(
                        space.subtract(a as f32, b as f32 + turns * period),
                        expected
                    );
                }
            }
        }
        assert_eq!(space.subtract(period / 2.0, 0.0), -period / 2.0);
        assert_eq!(space.subtract(0.0, period / 2.0), -period / 2.0);
    }
}

#[test]
fn circular_interpolation_preserves_short_displacements_at_large_coordinates() {
    let space = CircularComparativeSpace { period: 11.0 };
    assert_eq!(space.lerp_clamp(100_000_000.0, 0.0, 0.5), 0.5);
    let rng = fastrand::Rng::with_seed(0x117e_5ca1);
    for _ in 0..10_000 {
        let a = rng.i32(-100_000_000..100_000_000) as f32;
        let b = rng.i32(-100_000_000..100_000_000) as f32;
        let phase = f64::from(a).rem_euclid(11.0);
        let displacement = f64::from(circular_reference(b, a, 11.0));
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let expected = (phase + displacement * f64::from(t)).rem_euclid(11.0) as f32;
            assert_eq!(space.lerp_clamp(a, b, t), expected);
        }
    }
    // A finite, very large extrapolation must wrap instead of producing NaN.
    let wide = CircularComparativeSpace { period: f32::MAX };
    assert!(wide.lerp_clamp(f32::MAX * 0.75, 0.0, f32::MAX).is_finite());
}

#[test]
fn circular_interpolation_matches_exact_integer_and_subnormal_references() {
    let space = CircularComparativeSpace { period: 11.0 };
    assert_eq!(space.lerp_clamp(1.0, 2.0, 100_000_000.0), 2.0);
    let rng = fastrand::Rng::with_seed(0x5ca1_e0ff);
    // The operands and large t values are integer-valued f32 numbers. Integer
    // modular arithmetic is independent of the implementation's f64 remainder.
    for _ in 0..10_000 {
        let a = rng.i32(-100_000_000..100_000_000) as f32;
        let b = rng.i32(-100_000_000..100_000_000) as f32;
        let t = rng.u64(0..1_u64 << 60) as f32;
        let start = (a as i128).rem_euclid(11);
        let end = (b as i128).rem_euclid(11);
        let displacement = (end - start + 5).rem_euclid(11) - 5;
        for t in [t, -t] {
            let expected = (start + displacement * t as i128).rem_euclid(11) as f32;
            assert_eq!(space.lerp_clamp(a, b, t), expected, "a={a}, b={b}, t={t}");
        }
    }
    // In this scale every coordinate is an integer count of the minimum
    // subnormal. Small quarter-turn t values permit an exact integer oracle,
    // including ties-to-even when the interpolated count is fractional.
    for period_units in [3_i64, 5, 11, 127] {
        let space = CircularComparativeSpace {
            period: f32::from_bits(period_units as u32),
        };
        for start_units in 0..period_units {
            for end_units in 0..period_units {
                let displacement = (end_units - start_units + period_units / 2)
                    .rem_euclid(period_units)
                    - period_units / 2;
                for quarter_turns in [0_i64, 1, 2, 3, 4, 100_000_000] {
                    let numerator = (start_units * 4 + displacement * quarter_turns)
                        .rem_euclid(period_units * 4);
                    let integer = numerator / 4;
                    let remainder = numerator % 4;
                    let rounded =
                        integer + i64::from(remainder > 2 || (remainder == 2 && integer % 2 != 0));
                    let canonical = if rounded == period_units { 0 } else { rounded };
                    let observed = space.lerp_clamp(
                        f32::from_bits(start_units as u32),
                        f32::from_bits(end_units as u32),
                        quarter_turns as f32 / 4.0,
                    );
                    assert_eq!(
                        observed.to_bits(),
                        canonical as u32,
                        "period_units={period_units}, start={start_units}, end={end_units}, quarters={quarter_turns}"
                    );
                }
            }
        }
    }
}

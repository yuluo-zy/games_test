//! Behavioral regressions and invariants for compact storage and numeric helpers.
use std::{
    collections::VecDeque,
    panic::{AssertUnwindSafe, catch_unwind},
};

use glam::{Vec2, Vec3};
use utils::{
    angle_range::AngleRange,
    circular_f32::CircularF32,
    comparative_space::{CircularComparativeSpace, ComparativeSpace},
    easing_functions::EasingFunction,
    float_grid::{ByteFloatGrid, FloatGridSampler},
    halver::Halver,
    index_condenser::IndexCondenser,
    loop_range::{LoopIdx, LoopRange},
    selective_deserialize::{SaveHistorySnapshots, SelectiveDeserialize, SnapshotsField},
    subtractive_1d_boolean::{Interval, SubtractiveBoolean1D},
    threshold_range::threshold_range,
    timer::FilteredWallTime,
};

#[test]
fn loop_ranges_agree_with_remaining_iteration_and_checked_slicing() {
    let source = [10, 20, 30, 40];
    let range = LoopRange::new_loop(-1, 4, 4);
    let mut view = range.map_slice(&source);
    assert_eq!(view.len(), 5);
    assert_eq!(view[0], 40);
    assert_eq!(view.next(), Some(&40));
    assert_eq!(view.next_back(), Some(&40));
    assert_eq!(view.len(), 3);
    assert_eq!(view.size_hint(), (3, Some(3)));
    assert_eq!(view.collect::<Vec<_>>(), [&10, &20, &30]);
    assert!(range.subslice(5..5).is_empty());
    assert!(LoopRange::new_line(0, 0, 4).subslice(0..0).is_empty());
    let reversed = std::ops::Range { start: 3, end: 1 };
    assert!(catch_unwind(|| range.subslice(reversed)).is_err());
    assert!(catch_unwind(|| range.map_slice(&source)[5]).is_err());
    assert!(catch_unwind(|| range.map_slice(&source)[usize::MAX]).is_err());
    let reversed = LoopRange {
        start_idx: 3,
        end_idx: 1,
        period: 4,
    };
    assert_eq!((reversed.len(), reversed.size_hint()), (0, (0, Some(0))));
    assert!(reversed.is_empty());
}

#[test]
fn loop_arithmetic_does_not_wrap_signed_lengths() {
    let a = LoopIdx {
        idx: i32::MAX,
        period: 3,
    };
    let b = LoopIdx {
        idx: i32::MIN,
        period: 3,
    };
    assert_eq!(a - b, u32::MAX as i64);
    let range = LoopRange {
        start_idx: i32::MIN,
        end_idx: i32::MAX,
        period: 3,
    };
    assert_eq!(range.len(), u32::MAX as usize);
    assert_eq!(range.size_hint(), (range.len(), Some(range.len())));
    assert!(catch_unwind(|| LoopRange::new_loop(0, 1, 0)).is_err());
    assert!(catch_unwind(|| LoopRange::new_line(0, 1, -1)).is_err());
    assert_eq!(
        LoopRange::new_line(0, i32::MAX, i32::MAX).len(),
        i32::MAX as usize
    );
}

#[test]
fn dense_handles_follow_keys_through_removal_clear_and_reinsertion() {
    let mut indices = IndexCondenser::default();
    let mut handles: Vec<_> = (0..32).map(|key| indices.insert(key).unwrap()).collect();
    assert!(indices.insert(1).is_none());
    for key in (0..32).step_by(2) {
        assert!(indices.remove(&mut handles[key]));
        assert_eq!(indices.get(&handles[key]), None);
        for (index, key) in indices.keys().iter().enumerate() {
            assert_eq!(indices.get(&handles[*key]), Some(index as u32));
        }
    }
    indices.clear();
    assert!(handles.iter().all(|handle| indices.get(handle).is_none()));
    indices.insert(7).unwrap();
    assert_eq!(indices.get(&handles[7]), Some(0));
}

#[test]
fn circular_displacements_cover_multiple_turns_and_preserve_half_turn_ties() {
    let circle = CircularComparativeSpace { period: 10.0 };
    for a in -40..=40 {
        for b in -40..=40 {
            let expected = ((a - b + 5) as f32).rem_euclid(10.0) - 5.0;
            assert_eq!(circle.subtract(a as f32, b as f32), expected);
            assert_eq!(circle.distance(a as f32, b as f32), expected.abs());
        }
    }
    assert!(circle.distance(f32::MAX, -f32::MAX) <= 5.0);
    assert_eq!(circle.subtract(5.0, 0.0), -5.0);
    assert_eq!(circle.subtract(-5.0, 0.0), -5.0);
    assert_eq!(circle.lerp_clamp(39.0, -19.0, 0.5), 0.0);
}

#[test]
fn grid_sampling_preserves_centered_edges_and_legacy_extrapolation() {
    // A horizontal ramp gives directly calculable bilinear samples.
    let grid = ByteFloatGrid {
        values: &[0, 255, 0, 255],
        width: 2,
        height: 2,
    };
    for (x, centered, expected) in [
        (-1.0, true, 0.0),
        (0.0, true, 0.5),
        (1.0, true, 1.0),
        (-1.0, false, 0.0),
        (0.0, false, 1.0),
        (1.0, false, 2.0),
    ] {
        assert_eq!(
            grid.bilinear_sample_at_world_xz(Vec2::new(x, 0.0), Vec2::splat(2.0), centered),
            expected
        );
    }
}

#[test]
fn clipping_preserves_order_through_repeated_split_and_cover_operations() {
    let mut intervals = SubtractiveBoolean1D::new_range(0.0, 10.0);
    let cut = |start, end| Interval {
        start,
        end,
        upper_exclusive: false,
    };
    intervals.subtract(cut(2.0, 3.0));
    intervals.subtract(cut(5.0, 7.0));
    assert_eq!(
        intervals.result(),
        [cut(0.0, 2.0), cut(3.0, 5.0), cut(7.0, 10.0)]
    );
    // 此非退化洞重复扣除后仍保留三段。
    intervals.subtract(cut(2.0, 3.0));
    assert_eq!(intervals.result().len(), 3);
    intervals.subtract(cut(-1.0, 11.0));
    assert!(intervals.result().is_empty());
    intervals.subtract(cut(1.0, 2.0));
    assert!(intervals.result().is_empty());
}

#[test]
fn threshold_runs_partition_every_index_and_keep_nan_boundaries() {
    let values = [0.0, 1.0, 2.0, f32::NAN, 3.0, -1.0];
    let mut low = Vec::new();
    let mut high = Vec::new();
    threshold_range(
        &values,
        1.0,
        |range| low.push(range),
        |range| high.push(range),
    );
    assert_eq!(low, [0..2, 5..6]);
    assert_eq!(high, [2..3, 3..5]);
    let mut all: Vec<_> = low.into_iter().chain(high).collect();
    all.sort_by_key(|range| range.start);
    assert_eq!(
        all.into_iter().flatten().collect::<Vec<_>>(),
        (0..values.len()).collect::<Vec<_>>()
    );
}

#[test]
fn selective_decode_keeps_last_complete_value_and_skips_unrelated_payloads() {
    let parse = |text| {
        serde_json::from_str::<SelectiveDeserialize<SnapshotsField, Vec<u32>>>(text)
            .unwrap()
            .value
    };
    assert_eq!(
        parse(r#"{"ignored":{"large":[1,2,3]},"snapshots":[4,5]}"#),
        [4, 5]
    );
    assert_eq!(parse(r#"{"snapshots":[1],"snapshots":[2]}"#), [2]);
    assert!(parse(r#"{"other":123}"#).is_empty());
    // Deserialize directly: from_str independently checks for trailing text.
    let mut decoder = serde_json::Deserializer::from_str(r#"{"snapshots":[4],"broken":"#);
    let decoded: SelectiveDeserialize<SnapshotsField, Vec<u32>> =
        serde::Deserialize::deserialize(&mut decoder).unwrap();
    assert_eq!(decoded.value, [4]);
    let nested: SaveHistorySnapshots<Vec<u32>> =
        serde_json::from_str(r#"{"History":{"snapshots":[7]}}"#).unwrap();
    assert_eq!(nested.value.value, [7]);
}

#[test]
fn resumed_timer_bounds_history_and_ignores_tiny_deltas() {
    let mut timer = FilteredWallTime::from_observed_state(-1, 0.02, VecDeque::from([0.02; 9]));
    assert_eq!(timer.delta_secs(-1.0), 0.0);
    assert_eq!(timer.samples().len(), 9);
    assert!((timer.delta_secs(0.02) - 0.02).abs() < f32::EPSILON);
    assert_eq!(timer.samples().len(), 4);
    let previous = timer.previous_delta;
    assert_eq!(timer.delta_secs(f32::NAN), 0.0);
    assert_eq!(timer.previous_delta, previous);
    let mut fresh = FilteredWallTime::default();
    for _ in 0..20 {
        let previous = fresh.previous_delta;
        assert!(fresh.delta_secs(1.0) <= previous * 1.2);
        assert!(fresh.samples().len() <= 4);
    }
}

#[test]
fn angle_easing_and_half_conversion_have_independent_basic_contracts() {
    for bias in [0.0, 0.25, 0.5, 1.0] {
        assert_eq!(EasingFunction::Bias(bias).apply(0.0), 0.0);
        assert_eq!(EasingFunction::Bias(bias).apply(1.0), 1.0);
    }

    let circle = CircularF32::new(-std::f32::consts::FRAC_PI_2);
    assert!((circle.as_f32() - 3.0 * std::f32::consts::FRAC_PI_2).abs() < 1e-6);
    assert!((circle.distance(std::f32::consts::FRAC_PI_2) - std::f32::consts::PI).abs() < 1e-6);
    for length in [-1.0, f32::NAN, f32::INFINITY, std::f32::consts::TAU + 1.0] {
        assert!(catch_unwind(|| AngleRange::from_min_and_len(0.0, length)).is_err());
    }
    assert!(catch_unwind(|| AngleRange::from_min_and_len(f32::NAN, 1.0)).is_err());
    let range = AngleRange::from_min_and_len(0.0, 1.0);
    for (input, expected) in [(0.5, 0.5), (1.5, 1.0), (-0.5, 0.0)] {
        assert_eq!(range.clamp(input), expected);
    }
    for easing in [
        EasingFunction::CircularInOut,
        EasingFunction::CubicInOut,
        EasingFunction::SineInOut,
        EasingFunction::CubicOut,
    ] {
        assert!(easing.apply(0.0).abs() < 1e-6);
        assert!((easing.apply(1.0) - 1.0).abs() < 1e-6);
        let samples: Vec<_> = (0..=100).map(|i| easing.apply(i as f32 / 100.0)).collect();
        assert!(samples.windows(2).all(|pair| pair[0] <= pair[1]));
    }
    assert_eq!(
        Vec3::new(0.0, 1.0, -2.0)
            .to_f16()
            .map(|value| value.to_bits()),
        [0, 0x3c00, 0xc000]
    );
}

#[test]
fn hashing_random_and_coordinate_adapters_preserve_their_data_contracts() {
    use utils::{
        name_hash::NameHash,
        rgb9e5::pack_rgb9e5,
        rng::{init_rng, init_rng_u64, rng_from_seed_u64},
        swizzlers::{Vec3SliceAs2, project_vec3_slice_xz},
    };
    assert_eq!(NameHash::compute("").0, 0x2d06_8005_38d3_94c2);
    assert_eq!(NameHash::compute("岩石"), NameHash::compute("岩石"));
    assert_ne!(NameHash::compute("Rock"), NameHash::compute("rock"));
    assert_ne!(NameHash::compute("é"), NameHash::compute("e\u{301}"));
    for seed in [0, 1, u32::MAX] {
        assert_eq!(init_rng(seed), init_rng_u64(seed.into()));
        let a = rng_from_seed_u64(seed.into());
        let b = rng_from_seed_u64(seed.into());
        let sequence_a: Vec<_> = (0..32).map(|_| a.u64(..)).collect();
        let sequence_b: Vec<_> = (0..32).map(|_| b.u64(..)).collect();
        assert_eq!(sequence_a, sequence_b);
        assert!(sequence_a.windows(2).any(|pair| pair[0] != pair[1]));
    }
    assert_eq!(pack_rgb9e5(Vec3::ZERO), 0);
    assert_eq!(
        pack_rgb9e5(Vec3::ONE),
        (16 << 27) | (256 << 18) | (256 << 9) | 256
    );
    assert_eq!(pack_rgb9e5(Vec3::splat(f32::INFINITY)), u32::MAX);
    assert_eq!(pack_rgb9e5(Vec3::splat(-1.0)), 0);
    let source = [Vec3::new(1.0, 9.0, 2.0), Vec3::new(3.0, 8.0, 4.0)];
    assert_eq!(
        project_vec3_slice_xz(&source),
        [Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]
    );
    let view = source.as_slice().as_vec2();
    assert_eq!(view.as_source().as_ptr(), source.as_ptr());
    assert_eq!(view.len(), source.len());
    assert!(!view.is_empty());
}

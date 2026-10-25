//! WC-C constant/formula tests for `arch_walker::ArchWalker`.
//!
//! Contract: `ArchWalker::new` (0x14074c2d0) stores `+0x08 = brick_len` and the
//! constant `+0x0c = 0.1` tail ratio. `walk` steps by
//! `n = max(1, floor(total/brick_len + 0.5))`, `step = total/n`, and appends a
//! tail brick when `step * 0.1 < residual`.
use glam::Vec3;
use system_wall_constructor::arch_walker::ArchWalker;

#[test]
fn new_stores_brick_len_and_constant_tail_ratio() {
    let walker = ArchWalker::new(7, 0.5, f32::from_bits(0x40499999));
    assert_eq!(walker.kind, 7);
    assert_eq!(walker.extra, 0.5);
    assert_eq!(walker.brick_len.to_bits(), 0x40499999);
    assert_eq!(walker.tail_ratio.to_bits(), 0.1f32.to_bits());
}

/// 10 m / brick_len 2 -> n = 5, step = 2; the five samples are 2,4,6,8,10.
#[test]
fn brick_len_sets_even_step_count_and_positions() {
    let points = vec![Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)];
    let heights = [0.0f32, 0.0];
    let us = [0.0f32, 1.0];
    let walker = ArchWalker::new(0, 0.0, 2.0);
    let mut samples = Vec::new();
    walker.walk(&heights, &points, &us, |point, u| {
        samples.push((point, u));
    });
    assert_eq!(samples.len(), 5);
    for (index, (point, _)) in samples.iter().enumerate() {
        assert_eq!(point.x, 2.0 * (index + 1) as f32);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }
    assert_eq!(samples[4].1, 1.0);
}

/// stop 3.15 (bits 0x40499999): 6.3 m -> n = 2, step = 3.15.
#[test]
fn step_uses_3_1499999_brick_len() {
    let points = vec![Vec3::ZERO, Vec3::new(6.3, 0.0, 0.0)];
    let heights = [0.0f32, 0.0];
    let us = [0.0f32, 1.0];
    let walker = ArchWalker::new(0, 0.0, f32::from_bits(0x40499999));
    let mut xs = Vec::new();
    walker.walk(&heights, &points, &us, |point, _| xs.push(point.x));
    assert_eq!(xs.len(), 2);
    assert!((xs[0] - 3.15).abs() < 1e-5, "x0={}", xs[0]);
    assert!((xs[1] - 6.3).abs() < 1e-5, "x1={}", xs[1]);
}

/// A shorter `us` array bounds the walk; the leftover distance is larger than
/// `step * 0.1`, so exactly one tail sample is emitted at the last bounded
/// point with `us[last]` (0.1 tail ratio).
#[test]
fn tail_brick_fires_when_residual_exceeds_tenth_of_step() {
    let points = vec![
        Vec3::ZERO,
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
    ];
    let heights = [0.0f32, 0.0, 0.0];
    // us runs out after the second point, so the walk stops with residual 5 m
    // against step 10 m: 10 * 0.1 = 1 < 5 -> tail.
    let us = [0.0f32, 0.5];
    let walker = ArchWalker::new(0, 0.0, 10.0);
    let mut samples = Vec::new();
    walker.walk(&heights, &points, &us, |point, u| {
        samples.push((point, u));
    });
    assert_eq!(samples.len(), 1);
    assert_eq!(samples[0].0, Vec3::new(5.0, 0.0, 0.0));
    assert_eq!(samples[0].1.to_bits(), 0.5f32.to_bits());
}

/// No tail when the residual is exactly zero.
#[test]
fn no_tail_when_walk_consumes_the_whole_curve() {
    let points = vec![Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)];
    let heights = [0.0f32, 0.0];
    let us = [0.0f32, 1.0];
    let walker = ArchWalker::new(0, 0.0, 2.0);
    let mut count = 0usize;
    walker.walk(&heights, &points, &us, |_, _| count += 1);
    assert_eq!(count, 5);
}

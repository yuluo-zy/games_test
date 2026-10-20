//! WC-C byte-level contract tests for 96B `BrickTransformSsbo` records produced
//! by `place_pillars::assemble_stone_pillar`.
//!
//! Contract: `evidence/round11/contracts/wc-constructor-CONTRACT.md` §2.2 #9.
//! Layout asserted here (matches `country-core::geometry::instanced_wall::
//! BrickTransformSsbo::observed_bytes`):
//!
//! | offset | field |
//! |---|---|
//! | 0..12  | affine column 0 (x axis) |
//! | 12..24 | affine column 1 (y axis) |
//! | 24..36 | affine column 2 (z axis) |
//! | 36..48 | affine column 3 (translation/center) |
//! | 48..64 | deformation (zero) |
//! | 64..76 | surface (sentinel -100) |
//! | 76..80 | seed |
//! | 80..84 | material |
//! | 84..92 | wall_distance[2] |
//! | 92..96 | flags |
use system_wall_constructor::place_pillars::{
    assemble_stone_pillar, pillar_segments, PillarEmitContext,
};

const CONTEXT: PillarEmitContext = PillarEmitContext {
    material: 3,
    flags: 0,
    wall_distance: [0.25, 0.75],
};

fn f32_at(bytes: &[u8; 96], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn u32_at(bytes: &[u8; 96], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn record(
    x: [f32; 3],
    y: [f32; 3],
    z: [f32; 3],
    center: [f32; 3],
    seed: u32,
    material: u32,
    wall_distance: [f32; 2],
    flags: u32,
) -> [u8; 96] {
    let mut out = [0u8; 96];
    for (column, values) in [x, y, z, center].into_iter().enumerate() {
        for (component, value) in values.into_iter().enumerate() {
            let offset = column * 12 + component * 4;
            out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    // 48..64 deformation stays zero.
    for surface in 0..3 {
        let offset = 64 + surface * 4;
        out[offset..offset + 4].copy_from_slice(&(-100.0f32).to_le_bytes());
    }
    out[76..80].copy_from_slice(&seed.to_le_bytes());
    out[80..84].copy_from_slice(&material.to_le_bytes());
    out[84..88].copy_from_slice(&wall_distance[0].to_le_bytes());
    out[88..92].copy_from_slice(&wall_distance[1].to_le_bytes());
    out[92..96].copy_from_slice(&flags.to_le_bytes());
    out
}

/// Contract constant 1.26: `param_3 += 0.2; param_3 <= 1.26` selects one span.
#[test]
fn pillar_segments_split_boundary_is_126_after_point_two_offset() {
    // 1.05 + 0.2 = 1.25 <= 1.26 -> single span [0, top+0.2].
    let one = pillar_segments(1.05, 10.0);
    assert_eq!(one.len(), 1);
    assert_eq!(one[0].0, 0.0f32);
    assert_eq!(one[0].1, 10.0f32 + 0.2f32);
    assert!(!one[0].2);

    // Exactly on the boundary also stays single.
    let boundary = pillar_segments(1.06, 10.0);
    assert_eq!(boundary.len(), 1, "1.06 + 0.2 == 1.26 must not split");

    // 1.07 + 0.2 = 1.27 > 1.26 -> two spans, first flagged.
    let bottom = 1.07f32 + 0.2f32;
    let two = pillar_segments(1.07, 10.0);
    assert_eq!(two.len(), 2);
    assert_eq!(two[0], (0.0, bottom, true));
    assert_eq!(two[1].0, bottom);
    assert_eq!(two[1].1, 10.0f32 + 0.2f32);
    // |10.2 - 1.27| > 1.2 -> second span keeps flag (0.25 gap).
    assert!(two[1].2);

    // |2.4 - 1.27| = 1.13 <= 1.2 -> second span has no gap.
    let two_small = pillar_segments(1.07, 2.2);
    assert_eq!(two_small.len(), 2);
    assert!(two_small[0].2);
    assert!(!two_small[1].2);
}

/// Single-span pillar: full 96B records with exact offsets/values.
///
/// `bottom + 0.2 = 0.2 <= 1.26`, `span = 1.0`, `count = max(3, round(1.2/0.56))
/// = 3` -> one randomized split, two emitted bricks. The randomized split and
/// the 0.42/0.7 width jitter are independently recomputed from the documented
/// `random_splits_into` + `jitter_widths` formulas with the same fastrand seed.
#[test]
fn assemble_single_span_writes_exact_96b_records() {
    let seed = 1234u64;
    let mut out = Vec::new();
    assemble_stone_pillar(1.5, 2.0, -3.5, 0.0, 1.0, seed, CONTEXT, &mut out);
    assert_eq!(out.len(), 2);

    // Recompute the documented random sequence.
    let rng = fastrand::Rng::with_seed(seed);
    let span = 1.2f32;
    let spacing = 1.0f32 / 2.0f32; // count - 1 = 2
    let jitter = (spacing * 0.495f32).min(f32::from_bits(0x3e8f5c29)); // 0.28
    let split = ((rng.f32() + (-0.5f32)) * jitter) + 0.5f32;
    let first_high = split * span + 0.0f32;
    let a0 = rng.f32();
    let b0 = rng.f32();
    let width_x0 = (a0 * 0.05f32 - (1.0f32 - a0) * 0.05f32) + f32::from_bits(0x3ed70a3e); // 0.42
    let width_z0 = (b0 * 0.05f32 - (1.0f32 - b0) * 0.05f32) + f32::from_bits(0x3f333333); // 0.70

    let y_origin = 2.0f32 + f32::from_bits(0xbe4ccccd); // -0.2
    let expected0 = record(
        [width_x0, 0.0, 0.0],
        [0.0, first_high, 0.0],
        [0.0, 0.0, width_z0],
        [1.5, y_origin + (0.0f32 + first_high) * 0.5, -3.5],
        seed as u32,
        CONTEXT.material,
        CONTEXT.wall_distance,
        CONTEXT.flags,
    );
    assert_eq!(out[0], expected0);
    // First brick bottom edge sits at param y - 0.2.
    assert_eq!(
        f32_at(&out[0], 40) - f32_at(&out[0], 16) * 0.5,
        2.0f32 + f32::from_bits(0xbe4ccccd)
    );
    // 0.42 base with +/-0.05 jitter.
    assert!((0.37..=0.47).contains(&f32_at(&out[0], 0)));
    assert!((0.65..=0.75).contains(&f32_at(&out[0], 8)));
    // 48..76 sentinels and word fields.
    assert_eq!(&out[0][48..64], &[0u8; 16]);
    assert_eq!(f32_at(&out[0], 64), -100.0);
    assert_eq!(f32_at(&out[0], 68), -100.0);
    assert_eq!(f32_at(&out[0], 72), -100.0);
    assert_eq!(u32_at(&out[0], 76), seed as u32);
    assert_eq!(u32_at(&out[0], 80), 3);
    assert_eq!(f32_at(&out[0], 84), 0.25);
    assert_eq!(f32_at(&out[0], 88), 0.75);
    assert_eq!(u32_at(&out[0], 92), 0);

    // Second (last) split consumes the next two width draws.
    let a1 = rng.f32();
    let b1 = rng.f32();
    let width_x1 = (a1 * 0.05f32 - (1.0f32 - a1) * 0.05f32) + f32::from_bits(0x3ed70a3e);
    let width_z1 = (b1 * 0.05f32 - (1.0f32 - b1) * 0.05f32) + f32::from_bits(0x3f333333);
    let last_high = 1.0f32 * span + 0.0f32;
    let expected1 = record(
        [width_x1, 0.0, 0.0],
        [0.0, last_high - first_high, 0.0],
        [0.0, 0.0, width_z1],
        [1.5, y_origin + (first_high + last_high) * 0.5, -3.5],
        seed as u32 + 1,
        CONTEXT.material,
        CONTEXT.wall_distance,
        CONTEXT.flags,
    );
    assert_eq!(out[1], expected1);
    // Top edge reaches param y + top exactly (y - 0.2 + top + 0.2).
    assert_eq!(
        f32_at(&out[1], 40) + f32_at(&out[1], 16) * 0.5,
        2.0f32 + 1.0f32
    );
    // No 2.1 cap for this pillar.
    assert!(out.iter().all(|brick| u32_at(brick, 76) < seed as u32 + 2));
}

/// Two-span pillar with `bottom + 0.2 = 2.2 > 2.1`.
///
/// Pins the flagged-span tail width pair (0.7 / 0.84) and the exact top cap
/// record (0.72 / 0.5 / 0.85 at `y - 0.45`).
#[test]
fn assemble_two_span_sets_flag_tail_widths_and_top_cap() {
    let context = PillarEmitContext {
        material: 7,
        flags: 5,
        wall_distance: [1.0, 2.0],
    };
    let seed = 99u64;
    let mut out = Vec::new();
    assemble_stone_pillar(1.5, 2.0, -3.5, 2.0, 3.0, seed, context, &mut out);
    // First span: span = 2.2 - 0.25 = 1.95 -> round(1.95/0.56) = 3 -> 4 heights
    // -> 3 bricks. Second span: span = 1.0 -> 3 splits -> 2 bricks. Plus cap.
    assert_eq!(out.len(), 6);

    // The last pair of the flagged span uses the fixed 0.7 / 0.84 section.
    assert_eq!(u32_at(&out[2], 0), 0x3f333333); // 0.7
    assert_eq!(u32_at(&out[2], 8), 0x3f570a3e); // 0.84
    let y_origin = 2.0f32 + f32::from_bits(0xbe4ccccd);
    // The flagged span still ends at its own top (bottom+0.2), not at the gap.
    assert_eq!(
        f32_at(&out[2], 40) + f32_at(&out[2], 16) * 0.5,
        y_origin + (2.0f32 + 0.2f32)
    );

    // Unflagged span uses the 0.42+/-0.05 / 0.70+/-0.05 jitter family.
    for brick in &out[3..5] {
        assert!((0.37..=0.47).contains(&f32_at(brick, 0)));
        assert!((0.65..=0.75).contains(&f32_at(brick, 8)));
    }

    // Top cap: exact deterministic 96B record, center y = (y-0.2) - 0.25.
    let expected_cap = record(
        [f32::from_bits(0x3f3851ec), 0.0, 0.0], // 0.72
        [0.0, f32::from_bits(0x3f000000), 0.0], // 0.5
        [0.0, 0.0, f32::from_bits(0x3f59999a)], // 0.85
        [1.5, y_origin - 0.25f32, -3.5],
        seed as u32 + 5,
        7,
        [1.0, 2.0],
        5,
    );
    assert_eq!(out[5], expected_cap);
}

//! WC-C byte/constant contracts for `segmentation::add_splits_in_empty_range`
//! (0x140751240).
//!
//! Contract (`wc-constructor-CONTRACT.md` §2 / `calculate_wall_path_segmentation.rs`):
//! * `n = floor(curve.length / spacing)`; `n == 1 && length < spacing * 1.5`
//!   is dropped;
//! * for `i in 1..=n`, `t = i / (n + 1)` (not `i / n`);
//! * the nearest mask bucket is `base + segment + (fraction > 0.5)`;
//! * the bucket flag only ORs bit 3 (`flags[index] |= 8`);
//! * `coords[index]` is the 8B `{point_index u32, fraction f32}` record.
use system_wall_constructor::segmentation::calculate_wall_path_segmentation::{
    WallPathSegmentationMaskCoord, add_splits_in_empty_range,
};

fn coords(count: usize) -> Vec<WallPathSegmentationMaskCoord> {
    vec![
        WallPathSegmentationMaskCoord {
            point_index: 0,
            fraction: 0.0,
        };
        count
    ]
}

fn f32_at(coord: &WallPathSegmentationMaskCoord) -> f32 {
    coord.fraction
}

/// `t = i/(n+1)`: a 6 m line with spacing 1 gives n=6 and interior splits at
/// 1/7 .. 6/7. Buckets 0/1 are selected by `fraction > 0.5`.
#[test]
fn splits_use_i_over_n_plus_one_and_bucket_by_fraction() {
    let points = [[0.0, 0.0, 0.0], [6.0, 0.0, 0.0]];
    let mut flags = vec![0u8; 8];
    let mut mask_coords = coords(8);
    add_splits_in_empty_range(0, &points, &mut flags, &mut mask_coords, 1.0);

    // Only bit 3 is set, and only on the two endpoint buckets.
    assert_eq!(flags, vec![8, 8, 0, 0, 0, 0, 0, 0]);
    // Bucket 0 keeps the final write (i = 3 -> 3/7), bucket 1 keeps i = 6.
    assert_eq!(mask_coords[0].point_index, 0);
    assert_eq!(mask_coords[0].fraction.to_bits(), (3.0f32 / 7.0f32).to_bits());
    assert_eq!(mask_coords[1].point_index, 0);
    assert_eq!(mask_coords[1].fraction.to_bits(), (6.0f32 / 7.0f32).to_bits());
    // The 8B projection is little-endian {u32 point_index, f32 fraction}.
    assert_eq!(
        mask_coords[1].observed_bytes(),
        {
            let mut bytes = [0u8; 8];
            bytes[..4].copy_from_slice(&0u32.to_le_bytes());
            bytes[4..].copy_from_slice(&(6.0f32 / 7.0f32).to_le_bytes());
            bytes
        }
    );
    // No i/(n) = 1/6 split was produced.
    assert!(mask_coords.iter().all(|c| c.fraction.to_bits() != (1.0f32 / 6.0f32).to_bits()));
}

/// Existing flag bits survive; bit 3 is ORed, never overwritten.
#[test]
fn splits_only_or_flag_bit_three() {
    let points = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let mut flags = vec![0u8; 4];
    flags[0] = 0b0000_0001;
    flags[1] = 0b0000_0010;
    let mut mask_coords = coords(4);
    // n = 2 -> t = 1/3 (bucket 0) and 2/3 (bucket 1).
    add_splits_in_empty_range(0, &points, &mut flags, &mut mask_coords, 1.0);
    assert_eq!(flags[0], 0b0000_1001);
    assert_eq!(flags[1], 0b0000_1010);
    assert_eq!(mask_coords[0].fraction.to_bits(), (1.0f32 / 3.0f32).to_bits());
    assert_eq!(mask_coords[1].fraction.to_bits(), (2.0f32 / 3.0f32).to_bits());
    assert!(f32_at(&mask_coords[0]) < 0.5);
    assert!(f32_at(&mask_coords[1]) > 0.5);
}

/// `base` shifts every written bucket without changing local curve coordinates.
#[test]
fn splits_apply_base_offset_to_bucket_index() {
    let points = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let mut flags = vec![0u8; 8];
    let mut mask_coords = coords(8);
    add_splits_in_empty_range(5, &points, &mut flags, &mut mask_coords, 1.0);
    assert_eq!(flags, vec![0, 0, 0, 0, 0, 8, 8, 0]);
    assert_eq!(mask_coords[5].fraction.to_bits(), (1.0f32 / 3.0f32).to_bits());
    assert_eq!(mask_coords[6].fraction.to_bits(), (2.0f32 / 3.0f32).to_bits());
}

/// `n == 1 && length < spacing * 1.5` drops the only split.
#[test]
fn single_short_split_is_skipped_below_fifteen_spacings() {
    let points = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0]];
    let mut flags = vec![0u8; 4];
    let mut mask_coords = coords(4);
    // n = 1, spacing * 1.5 = 15 > 10 -> skip.
    add_splits_in_empty_range(0, &points, &mut flags, &mut mask_coords, 10.0);
    assert_eq!(flags, vec![0u8; 4]);
    assert!(mask_coords.iter().all(|c| c.fraction == 0.0));

    // n = 1, spacing * 1.5 = 9.9 < 10 -> emit t = 1/2 into bucket 0.
    let mut flags = vec![0u8; 4];
    let mut mask_coords = coords(4);
    add_splits_in_empty_range(0, &points, &mut flags, &mut mask_coords, 6.6);
    assert_eq!(flags, vec![8, 0, 0, 0]);
    assert_eq!(mask_coords[0].fraction, 0.5);
}

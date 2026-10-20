//! WC-C byte-level contract tests for 64B `WoodenPlankSsbo` output.
//!
//! Two producers are covered:
//!
//! * `construct_lintels::place_support_beams::place_support_beams` (0x141242530
//!   chain) writes affine columns in column-major order:
//!   `[x_axis, y_axis, z_axis, translation]`, then `ordinal@48`, `source@52`,
//!   `wall_id@56`, `flags@60`.
//! * `construct_fences::process_segment::emit_fences` (0x141238d80 chain) uses
//!   the frozen row-major `Affine3Packed` packer: rows
//!   `[x_axis, y_axis, z_axis, translation]` at 0/12/24/36, same metadata at
//!   48/52/56/60.
//!
//! Constants pinned here: support step limit 3.1499999, diagonal budget 0.85,
//! diagonal axis thickness 0.2625, diagonal horizontal extension
//! 0.1125 + 0.1875, plank thickness 0.225 / 0.3375, fence post 0.3/+0.1/-0.1
//! and rail thicknesses 0.1 / 0.2.
use country_core::resources::walls::publication_fields::WallId as PublicWallId;
use country_core::systems::wall::construction::fences::FenceSegmentInput;
use glam::Vec3;
use system_wall_constructor::construct_fences::process_segment::emit_fences;
use system_wall_constructor::construct_lintels::place_support_beams::{
    place_support_beams, SupportBeamCoord, SupportBeamGrower, SupportBeamSource,
};

fn f32_at(bytes: &[u8; 64], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn u32_at(bytes: &[u8; 64], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn column_major_record(
    x: [f32; 3],
    y: [f32; 3],
    z: [f32; 3],
    t: [f32; 3],
    ordinal: u32,
    source: u32,
    wall_id: u32,
    flags: u32,
) -> [u8; 64] {
    let mut out = [0u8; 64];
    for (column, values) in [x, y, z, t].into_iter().enumerate() {
        for (component, value) in values.into_iter().enumerate() {
            let offset = column * 12 + component * 4;
            out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    out[48..52].copy_from_slice(&ordinal.to_le_bytes());
    out[52..56].copy_from_slice(&source.to_le_bytes());
    out[56..60].copy_from_slice(&wall_id.to_le_bytes());
    out[60..64].copy_from_slice(&flags.to_le_bytes());
    out
}

fn column(bytes: &[u8; 64], index: usize) -> Vec3 {
    Vec3::new(
        f32_at(bytes, index * 12),
        f32_at(bytes, index * 12 + 4),
        f32_at(bytes, index * 12 + 8),
    )
}

fn beams_source(heights: [f32; 4]) -> SupportBeamSource {
    SupportBeamSource {
        heights: heights.to_vec(),
        ground_positions: vec![
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [6.0, 0.0, 0.0],
        ],
        raised_positions: vec![
            [0.0, heights[0], 0.0],
            [2.0, heights[1], 0.0],
            [4.0, heights[2], 0.0],
            [6.0, heights[3], 0.0],
        ],
        flags: vec![0, 0, 0, 0],
        source_coords: vec![
            SupportBeamCoord {
                point_index: 0,
                fraction: 0.0,
            },
            SupportBeamCoord {
                point_index: 1,
                fraction: 0.0,
            },
            SupportBeamCoord {
                point_index: 2,
                fraction: 0.0,
            },
            SupportBeamCoord {
                point_index: 2,
                fraction: 1.0,
            },
        ],
    }
}

/// Two symmetric 2 m grower halves (no join) emit one 3.15 m-step plank per
/// half plus one 0.85-budget diagonal per side. The fixture exercises the
/// 3.1499999 cap indirectly (each grower stops once the next point would make
/// the accumulated length 4.0).
#[test]
fn place_support_beams_writes_exact_64b_planks_and_diagonals() {
    let source = beams_source([5.0, 5.0, 5.0, 5.0]);
    let mut out = Vec::new();
    place_support_beams(&source, 77, 3, &mut out);
    assert_eq!(out.len(), 4, "two planks then two diagonals");

    // Plank 1: start (0,5,0) -> (2,5,0); plank bottom at height - 0.225.
    let plank_y = 5.0f32 + f32::from_bits(0xbe666667);
    let expected_plank = column_major_record(
        [0.0, f32::from_bits(0x3e666667), f32::from_bits(0xbe666667)],
        [2.0, 0.0, 0.0],
        [0.0, 0.0, f32::from_bits(0xbeaccccc)],
        [0.0, plank_y, 0.0],
        3,
        0,
        77,
        0,
    );
    assert_eq!(out[0], expected_plank);
    assert_eq!(f32_at(&out[0], 40), plank_y);
    assert_eq!(u32_at(&out[0], 48), 3);
    assert_eq!(u32_at(&out[0], 52), 0);
    assert_eq!(u32_at(&out[0], 56), 77);
    assert_eq!(u32_at(&out[0], 60), 0);

    // Plank 2: start x = 4.0, base_index = 2 -> source = 4*1000 + 2.
    assert_eq!(f32_at(&out[1], 36), 4.0);
    assert_eq!(f32_at(&out[1], 40), plank_y);
    assert_eq!(u32_at(&out[1], 52), 4002);

    // Diagonal 1 from anchor 0: budget = 2.0*0.85 = 1.7; length is capped by
    // the 0.4*height budget, extension = length + 0.1125 + 0.1875.
    let budget = 2.0f32 * 0.85f32;
    let height = 5.0f32 + f32::from_bits(0xbe666667); // -0.225
    let length = (budget + f32::from_bits(0xbe866666)).min(height * 0.4f32); // -0.2625
    let extension =
        (length + f32::from_bits(0x3de66667)) + f32::from_bits(0x3e400000); // 0.1125 + 0.1875
    let anchor_y = (height - length) + f32::from_bits(0xbe400000); // -0.1875
    assert_eq!(f32_at(&out[2], 36), 0.0);
    assert_eq!(f32_at(&out[2], 40), anchor_y);
    assert_eq!(f32_at(&out[2], 44), 0.0);
    // Axis thickness is exactly the 0.2625 constant on both side columns.
    let axis_scale = f32::from_bits(0x3e866666);
    let side = column(&out[2], 0).length();
    let third = column(&out[2], 2).length();
    assert!((side - axis_scale).abs() < 1e-5, "side={side}");
    assert!((third - axis_scale).abs() < 1e-5, "third={third}");
    // y-axis length = |a - b| - 0.2; |a-b| uses the 0.1125 / 0.1875 extension
    // and the -0.1125 end offset.
    let end_y = 5.0f32 + f32::from_bits(0xbde66667);
    let full = (extension * extension + (end_y - anchor_y) * (end_y - anchor_y)).sqrt();
    let y_axis = column(&out[2], 1).length();
    assert!((y_axis - (full - 0.2)).abs() < 1e-4, "y_axis={y_axis}");
    // 0.85 budget must bind below the 0.4*height cap for this fixture.
    assert!(budget + f32::from_bits(0xbe866666) < height * 0.4f32);
    assert_eq!(u32_at(&out[2], 52), 0);
    assert_eq!(u32_at(&out[2], 56), 77);

    // Diagonal 2 mirrors from the far end: anchor x = 6.
    assert_eq!(f32_at(&out[3], 36), 6.0);
    assert_eq!(f32_at(&out[3], 40), anchor_y);
    assert_eq!(u32_at(&out[3], 52), 3);
}

/// The 3.1499999 one-sided length limit is applied to the accumulated length.
#[test]
fn support_beam_grower_length_limit_is_31499999() {
    let limit = f32::from_bits(0x40499999);
    let grower = SupportBeamGrower::new(0, Vec3::ZERO);
    let below = limit - 0.001;
    assert!(
        grower
            .propose(1, &[[0.0, 0.0, 0.0], [below, 0.0, 0.0]], &[0, 0])
            .is_some()
    );
    let above = limit + 0.001;
    assert!(
        grower
            .propose(1, &[[0.0, 0.0, 0.0], [above, 0.0, 0.0]], &[0, 0])
            .is_none()
    );

    // Accumulated: 2.0 + 1.0 accepted, 2.0 + 1.2 rejected.
    let mut accumulated = SupportBeamGrower::new(0, Vec3::ZERO);
    accumulated.length = 2.0;
    assert!(
        accumulated
            .propose(1, &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], &[0, 0])
            .is_some()
    );
    assert!(
        accumulated
            .propose(1, &[[0.0, 0.0, 0.0], [1.2, 0.0, 0.0]], &[0, 0])
            .is_none()
    );
}

/// Fence records use the frozen row-major packer. Posts and rails expose the
/// documented 0.3 / +0.1 / -0.1 and 0.1 / 0.2 section bytes.
#[test]
fn emit_fences_writes_row_major_64b_post_and_rail_layout() {
    let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [0.0, 0.0, 4.0]];
    let input = FenceSegmentInput {
        wall: PublicWallId(7),
        height: 2.0,
        positions: &points,
        spacing_extent: 4.0,
        seed: 11,
        skip_first: false,
        segment_ordinal: 1,
        segment_count: 1,
    };
    let mut out = Vec::new();
    let pairs = emit_fences(input, &mut out).unwrap();
    assert!(pairs >= 1);
    assert!(!out.is_empty());

    let mut posts = 0usize;
    let mut rails = 0usize;
    for record in &out {
        // Row-major packer: columns live at (0,16,32), (4,20,36), (8,24,40);
        // translation at (12,28,44). This is the frozen 64B byte order.
        let column0 = Vec3::new(f32_at(record, 0), f32_at(record, 16), f32_at(record, 32));
        let column1 = Vec3::new(f32_at(record, 4), f32_at(record, 20), f32_at(record, 36));
        let column2 = Vec3::new(f32_at(record, 8), f32_at(record, 24), f32_at(record, 40));
        assert_eq!(u32_at(record, 48), 11, "segment_ordinal = seed");
        assert_eq!(u32_at(record, 56), 7);
        assert_eq!(u32_at(record, 60), 0);

        if (column0.length() - 0.3).abs() < 1e-5 {
            // Post: 0.3 square section, +0.1 vertical extension, -0.1 Y offset.
            posts += 1;
            assert!((column1.length() - 2.4).abs() < 1e-5);
            assert!((column2.length() - 0.3).abs() < 1e-5);
            assert_eq!(f32_at(record, 28), -0.1);
        } else {
            // Rail: 0.1 x 0.2 section with unclamped length on the y column.
            rails += 1;
            assert!((column0.length() - 0.1).abs() < 1e-5);
            assert!((column2.length() - 0.2).abs() < 1e-5);
        }
    }
    assert!(posts >= 2, "posts={posts}");
    assert!(rails >= 1, "rails={rails}");
}

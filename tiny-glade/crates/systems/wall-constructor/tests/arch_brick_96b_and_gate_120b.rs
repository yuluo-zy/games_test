//! WC-C byte-level contracts for the 96B arch `BrickTransformSsbo` records and
//! the 120B `ConstructGateCmd` payload produced by the recovered arch chain.
//!
//! `system_wall_constructor::construct_arches` delegates to
//! `country_core::systems::wall::construction::arches::
//! construct_regular_wall_arches` (0x14075f680 core chain, 130 frozen cases /
//! 4159 records in `evidence/building-arches/B-regular-arches-verification.json`).
//! The fixture below is the smallest explicit regular-wall input: one 5-point
//! open mask, flat zero segment values, `shape<2`, roof present.
//!
//! Layouts asserted:
//! * 96B `BrickTransformSsbo` offsets 0/12/24/36 (affine), 48 (zero
//!   deformation), 64 (-100 surface sentinel), 76 seed, 80 material,
//!   84/88 wall distance, 92 flags.
//! * 120B gate command = 104B `MaterializedWallSegment` + `wall_id` + `ordinal`
//!   (checked with `offset_of!` against a `repr(C)` projection).
use country_core::resources::walls::publication_fields::WallId as PublicWallId;
use country_core::systems::wall::construction::arches::{
    ConstructGateCmd120Projection, RegularWallArchesInput, WallArchesOutput,
    construct_regular_wall_arches,
};
use country_core::systems::wall::construction::segmentation::{
    ThresholdedCurveCoord, WallLoopRange, WallPathSegmentationMask,
};
use country_core::systems::wall::construction::structures::MaterializedWallSegment;
use glam::Vec3;
use std::mem::offset_of;

/// Contract-derived 96B record for one arch brick (`emit_arch_brick`,
/// 0x140763... core chain): x = direction*full_length, y = up*0.42,
/// z = side*0.7, translation = midpoint, surface sentinel -100, flags 0.
fn expected_arch_brick(
    start: Vec3,
    end: Vec3,
    us: [f32; 2],
    wall_id: u32,
    source_length: f32,
    ordinal: u32,
    index: u32,
) -> [u8; 96] {
    let center = (start + end) * 0.5;
    let delta = end - start;
    let full = ((delta.x * delta.x + delta.y * delta.y) + delta.z * delta.z).sqrt();
    let d = delta * (1.0f32 / full);
    let side = d.cross(Vec3::Y);
    let side = side * (1.0f32 / ((side.x * side.x + side.y * side.y) + side.z * side.z).sqrt());
    let up = side.cross(d);
    let x = d * full;
    let y = up * f32::from_bits(0x3ed70a3e); // 0.42
    let z = side * 0.7f32;
    let end_u = if us[1] < us[0] { us[1] + 1.0 } else { us[1] };

    let mut bytes = [0u8; 96];
    for (column, values) in [x, y, z, center].into_iter().enumerate() {
        for (component, value) in values.to_array().into_iter().enumerate() {
            let offset = column * 12 + component * 4;
            bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    for surface in 0..3 {
        let offset = 64 + surface * 4;
        bytes[offset..offset + 4].copy_from_slice(&(-100.0f32).to_le_bytes());
    }
    bytes[76..80].copy_from_slice(&(ordinal.wrapping_add(index)).to_le_bytes());
    bytes[80..84].copy_from_slice(&wall_id.to_le_bytes());
    bytes[84..88].copy_from_slice(&(us[0] * source_length).to_le_bytes());
    bytes[88..92].copy_from_slice(&(end_u * source_length).to_le_bytes());
    bytes[92..96].copy_from_slice(&0u32.to_le_bytes());
    bytes
}

fn f32_at(bytes: &[u8; 96], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn u32_at(bytes: &[u8; 96], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn flat_mask(points: usize) -> WallPathSegmentationMask {
    WallPathSegmentationMask {
        flags: {
            let mut flags = vec![0u8; points];
            flags[0] = 8;
            flags[points - 1] = 8;
            flags
        },
        coords: (0..points)
            .map(|index| {
                if index == points - 1 {
                    ThresholdedCurveCoord {
                        point_index: (index - 1) as u32,
                        fraction: 1.0,
                    }
                } else {
                    ThresholdedCurveCoord {
                        point_index: index as u32,
                        fraction: 0.0,
                    }
                }
            })
            .collect(),
        range: WallLoopRange::for_open(0, points as i32, points).unwrap(),
        length: 0.0,
    }
}

fn run_arches() -> WallArchesOutput {
    let prepared_positions = vec![
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [4.0, 0.0, 0.0],
        [6.0, 0.0, 0.0],
        [8.0, 0.0, 0.0],
    ];
    let prepared_u = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    // Zero segment values keep the regular profile flat at y=0 after
    // `regular_profile`'s weight blend, independent of arch inputs.
    let path_values = vec![0.0f32; 5];
    let masks = vec![flat_mask(5)];
    let mut arch = vec![3.0f32; 5];
    let mut third = vec![0.0f32; 5];
    let mut output = construct_regular_wall_arches(
        RegularWallArchesInput {
            masks: &masks,
            prepared_positions: &prepared_positions,
            prepared_u: &prepared_u,
            path_values: &path_values,
            shape_tag: 0,
            public_284: 1.0,
            observed_294: 0,
            source_curve_length: 8.0,
            wall_id: PublicWallId(42),
            has_roof: true,
        },
        &mut arch,
        &mut third,
    )
    .expect("valid arch fixture");
    // Closed-outline materialization copies the first third height to the last.
    assert_eq!(third[4], third[0]);
    output
}

/// Exact first 96B arch record. The walker emits one brick per 0.56 m step
/// (14 steps over 8 m) and appends a tail brick because the residual 0.5714 m
/// exceeds `step * 0.1`.
#[test]
fn arch_walker_writes_exact_first_96b_brick() {
    let output = run_arches();
    assert_eq!(output.segment_count, 1);
    assert_eq!(output.instances.len(), 15, "14 steps + 0.1-ratio tail");
    let bytes: Vec<[u8; 96]> = output
        .instances
        .iter()
        .map(|instance| instance.observed_bytes())
        .collect();

    let total = 8.0f32;
    let count = ((total / f32::from_bits(0x3f0f5c29)) + 0.5).floor().max(1.0) as usize;
    assert_eq!(count, 14);
    let step = total / count as f32;
    let first_t = step / 2.0f32; // first 2 m segment
    // walker curve coordinate: t * (u1 - u0) + u0 with u = [0, .25, ...].
    let end_u = first_t * 0.25f32;

    let expected = expected_arch_brick(
        Vec3::ZERO,
        Vec3::new(step, 0.0, 0.0),
        [0.0, end_u],
        42,
        8.0,
        1,
        0,
    );
    assert_eq!(bytes[0], expected);
    assert_eq!(f32_at(&bytes[0], 0), f32_at(&expected, 0));
    assert_eq!(f32_at(&bytes[0], 16), f32::from_bits(0x3ed70a3e));
    assert_eq!(f32_at(&bytes[0], 32), f32::from_bits(0x3f333333));
    // Seed is monotonically ordinal+index for every emitted brick.
    for (index, record) in bytes.iter().enumerate() {
        assert_eq!(u32_at(record, 76), 1 + index as u32);
        assert_eq!(u32_at(record, 80), 42);
        assert_eq!(u32_at(record, 92), 0);
        assert_eq!(f32_at(record, 64), -100.0);
        assert_eq!(&record[48..64], &[0u8; 16]);
    }
    // Tail brick spans the residual from the last step to the final point.
    let tail = &bytes[14];
    assert!((f32_at(tail, 36) - 7.714285f32).abs() < 1e-4);
    assert_eq!(f32_at(tail, 40), 0.0);
}

/// The same single-segment run emits a 120B gate command whose fields match
/// the native `{MaterializedWallSegment, WallId, ordinal}` order.
#[test]
fn gate_command_is_104b_segment_plus_wall_and_ordinal() {
    let output = run_arches();
    assert_eq!(output.gate_commands.len(), 1);
    let command: &ConstructGateCmd120Projection = &output.gate_commands[0];
    assert_eq!(command.wall_id, PublicWallId(42));
    assert_eq!(command.ordinal, 1);
    assert_eq!(command.segment.positions.len(), 5);
    assert_eq!(command.segment.heights, vec![0.0f32; 5]);
    assert_eq!(command.segment.points_u, vec![0.0, 0.25, 0.5, 0.75, 0.0]);
    assert_eq!(command.segment.source_indices, [0, 5]);
    assert_eq!(command.segment.range.observed_bytes(), {
        let mut bytes = [0u8; 12];
        bytes[..4].copy_from_slice(&0i32.to_le_bytes());
        bytes[4..8].copy_from_slice(&5i32.to_le_bytes());
        bytes[8..].copy_from_slice(&5i32.to_le_bytes());
        bytes
    });

    #[repr(C)]
    struct GateCmd120 {
        segment: MaterializedWallSegment,
        wall: u64,
        ordinal: u64,
    }
    assert_eq!(size_of::<MaterializedWallSegment>(), 104);
    assert_eq!(size_of::<GateCmd120>(), 120);
    assert_eq!(offset_of!(GateCmd120, segment), 0);
    assert_eq!(offset_of!(GateCmd120, wall), 104);
    assert_eq!(offset_of!(GateCmd120, ordinal), 112);
}

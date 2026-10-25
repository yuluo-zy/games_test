//! B canonical door-stair payload, brick construction and explicit diff protocol.
//!
//! Original entry 1407ff130/52B marks the real ResMut changed and forwards its
//! resource closure to with_diffs 1412af3a0. The constructors below append the
//! canonical payload to the real shared Sparse writer. The protocol adapter
//! remains distinct from the original four-SystemParam wall host: actual lane
//! ownership, initialization and registration remain gates. No empty host is
//! supplied or registered here.

use bevy_ecs::change_detection::ResMut;
use country_core::{
    geometry::instanced_wall::{BrickSourceId, BrickTransformSsbo},
    render::sparse_instance_buffer::SparseInstanceBufferWriter,
    resources::{
        render::Affine3Packed,
        walls::{WallId, decorator::DecoratorId, publication_fields::WallId as BrickSourceWallId},
    },
};
use glam::{Affine3A, Vec2, Vec3};

use super::deco_differential_state::{ConsumedDiffs, DecoDiffLog, WallAttachedDecoTracker};

/// Original add clones 32 bytes, and 140825150/152B compares eight ordered f32s.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DoorStairsAssemblyParams {
    pub words: [u32; 8],
}

impl PartialEq for DoorStairsAssemblyParams {
    fn eq(&self, other: &Self) -> bool {
        self.words
            .iter()
            .zip(other.words.iter())
            .all(|(a, b)| f32::from_bits(*a) == f32::from_bits(*b))
    }
}

const _: () = assert!(std::mem::size_of::<DoorStairsAssemblyParams>() == 32);
const _: () = assert!(std::mem::align_of::<DoorStairsAssemblyParams>() == 4);

/// Original 14200c7f0/4668B current-instance constructor, called with
/// depth0.56 by 1412af8f2. It appends to the actual shared sparse writer.
/// `true` means NoFreeChunks occurred; subsequent bricks are still attempted.
/// The host clears the whole lane once before visiting every current instance.
pub fn construct_door_stairs(
    params: &DoorStairsAssemblyParams,
    wall: WallId,
    rng: &fastrand::Rng,
    writer: &mut SparseInstanceBufferWriter<'_, BrickTransformSsbo>,
) -> bool {
    emit_door_stairs(params, wall, rng, &mut |brick| writer.push(brick).is_err())
}

fn emit_door_stairs(
    params: &DoorStairsAssemblyParams,
    wall: WallId,
    rng: &fastrand::Rng,
    emit: &mut impl FnMut(&BrickTransformSsbo) -> bool,
) -> bool {
    let p = params.words.map(f32::from_bits);
    let center = Vec2::new(p[0], p[1]);
    let axis = Vec2::new(p[2], p[3]);
    let height = p[4] - p[5];
    let extra_height = height + p[6];
    let y_count = ((extra_height / 0.2).ceil() as usize).max(2);
    let heights: Vec<f32> = {
        let splits = utils::random_splits(y_count, f32::from_bits(0x3d23_d70b) / extra_height, rng);
        let end = p[5] - p[6];
        splits.iter().map(|&t| t * end + (1.0 - t) * p[4]).collect()
    };
    let step_count = ((height / 0.2).ceil() as usize).clamp(2, 5);
    let step_depth = 0.56 / step_count as f32;
    let scale_depth = step_depth + 0.1;
    let x_count = ((p[7] / 0.45).ceil() as usize).max(3);
    let x_jitter = f32::from_bits(0x3e38_51eb) / p[7];
    let half_width = p[7] * 0.5;
    let material = BrickSourceId::from_wall_id(BrickSourceWallId(wall.0)).0;
    let mut failed = false;
    for (row, pair) in heights.windows(2).enumerate() {
        let row_height = (pair[1] - pair[0]).abs();
        let middle_y = pair[0] * 0.5 + pair[1] * 0.5;
        let seed = (wall.0 as u32).wrapping_add((row as u32).wrapping_mul(2));
        for step in 0..(row + 1).min(step_count) {
            let xs: Vec<f32> = {
                let splits = utils::random_splits(x_count, x_jitter, rng);
                splits
                    .iter()
                    .map(|&t| t * half_width - (1.0 - t) * half_width)
                    .collect()
            };
            let offset = step as f32 * step_depth + 0.28;
            let center_x = offset * axis.x + center.x;
            let center_z = offset * axis.y + center.y;
            for pair in xs.windows(2) {
                let across = pair[0] * 0.5 + pair[1] * 0.5;
                let translation = Vec3::new(
                    center_x - across * axis.y,
                    middle_y,
                    across * axis.x + center_z,
                );
                let rotation = utils::xz_rotation_bad(axis);
                let affine = Affine3A::from_scale_rotation_translation(
                    Vec3::new((pair[1] - pair[0]).abs(), row_height, scale_depth),
                    rotation,
                    translation,
                );
                let brick = BrickTransformSsbo {
                    affine_columns: Affine3Packed::from(affine)
                        .cols
                        .map(|column| column.to_array()),
                    seed,
                    material,
                    wall_distance: [pair[0], pair[1]],
                    ..BrickTransformSsbo::default()
                };
                failed |= emit(&brick);
            }
        }
    }
    failed
}

/// Reuses the original per-decorator seed exactly once. This writes real
/// canonical instances but does not replace the unregistered full host system.
pub fn write_door_stairs_instance(
    params: &DoorStairsAssemblyParams,
    wall: WallId,
    decorator: DecoratorId,
    writer: &mut SparseInstanceBufferWriter<'_, BrickTransformSsbo>,
) -> bool {
    let rng = fastrand::Rng::with_seed(utils::rng::init_rng(decorator.0));
    construct_door_stairs(params, wall, &rng, writer)
}

/// The real wall consumer must build all current values, then explicitly choose
/// the original Door no-op removal callback. A missing host is not success.
pub fn door_stairs_assemble_bricks<F>(
    mut tracker: ResMut<'_, WallAttachedDecoTracker<DoorStairsAssemblyParams>>,
    consume_wall: F,
) where
    F: for<'a> FnMut(
        WallId,
        DecoDiffLog<'a, DoorStairsAssemblyParams>,
    ) -> ConsumedDiffs<'a, DoorStairsAssemblyParams>,
{
    tracker.with_diffs(consume_wall);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_ordered_float_equality_is_not_bit_equality() {
        let a = DoorStairsAssemblyParams { words: [0; 8] };
        let mut b = a;
        b.words[7] = (-0.0f32).to_bits();
        assert_eq!(a, b);
        b.words[7] = f32::NAN.to_bits();
        assert_ne!(b, b);
    }

    #[test]
    fn row_seed_directional_intervals_and_rng_survive_emit_failure() {
        let params = DoorStairsAssemblyParams {
            words: [0.0f32, 0.0, 1.0, 0.0, 0.4, 0.0, 0.4, 0.9].map(f32::to_bits),
        };
        let rng = fastrand::Rng::with_seed(456);
        let mut bricks = Vec::new();
        assert!(!emit_door_stairs(
            &params,
            WallId(100_003),
            &rng,
            &mut |brick| {
                bricks.push(*brick);
                false
            }
        ));
        // Three y rows, with 1/2/2 steps respectively and two x intervals.
        assert_eq!(bricks.len(), 10);
        assert_eq!(
            bricks.iter().map(|brick| brick.seed).collect::<Vec<_>>(),
            [
                100_003, 100_003, 100_005, 100_005, 100_005, 100_005, 100_007, 100_007, 100_007,
                100_007
            ]
        );
        for step in bricks.chunks_exact(2) {
            assert_eq!(step[0].wall_distance[0], -0.45);
            assert_eq!(step[0].wall_distance[1], step[1].wall_distance[0]);
            assert_eq!(step[1].wall_distance[1], 0.45);
        }
        assert!(bricks.iter().all(|brick| brick.material == 0x8000_0003
            && brick.deformation == [0.0; 4]
            && brick.surface == [-100.0; 3]
            && brick.flags == 0));
        // Two y split samples and one x split sample for each of five steps.
        let expected_rng = fastrand::Rng::with_seed(456);
        for _ in 0..7 {
            expected_rng.f32();
        }
        assert_eq!(rng.get_seed(), expected_rng.get_seed());

        let failed_rng = fastrand::Rng::with_seed(456);
        let mut attempted = Vec::new();
        assert!(emit_door_stairs(
            &params,
            WallId(100_003),
            &failed_rng,
            &mut |brick| {
                attempted.push(*brick);
                attempted.len() == 1
            }
        ));
        assert_eq!(attempted, bricks);
        assert_eq!(failed_rng.get_seed(), rng.get_seed());
    }
}

//! B canonical chimney payload, brick construction and explicit diff protocol.
//!
//! The original 1407f8db0/117B entry marks the real ResMut changed and passes a
//! resource closure to with_diffs 1412ae660. The constructors below append the
//! canonical payload to the real shared Sparse writer. The protocol adapter
//! still requires a complete, explicitly supplied wall consumer; it is not the
//! original seven-SystemParam geometry/audio system. Lane ownership, resource
//! initialization, derived-info/audio consumption and registration remain gates.

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

/// Exact 40-byte carrier. Field meanings beyond the native offset contract are
/// not invented. 140824fe0/172B proves the comparison used by fullvisual.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ChimneyAssemblyParams {
    pub words: [u32; 10],
}

impl PartialEq for ChimneyAssemblyParams {
    fn eq(&self, other: &Self) -> bool {
        (2..8).all(|i| f32::from_bits(self.words[i]) == f32::from_bits(other.words[i]))
            && self.words[0] == other.words[0]
            && (self.words[0] == 1
                || f32::from_bits(self.words[1]) == f32::from_bits(other.words[1]))
            && self.words[8] == other.words[8]
            && f32::from_bits(self.words[9]) == f32::from_bits(other.words[9])
    }
}

const _: () = assert!(std::mem::size_of::<ChimneyAssemblyParams>() == 40);
const _: () = assert!(std::mem::align_of::<ChimneyAssemblyParams>() == 4);

/// The original constructor's two endpoints and rectangle size controls.
/// These values are supplied by the original per-current-instance branches.
#[derive(Clone, Copy, Debug)]
pub struct ChimneyBrickSection {
    pub end_2: f32,
    pub end_3: f32,
    pub base_size: Vec2,
    pub size_delta: f32,
}

/// Original 14200bcd0/2836B. Appends to the actual shared sparse writer.
/// `true` means at least one NoFreeChunks failure; all later bricks are still
/// attempted. Clearing the per-wall lane belongs to the surrounding host.
pub fn construct_crossbrick_chimney(
    params: &ChimneyAssemblyParams,
    section: ChimneyBrickSection,
    wall: WallId,
    rng: &fastrand::Rng,
    writer: &mut SparseInstanceBufferWriter<'_, BrickTransformSsbo>,
) -> bool {
    emit_crossbrick_chimney(params, section, wall, rng, &mut |brick| {
        writer.push(brick).is_err()
    })
}

fn emit_crossbrick_chimney(
    params: &ChimneyAssemblyParams,
    section: ChimneyBrickSection,
    wall: WallId,
    rng: &fastrand::Rng,
    emit: &mut impl FnMut(&BrickTransformSsbo) -> bool,
) -> bool {
    let center = Vec2::new(
        f32::from_bits(params.words[2]),
        f32::from_bits(params.words[3]),
    );
    let axis = Vec2::new(
        f32::from_bits(params.words[6]),
        f32::from_bits(params.words[7]),
    );
    let height = section.end_2 - section.end_3;
    let count = ((height / 0.4).ceil() as usize).max(3);
    // The original borrows the normalized split list, collects a second list,
    // then drops the first. Preserve interpolation order and allocation failure.
    let heights: Vec<f32> = {
        let splits = utils::random_splits(count, 0.2 / height, rng);
        splits
            .iter()
            .map(|&t| t * section.end_2 + (1.0 - t) * section.end_3)
            .collect()
    };
    let layer_count = (heights.len() - 1) as f32;
    let material = BrickSourceId::from_wall_id(BrickSourceWallId(params.words[8] as u64)).0;
    let mut distance = 0.0;
    let mut failed = false;
    for (layer, pair) in heights.windows(2).enumerate() {
        let low = Vec3::new(center.x, pair[0], center.y);
        let high = Vec3::new(center.x, pair[1], center.y);
        let middle_xz = Vec2::new((low.x + high.x) * 0.5, (low.z + high.z) * 0.5);
        let middle_y = (low.y + high.y) * 0.5;
        let layer_height = (high - low).length();
        let t = layer as f32 / layer_count;
        let s = ((t + f32::from_bits(0xbf59_999a)) / f32::from_bits(0x3e19_9998)).clamp(0.0, 1.0);
        let smooth = (3.0 - (s + s)) * (s * s);
        let size_offset = smooth * section.size_delta;
        let size = Vec2::new(
            size_offset + section.base_size.x,
            size_offset + section.base_size.y,
        );
        let rectangle = utils::geometry::Rectangle2d::new(middle_xz, size, axis);
        let sign = if layer & 1 == 0 { -1.0 } else { 1.0 };
        for edge in 0..4 {
            let a = rectangle.get_point(edge);
            let b = rectangle.get_point((edge + 1) & 3);
            let delta = b - a;
            let offset = ((delta * (1.0 / delta.length())) * 0.3) * 0.5 * sign;
            let a = a + offset;
            let b = b + offset;
            let middle = a * 0.5 + b * 0.5;
            let r = rng.f32();
            let thickness = (r * 0.05 - (1.0 - r) * 0.05) + 0.3;
            let rotation = utils::xz_rotation_bad(rectangle.get_edge_normal(edge));
            let length = (a - b).length();
            let affine = Affine3A::from_scale_rotation_translation(
                Vec3::new(length, layer_height, thickness),
                rotation,
                Vec3::new(middle.x, middle_y, middle.y),
            );
            let next_distance = length + distance;
            let brick = BrickTransformSsbo {
                affine_columns: Affine3Packed::from(affine)
                    .cols
                    .map(|column| column.to_array()),
                seed: (wall.0 as u32)
                    .wrapping_add(layer as u32)
                    .wrapping_add(edge as u32),
                material,
                wall_distance: [distance, next_distance],
                ..BrickTransformSsbo::default()
            };
            distance = next_distance;
            failed |= emit(&brick);
        }
    }
    failed
}

/// Original current-instance geometry branch. Uses the same deterministic rng
/// for the body and the optional lower section; no audio or diff normalization
/// is performed here. The real host must do those original side effects.
pub fn write_chimney_instance(
    params: &ChimneyAssemblyParams,
    wall: WallId,
    decorator: DecoratorId,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >,
    writer: &mut SparseInstanceBufferWriter<'_, BrickTransformSsbo>,
) -> bool {
    let rng = fastrand::Rng::with_seed(utils::rng::init_rng(decorator.0));
    let base = f32::from_bits(params.words[4]);
    let mut failed = construct_crossbrick_chimney(
        params,
        ChimneyBrickSection {
            end_2: f32::from_bits(params.words[5]),
            end_3: base,
            base_size: Vec2::splat(0.5),
            size_delta: 0.3,
        },
        wall,
        &rng,
        writer,
    );
    let extra = match params.words[0] {
        1 => Some(ChimneyBrickSection {
            end_2: base,
            end_3: base + -0.4,
            base_size: Vec2::splat(0.2),
            size_delta: 0.1,
        }),
        0 => {
            let center = Vec2::new(
                f32::from_bits(params.words[2]),
                f32::from_bits(params.words[3]),
            );
            let axis = Vec2::new(
                f32::from_bits(params.words[6]),
                f32::from_bits(params.words[7]),
            );
            let along = (axis * 1.3) * 0.5;
            let across = Vec2::new(-axis.y, axis.x) * 0.5;
            let mut end = base - f32::from_bits(params.words[1]);
            for point in [center + along + across, center - along + across] {
                end = end.min(terrain.sample_at_world_xz(point.x, point.y));
            }
            Some(ChimneyBrickSection {
                end_2: base,
                end_3: end,
                base_size: Vec2::new(1.3, 1.0),
                size_delta: -0.3,
            })
        }
        _ => {
            let height = f32::from_bits(params.words[1]);
            (height > 0.1).then_some(ChimneyBrickSection {
                end_2: base,
                end_3: base - height,
                base_size: Vec2::splat(0.8),
                size_delta: 0.0,
            })
        }
    };
    if let Some(section) = extra {
        failed |= construct_crossbrick_chimney(params, section, wall, &rng, writer);
    }
    failed
}

/// Explicit protocol seam. The callback must perform the real current-value
/// work and explicitly consume removals; no default or empty host is supplied.
/// Register the actual seven-parameter system only when that host is restored.
pub fn chimney_assemble_bricks<F>(
    mut tracker: ResMut<'_, WallAttachedDecoTracker<ChimneyAssemblyParams>>,
    consume_wall: F,
) where
    F: for<'a> FnMut(
        WallId,
        DecoDiffLog<'a, ChimneyAssemblyParams>,
    ) -> ConsumedDiffs<'a, ChimneyAssemblyParams>,
{
    tracker.with_diffs(consume_wall);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_float_equality_and_tag_one_inactive_word() {
        let a = ChimneyAssemblyParams { words: [0; 10] };
        let mut b = a;
        b.words[2] = (-0.0f32).to_bits();
        assert_eq!(a, b);
        b.words[2] = f32::NAN.to_bits();
        assert_ne!(b, b);
        let mut c = a;
        c.words[0] = 1;
        let mut d = c;
        d.words[1] = f32::NAN.to_bits();
        assert_eq!(c, d);
        d.words[8] = 1;
        assert_ne!(c, d);
    }

    #[test]
    fn emitted_payload_and_rng_continue_after_a_failed_brick() {
        let mut params = ChimneyAssemblyParams { words: [0; 10] };
        params.words[6] = 1.0f32.to_bits();
        params.words[8] = 100_001;
        let section = ChimneyBrickSection {
            end_2: 0.8,
            end_3: 0.0,
            base_size: Vec2::splat(0.5),
            size_delta: 0.0,
        };
        let rng = fastrand::Rng::with_seed(123);
        let mut bricks = Vec::new();
        assert!(!emit_crossbrick_chimney(
            &params,
            section,
            WallId(100),
            &rng,
            &mut |brick| {
                bricks.push(*brick);
                false
            },
        ));
        assert_eq!(bricks.len(), 8);
        assert_eq!(
            bricks.iter().map(|brick| brick.seed).collect::<Vec<_>>(),
            [100, 101, 102, 103, 101, 102, 103, 104]
        );
        assert_eq!(bricks[0].wall_distance[0], 0.0);
        assert_eq!(bricks.last().unwrap().wall_distance[1], 4.0);
        for pair in bricks.windows(2) {
            assert_eq!(pair[0].wall_distance[1], pair[1].wall_distance[0]);
        }
        assert!(bricks.iter().all(|brick| brick.material == 0x8000_0001
            && brick.deformation == [0.0; 4]
            && brick.surface == [-100.0; 3]
            && brick.flags == 0));
        // One split sample followed by one thickness sample per brick.
        let expected_rng = fastrand::Rng::with_seed(123);
        for _ in 0..9 {
            expected_rng.f32();
        }
        assert_eq!(rng.get_seed(), expected_rng.get_seed());

        let failed_rng = fastrand::Rng::with_seed(123);
        let mut attempted = Vec::new();
        assert!(emit_crossbrick_chimney(
            &params,
            section,
            WallId(100),
            &failed_rng,
            &mut |brick| {
                attempted.push(*brick);
                attempted.len() == 1
            },
        ));
        assert_eq!(attempted, bricks);
        assert_eq!(failed_rng.get_seed(), rng.get_seed());
    }
}

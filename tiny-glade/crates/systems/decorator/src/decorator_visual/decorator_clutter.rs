//! B generate_decorator_clutter@1407e3370, 6631 bytes including cleanup ranges.
//! Identity-bound evidence is retained in scene-clutter-producer-20261020.
//! Generation writes blueprints; actual entity and collider instantiation is a
//! separate original consumer. No RNG, transient spawn or subtype fallback.

use std::sync::Arc;

use bevy_ecs::prelude::*;
use country_core::resources::{
    clutter_meshes::{ClutterMeshTy, ClutterMeshes},
    roofs::Roof,
    walls::{
        cached_decorator_transforms::CachedDecoratorTransforms,
        decorator::DecoratorInfo,
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        public_walls::PublicWalls,
    },
};
use country_core::systems::ui::signifiers::UiSignifierStream;
use glam::Vec3;
use utils::geometry::rectangle::Rectangle3d;

use crate::blueprint::DecoratorBlueprints;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;
use crate::utils::{clutter_mesh_ty_data, new_clutter_cuboid};

pub const CLUTTER_TYPE_LOW: u8 = 0x38;
pub const CLUTTER_TYPE_HIGH: u8 = 0x3f;
pub const CLUTTER_TYPE_EXCLUDED: u8 = 0x3e;
pub const CLUTTER_HEIGHT_OFFSET: f32 = -0.5;
pub const CLUTTER_HEIGHT_MIN: f32 = 0.1;

/// Native exclusion test at 1407e3a76..3a8a. It skips 38..3d and 3f.
/// Original typed constructors encode Clutter as its inner 0..55 payload; the
/// seven unit variants encode 56..61 or 63. Raw 62 is outside that observed typed
/// domain. This predicate preserves the native byte test; the checked conversion
/// below rejects invalid values admitted by the wider host raw-u8 boundary.
pub fn is_clutter_type(ty: u8) -> bool {
    !(ty.wrapping_sub(CLUTTER_TYPE_LOW) < 8 && ty != CLUTTER_TYPE_EXCLUDED)
}

/// Native max-number shrink. NaN chooses the real 0.1 second operand.
pub fn clutter_height(h: f32) -> f32 {
    (h + CLUTTER_HEIGHT_OFFSET).max(CLUTTER_HEIGHT_MIN)
}

/// Exact resize table. Hex literals preserve the original float payloads.
fn resize_clutter_bounds(bounds: &mut Rectangle3d, tag: u8) {
    let bits = match tag {
        0 => [0x3f666666, 0x3f666666, 0x3f666666],
        7 => [0x3fc51eb8, 0x3faa3d71, 0x3fa43958],
        13 | 19 | 43 => [0x3f666666, 0x3fbc6a7f, 0x3f666666],
        14 => [0x3fe4dd2f, 0x3fbd9168, 0x3fc5c28f],
        20 => [0x40000000, 0x3f99999a, 0x3f800000],
        23 | 24 | 40 => [0x3fd28f5c, 0x3fb39581, 0x3fdeb852],
        25 | 26 | 41 => [0x3fb8f5c3, 0x3f9bc6a8, 0x3f800000],
        34 => [0x3fd8b439, 0x3f89fbe7, 0x3ff53f7d],
        35 => [0x3fe28f5c, 0x3fc2f1aa, 0x408e978d],
        36 | 42 => [0x3fce353f, 0x3fc5e354, 0x3fc9999a],
        _ => return,
    };
    bounds.set_size(Vec3::from_array(bits.map(f32::from_bits)));
}

/// Native birdspot payload is Vec3, packed by the existing builder boundary.
fn birdspot(tag: u8) -> Option<(u64, u32)> {
    let [x, y, z]: [u32; 3] = match tag {
        3 | 4 => [0, 0x404147ae, 0xbe800000],
        9 => [0, 0x3e99999a, 0],
        11 => [0x3f59999a, 0x3f8a3d71, 0x3ec18937],
        12 => [0x3f000000, 0x3f8a3d71, 0x3ec18937],
        16 => [0, 0x3fcb851f, 0x3ee66666],
        17 | 18 => [0, 0x3f71eb85, 0],
        44 => [0, 0x3fc7ae14, 0],
        _ => return None,
    };
    Some((u64::from(x) | (u64::from(y) << 32), z))
}

/// Original parameter identities and order, including the read-only signifier
/// resource retained as a scheduling dependency although this body does not use it.
pub fn generate_decorator_clutter(
    storage: Res<DecoratorStorage>,
    cached: Res<CachedDecoratorTransforms>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    _signifiers: Res<UiSignifierStream>,
    meshes: Res<ClutterMeshes>,
) {
    let buckets = storage
        .per_wall
        .iter()
        .map(|(&wall, bucket)| (DecoratorStorageAddr::Wall(wall), bucket))
        .chain(
            storage
                .terrain
                .iter()
                .map(|bucket| (DecoratorStorageAddr::Terrain, bucket)),
        )
        .chain(
            storage
                .stairs
                .iter()
                .map(|bucket| (DecoratorStorageAddr::Stair, bucket)),
        );
    for (addr, bucket) in buckets {
        for &id in &bucket.modified_ids {
            // Native modified lookup miss reaches panic@1407e4454.
            let info = bucket
                .decorators
                .get(&id)
                .expect("modified decorator is missing");
            let tag = info.decorator_type().0;
            if !is_clutter_type(tag) || !matches!(info.dst_tag(), 2..=4) {
                continue;
            }
            if info.dst_tag() == 2 {
                // Native lookup is performed before the type9 exception.
                let flat = walls
                    .roof_entity(addr.as_wall())
                    .and_then(|entity| roofs.get(entity).ok())
                    .is_some_and(Roof::is_flat);
                if tag != 9 && !flat {
                    continue;
                }
            }
            // Native helper is valid for its 56-variant enum. In particular the
            // B parent/child serde constructors cannot produce raw 0x3e. This
            // checked host boundary rejects it; this is not a native panic branch.
            let ty = ClutterMeshTy::from_native_tag(tag)
                .expect("decorator clutter type is outside the recovered native enum");
            let blueprint = blueprints.insert_for(id, addr, 0);
            let transform = cached.transform(id);
            let data = clutter_mesh_ty_data(ty, &meshes);
            let mut bounds = data.mesh.bounds;
            resize_clutter_bounds(&mut bounds, tag);
            let dimensions = bounds.extent();
            // Both cuboids use original full height for the local pose, even
            // though the second constructor shrinks each dimension by 0.5.
            let zero = dimensions.y * 0.0 * 0.5;
            let center = Vec3::new(zero, dimensions.y * 0.5, zero);
            let full = new_clutter_cuboid(dimensions, center);
            let shrunken = new_clutter_cuboid(
                Vec3::new(
                    clutter_height(dimensions.x),
                    clutter_height(dimensions.y),
                    clutter_height(dimensions.z),
                ),
                center,
            );
            // Native first constructs both values, then allocates shrink Arc
            // before full Arc (1407e3ea1..3ec2 and 1407e3f4a..3f75).
            let shrunken = Arc::new(shrunken);
            let full = Arc::new(full);
            blueprint.set_positioned_colliders(
                shrunken,
                full,
                &transform.translation,
                &transform.rotation,
            );
            blueprint.set_cached_transform(transform);
            let mut builder =
                WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            builder.scale = transform.scale;
            builder.mesh_name = data.mesh.mesh_name;
            builder.nani_name = data.nani_name;
            builder.sittable_for_birds = birdspot(tag);
            // Original 1407e41b3 loads metadata+45, not helper tuple+18.
            builder.solid_texture = data.mesh.solid_texture.map(|selector| selector.encoded());
            blueprint.visual_entities.push(builder);
        }
    }
}

pub fn spawns_clutter(info: &DecoratorInfo) -> bool {
    is_clutter_type(info.decorator_type().0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn native_excluded_types_are_skipped_and_helper_boundary_is_explicit() {
        for ty in 0x38..=0x3d {
            assert!(!is_clutter_type(ty));
        }
        assert!(!is_clutter_type(0x3f));
        assert!(is_clutter_type(0x3e));
        assert!(ClutterMeshTy::from_native_tag(0x3e).is_none());
        for ty in 0..56 {
            assert!(is_clutter_type(ty));
        }
    }

    #[test]
    fn resize_keeps_center_and_shrink_uses_half_unit_not_one_unit() {
        let mut bounds = Rectangle3d::new(Vec3::new(2.0, 4.0, 6.0), Vec3::splat(2.0), Vec2::X);
        resize_clutter_bounds(&mut bounds, 20);
        assert_eq!(bounds.footprint.center, Vec2::new(2.0, 6.0));
        assert_eq!((bounds.bottom + bounds.top) * 0.5, 4.0);
        assert_eq!(bounds.footprint.size, Vec2::new(2.0, 1.0));
        assert_eq!(clutter_height(2.0), 1.5);
        assert_eq!(clutter_height(0.2), 0.1);
        assert_eq!(clutter_height(f32::NAN), 0.1);
    }

    #[test]
    fn actual_birdspot_payload_is_not_only_a_boolean() {
        assert_eq!(birdspot(16), Some((0x3fcb851f00000000, 0x3ee66666)));
        assert_eq!(birdspot(11), Some((0x3f8a3d713f59999a, 0x3ec18937)));
        assert_eq!(birdspot(10), None);
    }
}

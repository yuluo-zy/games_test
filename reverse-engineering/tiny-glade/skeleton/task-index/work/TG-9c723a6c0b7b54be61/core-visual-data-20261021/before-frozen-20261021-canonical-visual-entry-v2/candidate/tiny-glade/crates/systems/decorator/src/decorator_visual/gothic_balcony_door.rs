//! Gothic door visual generator and the complete original door-stair leaf.
//!
//! B check_for_door_stairs 1407f3340..1407f3873 is reconstructed from the
//! original RNE instructions. The surrounding visual generator remains partial
//! and is not promoted to a complete generator by this geometry restoration.
use bevy_ecs::prelude::*;
use country_core::resources::{
    terrain_height::TerrainHeightsData,
    world_raster::{RasterBufferRef, WorldRaster},
    walls::{
        cached_decorator_transforms::CachedDecoratorTransform,
        decorator_storage::DecoratorStorage,
        decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
    },
};
use glam::Quat;
use utils::geometry::Rectangle3d;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;
use super::door_stairs_assemble_bricks::DoorStairsAssemblyParams;

#[path = "door_stairs_geometry.rs"]
mod door_stairs_geometry;
pub use door_stairs_geometry::sample_door_stairs;

pub const GOTHIC_DOOR_SUBTYPE: u8 = 0x09;

/// Original five-input leaf. Replaces the actual boxed assembly output only on
/// success; rejection leaves an existing value untouched. The original height
/// sampling is terrain, while the second scalar is path coverage, not terrain.
pub fn check_for_door_stairs(
    terrain: &TerrainHeightsData,
    paths: &WorldRaster<512>,
    transform: &CachedDecoratorTransform,
    bounds: &Rectangle3d,
    blueprint: &mut DecoratorBlueprint,
) -> bool {
    let path_grid = RasterBufferRef::<512>::new(paths.current_buffer());
    let Some(words) = sample_door_stairs(
        transform.rotation,
        transform.translation,
        bounds.footprint.size.x,
        bounds.bottom,
        bounds.top,
        |point| terrain.sample_at_world_xz(point.x, point.y),
        |point| path_grid.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0),
    ) else {
        return false;
    };
    blueprint.door_stairs_assembly = Some(Box::new(DoorStairsAssemblyParams {
        words: words.map(f32::to_bits),
    }));
    true
}

/// 0x1407f0900（O 结构）：生成哥特阳台门装饰视觉。
pub fn generate_gothic_balcony_doors(
    decorator_storage: Res<DecoratorStorage>,
    derived: DerivedDecoratorInfoResources,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    terrain: Res<TerrainHeightsData>,
    paths: Res<WorldRaster<512>>,
) {
    let derived = derived.as_view();
    let mesh_lib = derived.mesh_library();
    for (wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：门子类型 = 9（0x1407f0900 行 347）。
            if derived
                .subtype(country_core::resources::walls::decorator::DecoratorId(id))
                .0
                != GOTHIC_DOOR_SUBTYPE
            {
                continue;
            }
            // O：蓝图 + 视觉实体。
            let blueprint = blueprints.insert_for(
                id,
                country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(wall),
                info.state(),
            );
            let transform = cached.transform(id);
            let key = DecoratorLibKey::new(GOTHIC_DOOR_SUBTYPE, info.rank());
            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&mesh_lib.collision_record(key).mesh),
                std::sync::Arc::clone(&mesh_lib.interaction_record(key).mesh),
                &transform.translation,
                &transform.rotation,
            );
            blueprint.set_outline((0.0, 0.0, 0.0));
            check_for_door_stairs(
                &terrain,
                &paths,
                &transform,
                &mesh_lib.collision_record(key).bounds,
                blueprint,
            );
            let builder = WindowVisualEntityBuilder::new(transform.translation, Quat::IDENTITY, id);
            let _ = &mut transient;
            let _ = &builder;
        }
    }
}

/// 哥特门判别（O：派生子类型 0x09）。
pub fn is_gothic_door(derived_subtype: u8) -> bool {
    derived_subtype == GOTHIC_DOOR_SUBTYPE
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::world_raster::DirtyTilePublisher;
    use glam::{Vec2, Vec3};

    fn bounds() -> Rectangle3d {
        Rectangle3d::new(Vec3::ZERO, Vec3::new(2.0, 2.0, 1.0), Vec2::X)
    }

    #[test]
    fn actual_sampling_rejection_preserves_existing_box() {
        let mut terrain = TerrainHeightsData::new();
        terrain.set_height_range(0.0, 2.0);
        let paths = WorldRaster::new(DirtyTilePublisher::unique());
        let transform = CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(0.0, 5.0, 0.0),
            scale: Vec3::ONE,
        };
        let mut blueprint = DecoratorBlueprint::new();
        blueprint.door_stairs_assembly = Some(Box::new(DoorStairsAssemblyParams { words: [37; 8] }));
        let previous = std::ptr::from_ref(&**blueprint.door_stairs_assembly.as_ref().unwrap());
        assert!(!check_for_door_stairs(&terrain, &paths, &transform, &bounds(), &mut blueprint));
        let retained = blueprint.door_stairs_assembly.as_ref().unwrap();
        assert_eq!(std::ptr::from_ref(&**retained), previous);
        assert_eq!(retained.words, [37; 8]);
    }

    #[test]
    fn actual_height_and_path_owners_replace_with_all_eight_original_fields() {
        let mut terrain = TerrainHeightsData::new();
        terrain.set_height_range(0.0, 2.0);
        // Both shared interpolation modes sample only these raised rows at the
        // origin, while the forward point (z=1.06) samples the lower rows.
        terrain.heights_mut()[255 * 512..257 * 512].fill(u16::MAX);
        let mut paths = WorldRaster::new(DirtyTilePublisher::unique());
        paths.composite_mut().fill(255);
        let transform = CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
        };
        let mut blueprint = DecoratorBlueprint::new();
        blueprint.door_stairs_assembly = Some(Box::new(DoorStairsAssemblyParams { words: [37; 8] }));
        let previous = std::ptr::from_ref(&**blueprint.door_stairs_assembly.as_ref().unwrap());
        assert!(check_for_door_stairs(&terrain, &paths, &transform, &bounds(), &mut blueprint));
        let output = blueprint.door_stairs_assembly.as_ref().unwrap();
        assert_ne!(std::ptr::from_ref(&**output), previous);
        assert_eq!(output.words.map(f32::from_bits), [0.0, 0.0, 0.0, 1.0, 2.0, -0.8, 0.0, 2.0]);
    }
}


from pathlib import Path
root=Path('D:/game')
path=root/'tiny-glade/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs'
baseline=Path(__file__).parent/'gothic_balcony_door.before.rs'
old=baseline.read_text(encoding='utf-8') if baseline.exists() else path.read_text(encoding='utf-8')
if not baseline.exists():
    baseline.write_text(old,encoding='utf-8')
tail=old[old.index('/// 0x1407f0900'):old.index('#[cfg(test)]')]
tail=tail.replace('    _mesh_lib: Res<DecoratorMeshLibrary>,','''    _mesh_lib: Res<DecoratorMeshLibrary>,
    terrain: Res<TerrainHeightsData>,
    paths: Res<WorldRaster<512>>,''')
start=tail.index('            // O：门前楼梯检测')
end=tail.index('            // O：蓝图 + 视觉实体。',start)
tail=tail[:start]+tail[end:]
tail=tail.replace('            blueprints.insert(id, blueprint);','''            check_for_door_stairs(
                &terrain,
                &paths,
                &transform,
                &d.collision_mesh().bounds,
                &mut blueprint,
            );
            blueprints.insert(id, blueprint);''')
start=tail.index('/// 门墙高来源')
end=tail.index('/// 哥特门判别',start)
tail=tail[:start]+tail[end:]
prefix='''//! Gothic door visual generator and the complete original door-stair leaf.
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
        decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorSubtypes},
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

'''
path.write_text(prefix+tail,encoding='utf-8')

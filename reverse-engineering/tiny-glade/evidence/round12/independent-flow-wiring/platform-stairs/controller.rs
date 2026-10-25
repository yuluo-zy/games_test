//! B1407744d0: dirty-wall reconstruction followed by all-cache hole publication.
//! Dependency internals remain owned by their existing libraries.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    geometry::instanced_wall::BrickTransformSsbo,
    resources::{terrain_height::{TerrainHeightSsbo, TerrainHeightsData}, walls::{
        WallId, decorator::DecoratorInfo,
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        public_walls::PublicWalls, publication_source::TerrainHeightGrid512,
        wall_holes::WallHoles,
    }},
    systems::{collision::world::{PlatformStairColliderMap, RaycastWorld},
        ui::signifiers::signifier_create::UiSignifierStream,
        wall::construction::{platform_segmentation::WallPlatformStepRange,
            platform_stairs::{WallPlatformDecoratorObserved, WallPlatformStairSource,
                collect_platform_stair_blockers, prepare_platform_stair_segments,
                platform_stair_geometry, platform_stair_holes},
            platform_stair_rows::platform_stair_rows, structures::MaterializedWallSegment}},
};
use crate::{construct_arches::WallConstructorOutput, gpu_output::{ConstructorKey, ConstructorPart}};

/// Native pending Vec@0,32B ranges map@18,104B segments map@48,collider map@78.
/// Collider ownership cannot be cloned independently of the canonical world.
#[derive(Resource, Debug, Default)]
pub struct PlatformStairsStorage {
    pub pending: Vec<u64>,
    pub segments: hashbrown::HashMap<u64, Vec<WallPlatformStepRange>>,
    pub materialized: hashbrown::HashMap<u64, Vec<MaterializedWallSegment>>,
    pub colliders: PlatformStairColliderMap,
}

/// Native HashMap row = u32 key,4 padding bytes,72B authority. History80B is a
/// different projection. Serialize named fields explicitly; never read padding.
pub fn observed_decorator(id: u32, info: &DecoratorInfo) -> WallPlatformDecoratorObserved {
    let mut raw = [0u8;80];
    raw[..4].copy_from_slice(&id.to_le_bytes());
    macro_rules! field { ($offset:expr,$value:expr) => {
        { let bytes=$value.to_le_bytes(); raw[8+$offset..8+$offset+bytes.len()].copy_from_slice(&bytes); }
    }; }
    field!(0x00,info.dst_wall); field!(0x08,info.dst_secondary);
    field!(0x10,info.dst_payload); field!(0x18,info.u18);
    field!(0x20,info.u20); field!(0x24,info.u24); field!(0x28,info.u28);
    field!(0x2c,info.decorator_id); field!(0x30,info.u30);
    field!(0x38,info.u38); field!(0x3c,info.u3c); field!(0x40,info.u40);
    raw[0x4c..0x50].copy_from_slice(&[info.rank,info.u45,info.ty,info.state]);
    WallPlatformDecoratorObserved(raw)
}

/// The original eight resources use interior-mutability for statistics and GPU
/// storage. ResMut expresses those same writes in this Rust owner design; the
/// ninth parameter is the already shared source-owned96B output, not a new pool.
pub fn construct_platform_stairs(
    mut holes: ResMut<WallHoles>,
    mut storage: ResMut<PlatformStairsStorage>,
    mut walls: ResMut<PublicWalls>,
    _signifiers: Res<UiSignifierStream>,
    mut ssbo: ResMut<AssetSsboLibrary>,
    heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    decorators: Res<DecoratorStorage>,
    mut raycast: ResMut<RaycastWorld>,
    mut output: ResMut<WallConstructorOutput>,
) {
    // Original drain consumes the dirty queue before reconstruction. Preserve
    // order and repeats; this consumer does not sort/dedup a second time.
    let pending: Vec<_> = storage.pending.drain(..).collect();
    let terrain = (!pending.is_empty()).then(|| TerrainHeightGrid512 {
        encoded: std::sync::Arc::from(heights.heights()),
        minimum_y: heights.min_height(), maximum_y: heights.max_height(),
        sampling_fix: utils::float_grid::CENTERED_PATH.load(std::sync::atomic::Ordering::Relaxed),
    });
    let mut exhausted = false;
    for wall_id in pending {
        storage.materialized.remove(&wall_id);
        let id = WallId(wall_id);
        let Some(wall) = walls.get_unchecked(id) else {
            raycast.remove_platform_stair_collider(&mut storage.colliders, id);
            continue;
        };
        if wall.shape().tag() >= 2 || !storage.segments.contains_key(&wall_id) {
            raycast.remove_platform_stair_collider(&mut storage.colliders, id);
            continue;
        }
        let terrain = terrain.as_ref().expect("pending wall has actual terrain snapshot");
        let observed: Vec<_> = decorators.get(DecoratorStorageAddr::Wall(id))
            .into_iter().flat_map(|group| group.decorators().iter())
            .map(|(&id,info)| observed_decorator(id,info)).collect();
        let blockers = collect_platform_stair_blockers(wall.simplified_curve_2d(),terrain,&observed)
            .expect("invalid authoritative manual-stair blocker input");
        let path = wall.prepared_path.as_ref().expect("platform requires prepared wall path");
        let public = wall.building_projection();
        let segments = prepare_platform_stair_segments(WallPlatformStairSource {
            raw_curve: wall.simplified_curve_2d(), public: public.observed_bytes(),
            prepared_positions: &path.positions, prepared_u: &path.points_u,
            segment_values: &path.segment_values,
            arch_heights: wall.arch_segment_heights().expect("platform requires initialized height arrays"),
            steps: &storage.segments[&wall_id], blockers: &blockers,
        }).expect("invalid authoritative platform materialization input");
        // The cache is published before geometry construction, as in the native
        // insertion at774d..7750. Render-curve skips do not erase its holes.
        storage.materialized.insert(wall_id,segments);
        let geometry = platform_stair_geometry(public.observed_bytes(),&path.positions,
            &path.points_u,&storage.materialized[&wall_id],terrain)
            .expect("invalid authoritative platform geometry input");
        for cell in &geometry.cells {
            let rows = platform_stair_rows(wall.shape(),
                country_core::resources::walls::publication_fields::WallId(wall_id),cell)
                .expect("invalid authoritative platform render input");
            let records: Vec<_> = rows.iter().map(|row| bytemuck::pod_read_unaligned::<BrickTransformSsbo>(row)).collect();
            let buffer = ssbo.get_mut::<BrickTransformSsbo>()
                .expect("canonical BrickTransformSsbo must exist before platform construction");
            exhausted |= output.gpu.append_bricks(buffer,
                ConstructorKey { wall:wall_id,part:ConstructorPart::PlatformStairs },&records) != 0;
        }
        raycast.update_platform_stair_collider(&mut storage.colliders,id,&geometry);
        walls.get_mut(id).expect("wall remains published during constructor")
            .material_stats.add_stairs_length(geometry.stairs_length);
    }
    if exhausted { log::error!("platform stair brick capacity exhausted; successful instances retained"); }
    // This is deliberately outside the dirty loop. Other28 producers clear and
    // repopulate InProgress holes every frame; cached stairs must participate.
    for (&wall_id,segments) in &storage.materialized {
        let id=WallId(wall_id);
        let wall=walls.get_unchecked(id).expect("cached platform wall must remain published");
        let generated=platform_stair_holes(wall.simplified_curve_2d(),wall.max_y(),segments)
            .expect("invalid authoritative cached platform hole input");
        holes.extend(id,generated).expect("platform holes require InProgress owner");
    }
}

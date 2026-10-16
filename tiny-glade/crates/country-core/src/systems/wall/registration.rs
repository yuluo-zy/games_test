//! Local CPU registration, not a reconstruction of the original global schedule.
use super::{
    adjust_wall_height::{adjust_wall_height, register_wall_height_events},
    calc_wall_bend::{WallBendChangedEvent, calc_wall_bend},
    construct_wall_bricks::{
        RecalculateWallBricks, WallBrickBuildState, WallBricksChanged, construct_wall_bricks,
        finalize_wall_holes,
    },
    decorator_wall_events::decorator_wall_events,
    delete_wall::delete_wall,
    wall_state::{
        publish_public_wall_state::{
            WallPublicationSubscriptions, collect_wall_publication_dirty, publish_public_wall_state,
        },
        wall_spatial_hash::{WallSpatialHash, spatial_hash_update},
    },
};
use crate::resources::{
    in_progress_curve::InProgressCurve,
    walls::{
        InnerWalls,
        decorator_storage::DecoratorStorage,
        public_walls::{
            OnWallChanged, OnWallDeleted, PublicWalls, WallEntities, WallPlasters,
            WallPublicationDirty,
        },
        wall_holes::{PrevWallHoles, WallHoles},
    },
};
use crate::systems::wall::{
    eraser::{
        AnimateWallBulldozeCmd, WallBulldozeAudio, WallEraserStroke,
        animate_wall_bulldoze::{AnimateWallBulldozeBuffer, animate_wall_bulldoze},
        archivist::{WallEraserArchivist, finish_wall_eraser},
        ui_bulldozer::wall_eraser_bulldoze,
        wall_eraser::wall_eraser,
    },
    shapes::{
        calc_shape_height::calc_shape_height,
        circle::{create_circle::create_circle, ui_create_circle::ui_create_circle},
        rectangle::{
            create_rectangle::create_rectangle, edit_rectangle::edit_rectangle,
            ui_create_rectangle::ui_create_rectangle,
        },
        register_shape_resources,
    },
    snapping::{
        DisableSnapping,
        snapping_audio::snapping_audio,
        snapping_init::{SnappingDebugDisabled, snapping_init},
        snapping_maintain::{
            DisableSnappingInput, check_for_disable_snapping, snapping_maintain,
            snapping_signifiers,
        },
        snapping_to_ends::snapping_to_ends,
        snapping_to_segments::snapping_to_segments,
        SnappingMode,
    },
};
use bevy_ecs::prelude::*;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WallCpuStage {
    Edit,
    Publish,
    Spatial,
    RoofDependencies,
    Holes,
    Bricks,
    Delete,
}

/// Real terrain/path buffers, GameTime and optional subscriptions are supplied by
/// their owners. Existing resources/events are preserved. Event retention is
/// managed by the application's normal Bevy event update system.
pub fn register_wall_cpu_resources(world: &mut World) {
    world.init_resource::<InnerWalls>();
    world.init_resource::<PublicWalls>();
    world.init_resource::<WallEntities>();
    world.init_resource::<WallPlasters>();
    world.init_resource::<WallSpatialHash>();
    world.init_resource::<WallPublicationDirty>();
    world.init_resource::<WallHoles>();
    world.init_resource::<PrevWallHoles>();
    world.init_resource::<WallBrickBuildState>();
    world.init_resource::<Events<OnWallChanged>>();
    world.init_resource::<Events<OnWallDeleted>>();
    world.init_resource::<Events<WallBendChangedEvent>>();
    world.init_resource::<Events<RecalculateWallBricks>>();
    world.init_resource::<Events<WallBricksChanged>>();
    // 装饰权威存储资源（卡5 decorator_storage；OnWallDeleted 消费链）。
    world.init_resource::<DecoratorStorage>();
    register_wall_height_events(world);
    crate::systems::roof::register_roof_events(world);
    // R4+R5: snapping / eraser / shapes (valid_enclosure is a pure type).
    world.init_resource::<InProgressCurve>();
    world.init_resource::<SnappingMode>();
    world.init_resource::<SnappingDebugDisabled>();
    world.init_resource::<DisableSnappingInput>();
    world.init_resource::<Events<DisableSnapping>>();
    world.init_resource::<AnimateWallBulldozeBuffer>();
    world.init_resource::<WallEraserArchivist>();
    world.init_resource::<Events<AnimateWallBulldozeCmd>>();
    world.init_resource::<Events<WallBulldozeAudio>>();
    world.init_resource::<Events<WallEraserStroke>>();
    // snapping_audio emits the audio-domain PlayWallUiSound event.
    world.init_resource::<Events<crate::systems::audio::PlayWallUiSound>>();
    register_shape_resources(world);
    crate::systems::wall::history_recording::register_wall_recording_events(world);
    world.init_resource::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>();
    // Snapping / shape consumers read the real cursor raycast and terrain grid.
    world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
    world.init_resource::<crate::resources::terrain_height::TerrainHeightsData<
        crate::resources::terrain_height::TerrainHeightSsbo,
    >>();
}

/// Roof association/spatial/gable systems belong in RoofDependencies. Actual
/// opening producers belong before Holes. Clearing holes is explicit because
/// blindly clearing them without registering all producers would lose inputs.
pub fn add_wall_cpu_systems(schedule: &mut Schedule) {
    schedule.configure_sets(
        (
            WallCpuStage::Edit,
            WallCpuStage::Publish,
            WallCpuStage::Spatial,
            WallCpuStage::Delete,
            WallCpuStage::RoofDependencies,
            WallCpuStage::Holes,
            WallCpuStage::Bricks,
        )
            .chain(),
    );
    schedule.add_systems(adjust_wall_height.in_set(WallCpuStage::Edit));
    // R4+R5 edit systems: snapping candidates, eraser and shape creation/editing
    // all mutate InnerWalls before publication.
    schedule.add_systems(
        (
            snapping_init,
            snapping_to_ends,
            snapping_to_segments,
            check_for_disable_snapping,
            snapping_maintain,
            wall_eraser,
            calc_shape_height,
            create_circle,
            create_rectangle,
            edit_rectangle,
            ui_create_circle,
            ui_create_rectangle,
        )
            .in_set(WallCpuStage::Edit),
    );
    schedule.add_systems(
        (
            animate_wall_bulldoze,
            finish_wall_eraser,
            wall_eraser_bulldoze,
            snapping_audio,
            snapping_signifiers,
        )
            .after(WallCpuStage::Edit)
            .before(WallCpuStage::Publish),
    );
    schedule.add_systems(publish_public_wall_state.in_set(WallCpuStage::Publish));
    schedule.add_systems(
        collect_wall_publication_dirty
            .before(publish_public_wall_state)
            .run_if(
                bevy_ecs::schedule::common_conditions::resource_exists::<
                    WallPublicationSubscriptions,
                >,
            ),
    );
    schedule.add_systems((spatial_hash_update, calc_wall_bend).in_set(WallCpuStage::Spatial));
    schedule.add_systems(finalize_wall_holes.in_set(WallCpuStage::Holes));
    schedule.add_systems(construct_wall_bricks.in_set(WallCpuStage::Bricks));
    schedule.add_systems(delete_wall.in_set(WallCpuStage::Delete));
    // OnWallDeleted → DecoratorStorage::on_wall_removed（RE §5 消费链；分裂/合并
    // 迁移接线待 wall_compound_edit 拓扑）。
    schedule.add_systems(decorator_wall_events.in_set(WallCpuStage::Delete));
}

pub fn clear_wall_holes_for_next_frame(
    mut holes: ResMut<WallHoles>,
    mut previous: ResMut<PrevWallHoles>,
) {
    holes.clear_for_next_frame(&mut previous);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::instanced_wall::InstancedWall,
        recovered_building::{
            wall_collection::TerrainHeightGrid512, wall_constructor::WallBrickInstance,
        },
        resources::{
            walls::{WallState, wall_trigger_volume::WallTriggerVolume},
            world_raster::DirtyTilePublisher,
            world_raster_defs::{
                Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster,
            },
        },
        systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain,
        utils::dirty_tiles::DirtyTiles,
    };

    #[test]
    fn ecs_wall_create_update_dirty_publication_and_delete() {
        let mut world = World::new();
        register_wall_cpu_resources(&mut world);
        world.insert_resource(crate::resources::time::GameTime {
            delta: 0.1,
            ..Default::default()
        });
        // Explicit full-size input fixtures exercise the real named buffer path.
        world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        }));
        world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        let curve =
            system_roof::surface::Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        // Original freehand tag 2, source +65 clear, height type +66 follow terrain.
        let id = world
            .resource_mut::<InnerWalls>()
            .new_wall(WallState::new(curve, 2.0, 0, 0, false));
        let mut schedule = Schedule::default();
        add_wall_cpu_systems(&mut schedule);
        schedule.run(&mut world);
        let entity = world.resource::<WallEntities>().wall_entity(id).unwrap();
        assert!(world.resource::<WallBrickBuildState>().failures.is_empty());
        let old_bricks = world
            .get::<InstancedWall<WallBrickInstance>>(entity)
            .unwrap()
            .brick_transforms
            .clone();
        assert!(!old_bricks.is_empty());
        assert!(
            !world
                .get::<WallTriggerVolume>(entity)
                .unwrap()
                .default_tris()
                .is_empty()
        );
        assert!(
            world
                .resource::<PublicWalls>()
                .get(id)
                .unwrap()
                .prepared_path
                .is_some()
        );
        assert!(
            world
                .resource::<WallSpatialHash>()
                .segments_for_wall(id)
                .next()
                .is_some()
        );

        world
            .resource_mut::<Events<super::super::adjust_wall_height::AdjustWallHeightCmd>>()
            .send(super::super::adjust_wall_height::AdjustWallHeightCmd(
                crate::recovered_building::wall_height_command::AdjustWallHeightRequest {
                    anchor: None,
                    mode: 1,
                    value: 3.0,
                    wall: crate::recovered_building::wall_state::WallId(id.0),
                    observed_audio_flag: 1,
                },
            ));
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<InnerWalls>().get_unwrapped(id).height(),
            3.0
        );
        assert_eq!(
            world.resource::<WallEntities>().wall_entity(id),
            Some(entity)
        );
        assert_ne!(
            world
                .get::<InstancedWall<WallBrickInstance>>(entity)
                .unwrap()
                .brick_transforms,
            old_bricks
        );
        assert!(world.resource::<WallBrickBuildState>().failures.is_empty());

        // Installing real cursors after registration activates the optional
        // gatherer, without callers having to wire another system manually.
        let mut primary = tabloid::Publisher::<DirtyTiles>::new();
        let mut secondary = tabloid::Publisher::<DirtyTiles>::new();
        world.insert_resource(WallPublicationSubscriptions {
            primary: primary.subscribe(),
            secondary: secondary.subscribe(),
        });
        primary.publish(DirtyTiles::new_all_dirty()).unwrap();
        schedule.run(&mut world);
        assert!(
            world
                .resource::<PublicWalls>()
                .last_delta
                .changed
                .contains(&id)
        );
        assert!(world.resource::<WallPublicationDirty>().primary.is_empty());

        world.resource_mut::<InnerWalls>().remove(id);
        schedule.run(&mut world);
        assert!(world.resource::<PublicWalls>().get(id).is_none());
        assert!(world.resource::<WallEntities>().wall_entity(id).is_none());
        assert!(world.get_entity(entity).is_err());
        assert!(
            world
                .resource::<WallSpatialHash>()
                .segments_for_wall(id)
                .next()
                .is_none()
        );
        assert!(world.resource::<WallBrickBuildState>().failures.is_empty());
    }
}

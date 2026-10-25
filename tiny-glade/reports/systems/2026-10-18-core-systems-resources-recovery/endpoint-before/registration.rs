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
        ui_bulldozer::{BulldozerRayPick, enqueue_bulldoze_command, wall_eraser_bulldoze},
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
        DisableSnapping, SnappingMode,
        snapping_audio::snapping_audio,
        snapping_init::{SnappingDebugDisabled, snapping_init},
        snapping_maintain::{
            DisableSnappingInput, check_for_disable_snapping, snapping_maintain,
            snapping_signifiers,
        },
        snapping_to_ends::snapping_to_ends,
        snapping_to_segments::snapping_to_segments,
        telegraph_walls_are_merged::{TelegraphTimers, telegraph_walls_are_merged},
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
    world.init_resource::<crate::resources::rt::RtWorld>();
    world.init_resource::<Events<crate::resources::rt::UpdateRtWorldEvent>>();
    // 装饰权威存储资源（卡5 decorator_storage；OnWallDeleted 消费链）。
    world.init_resource::<DecoratorStorage>();
    register_wall_height_events(world);
    crate::resources::roofs::register_roof_events(world);
    // R4+R5: snapping / eraser / shapes (valid_enclosure is a pure type).
    world.init_resource::<InProgressCurve>();
    world.init_resource::<SnappingMode>();
    world.init_resource::<SnappingDebugDisabled>();
    world.init_resource::<DisableSnappingInput>();
    world.init_resource::<TelegraphTimers>();
    world.init_resource::<Events<DisableSnapping>>();
    world.init_resource::<AnimateWallBulldozeBuffer>();
    world.init_resource::<WallEraserArchivist>();
    world.init_resource::<BulldozerRayPick>();
    world.init_resource::<Events<AnimateWallBulldozeCmd>>();
    world.init_resource::<Events<WallBulldozeAudio>>();
    world.init_resource::<Events<WallEraserStroke>>();
    // snapping_audio emits the audio-domain PlayWallUiSound event.
    world.init_resource::<Events<crate::systems::audio::PlayWallUiSound>>();
    register_shape_resources(world);
    // Shape creation colour policy inputs (`get_building_and_roof_colors`
    // 0x140a94880): wall/halftimber colour tables, the persisted customisation
    // defaults, and the dense roof-component tables (producer seam).
    world.init_resource::<crate::resources::color_id::WallColorIds>();
    world.init_resource::<crate::resources::color_id::HalftimberColorIds>();
    world.init_resource::<crate::resources::glade_color_customization::GladeColorCustomization>();
    world.init_resource::<crate::resources::walls::color_selection::RoofColorComponentTables>();
    crate::systems::wall::history_recording::register_wall_recording_events(world);
    world.init_resource::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>();
    // Snapping / shape consumers read the real cursor raycast and terrain grid.
    world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
    world.init_resource::<crate::resources::terrain_height::TerrainHeightsData<
        crate::resources::terrain_height::TerrainHeightSsbo,
    >>();
    register_wall_drawing_resources(world);
}

/// Preserve live owners. Camera, window, border and clock are supplied by their
/// actual owners; this does not synthesize successful input/render resources.
pub fn register_wall_drawing_resources(world: &mut World) {
    use super::{
        draw_wall::{CurveNursery, RecordMouseInputIntoWall, WallDrawingIssues},
        finish_wall::{ContinueWallOriginalState, FinishWall},
        ui_continue_wall::SuppressContinueWall,
        ui_draw_wall::WallDrawState,
    };
    use crate::resources::{
        color_id::{SetHalftimberColorIdCmd, SetWallColorIdCmd},
        glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder},
        onboarding::NotifyHintSystemToolWasJustUsed,
        tree_storage::HideTreesNextToWallCmd,
        walls::decorator_archivist::LiveDecoratorArchivist,
    };
    world.init_resource::<InProgressCurve>();
    world.init_resource::<CurveNursery>();
    world.init_resource::<WallDrawState>();
    world.init_resource::<WallDrawingIssues>();
    world.init_resource::<ContinueWallOriginalState>();
    world.init_resource::<super::deferred_wall_history::PendingWallHistory>();
    world.init_resource::<crate::resources::color_id::WallColorIds>();
    world.init_resource::<crate::resources::color_id::HalftimberColorIds>();
    world.init_resource::<SuppressContinueWall>();
    world.init_resource::<LiveDecoratorArchivist>();
    world.init_resource::<Events<RecordMouseInputIntoWall>>();
    world.init_resource::<Events<FinishWall>>();
    world.init_resource::<Events<SetWallColorIdCmd>>();
    world.init_resource::<Events<SetHalftimberColorIdCmd>>();
    world.init_resource::<Events<HideTreesNextToWallCmd>>();
    world.init_resource::<Events<DisplayOutOfBorder>>();
    world.init_resource::<Events<FeedbackInputOutOfBorder>>();
    world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
    world.init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>();
    world.init_resource::<Events<crate::systems::audio::WallBendAudioParamReset>>();
    world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
}

/// Local input closure. UI must register the first canvas before recording
/// starts; completion must be recorded before the existing history finish.
/// This joins existing consumers, without registering another history owner.
pub fn add_wall_drawing_cpu_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    use super::{
        deferred_wall_history::history_record_wall_changed, draw_wall::draw_wall,
        finish_wall::finish_wall, ui_continue_wall::suppress_continue_wall,
        ui_draw_wall::ui_draw_wall,
    };
    use crate::systems::{
        backend::input::InputSystemSet,
        history::{HistoryCpuStage, recording::history_start_recording},
        mode_manager::ModeChangePublication,
    };
    schedule.add_systems(
        (
            ui_draw_wall.before(history_start_recording::<S>),
            suppress_continue_wall,
            draw_wall.after(history_start_recording::<S>),
            finish_wall.before(adjust_wall_height),
            history_record_wall_changed,
        )
            .chain()
            .in_set(WallCpuStage::Edit)
            .after(InputSystemSet::Derive)
            .after(ModeChangePublication)
            .before(HistoryCpuStage::Record),
    );
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
    schedule.add_systems(
        adjust_wall_height
            .after(super::finish_wall::finish_wall)
            .in_set(WallCpuStage::Edit),
    );
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
            // 归属核对（fresh）：缓冲入队是 native 跳表目标 handler，须在
            // ray-pick 生产者（wall_eraser_bulldoze）之后消费 AnimateWallBulldozeCmd。
            enqueue_bulldoze_command.after(wall_eraser_bulldoze),
            snapping_audio,
            snapping_signifiers,
        )
            .after(WallCpuStage::Edit)
            .before(WallCpuStage::Publish),
    );
    schedule.add_systems(publish_public_wall_state.in_set(WallCpuStage::Publish));
    // telegraph_walls_are_merged 读 public 墙（Publish 后），每帧 easing 计时驱动
    // 合并预览线（render-host 实体链登记）。
    schedule.add_systems(telegraph_walls_are_merged.after(WallCpuStage::Publish));
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
    use crate::geometry::instanced_wall::InstancedWall;
    use crate::resources::walls::WallState;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::wall_trigger_volume::WallTriggerVolume;
    use crate::resources::world_raster::DirtyTilePublisher;
    use crate::resources::world_raster_defs::Cobblestone1Raster;
    use crate::resources::world_raster_defs::Cobblestone2Raster;
    use crate::resources::world_raster_defs::Cobblestone3Raster;
    use crate::resources::world_raster_defs::PathRaster;
    use crate::systems::wall::construction::bricks::WallBrickInstance;
    use crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain;
    use crate::utils::dirty_tiles::DirtyTiles;

    fn wait_for_actual_bvh(
        glue: &crate::resources::rt::RtBvhBuilderGlueResource,
    ) -> crate::resources::rt::RtUploadData {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            if let Some(output) = glue.take_completed() {
                return output;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "wall RT producer did not reach the actual BVH worker"
            );
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

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
        let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[0.0, 0.0], [4.0, 0.0]],
            false,
        )
        .unwrap();
        // Original freehand tag 2, source +65 clear, height type +66 follow terrain.
        let id = world
            .resource_mut::<InnerWalls>()
            .new_wall(WallState::new(curve, 2.0, 0, 0, false));
        let mut schedule = Schedule::default();
        add_wall_cpu_systems(&mut schedule);
        let rt_glue = crate::resources::rt::RtBvhBuilderGlueResource::default();
        let _worker =
            crate::resources::rt::bvh_builder::RtBvhWorker::spawn(rt_glue.clone()).unwrap();
        world.insert_resource(rt_glue.clone());
        schedule.add_systems(crate::resources::rt::update_rt_world.after(WallCpuStage::Bricks));
        schedule.run(&mut world);
        let entity = world.resource::<WallEntities>().wall_entity(id).unwrap();
        assert!(world.resource::<WallBrickBuildState>().failures.is_empty());
        let old_bricks = world
            .get::<InstancedWall<WallBrickInstance>>(entity)
            .unwrap()
            .pending_instances()
            .to_vec();
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
        let wall_rt = world.resource::<crate::resources::rt::RtWorld>();
        let mesh = wall_rt
            .get(crate::resources::rt::RtMeshHandle::from_wall_id(id))
            .unwrap();
        let expected_positions: Vec<_> = world
            .get::<WallTriggerVolume>(entity)
            .unwrap()
            .default_tris()
            .iter()
            .flat_map(|triangle| triangle.points)
            .collect();
        assert_eq!(mesh.positions, expected_positions);
        assert_eq!(mesh.color, 0x0100_0000 | id.get_inner() as u32);
        let bvh = wait_for_actual_bvh(&rt_glue);
        assert!(!bvh.bvh.is_empty());
        assert_eq!(bvh.triangles.len(), bvh.metadata.len());
        assert!(
            bvh.metadata
                .iter()
                .all(|meta| meta.packed_color == mesh.color)
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
                crate::systems::wall::height_command::AdjustWallHeightRequest {
                    anchor: None,
                    mode: 1,
                    value: 3.0,
                    wall: crate::resources::walls::publication_fields::WallId(id.0),
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
                .pending_instances(),
            old_bricks
        );
        assert!(world.resource::<WallBrickBuildState>().failures.is_empty());

        let replacement: Vec<_> = world
            .get::<WallTriggerVolume>(entity)
            .unwrap()
            .default_tris()
            .iter()
            .flat_map(|triangle| triangle.points)
            .collect();
        assert_eq!(
            world
                .resource::<crate::resources::rt::RtWorld>()
                .get(crate::resources::rt::RtMeshHandle::from_wall_id(id))
                .unwrap()
                .positions,
            replacement
        );
        assert!(!wait_for_actual_bvh(&rt_glue).bvh.is_empty());

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
        assert!(!wait_for_actual_bvh(&rt_glue).bvh.is_empty());

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
        assert!(
            world
                .resource::<crate::resources::rt::RtWorld>()
                .meshes
                .is_empty()
        );
        let removed = wait_for_actual_bvh(&rt_glue);
        assert!(removed.bvh.is_empty());
        assert!(removed.triangles.is_empty());
        assert!(removed.metadata.is_empty());
    }
}

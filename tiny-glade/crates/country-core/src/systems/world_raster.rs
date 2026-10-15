//! B 0x140957bf0 consumes upload requests through its EventReader cursor.
//! Native global scheduling and history command producers are tracked separately.
use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::resources::world_raster_defs::WorldRasterResources;
use crate::{
    asset_libraries::ssbo_library::SsboType,
    render::ssbo::MappedStorageBuffer,
    resources::{
        active_raster_stroke::ActiveRasterStrokesStorage,
        cursor_terrain_raycast::CursorTerrainRaycast, onboarding::NotifyHintSystemToolWasJustUsed,
        walls::decorator_archivist::DecoratorArchivist, world_raster::WorldRaster,
    },
    systems::history::history_process::DebugHistoryMsg,
};
use bevy_ecs::event::Events;
use bevy_ecs::{event::EventReader, system::ResMut};
use glam::Vec3;

pub use crate::resources::world_raster::UploadWorldRasterBuffersToStagingCmd;

/// B 0x140957990. A caller's enum-variant constructor supplies the edit identity;
/// the same HistoryTransactions owner is shared with modes and building edits.
pub fn record_raster_history<M, S>(
    strokes: &mut ActiveRasterStrokesStorage,
    slot: u8,
    history: &mut crate::recovered_building::history_transactions::HistoryTransactions<M, S>,
    construct_edit: impl FnOnce(std::sync::Arc<crate::resources::raster_stroke::RasterStroke>) -> M,
) -> Result<(), crate::recovered_building::history_transactions::HistoryTransactionError> {
    let stroke = strokes[slot].take_history_stroke();
    history.add(construct_edit(stroke))
}

/// Logical projection, not Rust enum ABI or author field names. Discriminants
/// 0/1 begin (and become blend flags), 2 extends, >=3 commits. Begin alone reads
/// the other fields. B 0x142260b42 and 0x1422610d1 establish this ordering.
#[derive(Clone, Copy, Debug)]
pub struct RasterStrokeCmd {
    pub discriminant: u32,
    pub strength: f32,
    pub radius: f32,
    pub draw: bool,
}

/// Actual borrowed owners; B PDB init_state 0x140958040 establishes the types.
/// Mapped SSBO storage is borrowed separately by draw_raster.
pub struct RasterDrawResources<'a, C: Send + Sync + 'static> {
    pub strokes: &'a mut ActiveRasterStrokesStorage,
    pub archivist: &'a mut DecoratorArchivist<C>,
    pub cursor: &'a CursorTerrainRaycast,
    pub tool_used: &'a mut Events<NotifyHintSystemToolWasJustUsed>,
    pub debug_history: &'a mut Events<DebugHistoryMsg>,
}

/// B state machine, with the original three caller-owned closures: draw sample,
/// erase sample, record-history notification. Concrete event types belong to
/// Path/Cobblestone/Water/Garden. Both sample callbacks receive the observed false
/// byte; its author field name remains outside this generic function's contract.
#[allow(clippy::too_many_arguments)]
pub fn draw_raster<T, C>(
    command: RasterStrokeCmd,
    point: Vec3,
    slot: u8,
    resources: &mut RasterDrawResources<'_, C>,
    raster: &mut WorldRaster<512>,
    library: &mut AssetSsboLibrary,
    on_draw: impl FnMut(Vec3, f32, bool),
    on_erase: impl FnMut(Vec3, f32, bool),
    on_record: impl FnMut(),
) where
    T: SsboType<Storage = MappedStorageBuffer<'static, T>> + bytemuck::Pod,
    C: Send + Sync + 'static,
{
    draw_raster_with_staging(
        command,
        point,
        slot,
        resources,
        raster,
        |raster| {
            // Lookup occurs after mutation and, on commit, after notifications.
            let staging = library
                .get_mut::<T>()
                .expect("called `Option::unwrap()` on a `None` value");
            raster.copy_to_staging(1, bytemuck::cast_slice_mut(staging.mapped));
        },
        on_draw,
        on_erase,
        on_record,
    );
}

#[allow(clippy::too_many_arguments, clippy::neg_cmp_op_on_partial_ord)]
fn draw_raster_with_staging<C: Send + Sync + 'static>(
    command: RasterStrokeCmd,
    point: Vec3,
    slot: u8,
    resources: &mut RasterDrawResources<'_, C>,
    raster: &mut WorldRaster<512>,
    mut stage: impl FnMut(&mut WorldRaster<512>),
    mut on_draw: impl FnMut(Vec3, f32, bool),
    mut on_erase: impl FnMut(Vec3, f32, bool),
    mut on_record: impl FnMut(),
) {
    let stroke = &mut resources.strokes[slot];
    match command.discriminant {
        0 | 1 => {
            resources.archivist.decorator_cull_allow_any();
            stroke.start_new(
                resources.cursor.position(),
                command.draw as u8,
                command.radius,
                command.discriminant,
                command.strength,
            );
        }
        2 => {
            let delta = point - *stroke.last();
            // Native y*y+x*x, then z*z; ucomiss+jbe rejects unordered NaN too.
            let distance = ((delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z).sqrt();
            if !(distance > 0.001) {
                return;
            }
            stroke.add_point(point);
            stroke.resample_and_smooth();
            let mut dirty = crate::utils::dirty_tiles::DirtyTiles::default();
            dirty.rasterize_world_space_stroke(stroke.resampled(), stroke.brush_size());
            *stroke.dirty_current_mut() = dirty;
            raster.extend_dirty(stroke.dirty_current());
            raster.extend_dirty(stroke.dirty_previous());
            stroke.swap_dirty();
            raster.copy_composite_to_scratch();
            raster.rasterize_stroke(
                1,
                stroke.resampled(),
                stroke.brush_size(),
                stroke.mode() != 0,
                stroke.param_a(),
                stroke.param_b(),
            );
            stage(raster);
            if stroke.mode() != 0 {
                on_draw(point, stroke.brush_size(), false);
            } else {
                on_erase(point, stroke.brush_size(), false);
            }
        }
        _ => {
            if stroke.resampled().len() < 2 {
                return;
            }
            resources.debug_history.send(DebugHistoryMsg);
            on_record();
            resources.tool_used.send(NotifyHintSystemToolWasJustUsed);
            raster.extend_dirty(stroke.dirty_previous());
            stroke.dirty_previous_mut().clear();
            raster.commit_scratch_to_composite();
            stage(raster);
        }
    }
}

pub fn upload_world_raster_buffers_to_staging(
    mut commands: EventReader<UploadWorldRasterBuffersToStagingCmd>,
    mut rasters: WorldRasterResources,
    mut library: ResMut<AssetSsboLibrary>,
) {
    if commands.read().last().is_some() {
        rasters.upload_to_staging(&mut library);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{
        walls::decorator_archivist::DecoratorCullPolicy, world_raster::DirtyTilePublisher,
    };
    use std::{
        cell::Cell,
        collections::{HashMap, HashSet},
    };

    fn archive() -> DecoratorArchivist<()> {
        DecoratorArchivist {
            walls: HashMap::new(),
            special1: None,
            special2: None,
            collision_a: HashMap::new(),
            collision_b: HashMap::new(),
            culling: DecoratorCullPolicy {
                allowed_ids: HashSet::new(),
                allow_any: false,
            },
        }
    }

    #[test]
    fn stroke_preview_commit_and_notifications_share_real_buffers() {
        let mut strokes = ActiveRasterStrokesStorage::default();
        let mut archivist = archive();
        let mut cursor = CursorTerrainRaycast::new();
        cursor.update(Some(Vec3::new(-2.0, 0.0, 0.0)));
        let (mut tool_used, mut debug_history) = (Events::default(), Events::default());
        let mut resources = RasterDrawResources {
            strokes: &mut strokes,
            archivist: &mut archivist,
            cursor: &cursor,
            tool_used: &mut tool_used,
            debug_history: &mut debug_history,
        };
        let mut raster = WorldRaster::new(DirtyTilePublisher::unique());
        let mut staging = vec![0; 512 * 512];
        let (draws, records, uploads) = (Cell::new(0), Cell::new(0), Cell::new(0));
        let cmd = RasterStrokeCmd {
            discriminant: 0,
            strength: 1.0,
            radius: 1.0,
            draw: true,
        };
        for tag in [0, 2, 3] {
            draw_raster_with_staging(
                RasterStrokeCmd {
                    discriminant: tag,
                    ..cmd
                },
                Vec3::new(2.0, 0.0, 0.0),
                4,
                &mut resources,
                &mut raster,
                |r| {
                    uploads.set(uploads.get() + 1);
                    r.copy_to_staging(1, &mut staging);
                },
                |point, radius, flag| {
                    assert_eq!(point.x, 2.0);
                    assert_eq!(radius, 1.0);
                    assert!(!flag);
                    draws.set(draws.get() + 1);
                },
                |_, _, _| panic!("draw must not emit erase"),
                || records.set(records.get() + 1),
            );
            if tag == 2 {
                assert!(raster.buffer(0).iter().all(|b| *b == 0));
                assert!(raster.buffer(1).iter().any(|b| *b != 0));
            }
        }
        assert_eq!((draws.get(), records.get(), uploads.get()), (1, 1, 2));
        assert_eq!(raster.buffer(0), raster.buffer(1));
        assert!(resources.archivist.culling.allow_any);
        assert_eq!(resources.debug_history.len(), 1);
        assert_eq!(resources.tool_used.len(), 1);
        assert!(resources.strokes[4].dirty_previous().is_empty());
        assert!(staging.iter().any(|b| *b != 0));
    }

    #[test]
    fn threshold_nan_and_one_point_commit_have_no_publish_side_effects() {
        let mut strokes = ActiveRasterStrokesStorage::default();
        let mut archivist = archive();
        let cursor = CursorTerrainRaycast::new();
        let (mut tool_used, mut debug_history) = (Events::default(), Events::default());
        let mut resources = RasterDrawResources {
            strokes: &mut strokes,
            archivist: &mut archivist,
            cursor: &cursor,
            tool_used: &mut tool_used,
            debug_history: &mut debug_history,
        };
        let mut raster = WorldRaster::new(DirtyTilePublisher::unique());
        let cmd = RasterStrokeCmd {
            discriminant: 1,
            strength: 0.25,
            radius: 2.0,
            draw: true,
        };
        for (tag, point) in [
            (1, Vec3::ZERO),
            (2, Vec3::X * 0.001),
            (2, Vec3::splat(f32::NAN)),
            (3, Vec3::ZERO),
        ] {
            draw_raster_with_staging(
                RasterStrokeCmd {
                    discriminant: tag,
                    ..cmd
                },
                point,
                0,
                &mut resources,
                &mut raster,
                |_| panic!("must not upload"),
                |_, _, _| panic!("must not draw"),
                |_, _, _| panic!("must not erase"),
                || panic!("must not record"),
            );
        }
        assert_eq!(resources.strokes[0].param_a(), 1);
        assert_eq!(resources.strokes[0].param_b(), 0.25);
        assert_eq!(resources.strokes[0].resampled(), &[Vec3::ZERO]);
        assert!(resources.debug_history.is_empty() && resources.tool_used.is_empty());
    }

    #[test]
    fn history_takes_curve_and_keeps_active_input_and_blend_parameters() {
        use crate::recovered_building::history_transactions::{
            HistoryTransactions, ObservedHistoryLedger,
        };
        let mut strokes = ActiveRasterStrokesStorage::default();
        strokes[2].start_new(Vec3::ZERO, 1, 3.0, 1, 0.4);
        strokes[2].add_point(Vec3::X);
        strokes[2].resample_and_smooth();
        let expected = strokes[2].resampled().to_vec();
        let input = strokes[2].in_progress().input().to_vec();
        let mut history = HistoryTransactions::<_, ()>::from_observed(ObservedHistoryLedger {
            entries: vec![],
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 0,
            mutation_nonce: 0,
        });
        history.start_input_recording(0).unwrap();
        record_raster_history(&mut strokes, 2, &mut history, |stroke| stroke).unwrap();
        let saved = &history.pending().unwrap().edits[0];
        assert_eq!(&*saved.points, expected);
        assert_eq!(
            (saved.radius, saved.draw, saved.flags, saved.strength),
            (3.0, true, 1, 0.4)
        );
        assert!(strokes[2].resampled().is_empty());
        assert_eq!((strokes[2].brush_size(), strokes[2].mode()), (0.0, 0));
        assert_eq!(strokes[2].in_progress().input(), input);
        assert_eq!((strokes[2].param_a(), strokes[2].param_b()), (1, 0.4));
    }
}

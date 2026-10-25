//! `replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, closure_env$2>` (0x1409f4650).
//! Evidence: water-cpu-analysis "历史回放恢复水位". Water is restored by re-running the same
//! rasterize_stroke -> staging -> composite chain for each stored `ReplayCmd` (48 B in the original).

use super::raster::{BufferSel, WorldRaster};
use super::stroke::{StrokeParams, WriteMode};
use super::tool::{WaterDrawEvent, WaterToolConfig};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Weight threshold that enters the blend branch (fVar5 >= 0.01).
pub const BLEND_MIN_WEIGHT: f32 = 0.01;
/// Weight at/above which scratch is committed to composite (fVar36 >= 1.0).
pub const COMMIT_WEIGHT: f32 = 1.0;

/// Subset of `ReplayCmd`: points, weight (f32 @+0x14), clear flag (+0x18), processed (+0x19),
/// event flag (+0x1a), nested commands for clear records.
#[derive(Clone, Debug, Default)]
pub struct ReplayCmd {
    pub points: Vec<Vec3>,
    pub weight: f32,
    pub clear: bool,
    pub processed: bool,
    pub emit_event: bool,
    pub nested: Vec<ReplayCmd>,
}

/// `RecalculateHeightMapCmd` (EventWriter of the replay_raster FunctionSystem). UNKNOWN(evidence:
/// water-cpu-analysis replay_raster): when it is written; the water instance never writes it here.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RecalculateHeightMapCmd;

/// `Res<ScheduledTerrainReadback>` (terrain readback scheduling; unused by the water instance).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ScheduledTerrainReadback;

/// `RasterReplayResources`: the queue of stored `ReplayCmd`s plus the accumulated replay statistics.
#[derive(Resource, Clone, Debug, Default)]
pub struct ReplayQueue {
    pub cmds: Vec<ReplayCmd>,
    pub stats: ReplayStats,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplayStats {
    pub strokes: u32,
    pub clears: u32,
    pub committed: u32,
    /// Completion events (closure$2 -> atomic counter increment).
    pub events: u32,
}

/// `replay_raster` as a system. Signature (FunctionSystem wrapper): `(RasterReplayResources,
/// EventWriter<WaterDrawEvent>, ResMut<WaterRaster>, EventWriter<RecalculateHeightMapCmd>,
/// Res<ScheduledTerrainReadback>)`; the stroke width comes from `WaterToolConfig`.
pub fn replay_raster(
    mut queue: ResMut<ReplayQueue>,
    mut draws: EventWriter<WaterDrawEvent>,
    mut raster: ResMut<WorldRaster>,
    _recalc: EventWriter<RecalculateHeightMapCmd>,
    _readback: Res<ScheduledTerrainReadback>,
    config: Res<WaterToolConfig>,
) {
    if queue.cmds.is_empty() {
        return;
    }
    let cmds = std::mem::take(&mut queue.cmds);
    let (remaining, stats) = replay_commands(&mut raster, cmds, config.half_width_cells);
    queue.cmds = remaining;
    queue.stats.strokes += stats.strokes;
    queue.stats.clears += stats.clears;
    queue.stats.committed += stats.committed;
    queue.stats.events += stats.events;
    for _ in 0..stats.events {
        draws.write(WaterDrawEvent);
    }
}

/// Replay every unprocessed command; returns remaining commands (processed ones removed, matching
/// the `Vec<usize>` memmove at the end of the original).
///
/// UNKNOWN(evidence: water-cpu-analysis replay_raster): `cubic_in_out` easing / `Curve` resampling
/// of the points and the exact weight -> value conversion; the stroke is rasterized as stored.
/// Brush weight < 1 leaves the result in scratch only (commit requires weight >= 1.0 as in the doc).
pub fn replay_commands(
    raster: &mut WorldRaster,
    cmds: Vec<ReplayCmd>,
    half_width_cells: f32,
) -> (Vec<ReplayCmd>, ReplayStats) {
    let mut stats = ReplayStats::default();
    let mut remaining = Vec::new();
    for mut cmd in cmds {
        if cmd.processed {
            remaining.push(cmd);
            continue;
        }
        apply(raster, &mut cmd, half_width_cells, &mut stats);
        cmd.processed = true;
    }
    (remaining, stats)
}

fn apply(raster: &mut WorldRaster, cmd: &mut ReplayCmd, hw: f32, stats: &mut ReplayStats) {
    if cmd.clear {
        raster.clear_buffer();
        stats.clears += 1;
        for nested in cmd.nested.iter_mut() {
            apply(raster, nested, hw, stats);
        }
        raster.copy_to_staging(BufferSel::Composite);
        return;
    }
    raster.copy_composite_to_scratch();
    if cmd.weight >= BLEND_MIN_WEIGHT {
        raster.rasterize_stroke(
            &cmd.points,
            BufferSel::Scratch,
            StrokeParams { half_width_cells: hw, mode: WriteMode::AddWeighted, brush_weight: cmd.weight },
        );
        stats.strokes += 1;
    }
    if cmd.weight >= COMMIT_WEIGHT {
        raster.commit_scratch_to_composite();
        stats.committed += 1;
    }
    raster.copy_to_staging(BufferSel::Composite);
    if cmd.emit_event {
        stats.events += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::base_app;
    use bevy_app::Update;

    fn line(w: f32) -> ReplayCmd {
        ReplayCmd {
            points: vec![Vec3::new(-8.0, 0.0, 0.0), Vec3::new(8.0, 0.0, 0.0)],
            weight: w,
            emit_event: true,
            ..Default::default()
        }
    }

    #[test]
    fn replay_restores_water_and_drains_queue() {
        let mut r = WorldRaster::new();
        let (rem, st) = replay_commands(&mut r, vec![line(1.0)], 3.0);
        assert!(rem.is_empty());
        assert_eq!(st, ReplayStats { strokes: 1, clears: 0, committed: 1, events: 1 });
        assert_eq!(r.composite[256 * 512 + 256], 255);
        assert!(r.published);
    }

    #[test]
    fn low_weight_not_committed_tiny_weight_skipped() {
        let mut r = WorldRaster::new();
        let (_, st) = replay_commands(&mut r, vec![line(0.5), line(0.001)], 3.0);
        assert_eq!(st.strokes, 1);
        assert_eq!(st.committed, 0);
        assert_eq!(r.composite[256 * 512 + 256], 0);
        assert_eq!(r.scratch[256 * 512 + 256], 0, "second cmd re-copies composite over scratch");
    }

    #[test]
    fn clear_then_nested() {
        let mut r = WorldRaster::new();
        r.composite.fill(200);
        let clear = ReplayCmd { clear: true, nested: vec![line(1.0)], ..Default::default() };
        let (_, st) = replay_commands(&mut r, vec![clear], 3.0);
        assert_eq!(st.clears, 1);
        assert_eq!(r.composite[0], 0);
        assert_eq!(r.composite[256 * 512 + 256], 255);
    }

    #[test]
    fn processed_entries_are_kept_unprocessed_removed() {
        let mut r = WorldRaster::new();
        let done = ReplayCmd { processed: true, ..line(1.0) };
        let (rem, st) = replay_commands(&mut r, vec![done, line(1.0)], 3.0);
        assert_eq!(rem.len(), 1);
        assert_eq!(st.strokes, 1);
    }

    #[test]
    fn replay_matches_live_drawing_result_for_weight_one() {
        use crate::water::tool::*;
        let mut live = WorldRaster::new();
        let mut s = ActiveRasterStroke { draw_flag: true, ..Default::default() };
        let (a, b) = (Vec3::new(-8.0, 0.0, 0.0), Vec3::new(8.0, 0.0, 0.0));
        draw_raster(&mut live, &mut s, StrokePhase::Start, a, 3.0, 2.0);
        draw_raster(&mut live, &mut s, StrokePhase::Update, b, 3.0, 2.0);
        draw_raster(&mut live, &mut s, StrokePhase::Release, b, 3.0, 2.0);
        let mut rep = WorldRaster::new();
        replay_commands(&mut rep, vec![line(1.0)], 3.0);
        assert_eq!(live.composite, rep.composite);
    }

    #[test]
    fn system_replays_queue_emits_events_and_drains() {
        let mut app = base_app(0.016);
        app.init_resource::<ReplayQueue>()
            .init_resource::<ScheduledTerrainReadback>()
            .insert_resource(WaterToolConfig { radius_scale: 1.0, half_width_cells: 3.0 })
            .add_systems(Update, replay_raster);
        app.world_mut().resource_mut::<ReplayQueue>().cmds = vec![line(1.0), ReplayCmd { processed: true, ..line(1.0) }];
        app.update();
        let q = app.world().resource::<ReplayQueue>();
        assert_eq!(q.cmds.len(), 1, "the already-processed record is the one kept");
        assert_eq!(q.stats, ReplayStats { strokes: 1, clears: 0, committed: 1, events: 1 });
        assert_eq!(app.world().resource::<WorldRaster>().composite[256 * 512 + 256], 255);
        assert_eq!(app.world_mut().resource_mut::<Events<WaterDrawEvent>>().drain().count(), 1);
        app.update();
        assert_eq!(app.world().resource::<ReplayQueue>().stats.strokes, 1, "second frame replays nothing new");
    }
}

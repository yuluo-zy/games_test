//! B typed terrain history replay, integrated with the existing stroke and edit
//! command consumers. Pending history and ReplayBucket share the same schema.
use bevy_ecs::prelude::*;
use std::sync::Arc;
use tg_terrain::stroke::TerrainStroke;
use crate::{
    resources::{history::{ReplayBuffer, ReplayCommand, terrain_edit::{self, TerrainEdit, TerrainHistoryEffects}}, terrain_editing::{StrokeId, TerrainEditingData}, time::GameTime},
    systems::terrain::{draw_terrain_stroke::EvTerrainStrokeChanged, edit_terrain_stroke::{EditTerrainStrokeCmd, SetTerrainStrokePtsCmd}},
};

pub const DEFAULT_DURATION: f32 = 0.3;
pub const DURATION_MIN: f32 = 0.4;
pub const DURATION_FACTOR: f32 = 0.01;
pub const DURATION_MAX: f32 = 3.0;
pub const ANIMATION_THRESHOLD: f32 = 0.001;

#[derive(Clone, Debug)]
pub struct TerrainReplayCmd {
    pub edit: Arc<TerrainEdit>,
    pub elapsed: f32,
    pub duration: f32,
    pub undo: bool,
    /// B initializes2 and removes only after the third completed application.
    pub completion_countdown: i32,
}
impl TerrainReplayCmd {
    pub fn expanded(edit: Arc<TerrainEdit>, undo: bool, instant: bool) -> Self {
        let duration = if instant { 0.0 } else { terrain_edit::duration(&edit) };
        Self { edit, elapsed: 0.0, duration, undo, completion_countdown: 2 }
    }
    /// Compatibility constructor for callers holding a complete stroke.
    pub fn forward(stroke: Arc<TerrainStroke>, stroke_id: StrokeId) -> Self {
        Self::expanded(Arc::new(TerrainEdit::NewTerrainStroke { index: stroke_id, height_ws: stroke.height_ws, stroke_pts: stroke.points.clone() }), false, false)
    }
    pub fn backward(stroke: Arc<TerrainStroke>, stroke_id: StrokeId) -> Self {
        let mut command = Self::forward(stroke, stroke_id);
        command.undo = true;
        command
    }
    pub fn eased(&self) -> f32 {
        if self.duration >= ANIMATION_THRESHOLD {
            let ratio = self.elapsed / self.duration;
            let t = if ratio >= 0.0 { ratio } else { 0.0 };
            let t = if t <= 1.0 { t } else { 1.0 };
            let q = 1.0 - t;
            1.0 - (q * q) * q
        } else { 1.0 }
    }
    pub fn make_instant(&mut self) { self.duration = 0.0; }
}

pub fn adjacent_distance_sum(points: &[[f32; 3]]) -> f32 {
    points.windows(2).fold(0.0, |length, pair| {
        let dx = pair[0][0] - pair[1][0];
        let dy = pair[0][1] - pair[1][1];
        let dz = pair[0][2] - pair[1][2];
        length + (dz * dz + (dy * dy + dx * dx)).sqrt()
    })
}

fn advance_with_effects(command: &mut TerrainReplayCmd, editing: &mut TerrainEditingData, speed: f32, dt: f32) -> (bool, TerrainHistoryEffects) {
    if !command.undo && command.elapsed == 0.0 {
        if let TerrainEdit::NewTerrainStroke { index, height_ws, stroke_pts } = command.edit.as_ref() {
            assert!(index.0 < tg_terrain::stroke::ID_COUNTER_LIMIT, "terrain stroke ID exceeds native counter limit");
            editing.next_index = editing.next_index.max(index.0 + 1);
            editing.strokes.insert(*index, TerrainStroke { points: stroke_pts.clone(), falloff: 0.4, index: *index, height_ws: *height_ws });
        }
    }
    let progress = command.eased();
    let effects = terrain_edit::apply(&command.edit, editing, progress, command.undo);
    let complete = if progress >= 1.0 {
        command.completion_countdown -= 1;
        command.completion_countdown < 0
    } else { false };
    command.elapsed += dt * speed;
    (complete, effects)
}

pub fn replay_terrain(
    mut replay: ResMut<ReplayBuffer>, time: Res<GameTime>, mut editing: ResMut<TerrainEditingData>,
    mut edits: EventWriter<EditTerrainStrokeCmd>, mut points: EventWriter<SetTerrainStrokePtsCmd>,
    mut changed: EventWriter<EvTerrainStrokeChanged>,
) {
    let speed = replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else { return; };
    bucket.commands.retain_mut(|command| {
        let ReplayCommand::Terrain(command) = command else { return true; };
        let (complete, effects) = advance_with_effects(command, &mut editing, speed, time.delta);
        for edit in effects.edits { edits.write(EditTerrainStrokeCmd(edit)); }
        for point_set in effects.point_sets { points.write(point_set); }
        for id in effects.changed { changed.write(EvTerrainStrokeChanged(id)); }
        !complete
    });
}

pub fn terrain_replay_from_edit(edit: &TerrainEdit, _stroke_id: StrokeId) -> Option<TerrainReplayCmd> {
    Some(TerrainReplayCmd::expanded(Arc::new(edit.clone()), false, false))
}

/// Resolved CPU convenience path; the scheduled system above publishes through
/// the actual edit consumers so readback/heightmap/feedback side effects run.
pub fn advance_terrain_replay(command: &mut TerrainReplayCmd, editing: &mut TerrainEditingData, speed: f32, dt: f32) -> bool {
    let (complete, effects) = advance_with_effects(command, editing, speed, dt);
    for command in effects.edits {
        if let Some(stroke) = editing.strokes.get_mut(&command.id) {
            let outcome = tg_terrain::edit::apply_resolved(tg_terrain::edit::StrokeParameters { falloff: stroke.falloff, height: stroke.height_ws }, command);
            stroke.falloff = outcome.state.falloff;
            stroke.height_ws = outcome.state.height;
        }
    }
    for command in effects.point_sets {
        if let Some(stroke) = editing.strokes.get_mut(&command.id) { stroke.points = command.points; }
    }
    complete
}

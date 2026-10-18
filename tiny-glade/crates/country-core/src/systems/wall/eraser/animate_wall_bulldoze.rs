//! B `systems/wall/eraser/animate_wall_bulldoze.rs`: wall bulldoze animation.
//!
//! Evidence: `chapters/systems/wall/eraser/animate_wall_bulldoze.rs.md` (O,
//! 0x140ac27e0) + `systems/wall/eraser/bulldoze.rs`. Each frame phase
//! advances by dt / 0.2, then the CubicInOut ease slices the curve; at
//! phase == 1.0 the wall authority is removed. The 152 B command record is
//! represented by [`WallBulldozeCommand`].

use bevy_ecs::prelude::*;
use glam::Vec3;

use super::WallBulldozeAudio;
use crate::systems::wall::eraser::bulldoze::WallBulldozeCommand;
use crate::resources::history::wall_edit::slice_source_curve;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::{
    time::GameTime,
    walls::{InnerWalls, WallId},
};

/// Bulldoze animation duration (0x3e4ccccd @0x1429cd068).
const BULLDOZE_DURATION: f32 = 0.2;

fn ease(t: f32) -> f32 {
    if t < 0.5 {
        let x = t + t;
        x * x * x * 0.5
    } else {
        let x = t + t - 2.0;
        x * x * x * 0.5 + 1.0
    }
}

fn freehand_shape() -> WallShapeSnapshot {
    let mut bytes = [0; 28];
    bytes[..4].copy_from_slice(&2u32.to_le_bytes());
    WallShapeSnapshot::from_observed(bytes)
}

/// B `AnimateWallBulldozeBuffer`: the animation queue.
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimateWallBulldozeBuffer(pub Vec<WallBulldozeCommand>);

impl AnimateWallBulldozeBuffer {
    /// B new (0x140a82b20): empty queue.
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn contains(&self, wall: WallId) -> bool {
        self.0.iter().any(|c| c.wall.0 == wall.0)
    }
    pub fn push(&mut self, command: WallBulldozeCommand) {
        self.0.push(command);
    }
    /// B FUN_1412f6a20: drop finished rows.
    pub fn retain_finished(&mut self) {
        self.0.retain(|c| !c.finished);
    }
}

/// B 0x140ac27e0 (4-param system).
pub fn animate_wall_bulldoze(
    mut buffer: ResMut<AnimateWallBulldozeBuffer>,
    mut inner: ResMut<InnerWalls>,
    time: Res<GameTime>,
    mut audio: EventWriter<WallBulldozeAudio>,
) {
    let dt = time.delta;
    buffer.0.retain_mut(|cmd| {
        if cmd.finished {
            return false;
        }
        let wall = WallId(cmd.wall.0);
        let phase = cmd.phase + dt / BULLDOZE_DURATION;
        cmd.phase = if phase < 0.0 {
            0.0
        } else if phase > 1.0 {
            1.0
        } else {
            phase
        };
        if cmd.phase == 1.0 {
            if inner.contains(wall) {
                inner.remove(wall);
            }
            cmd.finished = true;
            return false;
        }
        let eased = ease(cmd.phase);
        let low = cmd.low * eased;
        let high = (1.0 - eased) + cmd.high * eased;
        let Some(state) = inner.get(wall).cloned() else {
            cmd.finished = true;
            return false;
        };
        let snapshot = state.snapshot().clone();
        let Ok(sliced) = slice_source_curve(&snapshot.source.curve, low, high) else {
            cmd.finished = true;
            return false;
        };
        let mut next = snapshot.clone();
        next.source.curve = sliced;
        next.source.shape = freehand_shape();
        next.source.curve_kind_tag = 2;
        next.source.style = u8::from(snapshot.source.style == 1);
        let freehand_ty = state.freehand_ty();
        inner.update_wall_state(
            wall,
            crate::resources::walls::WallState::from_snapshot(next, freehand_ty),
        );
        audio.write(WallBulldozeAudio {
            wall,
            position: Vec3::from_array(cmd.anchor),
            kind: 0,
            flag: 0,
        });
        true
    });
}

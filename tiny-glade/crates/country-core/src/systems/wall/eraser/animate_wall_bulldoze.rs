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
        // The original looks up authority before advancing phase (140ac28c0..2930).
        let Some(state) = inner.get(wall).cloned() else {
            cmd.finished = true;
            return false;
        };
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
        let low = (0.0 * (1.0 - eased)) + cmd.low * eased;
        let high = (1.0 - eased) + cmd.high * eased;
        // CurveU construction checks both bounds before slicing (140ac29fd..a3b).
        assert!((0.0..=1.0).contains(&low), "invalid bulldoze CurveU low: {low}");
        assert!((0.0..=1.0).contains(&high), "invalid bulldoze CurveU high: {high}");
        // The native slice helper receives the command's retained Source,
        // not the progressively shortened authority from the previous frame.
        let snapshot = cmd.original.clone();
        let sliced = slice_source_curve(&snapshot.source.curve, low, high)
            .expect("invalid bulldoze curve slice");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::WallState;

    fn setup(low: f32, high: f32) -> (World, Schedule, WallId) {
        let mut world = World::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(curve, 2.0, 0, 0, false));
        let original = inner.get(id).unwrap().snapshot().clone();
        world.insert_resource(inner);
        world.insert_resource(AnimateWallBulldozeBuffer(vec![WallBulldozeCommand {
            original, wall: crate::resources::walls::publication_fields::WallId(id.0),
            low, high, anchor: [0.0; 3], phase: 0.0, finished: false,
        }]));
        let mut time = GameTime::default();
        time.delta = 0.05;
        world.insert_resource(time);
        world.init_resource::<Events<WallBulldozeAudio>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(animate_wall_bulldoze);
        (world, schedule, id)
    }

    #[test]
    fn successive_frames_slice_the_retained_original_curve() {
        let (mut world, mut schedule, id) = setup(0.0, 0.0);
        schedule.run(&mut world);
        schedule.run(&mut world);
        let length = world.resource::<InnerWalls>().get(id).unwrap().snapshot().source.curve.length;
        assert!((length - 2.0).abs() < 0.00001, "length={length}");
    }

    #[test]
    #[should_panic(expected = "invalid bulldoze CurveU low")]
    fn invalid_low_panics_instead_of_dropping_the_command() {
        let (mut world, mut schedule, _) = setup(-1.0, 0.0);
        schedule.run(&mut world);
    }

    #[test]
    #[should_panic(expected = "invalid bulldoze CurveU high")]
    fn invalid_high_panics_instead_of_dropping_the_command() {
        let (mut world, mut schedule, _) = setup(0.0, 3.0);
        schedule.run(&mut world);
    }
}

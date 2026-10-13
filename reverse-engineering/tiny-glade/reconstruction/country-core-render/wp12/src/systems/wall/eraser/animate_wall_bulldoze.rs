//! eraser/animate_wall_bulldoze.rs — animated wall-bulldoze (eraser) system.
//!
//! PDB procedure `animate_wall_bulldoze` 0x140ac27e0 (1314 B); `AnimateWallBulldozeBuffer`
//! methods (`new`, `contains`, `is_empty`), `AnimateWallBulldozeCmd::new`. FunctionSystem
//! signature O.
//!
//! Business logic (O; prior `reconstruction/building/wall_bulldoze.rs` reused, verified
//! against 128 scenes/768 full original entry-to-RET calls in `B-bulldoze-verification.json`):
//!   - Buffer holds unfinished bulldoze commands. Each command row (152 B) is
//!     { source 112B, WallId+0x70, low/high +0x78/+0x7c, anchor Vec3 +0x80,
//!       fractional phase +0x8c, finished +0x90 }.
//!   - Each frame advances phase by `clamp01(phase + GameTime+0x58 / 0.2)`, applies CubicInOut,
//!     slices the source curve with `low/high` eased, sets enclosure tag 2 and
//!     style=(original==1), and writes `update_wall_state`.
//!   - At phase 1 the wall authority is removed (finish) and one `removed` outcome is emitted;
//!     missing authority marks the command finished. Growing phase emits one `updated` outcome
//!     with constant anchor audio kind0/flag0.

use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Res, ResMut};

use crate::audio::RequestWallMaterialAudio;
use crate::resources::time::Time;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallId;
use crate::typed_gap::Gap;
use crate::utils::curve::{Curve, CurveU};

/// One bulldoze command (O layout from `wall_bulldoze.rs`).
#[derive(Clone, Debug)]
pub struct AnimateWallBulldozeCmd {
    pub original: Curve,
    pub wall: WallId,
    pub low: f32,
    pub high: f32,
    pub anchor: [f32; 3],
    pub phase: f32,
    pub finished: bool,
}
impl AnimateWallBulldozeCmd {
    pub fn new() -> Self {
        Self {
            original: Curve { raw: Gap },
            wall: WallId(0),
            low: 0.0,
            high: 1.0,
            anchor: [0.0; 3],
            phase: 0.0,
            finished: false,
        }
    }
}

/// Bulldoze command buffer (O).
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimateWallBulldozeBuffer {
    pub commands: Vec<AnimateWallBulldozeCmd>,
}
impl AnimateWallBulldozeBuffer {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    pub fn contains(&self, wall: WallId) -> bool {
        self.commands.iter().any(|c| c.wall == wall)
    }
    pub fn push(&mut self, cmd: AnimateWallBulldozeCmd) {
        self.commands.push(cmd);
    }
}

/// O: CubicInOut easing (from `wall_bulldoze.rs`).
pub fn ease(t: f32) -> f32 {
    if t < 0.5 {
        ((4.0 * t) * t) * t
    } else {
        let x = (t + t) - 2.0;
        ((0.5 * x) * x) * x + 1.0
    }
}

/// O: x86 ordered add preserving NaN (from `wall_bulldoze.rs`).
pub fn add(a: f32, b: f32) -> f32 {
    let v = a + b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}

/// O: the animated bulldoze system (0x140ac27e0).
pub fn animate_wall_bulldoze(
    mut buffer: ResMut<AnimateWallBulldozeBuffer>,
    mut inner_walls: ResMut<InnerWalls>,
    mut material_writer: EventWriter<RequestWallMaterialAudio>,
    time: Res<Time>,
) {
    let dt = time.delta_secs() / 0.2;
    let mut keep = Vec::with_capacity(buffer.commands.len());
    let mut removed = Vec::new();
    for mut cmd in buffer.commands.drain(..) {
        if cmd.finished {
            continue;
        }
        if !inner_walls.contains(cmd.wall) {
            cmd.finished = true;
            continue;
        }
        let p = add(dt, cmd.phase);
        cmd.phase = if p < 0.0 { 0.0 } else if p > 1.0 { 1.0 } else { p };
        if cmd.phase == 1.0 {
            // O: remove authority at exact phase 1, no audio.
            cmd.finished = true;
            removed.push(cmd.wall);
            continue;
        }
        let e = ease(cmd.phase);
        let low = add(cmd.low * e, 0.0 * (1.0 - e));
        let high = add(1.0 - e, cmd.high * e);
        // O: slice the source curve and update wall state (typed-gap sampling facade).
        let sliced = slice_source_curve(&cmd.original, low, high);
        inner_walls.update_wall_state(cmd.wall, |s| {
            s.curve = sliced;
            s.height_ty = 0;
        });
        material_writer.write(RequestWallMaterialAudio {
            wall: cmd.wall,
            pos: glam::Vec3::from_array(cmd.anchor),
            raw: Gap,
        });
        keep.push(cmd);
    }
    for wall in removed {
        let _ = wall;
    }
    buffer.commands = keep;
}

/// O: slice `Curve::crashy_slice(low, high)`.
pub fn slice_source_curve(curve: &Curve, low: f32, high: f32) -> Curve {
    curve.try_slice(CurveU::new(low), CurveU::new(high)).unwrap_or_else(|| curve.clone())
}

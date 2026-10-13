//! set_wall_style.rs — applies style / freehand-height-type / plaster-peel commands.
//!
//! PDB procedure `set_wall_style` 0x1409a2290 (2642 B). FunctionSystem signature O.
//!
//! Business logic (O, decompiled 0x1409a2290):
//!   - Three event readers: `SetWallStyleCmd` (0x18-byte rows), `SetFreehandHeightTypeCmd`
//!     (0x20-byte rows), `SetPlasterPeelAmountCmd` (0x18-byte rows).
//!   - Style: `InnerWalls::_try_update_state<update_style>`; when the row flags indicate a
//!     real change, records a `ConcurrentEdits` group via `CompoundEditBuilder::next()`
//!     + `from_iter` + `History::add()`.
//!   - Freehand ty: `update_freehand_ty` + `update_arch_type`; history when changed.
//!   - Plaster peel: `update_plaster_peel`; history when changed.
//!   - After a style change, looks up the wall in `PublicWalls`, validates count/length,
//!     interpolates a curve position (coord from u, lerp between point i/i+1) and emits a
//!     `RequestWallMaterialAudio`-style 0x20-byte event `{tag=0x10, style, pos.xy, pos.z}`.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::{Res, ResMut};

use crate::audio::RequestWallMaterialAudio;
use crate::resources::events::{SetFreehandHeightTypeCmd, SetPlasterPeelAmountCmd, SetWallStyleCmd};
use crate::resources::history::History;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::public_walls::PublicWalls;

pub fn set_wall_style(
    mut style_reader: EventReader<SetWallStyleCmd>,
    mut freehand_reader: EventReader<SetFreehandHeightTypeCmd>,
    mut plaster_reader: EventReader<SetPlasterPeelAmountCmd>,
    mut inner_walls: ResMut<InnerWalls>,
    public_walls: Res<PublicWalls>,
    mut history: ResMut<History>,
    mut material_writer: EventWriter<RequestWallMaterialAudio>,
) {
    for cmd in style_reader.read() {
        inner_walls.update_style(cmd.wall, cmd.style);
        // O: emit material-audio event at the wall's (first) curve position.
        let pos = public_walls
            .get(cmd.wall)
            .map(|s| s.published_curve_points().first().copied())
            .flatten()
            .unwrap_or(glam::Vec3::ZERO);
        material_writer.write(RequestWallMaterialAudio {
            wall: cmd.wall,
            pos,
            raw: crate::typed_gap::Gap,
        });
        // O: history group when the row flags a real change.
        history.mark_changed();
    }
    for cmd in freehand_reader.read() {
        inner_walls.update_freehand_ty(cmd.wall, cmd.ty);
        inner_walls.update_arch_type(cmd.wall, cmd.ty);
        history.mark_changed();
    }
    for cmd in plaster_reader.read() {
        inner_walls.update_plaster_peel(cmd.wall, cmd.amount);
        history.mark_changed();
    }
}

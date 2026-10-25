//! debug_viz_threshold_curve.rs — debug visualization of the wall height threshold curve.
//!
//! PDB procedure `debug_viz_threshold_curve` 0x140a8b0c0 (1129 bytes). FunctionSystem
//! signature O. For each public wall draws debug spheres along the published curve.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::gui::DebugGui;
use crate::resources::walls::public_walls::PublicWalls;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::utils::debug_viz::DebugSphere;

pub fn debug_viz_threshold_curve(
    debug_gui: Res<DebugGui>,
    public_walls: Res<PublicWalls>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    let _ = &mut signifiers;
    if !debug_gui.enabled() {
        return;
    }
    for wall in public_walls.wall_ids() {
        let Some(state) = public_walls.get(wall) else { continue };
        let points = state.published_curve_points();
        for p in points.iter().step_by(3.max(points.len() / 16)) {
            let _ = DebugSphere::new(*p, 0.1);
        }
    }
}

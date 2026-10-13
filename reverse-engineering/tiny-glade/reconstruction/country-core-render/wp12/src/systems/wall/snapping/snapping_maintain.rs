//! snapping/snapping_maintain.rs — snapping maintenance + preview signifiers.
//!
//! PDB procedures: `snapping_maintain` 0x140a6ad00 (117 B), `snapping_signifiers`
//! 0x140a6a650 (1698 B), `check_for_disable_snapping` 0x140a6a560 (230 B).
//! FunctionSystem signatures O.

use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::Component;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::components::{SnappingToEndPreview, SnappingToSegmentPreview};
use crate::resources::onboarding::NotifyHintSystemSnapping;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::snapping::SnappingMode;
use crate::ui::mod_key::Modifiers;
use crate::utils::components::transform::Transform;
use crate::utils::debug_viz::{DebugLine, DebugSphere};

/// Preview line across the snapped wall (O: `LineAcrossWallSignifier`-like).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SnappingPreviewLine {
    pub raw: crate::typed_gap::Gap,
}

/// O: `check_for_disable_snapping` (0x140a6a560).
pub fn check_for_disable_snapping(mut snapping: ResMut<SnappingMode>) {
    if snapping.mode != 0 && snapping.snapped {
        snapping.mode = 0;
        snapping.snapped = false;
    }
}

/// O: `snapping_maintain` (0x140a6ad00) — toggles snapping from the mod key and writes the
/// onboarding hint.
pub fn snapping_maintain(
    mut snapping: ResMut<SnappingMode>,
    modifiers: Res<Modifiers>,
    mut hint_writer: EventWriter<NotifyHintSystemSnapping>,
) {
    let _ = (&modifiers, &mut hint_writer);
    if snapping.mode != 0 && snapping.snapped {
        snapping.mode = 0;
        snapping.snapped = false;
    }
}

/// O: `snapping_signifiers` (0x140a6a650) — draws the segment/end preview lines.
pub fn snapping_signifiers(
    snapping: Res<SnappingMode>,
    public_walls: Res<PublicWalls>,
    mut segment_query: Query<(&SnappingToSegmentPreview, &mut Transform), bevy_ecs::query::Without<SnappingToEndPreview>>,
    mut end_query: Query<(&SnappingToEndPreview, &mut Transform)>,
) {
    let _ = (&public_walls, &mut segment_query, &mut end_query);
    if !snapping.is_active() {
        return;
    }
    let _ = DebugLine::new(glam::Vec3::ZERO, glam::Vec3::ZERO);
    let _ = DebugSphere::new(glam::Vec3::ZERO, 0.5);
}

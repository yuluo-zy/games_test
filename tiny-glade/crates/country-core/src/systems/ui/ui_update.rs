//! Original B ui_update140b2de60: hover/captured-drag ownership and effects.
//! The picker supplies sorted real intersections; this system consumes physical
//! action slot0, preserves the captured Arc, emits effects, then clears frame
//! registrations. Native evidence: evidence/round12/ui-update and ui-picker.
use crate::{
    input::action_set::ActionSetStates,
    resources::{
        events::ShutdownGameCmd,
        gui::EscapeMenuJustOpened,
        mouse_capture::{OsCursorVisibilityCmd, OsCursorVisibilityCmdOrigin},
        window::WindowFocusEvent,
    },
    systems::{
        brush_preview::OverwriteBrushPreview,
        ui::resources::{
            ui_state::{UiInteraction, UiState, WidgetId},
            widget_params::{PlayUiSound, WidgetGeometry, WidgetParams},
        },
    },
};
use bevy_ecs::prelude::*;
use glam::Vec3;
use std::sync::Arc;

/// Original get_ui_volume140b2dc60 returns the registry Arc, not geometry.
/// Missing registration is the original Option::unwrap panic domain.
pub fn get_ui_volume(state: &UiState, id: WidgetId) -> Arc<WidgetParams> {
    state.widget(id).unwrap()
}
fn intersection_id(p: &[u8; 56]) -> WidgetId {
    WidgetId(u64::from_le_bytes(p[16..24].try_into().unwrap()))
}
fn none_intersection() -> [u8; 56] {
    let mut p = [0; 56];
    p[..4].copy_from_slice(&2u32.to_le_bytes());
    p
}

struct InteractionOwner {
    interaction: UiInteraction,
    params: Option<Arc<WidgetParams>>,
    intersection: [u8; 56],
}
impl InteractionOwner {
    fn enter_hover(
        &mut self,
        state: &UiState,
        hit: Option<[u8; 56]>,
        sounds: &mut EventWriter<PlayUiSound>,
        reuse_drag: bool,
    ) {
        let Some(hit) = hit else {
            self.interaction.native_tag = 0;
            self.params = None;
            self.intersection = none_intersection();
            return;
        };
        let id = intersection_id(&hit);
        let changed = self.interaction.native_tag == 0 || self.interaction.widget != id;
        // Ended drag over its same target retains the old Arc; ordinary hover
        // refreshes from this frame's registry even when target identity is equal.
        let params = if reuse_drag && !changed {
            self.params.as_ref().unwrap().clone()
        } else {
            get_ui_volume(state, id)
        };
        if changed {
            sounds.write(PlayUiSound::from_native_slot(params.sounds.slots[0]));
        }
        self.interaction = UiInteraction {
            native_tag: 1,
            native_phase: 1,
            widget: id,
        };
        self.params = Some(params);
        self.intersection = hit;
    }
}

/// Native brush option occupies WidgetParams+0x78..88. Tag0=None,
/// tag1=kind with current cursor fallback, tag2=offset position plus geometry
/// origin for Sphere/SphereCloud/PlaneDisc. Other geometry leaves flag false.
fn preview(owner: &InteractionOwner) -> OverwriteBrushPreview {
    if owner.interaction.native_tag == 0 {
        return OverwriteBrushPreview::default();
    }
    let params = owner.params.as_ref().unwrap();
    let mut raw = [0; 16];
    raw[..8].copy_from_slice(&params.extra_78[0].to_le_bytes());
    raw[8..].copy_from_slice(&params.extra_78[1].to_le_bytes());
    let tag = raw[0];
    if tag == 0 {
        return OverwriteBrushPreview::default();
    }
    if tag == 1 {
        return OverwriteBrushPreview {
            kind: raw[1],
            ..Default::default()
        };
    }
    assert_eq!(tag, 2, "valid native brush preview variant");
    let mut position = Vec3::new(
        f32::from_le_bytes(raw[4..8].try_into().unwrap()),
        f32::from_le_bytes(raw[8..12].try_into().unwrap()),
        f32::from_le_bytes(raw[12..16].try_into().unwrap()),
    );
    let origin = match params.resolved_geometry().as_ref() {
        WidgetGeometry::Sphere { center, .. } => Some(*center),
        WidgetGeometry::SphereCloud { centers, .. } => {
            let index = u64::from_le_bytes(owner.intersection[24..32].try_into().unwrap()) as usize;
            Some(centers[index]) // original bounds panic, not fallback position
        }
        WidgetGeometry::PlaneDisc { point, .. } => Some(*point),
        _ => None,
    };
    if let Some(origin) = origin {
        position += origin;
    }
    OverwriteBrushPreview {
        flag: u8::from(origin.is_some()),
        position,
        kind: raw[1],
        ..Default::default()
    }
}

/// Original8 SystemParams, original event read/write order, and full ownership.
/// Called after actual hover_intersection in original set6.
pub fn ui_update(
    mut state: ResMut<UiState>,
    actions: Res<ActionSetStates>,
    mut window_focus: EventReader<WindowFocusEvent>,
    mut escape: EventReader<EscapeMenuJustOpened>,
    mut shutdown: EventReader<ShutdownGameCmd>,
    mut brush: EventWriter<OverwriteBrushPreview>,
    mut cursor: EventWriter<OsCursorVisibilityCmd>,
    mut sounds: EventWriter<PlayUiSound>,
) {
    // Native visits all3 readers before processing hover. Only last focus payload
    // matters; no focus events means no focus-loss request this frame.
    let shutdown = shutdown.read().count() != 0;
    let escape = escape.read().count() != 0;
    let focus_lost = window_focus.read().last().is_some_and(|event| event.0 == 0);
    let click = actions.digital[0];
    let mut owner = InteractionOwner {
        interaction: state.interaction(),
        params: state.active_params(),
        intersection: state.widget_context(state.active_widget()).intersecting(),
    };
    let hit = state
        .hover_intersections()
        .first()
        .copied()
        .filter(|p| u32::from_le_bytes(p[..4].try_into().unwrap()) != 2);
    // Hover false is a genuine native branch: keep this owner through this frame.
    if owner.interaction.native_tag != 2
        && !(owner.interaction.native_tag == 1 && owner.interaction.native_phase == 0)
    {
        owner.enter_hover(&state, hit, &mut sounds, false);
    }
    let force_finish = shutdown || escape || focus_lost || click.just_released();
    if owner.interaction.native_tag == 2 {
        match owner.interaction.native_phase {
            3 | 4 => {
                if force_finish {
                    // Both release and forced end use hit identity: even Escape/focus-loss
                    // commits when pointer still intersects the captured widget.
                    owner.interaction.native_phase = u8::from(
                        hit.is_some_and(|hit| intersection_id(&hit) == owner.interaction.widget),
                    );
                } else {
                    // Prefer current registration while preserving captured intersection. A
                    // temporarily missing widget keeps the original strong Arc alive.
                    owner.params = Some(
                        state
                            .widget(owner.interaction.widget)
                            .unwrap_or_else(|| owner.params.as_ref().unwrap().clone()),
                    );
                    owner.interaction.native_phase = 4;
                }
            }
            2 => {}
            _ => owner.enter_hover(&state, hit, &mut sounds, true),
        }
    } else if owner.interaction.native_tag == 1 && !force_finish && click.just_pressed() {
        owner.interaction.native_tag = 2;
        owner.interaction.native_phase = 3;
    }
    brush.write(preview(&owner));
    let suppress =
        owner.interaction.native_tag != 0 && owner.intersection[49] & 8 != 0 && click.pressed();
    let origin = OsCursorVisibilityCmdOrigin(1);
    cursor.write(if suppress {
        OsCursorVisibilityCmd::Suppress(origin)
    } else {
        OsCursorVisibilityCmd::Unsuppress(origin)
    });
    if owner.interaction.native_tag >= 2 {
        let slot = match owner.interaction.native_phase {
            3 => Some(1),
            0 => Some(2),
            1 => Some(3),
            _ => None,
        };
        if let Some(slot) = slot {
            sounds.write(PlayUiSound::from_native_slot(
                owner.params.as_ref().unwrap().sounds.slots[slot],
            ));
        }
    }
    state.replace_interaction_full(owner.interaction, owner.params, owner.intersection);
    // Native HashMap.clear executes after sounds on every normal return.
    state.clear_widgets();
}

#[cfg(test)]
#[path = "ui_update_chain_tests.rs"]
mod chain_tests;

#[cfg(test)]
#[path = "ui_update_native_tests.rs"]
mod native_tests;

#[cfg(test)]
#[path = "ui_update_owner_lifecycle_tests.rs"]
mod owner_lifecycle_tests;

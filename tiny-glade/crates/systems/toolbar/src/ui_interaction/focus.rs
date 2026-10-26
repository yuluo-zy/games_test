//! Original focus propagation/visibility and focus event lifecycle.
//! B: 14233dcb0/2724B, 14233ce00/2164B, 14233a170/1652B,
//! 14233d680/1569B. See phase2-toolbar-focus for native evidence.
use bevy_ecs::prelude::*;
use country_core::{components::UiInteractionState,
    resources::toolbar::ToolbarUiIsInFocus,
    startup::init_photomode_state::PhotomodeState};
use std::collections::HashSet;
use crate::resouces_and_components::{UiShowIfParentIsInFocus, ShowOnlyInFpvAndFree, ToolbarFocusTracking};
use super::UiJustBecameInFocusEvent;

/// Preserve the native two-phase breadth-first ascent. All initially focused
/// descendants enter the visited set; each frontier first updates roots, then
/// propagates through nested parents. Stale/missing ancestors are skipped.
pub fn ui_focus_parent_if_self_in_focus(
    mut nested: Query<(Entity, &UiShowIfParentIsInFocus, ComponentWrite<UiInteractionState>)>,
    mut roots: Query<ComponentWrite<UiInteractionState>, Without<UiShowIfParentIsInFocus>>,
) {
    let mut visited = HashSet::new();
    let mut frontier = Vec::new();
    for (entity, parent, state) in &nested {
        if state.keep != 0 {
            visited.insert(entity);
            frontier.push(Entity::from_bits(parent.parent));
        }
    }
    while !frontier.is_empty() {
        for &entity in &frontier {
            if let Ok(mut state) = roots.get_mut(entity) { state.keep = 1; }
        }
        let mut next = Vec::new();
        for entity in frontier {
            if let Ok((_, parent, mut state)) = nested.get_mut(entity) {
                state.keep = 1;
                if visited.insert(entity) {
                    next.push(Entity::from_bits(parent.parent));
                }
            }
        }
        frontier = next;
    }
}

/// Parent keep is copied verbatim unless the item carries the camera-mode
/// restriction, which converts it to the native bool for Free/Walk only.
pub fn inherited_visibility(parent_keep: u8, fpv_only: bool, camera_mode: u8) -> u8 {
    if fpv_only {
        u8::from(parent_keep != 0 && camera_mode.wrapping_sub(1) < 2)
    } else {
        parent_keep
    }
}

pub fn ui_show_if_parent_in_focus(
    children: Query<(Entity, &UiShowIfParentIsInFocus, Option<&ShowOnlyInFpvAndFree>)>,
    mut states: Query<ComponentWrite<UiInteractionState>>,
    photo: Res<PhotomodeState>,
) {
    for (entity, parent, fpv_only) in &children {
        let parent_keep = states.get(Entity::from_bits(parent.parent))
            .expect("native parent UiInteractionState unwrap").keep;
        let mut state = states.get_mut(entity).expect("native child UiInteractionState unwrap");
        state.state = parent_keep;
        if fpv_only.is_some() {
            state.state = inherited_visibility(parent_keep, true, photo.camera_mode as u8);
        }
    }
}

/// The native membership set persists within the session: queried keep=0
/// removes membership; disappearance alone retains old entity generations.
/// Explicit host unload releases the session owner without changing this loop.
/// All nonzero keep values enter the set, but only the new exact value1 emits.
pub fn ui_send_focus_begin_events(
    states: Query<(Entity, &UiInteractionState)>,
    mut events: EventWriter<UiJustBecameInFocusEvent>,
    mut tracking: ResMut<ToolbarFocusTracking>,
) {
    for (entity, state) in &states {
        let changed = if state.keep != 0 { tracking.focused.insert(entity) } else { tracking.focused.remove(&entity) };
        if changed && state.keep == 1 { events.write(UiJustBecameInFocusEvent { entity }); }
    }
}

/// A single zero-sized event per update with any toolbar item in focus.
pub fn ui_notify_hints_if_toolbar_in_focus(
    states: Query<&UiInteractionState>,
    mut events: EventWriter<ToolbarUiIsInFocus>,
) {
    if states.iter().any(|state| state.keep != 0) { events.write(ToolbarUiIsInFocus); }
}

#[cfg(test)]
#[path = "focus_tests.rs"]
mod tests;

use country_core::render::extract::ComponentWrite;

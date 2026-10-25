//! B update_mod_key_state @140af9c00, complete506B. The two original
//! ActionSetStates reads project digital15/snapping_disable and
//! digital39/toggle_snapping; no ModKeyConfig resource exists in that contract.
use crate::{
    input::action_set::{ActionSetStates, DigitalActionState},
    resources::walls::decorator::DecoratorOpEvent,
    systems::mode_manager::AppMode,
};
use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub flags: u32,
}
impl Modifiers {
    pub fn disable_snapping(self) -> bool {
        self.flags as u8 != 0
    }
    pub fn is_disable_snapping_just_pressed(self) -> bool {
        (self.flags >> 8) as u8 != 0
    }
}

/// Original Local owner: previous AppMode16B plus toggled byte@+10.
/// Native zero/default state corresponds to Editing(None), toggled=false.
#[derive(Clone, Copy, Debug, Default)]
pub struct ToggleSnapping {
    pub previous_mode: AppMode,
    pub toggled: bool,
}

/// Typed equal AppModes compare the same active payloads as original tables
/// (wall shape, cobblestone index, eraser layer, decorator kind, editing id).
fn update_state(
    disable: DigitalActionState,
    toggle: DigitalActionState,
    mode: AppMode,
    local: &mut ToggleSnapping,
    modifiers: &mut Modifiers,
    mut consume_decorator_events: impl FnMut() -> bool,
) {
    let disabled = disable.pressed();
    let pressed = disable.just_pressed();
    modifiers.flags = (modifiers.flags & !0xffff) | u32::from(disabled) | (u32::from(pressed) << 8);
    // Original short circuit: when the mode changes the reader is not advanced
    // this frame. Otherwise it clears all unread events and tests nonempty.
    if local.previous_mode != mode || consume_decorator_events() {
        local.toggled = false;
    }
    local.previous_mode = mode;
    if toggle.just_pressed() {
        local.toggled = !local.toggled;
    }
    if local.toggled {
        modifiers.flags = (modifiers.flags & !0xff) | 1;
    }
}

pub fn update_mod_key_state(
    actions: Res<ActionSetStates>,
    mut modifiers: ResMut<Modifiers>,
    // The duplicate original Res has the same identity; the current public
    // array API shares its read-only borrow instead of copying either owner.
    toggle_actions: Res<ActionSetStates>,
    mut local: Local<ToggleSnapping>,
    mut decorator_events: EventReader<DecoratorOpEvent>,
    mode: Res<AppMode>,
) {
    update_state(
        actions.digital[15],
        toggle_actions.digital[39],
        *mode,
        &mut local,
        &mut modifiers,
        || decorator_events.read().last().is_some(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::action_set::DigitalActionPhase as P;
    fn action(phase: P, timer_seconds: f32) -> DigitalActionState {
        DigitalActionState {
            phase,
            timer_seconds,
        }
    }
    #[test]
    fn native_toggle_state_is_reset_by_mode_or_decorator_and_preserves_unread_on_mode_change() {
        let idle = action(P::Idle, 0.0);
        let pressed = action(P::JustPressed, 0.0);
        let mut local = ToggleSnapping::default();
        let mut modifiers = Modifiers { flags: 0xabcd_0000 };
        update_state(
            idle,
            pressed,
            AppMode::default(),
            &mut local,
            &mut modifiers,
            || false,
        );
        assert!(modifiers.disable_snapping());
        assert!(!modifiers.is_disable_snapping_just_pressed());
        let mut calls = 0;
        update_state(
            idle,
            idle,
            AppMode::Path,
            &mut local,
            &mut modifiers,
            || {
                calls += 1;
                true
            },
        );
        assert_eq!(calls, 0);
        assert!(!local.toggled);
        update_state(
            idle,
            pressed,
            AppMode::Path,
            &mut local,
            &mut modifiers,
            || {
                calls += 1;
                true
            },
        );
        assert_eq!(calls, 1);
        assert!(local.toggled);
        assert_eq!(modifiers.flags & 0xffff_0000, 0xabcd_0000);
    }
    #[test]
    fn native_disable_and_toggle_have_independent_edge_bytes() {
        let mut local = ToggleSnapping::default();
        let mut out = Modifiers::default();
        update_state(
            action(P::Repeating, -0.01),
            action(P::Idle, 0.0),
            AppMode::default(),
            &mut local,
            &mut out,
            || false,
        );
        assert!(out.disable_snapping());
        assert!(out.is_disable_snapping_just_pressed());
        update_state(
            action(P::Repeating, f32::NAN),
            action(P::Idle, 0.0),
            AppMode::default(),
            &mut local,
            &mut out,
            || false,
        );
        assert!(out.disable_snapping());
        assert!(!out.is_disable_snapping_just_pressed());
    }
}

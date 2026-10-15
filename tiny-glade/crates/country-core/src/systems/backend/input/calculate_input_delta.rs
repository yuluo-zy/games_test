//! B 0x1416feeb0: bool input edges and held/repeating digital actions.
//! The old chapter called the 0.1/0.05 branch an analog deadzone; native code
//! applies it to DigitalActionState tag 2. Analog values are copied unchanged.
use crate::input::action_set::{ActionSetStates, ActionSetValues, DigitalActionState};
use bevy_ecs::prelude::{Res, ResMut};

/// The payload for tags 0 and 4 is inactive in the original enum. Native code
/// leaves a scratch register there; this typed projection normalizes it to 0.
pub fn digital_delta(
    previous: bool,
    current: bool,
    state: DigitalActionState,
    dt: f32,
) -> DigitalActionState {
    if !previous {
        return DigitalActionState {
            disc: if current { 0 } else { 4 },
            val: 0.,
        };
    }
    if !current {
        return DigitalActionState {
            disc: 3,
            val: if state.disc == 1 { state.val } else { 0. },
        };
    }
    if state.disc == 2 {
        let mut timer = state.val;
        // UCOMISS/JB skips replenishment for unordered values as well as >0.
        if timer <= 0. {
            timer = (timer + 0.1).max(0.05);
        }
        return DigitalActionState {
            disc: 2,
            val: timer - dt,
        };
    }
    DigitalActionState {
        disc: 1,
        val: if state.disc == 1 { state.val + dt } else { 0. },
    }
}

pub fn update_action_states(values: &ActionSetValues, states: &mut ActionSetStates, dt: f32) {
    states.analog = values.current.analog;
    for (slot, state) in states.digital.iter_mut().enumerate() {
        *state = digital_delta(
            values.previous.digital[slot],
            values.current.digital[slot],
            *state,
            dt,
        );
    }
}

/// PDB signature uses bevy_time::Time<()> (native delta_secs at +0x58), not
/// paused GameTime. Input repeat continues while the game simulation is paused.
pub fn calculate_input_delta(
    values: Res<ActionSetValues>,
    time: Res<bevy_time::Time>,
    mut states: ResMut<ActionSetStates>,
) {
    update_action_states(&values, &mut states, time.delta_secs());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edges_hold_release_and_repeat_preserve_original_rules() {
        let down = digital_delta(false, true, DigitalActionState::default(), 0.2);
        assert!(down.just_pressed());
        let held = digital_delta(true, true, down, 0.2);
        assert_eq!(held, DigitalActionState { disc: 1, val: 0. });
        let held = digital_delta(true, true, held, 0.2);
        assert_eq!(
            digital_delta(true, false, held, 0.2),
            DigitalActionState { disc: 3, val: 0.2 }
        );
        let repeat = digital_delta(true, true, DigitalActionState { disc: 2, val: -1. }, 0.02);
        assert_eq!(repeat.val, 0.05 - 0.02);
        assert!(!repeat.just_pressed());
        let nan = digital_delta(
            true,
            true,
            DigitalActionState {
                disc: 2,
                val: f32::NAN,
            },
            0.02,
        );
        assert!(nan.val.is_nan());
    }
    #[test]
    fn all_slots_feed_the_existing_runtime_owner() {
        let mut values = ActionSetValues::default();
        values.current.digital.fill(true);
        values.current.analog.fill(glam::Vec2::new(2., -3.));
        let mut states = ActionSetStates::default();
        update_action_states(&values, &mut states, 0.1);
        assert!(states.digital.iter().all(|s| s.just_pressed()));
        assert_eq!(states.analog, values.current.analog);
    }
}

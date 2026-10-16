//! B 0x1416feeb0: bool input edges and held/repeating digital actions.
//! The old chapter called the 0.1/0.05 branch an analog deadzone; native code
//! applies it to DigitalActionState tag 2. Analog values are copied unchanged.
use crate::input::action_set::{ActionSetStates, ActionSetValues, DigitalActionState};
use bevy_ecs::prelude::{Event, Res, ResMut};

/// `UiHistoryButtonPressed`（B `calculate_input_delta::UiHistoryButtonPressed`，O：PDB
/// `translate_input` 签名 `EventReader<UiHistoryButtonPressed>`）。UI 历史按钮（撤销/重做
/// 等）按下事件，由 `translate_input` 消费。载荷语义待取证。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct UiHistoryButtonPressed;

/// The payload for tags 0 and 4 is inactive in the original enum. Native code
/// leaves a scratch register there; this typed projection normalizes it to 0.
///
/// `auto_repeat`（Shape B）仅用于槽 20..25（ui_left/right/down/up/tab_prev/tab_next），
/// held（disc==1）时 `timer = val + dt`；`timer < 0.2` 保持 disc 1 累加，`>= 0.2` 转
/// disc 2、val = 0.1（连发 0.1 s 间隔；O：fresh 0x1416feeb0 槽 20 段
/// `fVar7 < 0.2; if(!bVar5) fVar7=0.1; iVar6 = 2 - bVar5`）。
pub fn digital_delta(
    previous: bool,
    current: bool,
    state: DigitalActionState,
    dt: f32,
    auto_repeat: bool,
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
    if auto_repeat && state.disc == 1 {
        // Shape B（槽 20..25）：长按 0.2 s 后转 repeat，间隔 0.1 s。
        let timer = state.val + dt;
        if timer < 0.2 {
            return DigitalActionState {
                disc: 1,
                val: timer,
            };
        }
        return DigitalActionState {
            disc: 2,
            val: 0.1,
        };
    }
    DigitalActionState {
        disc: 1,
        val: if state.disc == 1 { state.val + dt } else { 0. },
    }
}

/// 走 Shape B（held 自动连发）的槽区间：20..=25（O：fresh 0x1416feeb0）。
pub fn uses_ui_auto_repeat(slot: usize) -> bool {
    (20..=25).contains(&slot)
}

pub fn update_action_states(values: &ActionSetValues, states: &mut ActionSetStates, dt: f32) {
    states.analog = values.current.analog;
    for (slot, state) in states.digital.iter_mut().enumerate() {
        *state = digital_delta(
            values.previous.digital[slot],
            values.current.digital[slot],
            *state,
            dt,
            uses_ui_auto_repeat(slot),
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
    fn delta(prev: bool, cur: bool, state: DigitalActionState, dt: f32) -> DigitalActionState {
        digital_delta(prev, cur, state, dt, false)
    }
    #[test]
    fn edges_hold_release_and_repeat_preserve_original_rules() {
        let down = delta(false, true, DigitalActionState::default(), 0.2);
        assert!(down.just_pressed());
        let held = delta(true, true, down, 0.2);
        assert_eq!(held, DigitalActionState { disc: 1, val: 0. });
        let held = delta(true, true, held, 0.2);
        assert_eq!(
            delta(true, false, held, 0.2),
            DigitalActionState { disc: 3, val: 0.2 }
        );
        let repeat = delta(true, true, DigitalActionState { disc: 2, val: -1. }, 0.02);
        assert_eq!(repeat.val, 0.05 - 0.02);
        assert!(!repeat.just_pressed());
        let nan = delta(
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
    fn ui_slots_auto_repeat_after_0_2_seconds_at_0_1_intervals() {
        // Shape B（槽 20..25）：按下沿 → disc 1 val 0；held 0.2 s 内 disc 1 累加；
        // ≥ 0.2 s 转 disc 2、val 0.1（连发间隔）。
        let repeat = |prev, cur, state, dt| digital_delta(prev, cur, state, dt, true);
        let mut state = repeat(false, true, DigitalActionState::default(), 0.1);
        assert!(state.just_pressed()); // 按下沿 disc 0
        state = repeat(true, true, state, 0.1); // held 1：disc 1 val 0
        assert_eq!(state, DigitalActionState { disc: 1, val: 0. });
        state = repeat(true, true, state, 0.1); // held 2：0.1 < 0.2
        assert_eq!(state, DigitalActionState { disc: 1, val: 0.1 });
        state = repeat(true, true, state, 0.1); // held 3：0.2 → repeat
        assert_eq!(state, DigitalActionState { disc: 2, val: 0.1 });
        let next = repeat(true, true, state, 0.05);
        assert_eq!(next, DigitalActionState { disc: 2, val: 0.05 });
        // Shape A 同槽位不会转 repeat：held 保持 disc 1。
        let held_a = delta(true, true, DigitalActionState { disc: 1, val: 0.5 }, 0.1);
        assert_eq!(held_a, DigitalActionState { disc: 1, val: 0.6 });
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
    #[test]
    fn only_the_ui_nav_slots_use_auto_repeat_shape() {
        assert!(!uses_ui_auto_repeat(19));
        assert!(uses_ui_auto_repeat(20));
        assert!(uses_ui_auto_repeat(25));
        assert!(!uses_ui_auto_repeat(26));
    }
}

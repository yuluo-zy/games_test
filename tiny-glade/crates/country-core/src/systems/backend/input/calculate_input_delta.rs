//! 将设备布尔值推进为按下、持续、连发和释放状态。
use crate::input::action_set::{
    ActionSetStates, ActionSetValues, DigitalActionPhase as Phase, DigitalActionState,
};
use bevy_ecs::prelude::{Event, Res, ResMut};

const REPEAT_DELAY_SECONDS: f32 = 0.2;
const REPEAT_INTERVAL_SECONDS: f32 = 0.1;
const MIN_REPEAT_REMAINDER_SECONDS: f32 = 0.05;

/// UI 历史按钮事件：零为撤销，其余为重做。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiHistoryButtonPressed(pub u8);
impl UiHistoryButtonPressed {
    pub fn is_redo(&self) -> bool {
        self.0 != 0
    }
}

pub fn digital_delta(
    previous: bool,
    current: bool,
    state: DigitalActionState,
    delta_seconds: f32,
    auto_repeat: bool,
) -> DigitalActionState {
    // 原1416feeb0逐槽内联：仅Held计时有效，初次Held从0开始；
    // 进入Repeating时先保留0.1秒，过期后保留超时余量并以0.05为下限。
    // 原算法不对时间做截断或有限值替换，不能用防御性默认值改变状态推进。
    let timer_seconds = state.timer_seconds;
    let (phase, timer_seconds) = match (previous, current) {
        (false, false) => (Phase::Idle, 0.0),
        (false, true) => (Phase::JustPressed, 0.0),
        (true, false) => (
            Phase::JustReleased,
            if state.phase == Phase::Held { timer_seconds } else { 0.0 },
        ),
        (true, true) if state.phase == Phase::Repeating => {
            let countdown = if timer_seconds <= 0.0 {
                (timer_seconds + REPEAT_INTERVAL_SECONDS).max(MIN_REPEAT_REMAINDER_SECONDS)
            } else {
                timer_seconds
            };
            (Phase::Repeating, countdown - delta_seconds)
        }
        (true, true) => {
            let held_seconds = if state.phase == Phase::Held {
                timer_seconds + delta_seconds
            } else {
                return DigitalActionState { phase: Phase::Held, timer_seconds: 0.0 };
            };
            // 原ucomiss的无序结果同样进入连发分支，保留NaN比较语义。
            if auto_repeat && !(held_seconds < REPEAT_DELAY_SECONDS) {
                (Phase::Repeating, REPEAT_INTERVAL_SECONDS)
            } else {
                (Phase::Held, held_seconds)
            }
        }
    };
    DigitalActionState {
        phase,
        timer_seconds,
    }
}

pub fn uses_ui_auto_repeat(slot: usize) -> bool {
    (20..=25).contains(&slot)
}

pub fn update_action_states(
    values: &ActionSetValues,
    states: &mut ActionSetStates,
    delta_seconds: f32,
) {
    // 原函数直接搬运六个Vec2；输入合法性由各设备／文件入口负责。
    states.analog = values.current.analog;
    for (slot, state) in states.digital.iter_mut().enumerate() {
        *state = digital_delta(
            values.previous.digital[slot],
            values.current.digital[slot],
            *state,
            delta_seconds,
            uses_ui_auto_repeat(slot),
        );
    }
}

/// 输入长按使用真实时间，游戏暂停不会阻止菜单导航。
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
    fn all_slots_match_original_input_function_fixture() {
        type Row = (usize, bool, bool, u32, u32, u32, u32, u32);
        let rows: Vec<Row> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/native/toolbar-input-delta.ron"
        ))).expect("原完整输入函数夹具");
        assert_eq!(rows.len(), 9600);
        let phase = |tag| match tag {
            0 => Phase::JustPressed, 1 => Phase::Held, 2 => Phase::Repeating,
            3 => Phase::JustReleased, 4 => Phase::Idle, _ => unreachable!(),
        };
        for (slot, previous, current, before, timer, delta, expected, expected_timer) in rows {
            let result = digital_delta(previous, current, DigitalActionState {
                phase: phase(before), timer_seconds: f32::from_bits(timer),
            }, f32::from_bits(delta), uses_ui_auto_repeat(slot));
            assert_eq!(result.phase, phase(expected), "槽{slot}，边沿{previous}/{current}，相位{before}");
            // 原previous=false分支不读取timer，写入的是寄存器非活动载荷；业务不观察它。
            if previous {
                assert_eq!(result.timer_seconds.to_bits(), expected_timer,
                    "槽{slot}，边沿{previous}/{current}，相位{before}，timer={timer:x} delta={delta:x}");
            }
        }
    }
    #[test]
    fn press_hold_repeat_and_release_keep_original_phase_boundaries() {
        let pressed = digital_delta(false, true, DigitalActionState::default(), 0.1, true);
        assert!(pressed.just_pressed());
        let held = digital_delta(true, true, pressed, 0.1, true);
        assert_eq!(held.phase, Phase::Held);
        assert_eq!(held.timer_seconds, 0.0);
        let held = digital_delta(true, true, held, 0.1, true);
        let repeated = digital_delta(true, true, held, 0.1, true);
        assert_eq!(repeated.phase, Phase::Repeating);
        assert_eq!(repeated.timer_seconds, 0.1);
        assert!(!repeated.just_pressed());
        let waiting = digital_delta(true, true, repeated, 0.02, true);
        assert!(!waiting.just_pressed());
        let released = digital_delta(true, false, waiting, 0.1, true);
        assert!(released.just_released());
        assert_eq!(
            digital_delta(false, false, released, f32::NAN, true),
            DigitalActionState::default()
        );
        assert_eq!(released.timer_seconds, 0.0);
    }
    #[test]
    fn ordinary_buttons_never_auto_repeat() {
        let mut state = digital_delta(false, true, DigitalActionState::default(), 1.0, false);
        for _ in 0..20 {
            state = digital_delta(true, true, state, 1.0, false);
            assert!(!state.just_pressed());
        }
        assert_eq!(state.phase, Phase::Held);
    }
}

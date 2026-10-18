//! B `systems/history/replay_mode.rs` — 回放模式进出检测 + HoldToFastReplay
//! （fresh 0x140a7c550 逐段；含 `time_continuous_lerp` 0x140c904e0）。
//!
//! ## fresh 控制流
//!
//! 1. 最后一个待处理 `HistoryCmd`（native 0x28B 记录）若为回放命令，映射出
//!    "恢复到的 AppMode" 并写模式事件 + `speed = 1.0` + hold 复位。**该段依赖
//!    native HistoryCmd 0x28B 载荷，crate 内 `HistoryCommand` 是 4B 模型 → 登记 U。**
//! 2. `AppMode != Replay`：若进入标志置位 → log + 清标志；return。
//! 3. `AppMode == Replay`：首次 log + 置标志；**每帧**调用
//!    `clearing_canvas_w_layer_and_sound`（`ClearCanvasCmd(true)`）。
//! 4. 输入覆盖（input byte0 bit0 置位）→ `speed = input f32@+4`；return。
//! 5. 未 fast：`accrued = (input[+0x80] < 3 || input[+0x88] < 3) ? hold + dt : 0`；
//!    `fast = 0.2 < accrued`（UCOMISS+seta，NaN 视为 false）；`hold = fast ? 1.0 : accrued`。
//!    fast → `speed = 1.0`；否则 `speed = NaN-aware max(speed,
//!    0.5*(pending_len + 1 - (replay_empty?1:0))²)`。
//! 6. 已 fast：
//!    - `input[+0x80] < 3`：写焦点事件 0x03000000；`speed = hold =
//!      lerp(hold, 6.0, 1-exp(-speed*0.2))`（`time_continuous_lerp`）。
//!    - 否则 `input[+0x88] >= 3`：写事件 0x03000200；`speed = 1.0`；hold 清零；
//!      `speed = NaN-aware max(1.0, 0.5*(pending_len+1-(replay_empty?1:0))²)`。
//!    - 否则：写事件 0x03000100；同 6a 的 lerp。
//!
//! `time_continuous_lerp(a, b, rate, dt) = a + (b-a)*(1-exp(-rate*dt))`
//! （fresh 0x140c904e0 反汇编：`xorps` 取负 rate、`mulss dt`、`expf`、
//! `1-exp`、`lerp`）。调用点常数 6.0（0x142934690）与 0.2（0x1429cd068）。
use crate::{resources::time::GameTime, systems::mode_manager::{AppMode, ClearCanvasCmd}};
use crate::resources::history::ReplayBuffer;
use crate::systems::history::PendingHistoryCommands;
use bevy_ecs::prelude::*;

/// AppMode 原生 tag == 0xd（Replay）。
pub fn is_replay_mode(mode: AppMode) -> bool {
    mode.native_tag() == 13
}

/// 进出回放模式（O：进入/退出判定；返回 `Some(true)` 进入、`Some(false)` 退出、
/// `None` 未变化）。
pub fn replay_mode_transition(previous: AppMode, current: AppMode) -> Option<bool> {
    let was = is_replay_mode(previous);
    let is = is_replay_mode(current);
    if was == is {
        None
    } else {
        Some(is)
    }
}

/// HoldToFastReplay 长按快进阈值（0x140a7c550：`fVar24 <= 0.2` 保持、
/// `0.2 < fVar24` 触发；秒制）。
pub const FAST_REPLAY_HOLD_SECONDS: f32 = 0.2;
/// fast 斜坡目标速度（0x142934690）。
pub const FAST_REPLAY_TARGET_SPEED: f32 = 6.0;
/// lerp 速率乘数（0x1429cd068）。
pub const FAST_REPLAY_LERP_DT: f32 = 0.2;

/// 焦点事件 kind（原生 16B 事件 +8 槽：0x03000000 / 0x03000100 / 0x03000200）。
pub const REPLAY_FOCUS_PRIMARY_READY: u32 = 0x0300_0000;
pub const REPLAY_FOCUS_AUX_READY: u32 = 0x0300_0100;
pub const REPLAY_FOCUS_BOTH_READY: u32 = 0x0300_0200;

/// 回放焦点事件（原生 `WindowFocusEvent` 写面；history 侧模型）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplayFocusEvent {
    pub kind: u32,
}

/// `time_continuous_lerp`（0x140c904e0）：
/// `a + (b - a) * (1 - exp(-rate * dt))`。
pub fn time_continuous_lerp(a: f32, b: f32, rate: f32, dt: f32) -> f32 {
    let decay = (-rate * dt).exp();
    a + (b - a) * (1.0 - decay)
}

/// NaN-aware `max`（0x140a7cb38..：`cmpunordss` + `maxss` + `andps/andnps`）。
pub fn nan_aware_max(a: f32, b: f32) -> f32 {
    if b.is_nan() {
        a
    } else {
        a.max(b)
    }
}

/// 快进按住输入（fresh 五个面：byte0 bit0 覆盖速度 + f32@+4，
/// `u32[+0x80]` / `u32[+0x88]` 两路 input-ready 计数）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ReplayFastHoldInput {
    /// input byte0 bit0：覆盖 `ReplayBuffer.speed = speed_override`。
    pub override_speed: bool,
    pub speed_override: f32,
    /// input `+0x80`。
    pub primary: u32,
    /// input `+0x88`。
    pub secondary: u32,
}
impl ReplayFastHoldInput {
    /// `input[+0x80] < 3 || input[+0x88] < 3`。
    pub fn engaged(self) -> bool {
        self.primary < 3 || self.secondary < 3
    }
}

/// HoldToFastReplay 状态（原生 8B：`{u32 fast @0, f32 hold @4}`）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HoldToFastReplay {
    pub fast: bool,
    pub hold_time: f32,
}
impl HoldToFastReplay {
    pub const FAST_REPLAY_THRESHOLD: f32 = FAST_REPLAY_HOLD_SECONDS;

    /// 未 fast 分支的推进（0x140a7c9cf..）：返回新的 `(fast, accrued/hold)`。
    pub fn advance(&mut self, engaged: bool, dt: f32) -> bool {
        let accrued = if engaged { self.hold_time + dt } else { 0.0 };
        self.fast = FAST_REPLAY_HOLD_SECONDS < accrued;
        self.hold_time = if self.fast { 1.0 } else { accrued };
        self.fast
    }

    pub fn reset(&mut self) {
        self.fast = false;
        self.hold_time = 0.0;
    }
}

/// 0x140a7cafc：`pending_len + 1 - (ReplayBuffer 为空 ? 1 : 0)`。
fn speed_sequence_length(replay: &ReplayBuffer, pending: &PendingHistoryCommands) -> f32 {
    pending.0.len() as f32 + 1.0 - if replay.is_empty() { 1.0 } else { 0.0 }
}

/// 一个世界内模拟 `clearing_canvas_w_layer_and_sound` 的 history 事件面。
pub fn replay_mode(
    mode: Res<AppMode>,
    time: Res<GameTime>,
    input: Res<ReplayFastHoldInput>,
    pending: Res<PendingHistoryCommands>,
    mut was_replay: Local<bool>,
    mut hold: Local<HoldToFastReplay>,
    mut replay_buffer: ResMut<ReplayBuffer>,
    mut clear: EventWriter<ClearCanvasCmd>,
    mut focus: EventWriter<ReplayFocusEvent>,
) {
    let is_replay = mode.native_tag() == 13;
    if !is_replay {
        if *was_replay {
            log::info!("exiting replay mode");
            *was_replay = false;
        }
        return;
    }
    if !*was_replay {
        log::info!("entering replay mode");
        *was_replay = true;
    }
    // 原版在 AppMode==Replay 的每帧都调用 clearing_canvas_w_layer_and_sound。
    clear.write(ClearCanvasCmd(true));

    if input.override_speed {
        replay_buffer.speed = input.speed_override;
        return;
    }
    let dt = time.delta;
    if !hold.fast {
        hold.advance(input.engaged(), dt);
        if hold.fast {
            replay_buffer.speed = 1.0;
        } else {
            let len = speed_sequence_length(&replay_buffer, &pending);
            replay_buffer.speed = nan_aware_max(replay_buffer.speed, 0.5 * (len * len));
        }
        return;
    }
    if input.primary < 3 {
        focus.write(ReplayFocusEvent {
            kind: REPLAY_FOCUS_PRIMARY_READY,
        });
        let value = time_continuous_lerp(
            hold.hold_time,
            FAST_REPLAY_TARGET_SPEED,
            replay_buffer.speed,
            FAST_REPLAY_LERP_DT,
        );
        hold.hold_time = value;
        replay_buffer.speed = value;
    } else if input.secondary >= 3 {
        focus.write(ReplayFocusEvent {
            kind: REPLAY_FOCUS_BOTH_READY,
        });
        replay_buffer.speed = 1.0;
        hold.reset();
        let len = speed_sequence_length(&replay_buffer, &pending);
        replay_buffer.speed = nan_aware_max(replay_buffer.speed, 0.5 * (len * len));
    } else {
        focus.write(ReplayFocusEvent {
            kind: REPLAY_FOCUS_AUX_READY,
        });
        let value = time_continuous_lerp(
            hold.hold_time,
            FAST_REPLAY_TARGET_SPEED,
            replay_buffer.speed,
            FAST_REPLAY_LERP_DT,
        );
        hold.hold_time = value;
        replay_buffer.speed = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::mode_manager::EditSubject;

    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(AppMode::Editing(EditSubject::None));
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        world.insert_resource(ReplayFastHoldInput::default());
        world.insert_resource(ReplayBuffer::new(1.0));
        world.init_resource::<PendingHistoryCommands>();
        world.init_resource::<Events<ClearCanvasCmd>>();
        world.init_resource::<Events<ReplayFocusEvent>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(replay_mode);
        (world, schedule)
    }

    #[test]
    fn replay_mode_tag_is_thirteen() {
        assert!(is_replay_mode(AppMode::Replay));
        assert!(!is_replay_mode(AppMode::LoadGame));
        assert_eq!(AppMode::Replay.native_tag(), 13);
    }

    #[test]
    fn transitions_detect_entry_and_exit() {
        assert_eq!(
            replay_mode_transition(AppMode::LoadGame, AppMode::Replay),
            Some(true)
        );
        assert_eq!(
            replay_mode_transition(AppMode::Replay, AppMode::LoadGame),
            Some(false)
        );
        assert_eq!(replay_mode_transition(AppMode::LoadGame, AppMode::Water), None);
    }

    #[test]
    fn hold_to_fast_threshold_is_zero_point_two_seconds() {
        let mut hold = HoldToFastReplay::default();
        assert!(!hold.advance(true, 0.1));
        assert!((hold.hold_time - 0.1).abs() < 1e-6);
        // 到 0.2 边界仍未触发（`0.2 < accrued` 严格大于）。
        assert!(!hold.advance(true, 0.1));
        assert!((hold.hold_time - 0.2).abs() < 1e-6);
        // 超过 0.2 → 触发，hold 钳到 1.0（原生 fast 分支写 1.0）。
        assert!(hold.advance(true, 0.01));
        assert_eq!(hold.hold_time, 1.0);
    }

    #[test]
    fn release_resets_accrual() {
        let mut hold = HoldToFastReplay::default();
        assert!(!hold.advance(true, 0.15));
        assert!(!hold.advance(false, 0.1));
        assert_eq!(hold.hold_time, 0.0);
        assert!(!hold.fast);
    }

    #[test]
    fn time_continuous_lerp_matches_formula() {
        let value = time_continuous_lerp(1.0, 6.0, 2.0, 0.2);
        let expected = 1.0 + (6.0 - 1.0) * (1.0 - (-2.0f32 * 0.2).exp());
        assert!((value - expected).abs() < 1e-6);
        // rate=0 → 返回 a。
        assert_eq!(time_continuous_lerp(3.0, 6.0, 0.0, 0.2), 3.0);
    }

    #[test]
    fn nan_aware_max_ignores_nan_current_speed() {
        assert_eq!(nan_aware_max(0.5, f32::NAN), 0.5);
        assert_eq!(nan_aware_max(0.5, 2.0), 2.0);
    }

    #[test]
    fn system_clears_canvas_every_replay_frame_and_keeps_override_speed() {
        let (mut world, mut schedule) = fixture();
        *world.resource_mut::<AppMode>() = AppMode::Replay;
        schedule.run(&mut world);
        schedule.run(&mut world);
        // 每帧一次 clear（不是仅进入帧）。
        assert_eq!(world.resource::<Events<ClearCanvasCmd>>().len(), 2);
        // 覆盖速度。
        {
            let mut input = world.resource_mut::<ReplayFastHoldInput>();
            input.override_speed = true;
            input.speed_override = 4.0;
        }
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().speed, 4.0);
    }

    #[test]
    fn fast_hold_raises_speed_from_one_on_later_frames() {
        let (mut world, mut schedule) = fixture();
        *world.resource_mut::<AppMode>() = AppMode::Replay;
        world.resource_mut::<ReplayFastHoldInput>().primary = 0;
        // dt=0.1：两帧 0.2 未过阈；第三帧过阈 → speed=1.0。
        schedule.run(&mut world);
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().speed, 1.0);
        assert!(world.resource::<Events<ReplayFocusEvent>>().is_empty());
    }
}

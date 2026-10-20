//! B replay mode entry, restoration target and hold-to-repeat commands.
//! 0x140a7c550 reads ModeJustChanged (current/previous AppMode), writes the
//! actual HistoryCmd transport and interpolates using GameTime.delta.
use crate::resources::history::ReplayBuffer;
use crate::systems::history::PendingHistoryCommands;
use crate::{
    input::action_set::MainActionSet,
    resources::time::GameTime,
    systems::mode_manager::{
        AppMode, AppModeChanged, EditSubject, HistoryCommand, RestoreAppTo,
    },
    systems::ui::resources::ui_state::UiState,
};
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
    if was == is { None } else { Some(is) }
}

/// HoldToFastReplay 长按快进阈值（0x140a7c550：`fVar24 <= 0.2` 保持、
/// `0.2 < fVar24` 触发；秒制）。
/// 0x140a7c7f6..7ff：R9B = 0x0d（Replay 的 AppMode 原生 tag），栈上 layer 字节 = 3。
const REPLAY_CANVAS_ARGUMENT: u8 = 0x0d;
const REPLAY_CANVAS_LAYER: u8 = 3;

pub const FAST_REPLAY_HOLD_SECONDS: f32 = 0.2;
/// fast 斜坡目标速度（0x142934690）。
pub const FAST_REPLAY_TARGET_SPEED: f32 = 6.0;
/// lerp 速率乘数（0x1429cd068）。
pub const FAST_REPLAY_LERP_DT: f32 = 0.2;

/// HistoryCmd words written by the native hold-repeat writer (four-byte payload).
pub const REPLAY_FOCUS_PRIMARY_READY: u32 = 0x0300_0000;
pub const REPLAY_FOCUS_AUX_READY: u32 = 0x0300_0100;
pub const REPLAY_FOCUS_BOTH_READY: u32 = 0x0300_0200;

/// Legacy host projection, retained for source compatibility. The replay
/// system writes HistoryCommand directly; this is not an original event type.
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
    if b.is_nan() { a } else { a.max(b) }
}

/// HoldToFastReplay 状态（原生 8B：`{u32 fast @0, f32 hold @4}`）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
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
fn queued_replay_speed(replay: &ReplayBuffer, pending: &PendingHistoryCommands) -> f32 {
    let length = pending
        .0
        .len()
        .wrapping_add(usize::from(!replay.is_empty()));
    // B 0x140a7cb0b squares the integer before converting to f32.
    length.wrapping_mul(length) as f32 * 0.5
}

/// Original signifier resource: native {Some tag=1, elapsed f32} or None.
/// Its display/query consumer is owned by signifier_replay_active.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ReplayModeElapsedTime(pub Option<f32>);

/// Restoration selection at B 0x140a7c61d..68b. Editing object references do
/// not survive replay; terrain editing returns to the terrain tool.
pub fn restoration_mode(previous: AppMode, existing: AppMode) -> AppMode {
    match previous {
        AppMode::Replay | AppMode::LoadGame => existing,
        AppMode::Editing(EditSubject::Terrain(_)) => AppMode::Terrain,
        AppMode::Editing(_) | AppMode::CloneClutter => AppMode::Editing(EditSubject::None),
        mode => mode,
    }
}

/// 0x140a7c550。Replay 模式每帧登记名为 "replay" 的 UI 画布控件，参数取自机器码：
/// `UiState::clearing_canvas_w_layer(ui, "replay", len 6, argument 0x0d, layer 3, empty sounds)`。
/// 这是 UI 控件登记，不是清世界画布，不产生 `ClearCanvasCmd`。
pub fn replay_mode(
    mode: Res<AppMode>,
    time: Res<GameTime>,
    speed_override: Res<crate::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,
    actions: Res<MainActionSet>,
    (mut modes, mut restore, mut elapsed): (
        EventReader<AppModeChanged>,
        ResMut<RestoreAppTo>,
        ResMut<ReplayModeElapsedTime>,
    ),
    pending: Res<PendingHistoryCommands>,
    mut was_replay: Local<bool>,
    mut hold: ResMut<HoldToFastReplay>,
    mut replay_buffer: ResMut<ReplayBuffer>,
    ui: Res<UiState>,
    mut commands: EventWriter<HistoryCommand>,
) {
    if let Some(change) = modes.read().last().copied() {
        if is_replay_mode(change.current) {
            restore.0 = restoration_mode(change.previous, restore.0);
            elapsed.0 = Some(0.0);
            replay_buffer.speed = 1.0;
            hold.reset();
        }
    }
    let primary = actions.digital[16].phase as u32;
    let secondary = actions.digital[17].phase as u32;
    let engaged = primary < 3 || secondary < 3;
    let is_replay = mode.native_tag() == 13;
    if !is_replay {
        if *was_replay {
            log::info!("Replay mode finished");
            *was_replay = false;
        }
        return;
    }
    if !*was_replay {
        log::info!("Replay mode started");
        *was_replay = true;
    }
    ui.clearing_canvas_w_layer_and_sound("replay", REPLAY_CANVAS_ARGUMENT, REPLAY_CANVAS_LAYER);

    if let Some(speed) = speed_override.0 {
        replay_buffer.speed = speed;
        return;
    }
    let dt = time.delta;
    if !hold.fast {
        hold.advance(engaged, dt);
        if hold.fast {
            replay_buffer.speed = 1.0;
        } else {
            let minimum = queued_replay_speed(&replay_buffer, &pending);
            replay_buffer.speed = nan_aware_max(replay_buffer.speed, minimum);
        }
        return;
    }
    if primary < 3 {
        commands.write(HistoryCommand {
            bytes: REPLAY_FOCUS_PRIMARY_READY.to_le_bytes(),
        });
        let value = time_continuous_lerp(
            hold.hold_time,
            FAST_REPLAY_TARGET_SPEED,
            time.delta,
            FAST_REPLAY_LERP_DT,
        );
        hold.hold_time = value;
        replay_buffer.speed = value;
    } else if secondary >= 3 {
        commands.write(HistoryCommand {
            bytes: REPLAY_FOCUS_BOTH_READY.to_le_bytes(),
        });
        replay_buffer.speed = 1.0;
        hold.reset();
        let minimum = queued_replay_speed(&replay_buffer, &pending);
        replay_buffer.speed = nan_aware_max(replay_buffer.speed, minimum);
    } else {
        commands.write(HistoryCommand {
            bytes: REPLAY_FOCUS_AUX_READY.to_le_bytes(),
        });
        let value = time_continuous_lerp(
            hold.hold_time,
            FAST_REPLAY_TARGET_SPEED,
            time.delta,
            FAST_REPLAY_LERP_DT,
        );
        hold.hold_time = value;
        replay_buffer.speed = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::action_set::DigitalActionPhase;

    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(AppMode::Editing(EditSubject::None));
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        world.init_resource::<crate::systems::timelapse_replay::ReplaySpeedMultiplierOverride>();
        world.init_resource::<MainActionSet>();
        world.init_resource::<HoldToFastReplay>();
        world.init_resource::<RestoreAppTo>();
        world.init_resource::<ReplayModeElapsedTime>();
        world.init_resource::<Events<AppModeChanged>>();
        world.init_resource::<Events<HistoryCommand>>();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.init_resource::<PendingHistoryCommands>();
        world.insert_resource(UiState::at_startup());
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
        assert_eq!(
            replay_mode_transition(AppMode::LoadGame, AppMode::Water),
            None
        );
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
    fn system_registers_replay_canvas_every_frame_without_clearing_world() {
        let (mut world, mut schedule) = fixture();
        *world.resource_mut::<AppMode>() = AppMode::Replay;
        schedule.run(&mut world);
        schedule.run(&mut world);
        // 机器码：每帧登记 "replay" 画布控件，且不发清画布命令。
        let id = crate::systems::ui::resources::ui_state::WidgetId::clearing_canvas("replay");
        let canvas = world
            .resource::<UiState>()
            .clearing_canvas_params(id)
            .expect("replay canvas widget registered");
        assert_eq!((canvas.argument_79, canvas.layer), (0x0d, 3));
        // 覆盖速度。
        world
            .resource_mut::<crate::systems::timelapse_replay::ReplaySpeedMultiplierOverride>()
            .0 = Some(4.0);
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().speed, 4.0);
    }

    #[test]
    fn restoration_uses_last_mode_event_and_resets_shared_hold() {
        let (mut world, mut schedule) = fixture();
        *world.resource_mut::<AppMode>() = AppMode::Replay;
        world
            .resource_mut::<crate::systems::timelapse_replay::ReplaySpeedMultiplierOverride>()
            .0 = Some(1.0);
        *world.resource_mut::<HoldToFastReplay>() = HoldToFastReplay {
            fast: true,
            hold_time: 5.0,
        };
        world
            .resource_mut::<Events<AppModeChanged>>()
            .send(AppModeChanged {
                current: AppMode::Replay,
                previous: AppMode::Editing(EditSubject::Terrain(7)),
            });
        schedule.run(&mut world);
        assert_eq!(world.resource::<RestoreAppTo>().0, AppMode::Terrain);
        assert_eq!(
            *world.resource::<HoldToFastReplay>(),
            HoldToFastReplay::default()
        );
        assert_eq!(world.resource::<ReplayModeElapsedTime>().0, Some(0.0));
        assert_eq!(
            restoration_mode(AppMode::Editing(EditSubject::Wall(19)), AppMode::Path),
            AppMode::Editing(EditSubject::None)
        );
        assert_eq!(
            restoration_mode(AppMode::LoadGame, AppMode::Path),
            AppMode::Path
        );
    }

    #[test]
    fn fast_hold_emits_real_history_command_and_uses_frame_delta() {
        let (mut world, mut schedule) = fixture();
        *world.resource_mut::<AppMode>() = AppMode::Replay;
        world.resource_mut::<MainActionSet>().digital[16].phase = DigitalActionPhase::Held;
        *world.resource_mut::<HoldToFastReplay>() = HoldToFastReplay {
            fast: true,
            hold_time: 1.0,
        };
        world.resource_mut::<ReplayBuffer>().speed = 9.0;
        schedule.run(&mut world);
        let expected = time_continuous_lerp(1.0, 6.0, 0.1, 0.2);
        assert_eq!(world.resource::<ReplayBuffer>().speed, expected);
        let emitted: Vec<_> = world
            .resource_mut::<Events<HistoryCommand>>()
            .drain()
            .collect();
        assert_eq!(
            emitted,
            [HistoryCommand {
                bytes: [0, 0, 0, 3]
            }]
        );
    }

    #[test]
    fn fast_hold_raises_speed_from_one_on_later_frames() {
        let (mut world, mut schedule) = fixture();
        *world.resource_mut::<AppMode>() = AppMode::Replay;
        world.resource_mut::<MainActionSet>().digital[16].phase = DigitalActionPhase::Held;
        // dt=0.1：两帧 0.2 未过阈；第三帧过阈 → speed=1.0。
        schedule.run(&mut world);
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().speed, 1.0);
        assert!(world.resource::<Events<HistoryCommand>>().is_empty());
    }
}

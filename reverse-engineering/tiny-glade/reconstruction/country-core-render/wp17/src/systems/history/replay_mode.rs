//! `systems::history::replay_mode` — 回放节奏（0x140a7c550，WP17）。
//!
//! O（反编译 0x140a7c550 + enriched 接口 7e240e5e…）：
//!   - 系统签名：`(Res<AppMode>, Res<PendingHistoryCommands>,
//!     ResMut<ReplayBuffer>, EventReader<ModeJustChanged>,
//!     ResMut<HoldToFastReplay>, ResMut<RestoreAppTo>,
//!     ResMut<ReplayModeElapsedTime>, EventWriter<HistoryCmd>,
//!     Res<ActionSetStates>, Res<GameTime>, Res<UiState>,
//!     Res<ReplaySpeedMultiplierOverride>)`。
//!   - `RestoreAppTo`（enum2$；default@行19/set@行24/get@行50）：记回放前的
//!     AppMode 以便 finish_replay 恢复。
//!   - `HoldToFastReplay`：enum2$ 资源（长按快进）。
//!   - AppMode==0xd（Replay）时按输入动作发 `HistoryCmd`（redo 命令字
//!     0x03000100/0x03000000 变体），`UiState::clearing_canvas_w_layer_and_sound`，
//!     `time_continuous_lerp` 驱动 `ReplayModeElapsedTime` 计时。

use crate::resources::history::replay_cmd_buffer::ReplayBuffer;
use crate::systems::history::boundary::{AppMode, HistoryCmdFacade};
use crate::systems::history::signifier_replay_active::ReplayModeElapsedTime;
use crate::typed_gap::Gap;

/// 回放后要恢复的应用模式（O：`RestoreAppTo`；set@行24 / get@行50）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RestoreAppTo(pub Option<AppMode>);

impl RestoreAppTo {
    /// O：`set`（源码行 24；mode_manager 0x140a86b60 内联）。
    pub fn set(&mut self, mode: AppMode) {
        self.0 = Some(mode);
    }
    /// O：`get`（源码行 50；finish_replay 内联）。
    pub fn get(&self) -> Option<AppMode> {
        self.0
    }
}

/// 长按快进标记（O：enum2$ 资源；布局 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HoldToFastReplay(pub bool);

impl HoldToFastReplay {
    /// O：按住（输入动作 pressed 内联）。
    pub fn held(&self) -> bool {
        self.0
    }
}

/// 回放输入辅助（O：`DigitalActionState::pressed` 等内联；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct ReplayInputBoundary(pub Gap);

/// 快进计时阈值（O：f32 常量）。
pub const FAST_REPLAY_PRESS_THRESHOLD: f32 = 0.2;

/// O：`replay_mode` 0x140a7c550。骨架实现：AppMode==Replay 且无 pending 命令时
/// 按输入/计时决定是否发 redo HistoryCmd；非 Replay 时仅日志（log gate）。
///
/// bevy SystemParam 全签名见模块头注解；独立门禁以显式参数暴露核心逻辑。
pub fn replay_mode(
    app_mode: &AppMode,
    pending_commands: &crate::systems::history::boundary::PendingHistoryCommandsFacade,
    buffer: &mut ReplayBuffer,
    hold_to_fast: &mut HoldToFastReplay,
    restore_app_to: &mut RestoreAppTo,
    replay_elapsed: &mut ReplayModeElapsedTime,
    out_commands: &mut Vec<HistoryCmdFacade>,
    input: &ReplayInputBoundary,
    dt: f32,
) {
    let _ = (pending_commands, input);
    if !app_mode.is_replay() {
        // O：非 Replay 分支仅日志（DAT_143634d88 惰性标记）。
        return;
    }
    // O：进入回放时记录要恢复的模式（首帧 set）。
    if restore_app_to.0.is_none() {
        restore_app_to.set(AppMode::Other(0));
    }
    // O：`UiState::clearing_canvas_w_layer_and_sound` 边界（U）。
    // O：`time_continuous_lerp` 驱动 elapsed（boundary）。
    let _ = replay_elapsed.tick(dt);
    // O：输入 pressed 时按 HoldToFastReplay 发 redo 命令（0x03000100 变体；
    // 精确命令字见编排合同 U）。
    if hold_to_fast.held() {
        out_commands.push(HistoryCmdFacade::redo());
    }
    let _ = buffer;
}

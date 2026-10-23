//! events.rs — `country_core::resources::events` 命令事件族（组 A restorer 接线）。
//!
//! 原模块承载跨域命令事件（构造/模式/灯光等）；本处只落地组 A 六 crate 消费的
//! 事件类型合同，载荷语义 U（typed gap，不造数据）。

use bevy_ecs::event::Event;

/// B run_game_thread 0x14059d450 registers this unit event under native
/// resource TypeId low word 0x2e383ebb9376bfa1. OS close sends it; the game
/// shutdown system decides when the loop can finish.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct ShutdownGameCmd;

/// B ConstructTimberframeCmd: UI producer14155dc66..dc87 writes a 32B
/// payload followed by the 8B EventId (40B EventInstance). The first28B
/// reuse ShapeParameters' canonical encoding; roof_gate is independently
/// stored at28 and both timberframe consumers generate roof beams iff0.
#[repr(C, align(4))]
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConstructTimberframeCmd {
    pub shape: [u8; 28],
    /// Native byte gate, nominal style semantics remain unresolved.
    pub roof_gate: u8,
    // Rust-safe padding, not an inferred native business field.
    reserved: [u8; 3],
}
impl ConstructTimberframeCmd {
    pub fn from_shape(
        shape: crate::systems::wall::valid_enclosure::ShapeParameters,
        roof_gate: u8,
    ) -> Self {
        Self {
            shape: shape.to_record(),
            roof_gate,
            reserved: [0; 3],
        }
    }
}

/// Native publisher 140a875e0 writes current AppMode at +0, previous at
/// +0x10, then Bevy EventId at +0x20; finally previous := current.
/// Keep one semantic event identity and both complete AppMode payloads.
/// The rebuilding compiler's private enum layout is not a native ABI contract.
pub use crate::systems::mode_manager::AppModeChanged as ModeJustChanged;

/// 菜单刚打开事件（O：`resources::gui::EscapeMenuJustOpened` 族——原模块
/// `resources::gui`；autoclose_color_menu 消费。载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct EscapeMenuJustOpened;

/// 点灯命令（O：`resources::events::LightLanternCmd`；color_menu 0x142291230
/// 的 EventWriter 面。载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct LightLanternCmd;

/// B1407b5980 消费最后一个未读点灯命令的装饰物编号。
/// 原事件记录的 u32 载荷位于 +8，事件队列的头部与填充不属于业务字段。
#[derive(Event, Clone, Copy, Debug)]
pub struct LightWindowCmd(pub u32);

/// 输入记录收尾命令（O：`resources::history::FinishInputRecordingCmd` 族——原
/// 模块 `resources::history`；color_menu 的 EventWriter 面。载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct FinishInputRecordingCmd;

/// 过山车提示事件（O：`resources::events::DisplayRollercoasterPrompt`；
/// `add_onboarding` 0x140650d50 register_event 顺序 21；`prompt_rollercoaster`
/// 0x14227bb70 消费后以 `DisplayHintRequest(0x1200, id 10)` + 常量 "Rollercoaster.Ride"
/// 触发过山车乘坐提示。unit）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct DisplayRollercoasterPrompt;

/// 写设置文件命令（O：`tiny_glade::app_config::WriteSettingsFileCmd`；unit 事件，
/// `exponential_back_off` 0x142284800 / `reset_hints` 0x141c1c8d0 /
/// `tutorial_completion` 0x14227c230 在提示区变更后 `EventInstance<..>` 追加
/// （8B 步长 = 纯 unit 载荷）。原模块路径 `tiny_glade::app_config`；落库在
/// country-core 供上层 onboarding/settings 消费）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct WriteSettingsFileCmd;

/// 保存玩家档案命令（O：`tiny_glade::app_config::SaveGameProfileCmd` 族；与
/// `WriteSettingsFileCmd` 同域，unit。启动会话经 `save_game_profile` 0x140917230
/// 写盘）。落库位置同 `WriteSettingsFileCmd`（app_config 域，I）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SaveGameProfileCmd;
pub use crate::systems::wall::{draw_wall::RecordMouseInputIntoWall, finish_wall::FinishWall};

/// Original wall-height adjustment entry notification. The signifier audio
/// dispatcher consumes the entire channel by presence, without a payload read.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct InitAdjWallHeightMode;

/// B `country_core::resources::events::FinishAdjWallHeightMode`：墙高度编辑模式结束
/// 通知（unit，事件实例仅 EventId）。`history_record_adjust_wall_height`
/// （0x140ac79e0 `EventReader<FinishAdjWallHeightMode>`）只判空并整队消费，
/// 载荷未被任何已取证消费者读取。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct FinishAdjWallHeightMode;

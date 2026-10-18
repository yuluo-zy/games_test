//! tutorial_completion（O 0x14227c230，296）：EventReader<TutorialHintComplete> →
//! 按 id 置 GameProfile 完成标志（18 路 switch）→ EventWriter<WriteSettingsFileCmd>。
//!
//! id→GameProfile 偏移（O 反编译 0x14227c230 各 case 写入字节）：
//! 0→0x1f4 1→0x1f5 2→0x1fe 3→0x1fc 4→0x1fa 5→0x1fb 6→0x1f9 7→0x1ff 8→0x1fd
//! 9→0x200 10→0x1f6 11→0x1f7 12→0x1f8 13→0x201 14→0x202 15→0x203 16→0x204 17→0x205。
//! 归属（I）：0 DeselectTool 1 FocusedEdit 2 Undo 3 DisableSnapping 4 TerrainSteepness
//! 5 TerrainFocus 6 Terrain 7 RoofShape 8 UnlinkWindows 9 ClutterFocus 10 CameraPan
//! 11 CameraRotate 12 CameraZoom 13..17 Stairs 五阶段。
//!
//! **已重建（2026-10-17，coregap-2）**：GameProfile 完成标志
//! （`mark_tutorial_op_complete`）+ `WriteSettingsFileCmd` 落库；写面修正为
//! **WriteSettingsFileCmd**（早前 RESTORE 表 "SaveGameProfileCmd" 表述改为同
//! app_config 域 I）。逐事件判等（事件步长 0x10B、id@+8、每个事件写一次命令）。

use bevy_ecs::prelude::*;

/// 教程提示完成事件（O：`system_onboarding::tutorial_completion::
/// TutorialHintComplete`，0x10B 步长，id@+8）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TutorialHintComplete {
    pub id: u8,
}

/// id → GameProfile 完成标志偏移（O 0x14227c230 switch 逐 case；见模块头）。
pub const COMPLETION_FLAG_OFFSET: [usize; 18] = [
    0x1f4, 0x1f5, 0x1fe, 0x1fc, 0x1fa, 0x1fb, 0x1f9, 0x1ff, 0x1fd, 0x200, 0x1f6, 0x1f7, 0x1f8,
    0x201, 0x202, 0x203, 0x204, 0x205,
];

/// tutorial_completion（O 0x14227c230）：读完成事件，按 id 置 GameProfile 完成标志
/// （`TutorialOp::completion_flag_offset` = 上述 switch），每个事件写一次
/// `WriteSettingsFileCmd`。**判等**（fresh 0x14227c230 逐 case 核对，2026-10-17）。
pub fn tutorial_completion(
    mut events: EventReader<TutorialHintComplete>,
    mut profile: ResMut<country_core::game_profile::GameProfile>,
    mut writer: EventWriter<country_core::resources::events::WriteSettingsFileCmd>,
) {
    for event in events.read() {
        // 原 18 路 switch：id ∈ 0..0x11 → 对应偏移置 1；越界（≥0x12）无 default →
        // 不改标志但仍写命令。
        if let Some(op) = country_core::resources::onboarding::TutorialOp::from_id(event.id) {
            profile.mark_tutorial_op_complete(op);
        }
        writer.write(country_core::resources::events::WriteSettingsFileCmd);
    }
}

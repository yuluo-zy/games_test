//! tutorial_completion（O 0x14227c230，296）：EventReader<TutorialHintComplete> →
//! 按 id 置 GameProfile 完成标志（18 路 switch）→ EventWriter<SaveGameProfileCmd>。
//!
//! id→GameProfile 偏移（O 反编译 0x14227c230 各 case 写入字节）：
//! 0→0x1f4 1→0x1f5 2→0x1fe 3→0x1fc 4→0x1fa 5→0x1fb 6→0x1f9 7→0x1ff 8→0x1fd
//! 9→0x200 10→0x1f6 11→0x1f7 12→0x1f8 13→0x201 14→0x202 15→0x203 16→0x204 17→0x205。
//! 归属（I）：0 DeselectTool 1 FocusedEdit 2 Undo 3 DisableSnapping 4 TerrainSteepness
//! 5 TerrainFocus 6 Terrain 7 RoofShape 8 UnlinkWindows 9 ClutterFocus 10 CameraPan
//! 11 CameraRotate 12 CameraZoom 13..17 Stairs 五阶段。
//!
//! **未实现**：country-core `GameProfile` 未承载 0x1f4..0x205 完成标志、
//! `SaveGameProfileCmd` 未落库 → 函数体不能按原签名落位；id→偏移映射已逐 case 核对
//! （下表），落库后可照 0x14227c230 逐语句重建。

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

/// tutorial_completion（O 0x14227c230）：读完成事件，按 id 置 GameProfile 偏移为 1，
/// 写 SaveGameProfileCmd。
/// **未实现**（分歧点）：GameProfile 提示区 0x1f4..0x205 未落库、SaveGameProfileCmd
/// 未落库。骨架保留，重建依赖 country-core 补字段后落位。
pub fn tutorial_completion(
    _events: EventReader<TutorialHintComplete>,
    _profile: ResMut<country_core::game_profile::GameProfile>,
) {
    // 未实现：见模块注释。
}

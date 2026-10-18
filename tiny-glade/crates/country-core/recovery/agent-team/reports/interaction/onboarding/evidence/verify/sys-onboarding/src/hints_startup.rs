//! hints_startup（O 0x14227b450，1433）：启动时按 GameProfile 完成标志插入 12 个
//! 条件资源。
//!
//! 真逻辑（evidence/batch3-core.txt 逐命令字节）：
//! - DeselectToolHintConditions←profile[0x1f4]；FocusedEditHintConditions←[0x1f5]；
//!   HintTerrainConditions←[0x1f9]；HintTerrainSteepnessConditions←[0x1fa]×2；
//!   HintDisableSnappingConditions←[0x1fc]；UndoHintConditions←0；HintRoofShapeConditions
//!   ←[0x1ff]×2；FocusedClutterHintConditions←[0x200]；HintFocusTerrainConditions←[0x1fb]；
//! - CameraHintsState：state=(0x1f6&&0x1f7&&0x1f8)?2:0，其余清零；
//! - CameraHintProgress：进度初值=各相机完成标志（0x1f6/0x1f7/0x1f8→1.0/0.0），
//!   阈值初值=(0.7,0.7,60.0)（0x3f333333/0x3f333333/0x42700000）；
//! - StairsHintConditions：清零。
//!
//! **未实现**：GameProfile 0x1f4..0x205 未落库 → 初值来源缺失；资源构造序列已登记，
//! 落库后可照 0x14227b450 重建 Commands::insert_resource 序列。

use bevy_ecs::prelude::*;

/// hints_startup（O 0x14227b450）：见模块头 12 资源插入序列。
/// **未实现**：GameProfile 提示区未落库；骨架保留。
pub fn hints_startup(
    _profile: Res<country_core::game_profile::GameProfile>,
    _commands: Commands,
) {
    // 未实现：见模块头分歧点。
}

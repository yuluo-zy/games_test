//! reset_hints（O 0x141c1c8d0，838）：复位全部提示状态（新 glade/新会话）。
//!
//! 真逻辑（evidence/batch3-core.txt）：读原TutorialOp事件（载荷 u8 @+8）。首事件
//! 载荷字节==0 → GameProfile 0x1f4..0x206 整段置 1（0x0101010101010101），状态=2；
//! 否则 → 12 条件资源复位（CameraHintProgress 阈值写 0.7/0.7/60=0x3f333333…/0x42700000，
//! 与 hints_startup 相同）、GameProfile 0x1f4..0x206 清零、0x1e0/0x1e8 清零，状态=0；
//! 最后写 SaveGameProfileCmd。常量：0x0101010101010101、0.7/0.7/60。
//!
//! **已重建（2026-10-17，coregap-2 GameProfile + SaveGameProfileCmd 落库）**；
//! 输入已按设置页producer及PDB EventReader<TutorialOp>对齐；兼容名字不创建第二通道。

use bevy_ecs::prelude::*;

/// 原EventReader<TutorialOp>。旧ResetHintsCommand仅作兼容导出，保持同一事件身份。
pub use country_core::resources::onboarding::TutorialOp as ResetHintsCommand;

/// 提示复位状态（O：reset 后状态 2/0 语义 U）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintResetState {
    pub pending: bool,
}

/// reset_hints（O 0x141c1c8d0）：见模块头。**重建**（2026-10-17）。
#[allow(clippy::too_many_arguments)]
pub fn reset_hints(
    mut trigger: EventReader<ResetHintsCommand>,
    mut profile: ResMut<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintResetState>,
    mut writer: EventWriter<country_core::game_profile::SaveGameProfileCmd>,
    mut deselect: ResMut<crate::hint_deselect_tool::DeselectToolHintConditions>,
    mut focused: ResMut<crate::hint_focused_edit::FocusedEditHintConditions>,
    mut terrain: ResMut<crate::hint_terrain::HintTerrainConditions>,
    mut steepness: ResMut<crate::hint_terrain_steepness::HintTerrainSteepnessConditions>,
    mut snapping: ResMut<crate::hint_disable_snapping::HintDisableSnappingConditions>,
    mut undo: ResMut<crate::hint_undo::UndoHintConditions>,
    mut roof_shape: ResMut<crate::hint_roof_shape::HintRoofShapeConditions>,
    mut clutter: ResMut<crate::hint_clutter_focus_edit::FocusedClutterHintConditions>,
    mut terrain_focus: ResMut<crate::hint_terrain_focus::HintFocusTerrainConditions>,
    mut camera_state: ResMut<crate::hint_camera_controls::CameraHintsState>,
    mut camera_progress: ResMut<crate::hint_camera_progress::CameraHintProgress>,
    mut stairs: ResMut<crate::hint_stairs::StairsHintConditions>,
) {
    if let Some(first) = trigger.read().next() {
        if *first == ResetHintsCommand::DisableHints {
            // O：0x1f4..0x206 全置 1（0x0101010101010101），state=2。
            profile.set_all_hint_flags();
            state.pending = true;
        } else {
            // O：12 条件资源复位 + CameraHintProgress 阈值 0.7/0.7/60 + 提示区清零。
            *deselect = Default::default();
            *focused = Default::default();
            *terrain = Default::default();
            *steepness = Default::default();
            *snapping = Default::default();
            *undo = Default::default();
            *roof_shape = Default::default();
            *clutter = Default::default();
            *terrain_focus = Default::default();
            *camera_state = Default::default();
            *camera_progress = Default::default();
            *stairs = Default::default();
            profile.reset_hint_flags();
            state.pending = false;
        }
        writer.write(country_core::game_profile::SaveGameProfileCmd);
    }
}

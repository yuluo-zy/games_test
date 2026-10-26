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
//! **已重建（2026-10-17，coregap-2 GameProfile 提示区落库）**：按 0x14227b450
//! Commands::insert_resource 序列照落；各条件资源字节值 = 对应完成标志（≤2 字节的
//! 资源字段名 I，值判等）。

use bevy_ecs::prelude::*;

/// hints_startup（O 0x14227b450）：见模块头 12 资源插入序列。**判等**（2026-10-17）。
pub fn hints_startup(
    profile: Res<country_core::game_profile::GameProfile>,
    mut commands: Commands,
) {
    // 命令序列与 O 一致：DeselectTool / FocusedEdit / Terrain / TerrainSteepness /
    // DisableSnapping / Undo / RoofShape / FocusedClutter / CameraHintsState /
    // TerrainFocus / CameraHintProgress / Stairs。
    commands.insert_resource(crate::hint_deselect_tool::DeselectToolHintConditions {
        count: 0,
        hint_done: profile.tutorial,
        state: 0,
    });
    commands.insert_resource(crate::hint_focused_edit::FocusedEditHintConditions {
        count: 0,
        hint_done: profile.focused_edit_completed,
        state: 0,
    });
    commands.insert_resource(crate::hint_terrain::HintTerrainConditions {
        count: 0,
        hint_done: profile.terrain_layering_completed,
        event_byte: 0,
    });
    commands.insert_resource(
        crate::hint_terrain_steepness::HintTerrainSteepnessConditions {
            hint_done: profile.terrain_steepness_completed,
            event_byte: profile.terrain_steepness_completed as u8,
        },
    );
    commands.insert_resource(
        crate::hint_disable_snapping::HintDisableSnappingConditions {
            hint_done: profile.disable_snapping_completed,
        },
    );
    commands.insert_resource(crate::hint_undo::UndoHintConditions::default());
    commands.insert_resource(crate::hint_roof_shape::HintRoofShapeConditions {
        hint_done: profile.roof_shape_completed,
        event_byte: profile.roof_shape_completed as u8,
    });
    commands.insert_resource(
        crate::hint_clutter_focus_edit::FocusedClutterHintConditions {
            hint_done: profile.clutter_focused_edit_completed,
            state: 0,
        },
    );
    let camera_state = if profile.camera_pan_completed
        && profile.camera_rotate_completed
        && profile.camera_zoom_completed
    {
        2
    } else {
        0
    };
    commands.insert_resource(crate::hint_camera_controls::CameraHintsState {
        state: camera_state,
        counters: [0.0; 4],
    });
    commands.insert_resource(crate::hint_terrain_focus::HintFocusTerrainConditions {
        hint_done: profile.terrain_focus_completed,
    });
    commands.insert_resource(
        crate::hint_camera_progress::CameraHintProgress::with_startup_thresholds(
            profile.camera_pan_completed,
            profile.camera_rotate_completed,
            profile.camera_zoom_completed,
        ),
    );
    commands.insert_resource(crate::hint_stairs::StairsHintConditions::default());
}

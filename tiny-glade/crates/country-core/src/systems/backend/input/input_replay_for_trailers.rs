//! `input_replay_for_trailers.rs` — trailer 专用输入录制/回放（B 0x14083c620 / 0x14083cdd0，
//! O）。
//!
//! 证据：`chapters/systems/backend/input/input_replay_for_trailers.rs.md`、
//! `ui-input/DIGEST.md` §2.3、反编译 `decomp-0x14083c620.txt` / `decomp-0x14083cdd0.txt`。
//!
//! `record_and_replay_input_for_trailers` @0x14083c620：
//! 1. 配置字节 @+0xfc == 1 才启用（`do_input_replay_ui` 的 "Enable input recording" checkbox）。
//! 2. 命中按键表（decomp 直读键码：`O` = `0x22`、`E`/第二键 = `0x1c`）：
//!    - 当前模式 0/4（AutoRecordPanicRepro/None）→ log `"Start recording"`（0x142a93740）+
//!      `MainCamera::store_parameters()` + 重建流（1 同步点）+ mode=AutoRecordPanicRepro；
//!    - 当前模式 1（Records）→ log `"Finished recording"`（0x142a93678）+ 释放流 + mode=None；
//!    - 当前模式 2/3 → no-op；
//!    - 第二键（0x1c）→ log `"Replay..."`（0x142a93778）+ mode=Replay + index=-1 + 回退最后
//!      同步点（恢复 `InputTranslationState` + Toolbar 类别 + `MainCamera::load_parameters`）。
//!
//! `do_input_replay_ui` @0x14083cdd0：egui 面板——"Enable input recording" checkbox（写
//! @+0xfc）、"Stream frame count: {}" 标签、`` `O` - start/stop recording `` 键提示、
//! Save 按钮（路径含 `"tmp/input_replay"` + `"json"`）。
use bevy_ecs::prelude::*;

use crate::{
    input::InputRecordReplayMode,
    resources::toolbar::ToolbarCategoryModes,
    systems::camera_rig::main_camera::{
        CameraRigParameters, MainCamera, NORMAL_FOV_OFFSET, NORMAL_PITCH_OFFSET,
        NORMAL_POSITION_OFFSET, NORMAL_YAW_OFFSET,
    },
};

use super::{
    button_input::KeyButtonInput,
    input_record_replay::{InputRecordReplayState, InputTranslationStateResource},
};

const LOG_TARGET: &str = "country_core::systems::backend::input::input_replay_for_trailers";

/// trailer 录制面板配置资源（O：0x14083c620 读 `配置字节@+0xfc == 1`；do_input_replay_ui
/// 的 checkbox 写同一字节）。字段名 I（面板语义来自章节串表）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct InputReplayForTrailers {
    /// 配置字节 @+0xfc：启用输入录制。
    pub enabled: bool,
}

/// trailer 录制触发键码（O：0x14083c620 `0x22` / `0x1c`，bevy 风格键码）。
const KEY_RECORD_TOGGLE: u16 = 0x22;
const KEY_REPLAY: u16 = 0x1c;

/// `record_and_replay_input_for_trailers` @0x14083c620（O）。
///
/// decomp 参数：param_1 = ResMut<InputRecordReplayState>、param_2 = Res<KeyButtonInput>、
/// param_3 = Res<MainCamera>、param_4 = Res<InputReplayForTrailers>（@+0xfc）、
/// param_5/param_6 = 回退恢复用 ResMut（游标 + Toolbar 类别）。
pub fn record_and_replay_input_for_trailers(
    mut replay: ResMut<InputRecordReplayState>,
    keyboard: Res<KeyButtonInput>,
    camera: Res<MainCamera>,
    config: Res<InputReplayForTrailers>,
    toolbar: Res<ToolbarCategoryModes>,
    mut translation: ResMut<InputTranslationStateResource>,
) {
    if !config.enabled {
        return;
    }
    if keyboard.just_pressed(KEY_RECORD_TOGGLE) {
        match replay.mode {
            InputRecordReplayMode::AutoRecordPanicRepro | InputRecordReplayMode::None => {
                log::info!(target: LOG_TARGET, "Start recording");
                // 相机参数入栈 + 重建流（1 同步点、帧容量 8）+ mode = AutoRecordPanicRepro。
                let _params = camera.store_parameters();
                let _toolbar_snapshot = toolbar.clone();
                replay.stream = crate::input::input_stream::InputStream::new();
                replay.mode = InputRecordReplayMode::AutoRecordPanicRepro;
                replay.index = 0;
            }
            InputRecordReplayMode::Records => {
                log::info!(target: LOG_TARGET, "Finished recording");
                replay.stream = crate::input::input_stream::InputStream::new();
                replay.mode = InputRecordReplayMode::None;
            }
            InputRecordReplayMode::Replay | InputRecordReplayMode::DisableInput => {}
        }
    }
    if keyboard.just_pressed(KEY_REPLAY) {
        log::info!(target: LOG_TARGET, "Replay...");
        replay.mode = InputRecordReplayMode::Replay;
        replay.index = u64::MAX;
        // 回退最后同步点：index = 同步点.frame_index - 1；恢复翻译状态 buffer。
        let last = replay
            .stream
            .sync_points()
            .back()
            .map(|point| (point.frame_index, point.state));
        if let Some((frame_index, state)) = last {
            replay.index = frame_index.wrapping_sub(1);
            translation.buffer = state;
            // Toolbar 类别 / 相机参数从同步点状态块恢复（O：sync+0x20 / +0x30）；状态块的
            // 子字段布局未解码（crate 的 InputTranslationState 为不透明 [u64;11]），待取证。
            let _ = &toolbar;
            let _ = &camera;
        }
    }
}

/// 从 0x24 B 相机参数记录恢复 MainCamera Normal body（`load_parameters` 0x140911520 的
/// crate 内等价；`store_parameters` 的反向）。I：字段布局 = [`CameraRigParameters`]。
pub fn load_camera_parameters(camera: &mut MainCamera, params: &CameraRigParameters) {
    camera.write_f32(NORMAL_POSITION_OFFSET, params.position.x);
    camera.write_f32(NORMAL_POSITION_OFFSET + 4, params.position.y);
    camera.write_f32(NORMAL_POSITION_OFFSET + 8, params.position.z);
    camera.write_f32(NORMAL_YAW_OFFSET, params.yaw);
    camera.write_f32(NORMAL_PITCH_OFFSET, params.pitch);
    camera.write_f32(NORMAL_FOV_OFFSET, params.vertical_fov_degrees);
}

/// `do_input_replay_ui` @0x14083cdd0（O 串，egui 面板）。
pub fn do_input_replay_ui(
    mut config: ResMut<InputReplayForTrailers>,
    replay: Res<InputRecordReplayState>,
    mut ui: ResMut<crate::resources::gui::EguiContextResource>,
) {
    let context = &mut ui.0;
    egui::Window::new("Input replay")
        .show(context, |ui| {
            ui.checkbox(&mut config.enabled, "Enable input recording");
            ui.label(format!("Stream frame count: {}", replay.stream.frame_count()));
            ui.label("`O` - start/stop recording");
            ui.label("`E` - replay");
            if ui.button("Save").clicked() {
                // O：保存路径含 "tmp/input_replay" + "json"（serde 形态待取证）。
                log::info!(target: LOG_TARGET, "tmp/input_replay.json");
            }
            if ui.button("Replay sheep").clicked() {
                // I：回放触发（串证据；动作待取证）。
                log::info!(target: LOG_TARGET, "Replay sheep");
            }
        });
}

//! 录制快捷键和面板只产生控制命令，所有状态切换由控制器统一处理。
use super::{
    button_input::KeyButtonInput,
    input_record_replay::{InputRecordReplayState, InputReplayCommand},
};
use crate::input::InputRecordReplayMode;
use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct InputReplayForTrailers {
    pub enabled: bool,
}
const KEY_RECORD_TOGGLE: u16 = 0x22;
const KEY_REPLAY: u16 = 0x1c;

pub fn record_and_replay_input_for_trailers(
    replay: Res<InputRecordReplayState>,
    keyboard: Res<KeyButtonInput>,
    config: Res<InputReplayForTrailers>,
    mut commands: EventWriter<InputReplayCommand>,
) {
    if !config.enabled {
        return;
    }
    if keyboard.just_pressed(KEY_RECORD_TOGGLE) {
        let command = if replay.mode == InputRecordReplayMode::Records {
            InputReplayCommand::Stop
        } else {
            InputReplayCommand::StartRecording
        };
        commands.write(command);
    }
    if keyboard.just_pressed(KEY_REPLAY) {
        commands.write(InputReplayCommand::Replay { rewind: 0 });
    }
}

pub fn do_input_replay_ui(
    mut config: ResMut<InputReplayForTrailers>,
    replay: Res<InputRecordReplayState>,
    context: Res<crate::resources::gui::EguiContextResource>,
    mut commands: EventWriter<InputReplayCommand>,
) {
    egui::Window::new("输入录制与回放").show(&context.0, |ui| {
        ui.checkbox(&mut config.enabled, "启用录制快捷键");
        ui.label(format!(
            "模式：{}；帧数：{}",
            replay.mode,
            replay.stream.frame_count()
        ));
        if ui.button("开始录制").clicked() {
            commands.write(InputReplayCommand::StartRecording);
        }
        if ui.button("停止").clicked() {
            commands.write(InputReplayCommand::Stop);
        }
        if ui
            .add_enabled(
                replay.stream.frame_count() > 0,
                egui::Button::new("从最近同步点回放"),
            )
            .clicked()
        {
            commands.write(InputReplayCommand::Replay { rewind: 0 });
        }
        if let Some(error) = &replay.last_error {
            ui.colored_label(egui::Color32::RED, error);
        }
        ui.label("O：开始/停止录制；I：回放");
    });
}

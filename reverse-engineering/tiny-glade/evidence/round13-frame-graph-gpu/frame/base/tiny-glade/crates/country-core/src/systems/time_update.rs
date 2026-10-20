//! 控制命令之后、动作翻译之前，使用本帧时间源更新完整时钟。
use bevy_ecs::prelude::*;

use crate::{
    resources::{
        game_clock::{ClockGate, DeltaSource},
        gui::EscMenuState,
        shader_refactoring_state::ShaderRefactoringState,
        time::GameTime,
    },
    systems::backend::input::input_record_replay::InputRecordReplayState,
};

pub fn time_update(
    elapsed: Res<bevy_time::Time>,
    refactoring: Res<ShaderRefactoringState>,
    menu: Res<EscMenuState>,
    mut replay: ResMut<InputRecordReplayState>,
    mut game: ResMut<GameTime>,
) {
    let gate = ClockGate::from_menu(refactoring.disabled(), &menu, game.paused);
    let source = if let Some(frame) = replay.current_frame() {
        if let Err(error) = frame.validate() {
            replay.fail_current_frame(error);
            game.delta = 0.0;
            game.ticks_60 = 0;
            game.ran = false;
            return;
        }
        DeltaSource::Replay {
            frame_delta: frame.time_delta_seconds,
        }
    } else {
        let delta = elapsed.delta();
        DeltaSource::Live {
            secs: delta.as_secs(),
            nanos: delta.subsec_nanos(),
        }
    };
    // GameClock 保留原算法的写入顺序；先更新副本，避免错误留下半更新时钟。
    let mut next_clock = *game;
    match next_clock.update(gate, source) {
        Ok(()) => *game = next_clock,
        Err(error) => {
            replay.fail_current_frame(error);
            game.delta = 0.0;
            game.ticks_60 = 0;
            game.ran = false;
        }
    }
}

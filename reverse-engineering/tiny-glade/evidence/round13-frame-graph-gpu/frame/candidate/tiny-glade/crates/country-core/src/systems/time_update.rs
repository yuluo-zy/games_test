//! 控制命令之后、动作翻译之前，使用本帧时间源更新完整时钟。
use bevy_ecs::prelude::*;

use crate::{
    resources::{
        game_clock::{ClockGate, DeltaSource, GameClockError},
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
    replay: Res<InputRecordReplayState>,
    mut game: ResMut<GameTime>,
) {
    let gate = ClockGate::from_menu(refactoring.disabled(), &menu, game.paused);
    // B 140922c61..c83 checks the replay index before reading +0xc8.
    // Replay mode with no current frame is an invariant failure, not live time.
    let source = if replay.mode == crate::input::InputRecordReplayMode::Replay {
        let frame = replay.current_frame().expect("Out of bounds access");
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
    // B writes delta at 140922ca0/140922d7f, then converts and adds it.
    // Invalid input/overflow panics before any later clock writes. Keep those
    // effects on the actual resource; rolling back or zeroing it changes B.
    if let Err(error) = game.update(gate, source) {
        match error {
            GameClockError::InvalidDelta if game.delta < 0.0 => {
                panic!("cannot convert float seconds to Duration: value is negative");
            }
            GameClockError::InvalidDelta => {
                panic!("cannot convert float seconds to Duration: value is either too big or NaN");
            }
            GameClockError::DurationOverflow => panic!("overflow when adding durations"),
        }
    }
}

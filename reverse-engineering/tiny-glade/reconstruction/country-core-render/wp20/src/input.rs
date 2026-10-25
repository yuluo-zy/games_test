//! `input` — INTEGRATION MARKER facade for the wp15-recovered input domain.
//!
//! The real types live in `reconstruction/country-core/src/input/` (wp15).
//! This standalone gate declares typed-gap mirrors so the WP20 systems
//! (`systems::backend::input`) compile offline; the integration owner maps
//! these to the real wp15 types (same names / module paths). Business logic is
//! NOT re-recovered here.

#![allow(dead_code, unused_imports)]

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::Gap;

/// `input/input_stream.rs` (wp15).
pub mod input_stream {
    use super::*;

    /// `InputTranslationState` (wp15; 88 B snapshot in each sync point).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct InputTranslationState {
        pub _gap: Gap,
    }

    /// `InputFrame` (wp15; 0xd0 B per-frame input record).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct InputFrame {
        pub _gap: Gap,
    }

    /// `InputStreamSyncPoint` (wp15; 0x60 B).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct InputStreamSyncPoint {
        pub translation: InputTranslationState,
        pub frame_index: u64,
    }

    /// `InputStream` (wp15; 64 B input-recording ring).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct InputStream {
        pub sync_points: VecDeque<InputStreamSyncPoint>,
        pub frames: VecDeque<InputFrame>,
    }

    impl InputStream {
        pub fn new() -> Self {
            InputStream {
                sync_points: VecDeque::new(),
                frames: VecDeque::new(),
            }
        }
        pub fn frame_count(&self) -> usize {
            self.frames.len()
        }
        pub fn add_frame(&mut self, _frame: InputFrame, _record: bool) {
            todo!("wp15 InputStream::add_frame")
        }
        pub fn on_history_sync(&mut self, _translation: &InputTranslationState, _record: bool) {
            todo!("wp15 InputStream::on_history_sync")
        }
        pub fn drop_oldest_sync_point(&mut self) {
            todo!("wp15 InputStream::drop_oldest_sync_point")
        }
        pub fn last_frame_mut(&mut self) -> Option<&mut InputFrame> {
            self.frames.back_mut()
        }
    }

    /// `InputRecordReplayMode` (wp15; mode u64 tags `0x8000000000000000..4`).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum InputRecordReplayMode {
        #[default]
        Idle,
        Recording,
        Replaying,
        Finished,
    }

    /// `InputRecordReplayState` (wp15; 80 B resource `{stream, mode@+0x40,
    /// index@+0x48}`).
    #[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
    pub struct InputRecordReplayState {
        pub stream: InputStream,
        pub mode: InputRecordReplayMode,
        pub index: i64,
    }

    impl InputRecordReplayState {
        pub fn new() -> Self {
            InputRecordReplayState {
                stream: InputStream::new(),
                mode: InputRecordReplayMode::Idle,
                index: -1,
            }
        }
    }

    /// `InputReplayInitialState` (wp15).
    #[derive(Clone, Debug, Default)]
    pub struct InputReplayInitialState {
        pub _gap: Gap,
    }
}

/// `input/action_set.rs` (wp15).
pub mod action_set {
    use super::*;

    /// `DigitalActionState` (wp15).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub enum DigitalActionState {
        #[default]
        Released,
        Pressed,
        JustPressed,
        JustReleased,
    }

    /// `ActionSetAction` (wp15).
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionSetAction {
        Analog(glam::Vec2),
        Digital(DigitalActionState),
    }

    /// `ActionSetStates` (wp15 resource).
    #[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
    pub struct ActionSetStates(pub Gap);

    /// `ActionSetValues` (wp15 resource).
    #[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
    pub struct ActionSetValues(pub Gap);

    /// `DigitalActionSetActionType` (wp15).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct DigitalActionSetActionType(pub u16);
}

/// `input/mod.rs` (wp15): `ActiveInputMode` / `ActiveInputModeTracker`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ActiveInputMode {
    #[default]
    MouseKeyboard,
    Controller,
}

#[derive(Clone, Debug, Default)]
pub struct ActiveInputModeTracker {
    pub _gap: Gap,
}

/// `MouseMoved` event (wp15 `translate_input` consumer; layout U).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct MouseMoved {
    pub delta: glam::Vec2,
}

/// `InputCursorMoved` / `InputCursorDelta` events (WP20
/// `translate_input_to_cursor_moved` outputs; layout U).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct InputCursorMoved {
    pub delta: glam::Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct InputCursorDelta {
    pub _gap: Gap,
}

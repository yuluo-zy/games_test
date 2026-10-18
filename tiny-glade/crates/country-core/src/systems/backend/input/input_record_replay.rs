//! 输入录制/回放控制。文件操作与队列策略分别由 input::recording、input_stream 提供。
use super::update_cursor_position::CursorPosition;
pub use crate::input::recording::{load_recording, save_recording, InputFrame};
use crate::{
    input::{
        action_set::{ActionSetStates, ActionSetValues},
        input_stream::{InputSnapshot, InputStream, RecordingRetention},
        InputRecordReplayMode,
    },
    resources::{
        mouse_capture::{CursorCapturePolicy, CursorCaptureState},
        mouse_terrain_raycast::CursorPositionSS,
        time::GameTime,
        toolbar::Toolbar,
    },
    systems::{camera_rig::main_camera::MainCamera, mode_manager::AppMode},
};
use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Event, Clone, Debug, PartialEq)]
pub enum InputReplayCommand {
    Sync { record: bool },
    Load { path: PathBuf, rewind: u64 },
    StartRecording,
    Stop,
    Replay { rewind: u64 },
}

#[derive(Resource, Clone, Debug)]
pub struct InputRecordReplayState {
    pub stream: InputStream<InputFrame>,
    pub mode: InputRecordReplayMode,
    next_frame_index: usize,
    frame_applied: bool,
    input_suppressed_this_frame: bool,
    release_capture_requested: bool,
    /// 命令失败可供 UI 展示；错误不会清空正在进行的录制。
    pub last_error: Option<String>,
}
impl Default for InputRecordReplayState {
    fn default() -> Self {
        Self {
            stream: InputStream::new(),
            mode: InputRecordReplayMode::AutoRecordPanicRepro,
            next_frame_index: 0,
            frame_applied: false,
            input_suppressed_this_frame: false,
            release_capture_requested: false,
            last_error: None,
        }
    }
}
impl InputRecordReplayState {
    pub fn current_frame(&self) -> Option<&InputFrame> {
        (self.mode == InputRecordReplayMode::Replay)
            .then(|| self.stream.get_frame(self.next_frame_index))
            .flatten()
    }
    pub fn next_frame_index(&self) -> usize {
        self.next_frame_index
    }
    pub fn start_recording(&mut self, snapshot: InputSnapshot) {
        self.stop();
        self.stream.reset(snapshot);
        self.mode = InputRecordReplayMode::Records;
        self.next_frame_index = 0;
        self.frame_applied = false;
        self.last_error = None;
    }
    pub fn stop(&mut self) {
        if self.mode == InputRecordReplayMode::Replay {
            self.release_capture_requested = true;
        }
        self.mode = InputRecordReplayMode::None;
        self.frame_applied = false;
    }
    fn release_capture_if_requested(&mut self, capture: &mut CursorCaptureState) {
        if std::mem::take(&mut self.release_capture_requested) {
            capture.restore_for_replay(None);
        }
    }
    /// 时钟失败时终止回放，并屏蔽本帧已经采集到的实时动作。
    pub(crate) fn fail_current_frame(&mut self, error: impl std::fmt::Display) {
        self.stop();
        self.release_capture_requested = true;
        self.input_suppressed_this_frame = true;
        let message = error.to_string();
        log::error!("输入帧已停止：{message}");
        self.last_error = Some(message);
    }
    /// 先检查回退目标，再修改模式和资源；从不使用减一回绕的哨兵索引。
    pub fn seek(
        &mut self,
        rewind: u64,
        toolbar: &mut Toolbar,
        app_mode: &mut AppMode,
        camera: &mut MainCamera,
    ) -> Result<()> {
        let point = self
            .stream
            .sync_points()
            .iter()
            .rev()
            .filter(|point| point.frame_index < self.stream.frame_count())
            .nth(usize::try_from(rewind)?)
            .context("没有可用的回放同步点")?;
        point.state.restore(toolbar, app_mode, camera)?;
        self.next_frame_index = point.frame_index;
        self.mode = InputRecordReplayMode::Replay;
        self.frame_applied = false;
        self.last_error = None;
        Ok(())
    }
    pub fn load(
        &mut self,
        path: &Path,
        rewind: u64,
        toolbar: &mut Toolbar,
        app_mode: &mut AppMode,
        camera: &mut MainCamera,
    ) -> Result<()> {
        let mut replacement = Self {
            stream: load_recording(path)?,
            ..Self::default()
        };
        replacement.seek(rewind, toolbar, app_mode, camera)?;
        *self = replacement;
        Ok(())
    }
}

/// Desktop schedule adapter for the retained replay/capture policy. Exposing
/// an exclusive world entry keeps CursorCapturePolicy private to this module's
/// implementation while allowing the actual tiny-glade host to begin a frame.
pub fn begin_input_frame(world: &mut World) {
    world
        .resource_mut::<InputRecordReplayState>()
        .input_suppressed_this_frame = false;
    *world.resource_mut::<CursorCapturePolicy>() = CursorCapturePolicy::Live;
}

pub fn control_input_replay(
    mut toolbar: ResMut<Toolbar>,
    mut app_mode: ResMut<AppMode>,
    mut camera: ResMut<MainCamera>,
    mut replay: ResMut<InputRecordReplayState>,
    mut commands: EventReader<InputReplayCommand>,
    mut capture: CursorCaptureState,
) {
    for command in commands.read() {
        let result = match command {
            InputReplayCommand::Sync { record } => {
                if matches!(
                    replay.mode,
                    InputRecordReplayMode::AutoRecordPanicRepro | InputRecordReplayMode::Records
                ) {
                    let snapshot = InputSnapshot::capture(&toolbar, *app_mode, &camera);
                    if !record && replay.mode == InputRecordReplayMode::AutoRecordPanicRepro {
                        replay.stream.reset(snapshot);
                    } else {
                        replay.stream.push_sync_point(snapshot);
                    }
                }
                Ok(())
            }
            InputReplayCommand::Load { path, rewind } => {
                replay.load(path, *rewind, &mut toolbar, &mut app_mode, &mut camera)
            }
            InputReplayCommand::Replay { rewind } => {
                replay.seek(*rewind, &mut toolbar, &mut app_mode, &mut camera)
            }
            InputReplayCommand::StartRecording => {
                replay.start_recording(InputSnapshot::capture(&toolbar, *app_mode, &camera));
                Ok(())
            }
            InputReplayCommand::Stop => {
                replay.stop();
                Ok(())
            }
        };
        if let Err(error) = result {
            let message = format!("{error:#}");
            log::error!("输入录制命令失败：{message}");
            replay.last_error = Some(message);
        }
    }
    replay.release_capture_if_requested(&mut capture);
}

/// 实时输入计算完、游戏消费者运行前执行：回放覆盖同一份动作资源。
/// 普通录制帧按值复制定长动作数组，不分配动作列表。
pub fn input_record_replay(
    time: Res<GameTime>,
    context: (Res<Toolbar>, Res<AppMode>, Res<MainCamera>),
    cursor_state: (ResMut<CursorPosition>, ResMut<CursorPositionSS>),
    mut capture: CursorCaptureState,
    mut actions: ResMut<ActionSetStates>,
    mut values: ResMut<ActionSetValues>,
    mut replay: ResMut<InputRecordReplayState>,
) {
    let (toolbar, app_mode, camera) = context;
    let (mut cursor, mut screen_cursor) = cursor_state;
    replay.frame_applied = false;
    replay.release_capture_if_requested(&mut capture);
    if replay.input_suppressed_this_frame {
        *actions = ActionSetStates::default();
        values.current = Default::default();
        return;
    }
    match replay.mode {
        InputRecordReplayMode::AutoRecordPanicRepro | InputRecordReplayMode::Records => {
            let frame = InputFrame {
                actions: *actions,
                cursor_position: [cursor.x, cursor.y],
                screen_cursor_position: screen_cursor.0,
                captured_cursor_position: capture.position(),
                time_delta_seconds: time.delta,
            };
            // 非法宿主时间不写入不可加载的录制；保留已录好的帧并报告错误。
            if let Err(error) = frame.validate() {
                replay.last_error = Some(error.to_string());
                return;
            }
            let retention = if replay.mode == InputRecordReplayMode::Records {
                RecordingRetention::All
            } else {
                RecordingRetention::Recent
            };
            replay.stream.record_frame(
                frame,
                || InputSnapshot::capture(&toolbar, *app_mode, &camera),
                retention,
            );
        }
        InputRecordReplayMode::Replay => {
            if let Some(frame) = replay.current_frame().copied() {
                *actions = frame.actions;
                values.current = frame.actions.values();
                cursor.x = frame.cursor_position[0];
                cursor.y = frame.cursor_position[1];
                screen_cursor.0 = frame.screen_cursor_position;
                capture.restore_for_replay(frame.captured_cursor_position);
                replay.frame_applied = true;
            } else {
                // 外部手动切换到空流也不能导致帧索引 panic 或泄漏上一帧按下沿。
                *actions = ActionSetStates::default();
                values.current = Default::default();
                replay.stop();
                replay.last_error = Some("回放已结束或没有输入帧".into());
            }
        }
        InputRecordReplayMode::DisableInput => {
            *actions = ActionSetStates::default();
            values.current = Default::default();
        }
        InputRecordReplayMode::None => {}
    }
    replay.release_capture_if_requested(&mut capture);
}

/// A completed frame is advanced once. The original backend set1 runs this
/// before the next frame's set5 writer; the standalone pipeline may instead
/// call it after consumers. frame_applied preserves both entry points.
pub fn advance_input_replay_frame(
    mut replay: ResMut<InputRecordReplayState>,
    mut capture: CursorCaptureState,
) {
    if replay.mode == InputRecordReplayMode::Replay && replay.frame_applied {
        replay.frame_applied = false;
        replay.next_frame_index += 1;
        if replay.next_frame_index >= replay.stream.frame_count() {
            replay.stop();
        }
    }
    replay.release_capture_if_requested(&mut capture);
}

//! `input_record_replay.rs` — 录制/回放状态机（B 0x1409423d0 / 0x140941a30 / 0x140942280）。
//!
//! 证据：`input/ALGORITHMS.md` §2.6、`ui-input/DIGEST.md` §2.3、反编译
//! `ui-input/decomp/decomp-0x1409423d0.txt` / `decomp-0x140941a30.txt` /
//! `decomp-0x140942280.txt`。
//!
//! 状态资源 `InputRecordReplayState` = 64 B `InputStream` + `mode`（niche u64 @+0x40）+
//! `index`（u64 @+0x48）（O：0x1409423d0 `param_9` 布局）。
//!
//! 帧布局（回放视图，O，re-analyst RE-ANALYSIS2 精确化）：`InputFrame` = 0xd0 B =
//! 两 Vec 头 @0x00/+0x18（空载 {0,4,0}）+ head 三元 @+0x30/+0x38/+0x3c + 游标 @+0x40 +
//! 归一除数 @+0x44 + **状态块 11 qword @+0x68..+0xb8** + 前次归一游标 @+0xc0 + 帧增量 @+0xc8。
//! **对账**：0x58..0xa8 是状态**回写目标**偏移（翻译状态资源 buffer），帧偏移实为
//! +0x68..0xb8（= 状态 +0x10）。serde 视图字段序 = [translation_state@+0x00, prev@+0xc0,
//! time@+0xc8, main_action_set_value@+0x68]。
//!
//! 同步点（0x60 B，O）：`+0x18` toolbar current、`+0x20/+0x28` ToolbarCategoryModes Vec、
//! `+0x30` camera、`+0x58` frame_index；seek = index=frame_index−1 + clone toolbar +
//! load_parameters + immediately_finish_interpolating（toolbar/camera 从状态块子字段恢复
//! 仍待类型化）。
//!
//! 常量（O，RNE 直读）：`_UNK_1429e95b0` = `0x4330000000000000`×2（u32→f64 精确转换魔数，
//! 非“原点”）；`_UNK_142a171f0` = `bf000000 bf000000 00000000 00000000`（`[-0.5,-0.5,0,0]`）；
//! `_UNK_1429258e0` = `3f000000 3f000000 …`（`[0.5,0.5]`）；`_UNK_1429a07d0` = `0x4f7fffff`。
//!
//! 事件 `InputReplayCommand`：`control_input_replay` 消费 0x30 B 实例
//! `{event_id u64 @0, payload 40 B @8}`；同步变体 payload 首字节 = record 标志
//! （O：`(char)plVar23[1]` 作 `on_history_sync` 的 record 参数）。变体名 U，本实现按
//! 已证实的两条消费路径建模。
use bevy_ecs::prelude::*;
use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    input::{
        input_stream::{InputStream, InputTranslationState},
        InputRecordReplayMode,
    },
    resources::{
        mouse_terrain_raycast::CursorPositionSS, toolbar::ToolbarCategoryModes, time::GameTime,
    },
    systems::camera_rig::main_camera::MainCamera,
};

use super::update_cursor_position::CursorPosition;

/// `_UNK_142a171f0`（O：RNE @0x142a171f0 = `bf000000 bf000000 00000000 00000000`）。
const NORMALIZE_MATRIX: [f32; 4] = [-0.5, -0.5, 0.0, 0.0];
/// `_UNK_1429258e0`（O：RNE @0x1429258e0 = `3f000000 3f000000 …`）。
const UNNORMALIZE_SCALE: Vec2 = Vec2::new(0.5, 0.5);
/// `_UNK_1429a07d0`（O：RNE = `0x4f7fffff`，u32::MAX 的 f32 舍入）。
const U32_MAX_AS_F32: f32 = 4_294_967_040.0;

const LOG_TARGET: &str = "country_core::systems::backend::input::input_record_replay";

/// 由外部（历史/模式系统）下发的录制/回放命令（`control_input_replay` 消费）。
///
/// O 结构：0x30 B 事件实例 `{event_id u64 @0, payload @8}`；同步变体 payload 首字节 = record
/// 标志（0x140941a30 `(char)plVar23[1]` 作为 `on_history_sync` 的 `record` 参数）。
#[derive(Event, Clone, Debug, PartialEq)]
pub enum InputReplayCommand {
    /// `InputStream::on_history_sync`：按给定翻译状态打同步点。
    Sync {
        state: InputTranslationState,
        /// true = 追加（录制中）；false = 先清空两队列再打点（复位）。
        record: bool,
    },
    /// `InputStream::try_load`：从路径加载录制文件。
    Load {
        path: std::path::PathBuf,
    },
}

/// 一帧录制输入（0xd0 B，回放视图，O）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputFrame {
    /// @+0x30：回放输出 qword。
    pub output_qword: u64,
    /// @+0x38：回放输出 word。
    pub output_word: u32,
    /// @+0x3c：回放输出 byte。
    pub output_byte: u8,
    /// @+0x40：游标位置（2×f32；回放 clamp 到 u32 游标）。
    pub cursor_pos: Vec2,
    /// @+0x44：归一除数（录制时 == `cursor_pos.y`，机器码把两者放在同一 8 B 槽）。
    pub divisor: f32,
    /// @+0x68..+0xb8：11 qword 翻译状态快照。
    pub state: InputTranslationState,
    /// @+0xc0：前次归一游标位置。
    pub prev_normalized_cursor_pos: Vec2,
    /// @+0xc8：帧增量秒。
    pub time_delta_seconds: f32,
}

impl Default for InputFrame {
    fn default() -> Self {
        Self {
            output_qword: 0,
            output_word: 0,
            output_byte: 0,
            cursor_pos: Vec2::ZERO,
            divisor: 0.0,
            state: InputTranslationState::default(),
            prev_normalized_cursor_pos: Vec2::ZERO,
            time_delta_seconds: 0.0,
        }
    }
}

/// `InputFrame` 的 serde 视图（O：RE-ANALYSIS2 字段序 = [translation_state@+0x00,
/// prev_normalized_cursor_pos@+0xc0, time_delta_seconds@+0xc8,
/// main_action_set_value@+0x68]；键名
/// `translation_state`/`prev_normalized_cursor_pos`/`time_delta_seconds`/
/// `main_action_set_value`）。与运行时帧布局是不同子结构（ALGORITHMS §6.2 裁决），
/// 序列化/反序列化用本视图。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputFrameSerde {
    /// 11 qword 翻译状态快照。
    pub translation_state: [u64; 11],
    /// 前次归一游标位置。
    pub prev_normalized_cursor_pos: [f32; 2],
    /// 帧增量秒。
    pub time_delta_seconds: f32,
    /// 打包动作集（`MainActionSet::deconstruct` 产物，`Vec<[u8;12]>`）。
    pub main_action_set_value: Vec<[u8; 12]>,
}

impl From<&InputFrame> for InputFrameSerde {
    fn from(frame: &InputFrame) -> Self {
        Self {
            translation_state: frame.state.0,
            prev_normalized_cursor_pos: [frame.prev_normalized_cursor_pos.x, frame.prev_normalized_cursor_pos.y],
            time_delta_seconds: frame.time_delta_seconds,
            // 运行时录制帧不携带动作集 Vec（ALGORITHMS §6.2 两视图差异）；serde 侧为空。
            main_action_set_value: Vec::new(),
        }
    }
}

impl From<InputFrameSerde> for InputFrame {
    fn from(serde_form: InputFrameSerde) -> Self {
        Self {
            output_qword: 0,
            output_word: 0,
            output_byte: 0,
            cursor_pos: Vec2::ZERO,
            divisor: 0.0,
            state: InputTranslationState(serde_form.translation_state),
            prev_normalized_cursor_pos: Vec2::new(
                serde_form.prev_normalized_cursor_pos[0],
                serde_form.prev_normalized_cursor_pos[1],
            ),
            time_delta_seconds: serde_form.time_delta_seconds,
        }
    }
}

/// 录制文件（O：serialize<Compact> 0x1402e0060 两键 `{replay_initial_state, first_frame}`）。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordingFile {
    /// `replay_initial_state`：10 字段初始态（见 [`InputReplayInitialState`]）。
    pub replay_initial_state: InputReplayInitialState,
    /// `first_frame`：首帧（serde 视图）。
    pub first_frame: InputFrameSerde,
}

/// `InputReplayInitialState`（O：serialize<Pretty> 键表 0x142c5b422..0x142c5b46d，
/// 长度 `0xb/0xc/0xd/0xd/0xb/0xf/0xf` = `window_size/cursor_depth/cursor_pos_ws/
/// center_pos_ws/ui_settings/sheep_positions/sheep_rotations`，尾 3 键
/// `app_mode/toolbar/camera`）。10 字段供 trailer 初始态恢复。
///
/// 字段名/序 O；各字段类型：window_size/cursor_pos_ws/center_pos_ws 为坐标（I：
/// window 空间 f32 对）、cursor_depth f32（I）、app_mode/toolbar/camera 复用 crate
/// 存档/工具栏类型（I：trailer 初始态与存档节共享 schema 候选）、ui_settings 与
/// sheep_positions/rotations 的具体载荷 U（诚实登记，透传 JSON 值）。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputReplayInitialState {
    pub window_size: [f32; 2],
    pub cursor_depth: f32,
    pub cursor_pos_ws: [f32; 2],
    pub center_pos_ws: [f32; 2],
    pub ui_settings: serde_json::Value,
    pub sheep_positions: Vec<[f32; 3]>,
    pub sheep_rotations: Vec<f32>,
    pub app_mode: crate::systems::mode_manager::AppMode,
    pub toolbar: crate::resources::toolbar::Toolbar,
    pub camera: crate::systems::camera_rig::CameraRigParametersCache,
}

impl Default for InputReplayInitialState {
    fn default() -> Self {
        Self {
            window_size: [0.0; 2],
            cursor_depth: 0.0,
            cursor_pos_ws: [0.0; 2],
            center_pos_ws: [0.0; 2],
            ui_settings: serde_json::Value::Null,
            sheep_positions: Vec::new(),
            sheep_rotations: Vec::new(),
            app_mode: crate::systems::mode_manager::AppMode::Editing(
                crate::systems::mode_manager::EditSubject::None,
            ),
            toolbar: crate::resources::toolbar::Toolbar::default(),
            camera: crate::systems::camera_rig::CameraRigParametersCache::default(),
        }
    }
}

/// `try_load` @0x1409499c0（O 结构）：读录制文件 → serde 反序列化 → 把首帧推入流，
/// 置 mode=Replay、index=-1。文件路径层与 save/session 组的格式约定待协调，本实现
/// 接受 JSON 路径。
pub fn try_load(path: &std::path::Path, stream: &mut InputStream<InputFrame>) -> Result<(), ()> {
    let text = std::fs::read_to_string(path).map_err(|_| ())?;
    let file: RecordingFile = serde_json::from_str(&text).map_err(|_| ())?;
    stream.add_frame(file.first_frame.into(), false);
    Ok(())
}

/// 回退到同步点（O：RE-ANALYSIS2 `index = frame_index − 1` + clone toolbar +
/// `load_parameters` + `immediately_finish_interpolating`）。
///
/// 同步点 0x60 B 子字段已类型化（O：0x140941a30）：
/// - `@+0x18` 字节 = ToolbarCategoryModes `current`（[`InputTranslationState::toolbar_current`]）；
/// - `@+0x20/+0x28` = 工具栏模式 Vec 头（clone 源 FUN_1412f7fa0，宿主持有分配）；
/// - `@+0x30..+0x58` = MainCamera 参数（`load_parameters` 源，40 B）；
/// - `@+0x58` = frame_index。
///
/// 本实现设置 index 并回写翻译状态 buffer（可测核心）；toolbar Vec 的 clone 与 camera 的
/// `load_parameters`/`immediately_finish_interpolating` 依赖宿主持有资源的真实分配，
/// 库内登记待接（偏移已定）。
pub fn seek_to_sync_point(
    replay: &mut InputRecordReplayState,
    translation: &mut InputTranslationStateResource,
    sync_index: usize,
) {
    let point = replay
        .stream
        .sync_points()
        .get(sync_index)
        .expect("No sync points found in replay state");
    if replay.mode != InputRecordReplayMode::Replay {
        panic!(
            "Can only seek to sync point if replaying but mode is {}",
            replay.mode
        );
    }
    replay.index = point.frame_index.wrapping_sub(1);
    translation.buffer = point.state;
    // 子字段恢复（O 偏移类型化）：toolbar_current() → Toolbar current；
    // toolbar_vec_cap_ptr() → 工具栏 Vec（clone FUN_1412f7fa0）；camera_params() →
    // MainCamera::load_parameters + immediately_finish_interpolating。宿主接线待接。
    let _ = (
        point.state.toolbar_current(),
        point.state.toolbar_vec_cap_ptr(),
        point.state.camera_params(),
    );
}

/// 录制/回放状态资源（O：0x1409423d0 `param_9` = `InputStream @0x00..0x40` + `mode @0x40`
/// + `index @0x48`）。
#[derive(Resource, Clone, Debug)]
pub struct InputRecordReplayState {
    /// 录制流（同步点 + 帧两队列）。
    pub stream: InputStream<InputFrame>,
    /// 模式（niche 存储，`0x8000000000000000 + 变体`）。
    pub mode: InputRecordReplayMode,
    /// 当前回放帧序（Replaying 时每帧 `+= 1`）。
    pub index: u64,
}

impl Default for InputRecordReplayState {
    fn default() -> Self {
        Self {
            stream: InputStream::new(),
            mode: InputRecordReplayMode::AutoRecordPanicRepro,
            index: 0,
        }
    }
}

/// 回放输出资源（O：`input_record_replay` `param_5`，帧 @+0x30/+0x38/+0x3c 与之互拷；
/// 字段语义名 U，按机器码偏移命名）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct InputReplayOutputState {
    /// @+0xe8：输出 qword。
    pub qword: u64,
    /// @+0xf0：输出 word。
    pub word: u32,
    /// @+0xf4：输出 byte。
    pub byte: u8,
}

/// 翻译状态资源（O：`input_record_replay` `param_8` 与 `reset_input_state` 同一资源）。
///
/// 机器码布局：`current @0x00..0x58`（11 qword，`reset_input_state` 的置换目标）+
/// `buffer @0x58..0xb0`（11 qword，录制/回放的状态块）。字节 @0xa3（工具/画布 busy 位）位于
/// buffer 内（O：0x1409423d0 default 分支 `bStack_105 = *(byte*)(lVar4 + 0xa3)`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct InputTranslationStateResource {
    /// @0x00..0x58：当前翻译状态。
    pub current: InputTranslationState,
    /// @0x58..0xb0：录制/回放缓冲状态块。
    pub buffer: InputTranslationState,
}

impl InputTranslationStateResource {
    /// O：0x1409423d0 `(bStack_105 & 1) != 0` 参与“开始录制”条件（工具/画布 busy 位 @+0xa3）。
    pub fn canvas_busy(&self) -> bool {
        self.buffer.0[9] & (1 << (3 * 8)) != 0
    }
}

/// 外部录制开关资源（O：0x1409423d0 default 分支 `*param_11 != 0`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ExternalRecordFlag(pub bool);

/// `f32→u32` clamp（O：0x1409423d0 case 2：NaN/负 → 0，> `0x4f7fffff` → u32::MAX）。
fn clamp_f32_to_u32(value: f32) -> u32 {
    if value.is_nan() || value < 0.0 {
        0
    } else if U32_MAX_AS_F32 < value {
        u32::MAX
    } else {
        value as u32
    }
}

/// `normalize_cursor_pos`（0x1409423d0 内联，行 223，O）：u32 游标对 → f32 对。
/// 原实现用 `0x4330000000000000 | bits` 减 2^52 把 u32 位型精确转 f64；`as f32` 与魔数路径
/// 数值一致（0..u32::MAX 可精确表示为 f64，转 f32 舍入相同）。
pub fn normalize_cursor_pos(cursor: (u32, u32)) -> Vec2 {
    Vec2::new(cursor.0 as f32, cursor.1 as f32)
}

/// `unnormalize_cursor_pos`（0x1409423d0 内联，行 228，O）：
/// `pos = (dx*0.5 + prev.x*dy, dy*0.5 + prev.y*dy)`（dx/dy = 当前游标增量，prev = 帧 @+0xc0）。
pub fn unnormalize_cursor_pos(delta: Vec2, prev: Vec2) -> Vec2 {
    Vec2::new(
        delta.x * UNNORMALIZE_SCALE.x + prev.x * delta.y,
        delta.y * UNNORMALIZE_SCALE.y + prev.y * delta.y,
    )
}

/// 把一帧的 11 qword 状态块解包回翻译状态资源 buffer（O：case 2 写 `+0x58..0xa8`）。
pub fn apply_frame_to_translation(frame: &InputFrame, translation: &mut InputTranslationStateResource) {
    translation.buffer = frame.state;
}

/// `input_record_replay` @0x1409423d0（系统主状态机，O）。
///
/// decomp 参数 `param_1..param_11` 对应（未实现的 `param_2` 打包动作集见下）：
/// - `time` = param_1（GameTime，`+0x28` delta → 帧 @+0xc8）；
/// - `toolbar` = param_3（ToolbarCategoryModes 克隆源，流重建）；
/// - `camera` = param_4（`MainCamera::store_parameters`，流重建）；
/// - `output` = param_5（回放输出资源 @+0xe8/+0xf0/+0xf4）；
/// - `cursor` = param_6（u32 游标位置）；
/// - `screen_cursor` = param_7（f32 屏幕游标 CursorPositionSS）；
/// - `translation` = param_8（翻译状态资源）；
/// - `replay` = param_9（流 + mode + index）；
/// - `commands` = param_10（Events<InputReplayCommand> 双缓冲）；
/// - `external_record` = param_11（外部录制开关）。
///
/// 未做（待取证）：`param_2` 的打包动作集来源与同步点初始态字段、`param_5` 字段语义名、
/// 流重建时同步点初始 `frame_index`。
#[allow(clippy::too_many_arguments)]
pub fn input_record_replay(
    time: Res<GameTime>,
    toolbar: Res<ToolbarCategoryModes>,
    camera: Res<MainCamera>,
    mut output: ResMut<InputReplayOutputState>,
    mut cursor: ResMut<CursorPosition>,
    mut screen_cursor: ResMut<CursorPositionSS>,
    mut translation: ResMut<InputTranslationStateResource>,
    mut replay: ResMut<InputRecordReplayState>,
    commands: EventReader<InputReplayCommand>,
    external_record: Res<ExternalRecordFlag>,
) {
    match replay.mode {
        InputRecordReplayMode::AutoRecordPanicRepro | InputRecordReplayMode::Records => {
            let wants_record = !commands.is_empty() || external_record.0 || translation.canvas_busy();
            if wants_record && replay.mode == InputRecordReplayMode::AutoRecordPanicRepro {
                // 重建流：克隆工具栏 + 相机参数入栈 + 1 同步点 + 帧容量 8（O：default 分支
                // FUN_1412f7fa0 / FUN_140911710 / FUN_141301640；`frames.cap = 8`）。
                let _camera = camera.store_parameters();
                let _toolbar_snapshot = toolbar.clone();
                replay.stream = InputStream::new();
            }
            // 帧构造：输出资源 → @+0x30..0x3c；游标 → @0x40/@0x44；
            // 状态块 @+0x68..0xb8 = 翻译状态 buffer；@+0xc0 = 归一增量；@+0xc8 = delta。
            let frame = InputFrame {
                output_qword: output.qword,
                output_word: output.word,
                output_byte: output.byte,
                cursor_pos: normalize_cursor_pos((cursor.x, cursor.y)),
                divisor: cursor.y as f32,
                state: translation.buffer,
                prev_normalized_cursor_pos: normalize_cursor_delta(
                    normalize_cursor_pos((cursor.x, cursor.y)),
                    screen_cursor.0,
                ),
                time_delta_seconds: time.delta,
            };
            // record = 仅 AutoRecordPanicRepro 裁剪（O：`mode != Records && mode < Finished`）。
            let record = replay.mode == InputRecordReplayMode::AutoRecordPanicRepro;
            replay.stream.add_frame(frame, record);
        }
        InputRecordReplayMode::Replay => {
            // case 2：越界 panic "Out of bounds access"（O：`head@+0x38 <= index`）。
            if replay.index >= replay.stream.frame_count() as u64 {
                panic!("Out of bounds access");
            }
            let frame = *replay.stream.frame(replay.index as usize);
            // 帧 → 翻译状态资源 + 回放输出资源 + u32 游标。
            translation.buffer = frame.state;
            output.qword = frame.output_qword;
            output.word = frame.output_word;
            output.byte = frame.output_byte;
            let (x, y) = (clamp_f32_to_u32(frame.cursor_pos.x), clamp_f32_to_u32(frame.cursor_pos.y));
            cursor.x = x;
            cursor.y = y;
            // 屏幕游标 = 未归一化（O：`pos = delta*scale + prev*dy`，delta = 当前游标）。
            screen_cursor.0 = unnormalize_cursor_pos(
                normalize_cursor_pos((x, y)),
                frame.prev_normalized_cursor_pos,
            );
        }
        InputRecordReplayMode::DisableInput => {
            // case 3：清零 20 qword（0x00..0xa0，翻译状态复位，O）。
            *translation = InputTranslationStateResource::default();
        }
        InputRecordReplayMode::None => {}
    }
}

/// 录制时的归一增量（O：0x1409423d0 default 分支 `divps`：`{-0.5x+cx, -0.5y+cy} / y`）。
pub fn normalize_cursor_delta(cursor: Vec2, screen: Vec2) -> Vec2 {
    if cursor.y == 0.0 {
        return Vec2::ZERO;
    }
    Vec2::new(
        (NORMALIZE_MATRIX[0] * cursor.x + screen.x) / cursor.y,
        (NORMALIZE_MATRIX[1] * cursor.y + screen.y) / cursor.y,
    )
}

/// `advance_input_replay_frame` @0x140942280（O）：Replaying 时 `index += 1`；帧尽 → mode =
/// Finished（`0x8000000000000004`），verbosity>2 打日志。
pub fn advance_input_replay_frame(mut replay: ResMut<InputRecordReplayState>) {
    if replay.mode != InputRecordReplayMode::Replay {
        return;
    }
    replay.index = replay.index.wrapping_add(1);
    if replay.stream.frame_count() as u64 <= replay.index {
        log::info!(target: LOG_TARGET, "Finished input replay");
        replay.mode = InputRecordReplayMode::None;
    }
}

/// `control_input_replay` @0x140941a30（O）：消费 `Events<InputReplayCommand>`，执行
/// `on_history_sync` / `try_load` / seek。
///
/// decomp 参数：param_1/2/3 = 三处 ResMut（tick 回写）、param_4 = ResMut<流>、param_5 =
/// Events（元素 stride 0x30）。`try_load`（0x1409499c0）依赖录制文件 serde 形态（待取证），
/// 本实现登记为日志错误。
pub fn control_input_replay(
    toolbar: Res<ToolbarCategoryModes>,
    _output: ResMut<InputReplayOutputState>,
    camera: Res<MainCamera>,
    mut replay: ResMut<InputRecordReplayState>,
    mut commands: EventReader<InputReplayCommand>,
) {
    for command in commands.read() {
        match command {
            InputReplayCommand::Sync { state, record } => {
                // O：`FUN_1409494c0(stream, &state, record)`；record=false 时先清空两队列。
                replay.stream.on_history_sync(*state, *record);
                let _ = (camera.store_parameters(), toolbar.clone());
            }
            InputReplayCommand::Load { path } => {
                log::info!(target: LOG_TARGET, "Starting input replay");
                // try_load（0x1409499c0）：读文件 + serde 反序列化 → mode=Replay、index=-1。
                match try_load(path, &mut replay.stream) {
                    Ok(()) => {
                        replay.mode = InputRecordReplayMode::Replay;
                        replay.index = u64::MAX;
                    }
                    Err(()) => {
                        log::error!(
                            target: LOG_TARGET,
                            "Failed to load an input recording from {}",
                            path.display()
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_frame_serde_round_trips_the_serde_view() {
        let frame = InputFrame {
            output_qword: 1,
            output_word: 2,
            output_byte: 3,
            cursor_pos: Vec2::new(4.0, 5.0),
            divisor: 5.0,
            state: InputTranslationState([7; 11]),
            prev_normalized_cursor_pos: Vec2::new(0.25, -0.5),
            time_delta_seconds: 0.016,
        };
        let serde_form = InputFrameSerde::from(&frame);
        let json = serde_json::to_string(&serde_form).unwrap();
        assert!(json.contains("translation_state"));
        assert!(json.contains("prev_normalized_cursor_pos"));
        assert!(json.contains("time_delta_seconds"));
        assert!(json.contains("main_action_set_value"));
        let back: InputFrameSerde = serde_json::from_str(&json).unwrap();
        let restored: InputFrame = back.into();
        assert_eq!(restored.state, frame.state);
        assert_eq!(restored.prev_normalized_cursor_pos, frame.prev_normalized_cursor_pos);
        assert_eq!(restored.time_delta_seconds, frame.time_delta_seconds);
    }

    #[test]
    fn try_load_reads_a_recording_file_into_the_stream() {
        let frame = InputFrameSerde {
            translation_state: [1; 11],
            prev_normalized_cursor_pos: [0.0, 0.0],
            time_delta_seconds: 0.016,
            main_action_set_value: Vec::new(),
        };
        let file = RecordingFile {
            replay_initial_state: InputReplayInitialState::default(),
            first_frame: frame,
        };
        let json = serde_json::to_string(&file).unwrap();
        let path = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(path.path(), json).unwrap();
        let mut stream = InputStream::<InputFrame>::new();
        assert!(try_load(path.path(), &mut stream).is_ok());
        assert_eq!(stream.frame_count(), 1);
        assert_eq!(stream.frame(0).state, InputTranslationState([1; 11]));
        // 不存在路径 → Err。
        assert!(try_load(std::path::Path::new("/nonexistent/recording.json"), &mut stream).is_err());
    }

    #[test]
    fn seek_to_sync_point_requires_replay_mode_and_rewinds() {
        let mut stream = InputStream::<InputFrame>::new();
        stream.on_history_sync(InputTranslationState([9; 11]), true);
        stream.add_frame(InputFrame::default(), false);
        stream.add_frame(InputFrame::default(), false);
        let mut replay = InputRecordReplayState {
            stream,
            mode: InputRecordReplayMode::Replay,
            index: 5,
        };
        let mut translation = InputTranslationStateResource::default();
        seek_to_sync_point(&mut replay, &mut translation, 0);
        // 首同步点 frame_index = 0（在 add_frame 前打的点）→ index = 0-1 回绕为 u64::MAX，
        // buffer 回写同步点状态。
        assert_eq!(translation.buffer, InputTranslationState([9; 11]));
        let _ = replay.index;
    }
}

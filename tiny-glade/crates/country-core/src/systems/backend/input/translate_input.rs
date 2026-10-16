//! `translate_input.rs` — 输入映射主系统（B 0x1417cd0a0，O）。
//!
//! 证据：`chapters/systems/backend/input/translate_input.rs.md`、`ui-input/DIGEST.md` §2.3、
//! 反编译 `ui-input/decomp/decomp-0x1417cd0a0.txt`、PDB 签名
//! `structures.json.gz`（translate_input.rs）。
//!
//! PDB 签名（O）：`(Res<GameTime>, Res<ButtonInput<KeyCode>>, Res<ButtonInput<MouseButton>>,
//! Option<Res<SteamClientResource>>, Res<EscMenuState>, ResMut<ActionSetValues>,
//! ResMut<ActiveInputModeTracker>, EventReader<MouseWheel>, Local<MouseMoved>,
//! EventReader<InputCursorDelta>, EventReader<UiHistoryButtonPressed>, Res<GameProfile>)`。
//!
//! 行为（O，0x1417cd0a0）：
//! 1. Steam 门（`Option<Res<SteamClientResource>>`）：有且活跃才走 Steam 输入路径（crate 无
//!    `systems::steam` 模块，本实现以 [`SteamInputActive`] 布尔资源登记该门，待取证）。
//! 2. 鼠标按钮 → digital 槽 0 `click`（`*(byte*)(values + 0x88) |= 按下位`）。
//! 3. `EscMenuState` 门：tag == 2 或（tag == 10 且 flag != 1）才处理键盘映射。
//! 4. 键盘数字/模拟映射：`GameTime.delta * 30.0`（`_UNK_1429cd034` = 30.0，O：RNE 直读）
//!    为模拟增量；键码取自 `game_key_bindings`（crate 尚无该模块，本实现以
//!    [`BindingsTable`] 建模，默认序列来自 reconstruction 证据，逐槽映射待取证）。
//! 5. `EventReader<InputCursorDelta>` → `Local<MouseMoved>` 累加 → analog 槽 0
//!    `input_space_cursor_move`。
//!
//! 未做（待取证）：SteamClientResource/GameProfile 真实类型与字段、逐动作键码绑定
//! （`game_key_bindings` 模块）、MouseWheel 消费、UiHistoryButtonPressed 语义。
use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::{
    input::{
        action_set::{ActionSetValues, ANALOG_SLOTS, DIGITAL_SLOTS},
        ActiveInputModeTracker,
    },
    resources::{gui::EscMenuState, time::GameTime},
};

use super::{
    button_input::{KeyButtonInput, MouseButtonInput, UNBOUND_KEY},
    calculate_input_delta::UiHistoryButtonPressed,
    translate_input_to_cursor_moved::InputCursorDelta,
};

/// `MouseMoved`（PDB 命名实体，`Local<MouseMoved>`）：游标增量帧间累加器。
#[derive(Clone, Copy, Debug, Default)]
pub struct MouseMoved(pub Vec2);

/// Steam 输入活跃门（O：RE-ANALYSIS2——translate_input 内 Arc 克隆 `SteamClientManager` →
/// `SteamAPI_SteamUtils_v010`（0x142923570）→ `IsSteamRunningOnSteamDeck`
/// （0x1429234f0）门控事件源）。`systems::steam` 不在本 crate（库外接线登记，controllers →
/// InputTranslationState 实际在 `systems::steam::update_steam_inputs`，22 同构块），
/// 本资源由宿主按该门填充；false = 走键盘/鼠标路径。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SteamInputActive(pub bool);

/// 一个动作的键/鼠绑定（I：`game_key_bindings` 的 `KeyBinding {key, mouse}`，u16 槽码，
/// `0xc3` = 未绑定）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionBinding {
    pub key: Option<u16>,
    pub mouse: Option<u16>,
}

/// 绑定表（模拟槽 0..6、数字槽 0..40，槽号 = `game_key_bindings` 动作槽）。
#[derive(Clone, Debug)]
pub struct BindingsTable {
    pub analog: [ActionBinding; ANALOG_SLOTS],
    pub digital: [ActionBinding; DIGITAL_SLOTS],
}

impl Default for BindingsTable {
    fn default() -> Self {
        // O（re-analyst 定案，RE-ANALYSIS2）：KeyBindings = 40 动作槽 × 12 B
        // `{key u32@0, value u64@4}`，`0xc3` = 未绑定哨兵；translate_input 逐槽读满 40 槽
        // （i32 索引步 3，0x00..0x6f + 尾 0x70/0x71 u16 = 鼠标区）。键码序列来自
        // `load_game_profile` 默认构建（O）：`{0x07..0x0f, 0x23, 0x27, 0x2d, 0x2b, 0xc3,
        // 0x38, 0xab, 0xa5, 0x1b, 0x49, 0x3a, 0x51, 0x52, 0x50, 0x53, 0x4f, 0x4e}` +
        // 鼠标 `{1,2,1}`。槽位与 46 动作的精确对应仍待取证（动作↔槽映射）。
        const KEYS: [u16; 26] = [
            0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x23, 0x27, 0x2d, 0x2b,
            0xc3, 0x38, 0xab, 0xa5, 0x1b, 0x49, 0x3a, 0x51, 0x52, 0x50, 0x53, 0x4f, 0x4e,
        ];
        const MOUSE: [u16; 3] = [1, 2, 1];
        let mut bindings = [ActionBinding { key: None, mouse: None }; DIGITAL_SLOTS];
        for (i, code) in KEYS.iter().enumerate() {
            bindings[i].key = (*code != UNBOUND_KEY).then_some(*code);
        }
        for (i, button) in MOUSE.iter().enumerate() {
            bindings[i].mouse = Some(*button);
        }
        Self {
            analog: [ActionBinding { key: None, mouse: None }; ANALOG_SLOTS],
            digital: bindings,
        }
    }
}

/// `FUN_14089aef0`（O：按键查询谓词；返回键是否按下）。
fn key_pressed(keyboard: &KeyButtonInput, binding: &ActionBinding) -> bool {
    binding.key.is_some_and(|code| keyboard.pressed(code))
}

/// 数字槽翻译（O 模式）：键按下 → `just_pressed`（disc=0）；未按下 → 保持空闲。
fn translate_digital(keyboard: &KeyButtonInput, bindings: &BindingsTable, out: &mut [bool; DIGITAL_SLOTS]) {
    for (slot, binding) in bindings.digital.iter().enumerate() {
        if key_pressed(keyboard, binding) {
            out[slot] = true;
        }
    }
}

/// 模拟槽翻译（O：0x1417cd0a0 重复段）：键按下时按方向累加 `delta * 30.0`。
/// 槽位方向表（I，对应动作名 camera_pan/camera_height/camera_zoom/camera_rotate 的键驱动）。
fn translate_analog(
    keyboard: &KeyButtonInput,
    bindings: &BindingsTable,
    dt: f32,
    out: &mut [Vec2; ANALOG_SLOTS],
) {
    // `_UNK_1429cd034` = 30.0（O：RNE @0x1429cd034 = `0000f041`）。
    const KEY_SPEED: f32 = 30.0;
    let step = dt * KEY_SPEED;
    for (slot, binding) in bindings.analog.iter().enumerate() {
        if key_pressed(keyboard, binding) {
            // 单键绑定按默认方向驱动；精确方向表待取证。
            out[slot].y -= step;
        }
    }
}

/// `translate_input` @0x1417cd0a0（O 结构）。
///
/// 相对 PDB 签名缺 `Option<Res<SteamClientResource>>`、`EventReader<MouseWheel>`、
/// `Res<GameProfile>`：前两者用 [`SteamInputActive`] / 键鼠资源代替（待取证），GameProfile
/// 的键绑定用 [`BindingsTable`] 传入（默认序列来自证据）。语义等价：Steam inactive 时走
/// 键盘/鼠标路径。
#[allow(clippy::too_many_arguments)]
pub fn translate_input(
    time: Res<GameTime>,
    keyboard: Res<KeyButtonInput>,
    mouse: Res<MouseButtonInput>,
    menu: Res<EscMenuState>,
    mut values: ResMut<ActionSetValues>,
    mut mode_tracker: ResMut<ActiveInputModeTracker>,
    steam: Res<SteamInputActive>,
    mut mouse_moved: Local<MouseMoved>,
    mut cursor_delta: EventReader<InputCursorDelta>,
    mut _history_buttons: EventReader<UiHistoryButtonPressed>,
) {
    // Steam 门（O：`*param_4 != 0` 时走 Steam；本实现只登记 inactive 分支）。
    if steam.0 {
        return;
    }
    let now = core::time::Duration::from_millis(0);

    // 鼠标左键 → digital 槽 0 `click`（O：`values + 0x88 |= 按下位`）。
    if mouse.just_pressed(0) {
        values.current.digital[0] = true;
    }

    // EscMenuState 门（O：`tag == 2 || (tag == 10 && flag != 1)`）。
    let view = menu.view();
    let gate_open = view.tag == 2 || (view.tag == 10 && view.flag != 1);
    let bindings = BindingsTable::default();
    if gate_open {
        let dt = time.delta;
        translate_digital(&keyboard, &bindings, &mut values.current.digital);
        translate_analog(&keyboard, &bindings, dt, &mut values.current.analog);

        // `EventReader<InputCursorDelta>` → `Local<MouseMoved>` 累加 → analog 槽 0
        // `input_space_cursor_move`（O：MouseMoved 事件消费进翻译状态）。
        let accumulated = mouse_moved.0 + cursor_delta.read().fold(Vec2::ZERO, |acc, d| acc + d.0);
        values.current.analog[0] += accumulated;
        mouse_moved.0 = Vec2::ZERO;

        // 键鼠使用 → 输入方式切到 MouseKeyboard（O：ActiveInputModeTracker 更新）。
        *mode_tracker = ActiveInputModeTracker::new_default_mouse_keyboard(now);
    }
}

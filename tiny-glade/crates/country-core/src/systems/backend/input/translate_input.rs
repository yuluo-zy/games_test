//! `translate_input.rs` — 输入映射主系统（B 0x1417cd0a0，O 全解）。
//!
//! 证据：`chapters/systems/backend/input/translate_input.rs.md`、`ui-input/DIGEST.md` §2.3、
//! `utils-input-config/DIGEST.md` §2.3、反编译 `ui-input/decomp/decomp-0x1417cd0a0.txt`、
//! `input/ALGORITHMS.md` §1.2、re-analyst-interaction RE-ANALYSIS2 §1。
//!
//! PDB 签名（O）：`(Res<GameTime>, Res<ButtonInput<KeyCode>>, Res<ButtonInput<MouseButton>>,
//! Option<Res<SteamClientResource>>, Res<EscMenuState>, ResMut<ActionSetValues>,
//! ResMut<ActiveInputModeTracker>, EventReader<MouseWheel>, Local<MouseMoved>,
//! EventReader<InputCursorDelta>, EventReader<UiHistoryButtonPressed>, Res<GameProfile>)`。
//!
//! 行为（O，0x1417cd0a0）：
//! 1. Steam 门（`Option<Res<SteamClientResource>>`）：有且活跃才走 Steam 输入路径（crate 无
//!    `systems::steam` 模块，本实现以 [`SteamInputActive`] 布尔资源登记该门）。非 Deck 时
//!    检查键盘/鼠标事件源挂起计数决定是否进入翻译。
//! 2. 鼠标按下事件扫描 → digital 槽 0 `click`（`*(byte*)(values + 0x88) |= 按下位`）。
//! 3. `EscMenuState` 门：tag == 2 或（tag == 10 且 flag != 1）才处理键盘/滚轮映射。
//! 4. 键盘绑定表翻译：**40 槽 × 12 B `{key u32@0, value u64@4}`**（`0xc3` = 未绑定）。
//!    模拟槽 0..9 → 相机 pan/height/rotate/zoom；数字槽 10..38 → 26 个数字动作
//!    （含槽 37/38 鼠标绑定 → camera_rotate_mode/camera_pan_mode）。
//! 5. `EventReader<MouseWheel>` → analog 槽 4 `camera_zoom`（权重 8.0/1.0）。
//! 6. `EventReader<InputCursorDelta>` + `Local<MouseMoved>` → 右键阈值判断（`3.0`）。
//! 7. 硬编码键（ui_cancel=0x73、screenshot 修饰 0x3d、demo 门 0x143634cd8 内 8 键）。
//! 8. 键鼠使用 → 输入方式切到 MouseKeyboard（`ActiveInputModeTracker`）。
//!
//! 动作↔槽映射（O：ron serialize 字段名表 @0x140831740 与反编译逐槽读序互证）：
//! 槽 0..3 move_forward/back/left/right → analog[2] camera_pan；槽 4..5 move_up/down →
//! analog[3] camera_height；槽 6..7 rotate_left/right → analog[5] camera_rotate（±1800/s，
//! 乘原始 GameTime delta）；槽 8..9 zoom_in/out → analog[4] camera_zoom（±1.5 × dt×30）。
//! 数字槽 10..38：10 building→tool_building、11 path→tool_path、12 decorators→tool_decorator、
//! 13 customize→tool_customize、14 eraser→tool_eraser、15 terrain→tool_terrain、
//! 16 garden→tool_garden、17 tree→tool_tree、18 water→tool_water、19 photomode→photomode、
//! 20 time_of_day→time_of_day、21 undo→undo、22 redo→redo、23 focus_edit→right_click、
//! 24 disable_snapping→snapping_disable、25 screenshot→screenshot（& !0x3d 修饰）、
//! 26 feedback→screenshot_feedback、27 hide_ui_photomode→hide_ui_photomode、
//! 28 ui_cancel→ui_cancel（| 硬编码 0x73）、29 ui_confirm→ui_confirm、30..35
//! ui_left/right/down/up/tab_prev/tab_next、36 → cinematic_camera_ctrls、37/38 鼠标
//! {1,2} → camera_rotate_mode/camera_pan_mode、39 鼠标 {1} → focus_edit（右键手势）。
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

/// `Local<MouseMoved>`（PDB 命名实体）：游标增量帧间累加器（O：`param_9`，右键手势阈值判断）。
#[derive(Clone, Copy, Debug, Default)]
pub struct MouseMoved(pub f32);

/// Steam 输入活跃门（O：RE-ANALYSIS2——translate_input 内 Arc 克隆 `SteamClientManager` →
/// `SteamAPI_SteamUtils_v010`（0x142923570）→ `IsSteamRunningOnSteamDeck`
/// （0x1429234f0）门控事件源）。`systems::steam` 不在本 crate（库外接线登记，controllers →
/// InputTranslationState 实际在 `systems::steam::update_steam_inputs`，22 同构块），
/// 本资源由宿主按该门填充；false = 走键盘/鼠标路径。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SteamInputActive(pub bool);

/// Demo 门（O：全局字节 `0x143634cd8` = demo 开关；为 1 时额外启用演示模式的硬编码键）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DemoModeEnabled(pub bool);

/// 绑定表槽数（O：40 槽 × 12 B）。
pub const BINDING_SLOTS: usize = 40;

/// 一个动作的键/鼠绑定（O：`KeyBinding {key u32@0, value u64@4}`；`0xc3` = 未绑定；
/// 鼠标绑定为 u16 对 @0x1bc..0x1c8）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionBinding {
    pub key: u16,
    pub mouse: u16,
}

impl ActionBinding {
    pub const UNBOUND: Self = Self {
        key: UNBOUND_KEY,
        mouse: 0,
    };

    pub fn bound_key(&self) -> bool {
        self.key != UNBOUND_KEY
    }
}

/// 绑定表（槽号 = `game_key_bindings` 动作槽，0..39）。
#[derive(Clone, Debug)]
pub struct BindingsTable {
    pub slots: [ActionBinding; BINDING_SLOTS],
}

impl Default for BindingsTable {
    fn default() -> Self {
        // O（utils-input-config DIGEST §2.3，default @0x14089b110 原字节）：36 u32 键码 +
        // `[0x1b0]=0xc3` + u16 鼠标 `{1,2,1}`。键码为 bevy 风格 KeyCode u16。
        const KEYS: [u16; 36] = [
            0x2a, 0x26, 0x14, 0x17, 0x18, 0x24, 0xc3, 0xc3, 0xc3, 0xc3, // 0..9 模拟
            0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x23, // 10..19
            0x27, 0x2d, 0x2b, 0xc3, 0x38, 0xab, 0xa5, 0x1b, 0x49, 0x3a, // 20..29
            0x51, 0x52, 0x50, 0x53, 0x4f, 0x4e, // 30..35
        ];
        const MOUSE: [u16; 3] = [1, 2, 1];
        let mut slots = [ActionBinding::UNBOUND; BINDING_SLOTS];
        for (i, code) in KEYS.iter().enumerate() {
            slots[i].key = *code;
        }
        // 槽 36（cinematic_camera_ctrls）默认 0xc3；槽 37/38/39 = 鼠标绑定。
        for (i, button) in MOUSE.iter().enumerate() {
            slots[37 + i].mouse = *button;
        }
        Self { slots }
    }
}

/// 模拟槽翻译目标（O：0x1417cd0a0 逐槽写 analog）。
struct AnalogTarget {
    /// analog 槽号（0..6；camera_pan=2、camera_height=3、camera_zoom=4、camera_rotate=5）。
    slot: usize,
    /// 分量（0=x、1=y）。
    axis: usize,
    /// 方向（+1/−1）。
    sign: f32,
    /// 增量标量。
    scalar: f32,
    /// true = 乘原始 GameTime delta；false = 乘 `delta * 30.0`。
    use_raw_delta: bool,
}

/// 绑定槽 → 模拟目标（O：0x1417cd0a0 行 186..281 逐槽写；标量 RNE 直读：
/// `_UNK_1429cd034`=30.0、`_UNK_142ce2d30`=1800、`_UNK_142ce2d34`=−1800、
/// `_UNK_1429258d0`=1.5、`_UNK_1429df180`=−1.5）。
const ANALOG_TARGETS: [AnalogTarget; 10] = [
    // 槽 0..3 move_forward/back/left/right → camera_pan（analog 2）。
    AnalogTarget { slot: 2, axis: 1, sign: -1.0, scalar: 1.0, use_raw_delta: false },
    AnalogTarget { slot: 2, axis: 1, sign: 1.0, scalar: 1.0, use_raw_delta: false },
    AnalogTarget { slot: 2, axis: 0, sign: -1.0, scalar: 1.0, use_raw_delta: false },
    AnalogTarget { slot: 2, axis: 0, sign: 1.0, scalar: 1.0, use_raw_delta: false },
    // 槽 4..5 move_up/down → camera_height（analog 3）。
    AnalogTarget { slot: 3, axis: 0, sign: 1.0, scalar: 1.0, use_raw_delta: false },
    AnalogTarget { slot: 3, axis: 0, sign: -1.0, scalar: 1.0, use_raw_delta: false },
    // 槽 6..7 rotate_left/right → camera_rotate（analog 5），±1800·原始 delta。
    AnalogTarget { slot: 5, axis: 0, sign: -1.0, scalar: 1800.0, use_raw_delta: true },
    AnalogTarget { slot: 5, axis: 0, sign: 1.0, scalar: 1800.0, use_raw_delta: true },
    // 槽 8..9 zoom_in/out → camera_zoom（analog 4），±1.5·(delta·30)。
    AnalogTarget { slot: 4, axis: 0, sign: 1.0, scalar: 1.5, use_raw_delta: false },
    AnalogTarget { slot: 4, axis: 0, sign: -1.0, scalar: 1.5, use_raw_delta: false },
];

/// 绑定槽 → 数字动作槽（O：0x1417cd0a0 行 476..636 逐槽写）。
/// 前两个为 None 表示该槽映射到鼠标手势/修饰键，单独处理。
const DIGITAL_TARGETS: [(usize, u16); 29] = [
    (10, 4), // building → tool_building
    (11, 5), // path → tool_path
    (12, 6), // decorators → tool_decorator
    (13, 7), // customize → tool_customize
    (14, 9), // eraser → tool_eraser
    (15, 8), // terrain → tool_terrain
    (16, 10), // garden → tool_garden
    (17, 11), // tree → tool_tree
    (18, 12), // water → tool_water
    (19, 13), // photomode
    (20, 14), // time_of_day
    (21, 16), // undo
    (22, 17), // redo
    (23, 1), // focus_edit → right_click
    (24, 15), // disable_snapping → snapping_disable
    (25, 29), // screenshot（+ !0x3d 修饰）
    (26, 31), // feedback → screenshot_feedback
    (27, 32), // hide_ui_photomode
    (28, 18), // ui_cancel（| 硬编码 0x73）
    (29, 19), // ui_confirm
    (30, 20), // ui_left
    (31, 21), // ui_right
    (32, 22), // ui_down
    (33, 23), // ui_up
    (34, 24), // ui_tab_prev
    (35, 25), // ui_tab_next
    (36, 33), // cinematic_camera_ctrls
    (37, 2), // 鼠标 {1} → camera_rotate_mode
    (38, 3), // 鼠标 {2} → camera_pan_mode
];

/// `FUN_14089aef0`（O：按键查询谓词；返回键是否按下）。
fn key_pressed(keyboard: &KeyButtonInput, code: u16) -> bool {
    keyboard.pressed(code)
}

/// 模拟槽翻译（O：0x1417cd0a0 行 186..281）：键按下时按目标槽/分量/方向累加。
fn translate_analog(
    keyboard: &KeyButtonInput,
    bindings: &BindingsTable,
    delta: f32,
    out: &mut [Vec2; ANALOG_SLOTS],
) {
    // `_UNK_1429cd034` = 30.0（O：RNE @0x1429cd034 = `0000f041`）。
    const KEY_SPEED: f32 = 30.0;
    let step = delta * KEY_SPEED;
    for (slot, target) in ANALOG_TARGETS.iter().enumerate() {
        let binding = &bindings.slots[slot];
        if binding.bound_key() && key_pressed(keyboard, binding.key) {
            let amount = if target.use_raw_delta {
                delta * target.scalar * target.sign
            } else {
                step * target.scalar * target.sign
            };
            if target.axis == 0 {
                out[target.slot].x += amount;
            } else {
                out[target.slot].y += amount;
            }
        }
    }
}

/// 数字槽翻译（O：0x1417cd0a0 行 476..636）：键按下 → 置数字槽真。
fn translate_digital(
    keyboard: &KeyButtonInput,
    mouse: &MouseButtonInput,
    bindings: &BindingsTable,
    out: &mut [bool; DIGITAL_SLOTS],
) {
    for (binding_slot, digital_slot) in DIGITAL_TARGETS {
        let binding = &bindings.slots[binding_slot as usize];
        let pressed = if binding_slot >= 37 {
            // 鼠标绑定槽（37=Rotate、38=Move）。
            binding.mouse != 0 && mouse.pressed(binding.mouse)
        } else {
            binding.bound_key() && key_pressed(keyboard, binding.key)
        };
        if pressed {
            out[digital_slot as usize] = true;
        }
    }
    // 硬编码 ui_cancel 键 0x73（O：行 338..341，`FUN_14089aef0(lVar28 + 0x20, 0x73)`）。
    if key_pressed(keyboard, 0x73) {
        out[18] = true;
    }
}

/// `translate_input` @0x1417cd0a0（O 全解）。
///
/// 相对 PDB 签名缺 `Option<Res<SteamClientResource>>`、`EventReader<MouseWheel>`、
/// `Res<GameProfile>`：前两者用 [`SteamInputActive`] / 滚轮读取代替（Steam 全链库外登记），
/// GameProfile 的键绑定用 [`BindingsTable`] 传入（默认序列来自 O 原字节）。
#[allow(clippy::too_many_arguments)]
pub fn translate_input(
    time: Res<GameTime>,
    keyboard: Res<KeyButtonInput>,
    mouse: Res<MouseButtonInput>,
    menu: Res<EscMenuState>,
    mut values: ResMut<ActionSetValues>,
    mut mode_tracker: ResMut<ActiveInputModeTracker>,
    steam: Res<SteamInputActive>,
    demo: Res<DemoModeEnabled>,
    mut mouse_moved: Local<MouseMoved>,
    mut wheel: EventReader<MouseWheel>,
    mut cursor_delta: EventReader<InputCursorDelta>,
    mut _history_buttons: EventReader<UiHistoryButtonPressed>,
) {
    // Steam 门（O：`*param_4 != 0` 时走 Steam；本实现只登记 inactive 分支）。
    if steam.0 {
        return;
    }
    let now = core::time::Duration::from_millis(0);

    // 鼠标按下事件扫描 → digital 槽 0 `click`（O：行 96..180 SIMD 扫描 `param_3`
    // `ButtonInput<MouseButton>` 按下事件位，任一按下沿即置 click）。
    if mouse.just_pressed_any() {
        values.current.digital[0] = true;
    }

    // EscMenuState 门（O：`tag == 2 || (tag == 10 && flag != 1)`）。
    let view = menu.view();
    let gate_open = view.tag == 2 || (view.tag == 10 && view.flag != 1);
    let bindings = BindingsTable::default();
    if gate_open {
        let dt = time.delta;
        let mut analog = values.current.analog;
        translate_analog(&keyboard, &bindings, dt, &mut analog);
        values.current.analog = analog;
        translate_digital(&keyboard, &mouse, &bindings, &mut values.current.digital);

        // `EventReader<MouseWheel>` → analog 槽 4 camera_zoom（O：行 282..336；逐事件
        // 权重 = tag 0（Line）8.0 / 其余 1.0，`_UNK_142934604`=8.0、`_UNK_142925904`=1.0；
        // 只累加 y 分量到 camera_zoom.x）。
        for event in wheel.read() {
            let weight = if event.unit == 0 { 8.0 } else { 1.0 };
            values.current.analog[4].x += weight * event.y;
        }

        // 右键手势（O：行 438..471）：`Local<MouseMoved>` 在右键按下沿累加游标增量长度；
        // `right_click = (MouseMoved < 3.0) & 右键释放 | 槽 23 focus_edit 绑定 | 既有`。
        // 阈值 3.0 = `_UNK_1429cd078`。
        let moved_len = cursor_delta.read().fold(0.0, |acc, d| acc + d.0.length());
        if mouse.just_pressed(1) {
            mouse_moved.0 += moved_len;
        }
        if mouse.just_released(1) && mouse_moved.0 < 3.0 {
            values.current.digital[1] = true;
        }
        if bindings.slots[23].bound_key() && key_pressed(&keyboard, bindings.slots[23].key) {
            values.current.digital[1] = true;
        }

        // screenshot 修饰（O：行 678..693）：槽 25 按下且键 0x3d 未按下。
        let screenshot = bindings.slots[25];
        if screenshot.bound_key()
            && key_pressed(&keyboard, screenshot.key)
            && !key_pressed(&keyboard, 0x3d)
        {
            values.current.digital[29] = true;
        }

        // Demo 门硬编码键（O：全局 0x143634cd8 != 0，行 730..776）。
        if demo.0 {
            if key_pressed(&keyboard, 0x35) && key_pressed(&keyboard, 0x3d) {
                values.current.digital[26] = true; // clear_canvas
            }
            if key_pressed(&keyboard, 0x40) {
                values.current.digital[28] = true; // developer_menu
            }
            if key_pressed(&keyboard, 0xab) && key_pressed(&keyboard, 0x3d) {
                values.current.digital[30] = true; // screenshot_grading_reference
            }
            if key_pressed(&keyboard, 0x25) && !key_pressed(&keyboard, 0x33) {
                values.current.digital[34] = true; // human_scale_ref_toggle
            }
            if key_pressed(&keyboard, 0x23) {
                values.current.digital[35] = true; // ivy_growth_toggle
            }
            if key_pressed(&keyboard, 0x29) {
                values.current.digital[36] = true; // ivy_spawn
            }
            if key_pressed(&keyboard, 0xa6) {
                values.current.digital[37] = true; // camera_snapshot_add
            }
        }
        if key_pressed(&keyboard, 0xa7) {
            values.current.digital[38] = true; // camera_snapshot_restore（恒启）
        }

        // 键鼠使用 → 输入方式切到 MouseKeyboard（O：ActiveInputModeTracker 更新）。
        *mode_tracker = ActiveInputModeTracker::new_default_mouse_keyboard(now);
    }
}

/// 滚轮事件（I：bevy `MouseWheel` 等价物；`unit` 0=Line/1=Pixel，`x/y` 增量）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct MouseWheel {
    pub unit: u8,
    pub x: f32,
    pub y: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bindings_match_the_original_sequence() {
        let b = BindingsTable::default();
        assert_eq!(b.slots[0].key, 0x2a); // move_forward
        assert_eq!(b.slots[3].key, 0x17); // move_right
        assert_eq!(b.slots[4].key, 0x18); // move_up
        assert_eq!(b.slots[6].key, UNBOUND_KEY); // rotate_left 默认未绑定
        assert_eq!(b.slots[10].key, 0x07); // building
        assert_eq!(b.slots[15].key, 0x0c); // terrain
        assert_eq!(b.slots[28].key, 0x49); // ui_cancel
        assert_eq!(b.slots[36].key, UNBOUND_KEY); // cinematic_camera_ctrls
        assert_eq!(b.slots[37].mouse, 1); // camera_rotate_mode
        assert_eq!(b.slots[38].mouse, 2); // camera_pan_mode
        assert_eq!(b.slots[39].mouse, 1); // focus_edit
    }

    #[test]
    fn analog_translation_writes_pan_height_zoom_rotate_slots() {
        let mut keyboard = KeyButtonInput::default();
        keyboard.pressed.insert(0x2a); // move_forward → camera_pan.y-
        let mut analog = [Vec2::ZERO; ANALOG_SLOTS];
        translate_analog(&keyboard, &BindingsTable::default(), 1.0 / 60.0, &mut analog);
        assert!(analog[2].y < 0.0); // camera_pan.y 减少
        assert_eq!(analog[2].x, 0.0);
        assert_eq!(analog[3], Vec2::ZERO);
        assert_eq!(analog[4], Vec2::ZERO);
        assert_eq!(analog[5], Vec2::ZERO);
        // move_up → camera_height.x+。
        keyboard.pressed.clear();
        keyboard.pressed.insert(0x18);
        let mut analog = [Vec2::ZERO; ANALOG_SLOTS];
        translate_analog(&keyboard, &BindingsTable::default(), 1.0 / 60.0, &mut analog);
        assert!(analog[3].x > 0.0);
        // zoom_in 默认未绑定 → 无 zoom 输出。
        assert_eq!(analog[4], Vec2::ZERO);
    }

    #[test]
    fn digital_translation_maps_binding_slots_to_actions() {
        let mut keyboard = KeyButtonInput::default();
        keyboard.pressed.insert(0x07); // building
        keyboard.pressed.insert(0x49); // ui_cancel（绑定槽 28）
        let mut out = [false; DIGITAL_SLOTS];
        translate_digital(&keyboard, &MouseButtonInput::default(), &BindingsTable::default(), &mut out);
        assert!(out[4]); // tool_building
        assert!(out[18]); // ui_cancel
        assert!(!out[5]);
        assert!(!out[2]); // camera_rotate_mode 需鼠标
    }

    #[test]
    fn camera_rotate_uses_raw_delta_and_scales_by_1800() {
        // 绑定槽 6/7 默认未绑定，但可经 assign 绑定后按原始 delta×1800 驱动。
        let mut bindings = BindingsTable::default();
        bindings.slots[6].key = 0x1a;
        bindings.slots[7].key = 0x1b;
        let mut keyboard = KeyButtonInput::default();
        keyboard.pressed.insert(0x1a);
        let mut analog = [Vec2::ZERO; ANALOG_SLOTS];
        translate_analog(&keyboard, &bindings, 1.0 / 60.0, &mut analog);
        // rotate_left：−1800 × delta。
        assert!((analog[5].x - (-1800.0 / 60.0)).abs() < 1e-3);
    }
}

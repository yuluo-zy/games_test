//! 键鼠事件到动作槽的翻译。键位表为可配置资源，手势使用本帧光标增量。
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

/// Steam 输入门（O：RE-ANALYSIS2 + fresh 反编译 2026-10-17——translate_input 内 Arc 克隆
/// `SteamClientManager` → `SteamAPI_SteamUtils_v010`（0x142923570）→
/// `IsSteamRunningOnSteamDeck`（0x1429234f0）只参与 mode-tracker 更新门控（Deck 时仅键盘
/// 计数）与 screenshot 修饰跳过；主体（点击扫描/门/绑定翻译/滚轮/手势/历史）**恒运行**）。
/// `systems::steam` 不在本 crate（库外接线登记，手柄动作需要由宿主写入统一动作值，设备接线位于
/// `systems::steam::update_steam_inputs`，22 同构块），本资源由宿主按该门填充；
/// `present=false` = 无 Steam（走键盘/鼠标路径）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SteamInputActive {
    /// 原生 `Option<Res<SteamClientResource>>` 是否为 Some（Steam 客户端存在）。
    pub present: bool,
    /// `IsSteamRunningOnSteamDeck` 结果（仅 present 时有意义）。
    pub on_deck: bool,
}

/// Demo 门（O：全局字节 `0x143634cd8` = demo 开关；为 1 时额外启用演示模式的硬编码键）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DemoModeEnabled(pub bool);

/// The retained table is a numeric projection of the loaded profile, not an
/// independently configured resource. Slots 0..36 are keys, 37..39 are mouse.
pub const BINDING_SLOTS: usize = 40;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionBinding {
    pub key: u16,
    pub mouse: Option<u16>,
}
impl ActionBinding {
    pub const UNBOUND: Self = Self {
        key: UNBOUND_KEY,
        mouse: None,
    };
    pub fn bound_key(&self) -> bool {
        self.key != UNBOUND_KEY
    }
}
#[derive(Resource, Clone, Debug)]
pub struct BindingsTable {
    pub slots: [ActionBinding; BINDING_SLOTS],
}
impl TryFrom<&crate::game_profile::GameKeyBindings> for BindingsTable {
    type Error = anyhow::Error;
    fn try_from(bindings: &crate::game_profile::GameKeyBindings) -> Result<Self, Self::Error> {
        use bevy_input::mouse::MouseButton;
        let mut slots = [ActionBinding::UNBOUND; BINDING_SLOTS];
        for (slot, key) in bindings.bindings.iter().copied().enumerate() {
            if let Some(key) = key {
                slots[slot].key = crate::game_profile::desktop_keycode_index(key)?;
            }
        }
        for (slot, mouse) in bindings.mouse.iter().copied().enumerate() {
            slots[37 + slot].mouse = match mouse {
                None => None,
                Some(MouseButton::Left) => Some(0),
                Some(MouseButton::Right) => Some(1),
                Some(MouseButton::Middle) => Some(2),
                Some(MouseButton::Back) => Some(3),
                Some(MouseButton::Forward) => Some(4),
                Some(MouseButton::Other(button)) if button > 4 => Some(button),
                Some(MouseButton::Other(button)) => anyhow::bail!(
                    "mouse Other({button}) collides with a standard button in the desktop u16 facade"),
            };
        }
        Ok(Self { slots })
    }
}
impl Default for BindingsTable {
    fn default() -> Self {
        Self::try_from(&crate::game_profile::GameKeyBindings::default())
            .expect("original default bindings use supported desktop inputs")
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
    AnalogTarget {
        slot: 2,
        axis: 1,
        sign: -1.0,
        scalar: 1.0,
        use_raw_delta: false,
    },
    AnalogTarget {
        slot: 2,
        axis: 1,
        sign: 1.0,
        scalar: 1.0,
        use_raw_delta: false,
    },
    AnalogTarget {
        slot: 2,
        axis: 0,
        sign: -1.0,
        scalar: 1.0,
        use_raw_delta: false,
    },
    AnalogTarget {
        slot: 2,
        axis: 0,
        sign: 1.0,
        scalar: 1.0,
        use_raw_delta: false,
    },
    // 槽 4..5 move_up/down → camera_height（analog 3）。
    AnalogTarget {
        slot: 3,
        axis: 0,
        sign: 1.0,
        scalar: 1.0,
        use_raw_delta: false,
    },
    AnalogTarget {
        slot: 3,
        axis: 0,
        sign: -1.0,
        scalar: 1.0,
        use_raw_delta: false,
    },
    // 槽 6..7 rotate_left/right → camera_rotate（analog 5），±1800·原始 delta。
    AnalogTarget {
        slot: 5,
        axis: 0,
        sign: -1.0,
        scalar: 1800.0,
        use_raw_delta: true,
    },
    AnalogTarget {
        slot: 5,
        axis: 0,
        sign: 1.0,
        scalar: 1800.0,
        use_raw_delta: true,
    },
    // 槽 8..9 zoom_in/out → camera_zoom（analog 4），±1.5·(delta·30)。
    AnalogTarget {
        slot: 4,
        axis: 0,
        sign: 1.0,
        scalar: 1.5,
        use_raw_delta: false,
    },
    AnalogTarget {
        slot: 4,
        axis: 0,
        sign: -1.0,
        scalar: 1.5,
        use_raw_delta: false,
    },
];

/// 绑定槽 → 数字动作槽（O：0x1417cd0a0 行 476..636 逐槽写）。
/// 前两个为 None 表示该槽映射到鼠标手势/修饰键，单独处理。
const DIGITAL_TARGETS: [(usize, u16); 28] = [
    (10, 4),  // building → tool_building
    (11, 5),  // path → tool_path
    (12, 6),  // decorators → tool_decorator
    (13, 7),  // customize → tool_customize
    (14, 9),  // eraser → tool_eraser
    (15, 8),  // terrain → tool_terrain
    (16, 10), // garden → tool_garden
    (17, 11), // tree → tool_tree
    (18, 12), // water → tool_water
    (19, 13), // photomode
    (20, 14), // time_of_day
    (21, 16), // undo
    (22, 17), // redo
    (23, 1),  // focus_edit → right_click
    (24, 15), // disable_snapping → snapping_disable
    (26, 31), // feedback → screenshot_feedback
    (27, 32), // hide_ui_photomode
    (28, 18), // ui_cancel
    (29, 19), // ui_confirm
    (30, 20), // ui_left
    (31, 21), // ui_right
    (32, 22), // ui_down
    (33, 23), // ui_up
    (34, 24), // ui_tab_prev
    (35, 25), // ui_tab_next
    (36, 33), // cinematic_camera_ctrls
    (37, 2),  // 鼠标 {1} → camera_rotate_mode
    (38, 3),  // 鼠标 {2} → camera_pan_mode
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
        let binding = &bindings.slots[binding_slot];
        let pressed = if binding_slot >= 37 {
            // 鼠标绑定槽（37=Rotate、38=Move）。
            binding.mouse.is_some_and(|button| mouse.pressed(button))
        } else {
            binding.bound_key() && key_pressed(keyboard, binding.key)
        };
        if pressed {
            out[digital_slot as usize] = true;
        }
    }
    // 硬编码 ESC 键 0x73 → main_menu（digital[27]@+0xa3，O：0x1417ce01b `or [r14+0xa3]`）。
    // 注意：DIGITAL_TARGETS 的 (28, 18) 是**槽 28 绑定**的 ui_cancel（+0x9a），与 0x73 不同目标。
    if key_pressed(keyboard, 0x73) {
        out[27] = true;
    }
}

/// `translate_input` @0x1417cd0a0（O 全解，fresh 反编译 + 机器码 2026-10-17 重建）。
///
/// 相对 PDB 签名缺 `Option<Res<SteamClientResource>>`、`EventReader<MouseWheel>`、
/// `Res<GameProfile>`：前两者用 [`SteamInputActive`] / 滚轮读取代替（Steam 全链库外登记），
/// GameProfile 的真实键位投影到当前数字输入适配器，不保留独立默认绑定权威。
///
/// A1：原生**无提前 return**——Steam 只参与 mode-tracker 更新门控（Deck 时仅键盘计数）与
/// screenshot 修饰跳过；主体恒运行。
#[allow(clippy::too_many_arguments)]
pub fn translate_input(
    time: Res<GameTime>,
    elapsed: Res<bevy_time::Time>,
    keyboard: Res<KeyButtonInput>,
    profile: Res<crate::game_profile::GameProfile>,
    mouse: Res<MouseButtonInput>,
    menu: Res<EscMenuState>,
    mut values: ResMut<ActionSetValues>,
    mut mode_tracker: ResMut<ActiveInputModeTracker>,
    steam: Res<SteamInputActive>,
    demo: Res<DemoModeEnabled>,
    mut mouse_moved: Local<MouseMoved>,
    mut wheel: EventReader<MouseWheel>,
    mut cursor_delta: EventReader<InputCursorDelta>,
    mut history_buttons: EventReader<UiHistoryButtonPressed>,
) {
    let bindings = BindingsTable::try_from(&profile.key_bindings)
        .expect("loaded profile bindings must be representable by the desktop input facade");
    // A2（O：LAB_1417cd126）mode-tracker 更新在函数**开头**，按输入事件条件：
    // 无 Steam / 非 Deck → mouse.just_pressed || keyboard.just_pressed（+0x38 计数）；
    // Steam + Deck → 仅 keyboard.just_pressed（0x1417cd082）。只更新
    // `mouse_keyboard_deadline`（+0x10/+0x18 = Instant::now()），controller_deadline 不动。
    // 使用宿主单调时间；暂停游戏不会阻止键鼠/手柄提示切换。
    let has_mouse_events = mouse.get_just_pressed().next().is_some();
    let has_keyboard_events = keyboard.get_just_pressed().next().is_some();
    let update_tracker = if steam.present && steam.on_deck {
        has_keyboard_events
    } else {
        has_mouse_events || has_keyboard_events
    };
    if update_tracker {
        mode_tracker.mouse_keyboard_deadline = elapsed
            .elapsed()
            .saturating_add(crate::input::INPUT_MODE_COOLDOWN);
    }

    // 鼠标按下扫描 → digital 槽 0 `click`（O：0x1417cd0a0 行 96..180 SIMD 扫描 pressed 集，
    // 任一键值 == 0（MouseButton::Left）→ `click = pressed(Left)`，`*(values+0x88) |= 按下位`）。
    if mouse.pressed(0) {
        values.current.digital[0] = true;
    }

    // EscMenuState 门（O：`tag == 2 || (tag == 10 && flag != 1)`）。
    let view = menu.view();
    let gate_open = view.tag == 2 || (view.tag == 10 && view.flag != 1);
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

        // B @1417cd959 uses the configured mouse_focus_edit binding, including
        // Some(Left). Just-press resets distance; held frames alone accumulate.
        // The release frame compares the already accumulated distance (< 3).
        let last_len = cursor_delta
            .read()
            .last()
            .map_or(0.0, |delta| delta.0.length());
        if let Some(button) = bindings.slots[39].mouse {
            if mouse.just_pressed(button) {
                mouse_moved.0 = 0.0;
            } else if mouse.pressed(button) {
                mouse_moved.0 += last_len;
            }
            if mouse.just_released(button) && mouse_moved.0 < 3.0 {
                values.current.digital[1] = true;
            }
        }
        if bindings.slots[23].bound_key() && key_pressed(&keyboard, bindings.slots[23].key) {
            values.current.digital[1] = true;
        }

        // screenshot 修饰（O：行 678..693，A4）：槽 25 按下且键 0x3d 未按下 →
        // digital[29]；**仅无 Steam（puVar5==0）时执行**，Steam 存在时跳过（0x1417cdffc
        // `test r12,r12; jnz`）。
        if !steam.present {
            let screenshot = bindings.slots[25];
            if screenshot.bound_key()
                && key_pressed(&keyboard, screenshot.key)
                && !key_pressed(&keyboard, 0x3d)
            {
                values.current.digital[29] = true;
            }
        }

        // Demo 门硬编码键（O：全局 0x143634cd8 != 0，行 730..776；0xa6 经机器码核验
        // `setne`——`pressed(0xab) && pressed(0x3d)`）。
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

        // UiHistoryButtonPressed 消费（O：0x1417cd0a0 行 780..833）：**只消费最末事件**；
        // 载荷字节 `0` = undo → digital[16]（+0x98），非零 = redo → digital[17]（+0x99）。
        if let Some(history) = history_buttons.read().last() {
            if history.is_redo() {
                values.current.digital[17] = true;
            } else {
                values.current.digital[16] = true;
            }
        }
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
    fn profile_remapping_reaches_the_live_table_and_left_mouse_is_bound() {
        use bevy_input::{keyboard::KeyCode, mouse::MouseButton};
        let mut profile = crate::game_profile::GameKeyBindings::default();
        profile.bindings[21] = Some(KeyCode::KeyU);
        profile.mouse[0] = Some(MouseButton::Left);
        profile.mouse[2] = None;
        let table = BindingsTable::try_from(&profile).unwrap();
        let mut keyboard = KeyButtonInput::default();
        keyboard.press(crate::game_profile::desktop_keycode_index(KeyCode::KeyU).unwrap());
        let mut mouse = MouseButtonInput::default();
        mouse.press(0);
        let mut output = [false; DIGITAL_SLOTS];
        translate_digital(&keyboard, &mouse, &table, &mut output);
        assert!(
            output[16],
            "the user's remapped undo key must reach the action set"
        );
        assert!(
            output[2],
            "a configured left button is not an unbound mouse slot"
        );
        assert_eq!(table.slots[39].mouse, None);
    }

    #[test]
    fn screenshot_is_not_written_by_the_unconditional_digital_mapping() {
        let mut keyboard = KeyButtonInput::default();
        keyboard.press(0xab);
        let mut output = [false; DIGITAL_SLOTS];
        translate_digital(
            &keyboard,
            &MouseButtonInput::default(),
            &BindingsTable::default(),
            &mut output,
        );
        assert!(
            !output[29],
            "Steam/modifier gate owns the screenshot action"
        );
    }

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
        assert_eq!(b.slots[37].mouse, Some(1)); // camera_rotate_mode
        assert_eq!(b.slots[38].mouse, Some(2)); // camera_pan_mode
        assert_eq!(b.slots[39].mouse, Some(1)); // focus_edit
    }

    #[test]
    fn analog_translation_writes_pan_height_zoom_rotate_slots() {
        let mut keyboard = KeyButtonInput::default();
        keyboard.press(0x2a); // move_forward → camera_pan.y-
        let mut analog = [Vec2::ZERO; ANALOG_SLOTS];
        translate_analog(
            &keyboard,
            &BindingsTable::default(),
            1.0 / 60.0,
            &mut analog,
        );
        assert!(analog[2].y < 0.0); // camera_pan.y 减少
        assert_eq!(analog[2].x, 0.0);
        assert_eq!(analog[3], Vec2::ZERO);
        assert_eq!(analog[4], Vec2::ZERO);
        assert_eq!(analog[5], Vec2::ZERO);
        // move_up → camera_height.x+。
        keyboard.reset_all();
        keyboard.press(0x18);
        let mut analog = [Vec2::ZERO; ANALOG_SLOTS];
        translate_analog(
            &keyboard,
            &BindingsTable::default(),
            1.0 / 60.0,
            &mut analog,
        );
        assert!(analog[3].x > 0.0);
        // zoom_in 默认未绑定 → 无 zoom 输出。
        assert_eq!(analog[4], Vec2::ZERO);
    }

    #[test]
    fn digital_translation_maps_binding_slots_to_actions() {
        let mut keyboard = KeyButtonInput::default();
        keyboard.press(0x07); // building
        keyboard.press(0x49); // ui_cancel（绑定槽 28）
        let mut out = [false; DIGITAL_SLOTS];
        translate_digital(
            &keyboard,
            &MouseButtonInput::default(),
            &BindingsTable::default(),
            &mut out,
        );
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
        keyboard.press(0x1a);
        let mut analog = [Vec2::ZERO; ANALOG_SLOTS];
        translate_analog(&keyboard, &bindings, 1.0 / 60.0, &mut analog);
        // rotate_left：−1800 × delta。
        assert!((analog[5].x - (-1800.0 / 60.0)).abs() < 1e-3);
    }
}

//! B `game_profile.rs` — 玩家档案（0x208B=520B；ron）。
//!
//! `serialize<ron>` 0x140a658a0 只写 **5 字段**：tutorial / tutorial_cooldowns /
//! **player_id** / key_bindings / new_glade_count（reviewer-coregap-2 §2 fresh 反编译
//! 纠错：**不写 game_installation_id**）；`deserialize` 0x1404b0ed0 名表
//! tutorial/tutorial_cooldowns/player_id/key_bindings/new_glade_count，case 0x14 把
//! 旧档 `game_installation_id`（20 字符）作为 player_id 的别名。
//! `default` 0x140916420 = `Uuid::new_v4`（player_id）+ 默认键位。
//!
//! 真实消费方 = `load_game` 0x14146d030（第 3 参 `Res<GameProfile>`）/ save_game /
//! startup_session（`load_game_profile` 0x140916480 / `save_game_profile` 0x140917230）。
//! **load_completion_checker 不含 GameProfile**（re-analyst 澄清：skeleton 引用是 load_game
//! tuple 注释串）。
use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A configured keyboard binding is the original Option<KeyCode>, not a key/action pair.
/// B serialize @140831740 calls the locked dependency's Option<KeyCode> serializer.
pub type KeyBinding = Option<bevy_input::keyboard::KeyCode>;

/// Keyboard and mouse bindings share the loaded profile authority. The array
/// representation is retained for the current input pipeline; persistence uses
/// the original forty named fields (37 keyboard and three mouse).
#[derive(Clone, Debug, PartialEq)]
pub struct GameKeyBindings {
    pub bindings: [KeyBinding; 37],
    pub mouse: [Option<bevy_input::mouse::MouseButton>; 3],
}
impl Default for GameKeyBindings {
    fn default() -> Self {
        use bevy_input::keyboard::KeyCode::*;
        use bevy_input::mouse::MouseButton;
        Self {
            bindings: [
                Some(KeyW),
                Some(KeyS),
                Some(KeyA),
                Some(KeyD),
                Some(KeyE),
                Some(KeyQ),
                None,
                None,
                None,
                None,
                Some(Digit1),
                Some(Digit2),
                Some(Digit3),
                Some(Digit4),
                Some(Digit5),
                Some(Digit6),
                Some(Digit7),
                Some(Digit8),
                Some(Digit9),
                Some(KeyP),
                Some(KeyT),
                Some(KeyZ),
                Some(KeyX),
                None,
                Some(ControlLeft),
                Some(F12),
                Some(F6),
                Some(KeyH),
                Some(Delete),
                Some(Enter),
                Some(ArrowLeft),
                Some(ArrowRight),
                Some(ArrowDown),
                Some(ArrowUp),
                Some(PageUp),
                Some(PageDown),
                None,
            ],
            mouse: [
                Some(MouseButton::Right),
                Some(MouseButton::Middle),
                Some(MouseButton::Right),
            ],
        }
    }
}

// A serde projection reuses Bevy's enum implementation. Native discriminants
// and uninitialized padding are never reinterpreted as a new wire protocol.
#[derive(Serialize, Deserialize)]
#[serde(rename = "KeyBindings")]
struct SerializedKeyBindings {
    move_forward: KeyBinding,
    move_back: KeyBinding,
    move_left: KeyBinding,
    move_right: KeyBinding,
    move_up: KeyBinding,
    move_down: KeyBinding,
    rotate_left: KeyBinding,
    rotate_right: KeyBinding,
    zoom_in: KeyBinding,
    zoom_out: KeyBinding,
    building: KeyBinding,
    path: KeyBinding,
    decorators: KeyBinding,
    customize: KeyBinding,
    eraser: KeyBinding,
    terrain: KeyBinding,
    garden: KeyBinding,
    tree: KeyBinding,
    water: KeyBinding,
    photomode: KeyBinding,
    time_of_day: KeyBinding,
    undo: KeyBinding,
    redo: KeyBinding,
    focus_edit: KeyBinding,
    disable_snapping: KeyBinding,
    screenshot: KeyBinding,
    feedback: KeyBinding,
    hide_ui_photomode: KeyBinding,
    ui_cancel: KeyBinding,
    ui_confirm: KeyBinding,
    ui_left: KeyBinding,
    ui_right: KeyBinding,
    ui_down: KeyBinding,
    ui_up: KeyBinding,
    ui_tab_prev: KeyBinding,
    ui_tab_next: KeyBinding,
    mouse_rotate: Option<bevy_input::mouse::MouseButton>,
    mouse_pan: Option<bevy_input::mouse::MouseButton>,
    mouse_focus_edit: Option<bevy_input::mouse::MouseButton>,
    experimental_camera: KeyBinding,
}
impl Serialize for GameKeyBindings {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        SerializedKeyBindings {
            move_forward: self.bindings[0],
            move_back: self.bindings[1],
            move_left: self.bindings[2],
            move_right: self.bindings[3],
            move_up: self.bindings[4],
            move_down: self.bindings[5],
            rotate_left: self.bindings[6],
            rotate_right: self.bindings[7],
            zoom_in: self.bindings[8],
            zoom_out: self.bindings[9],
            building: self.bindings[10],
            path: self.bindings[11],
            decorators: self.bindings[12],
            customize: self.bindings[13],
            eraser: self.bindings[14],
            terrain: self.bindings[15],
            garden: self.bindings[16],
            tree: self.bindings[17],
            water: self.bindings[18],
            photomode: self.bindings[19],
            time_of_day: self.bindings[20],
            undo: self.bindings[21],
            redo: self.bindings[22],
            focus_edit: self.bindings[23],
            disable_snapping: self.bindings[24],
            screenshot: self.bindings[25],
            feedback: self.bindings[26],
            hide_ui_photomode: self.bindings[27],
            ui_cancel: self.bindings[28],
            ui_confirm: self.bindings[29],
            ui_left: self.bindings[30],
            ui_right: self.bindings[31],
            ui_down: self.bindings[32],
            ui_up: self.bindings[33],
            ui_tab_prev: self.bindings[34],
            ui_tab_next: self.bindings[35],
            mouse_rotate: self.mouse[0],
            mouse_pan: self.mouse[1],
            mouse_focus_edit: self.mouse[2],
            experimental_camera: self.bindings[36],
        }
        .serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for GameKeyBindings {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let fields = SerializedKeyBindings::deserialize(deserializer)?;
        Ok(Self {
            bindings: [
                fields.move_forward,
                fields.move_back,
                fields.move_left,
                fields.move_right,
                fields.move_up,
                fields.move_down,
                fields.rotate_left,
                fields.rotate_right,
                fields.zoom_in,
                fields.zoom_out,
                fields.building,
                fields.path,
                fields.decorators,
                fields.customize,
                fields.eraser,
                fields.terrain,
                fields.garden,
                fields.tree,
                fields.water,
                fields.photomode,
                fields.time_of_day,
                fields.undo,
                fields.redo,
                fields.focus_edit,
                fields.disable_snapping,
                fields.screenshot,
                fields.feedback,
                fields.hide_ui_photomode,
                fields.ui_cancel,
                fields.ui_confirm,
                fields.ui_left,
                fields.ui_right,
                fields.ui_down,
                fields.ui_up,
                fields.ui_tab_prev,
                fields.ui_tab_next,
                fields.experimental_camera,
            ],
            mouse: [
                fields.mouse_rotate,
                fields.mouse_pan,
                fields.mouse_focus_edit,
            ],
        })
    }
}

/// Numeric adapter used by the existing u16 desktop input facade. The order is
/// the locked Bevy 0.16 KeyCode order (B native table 14294c58c). Native-specific
/// scan codes have no u16 identity in that facade and are rejected explicitly.
pub fn desktop_keycode_index(key: bevy_input::keyboard::KeyCode) -> anyhow::Result<u16> {
    use bevy_input::keyboard::KeyCode;
    const IDENTIFIED: [KeyCode; 194] = [
        KeyCode::Backquote,
        KeyCode::Backslash,
        KeyCode::BracketLeft,
        KeyCode::BracketRight,
        KeyCode::Comma,
        KeyCode::Digit0,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
        KeyCode::Equal,
        KeyCode::IntlBackslash,
        KeyCode::IntlRo,
        KeyCode::IntlYen,
        KeyCode::KeyA,
        KeyCode::KeyB,
        KeyCode::KeyC,
        KeyCode::KeyD,
        KeyCode::KeyE,
        KeyCode::KeyF,
        KeyCode::KeyG,
        KeyCode::KeyH,
        KeyCode::KeyI,
        KeyCode::KeyJ,
        KeyCode::KeyK,
        KeyCode::KeyL,
        KeyCode::KeyM,
        KeyCode::KeyN,
        KeyCode::KeyO,
        KeyCode::KeyP,
        KeyCode::KeyQ,
        KeyCode::KeyR,
        KeyCode::KeyS,
        KeyCode::KeyT,
        KeyCode::KeyU,
        KeyCode::KeyV,
        KeyCode::KeyW,
        KeyCode::KeyX,
        KeyCode::KeyY,
        KeyCode::KeyZ,
        KeyCode::Minus,
        KeyCode::Period,
        KeyCode::Quote,
        KeyCode::Semicolon,
        KeyCode::Slash,
        KeyCode::AltLeft,
        KeyCode::AltRight,
        KeyCode::Backspace,
        KeyCode::CapsLock,
        KeyCode::ContextMenu,
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
        KeyCode::Enter,
        KeyCode::SuperLeft,
        KeyCode::SuperRight,
        KeyCode::ShiftLeft,
        KeyCode::ShiftRight,
        KeyCode::Space,
        KeyCode::Tab,
        KeyCode::Convert,
        KeyCode::KanaMode,
        KeyCode::Lang1,
        KeyCode::Lang2,
        KeyCode::Lang3,
        KeyCode::Lang4,
        KeyCode::Lang5,
        KeyCode::NonConvert,
        KeyCode::Delete,
        KeyCode::End,
        KeyCode::Help,
        KeyCode::Home,
        KeyCode::Insert,
        KeyCode::PageDown,
        KeyCode::PageUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::ArrowUp,
        KeyCode::NumLock,
        KeyCode::Numpad0,
        KeyCode::Numpad1,
        KeyCode::Numpad2,
        KeyCode::Numpad3,
        KeyCode::Numpad4,
        KeyCode::Numpad5,
        KeyCode::Numpad6,
        KeyCode::Numpad7,
        KeyCode::Numpad8,
        KeyCode::Numpad9,
        KeyCode::NumpadAdd,
        KeyCode::NumpadBackspace,
        KeyCode::NumpadClear,
        KeyCode::NumpadClearEntry,
        KeyCode::NumpadComma,
        KeyCode::NumpadDecimal,
        KeyCode::NumpadDivide,
        KeyCode::NumpadEnter,
        KeyCode::NumpadEqual,
        KeyCode::NumpadHash,
        KeyCode::NumpadMemoryAdd,
        KeyCode::NumpadMemoryClear,
        KeyCode::NumpadMemoryRecall,
        KeyCode::NumpadMemoryStore,
        KeyCode::NumpadMemorySubtract,
        KeyCode::NumpadMultiply,
        KeyCode::NumpadParenLeft,
        KeyCode::NumpadParenRight,
        KeyCode::NumpadStar,
        KeyCode::NumpadSubtract,
        KeyCode::Escape,
        KeyCode::Fn,
        KeyCode::FnLock,
        KeyCode::PrintScreen,
        KeyCode::ScrollLock,
        KeyCode::Pause,
        KeyCode::BrowserBack,
        KeyCode::BrowserFavorites,
        KeyCode::BrowserForward,
        KeyCode::BrowserHome,
        KeyCode::BrowserRefresh,
        KeyCode::BrowserSearch,
        KeyCode::BrowserStop,
        KeyCode::Eject,
        KeyCode::LaunchApp1,
        KeyCode::LaunchApp2,
        KeyCode::LaunchMail,
        KeyCode::MediaPlayPause,
        KeyCode::MediaSelect,
        KeyCode::MediaStop,
        KeyCode::MediaTrackNext,
        KeyCode::MediaTrackPrevious,
        KeyCode::Power,
        KeyCode::Sleep,
        KeyCode::AudioVolumeDown,
        KeyCode::AudioVolumeMute,
        KeyCode::AudioVolumeUp,
        KeyCode::WakeUp,
        KeyCode::Meta,
        KeyCode::Hyper,
        KeyCode::Turbo,
        KeyCode::Abort,
        KeyCode::Resume,
        KeyCode::Suspend,
        KeyCode::Again,
        KeyCode::Copy,
        KeyCode::Cut,
        KeyCode::Find,
        KeyCode::Open,
        KeyCode::Paste,
        KeyCode::Props,
        KeyCode::Select,
        KeyCode::Undo,
        KeyCode::Hiragana,
        KeyCode::Katakana,
        KeyCode::F1,
        KeyCode::F2,
        KeyCode::F3,
        KeyCode::F4,
        KeyCode::F5,
        KeyCode::F6,
        KeyCode::F7,
        KeyCode::F8,
        KeyCode::F9,
        KeyCode::F10,
        KeyCode::F11,
        KeyCode::F12,
        KeyCode::F13,
        KeyCode::F14,
        KeyCode::F15,
        KeyCode::F16,
        KeyCode::F17,
        KeyCode::F18,
        KeyCode::F19,
        KeyCode::F20,
        KeyCode::F21,
        KeyCode::F22,
        KeyCode::F23,
        KeyCode::F24,
        KeyCode::F25,
        KeyCode::F26,
        KeyCode::F27,
        KeyCode::F28,
        KeyCode::F29,
        KeyCode::F30,
        KeyCode::F31,
        KeyCode::F32,
        KeyCode::F33,
        KeyCode::F34,
        KeyCode::F35,
    ];
    IDENTIFIED
        .iter()
        .position(|candidate| *candidate == key)
        .map(|index| index as u16 + 1)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "native scan-code binding {key:?} is not supported by the desktop u16 input facade"
            )
        })
}

/// 教学/提示计时区（20B @+0x1e0..+0x1f4，O：`exponential_back_off` 0x142284800 写面 +
/// `reset_hints` 0x141c1c8d0 清零面）。
///
/// 布局（fresh 反编译，evidence `batch3-core.txt`）：
/// `+0x1e0`/`+0x1e4` 第一对计时/冷却（0x1e4 缺省 720.0，`clamp(old*1.5,720,10800)`）；
/// `+0x1e8`/`+0x1ec` 第二对（同上）；`+0x1f0` 每次帧 `+= dt` 的累加器
/// （`reset_hints` 以两个 u64 整块清零 0x1e0..0x1ef，即本结构 20B 中的前 16B）。
///
/// 字段名 I（按消费 hint 语义命名）；ron 序列化形状与原版一致性待取证（serde 仍派生）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TutorialCooldowns {
    /// +0x1e0：第一计时（f32，`reset_hints` 清零）。
    pub timer_a: f32,
    /// +0x1e4：第一冷却（f32，缺省 720.0）。
    pub cooldown_a: f32,
    /// +0x1e8：第二计时（f32）。
    pub timer_b: f32,
    /// +0x1ec：第二冷却（f32，缺省 720.0）。
    pub cooldown_b: f32,
    /// +0x1f0：鼠标未动累加器（f32，每帧 += GameTime.delta）。
    pub time_since_mouse_moved: f32,
}

impl TutorialCooldowns {
    /// 冷却缺省值（O：`exponential_back_off` 在 0.0 时写入 `0x44340000` = 720.0）。
    pub const DEFAULT_COOLDOWN: f32 = 720.0;

    /// 缺省计时区（I：除冷却外全 0）。
    pub fn default_cooldowns() -> Self {
        Self {
            timer_a: 0.0,
            cooldown_a: Self::DEFAULT_COOLDOWN,
            timer_b: 0.0,
            cooldown_b: Self::DEFAULT_COOLDOWN,
            time_since_mouse_moved: 0.0,
        }
    }
}

/// 玩家档案（0x208B；ron）。`player_id` 序列化（第 3 位）；`game_installation_id`
/// **不是**序列化字段（仅旧档 `game_installation_id` 名经 serde alias 映射到
/// `player_id`，reviewer-coregap-2 §2 fresh `serialize` 0x140a658a0 /
/// `deserialize` 0x1404b0ed0 纠错）。
/// 字段声明序按原文 ron serialize 序：tutorial → tutorial_cooldowns →
/// player_id → key_bindings → new_glade_count（RE-DOMAIN-TYPES §4② REVIEW4 复核 +
/// reviewer §2 修正）。
///
/// `Resource`（O：native `Res<GameProfile>`，load_game 0x14146d030 第 3 参 /
/// startup_session load_game_session 第 3 参，① T2 契约 §1；crate 以 bevy Resource
/// 承载，宿主经 `load_game_profile` 0x140916480 装配）。
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameProfile {
    /// @+0x1f4（bool）：ron 序列化的 `tutorial` 字段；也作 deselect_tool 提示完成标志
    /// （`tutorial_completion` 0x14227c230 case 0 写 1，`exponential_back_off` 清 0）。
    pub tutorial: bool,
    /// @+0x1e0（20B 计时区）
    pub tutorial_cooldowns: TutorialCooldowns,
    /// @+0x1c8（Uuid 16B）：内部身份，**ron 序列化第 3 位**（O：serialize 0x140a658a0
    /// 写 PlayerId；deserialize 0x1404b0ed0 名表 case 0x14 把旧档
    /// `game_installation_id`（20 字符）别名到本字段）。
    #[serde(alias = "game_installation_id")]
    pub player_id: Uuid,
    /// @+0x000
    pub key_bindings: GameKeyBindings,
    /// @+0x1d8 (`usize`; B 64-bit). Original serialize @140a658a0
    /// calls serialize_field<usize>; startup @14166f893 reads a qword.
    pub new_glade_count: usize,
    /// @+0x1bc：相机 rotate 提示字形模式（u16，O：`camera_rotate_hint` 0x14227d2f0
    /// 读 `*(short*)(profile+0x1bc)` 比 1/2）。
    #[serde(skip_serializing, default)]
    pub camera_rotate_glyph_mode: u16,
    /// @+0x1c0：相机 pan 提示字形模式（u16，O：`camera_pan_hint` 0x14227d000 读
    /// `*(short*)(profile+0x1c0)` 比 1/2）。
    #[serde(skip_serializing, default)]
    pub camera_pan_glyph_mode: u16,
    /// @+0x1c4：聚焦/相机绑定提示字形模式（u16，O：`make_focus_or_camera_binding_hint`
    /// 0x141c3de00 读 `*(short*)(profile+0x1c4)` 比 1/2）。
    #[serde(skip_serializing, default)]
    pub focus_glyph_mode: u16,
    // ---- 完成标志区 @+0x1f5..0x206（18 个 hint 完成标志 + 1 个复位字节，O）----
    // 偏移与写面：`tutorial_completion` 0x14227c230 18 路 switch（case 1..17 →
    // 0x1f5..0x205）+ `reset_hints` 0x141c1c8d0（0x1f4..0x206 全置 1 / 全清 0）。
    // 未进 ron 序列化（原文只写 5 字段）。
    /// @+0x1f5：focused_edit 提示完成标志（O：`hint_focused_edit_condition` 0x14227da70
    /// 读 `!= 0` → 状态 1）。
    #[serde(skip_serializing, default)]
    pub focused_edit_completed: bool,
    /// @+0x1f6：相机提示完成标志之一（O：`hints_startup` 0x14227b450 读，三标志
    /// 全置 → CameraHintsState 2；`hint_camera_display_condition` 0x14227c910 读
    /// `== 1`）。pan/rotate/zoom 与偏移的精确对应 U（仅 0x1f6..0x1f8 簇确认）。
    #[serde(skip_serializing, default)]
    pub camera_pan_completed: bool,
    /// @+0x1f7：相机提示完成标志之二（同上簇）。
    #[serde(skip_serializing, default)]
    pub camera_rotate_completed: bool,
    /// @+0x1f8：相机提示完成标志之三（同上簇）。
    #[serde(skip_serializing, default)]
    pub camera_zoom_completed: bool,
    /// @+0x1f9：terrain_layering 提示完成标志（O：`hint_terrain_layering` 0x142283cc0
    /// 读 `== 0` 才展示）。
    #[serde(skip_serializing, default)]
    pub terrain_layering_completed: bool,
    /// @+0x1fa：terrain_steepness 提示完成标志（O：`hint_terrain_steepness` 0x142283640）。
    #[serde(skip_serializing, default)]
    pub terrain_steepness_completed: bool,
    /// @+0x1fb：terrain_focus 提示完成标志（O：`hint_terrain_focus` 0x142283980）。
    #[serde(skip_serializing, default)]
    pub terrain_focus_completed: bool,
    /// @+0x1fc：disable_snapping 提示完成标志（O：`hint_disable_snapping` 0x14227e140
    /// 读 `== 0` 才展示）。
    #[serde(skip_serializing, default)]
    pub disable_snapping_completed: bool,
    /// @+0x1fd：unlink_windows 提示完成标志（O：`hint_unlink_windows` 0x14227d760
    /// 读 `!= 0` 返回）。
    #[serde(skip_serializing, default)]
    pub unlink_windows_completed: bool,
    /// @+0x1fe：undo 提示完成标志（O：`hint_undo` 0x142284220 读）。
    #[serde(skip_serializing, default)]
    pub undo_completed: bool,
    /// @+0x1ff：roof_shape 提示完成标志（O：`hint_roof_shape` 0x142284ed0 读 `== 0`）。
    #[serde(skip_serializing, default)]
    pub roof_shape_completed: bool,
    /// @+0x200：clutter_focused_edit 提示完成标志（O：`hint_clutter_focused_edit_condition`
    /// 0x141c3d890 读 `!= 0` → 状态 1）。
    #[serde(skip_serializing, default)]
    pub clutter_focused_edit_completed: bool,
    /// @+0x201：楼梯提示阶段 1 完成标志（O：`hint_stairs` 0x142285a20 五阶段状态机）。
    #[serde(skip_serializing, default)]
    pub stairs_stage1_completed: bool,
    /// @+0x202：楼梯提示阶段 2。
    #[serde(skip_serializing, default)]
    pub stairs_stage2_completed: bool,
    /// @+0x203：楼梯提示阶段 3。
    #[serde(skip_serializing, default)]
    pub stairs_stage3_completed: bool,
    /// @+0x204：楼梯提示阶段 4。
    #[serde(skip_serializing, default)]
    pub stairs_stage4_completed: bool,
    /// @+0x205：楼梯提示阶段 5。
    #[serde(skip_serializing, default)]
    pub stairs_stage5_completed: bool,
    /// @+0x206：复位计数/哨兵字节（O：`reset_hints` 两分支都写；未在
    /// tutorial_completion 18 路 switch 中，消费面 U）。
    #[serde(skip_serializing, default)]
    pub hint_reset_flag_206: bool,
}

impl Default for GameProfile {
    fn default() -> Self {
        Self {
            tutorial: false,
            tutorial_cooldowns: TutorialCooldowns::default_cooldowns(),
            player_id: Uuid::new_v4(),
            key_bindings: GameKeyBindings::default(),
            new_glade_count: 0,
            camera_rotate_glyph_mode: 0,
            camera_pan_glyph_mode: 0,
            focus_glyph_mode: 0,
            focused_edit_completed: false,
            camera_pan_completed: false,
            camera_rotate_completed: false,
            camera_zoom_completed: false,
            terrain_layering_completed: false,
            terrain_steepness_completed: false,
            terrain_focus_completed: false,
            disable_snapping_completed: false,
            unlink_windows_completed: false,
            undo_completed: false,
            roof_shape_completed: false,
            clutter_focused_edit_completed: false,
            stairs_stage1_completed: false,
            stairs_stage2_completed: false,
            stairs_stage3_completed: false,
            stairs_stage4_completed: false,
            stairs_stage5_completed: false,
            hint_reset_flag_206: false,
        }
    }
}

impl GameProfile {
    /// Original load_game_profile @140916480: existing profile.ron is read and
    /// decoded; read/parse failure logs and falls back to the real constructor.
    pub fn load_from_disk() -> Self {
        let path = paths::config().join("profile.ron");
        if !path.is_file() {
            log::info!("No profile.ron found. Using defaults.");
            return Self::default();
        }
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) => {
                log::error!(
                    "Failed reading profile.ron. Falling back to defaults. The error was: {error}"
                );
                return Self::default();
            }
        };
        ron::from_str(&text).unwrap_or_else(|error| {
            log::error!(
                "Failed deserializing profile.ron. Falling back to defaults. The error was: {error}"
            );
            Self::default()
        })
    }
    /// 新 glade 计数消费（load_game 0x14146d030 第 3 参 `Res<GameProfile>`，I）。
    pub fn increment_new_glade_count(&mut self) {
        self.new_glade_count = self.new_glade_count.wrapping_add(1);
    }

    /// 标记一个教学操作完成（O：`tutorial_completion` 0x14227c230 的 18 路 switch
    /// 写面；`TutorialOp::completion_flag_offset` 决定写哪个完成标志字节）。
    /// 相机提示三标志的 pan/rotate/zoom 归属见字段注释（U）。
    pub fn mark_tutorial_op_complete(&mut self, op: crate::resources::onboarding::TutorialOp) {
        let offset = op.completion_flag_offset();
        match offset {
            0x1f4 => self.tutorial = true,
            0x1f5 => self.focused_edit_completed = true,
            0x1f6 => self.camera_pan_completed = true,
            0x1f7 => self.camera_rotate_completed = true,
            0x1f8 => self.camera_zoom_completed = true,
            0x1f9 => self.terrain_layering_completed = true,
            0x1fa => self.terrain_steepness_completed = true,
            0x1fb => self.terrain_focus_completed = true,
            0x1fc => self.disable_snapping_completed = true,
            0x1fd => self.unlink_windows_completed = true,
            0x1fe => self.undo_completed = true,
            0x1ff => self.roof_shape_completed = true,
            0x200 => self.clutter_focused_edit_completed = true,
            0x201 => self.stairs_stage1_completed = true,
            0x202 => self.stairs_stage2_completed = true,
            0x203 => self.stairs_stage3_completed = true,
            0x204 => self.stairs_stage4_completed = true,
            0x205 => self.stairs_stage5_completed = true,
            _ => unreachable!("TutorialOp offset"),
        }
    }

    /// 清空提示区（O：`reset_hints` 0x141c1c8d0 事件非空分支：0x1f4..0x206 全清 0、
    /// 计时区 **0x1e0..0x1ef** 清 0、+0x206 清 0）。**+0x1f0（time_since_mouse_moved）
    /// 不清**（reviewer §2 登记：二进制清零分支只写两个 u64 到 0x1e0/0x1e8）。
    /// `tutorial`（0x1f4）一并清 0。
    pub fn reset_hint_flags(&mut self) {
        self.tutorial = false;
        self.focused_edit_completed = false;
        self.camera_pan_completed = false;
        self.camera_rotate_completed = false;
        self.camera_zoom_completed = false;
        self.terrain_layering_completed = false;
        self.terrain_steepness_completed = false;
        self.terrain_focus_completed = false;
        self.disable_snapping_completed = false;
        self.unlink_windows_completed = false;
        self.undo_completed = false;
        self.roof_shape_completed = false;
        self.clutter_focused_edit_completed = false;
        self.stairs_stage1_completed = false;
        self.stairs_stage2_completed = false;
        self.stairs_stage3_completed = false;
        self.stairs_stage4_completed = false;
        self.stairs_stage5_completed = false;
        self.hint_reset_flag_206 = false;
        // 0x1e0..0x1ef：两对计时/冷却 → 0（不动 +0x1f0 累加器）。
        let tc = &mut self.tutorial_cooldowns;
        tc.timer_a = 0.0;
        tc.cooldown_a = 0.0;
        tc.timer_b = 0.0;
        tc.cooldown_b = 0.0;
    }

    /// 置位整个完成标志区（O：`reset_hints` 事件==0 分支：0x1f4..0x206 全置 1）。
    pub fn set_all_hint_flags(&mut self) {
        self.tutorial = true;
        self.focused_edit_completed = true;
        self.camera_pan_completed = true;
        self.camera_rotate_completed = true;
        self.camera_zoom_completed = true;
        self.terrain_layering_completed = true;
        self.terrain_steepness_completed = true;
        self.terrain_focus_completed = true;
        self.disable_snapping_completed = true;
        self.unlink_windows_completed = true;
        self.undo_completed = true;
        self.roof_shape_completed = true;
        self.clutter_focused_edit_completed = true;
        self.stairs_stage1_completed = true;
        self.stairs_stage2_completed = true;
        self.stairs_stage3_completed = true;
        self.stairs_stage4_completed = true;
        self.stairs_stage5_completed = true;
        self.hint_reset_flag_206 = true;
    }
}

/// Original PreStartup producer @140916480 uses Commands.insert_resource.
pub fn load_game_profile(mut commands: bevy_ecs::system::Commands) {
    commands.insert_resource(GameProfile::load_from_disk());
}

/// Canonical request consumed by the original on_event condition, set43.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SaveGameProfileCmd;

/// B write_profile_via_tmp @140917360. Keep tempfile/ron implementations;
/// the existing config-directory producer owns directory creation.
pub fn write_profile_via_tmp(profile: &GameProfile) -> anyhow::Result<()> {
    write_profile_in_directory(profile, &paths::config())
}

fn write_profile_in_directory(
    profile: &GameProfile,
    directory: &std::path::Path,
) -> anyhow::Result<()> {
    use std::io::Write;
    let mut temporary = tempfile::Builder::new()
        .prefix(".tmp")
        .tempfile_in(directory)?;
    // Original Options default recursion limit128 and PrettyConfig::default;
    // serialization failure unwraps, while filesystem failure returns/logs.
    let text = ron::Options::default()
        .to_string_pretty(profile, ron::ser::PrettyConfig::default())
        .expect("serializing GameProfile to RON");
    temporary.write_all(text.as_bytes())?;
    temporary.persist(directory.join("profile.ron"))?;
    Ok(())
}

/// Native save_game_profile @140917230: one current profile write, log Err.
/// The on_event<SaveGameProfileCmd> reader belongs to the registration condition.
pub fn save_game_profile(profile: bevy_ecs::change_detection::Res<GameProfile>) {
    if let Err(error) = write_profile_via_tmp(&profile) {
        log::error!(target: "country_core::game_profile", "Error writing profile.ron: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_profile_tempfile_write_replaces_the_file_with_the_loaded_binding_schema() {
        use bevy_input::{keyboard::KeyCode, mouse::MouseButton};
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("profile.ron"), "previous contents").unwrap();
        let mut profile = GameProfile::default();
        profile.key_bindings.bindings[21] = Some(KeyCode::KeyU);
        profile.key_bindings.mouse[2] = Some(MouseButton::Left);
        profile.new_glade_count = 7;
        write_profile_in_directory(&profile, directory.path()).unwrap();
        let saved = std::fs::read_to_string(directory.path().join("profile.ron")).unwrap();
        let loaded: GameProfile = ron::from_str(&saved).unwrap();
        assert_eq!(loaded.key_bindings, profile.key_bindings);
        assert_eq!(loaded.player_id, profile.player_id);
        assert_eq!(loaded.new_glade_count, 7);
        assert_eq!(
            std::fs::read_dir(directory.path()).unwrap().count(),
            1,
            "successful persist must not leave a temporary file"
        );
        assert!(write_profile_in_directory(&profile, &directory.path().join("missing")).is_err());
    }

    #[test]
    fn native_binding_schema_preserves_unbound_keys_and_left_mouse() {
        use bevy_input::{keyboard::KeyCode, mouse::MouseButton};
        let mut bindings = GameKeyBindings::default();
        bindings.bindings[21] = Some(KeyCode::KeyU);
        bindings.bindings[36] = None;
        bindings.mouse = [Some(MouseButton::Left), None, Some(MouseButton::Other(12))];
        let text = ron::to_string(&bindings).unwrap();
        assert!(text.contains("undo:Some(KeyU)"));
        assert!(text.contains("mouse_rotate:Some(Left)"));
        assert!(text.contains("mouse_pan:None"));
        assert!(text.contains("experimental_camera:None"));
        assert_eq!(ron::from_str::<GameKeyBindings>(&text).unwrap(), bindings);
        assert!(ron::from_str::<GameKeyBindings>("[(42,0)]").is_err());
    }

    #[test]
    fn native_default_binding_identifiers_match_the_desktop_input_codes() {
        let bindings = GameKeyBindings::default();
        assert_eq!(
            desktop_keycode_index(bindings.bindings[0].unwrap()).unwrap(),
            0x2a
        );
        assert_eq!(
            desktop_keycode_index(bindings.bindings[21].unwrap()).unwrap(),
            0x2d
        );
        assert!(bindings.bindings[36].is_none());
        assert!(
            desktop_keycode_index(bevy_input::keyboard::KeyCode::Unidentified(
                bevy_input::keyboard::NativeKeyCode::MacOS(12)
            ))
            .is_err()
        );
    }

    #[test]
    fn profile_ron_writes_player_id_and_aliases_legacy_installation_id() {
        let mut bindings = GameKeyBindings::default();
        bindings.bindings[0] = Some(bevy_input::keyboard::KeyCode::KeyX);
        let profile = GameProfile {
            key_bindings: bindings,
            new_glade_count: 3,
            tutorial_cooldowns: TutorialCooldowns::default_cooldowns(),
            tutorial: true,
            player_id: Uuid::new_v4(),
            ..GameProfile::default()
        };
        let text = ron::to_string(&profile).unwrap();
        // player_id 序列化（第 3 位）；game_installation_id 不是独立序列化字段。
        assert!(text.contains("player_id"));
        assert!(!text.contains("game_installation_id"));
        assert!(text.contains("key_bindings"));
        assert!(text.contains("new_glade_count"));
        let back: GameProfile = ron::from_str(&text).unwrap();
        assert_eq!(back.new_glade_count, 3);
        assert_eq!(back.key_bindings, profile.key_bindings);
        assert_eq!(back.player_id, profile.player_id);

        // 旧档别名：ron 字段 `game_installation_id` 反序列化到 player_id
        // （O：deserialize 0x1404b0ed0 名表 case 0x14 20 字符别名）。
        let legacy = ron::to_string(&profile)
            .unwrap()
            .replace("player_id:", "game_installation_id:");
        let back_legacy: GameProfile = ron::from_str(&legacy).unwrap();
        assert_eq!(back_legacy.player_id, profile.player_id);
    }

    #[test]
    fn default_profile_has_a_generated_player_id() {
        let profile = GameProfile::default();
        assert_ne!(profile.player_id, Uuid::nil());
        assert_eq!(profile.new_glade_count, 0);
    }

    #[test]
    fn hint_region_flags_roundtrip_the_binary_offsets() {
        // 新增提示区字段不进 ron 序列化，但 reset/置位辅助与二进制写面一致。
        let mut profile = GameProfile::default();
        profile.set_all_hint_flags();
        assert!(profile.tutorial);
        assert!(profile.focused_edit_completed);
        assert!(profile.stairs_stage5_completed);
        assert!(profile.hint_reset_flag_206);
        profile.reset_hint_flags();
        assert!(!profile.tutorial);
        assert!(!profile.focused_edit_completed);
        // 二进制清零分支：0x1e0..0x1ef（两对计时/冷却）→ 0；+0x1f0 累加器不清。
        assert_eq!(profile.tutorial_cooldowns.cooldown_a, 0.0);
        assert_eq!(profile.tutorial_cooldowns.cooldown_b, 0.0);

        // TutorialOp → 偏移逐项（与 onboarding::tests 同步）。
        use crate::resources::onboarding::TutorialOp;
        profile.mark_tutorial_op_complete(TutorialOp::Tutorial);
        profile.mark_tutorial_op_complete(TutorialOp::FocusedEdit);
        profile.mark_tutorial_op_complete(TutorialOp::Undo);
        profile.mark_tutorial_op_complete(TutorialOp::StairsStage5);
        assert!(profile.tutorial);
        assert!(profile.focused_edit_completed);
        assert!(profile.undo_completed);
        assert!(profile.stairs_stage5_completed);
        assert!(!profile.terrain_focus_completed);
    }
}

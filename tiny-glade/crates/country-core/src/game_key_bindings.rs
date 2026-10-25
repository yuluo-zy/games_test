//! B 原 `game_key_bindings.rs`：动作身份、显示描述和权威档案改键规则。
//! 恢复行为而非旧ABI；键位仍存于 `GameProfile.key_bindings`，供原输入管线消费。
use crate::game_profile::GameKeyBindings;
use bevy_input::{keyboard::KeyCode, mouse::MouseButton};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyBinding {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBack,
    RotateLeft,
    RotateRight,
    ZoomIn,
    ZoomOut,
    Building,
    Path,
    Decorators,
    Customize,
    Eraser,
    Terrain,
    Garden,
    Tree,
    Water,
    Photomode,
    TimeOfDay,
    Undo,
    Redo,
    FocusEdit,
    DisableSnapping,
    Screenshot,
    Feedback,
    HideUiPhotomode,
    UiCancel,
    UiConfirm,
    UiLeft,
    UiRight,
    UiDown,
    UiUp,
    UiTabPrev,
    UiTabNext,
    ExperimentalCamera,
}

/// 原动作描述可为本地化文字（可附方向）或工具图标；不把图标名当显示文字。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionDescription {
    Text(&'static str),
    Localized {
        key: &'static str,
        suffix: &'static str,
    },
    Image(&'static str),
}
impl KeyBinding {
    pub const ALL: [Self; 37] = [
        Self::MoveUp,
        Self::MoveDown,
        Self::MoveLeft,
        Self::MoveRight,
        Self::MoveForward,
        Self::MoveBack,
        Self::RotateLeft,
        Self::RotateRight,
        Self::ZoomIn,
        Self::ZoomOut,
        Self::Building,
        Self::Path,
        Self::Decorators,
        Self::Customize,
        Self::Eraser,
        Self::Terrain,
        Self::Garden,
        Self::Tree,
        Self::Water,
        Self::Photomode,
        Self::TimeOfDay,
        Self::Undo,
        Self::Redo,
        Self::FocusEdit,
        Self::DisableSnapping,
        Self::Screenshot,
        Self::Feedback,
        Self::HideUiPhotomode,
        Self::UiCancel,
        Self::UiConfirm,
        Self::UiLeft,
        Self::UiRight,
        Self::UiDown,
        Self::UiUp,
        Self::UiTabPrev,
        Self::UiTabNext,
        Self::ExperimentalCamera,
    ];
    pub fn from_native_tag(tag: u8) -> Option<Self> {
        Self::ALL.get(usize::from(tag)).copied()
    }
    /// B get_keycode跳表；前六个动作枚举顺序与档案字段顺序不同。
    pub const fn profile_slot(self) -> usize {
        const SLOTS: [usize; 37] = [
            4, 5, 2, 3, 0, 1, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
        ];
        SLOTS[self as usize]
    }
    pub fn get_keycode(self, bindings: &GameKeyBindings) -> Option<KeyCode> {
        bindings.bindings[self.profile_slot()]
    }
    pub fn action_desc(self) -> ActionDescription {
        use ActionDescription::*;
        match self {
            Self::MoveUp => Localized {
                key: "KeyBindings.MoveUp",
                suffix: "",
            },
            Self::MoveDown => Localized {
                key: "KeyBindings.MoveDown",
                suffix: "",
            },
            Self::MoveLeft => Localized {
                key: "KeyBindings.Move",
                suffix: " ⬅️",
            },
            Self::MoveRight => Localized {
                key: "KeyBindings.Move",
                suffix: " ➡️",
            },
            Self::MoveForward => Localized {
                key: "KeyBindings.Move",
                suffix: " ⬆️",
            },
            Self::MoveBack => Localized {
                key: "KeyBindings.Move",
                suffix: " ⬇️",
            },
            Self::RotateLeft => Localized {
                key: "KeyBindings.Rotate",
                suffix: " ⬅️",
            },
            Self::RotateRight => Localized {
                key: "KeyBindings.Rotate",
                suffix: " ➡️",
            },
            Self::ZoomIn => Localized {
                key: "KeyBindings.ZoomIn",
                suffix: "",
            },
            Self::ZoomOut => Localized {
                key: "KeyBindings.ZoomOut",
                suffix: "",
            },
            Self::Building => Image("toolbar/building"),
            Self::Path => Image("toolbar/path"),
            Self::Decorators => Image("toolbar/decorators"),
            Self::Customize => Image("toolbar/customization"),
            Self::Eraser => Image("toolbar/eraser"),
            Self::Terrain => Image("toolbar/terrain"),
            Self::Garden => Image("toolbar/garden"),
            Self::Tree => Image("toolbar/tree"),
            Self::Water => Image("toolbar/water"),
            Self::Photomode => Image("toolbar/photomode"),
            Self::TimeOfDay => Image("toolbar/time_of_day"),
            Self::Undo => Localized {
                key: "KeyBindings.Undo",
                suffix: "",
            },
            Self::Redo => Localized {
                key: "KeyBindings.Redo",
                suffix: "",
            },
            Self::FocusEdit => Localized {
                key: "KeyBindings.FocusOrDeselect",
                suffix: "",
            },
            Self::DisableSnapping => Localized {
                key: "KeyBindings.DisableSnapping",
                suffix: "",
            },
            Self::Screenshot => Localized {
                key: "KeyBindings.TakeScreenshot",
                suffix: "",
            },
            Self::Feedback => Localized {
                key: "KeyBindings.SendFeedback",
                suffix: "",
            },
            Self::HideUiPhotomode => Localized {
                key: "KeyBindings.HideUiInPhotoMode",
                suffix: "",
            },
            Self::UiCancel => Text("UI Cancel"),
            Self::UiConfirm => Text("UI Confirm"),
            Self::UiLeft => Text("UI Left"),
            Self::UiRight => Text("UI Right"),
            Self::UiDown => Text("UI Down"),
            Self::UiUp => Text("UI Up"),
            Self::UiTabPrev => Text("UI Tab Prev"),
            Self::UiTabNext => Text("UI Tab Next"),
            Self::ExperimentalCamera => Text("(WIP) Camera snapshots"),
        }
    }
}
impl GameKeyBindings {
    /// 原14089b2a0：先清35个字段的相同键，再写目标。
    /// water与experimental_camera不参与清重，鼠标也不参与；保留原非对称规则。
    pub fn assign_key(&mut self, action: KeyBinding, key: Option<KeyCode>) {
        for (slot, value) in self.bindings.iter_mut().enumerate() {
            if slot != 18 && slot != 36 && *value == key {
                *value = None;
            }
        }
        self.bindings[action.profile_slot()] = key;
    }
    /// 原14089b280只写目标，允许多个鼠标动作共用同一键。
    pub fn assign_mouse_button(&mut self, action: MouseBinding, button: Option<MouseButton>) {
        self.mouse[action as usize] = button;
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MouseBinding {
    Rotate,
    Pan,
    FocusEdit,
}
impl MouseBinding {
    pub const ALL: [Self; 3] = [Self::Rotate, Self::Pan, Self::FocusEdit];
    pub fn get_mouse_button(self, bindings: &GameKeyBindings) -> Option<MouseButton> {
        bindings.mouse[self as usize]
    }
    pub fn action_desc(self) -> ActionDescription {
        ActionDescription::Localized {
            key: match self {
                Self::Rotate => "KeyBindings.Rotate",
                Self::Pan => "KeyBindings.Move",
                Self::FocusEdit => "KeyBindings.FocusOrDeselect",
            },
            suffix: "",
        }
    }
}
/// 原14089c970，包括额外鼠标键的数字，不缩成未知图标。
pub fn mouse_button_to_string(button: MouseButton) -> String {
    match button {
        MouseButton::Left => "LMB".into(),
        MouseButton::Right => "RMB".into(),
        MouseButton::Middle => "MMB".into(),
        MouseButton::Back => "Back mouse button".into(),
        MouseButton::Forward => "Forward mouse button".into(),
        MouseButton::Other(number) => format!("Mouse button {number}"),
    }
}
/// 原14089d490两个静态表逐项提取；None由调用方显示?，表外Some显示???。
/// 原表的AltRight文字就是Right Right，保留原文而非凭经验修正。
pub fn keycode_to_str(key: KeyCode) -> Option<&'static str> {
    use KeyCode::*;
    Some(match key {
        MediaSelect => "MediaSelect",
        MediaStop => "MediaStop",
        Power => "Power",
        Sleep => "Sleep",
        Copy => "Copy",
        Cut => "Cut",
        Paste => "Paste",
        F1 => "F1",
        F2 => "F2",
        F3 => "F3",
        F4 => "F4",
        F5 => "F5",
        F6 => "F6",
        F7 => "F7",
        F8 => "F8",
        F9 => "F9",
        F10 => "F10",
        F11 => "F11",
        F12 => "F12",
        F13 => "F13",
        F14 => "F14",
        F15 => "F15",
        F16 => "F16",
        F17 => "F17",
        F18 => "F18",
        F19 => "F19",
        F20 => "F20",
        F21 => "F21",
        F22 => "F22",
        F23 => "F23",
        F24 => "F24",
        Backslash => "\\",
        BracketLeft => "[",
        BracketRight => "]",
        Comma => ",",
        Digit0 => "0",
        Digit1 => "1",
        Digit2 => "2",
        Digit3 => "3",
        Digit4 => "4",
        Digit5 => "5",
        Digit6 => "6",
        Digit7 => "7",
        Digit8 => "8",
        Digit9 => "9",
        KeyA => "A",
        KeyB => "B",
        KeyC => "C",
        KeyD => "D",
        KeyE => "E",
        KeyF => "F",
        KeyG => "G",
        KeyH => "H",
        KeyI => "I",
        KeyJ => "J",
        KeyK => "K",
        KeyL => "L",
        KeyM => "M",
        KeyN => "N",
        KeyO => "O",
        KeyP => "P",
        KeyQ => "Q",
        KeyR => "R",
        KeyS => "S",
        KeyT => "T",
        KeyU => "U",
        KeyV => "V",
        KeyW => "W",
        KeyX => "X",
        KeyY => "Y",
        KeyZ => "Z",
        Minus => "-",
        Period => ".",
        Quote => "'",
        Semicolon => ";",
        Slash => "/",
        AltLeft => "Left Alt",
        AltRight => "Right Right",
        Backspace => "Backspace",
        ControlLeft => "Left Control",
        ControlRight => "Right Control",
        Enter => "Enter",
        SuperLeft => "Left Super",
        SuperRight => "Right Super",
        ShiftLeft => "Left Shift",
        ShiftRight => "Right Shift",
        Space => "Space",
        Tab => "Tab",
        Convert => "Convert",
        Delete => "Delete",
        End => "End",
        Home => "Home",
        Insert => "Insert",
        PageDown => "PageDown",
        PageUp => "PageUp",
        ArrowDown => "Down",
        ArrowLeft => "Left",
        ArrowRight => "Right",
        ArrowUp => "Up",
        Numpad0 => "Numpad0",
        Numpad1 => "Numpad1",
        Numpad2 => "Numpad2",
        Numpad3 => "Numpad3",
        Numpad4 => "Numpad4",
        Numpad5 => "Numpad5",
        Numpad6 => "Numpad6",
        Numpad7 => "Numpad7",
        Numpad8 => "Numpad8",
        Numpad9 => "Numpad9",
        NumpadAdd => "NumpadAdd",
        NumpadComma => "NumpadComma",
        NumpadDecimal => "NumpadDecimal",
        NumpadDivide => "NumpadDivide",
        NumpadEnter => "NumpadEnter",
        NumpadMultiply => "NumpadMultiply",
        NumpadSubtract => "NumpadSubtract",
        Escape => "Escape",
        Pause => "Pause",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn key(tag: usize) -> Option<KeyCode> {
        use KeyCode::*;
        const KEYS: [Option<KeyCode>; 195] = [
            Some(Backquote),
            Some(Backslash),
            Some(BracketLeft),
            Some(BracketRight),
            Some(Comma),
            Some(Digit0),
            Some(Digit1),
            Some(Digit2),
            Some(Digit3),
            Some(Digit4),
            Some(Digit5),
            Some(Digit6),
            Some(Digit7),
            Some(Digit8),
            Some(Digit9),
            Some(Equal),
            Some(IntlBackslash),
            Some(IntlRo),
            Some(IntlYen),
            Some(KeyA),
            Some(KeyB),
            Some(KeyC),
            Some(KeyD),
            Some(KeyE),
            Some(KeyF),
            Some(KeyG),
            Some(KeyH),
            Some(KeyI),
            Some(KeyJ),
            Some(KeyK),
            Some(KeyL),
            Some(KeyM),
            Some(KeyN),
            Some(KeyO),
            Some(KeyP),
            Some(KeyQ),
            Some(KeyR),
            Some(KeyS),
            Some(KeyT),
            Some(KeyU),
            Some(KeyV),
            Some(KeyW),
            Some(KeyX),
            Some(KeyY),
            Some(KeyZ),
            Some(Minus),
            Some(Period),
            Some(Quote),
            Some(Semicolon),
            Some(Slash),
            Some(AltLeft),
            Some(AltRight),
            Some(Backspace),
            Some(CapsLock),
            Some(ContextMenu),
            Some(ControlLeft),
            Some(ControlRight),
            Some(Enter),
            Some(SuperLeft),
            Some(SuperRight),
            Some(ShiftLeft),
            Some(ShiftRight),
            Some(Space),
            Some(Tab),
            Some(Convert),
            Some(KanaMode),
            Some(Lang1),
            Some(Lang2),
            Some(Lang3),
            Some(Lang4),
            Some(Lang5),
            Some(NonConvert),
            Some(Delete),
            Some(End),
            Some(Help),
            Some(Home),
            Some(Insert),
            Some(PageDown),
            Some(PageUp),
            Some(ArrowDown),
            Some(ArrowLeft),
            Some(ArrowRight),
            Some(ArrowUp),
            Some(NumLock),
            Some(Numpad0),
            Some(Numpad1),
            Some(Numpad2),
            Some(Numpad3),
            Some(Numpad4),
            Some(Numpad5),
            Some(Numpad6),
            Some(Numpad7),
            Some(Numpad8),
            Some(Numpad9),
            Some(NumpadAdd),
            Some(NumpadBackspace),
            Some(NumpadClear),
            Some(NumpadClearEntry),
            Some(NumpadComma),
            Some(NumpadDecimal),
            Some(NumpadDivide),
            Some(NumpadEnter),
            Some(NumpadEqual),
            Some(NumpadHash),
            Some(NumpadMemoryAdd),
            Some(NumpadMemoryClear),
            Some(NumpadMemoryRecall),
            Some(NumpadMemoryStore),
            Some(NumpadMemorySubtract),
            Some(NumpadMultiply),
            Some(NumpadParenLeft),
            Some(NumpadParenRight),
            Some(NumpadStar),
            Some(NumpadSubtract),
            Some(Escape),
            Some(Fn),
            Some(FnLock),
            Some(PrintScreen),
            Some(ScrollLock),
            Some(Pause),
            Some(BrowserBack),
            Some(BrowserFavorites),
            Some(BrowserForward),
            Some(BrowserHome),
            Some(BrowserRefresh),
            Some(BrowserSearch),
            Some(BrowserStop),
            Some(Eject),
            Some(LaunchApp1),
            Some(LaunchApp2),
            Some(LaunchMail),
            Some(MediaPlayPause),
            Some(MediaSelect),
            Some(MediaStop),
            Some(MediaTrackNext),
            Some(MediaTrackPrevious),
            Some(Power),
            Some(Sleep),
            Some(AudioVolumeDown),
            Some(AudioVolumeMute),
            Some(AudioVolumeUp),
            Some(WakeUp),
            Some(Meta),
            Some(Hyper),
            Some(Turbo),
            Some(Abort),
            Some(Resume),
            Some(Suspend),
            Some(Again),
            Some(Copy),
            Some(Cut),
            Some(Find),
            Some(Open),
            Some(Paste),
            Some(Props),
            Some(Select),
            Some(Undo),
            Some(Hiragana),
            Some(Katakana),
            Some(F1),
            Some(F2),
            Some(F3),
            Some(F4),
            Some(F5),
            Some(F6),
            Some(F7),
            Some(F8),
            Some(F9),
            Some(F10),
            Some(F11),
            Some(F12),
            Some(F13),
            Some(F14),
            Some(F15),
            Some(F16),
            Some(F17),
            Some(F18),
            Some(F19),
            Some(F20),
            Some(F21),
            Some(F22),
            Some(F23),
            Some(F24),
            Some(F25),
            Some(F26),
            Some(F27),
            Some(F28),
            Some(F29),
            Some(F30),
            Some(F31),
            Some(F32),
            Some(F33),
            Some(F34),
            Some(F35),
            None,
        ];
        KEYS[tag - 1]
    }
    #[test]
    fn original_assign_key_all_actions_named_keys_and_collision_profiles() {
        let rows: Vec<(u8, usize, u8, Vec<usize>)> = ron::from_str(include_str!(
            "../../../tests/fixtures/native/toolbar-keybindings-assign.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 14430);
        for (action, incoming, pattern, expected) in rows {
            let mut bindings = GameKeyBindings::default();
            for (i, value) in bindings.bindings.iter_mut().enumerate() {
                *value = key(if pattern == 0 || i % 3 == 0 {
                    incoming
                } else if i % 3 == 1 {
                    195
                } else {
                    (i * 7) % 194 + 1
                });
            }
            bindings.assign_key(KeyBinding::from_native_tag(action).unwrap(), key(incoming));
            for (slot, expected) in expected.into_iter().enumerate() {
                assert_eq!(
                    bindings.bindings[slot],
                    key(expected),
                    "action={action},key={incoming},pattern={pattern},slot={slot}"
                );
            }
        }
    }
    #[test]
    fn original_native_payload_assignment_distinguishes_platform_and_value() {
        use bevy_input::keyboard::NativeKeyCode as N;
        let native = |kind, payload| {
            KeyCode::Unidentified(match kind {
                0 => N::Unidentified,
                1 => N::Android(payload),
                2 => N::MacOS(payload as u16),
                3 => N::Windows(payload as u16),
                4 => N::Xkb(payload),
                _ => unreachable!(),
            })
        };
        let encode = |key: Option<KeyCode>| match key {
            None => 0,
            Some(KeyCode::Unidentified(n)) => match n {
                N::Unidentified => 1u64 << 32,
                N::Android(p) => (2u64 << 32) | u64::from(p),
                N::MacOS(p) => (3u64 << 32) | u64::from(p),
                N::Windows(p) => (4u64 << 32) | u64::from(p),
                N::Xkb(p) => (5u64 << 32) | u64::from(p),
            },
            _ => unreachable!(),
        };
        let rows: Vec<(u8, u8, u32, u8, Vec<u64>)> = ron::from_str(include_str!(
            "../../../tests/fixtures/native/toolbar-keybindings-native-keys.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 1628);
        for (action, kind, payload, pattern, expected) in rows {
            let mut bindings = GameKeyBindings::default();
            for (i, value) in bindings.bindings.iter_mut().enumerate() {
                let (k, p) = if pattern == 0 || i % 3 == 0 {
                    (kind, payload)
                } else if i % 3 == 1 {
                    ((kind + 1) % 5, payload)
                } else {
                    (kind, payload.wrapping_add(1))
                };
                *value = Some(native(k, p));
            }
            bindings.assign_key(
                KeyBinding::from_native_tag(action).unwrap(),
                Some(native(kind, payload)),
            );
            for (slot, want) in expected.into_iter().enumerate() {
                assert_eq!(
                    encode(bindings.bindings[slot]),
                    want,
                    "原native赋值 action={action},kind={kind},payload={payload},pattern={pattern},slot={slot}"
                );
            }
        }
    }
    #[test]
    fn original_key_names_all_named_keycodes() {
        let rows: Vec<(usize, Option<String>)> = ron::from_str(include_str!(
            "../../../tests/fixtures/native/toolbar-keybindings-labels.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 194);
        for (tag, expected) in rows {
            assert_eq!(
                keycode_to_str(key(tag).unwrap()),
                expected.as_deref(),
                "原KeyCode显示tag={tag}"
            );
        }
    }
    #[test]
    fn mouse_assignment_retains_shared_bindings_and_profile_roundtrip() {
        let mut bindings = GameKeyBindings::default();
        bindings.assign_mouse_button(MouseBinding::Pan, Some(MouseButton::Right));
        assert_eq!(bindings.mouse, [Some(MouseButton::Right); 3]);
        bindings.assign_key(KeyBinding::MoveForward, Some(KeyCode::ArrowUp));
        let text = ron::to_string(&bindings).unwrap();
        let loaded: GameKeyBindings = ron::from_str(&text).unwrap();
        assert_eq!(bindings, loaded);
        assert_eq!(
            KeyBinding::MoveForward.get_keycode(&loaded),
            Some(KeyCode::ArrowUp)
        );
    }
}

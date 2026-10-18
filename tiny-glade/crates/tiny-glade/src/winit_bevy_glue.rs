//! `tiny_glade::winit_bevy_glue` — winit → Bevy input translation.
//!
//! Reconstructed from B `crates/tiny-glade/src/winit_bevy_glue.rs`.
//! VA `0x1400fb850` (`tiny_glade::winit_bevy_glue::winit_to_bevy_keycode`).
//!
//! The fresh disassembly takes a `&winit::event::KeyEvent` in RDX and a
//! return-by-sret pointer in RCX. `winit` 0.29 `KeyEvent` lays out as
//! `logical_key @0x00`, `physical_key @0x58` (verified with
//! `std::mem::offset_of!`), so:
//! * `word [event+0x58] == 5` is `PhysicalKey::Code` (the niche encoding of
//!   `PhysicalKey`); the byte at `+0x5a` is the winit `KeyCode` discriminant;
//! * the 256-entry table at `0x14294c58c` maps entry `b ^ 0x80` to
//!   `b + 1` for `b <= 193`, else `1` (`Unidentified`). That is exactly the
//!   one-to-one variant ordering between winit 0.29 `KeyCode` and
//!   `bevy_input` 0.16 `KeyCode` (Bevy prepends `Unidentified`), so the literal
//!   table is expressed as an exhaustive name match;
//! * the non-`Code` path copies the 8-byte winit `NativeKeyCode` payload into
//!   the Bevy `NativeKeyCode` payload. Both enums have the identical five
//!   variants (`Unidentified`, `Android(u32)`, `MacOS(u16)`, `Windows(u16)`,
//!   `Xkb(u32)`), so the machine-code bit copy is the explicit `match` below.
//!
//! restorer-tinyglade-E fresh 复核（0x1400fb850，2026-10-17）逐语句一致：
//! `*(u16*)(event+0x58)==5`（`PhysicalKey::Code` niche）→ 表
//! `0x14294c58c[b^0x80]` 取 u32；否则 tag=0 + 从 `+0x58` 拷 8 B 载荷。
//! 本模块只有这一支业务函数；`winit_code_to_bevy_keycode` 是模块内测试/复用
//! helper（非原函数，等价于同一张表），不计业务函数。

use bevy_input::keyboard::{KeyCode, NativeKeyCode};
use winit::keyboard::{KeyCode as WinitKeyCode, NativeKeyCode as WinitNativeKeyCode, PhysicalKey};

/// Translate a winit key event's physical key to a Bevy key code.
pub fn winit_to_bevy_keycode(event: &winit::event::KeyEvent) -> KeyCode {
    match event.physical_key {
        PhysicalKey::Code(code) => map_winit_code(code),
        PhysicalKey::Unidentified(native) => KeyCode::Unidentified(match native {
            WinitNativeKeyCode::Unidentified => NativeKeyCode::Unidentified,
            WinitNativeKeyCode::Android(code) => NativeKeyCode::Android(code),
            WinitNativeKeyCode::MacOS(code) => NativeKeyCode::MacOS(code),
            WinitNativeKeyCode::Windows(code) => NativeKeyCode::Windows(code),
            WinitNativeKeyCode::Xkb(code) => NativeKeyCode::Xkb(code),
        }),
    }
}

/// Convenience overload for callers that already have a winit `KeyCode`
/// (module helper, not an original B function).
#[cfg(test)]
fn winit_code_to_bevy_keycode(code: WinitKeyCode) -> KeyCode {
    map_winit_code(code)
}

fn map_winit_code(code: WinitKeyCode) -> KeyCode {
    match code {
        winit::keyboard::KeyCode::Backquote => bevy_input::keyboard::KeyCode::Backquote,
        winit::keyboard::KeyCode::Backslash => bevy_input::keyboard::KeyCode::Backslash,
        winit::keyboard::KeyCode::BracketLeft => bevy_input::keyboard::KeyCode::BracketLeft,
        winit::keyboard::KeyCode::BracketRight => bevy_input::keyboard::KeyCode::BracketRight,
        winit::keyboard::KeyCode::Comma => bevy_input::keyboard::KeyCode::Comma,
        winit::keyboard::KeyCode::Digit0 => bevy_input::keyboard::KeyCode::Digit0,
        winit::keyboard::KeyCode::Digit1 => bevy_input::keyboard::KeyCode::Digit1,
        winit::keyboard::KeyCode::Digit2 => bevy_input::keyboard::KeyCode::Digit2,
        winit::keyboard::KeyCode::Digit3 => bevy_input::keyboard::KeyCode::Digit3,
        winit::keyboard::KeyCode::Digit4 => bevy_input::keyboard::KeyCode::Digit4,
        winit::keyboard::KeyCode::Digit5 => bevy_input::keyboard::KeyCode::Digit5,
        winit::keyboard::KeyCode::Digit6 => bevy_input::keyboard::KeyCode::Digit6,
        winit::keyboard::KeyCode::Digit7 => bevy_input::keyboard::KeyCode::Digit7,
        winit::keyboard::KeyCode::Digit8 => bevy_input::keyboard::KeyCode::Digit8,
        winit::keyboard::KeyCode::Digit9 => bevy_input::keyboard::KeyCode::Digit9,
        winit::keyboard::KeyCode::Equal => bevy_input::keyboard::KeyCode::Equal,
        winit::keyboard::KeyCode::IntlBackslash => bevy_input::keyboard::KeyCode::IntlBackslash,
        winit::keyboard::KeyCode::IntlRo => bevy_input::keyboard::KeyCode::IntlRo,
        winit::keyboard::KeyCode::IntlYen => bevy_input::keyboard::KeyCode::IntlYen,
        winit::keyboard::KeyCode::KeyA => bevy_input::keyboard::KeyCode::KeyA,
        winit::keyboard::KeyCode::KeyB => bevy_input::keyboard::KeyCode::KeyB,
        winit::keyboard::KeyCode::KeyC => bevy_input::keyboard::KeyCode::KeyC,
        winit::keyboard::KeyCode::KeyD => bevy_input::keyboard::KeyCode::KeyD,
        winit::keyboard::KeyCode::KeyE => bevy_input::keyboard::KeyCode::KeyE,
        winit::keyboard::KeyCode::KeyF => bevy_input::keyboard::KeyCode::KeyF,
        winit::keyboard::KeyCode::KeyG => bevy_input::keyboard::KeyCode::KeyG,
        winit::keyboard::KeyCode::KeyH => bevy_input::keyboard::KeyCode::KeyH,
        winit::keyboard::KeyCode::KeyI => bevy_input::keyboard::KeyCode::KeyI,
        winit::keyboard::KeyCode::KeyJ => bevy_input::keyboard::KeyCode::KeyJ,
        winit::keyboard::KeyCode::KeyK => bevy_input::keyboard::KeyCode::KeyK,
        winit::keyboard::KeyCode::KeyL => bevy_input::keyboard::KeyCode::KeyL,
        winit::keyboard::KeyCode::KeyM => bevy_input::keyboard::KeyCode::KeyM,
        winit::keyboard::KeyCode::KeyN => bevy_input::keyboard::KeyCode::KeyN,
        winit::keyboard::KeyCode::KeyO => bevy_input::keyboard::KeyCode::KeyO,
        winit::keyboard::KeyCode::KeyP => bevy_input::keyboard::KeyCode::KeyP,
        winit::keyboard::KeyCode::KeyQ => bevy_input::keyboard::KeyCode::KeyQ,
        winit::keyboard::KeyCode::KeyR => bevy_input::keyboard::KeyCode::KeyR,
        winit::keyboard::KeyCode::KeyS => bevy_input::keyboard::KeyCode::KeyS,
        winit::keyboard::KeyCode::KeyT => bevy_input::keyboard::KeyCode::KeyT,
        winit::keyboard::KeyCode::KeyU => bevy_input::keyboard::KeyCode::KeyU,
        winit::keyboard::KeyCode::KeyV => bevy_input::keyboard::KeyCode::KeyV,
        winit::keyboard::KeyCode::KeyW => bevy_input::keyboard::KeyCode::KeyW,
        winit::keyboard::KeyCode::KeyX => bevy_input::keyboard::KeyCode::KeyX,
        winit::keyboard::KeyCode::KeyY => bevy_input::keyboard::KeyCode::KeyY,
        winit::keyboard::KeyCode::KeyZ => bevy_input::keyboard::KeyCode::KeyZ,
        winit::keyboard::KeyCode::Minus => bevy_input::keyboard::KeyCode::Minus,
        winit::keyboard::KeyCode::Period => bevy_input::keyboard::KeyCode::Period,
        winit::keyboard::KeyCode::Quote => bevy_input::keyboard::KeyCode::Quote,
        winit::keyboard::KeyCode::Semicolon => bevy_input::keyboard::KeyCode::Semicolon,
        winit::keyboard::KeyCode::Slash => bevy_input::keyboard::KeyCode::Slash,
        winit::keyboard::KeyCode::AltLeft => bevy_input::keyboard::KeyCode::AltLeft,
        winit::keyboard::KeyCode::AltRight => bevy_input::keyboard::KeyCode::AltRight,
        winit::keyboard::KeyCode::Backspace => bevy_input::keyboard::KeyCode::Backspace,
        winit::keyboard::KeyCode::CapsLock => bevy_input::keyboard::KeyCode::CapsLock,
        winit::keyboard::KeyCode::ContextMenu => bevy_input::keyboard::KeyCode::ContextMenu,
        winit::keyboard::KeyCode::ControlLeft => bevy_input::keyboard::KeyCode::ControlLeft,
        winit::keyboard::KeyCode::ControlRight => bevy_input::keyboard::KeyCode::ControlRight,
        winit::keyboard::KeyCode::Enter => bevy_input::keyboard::KeyCode::Enter,
        winit::keyboard::KeyCode::SuperLeft => bevy_input::keyboard::KeyCode::SuperLeft,
        winit::keyboard::KeyCode::SuperRight => bevy_input::keyboard::KeyCode::SuperRight,
        winit::keyboard::KeyCode::ShiftLeft => bevy_input::keyboard::KeyCode::ShiftLeft,
        winit::keyboard::KeyCode::ShiftRight => bevy_input::keyboard::KeyCode::ShiftRight,
        winit::keyboard::KeyCode::Space => bevy_input::keyboard::KeyCode::Space,
        winit::keyboard::KeyCode::Tab => bevy_input::keyboard::KeyCode::Tab,
        winit::keyboard::KeyCode::Convert => bevy_input::keyboard::KeyCode::Convert,
        winit::keyboard::KeyCode::KanaMode => bevy_input::keyboard::KeyCode::KanaMode,
        winit::keyboard::KeyCode::Lang1 => bevy_input::keyboard::KeyCode::Lang1,
        winit::keyboard::KeyCode::Lang2 => bevy_input::keyboard::KeyCode::Lang2,
        winit::keyboard::KeyCode::Lang3 => bevy_input::keyboard::KeyCode::Lang3,
        winit::keyboard::KeyCode::Lang4 => bevy_input::keyboard::KeyCode::Lang4,
        winit::keyboard::KeyCode::Lang5 => bevy_input::keyboard::KeyCode::Lang5,
        winit::keyboard::KeyCode::NonConvert => bevy_input::keyboard::KeyCode::NonConvert,
        winit::keyboard::KeyCode::Delete => bevy_input::keyboard::KeyCode::Delete,
        winit::keyboard::KeyCode::End => bevy_input::keyboard::KeyCode::End,
        winit::keyboard::KeyCode::Help => bevy_input::keyboard::KeyCode::Help,
        winit::keyboard::KeyCode::Home => bevy_input::keyboard::KeyCode::Home,
        winit::keyboard::KeyCode::Insert => bevy_input::keyboard::KeyCode::Insert,
        winit::keyboard::KeyCode::PageDown => bevy_input::keyboard::KeyCode::PageDown,
        winit::keyboard::KeyCode::PageUp => bevy_input::keyboard::KeyCode::PageUp,
        winit::keyboard::KeyCode::ArrowDown => bevy_input::keyboard::KeyCode::ArrowDown,
        winit::keyboard::KeyCode::ArrowLeft => bevy_input::keyboard::KeyCode::ArrowLeft,
        winit::keyboard::KeyCode::ArrowRight => bevy_input::keyboard::KeyCode::ArrowRight,
        winit::keyboard::KeyCode::ArrowUp => bevy_input::keyboard::KeyCode::ArrowUp,
        winit::keyboard::KeyCode::NumLock => bevy_input::keyboard::KeyCode::NumLock,
        winit::keyboard::KeyCode::Numpad0 => bevy_input::keyboard::KeyCode::Numpad0,
        winit::keyboard::KeyCode::Numpad1 => bevy_input::keyboard::KeyCode::Numpad1,
        winit::keyboard::KeyCode::Numpad2 => bevy_input::keyboard::KeyCode::Numpad2,
        winit::keyboard::KeyCode::Numpad3 => bevy_input::keyboard::KeyCode::Numpad3,
        winit::keyboard::KeyCode::Numpad4 => bevy_input::keyboard::KeyCode::Numpad4,
        winit::keyboard::KeyCode::Numpad5 => bevy_input::keyboard::KeyCode::Numpad5,
        winit::keyboard::KeyCode::Numpad6 => bevy_input::keyboard::KeyCode::Numpad6,
        winit::keyboard::KeyCode::Numpad7 => bevy_input::keyboard::KeyCode::Numpad7,
        winit::keyboard::KeyCode::Numpad8 => bevy_input::keyboard::KeyCode::Numpad8,
        winit::keyboard::KeyCode::Numpad9 => bevy_input::keyboard::KeyCode::Numpad9,
        winit::keyboard::KeyCode::NumpadAdd => bevy_input::keyboard::KeyCode::NumpadAdd,
        winit::keyboard::KeyCode::NumpadBackspace => bevy_input::keyboard::KeyCode::NumpadBackspace,
        winit::keyboard::KeyCode::NumpadClear => bevy_input::keyboard::KeyCode::NumpadClear,
        winit::keyboard::KeyCode::NumpadClearEntry => bevy_input::keyboard::KeyCode::NumpadClearEntry,
        winit::keyboard::KeyCode::NumpadComma => bevy_input::keyboard::KeyCode::NumpadComma,
        winit::keyboard::KeyCode::NumpadDecimal => bevy_input::keyboard::KeyCode::NumpadDecimal,
        winit::keyboard::KeyCode::NumpadDivide => bevy_input::keyboard::KeyCode::NumpadDivide,
        winit::keyboard::KeyCode::NumpadEnter => bevy_input::keyboard::KeyCode::NumpadEnter,
        winit::keyboard::KeyCode::NumpadEqual => bevy_input::keyboard::KeyCode::NumpadEqual,
        winit::keyboard::KeyCode::NumpadHash => bevy_input::keyboard::KeyCode::NumpadHash,
        winit::keyboard::KeyCode::NumpadMemoryAdd => bevy_input::keyboard::KeyCode::NumpadMemoryAdd,
        winit::keyboard::KeyCode::NumpadMemoryClear => bevy_input::keyboard::KeyCode::NumpadMemoryClear,
        winit::keyboard::KeyCode::NumpadMemoryRecall => bevy_input::keyboard::KeyCode::NumpadMemoryRecall,
        winit::keyboard::KeyCode::NumpadMemoryStore => bevy_input::keyboard::KeyCode::NumpadMemoryStore,
        winit::keyboard::KeyCode::NumpadMemorySubtract => bevy_input::keyboard::KeyCode::NumpadMemorySubtract,
        winit::keyboard::KeyCode::NumpadMultiply => bevy_input::keyboard::KeyCode::NumpadMultiply,
        winit::keyboard::KeyCode::NumpadParenLeft => bevy_input::keyboard::KeyCode::NumpadParenLeft,
        winit::keyboard::KeyCode::NumpadParenRight => bevy_input::keyboard::KeyCode::NumpadParenRight,
        winit::keyboard::KeyCode::NumpadStar => bevy_input::keyboard::KeyCode::NumpadStar,
        winit::keyboard::KeyCode::NumpadSubtract => bevy_input::keyboard::KeyCode::NumpadSubtract,
        winit::keyboard::KeyCode::Escape => bevy_input::keyboard::KeyCode::Escape,
        winit::keyboard::KeyCode::Fn => bevy_input::keyboard::KeyCode::Fn,
        winit::keyboard::KeyCode::FnLock => bevy_input::keyboard::KeyCode::FnLock,
        winit::keyboard::KeyCode::PrintScreen => bevy_input::keyboard::KeyCode::PrintScreen,
        winit::keyboard::KeyCode::ScrollLock => bevy_input::keyboard::KeyCode::ScrollLock,
        winit::keyboard::KeyCode::Pause => bevy_input::keyboard::KeyCode::Pause,
        winit::keyboard::KeyCode::BrowserBack => bevy_input::keyboard::KeyCode::BrowserBack,
        winit::keyboard::KeyCode::BrowserFavorites => bevy_input::keyboard::KeyCode::BrowserFavorites,
        winit::keyboard::KeyCode::BrowserForward => bevy_input::keyboard::KeyCode::BrowserForward,
        winit::keyboard::KeyCode::BrowserHome => bevy_input::keyboard::KeyCode::BrowserHome,
        winit::keyboard::KeyCode::BrowserRefresh => bevy_input::keyboard::KeyCode::BrowserRefresh,
        winit::keyboard::KeyCode::BrowserSearch => bevy_input::keyboard::KeyCode::BrowserSearch,
        winit::keyboard::KeyCode::BrowserStop => bevy_input::keyboard::KeyCode::BrowserStop,
        winit::keyboard::KeyCode::Eject => bevy_input::keyboard::KeyCode::Eject,
        winit::keyboard::KeyCode::LaunchApp1 => bevy_input::keyboard::KeyCode::LaunchApp1,
        winit::keyboard::KeyCode::LaunchApp2 => bevy_input::keyboard::KeyCode::LaunchApp2,
        winit::keyboard::KeyCode::LaunchMail => bevy_input::keyboard::KeyCode::LaunchMail,
        winit::keyboard::KeyCode::MediaPlayPause => bevy_input::keyboard::KeyCode::MediaPlayPause,
        winit::keyboard::KeyCode::MediaSelect => bevy_input::keyboard::KeyCode::MediaSelect,
        winit::keyboard::KeyCode::MediaStop => bevy_input::keyboard::KeyCode::MediaStop,
        winit::keyboard::KeyCode::MediaTrackNext => bevy_input::keyboard::KeyCode::MediaTrackNext,
        winit::keyboard::KeyCode::MediaTrackPrevious => bevy_input::keyboard::KeyCode::MediaTrackPrevious,
        winit::keyboard::KeyCode::Power => bevy_input::keyboard::KeyCode::Power,
        winit::keyboard::KeyCode::Sleep => bevy_input::keyboard::KeyCode::Sleep,
        winit::keyboard::KeyCode::AudioVolumeDown => bevy_input::keyboard::KeyCode::AudioVolumeDown,
        winit::keyboard::KeyCode::AudioVolumeMute => bevy_input::keyboard::KeyCode::AudioVolumeMute,
        winit::keyboard::KeyCode::AudioVolumeUp => bevy_input::keyboard::KeyCode::AudioVolumeUp,
        winit::keyboard::KeyCode::WakeUp => bevy_input::keyboard::KeyCode::WakeUp,
        winit::keyboard::KeyCode::Meta => bevy_input::keyboard::KeyCode::Meta,
        winit::keyboard::KeyCode::Hyper => bevy_input::keyboard::KeyCode::Hyper,
        winit::keyboard::KeyCode::Turbo => bevy_input::keyboard::KeyCode::Turbo,
        winit::keyboard::KeyCode::Abort => bevy_input::keyboard::KeyCode::Abort,
        winit::keyboard::KeyCode::Resume => bevy_input::keyboard::KeyCode::Resume,
        winit::keyboard::KeyCode::Suspend => bevy_input::keyboard::KeyCode::Suspend,
        winit::keyboard::KeyCode::Again => bevy_input::keyboard::KeyCode::Again,
        winit::keyboard::KeyCode::Copy => bevy_input::keyboard::KeyCode::Copy,
        winit::keyboard::KeyCode::Cut => bevy_input::keyboard::KeyCode::Cut,
        winit::keyboard::KeyCode::Find => bevy_input::keyboard::KeyCode::Find,
        winit::keyboard::KeyCode::Open => bevy_input::keyboard::KeyCode::Open,
        winit::keyboard::KeyCode::Paste => bevy_input::keyboard::KeyCode::Paste,
        winit::keyboard::KeyCode::Props => bevy_input::keyboard::KeyCode::Props,
        winit::keyboard::KeyCode::Select => bevy_input::keyboard::KeyCode::Select,
        winit::keyboard::KeyCode::Undo => bevy_input::keyboard::KeyCode::Undo,
        winit::keyboard::KeyCode::Hiragana => bevy_input::keyboard::KeyCode::Hiragana,
        winit::keyboard::KeyCode::Katakana => bevy_input::keyboard::KeyCode::Katakana,
        winit::keyboard::KeyCode::F1 => bevy_input::keyboard::KeyCode::F1,
        winit::keyboard::KeyCode::F2 => bevy_input::keyboard::KeyCode::F2,
        winit::keyboard::KeyCode::F3 => bevy_input::keyboard::KeyCode::F3,
        winit::keyboard::KeyCode::F4 => bevy_input::keyboard::KeyCode::F4,
        winit::keyboard::KeyCode::F5 => bevy_input::keyboard::KeyCode::F5,
        winit::keyboard::KeyCode::F6 => bevy_input::keyboard::KeyCode::F6,
        winit::keyboard::KeyCode::F7 => bevy_input::keyboard::KeyCode::F7,
        winit::keyboard::KeyCode::F8 => bevy_input::keyboard::KeyCode::F8,
        winit::keyboard::KeyCode::F9 => bevy_input::keyboard::KeyCode::F9,
        winit::keyboard::KeyCode::F10 => bevy_input::keyboard::KeyCode::F10,
        winit::keyboard::KeyCode::F11 => bevy_input::keyboard::KeyCode::F11,
        winit::keyboard::KeyCode::F12 => bevy_input::keyboard::KeyCode::F12,
        winit::keyboard::KeyCode::F13 => bevy_input::keyboard::KeyCode::F13,
        winit::keyboard::KeyCode::F14 => bevy_input::keyboard::KeyCode::F14,
        winit::keyboard::KeyCode::F15 => bevy_input::keyboard::KeyCode::F15,
        winit::keyboard::KeyCode::F16 => bevy_input::keyboard::KeyCode::F16,
        winit::keyboard::KeyCode::F17 => bevy_input::keyboard::KeyCode::F17,
        winit::keyboard::KeyCode::F18 => bevy_input::keyboard::KeyCode::F18,
        winit::keyboard::KeyCode::F19 => bevy_input::keyboard::KeyCode::F19,
        winit::keyboard::KeyCode::F20 => bevy_input::keyboard::KeyCode::F20,
        winit::keyboard::KeyCode::F21 => bevy_input::keyboard::KeyCode::F21,
        winit::keyboard::KeyCode::F22 => bevy_input::keyboard::KeyCode::F22,
        winit::keyboard::KeyCode::F23 => bevy_input::keyboard::KeyCode::F23,
        winit::keyboard::KeyCode::F24 => bevy_input::keyboard::KeyCode::F24,
        winit::keyboard::KeyCode::F25 => bevy_input::keyboard::KeyCode::F25,
        winit::keyboard::KeyCode::F26 => bevy_input::keyboard::KeyCode::F26,
        winit::keyboard::KeyCode::F27 => bevy_input::keyboard::KeyCode::F27,
        winit::keyboard::KeyCode::F28 => bevy_input::keyboard::KeyCode::F28,
        winit::keyboard::KeyCode::F29 => bevy_input::keyboard::KeyCode::F29,
        winit::keyboard::KeyCode::F30 => bevy_input::keyboard::KeyCode::F30,
        winit::keyboard::KeyCode::F31 => bevy_input::keyboard::KeyCode::F31,
        winit::keyboard::KeyCode::F32 => bevy_input::keyboard::KeyCode::F32,
        winit::keyboard::KeyCode::F33 => bevy_input::keyboard::KeyCode::F33,
        winit::keyboard::KeyCode::F34 => bevy_input::keyboard::KeyCode::F34,
        winit::keyboard::KeyCode::F35 => bevy_input::keyboard::KeyCode::F35,
        _ => KeyCode::Unidentified(NativeKeyCode::Unidentified),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_code_mapping_matches_b_table() {
        // Table entry b = b+1: winit Backquote (0) -> Bevy Backquote (1),
        // winit Space (62) -> Bevy Space (63), winit F35 (193) -> Bevy F35 (194).
        assert_eq!(
            winit_code_to_bevy_keycode(WinitKeyCode::Backquote),
            KeyCode::Backquote
        );
        assert_eq!(
            winit_code_to_bevy_keycode(WinitKeyCode::Space),
            KeyCode::Space
        );
        assert_eq!(winit_code_to_bevy_keycode(WinitKeyCode::F35), KeyCode::F35);
    }
}

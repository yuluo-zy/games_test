//! `game_key_bindings.rs` — KeyBindings / KeyBinding / KeyBindingDesc /
//! MouseBinding (profile-persisted key mapping).
//!
//! Source: B skeleton `crates/country-core/src/game_key_bindings.rs`.
//! PDB named entities: `KeyBinding`, `KeyBindingDesc`, `KeyBindings`, `MouseBinding`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `KeyBindings::assign_mouse_button` @0x14089b280 — writes two `u16` at
//!   `self + 0x1bc + action_index*4` and `self + 0x1be + action_index*4`
//!   ⇒ each action slot below 0x1bc is `{ key: u16, mouse: u16 }` (4 B stride).
//! - `KeyBindings::assign_key` @0x14089b2a0 — remap with conflict handling
//!   (body 1700 lines; per-action swap semantics not fully recovered).
//! - `MouseBinding::get_mouse_button` @0x14089c880, `MouseBinding::action_desc`
//!   @0x14089c8a0, `KeyBinding::action_desc` @0x14089cb60, `KeyBinding::get_keycode`
//!   @0x14089d300, `keycode_to_str` @0x14089d490 (jump table on the code byte),
//!   `mouse_button_to_string` @0x14089c970, `KeyBindingDesc::{text,icon}`.
//! - Default key-bindings sequence (O: `load_game_profile` default build):
//!   26 key slots `{7,8,9,10,0xb,0xc,0xd,0xe,0xf,0x23,0x27,0x2d,0x2b,0xc3,0x38,
//!   0xab,0xa5,0x1b,0x49,0x3a,0x51,0x52,0x50,0x53,0x4f,0x4e}` (`0xc3` = unset),
//!   then mouse buttons `{1,2,1}`, then one more `0xc3`.
//!   These are bevy-input-style key codes; exact KeyCode enum mapping U.

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

use crate::Gap;

/// A key-code binding (O: u16 slot in `assign_mouse_button`; code value U).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct KeyCode(pub u16);

/// `KeyBinding` (O named entity): one action's bound key/mouse pair.
/// Layout I: `{ key: KeyCode, mouse: MouseBinding }` (u16 + u16).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub mouse: MouseBinding,
}

impl KeyBinding {
    /// `KeyBinding::get_keycode` @0x14089d300 (O).
    pub fn get_keycode(&self) -> KeyCode {
        self.key
    }
    /// `KeyBinding::action_desc` @0x14089cb60 (O): UI description string.
    pub fn action_desc(&self) -> String {
        String::new()
    }
}

/// `KeyBindingDesc` (O named entity): static description of one binding slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyBindingDesc {
    /// `KeyBindingDesc::text` (O inline in `KeyBinding::action_desc`).
    pub text: &'static str,
    /// `KeyBindingDesc::icon` (O inline in `KeyBinding::action_desc`).
    pub icon: &'static str,
}

/// `MouseBinding` (O named entity). Discriminant range used by
/// `mouse_button_to_string` jump table (u16).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MouseBinding(pub u16);

impl MouseBinding {
    /// `MouseBinding::get_mouse_button` @0x14089c880 (O).
    pub fn get_mouse_button(&self) -> u16 {
        self.0
    }
    /// `MouseBinding::action_desc` @0x14089c8a0 (O).
    pub fn action_desc(&self) -> String {
        mouse_button_to_string(self)
    }
}

/// `game_key_bindings::keycode_to_str` @0x14089d490 (O): keycode → label.
/// Jump-table body (range `(byte + 0x7b) < 0xf4`) U → returns "".
pub fn keycode_to_str(code: KeyCode) -> String {
    String::new()
}

/// `game_key_bindings::mouse_button_to_string` @0x14089c970 (O): mouse → label.
pub fn mouse_button_to_string(_b: &MouseBinding) -> String {
    String::new()
}

/// `KeyBindings` (O named entity; persisted). Slots live at offset 0x1bc as
/// `{key u16, mouse u16}` per action (O: `assign_mouse_button`). Leading layout
/// (the action-id table) U → typed gap.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KeyBindings {
    /// Action slots: each `{key, mouse}` (u16+u16). Recovered slot count I:
    /// default sequence supplies 26 key slots + 3 mouse-only actions.
    pub slots: Vec<KeyBinding>,
    /// Typed gap for the pre-0x1bc region.
    pub _gap: Gap,
}

impl KeyBindings {
    /// Default bindings (O sequence from `load_game_profile` default build).
    pub fn default_bindings() -> Self {
        const DEFAULTS: [u16; 26] = [
            0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x23, 0x27, 0x2d,
            0x2b, 0xc3, 0x38, 0xab, 0xa5, 0x1b, 0x49, 0x3a, 0x51, 0x52, 0x50, 0x53,
            0x4f, 0x4e,
        ];
        let mut slots: Vec<KeyBinding> = DEFAULTS
            .iter()
            .map(|&k| KeyBinding {
                key: KeyCode(k),
                mouse: MouseBinding(0),
            })
            .collect();
        // Three mouse-only actions (O: u16 `{1,2,1}`).
        for (i, m) in [1u16, 2, 1].iter().enumerate() {
            if let Some(slot) = slots.get_mut(i) {
                slot.mouse = MouseBinding(*m);
            }
        }
        slots.push(KeyBinding {
            key: KeyCode(0xc3),
            mouse: MouseBinding(0),
        });
        KeyBindings {
            slots,
            _gap: Gap,
        }
    }

    /// `KeyBindings::assign_key` @0x14089b2a0 (O; conflict handling U).
    pub fn assign_key(&mut self, _action: u8, _key: KeyCode) {
        todo!("recover assign_key conflict resolution")
    }

    /// `KeyBindings::assign_mouse_button` @0x14089b280 (O):
    /// writes `{key,mouse}` at `0x1bc + action*4`.
    pub fn assign_mouse_button(&mut self, action: u8, key: KeyCode, mouse: u16) {
        if let Some(slot) = self.slots.get_mut(action as usize) {
            slot.key = key;
            slot.mouse = MouseBinding(mouse);
        }
    }
}

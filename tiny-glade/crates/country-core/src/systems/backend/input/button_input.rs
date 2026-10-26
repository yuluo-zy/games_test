//! 使用锁定Bevy ButtonInput管理按住与边沿；保留已有u16接口。
//! 原KeyCode::Unidentified保留平台载荷，MouseButton::Other与标准键独立。
use bevy_ecs::resource::Resource;
use bevy_input::{
    ButtonInput,
    keyboard::{KeyCode, NativeKeyCode},
    mouse::MouseButton,
};
use std::ops::{Deref, DerefMut};
/// 键位配置中的未绑定哨兵（不是Unidentified）。
pub const UNBOUND_KEY: u16 = 0xc3;
#[derive(Resource, Clone, Debug, Default)]
pub struct KeyButtonInput {
    numeric: ButtonInput<u16>,
    // 只存数字接口无法表达的平台原生键；不为普通键重复维护整份集合。
    native: ButtonInput<NativeKeyCode>,
}
impl KeyButtonInput {
    pub fn apply_event(&mut self, event: &bevy_input::keyboard::KeyboardInput) {
        if event.state.is_pressed() {
            self.press_key(event.key_code)
        } else {
            self.release_key(event.key_code)
        }
    }
    pub fn press_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Unidentified(code) => self.native.press(code),
            key => self.numeric.press(
                crate::game_profile::desktop_keycode_index(key)
                    .expect("194个命名KeyCode均可数字投影"),
            ),
        }
    }
    pub fn release_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Unidentified(code) => self.native.release(code),
            key => self.numeric.release(
                crate::game_profile::desktop_keycode_index(key)
                    .expect("194个命名KeyCode均可数字投影"),
            ),
        }
    }
    pub fn pressed_key(&self, key: KeyCode) -> bool {
        match key {
            KeyCode::Unidentified(code) => self.native.pressed(code),
            key => self
                .numeric
                .pressed(crate::game_profile::desktop_keycode_index(key).expect("命名KeyCode投影")),
        }
    }
    pub fn any_just_pressed(&self) -> bool {
        self.numeric.get_just_pressed().next().is_some()
            || self.native.get_just_pressed().next().is_some()
    }
    /// 帧尾只清边沿；会话切换的reset_all同时清按住状态。
    pub fn clear(&mut self) {
        self.numeric.clear();
        self.native.clear();
    }
    pub fn reset_all(&mut self) {
        self.numeric.reset_all();
        self.native.reset_all();
    }
}
impl Deref for KeyButtonInput {
    type Target = ButtonInput<u16>;
    fn deref(&self) -> &Self::Target {
        &self.numeric
    }
}
impl DerefMut for KeyButtonInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.numeric
    }
}
/// 标准鼠标数字键：Left0/Right1/Middle2/Back3/Forward4，其余原数字接口保持兼容。
#[derive(Resource, Clone, Debug, Default)]
pub struct MouseButtonInput {
    numeric: ButtonInput<u16>,
    // Other(0..4)与标准五键不同身份；其余Other继续复用无冲突数字空间。
    other_low: ButtonInput<u16>,
}
impl MouseButtonInput {
    pub fn apply_event(&mut self, event: &bevy_input::mouse::MouseButtonInput) {
        if event.state.is_pressed() {
            self.press_button(event.button)
        } else {
            self.release_button(event.button)
        }
    }
    fn numeric_button(button: MouseButton) -> Option<u16> {
        match button {
            MouseButton::Left => Some(0),
            MouseButton::Right => Some(1),
            MouseButton::Middle => Some(2),
            MouseButton::Back => Some(3),
            MouseButton::Forward => Some(4),
            MouseButton::Other(n) if n > 4 => Some(n),
            MouseButton::Other(_) => None,
        }
    }
    pub fn press_button(&mut self, button: MouseButton) {
        if let Some(code) = Self::numeric_button(button) {
            self.numeric.press(code)
        } else if let MouseButton::Other(n) = button {
            self.other_low.press(n)
        }
    }
    pub fn release_button(&mut self, button: MouseButton) {
        if let Some(code) = Self::numeric_button(button) {
            self.numeric.release(code)
        } else if let MouseButton::Other(n) = button {
            self.other_low.release(n)
        }
    }
    pub fn pressed_button(&self, button: MouseButton) -> bool {
        Self::numeric_button(button).map_or_else(
            || matches!(button,MouseButton::Other(n)if self.other_low.pressed(n)),
            |code| self.numeric.pressed(code),
        )
    }
    pub fn just_pressed_button(&self, button: MouseButton) -> bool {
        Self::numeric_button(button).map_or_else(
            || matches!(button,MouseButton::Other(n)if self.other_low.just_pressed(n)),
            |code| self.numeric.just_pressed(code),
        )
    }
    pub fn just_released_button(&self, button: MouseButton) -> bool {
        Self::numeric_button(button).map_or_else(
            || matches!(button,MouseButton::Other(n)if self.other_low.just_released(n)),
            |code| self.numeric.just_released(code),
        )
    }
    pub fn any_just_pressed(&self) -> bool {
        self.numeric.get_just_pressed().next().is_some()
            || self.other_low.get_just_pressed().next().is_some()
    }
    pub fn clear(&mut self) {
        self.numeric.clear();
        self.other_low.clear();
    }
    pub fn reset_all(&mut self) {
        self.numeric.reset_all();
        self.other_low.reset_all();
    }
}
impl Deref for MouseButtonInput {
    type Target = ButtonInput<u16>;
    fn deref(&self) -> &Self::Target {
        &self.numeric
    }
}
impl DerefMut for MouseButtonInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.numeric
    }
}

//! `button_input.rs` — 键鼠按钮输入抽象（crate 无 `bevy_input` 依赖的等价层）。
//!
//! `translate_input`（B 0x1417cd0a0）与 `reset_input_state`（0x140942b00）在 PDB/机器码里
//! 消费 `bevy_input::ButtonInput<KeyCode>` / `ButtonInput<MouseButton>`（O：PDB 签名、wp20
//! 分析）。本 crate 的 `Cargo.toml` 只依赖 `bevy_ecs`/`bevy_time`，未纳入 `bevy_input`，
//! 因此这里提供等价的最小 `ButtonInput` 语义（`pressed` / `just_pressed` / `reset_all`），
//! 供后端输入系统消费，避免伪造游戏业务。键码沿用 `game_key_bindings` 的 u16 槽码
//! （`0xc3` = 未绑定哨兵，O）。
use std::collections::HashSet;

use bevy_ecs::resource::Resource;

/// 未绑定哨兵键码（O：`game_key_bindings` 默认序列 `0xc3` = unset）。
pub const UNBOUND_KEY: u16 = 0xc3;

/// `ButtonInput<KeyCode>` 的等价抽象（Resource）。
#[derive(Resource, Clone, Debug, Default)]
pub struct KeyButtonInput {
    /// 当前按下的键码集合。
    pub pressed: HashSet<u16>,
    /// 本帧新按下的键码沿。
    pub just_pressed: HashSet<u16>,
}

impl KeyButtonInput {
    /// `pressed(&KeyCode)`（bevy 语义）。
    pub fn pressed(&self, code: u16) -> bool {
        self.pressed.contains(&code)
    }

    /// `just_pressed(&KeyCode)`（bevy 语义）。
    pub fn just_pressed(&self, code: u16) -> bool {
        self.just_pressed.contains(&code)
    }

    /// `reset_all<KeyCode>()`（O：0x140942b00 尾段调用）：复位按键沿状态。
    pub fn reset_all(&mut self) {
        self.just_pressed.clear();
    }
}

/// 鼠标按钮（O：bevy `MouseButton` 判别：Left=0 / Right=1 / Middle=2，与
/// `game_key_bindings` 的 u16 `{1,2,1}` 鼠标绑定同空间）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MouseButtonInput {
    pub pressed: HashSet<u16>,
    pub just_pressed: HashSet<u16>,
}

impl MouseButtonInput {
    pub fn pressed(&self, button: u16) -> bool {
        self.pressed.contains(&button)
    }

    pub fn just_pressed(&self, button: u16) -> bool {
        self.just_pressed.contains(&button)
    }

    /// 任一鼠标按钮本帧新按下（O：0x1417cd0a0 对 `ButtonInput<MouseButton>` 按下事件位扫描）。
    pub fn just_pressed_any(&self) -> bool {
        !self.just_pressed.is_empty()
    }

    /// 鼠标按钮本帧释放（O：`FUN_14089a9f0` = just_released 语义）。
    pub fn just_released(&self, button: u16) -> bool {
        self.just_pressed(button) && !self.pressed(button)
    }

    pub fn reset_all(&mut self) {
        self.just_pressed.clear();
    }
}

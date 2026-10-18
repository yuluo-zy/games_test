//! 使用锁定版本的 Bevy ButtonInput 管理按住、按下和释放边沿。
//! 键盘和鼠标使用不同资源类型，保持独立的数字键码空间。
use bevy_ecs::resource::Resource;
use bevy_input::ButtonInput;
use std::ops::{Deref, DerefMut};

/// 键位配置中的未绑定哨兵。
pub const UNBOUND_KEY: u16 = 0xc3;

#[derive(Resource, Clone, Debug, Default)]
pub struct KeyButtonInput(ButtonInput<u16>);
impl Deref for KeyButtonInput {
    type Target = ButtonInput<u16>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for KeyButtonInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 鼠标数字键码：左键 0、右键 1、中键 2。
#[derive(Resource, Clone, Debug, Default)]
pub struct MouseButtonInput(ButtonInput<u16>);
impl Deref for MouseButtonInput {
    type Target = ButtonInput<u16>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MouseButtonInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

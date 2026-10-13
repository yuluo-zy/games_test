//! `resources::critters::sheep` — 绵羊 CPU 状态（B 骨架 `resources/critters/sheep.rs`）。
//! serde_json 自动派生的 Serialize/Deserialize（`_::impl$0::serialize/deserialize`，O）复用依赖。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::event::Event;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// 绵羊组件（O；serde；`being_petted` 行 136 读 `f32 > 0.1` 判定，见 WP6 sheep_pet）。
#[derive(Component, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Sheep {
    /// 抚摸动画幅度（`+0x58`，O；`> 0.1` → being_petted）。
    pub amplitude: f32,
    /// 控制标志（`+0x60`，O）。
    pub control_flag: u8,
    /// 其余字段（`+0..0x58`、`+0x64..`）U。
    pub _gap: Gap,
}

impl Sheep {
    /// `Sheep::being_petted`（O 名；reva 反汇编 `amplitude > 0x3c23d70a`(0.1)）。
    pub fn being_petted(&self) -> bool {
        self.amplitude > f32::from_bits(0x3c23d70a)
    }
}

/// 绵羊移动状态（O 名；字段 U）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SheepMoveState(pub Gap);

/// 绵羊音频（O 名；字段 U）。
#[derive(Component, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SheepAudio(pub Gap);

/// 绵羊配饰（O 名；字段 U）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SheepAccessory(pub Gap);

/// 浮水圈持有（O 名；字段 U）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FloatieHolder(pub Gap);

/// 雨伞持有（O 名；字段 U）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UmbrellaHolder(pub Gap);

/// 检查绵羊是否在建筑内的命令（O 名；payload U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct CmdCheckForSheepInsideBuilding(pub Gap);

/// 解冻绵羊命令（O 名；payload U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct CmdUnfreezeSheep(pub Gap);

/// 组件注册（骨架 `impl$1/7/8/9/10::register_required_components`，O 名）。
pub fn register_required_components() {
    let _ = core::marker::PhantomData::<Sheep>;
}

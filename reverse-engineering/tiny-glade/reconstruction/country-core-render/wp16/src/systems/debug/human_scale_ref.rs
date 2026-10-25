//! human_scale_ref.rs — 人类尺度参考对象（O）。
//!
//! 源 `systems/debug/human_scale_ref.rs`；实体 `HumanScaleRef`。
//! `human_scale_ref`（0x1408b2b30）：debug 模式放置/维护一个人形尺度参考。

#![allow(dead_code)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 人类尺度参考组件（O：`HumanScaleRef`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct HumanScaleRef(pub Gap);

/// `human_scale_ref`（O 0x1408b2b30）。
pub fn human_scale_ref(_world: &mut bevy_ecs::world::World) {}

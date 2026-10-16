//! `signifiers/mod.rs` — 指示符流/生成/动画/回收（O：ui-input DIGEST）。
//!
//! 原文件：signifier_create.rs（0x140b2cf90/0x140b2cfa0）、animation.rs
//! （signifier_animation）、signifier_mark_for_delete.rs（signfier_mark_for_delete/
//! despawn_marked）、mod.rs（register_required_components/clone_behavior 胶水）。
//! 动画推进与回收依赖实体组件机制，登记待取证。
pub mod signifier_create;

use bevy_ecs::prelude::*;

pub use signifier_create::{UiAppendSignifier, UiSignifierStream};

/// 注册指示符事件与资源。
pub fn register_signifier_resources(world: &mut World) {
    world.init_resource::<UiSignifierStream>();
    world.init_resource::<Events<signifier_create::SignifierSpawned>>();
}

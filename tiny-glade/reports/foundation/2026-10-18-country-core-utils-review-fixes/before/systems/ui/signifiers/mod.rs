//! `signifiers/mod.rs` — 指示符流/生成/动画/回收（O：ui-input DIGEST）。
//!
//! 原文件：signifier_create.rs（0x140b2cf90/0x140b2cfa0）、animation.rs
//! （signifier_animation_t/0x140a69340、signifier_animation/0x140a69c30）、
//! signifier_mark_for_delete.rs（signfier_mark_for_delete/0x140929160、
//! despawn_marked/0x140929a90）、mod.rs（register_required_components/clone_behavior 胶水）。
//! 动画推进（t-tick）与删除链已落地；scale/alpha 目标 switch 与 config_hash→widget
//! 匹配依赖宿主 UiState，登记待取证。
pub mod animation;
pub mod signifier_create;
pub mod signifier_factory;
pub mod signifier_mark_for_delete;

use bevy_ecs::prelude::*;

pub use animation::{SignifierAnimation, SignifierParentToVolumeOrigin};
pub use signifier_create::{SignifierId, UiAppendSignifier, UiSignifierEntry, UiSignifierStream};
pub use signifier_factory::{SignifierClosure, SignifierEntityBuilder, SignifierFactory};
pub use signifier_mark_for_delete::{
    SignifierMarkedForDeletion, SignifierMinimumLifespan, signfier_mark_for_delete,
};

/// 注册指示符资源。
pub fn register_signifier_resources(world: &mut World) {
    world.init_resource::<UiSignifierStream>();
}

/// 本地可运行顺序（仅接本模块内部可自持的组件；host 插入 SignifierAnimation 实体）。
pub fn add_signifier_systems(schedule: &mut Schedule) {
    schedule.add_systems((
        signifier_create::signifier_create,
        animation::signifier_animation_t,
        animation::signifier_animation,
        signifier_mark_for_delete::signfier_mark_for_delete,
        signifier_mark_for_delete::despawn_marked,
    ));
}

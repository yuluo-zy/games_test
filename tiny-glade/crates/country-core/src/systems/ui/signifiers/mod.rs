//! `signifiers/mod.rs` — 指示符流/生成/动画/回收（O：ui-input DIGEST）。
//!
//! 原文件：signifier_create.rs（0x140b2cf90/0x140b2cfa0）、animation.rs
//! （signifier_animation_t/0x140a69340、signifier_animation/0x140a69c30）、
//! signifier_mark_for_delete.rs（signfier_mark_for_delete/0x140929160、
//! despawn_marked/0x140929a90）、mod.rs（register_required_components/clone_behavior 胶水）。
//! 捕获 WidgetParams 拥有实际工厂列表，ID 同时保存 hash 与 WidgetId。
//! 原版顺序与 deferred 边界由本组保持；数字 Raw 生产者仍属后序模块阻断接口。
pub mod animation;
pub mod signifier_create;
pub mod signifier_factory;
pub mod signifier_mark_for_delete;

#[cfg(test)]
mod tests;

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

/// 原 set6 consumer 顺序。保留原 chain_ignore_deferred，不提前提交实体命令。
pub fn add_signifier_systems(schedule: &mut Schedule) {
    schedule.add_systems(
        (
            signifier_create::signifier_create,
            signifier_mark_for_delete::signfier_mark_for_delete,
            animation::signifier_animation_t,
            animation::signifier_animation,
            signifier_mark_for_delete::despawn_marked,
        )
            .chain_ignore_deferred(),
    );
}

//! `signifiers/mod.rs` — 指示符流/生成/动画/回收（O：ui-input DIGEST）。
//!
//! 原文件：signifier_create.rs（0x140b2cf90/0x140b2cfa0）、animation.rs
//! （signifier_animation_t/0x140a69340、signifier_animation/0x140a69c30）、
//! signifier_mark_for_delete.rs（signfier_mark_for_delete/0x140929160、
//! despawn_marked/0x140929a90）、mod.rs（register_required_components/clone_behavior 胶水）。
//! Candidate complete factory lifecycle, with original captured-owner and deferred semantics.
//! Raw producers and debug systems remain explicit integration blockers.
pub mod animation;
pub mod signifier_create;
pub mod signifier_factory;
pub mod signifier_mark_for_delete;

#[cfg(test)]
mod tests;

use bevy_ecs::prelude::*;

pub use animation::{SignifierAnimation, SignifierParentToVolumeOrigin};
pub use signifier_create::{SignifierId, UiAppendSignifier, UiSignifierEntry, UiSignifierStream};
pub use signifier_factory::{SignifierClosure, SignifierEntityBuilder, SignifierFactory, SignifierMesh};
pub use signifier_mark_for_delete::{
    SignifierMarkedForDeletion, SignifierMinimumLifespan, signfier_mark_for_delete,
};

/// 注册指示符资源。
pub fn register_signifier_resources(world: &mut World) {
    world.init_resource::<UiSignifierStream>();
}

/// 请求生产者应在本组之前执行；创建、动画、标记和销毁按顺序执行。
/// Original chain_ignore_deferred preserves command visibility across the slice.
/// Host must order this slice after ui_update in set6; debug members stay explicit U.
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

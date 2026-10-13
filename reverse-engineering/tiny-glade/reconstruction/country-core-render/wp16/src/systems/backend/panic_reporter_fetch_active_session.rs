//! panic_reporter_fetch_active_session.rs — 崩溃报告取活动会话（O；全内联）。
//!
//! 源 `systems/backend/panic_reporter_fetch_active_session.rs`（PDB 记录函数
//! `panic_reporter_fetch_active_session`，全内联）。启动/崩溃时向崩溃报告器
//! 提供当前活动 session 标识。

#![allow(dead_code)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 活动会话标识（O：`Res<ActiveSession>`；反馈报告器读取）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ActiveSession(pub Gap);

/// `panic_reporter_fetch_active_session`（O）。
pub fn panic_reporter_fetch_active_session(_world: &mut bevy_ecs::world::World) {
    // U：报告器表面（typed gap）。
}

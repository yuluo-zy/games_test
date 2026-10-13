//! configwatch.rs — 配置热载系统（O 0x14085b1c0）。
//!
//! 源 `systems/backend/configwatch.rs`。每帧检查 `ConfigWatch` 是否有变更的
//! 配置文件字节；有则重载 `RenderConfig`/GladeSettings。

#![allow(dead_code)]

/// `configwatch`（O 0x14085b1c0）：排空 `ConfigWatch` 待处理文件，重载配置。
pub fn configwatch(_world: &mut bevy_ecs::world::World) {
    // U：具体重载目标（RenderConfig 字段映射 typed gap）。
}

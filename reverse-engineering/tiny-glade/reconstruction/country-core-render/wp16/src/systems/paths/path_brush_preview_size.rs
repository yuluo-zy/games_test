//! path_brush_preview_size.rs — 路径笔刷预览尺寸（O 0x140b1c010）。
//!
//! 源 `systems/paths/path_brush_preview_size.rs`；命令 `SetSizeBasedOnAppModePath`。
//! 按 UI 模式设置路径笔刷预览圆尺寸。

#![allow(dead_code)]

use bevy_ecs::component::Component;

/// 按模式设置笔刷尺寸命令（O：PDB 实体）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct SetSizeBasedOnAppModePath(pub crate::Gap);

/// 路径笔刷预览尺寸组件（O：`path_brush_preview_size` 系统写）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PathBrushPreviewSize(pub f32);

/// `path_brush_preview_size`（O 0x140b1c010）。
pub fn path_brush_preview_size(_world: &mut bevy_ecs::world::World) {}

//! lazy_grid_debug.rs — 懒网格调试可视化（O；全内联）。
//!
//! 源 `systems/debug/lazy_grid_debug.rs`（PDB 记录 `impl$1::default` /
//! `impl$0::build::closure$0` / `show_lazy_grid_debug::closure$0`）。
//! 显示 autoclutter 懒网格的单元格。

#![allow(dead_code)]

use bevy_ecs::resource::Resource;

/// 懒网格调试开关（O：资源）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct LazyGridDebug(pub bool);

/// `show_lazy_grid_debug`（O）。
pub fn show_lazy_grid_debug(_world: &mut bevy_ecs::world::World) {}

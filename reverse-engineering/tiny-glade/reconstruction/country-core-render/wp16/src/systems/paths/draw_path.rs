//! draw_path.rs — 路径绘制系统（O 0x140ae4d00）。
//!
//! 源 SHA256（B skeleton）`systems/paths/draw_path.rs`；实体 `DrawState` /
//! `PathRastersResources`。
//!
//! 功能（O）：UI 拖拽路径 → `DrawState::update`（状态机：path/cobblestone
//! 1..3/eraser）→ 生成路径段，按当前路径变体对 `WorldRaster<512>`（Path /
//! Cobblestone1/2/3）做 CPU 绘制（`world_raster::draw_raster`），同时擦除其它
//! 变体（`erase_other_path_variants`）。`PathRastersResources::as_float_grid`
//! （0x140ae4c20）导出 4 路（GPU 暂存 vs CPU）浮点栅格供采样
//! （nani/audio/autoclutter）。

#![allow(dead_code)]

use crate::boundary::WorldRaster;

/// 路径变体（I：来自 UI 模式；O：draw_path 的 `local_158` switch 0..3）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PathKind {
    DirtPath = 0,
    Cobblestone1 = 1,
    Cobblestone2 = 2,
    Cobblestone3 = 3,
    /// eraser（O：draw_path 的 `lVar20==4` 分支）。
    Eraser = 4,
}

/// 路径绘制状态（O：`DrawState`；`update` 0x140ae4bd0 / `start_or_continue` /
/// `finish` 内联）。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DrawState {
    /// 当前绘制模式（O：状态字节；0=idle, 1=draw, 2/3=…）。
    pub mode: u8,
}

impl DrawState {
    /// `update`（O 0x140ae4bd0）：按 UI 模式推进状态机。
    pub fn update(&mut self, _ui_mode: u8) {
        // U：精确状态转移表（typed gap）。
    }
    pub fn start_or_continue(&mut self) {}
    pub fn finish(&mut self) {}
}

/// 路径栅格资源（O：`Res<PathRastersResources>`；Path/Cobblestone1/2/3 四个
/// `WorldRaster<512>` + GPU 暂存标志 +0x298）。
#[derive(Clone, Debug, Default)]
pub struct PathRastersResources {
    pub path: WorldRaster<512>,
    pub cobblestone1: WorldRaster<512>,
    pub cobblestone2: WorldRaster<512>,
    pub cobblestone3: WorldRaster<512>,
}

impl PathRastersResources {
    /// `as_float_grid`（O 0x140ae4c20）：返回 4×2（每变体 f32 网格 + 标志）
    /// 的采样视图；读取 CPU（+8/+0x10）或 GPU 暂存（+0x20/+0x28）取决于
    /// +0x298 标志。
    pub fn as_float_grid(&self) -> [[f32; 4]; 4] {
        // U：栅格→f32 采样（typed gap）。
        [[0.0; 4]; 4]
    }
}

/// `draw_path`（O 0x140ae4d00）：排空绘制输入（起点/终点/变体），按变体调
/// `draw_raster<StagingSsbo, WorldRaster<512>, …>`。
pub fn draw_path(_world: &mut bevy_ecs::world::World) {}

/// `erase_other_path_variants`（O：draw_path 内联）—— 绘制一种变体时擦除
/// 其余变体。
pub fn erase_other_path_variants(
    _world: &mut bevy_ecs::world::World,
    _active: PathKind,
) {
}

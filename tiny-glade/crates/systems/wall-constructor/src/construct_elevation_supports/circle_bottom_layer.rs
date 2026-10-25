//! 圆形底部砖层（construct_elevation_supports/circle_bottom_layer.rs）。
//!
//! `construct_circle_bottom_layer`（0x14124b2c0）+ `add_bottom_brick`
//! （0x141fe1de0，11 参；circle/rect 两闭包共用同一体）。
//!
//! 2026-10-17 restorer-remaining-wallconstructor fresh 结论（证据
//! `evidence/elev-0x14124b2c0.c`、`evidence/abb-dis.result.json`）：
//! 旧实现（TAU 均分 + 仅写 position 的 96B 记录）为**近似体**，已移除。
//!
//! `construct_circle_bottom_layer` 已解结构：
//! - 入参 circle：center Vec2@0、`radius+0.28`@8、@0xc；
//! - `Circle2d::perimeter()` → `ceil(perimeter / 0.69)` 段数；
//! - `WallStyle::is_halftimbered()` 早退；
//! - `AssetSsboLibrary` FrozenMap 取 `BrickTransformSsbo` 缓冲（0x3a 字节
//!   类型名 memcmp），`BufferState` 加锁（+0x18 自旋位 / parking_lot
//!   lock_slow / release_resource）；
//! - `utils::random_splits_into` 生成 u 序列，逐对 `Circle2d::get_pos_at_u`
//!   → `add_bottom_brick`（循环按首/次点交替推进）；
//! - 解锁后释放 BufferState。
//!
//! `add_bottom_brick` 已解结构（105 条指令）：`fastrand::Rng::f32` 抖动、
//! `x0y`×2、`xvy`、`render::impl$1::from`（0x60B 记录装配）、
//! `instanced_wall::impl$1::from`/`impl$3::default`、
//! `SparseInstanceBufferWriter::push<BrickTransformSsbo>`，失败置脏字节 1。
//!
//! **未实现**：两函数的 11 参 ABI、闭包环境字段、记录装配常量
//! （0x142925870/0x1429cd068/0x142925970/0x142a81ca0/0x1429258e0/
//! 0x1429258f0/0x1429345b0/0x142fcb7f0）未逐条锚定。
use glam::Vec3;

/// B `add_bottom_brick`（0x141fe1de0）。**未实现**：见模块文档。
pub fn add_bottom_brick(
    _position: Vec3,
    _rotation: f32,
    _width: f32,
    _depth: f32,
    _out: &mut Vec<[u8; 96]>,
) {
    todo!("未实现: add_bottom_brick 0x141fe1de0（11 参 ABI/闭包环境/0x60B 记录装配常量未逐条锚定）")
}

/// B `construct_circle_bottom_layer`（0x14124b2c0）。**未实现**：见模块文档。
pub fn construct_circle_bottom_layer(
    _center: Vec3,
    _radius: f32,
    _count: usize,
    _out: &mut Vec<[u8; 96]>,
) {
    todo!(
        "未实现: construct_circle_bottom_layer 0x14124b2c0（random_splits_into/get_pos_at_u/BufferState 锁链未逐语句）"
    )
}

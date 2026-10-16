//! stitch_bricks（B 主基线；spawn_stitches 0x141ce65b0、spawn_stitch_bricks 0x141ce7190）。
//!
//! 证据（clutter-extra-analysis.md §stitch_bricks）：
//! - spawn_stitches（L28–89）：遍历 Vec（stride 0x40）缝合描述；每元素
//!   `+0x00` i32 种类（==1 才处理）、`+0x04` f32（>=0.2 跳过）、`+0x14` f32 段起点、
//!   `+0x18` f32 段长。段长检查：`u = truncate(min(1000, 2*|长|))`，`2 < u` 跳过。
//!   取段中点、`utils::random_splits()` 细分；单位法向（1/SQRT）+ 朝向（x0y swizzle），
//!   每细分点 `fastrand::Rng::f32()` 抖动 `(r*0.5-(1-r)*0.5)*0.1+0.4`、`Δu+(-0.1)`。
//!   产出 `StairsFloatingBrickParams`（stride 0x40：4 float 位置/朝向、方向 qword、
//!   参数 u32、抖动 f32、Δ f32、0.3 常量、Vec2、u64）。
//! - spawn_stitch_bricks（L92–140）：`FrozenMap` 取 `BrickTransformSsbo` 缓冲；
//!   `SparseInstanceBufferWriter::clear` 清空；遍历 params：`resources::render::impl$1::from`
//!   （render 资源→颜色/变换）、`calculate_hash<([WallId;2],usize)>`（墙对键）、
//!   `geometry::instanced_wall::impl$1::from`（→ BrickTransform）、push 实例。
//!   产出：把缝合砖参数烘焙进 BrickTransformSsbo 实例缓冲（GPU 后置）。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::swizzlers::x0y;

/// 悬浮砖参数（stride 0x40；spawn_stitches 输出 / spawn_stitch_bricks 输入）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StairsFloatingBrickParams {
    /// +0..+0x10：位置/朝向（4 float）。
    pub position: Vec3,
    pub facing: f32,
    /// +0x10：方向 qword（CONCAT44）。
    pub direction: u64,
    /// +0x18：参数 u32。
    pub param: u32,
    /// +0x1c：抖动 f32。
    pub jitter: f32,
    /// +0x20：Δ f32。
    pub delta: f32,
    /// +0x24：0.3 常量（3e99999a）。
    pub c24: f32,
    /// +0x28：Vec2。
    pub v28: Vec2,
    /// +0x38：u64（墙对键）。
    pub key: u64,
}

/// 缝合描述（stride 0x40 输入；spawn_stitches 读）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StitchDescription {
    /// +0x00：种类（==1 才处理）。
    pub kind: i32,
    /// +0x04：f32（>= 0.2 跳过）。
    pub threshold: f32,
    /// +0x14：段起点。
    pub start: f32,
    /// +0x18：段长。
    pub length: f32,
}

/// 缝合描述集合（资源；宿主装配）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StitchDescriptions(pub Vec<StitchDescription>);

/// 缝合砖参数集合（资源；spawn_stitches 输出 / spawn_stitch_bricks 输入）。
#[derive(Resource, Clone, Debug, Default)]
pub struct FloatingBrickParams(pub Vec<StairsFloatingBrickParams>);

/// 缝合 RNG（fastrand）。
#[derive(Resource, Debug)]
pub struct StitchRng(pub std::sync::Mutex<fastrand::Rng>);

impl Default for StitchRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// spawn_stitches（0x141ce65b0）：把缝合描述细分为悬浮砖参数（只缝短段）。
pub fn spawn_stitches(
    mut params: ResMut<FloatingBrickParams>,
    descriptions: Res<StitchDescriptions>,
    mut rng: ResMut<StitchRng>,
) {
    let r = &mut rng.0.lock().unwrap();
    params.0.clear();
    for desc in &descriptions.0 {
        if desc.kind != 1 {
            continue;
        }
        if desc.threshold >= 0.2 {
            continue;
        }
        // 段长检查（O：u = truncate(min(1000, 2*|长|))；2 < u 跳过 → 只缝短段）。
        let u = (1000.0f32.min((2.0 * desc.length.abs())).trunc()) as usize;
        if 2 < u {
            continue;
        }
        // 取段中点 + random_splits 细分（2 段）；单位法向（1/SQRT 归一）+ x0y swizzle
        // （O：每细分点抖动 (r*0.5-(1-r)*0.5)*0.1+0.4、Δu + (-0.1)）。
        let mid = desc.start + desc.length * 0.5;
        let mut bricks = Vec::new();
        for k in 0..2 {
            let t = k as f32 / 2.0;
            let point = desc.start + desc.length * t;
            // 抖动（O）：(r*0.5-(1-r)*0.5)*0.1+0.4；Δu + (-0.1)。
            let rnd = r.f32();
            let jitter = (rnd * 0.5 - (1.0 - rnd) * 0.5) * 0.1 + 0.4;
            let delta = desc.length * 0.5 + (-0.1);
            // 段方向 + 法向（O：1/SQRT 归一化 + x0y swizzle；沿法向偏移 jitter）。
            let seg_dir = desc.length * 0.5;
            let normal = Vec2::new(-seg_dir, seg_dir).normalize_or_zero();
            let pos = x0y(Vec2::new(point, mid)) + Vec3::new(
                normal.x * jitter,
                0.0,
                normal.y * jitter,
            );
            bricks.push(StairsFloatingBrickParams {
                position: pos,
                facing: normal.y.atan2(normal.x),
                direction: 0,
                param: 0,
                jitter,
                delta,
                c24: 0.3,
                v28: Vec2::new(point, mid),
                key: 0,
            });
        }
        params.0.extend(bricks);
    }
}

/// 墙对键（`calculate_hash<([WallId;2],usize)>` 语义；宿主填充）。
pub fn wall_pair_key(a: u64, b: u64, i: usize) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    [a, b].hash(&mut h);
    i.hash(&mut h);
    h.finish()
}

/// spawn_stitch_bricks（0x141ce7190）：把缝合砖参数烘焙进 BrickTransformSsbo 实例
/// 缓冲（清空 → 逐条 push；缓冲上传由宿主 render_loop 接线，GPU 后置）。
pub fn spawn_stitch_bricks(
    mut bricks: ResMut<StitchBrickInstances>,
    params: Res<FloatingBrickParams>,
) {
    bricks.0.clear();
    for p in &params.0 {
        let key = if p.key != 0 { p.key } else { wall_pair_key(p.direction >> 32, p.direction as u32 as u64, 0) };
        bricks.0.push((*p, key));
    }
}

/// 缝合砖实例缓冲（CPU 侧；宿主写 BrickTransformSsbo）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StitchBrickInstances(pub Vec<(StairsFloatingBrickParams, u64)>);

/// `InterShapeBrickStitches`（stitch_bricks.rs PDB 命名实体；clear L148）。
#[derive(Resource, Clone, Debug, Default)]
pub struct InterShapeBrickStitches(pub Vec<StairsFloatingBrickParams>);

/// on_exit_game_state（L124）：退出游戏状态清理。
pub fn on_exit_game_state(mut stitches: ResMut<InterShapeBrickStitches>) {
    stitches.0.clear();
}

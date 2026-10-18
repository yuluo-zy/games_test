//! stitch_bricks（B 主基线；spawn_stitches 0x141ce65b0、spawn_stitch_bricks 0x141ce7190）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x141ce65b0_*.c`、`0x141ce7190_*.c`、`evidence/disasm5_1.txt`；
//! 常量实读 `0x1429cd068=0.2`、`0x14295b630=1000.0`、`0x142925870=0.5`、
//! `0x142925984=0.1`、`0x142925970=0.4`、`0x1429eb6f8=-0.1`、`0x3e99999a=0.3`）。
//!
//! spawn_stitches（O）：
//! - 遍历描述表 stride 0x40：`kind@+0 != 1` 跳过；**`0.2 <= threshold@+4` 跳过**。
//! - 段长检查：`diff = start(+0x14) - length(+0x18)`；`n = trunc(min(1000, 2*|diff|))`；
//!   **`n <= 2` 跳过，`n > 2` 处理**（0x141ce6912 `CMP RSI,2; JA process`）。
//! - `random_splits_into(n, 0.2/|diff|, rng)` 得 u 细分；每细分点：
//!   `jitter = (r*0.5-(1-r)*0.5)*0.1+0.4`；`delta = Δu + (-0.1)`；单位方向 + 法向
//!   （1/SQRT，x0y/xvy swizzle）；产出 `StairsFloatingBrickParams` stride 0x40
//!   （4 float 位置/朝向、+0x10 方向 qword、+0x18 param、+0x1c jitter、+0x20 delta、
//!   +0x24=0.3、+0x28 Vec2、+0x38 键）。
//!
//! spawn_stitch_bricks（O）：FrozenMap 取 `BrickTransformSsbo`；`SparseInstanceBufferWriter::clear`；
//! 逐 params：`resources::render::impl$1::from` → `calculate_hash<([WallId;2],usize)>` →
//! `geometry::instanced_wall::impl$1::from` → `push<BrickTransformSsbo>`。
//! 本 crate 以 [`StitchBrickInstances`] 承载 CPU 侧实例（GPU 写入为宿主 render 边界 U）。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::swizzlers::x0y;

use country_core::systems::wall::construction::bricks::random_splits_into;

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
    /// +0x24：0.3 常量（0x3e99999a）。
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
        // O：kind==1 才处理。
        if desc.kind != 1 {
            continue;
        }
        // O：`0.2 <= threshold` 跳过。
        if desc.threshold >= 0.2 {
            continue;
        }
        // O：diff = start - length；n = trunc(min(1000, 2*|diff|))。
        let diff = desc.start - desc.length;
        let span = diff.abs();
        let n = (1000.0f32.min(2.0 * span).trunc()) as usize;
        // O：n<=2 跳过；n>2 处理（0x141ce6912 `CMP RSI,2; JA process`）。
        if n <= 2 {
            continue;
        }
        // O：random_splits_into(n, 0.2/|diff|, rng)。
        let mut splits = Vec::new();
        random_splits_into(n, 0.2 / span.max(f32::MIN_POSITIVE), r, &mut splits);
        // U：段几何（宿主 curve/slot 数据）在本 crate 以 start/length 沿 X 的合成段承载；
        // random_splits 的 u 按该段插值（方向/法向与真实缝合曲线为 I）。
        let a = Vec3::new(desc.start, 0.0, 0.0);
        let b = Vec3::new(desc.start + desc.length, 0.0, 0.0);
        let dir = (b - a).normalize_or_zero();
        let normal = Vec2::new(-dir.z, dir.x);
        let mut prev = a;
        for (k, &t) in splits.iter().enumerate() {
            let point = a.lerp(b, t);
            // O：抖动 `(r*0.5-(1-r)*0.5)*0.1+0.4`。
            let rnd = r.f32();
            let jitter = (rnd * 0.5 - (1.0 - rnd) * 0.5) * 0.1 + 0.4;
            // O：delta = Δu + (-0.1)。
            let delta = (point - prev).length() * 0.5 + (-0.1);
            let pos = x0y(Vec2::new(point.x + normal.x * jitter, point.z + normal.y * jitter));
            params.0.push(StairsFloatingBrickParams {
                position: pos,
                facing: normal.y.atan2(normal.x),
                direction: ((dir.x.to_bits() as u64) << 32) | dir.z.to_bits() as u64,
                param: k as u32,
                jitter,
                delta,
                c24: 0.3,
                v28: Vec2::new(point.x, point.z),
                key: 0,
            });
            prev = point;
        }
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
    // O：SparseInstanceBufferWriter::clear。
    bricks.0.clear();
    for p in &params.0 {
        // O：calculate_hash<([WallId;2],usize)>（宿主 direction qword 承载墙对）。
        let key = if p.key != 0 {
            p.key
        } else {
            wall_pair_key(p.direction >> 32, p.direction as u32 as u64, p.param as usize)
        };
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

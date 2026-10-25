//! ivy_direction_proposer.rs — 常春藤生长方向提议（WP16 权威；WP21 逻辑重述）。
//!
//! O：`IvyDirectionProposer`（两个 FastNoise + 常量 1.0/2.0/1.3；`impl$0::default`
//! 0x1412343b0、`get_direction` 0x141234590：θ 混合 → (cosθ,sinθ)）。`IvySpawnNoise::new`。
//! 复用边界：bracket-noise 0.8.2（缓存无 → typed gap）。

use bevy_ecs::resource::Resource;
use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

/// 常春藤方向提议器（O：`Res<IvyDirectionProposer>`；grower 采样生长方向）。
///
/// 布局 O（`impl$0::default` 0x1412343b0）：两个 `bracket_noise::FastNoise`
/// （seeded + fractal octaves/gain + seed）+ 参数（0x3f800000=1.0、0x40000000=2.0、
/// 0x3fa66666=1.3，O 常量）。
#[derive(Resource)]
pub struct IvyDirectionProposer {
    /// 场 A（O：bracket_noise FastNoise）。
    pub noise_a: crate::reused::noise::FastNoise,
    /// 场 B（O）。
    pub noise_b: crate::reused::noise::FastNoise,
    /// 种子/参数（O 常量见 default；typed gap）。
    pub _params: Gap,
}

impl Default for IvyDirectionProposer {
    fn default() -> Self {
        IvyDirectionProposer {
            noise_a: crate::reused::noise::FastNoise::seeded(0),
            noise_b: crate::reused::noise::FastNoise::seeded(0),
            _params: Gap,
        }
    }
}

impl IvyDirectionProposer {
    /// `new`（O：`insert_ivy_resources` 内联 `IvySpawnNoise::new`；这里合并到 default）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `get_direction`（O 0x141234590）：3×noise3d + 3×uniform_f32 混合，
    /// 得到 `θ = ((v*80 - (1-v)*80) * DEG2RAD * 0.5)`（v 归一化到 [0,1]），
    /// 返回 `(cos θ, sin θ)`。I：噪声采样的具体坐标映射。
    pub fn get_direction(&mut self, _pos: Vec3) -> Vec2 {
        // O：θ 范围对称 [-40°, +40°]（80° × 0.5）；实际 noise 采样 U。
        let theta = 0.0f32; // U
        Vec2::new(theta.cos(), theta.sin())
    }
}

/// 常春藤生成噪声参数（O：PDB 实体；`insert_ivy_resources` 内联）。
#[derive(Clone, Copy, Debug, Default)]
pub struct IvySpawnNoise(pub Gap);

impl IvySpawnNoise {
    pub fn new() -> Self {
        Self::default()
    }
}

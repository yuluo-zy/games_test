//! ivy_direction_proposer.rs — 常春藤生长方向提议（WP16 逻辑重述）。
//!
//! O：`IvyDirectionProposer`（两个 FastNoise + 常量 1.0/2.0/1.3；`impl$0::default`
//! 0x1412343b0、`get_direction` 0x141234590：θ 混合 → (cosθ,sinθ)）。`IvySpawnNoise::new`。
//! 复用边界：bracket-noise 0.8.2（缓存无 → typed gap）。

use bevy_ecs::resource::Resource;
use glam::Vec2;

use crate::typed_gap::Gap;

/// `IvyDirectionProposer`（O：两个 FastNoise + 常量）。
#[derive(Resource, Clone, Debug, Default)]
pub struct IvyDirectionProposer {
    /// FastNoise 0（O：seeded + fractal octaves/gain；bracket-noise 复用边界）。
    pub noise_a: Gap,
    /// FastNoise 1（O）。
    pub noise_b: Gap,
    /// 常量 1.0 / 2.0 / 1.3（O：0x3f800000/0x40000000/0x3fa66666）。
    pub const_a: f32,
    pub const_b: f32,
    pub const_c: f32,
}

impl IvyDirectionProposer {
    /// `default`（O 0x1412343b0）。
    pub fn new() -> Self {
        Self {
            noise_a: Gap::default(),
            noise_b: Gap::default(),
            const_a: 1.0,
            const_b: 2.0,
            const_c: 1.3,
        }
    }
    /// `get_direction`（O 0x141234590）：3×get_noise3d + 3×uniform_f32 混合 →
    /// `θ = ((v*80 − (1−v)*80) * DEG2RAD * 0.5)`（v 归一 [0,1]）→ `(cosθ, sinθ)`。
    pub fn get_direction(&self, _x: f32, _y: f32, _z: f32) -> Vec2 {
        // 噪声坐标映射细节 U（wp16）；方向混合语义 O。
        let theta: f32 = 0.0;
        Vec2::new(theta.cos(), theta.sin())
    }
}

/// `IvySpawnNoise`（O：`new` 0x1412343b0 同 default 折叠面）。
#[derive(Clone, Debug, Default)]
pub struct IvySpawnNoise(pub Gap);

impl IvySpawnNoise {
    pub fn new() -> Self {
        Self::default()
    }
}

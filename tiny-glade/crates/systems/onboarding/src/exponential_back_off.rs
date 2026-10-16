//! exponential_back_off（0x142284800，源码 L0）。
//!
//! 证据（skeleton PDB）：raw_vec + f32::_ —— 资源化退避计时。语义：提示重复触发
//! 的指数退避（触发间隔逐次加倍，封顶后复位）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

/// 指数退避计时（资源）。
#[derive(Resource, Clone, Debug)]
pub struct ExponentialBackOff {
    /// 各提示类型的退避剩余时间。
    pub remaining: [f32; 16],
    /// 各提示类型的触发计数（决定退避底数）。
    pub strikes: [u32; 16],
}

impl Default for ExponentialBackOff {
    fn default() -> Self {
        Self {
            remaining: [0.0; 16],
            strikes: [0; 16],
        }
    }
}

/// 退避底数（O 语义：触发间隔 2^n 指数加倍；基础 0.5s）。
pub const BACKOFF_BASE_S: f32 = 0.5;

/// exponential_back_off（0x142284800）：计时推进 + 查询门控。
pub fn exponential_back_off(
    time: Res<GameTime>,
    mut back_off: ResMut<ExponentialBackOff>,
) {
    let dt = time.delta;
    for r in &mut back_off.remaining {
        *r = (*r - dt).max(0.0);
    }
}

impl ExponentialBackOff {
    /// 触发（返回是否允许显示：退避已过）。
    pub fn trigger(&mut self, hint: usize) -> bool {
        let hint = hint.min(15);
        if self.remaining[hint] > 0.0 {
            return false;
        }
        self.strikes[hint] += 1;
        let base = BACKOFF_BASE_S * (1u64 << self.strikes[hint].min(6)) as f32;
        self.remaining[hint] = base;
        true
    }
    /// 重置（提示完成后）。
    pub fn reset(&mut self, hint: usize) {
        let hint = hint.min(15);
        self.remaining[hint] = 0.0;
        self.strikes[hint] = 0;
    }
}

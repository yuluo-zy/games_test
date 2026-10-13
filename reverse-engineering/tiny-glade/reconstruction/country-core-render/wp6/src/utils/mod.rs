//! utils/mod.rs — WP6 恢复的游戏 utils（Transform + float_grid gap）。

#![allow(dead_code, unused_imports)]

pub mod components;

/// `utils::float_grid`（O：`bilinear_sample_at_world_xz<RasterBufferRef<512>>` 的宿主；
/// 布局 U）。
pub mod float_grid {
    /// 双线性世界 XZ 采样（O 名称；实现 U）。
    pub fn bilinear_sample_at_world_xz<T>(_grid: &T, _x: f32, _z: f32) -> f32 {
        todo!("recover bilinear_sample_at_world_xz")
    }
}

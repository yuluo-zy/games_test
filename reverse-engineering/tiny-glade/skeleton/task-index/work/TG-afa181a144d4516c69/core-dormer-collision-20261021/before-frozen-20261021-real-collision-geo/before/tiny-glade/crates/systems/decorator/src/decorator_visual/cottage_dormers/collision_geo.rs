//! B `cottage_dormers/collision_geo.rs` — 老虎窗碰撞几何（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x14128a0b0（get_collision_mesh，50KB）+ 骨架
//! callees（O）：DormerKey::quantized_from（内联）、inverse_lerp、swizzlers
//! xvy ×7、递归 get_collision_mesh（自调用 1 次）、FUN_141289c80 ×2（子网格
//! 构建）。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 缓存键量化：`round(dist * 5.0)`（0x40a00000 = 5.0）；
//! - 键布局 `{i32 x, i32 z, i32 count, i8 roof_idx}`（按字段位置命名，语义 U）；
//! - 网格构建递归一次 + 两子网格构造（FUN_141289c80 ×2），数学主体 U。
//!
//! U 登记：① 网格细分算法（三角带采样） ② x/z/count 的精确来源。

use glam::Vec2;
use std::collections::HashMap;
use std::sync::Arc;

/// 量化尺度（O：0x40a00000 = 5.0）。
pub const DORMER_KEY_QUANT_SCALE: f32 = 5.0;

/// 老虎窗碰撞网格缓存键（O 布局）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DormerKey {
    /// i32 @+0：量化 x。
    pub x: i32,
    /// i32 @+4：量化 z。
    pub z: i32,
    /// i32 @+8：计数（语义 U）。
    pub count: i32,
    /// i8 @+0xc：屋顶索引（语义 U）。
    pub roof_idx: i8,
}

impl DormerKey {
    /// 量化（O：`round(dist*5)`）。
    #[inline]
    pub fn quantize(v: f32) -> i32 {
        (v * DORMER_KEY_QUANT_SCALE).round() as i32
    }

    /// 由 XZ 位置量化（O 结构；x/z 的来源 U）。
    pub fn quantized_from_xz(pos: Vec2, count: i32, roof_idx: i8) -> Self {
        Self {
            x: Self::quantize(pos.x),
            z: Self::quantize(pos.y),
            count,
            roof_idx,
        }
    }

    /// 由单一距离量化（O 公式；`count`/`roof_idx` 显式给出）。
    pub fn quantized_from_dist(dist: f32, count: i32, roof_idx: i8) -> Self {
        Self {
            x: Self::quantize(dist),
            z: 0,
            count,
            roof_idx,
        }
    }
}

/// 碰撞网格缓存（O：`HashMap<DormerKey, Arc<RaycastMeshData>>`；这里以
/// 顶点列表承载几何，RaycastMeshData 由宿主构建）。
#[derive(Clone, Debug, Default)]
pub struct DormerCollisionCache {
    map: HashMap<DormerKey, Arc<Vec<Vec3>>>,
}

use glam::Vec3;

impl DormerCollisionCache {
    /// 查询或构建（O：缓存命中直接返回；未命中调用构建闭包）。
    pub fn get_or_build<F>(&mut self, key: DormerKey, build: F) -> Arc<Vec<Vec3>>
    where
        F: FnOnce(DormerKey) -> Vec<Vec3>,
    {
        if let Some(v) = self.map.get(&key) {
            return Arc::clone(v);
        }
        let mesh = Arc::new(build(key));
        self.map.insert(key, Arc::clone(&mesh));
        mesh
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// 0x14128a0b0（O 结构）：老虎窗碰撞网格。
///
/// 原函数为递归细分 + 三角带采样；数学主体 U，因此本函数**显式返回空**
/// （不以伪造顶点冒充），缓存/量化结构已按机器码落地。调用方应通过
/// [`DormerCollisionCache::get_or_build`] 在取证完成后插入真实构建闭包。
pub fn get_collision_mesh(cache: &mut DormerCollisionCache, key: DormerKey) -> Arc<Vec<Vec3>> {
    cache.get_or_build(key, |_key| Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantization_rounds_at_five_times() {
        assert_eq!(DormerKey::quantize(0.0), 0);
        assert_eq!(DormerKey::quantize(0.1), 1); // 0.5 → round away from zero
        assert_eq!(DormerKey::quantize(0.09), 0);
        assert_eq!(DormerKey::quantize(-0.1), -1);
        assert_eq!(DormerKey::quantize(2.0), 10);
    }

    #[test]
    fn key_layout_roundtrip() {
        let k = DormerKey::quantized_from_xz(Vec2::new(0.2, -0.3), 4, 2);
        assert_eq!(k, DormerKey { x: 1, z: -2, count: 4, roof_idx: 2 });
    }

    #[test]
    fn cache_reuses_built_mesh() {
        let mut cache = DormerCollisionCache::default();
        let key = DormerKey::quantized_from_dist(1.0, 1, 0);
        let mut calls = 0;
        let a = cache.get_or_build(key, |_| {
            calls += 1;
            vec![Vec3::ZERO]
        });
        let _b = cache.get_or_build(key, |_| {
            calls += 1;
            vec![]
        });
        assert_eq!(calls, 1);
        assert_eq!(a.len(), 1);
        assert_eq!(cache.len(), 1);
    }
}

//! B `resources/walls/cached_decorator_transforms.rs` — 装饰变换缓存资源（原 11 函数：
//! 业务 7）。
//!
//! 证据：`chapters/resources/walls/cached_decorator_transforms.rs.md` + `wall/decomp/*`。
//! 作用：墙高/几何变化后装饰重定位的缓存；缓存条目 = 10 f32 紧凑变换
//! `{rotation: Quat@[0..4], translation: Vec3@[4..7], scale: Vec3@[7..10]}`，
//! `insert` 0x140a1ada0 按 3 批做有限性（NaN/Inf）校验并 panic。
//! **纯数据缓存，非实体存储**（卡5 实体机制一致：实体由消费系统读后生成）。
//!
//! Original cache owner is the current map; previous-frame scratch belongs to
//! cache_decorator_transforms's Local<CachedDecoratorTransformStorage>.

use std::collections::HashMap;

use glam::{Quat, Vec3};

/// B 缓存变换条目：紧凑 Transform（10 f32 分量布局，insert 0x140a1ada0 读面 O）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CachedDecoratorTransform {
    /// mat[0..4]：rotation（Quat；有限性校验 movmskps==0xf 全 4 分量）。
    pub rotation: Quat,
    /// mat[4..7]：translation（Vec3）。
    pub translation: Vec3,
    /// mat[7..10]：scale（Vec3）。
    pub scale: Vec3,
}

impl CachedDecoratorTransform {
    /// 由 10 个 f32 分量构建（分量序 {rotation[0..4], translation[4..7], scale[7..10]}）。
    pub fn from_components(components: &[f32; 10]) -> Self {
        Self {
            rotation: Quat::from_array(components[0..4].try_into().unwrap()),
            translation: Vec3::from_array(components[4..7].try_into().unwrap()),
            scale: Vec3::from_array(components[7..10].try_into().unwrap()),
        }
    }

    /// 写回 10 个 f32 分量。
    pub fn to_components(&self) -> [f32; 10] {
        let mut out = [0.0; 10];
        out[0..4].copy_from_slice(&self.rotation.to_array());
        out[4..7].copy_from_slice(&self.translation.to_array());
        out[7..10].copy_from_slice(&self.scale.to_array());
        out
    }
}

/// B CachedDecoratorTransformStorage：缓存存储（HashMap<u32, CachedDecoratorTransform>；
/// `get` 0x140a1af40 探针头 {ctrl@+0, mask@+8, len@+0x10, seed@+0x18, hasher@+0x20}）。
#[derive(Debug, Default)]
pub struct CachedDecoratorTransformStorage {
    map: HashMap<u32, CachedDecoratorTransform>,
}

impl Clone for CachedDecoratorTransformStorage {
    fn clone(&self) -> Self { Self { map: self.map.clone() } }
    fn clone_from(&mut self, source: &Self) {
        // 原回滚覆盖同一缓存表，不额外产生一张临时map。
        self.map.clone_from(&source.map);
    }
}

#[cfg(test)]
mod backup_clone_tests {
    use super::*;
    #[test]
    fn backup_transform_clone_from_reuses_matching_capacity_without_aliasing() {
        let mut source = CachedDecoratorTransformStorage { map: HashMap::with_capacity(64) };
        let mut target = CachedDecoratorTransformStorage { map: HashMap::with_capacity(64) };
        let transform = CachedDecoratorTransform { rotation: Quat::IDENTITY, translation: Vec3::X, scale: Vec3::ONE };
        source.insert(7, transform);
        for id in 0..64 { target.insert(id, CachedDecoratorTransform::default()); }
        let capacity = target.map.capacity();
        target.clone_from(&source);
        assert_eq!(target.map.capacity(), capacity);
        assert_eq!(target.map.len(), 1);
        source.clear();
        assert_eq!(target.get(7), Some(&transform));
    }
}

impl CachedDecoratorTransformStorage {
    /// 0x140a1af40（O）：存储查表（未命中 None）。
    pub fn get(&self, id: u32) -> Option<&CachedDecoratorTransform> {
        self.map.get(&id)
    }

    /// 写入（insert 0x140a1ada0 校验后经 FUN_14152aee0 落缓存）。
    pub(crate) fn insert(&mut self, id: u32, transform: CachedDecoratorTransform) {
        self.map.insert(id, transform);
    }

    /// 清空（swap_buffers_and_clear 重算 cap 后的清册）。
    pub(crate) fn clear(&mut self) {
        self.map.clear();
    }
}

/// Current authoritative transform map. Original cache_decorator_transforms
/// borrows its previous-frame scratch as a Local<CachedDecoratorTransformStorage>;
/// reads after insert observe this current map in the same frame.
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct CachedDecoratorTransforms {
    pub storage: CachedDecoratorTransformStorage,
}

impl CachedDecoratorTransforms {
    fn read(&self) -> &CachedDecoratorTransformStorage {
        &self.storage
    }

    /// 0x140a1aa20（O）：查缓存取变换（未命中 panic，`try_transform` 为 Option 版）。
    pub fn transform(&self, id: u32) -> CachedDecoratorTransform {
        self.read().get(id).copied().unwrap_or_else(|| {
            panic!("CachedDecoratorTransforms::transform: no cached transform for {}", id)
        })
    }

    /// 0x140a1a9f0（O）：查缓存取位置（translation；未命中 panic）。
    pub fn position(&self, id: u32) -> Vec3 {
        self.transform(id).translation
    }

    /// 0x140a1ac80（O）：查缓存取变换（Option 版）。
    pub fn try_transform(&self, id: u32) -> Option<CachedDecoratorTransform> {
        self.read().get(id).copied()
    }

    /// 0x140a1ab80（O）：查缓存取位置（Option 版）。
    pub fn try_position(&self, id: u32) -> Option<Vec3> {
        self.try_transform(id).map(|t| t.translation)
    }

    /// 0x140a1ada0（O，缓存 3×3/紧凑变换 有限性校验）：写入缓存。9/10 分量分 3 批
    /// 有限性校验：
    /// - mat[4..7] translation → panic "assertion failed: transform.translation.is_finite()"
    ///   （0x142ac8e00, line 0x33）
    /// - mat[0..4] rotation（movmskps==0xf 全 4 分量）→ panic "assertion failed:
    ///   transform.rotation.is_finite()"（0x142ac8e98, line 0x30）
    /// - mat[7..10] scale → panic "assertion failed: transform.scale.is_finite()"
    ///   （0x142ac8ee0, line 0x2d）
    /// 全部通过 → FUN_14152aee0 落缓存。
    pub fn insert(&mut self, id: u32, transform: CachedDecoratorTransform) {
        assert!(
            transform.translation.is_finite(),
            "assertion failed: transform.translation.is_finite()"
        );
        assert!(
            transform.rotation.is_finite(),
            "assertion failed: transform.rotation.is_finite()"
        );
        assert!(
            transform.scale.is_finite(),
            "assertion failed: transform.scale.is_finite()"
        );
        self.storage.insert(id, transform);
    }

    /// B140a1aeb0: swap with the actual system-local previous map, then clear
    /// this current destination. The caller can read previous during rebuilding.
    pub fn swap_buffers_and_clear(&mut self, previous: &mut CachedDecoratorTransformStorage) {
        std::mem::swap(&mut self.storage, previous);
        self.storage.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform(x: f32, y: f32, z: f32) -> CachedDecoratorTransform {
        CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(x, y, z),
            scale: Vec3::ONE,
        }
    }

    #[test]
    fn insert_then_try_read_and_read() {
        let mut cache = CachedDecoratorTransforms::default();
        cache.insert(7, transform(1.0, 2.0, 3.0));
        // The authoritative current map is readable immediately after insert.
        assert_eq!(cache.try_position(7), Some(Vec3::new(1.0, 2.0, 3.0)));
        assert_eq!(cache.position(7), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(cache.transform(7), transform(1.0, 2.0, 3.0));
        let mut previous = CachedDecoratorTransformStorage::default();
        cache.swap_buffers_and_clear(&mut previous);
        assert!(cache.try_position(7).is_none());
        assert_eq!(previous.get(7).copied(), Some(transform(1.0, 2.0, 3.0)));
        cache.insert(7, transform(4.0, 5.0, 6.0));
        assert_eq!(cache.position(7), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(previous.get(7).copied(), Some(transform(1.0, 2.0, 3.0)));
    }

    #[test]
    fn components_roundtrip() {
        let t = CachedDecoratorTransform {
            rotation: Quat::from_rotation_y(0.7),
            translation: Vec3::new(1.0, 2.0, 3.0),
            scale: Vec3::new(2.0, 1.0, 0.5),
        };
        let comps = t.to_components();
        assert_eq!(CachedDecoratorTransform::from_components(&comps), t);
    }

    #[test]
    fn insert_panics_on_non_finite() {
        let mut cache = CachedDecoratorTransforms::default();
        // translation NaN → panic。
        let bad = CachedDecoratorTransform {
            translation: Vec3::new(f32::NAN, 0.0, 0.0),
            ..transform(0.0, 0.0, 0.0)
        };
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cache.insert(1, bad);
        }))
        .is_err());
        // rotation Inf → panic。
        let bad = CachedDecoratorTransform {
            rotation: Quat::from_array([f32::INFINITY, 0.0, 0.0, 1.0]),
            ..transform(0.0, 0.0, 0.0)
        };
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cache.insert(2, bad);
        }))
        .is_err());
        // scale NaN → panic。
        let bad = CachedDecoratorTransform {
            scale: Vec3::new(1.0, f32::NAN, 1.0),
            ..transform(0.0, 0.0, 0.0)
        };
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cache.insert(3, bad);
        }))
        .is_err());
    }

    #[test]
    fn storage_get_missing() {
        let storage = CachedDecoratorTransformStorage::default();
        assert!(storage.get(99).is_none());
    }
}

//! cached_decorator_transforms.rs — 装饰缓存变换（WP12 系统向 facade）。
//!
//! O 名；布局 U（typed gap）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::walls::decorator::DecoratorId;
use crate::typed_gap::Gap;

/// 缓存装饰变换（O 名；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct CachedDecoratorTransforms {
    pub raw: Gap,
}

impl CachedDecoratorTransforms {
    /// O：按 id 取缓存位置。
    pub fn try_position(&self, _id: DecoratorId) -> Option<glam::Vec3> {
        None
    }
    /// O：按 id 取缓存变换。
    pub fn try_transform(
        &self,
        _id: DecoratorId,
    ) -> Option<crate::utils::components::transform::Transform> {
        None
    }
}

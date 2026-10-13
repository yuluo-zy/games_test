//! decorator_archivist.rs — 装饰归档/剪枝资源（WP12 系统向 facade）。
//!
//! O 名；布局 U（typed gap）。WP12 墙系统以 `Res<DecoratorArchivist>` /
//! 裸 `DecoratorArchivalResources` 消费。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use bevy_ecs::system::SystemParam;

use crate::typed_gap::Gap;

/// 装饰归档资源（O 名；`merge_walls` / `finish_edit_rectangle` 消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorArchivist {
    pub raw: Gap,
}

impl DecoratorArchivist {
    /// O：装饰剔除放行（cull 允许任意/地形/楼梯）。
    pub fn decorator_cull_allow_any(&self) -> bool {
        true
    }
    pub fn decorator_cull_allow_terrain(&self) -> bool {
        true
    }
    pub fn decorator_cull_allow_stairs(&self) -> bool {
        true
    }
}

/// SystemParam-style resources bundle the wall/roof systems read。
#[derive(SystemParam, Debug)]
pub struct DecoratorArchivalResources<'w, 's> {
    _marker: std::marker::PhantomData<(&'w (), &'s ())>,
}

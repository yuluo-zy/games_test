//! decorator_storage.rs — 装饰存储资源（WP12 系统向 facade）。
//!
//! O 名；布局 U（typed gap）。`finish_edit_rectangle` 以 `ResMut<DecoratorStorage>` 消费。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::walls::decorator::DecoratorId;
use crate::typed_gap::Gap;

/// 装饰存储（O 名；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorStorage {
    pub raw: Gap,
}

impl DecoratorStorage {
    /// O：按 id 查装饰信息。
    pub fn decorator_info(&self, _id: DecoratorId) -> Option<Gap> {
        None
    }
    /// O：遍历地形装饰。
    pub fn iter_terrain_decorators(&self, _pred: impl FnMut(DecoratorId)) {}
    /// O：遍历楼梯装饰。
    pub fn iter_stair_decorators(&self, _pred: impl FnMut(DecoratorId)) {}
}

/// 装饰存储地址（O 名；布局 U）。
#[derive(Clone, Copy, Debug)]
pub enum DecoratorStorageAddr {
    Wall(crate::resources::walls::WallId),
}

impl DecoratorStorageAddr {
    pub fn as_wall(&self) -> Option<crate::resources::walls::WallId> {
        match self {
            DecoratorStorageAddr::Wall(w) => Some(*w),
        }
    }
}

/// 装饰集合（O 名；布局 U）。
#[derive(Clone, Debug)]
pub struct Decorators {
    pub raw: Gap,
}

impl Decorators {
    pub fn iter_maybe_mut(&mut self, _f: impl FnMut(DecoratorId)) {}
}

/// 可空装饰引用（O 名；布局 U）。
#[derive(Clone, Debug)]
pub struct DecoratorMaybeMut<'a> {
    pub raw: Gap,
    pub _p: std::marker::PhantomData<&'a ()>,
}

impl<'a> DecoratorMaybeMut<'a> {
    pub fn get_mut(&self) -> Option<Gap> {
        None
    }
}

/// 空装饰态（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct EmptyDecoratorState {
    pub raw: Gap,
}

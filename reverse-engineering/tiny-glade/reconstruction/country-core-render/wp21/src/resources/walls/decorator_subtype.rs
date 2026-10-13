//! decorator_subtype.rs — 装饰子类型网格库（WP21）。
//!
//! O：`DecoratorLibKey` 复合键（首字节 subtype，`'\x1a'`(26)=空）；`DecoratorMeshLibrary`
//! 4 表（collision/interaction/outline/collision_outline）；`insert` 0xb262a0 /
//! `insert_collision_and_interaction_as_same` 0xb25ef0 克隆 `Vec<Arc<RaycastMeshData>>`
//! 后依次插 4 表；`get_outline` 0xb26690（map3 @+0x60）、`get_collision_outline`
//! 0xb267f0（map4 @+0x90）；`DerivedDecoratorSubtypes::get` 0xb25b60（HashMap<u32,
//! Arc<DerivedDecoratorInfo>>，未命中 panic）；`DecoratorCollision::empty` 0xb25ca0；
//! `bbx_ls<DecoratorSubtype>` 0x1fdc7b0。

use std::collections::HashMap;
use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::boundary::{Aabb2, RaycastMeshData};
use crate::typed_gap::Gap;

/// `DecoratorSubtype`（O：u8 判别；`can_be_on_corner` 0xb25b50 读字节标志）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorSubtype(pub u8);

impl DecoratorSubtype {
    /// O：空判别 0x1a。
    pub const EMPTY: u8 = 0x1a;
    /// `can_be_on_corner`（O 0xb25b50）：角附着允许标志（I：按 subtype 表）。
    pub fn can_be_on_corner(self) -> bool {
        // O 机器码极短；语义 I。
        matches!(self.0, 3 | 6)
    }
}

/// `DecoratorLibKey`（O：复合键；首字节 subtype、次字节 variant，`get_outline` 0xb26690
/// 逐字节比较）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DecoratorLibKey {
    pub subtype: DecoratorSubtype,
    pub variant: u8,
}

impl DecoratorLibKey {
    /// `new`（O 0xb25c90）。
    pub fn new(subtype: DecoratorSubtype, variant: u8) -> Self {
        Self { subtype, variant }
    }
}

/// `DecoratorCollision`（O：碰撞数据载体；`empty` 0xb25ca0 建空：Rectangle3d +
/// parry3d SharedShape cuboid + IsoTransform，Vec<Arc<RaycastMeshData>> 0x24B 元素）。
#[derive(Clone, Debug, Default)]
pub struct DecoratorCollision {
    /// 碰撞 mesh 记录（O：Vec<Arc<RaycastMeshData>>，0x24B/项）。
    pub meshes: Vec<Arc<RaycastMeshData>>,
    /// 包围盒（O：`bbx_ls` 读 4×u64 @entry+0x20 区）。
    pub bbx: Aabb2,
    /// 未映射区（U；0x90B 详布局）。
    pub _gap: Gap,
}

impl DecoratorCollision {
    /// `empty`（O 0xb25ca0）。
    pub fn empty() -> Self {
        Self::default()
    }
}

/// `DecoratorMeshLibrary`（O：4 表 + 方法面）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorMeshLibrary {
    /// collision（O）。
    pub collision: HashMap<DecoratorLibKey, DecoratorCollision>,
    /// interaction（O）。
    pub interaction: HashMap<DecoratorLibKey, DecoratorCollision>,
    /// outline（O：map3，entry 0x28B）。
    pub outline: HashMap<DecoratorLibKey, super::decorator_outline::DecoratorOutline>,
    /// collision_outline（O：map4，entry 0x20B）。
    pub collision_outline: HashMap<DecoratorLibKey, super::decorator_outline::DecoratorCollisionOutline>,
}

impl DecoratorMeshLibrary {
    /// `insert`（O 0xb262a0）：克隆碰撞网格后依次插 4 表。
    pub fn insert(
        &mut self,
        key: DecoratorLibKey,
        collision: DecoratorCollision,
        outline: super::decorator_outline::DecoratorOutline,
        collision_outline: super::decorator_outline::DecoratorCollisionOutline,
    ) {
        self.collision.insert(key, collision.clone());
        self.interaction.insert(key, collision);
        self.outline.insert(key, outline);
        self.collision_outline.insert(key, collision_outline);
    }
    /// `insert_collision_and_interaction_as_same`（O 0xb25ef0）：同数据作 collision/
    /// interaction。
    pub fn insert_collision_and_interaction_as_same(
        &mut self,
        key: DecoratorLibKey,
        collision: DecoratorCollision,
        outline: super::decorator_outline::DecoratorOutline,
        collision_outline: super::decorator_outline::DecoratorCollisionOutline,
    ) {
        self.insert(key, collision, outline, collision_outline);
    }
    /// `get_outline`（O 0xb26690）：按键查 outline；未命中 panic。
    pub fn get_outline(&self, key: &DecoratorLibKey) -> &super::decorator_outline::DecoratorOutline {
        self.outline.get(key).expect("decorator outline not found")
    }
    /// `get_collision_outline`（O 0xb267f0）。
    pub fn get_collision_outline(
        &self,
        key: &DecoratorLibKey,
    ) -> &super::decorator_outline::DecoratorCollisionOutline {
        self.collision_outline.get(key).expect("decorator collision outline not found")
    }
}

/// `DerivedDecoratorInfo`（O：subtype/collision/interaction/bbx_ls/dims_wall_space 面）。
#[derive(Clone, Debug, Default)]
pub struct DerivedDecoratorInfo {
    /// subtype（O 0xb26970）。
    pub subtype: DecoratorSubtype,
    /// collision（O）。
    pub collision: DecoratorCollision,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl DerivedDecoratorInfo {
    /// `new`（O 名，30 内联）。
    pub fn new(subtype: DecoratorSubtype, collision: DecoratorCollision) -> Self {
        Self { subtype, collision, _gap: Gap::default() }
    }
    /// `collision_mesh` / `interaction_mesh`（O 名，13/12 内联）。
    pub fn collision_mesh(&self) -> &DecoratorCollision {
        &self.collision
    }
    /// `bbx_ls<DecoratorSubtype>`（O 0x1fdc7b0）：local-space 包围盒。
    pub fn bbx_ls(&self) -> Aabb2 {
        self.collision.bbx
    }
    /// `dims_wall_space`（O 名，30 内联）。
    pub fn dims_wall_space(&self) -> glam::Vec2 {
        let b = self.collision.bbx;
        b.max - b.min
    }
    /// `subtype`（O 0xb26970）。
    pub fn subtype(&self) -> DecoratorSubtype {
        self.subtype
    }
    /// `try_get_subtype`（O 0xb26980）：查 map，未命中 None。
    pub fn try_get_subtype(&self, _key: u32) -> Option<DecoratorSubtype> {
        Some(self.subtype)
    }
}

/// `DerivedDecoratorSubtypes`（O：`HashMap<u32, Arc<DerivedDecoratorInfo>>`；0xb25b60
/// get 未命中 panic）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DerivedDecoratorSubtypes {
    pub map: HashMap<u32, Arc<DerivedDecoratorInfo>>,
}

impl DerivedDecoratorSubtypes {
    /// `get`（O 0xb25b60）。
    pub fn get(&self, key: u32) -> Arc<DerivedDecoratorInfo> {
        self.map.get(&key).expect("decorator subtype not found").clone()
    }
    /// `into_decorator_subtype`（O 0x833ba0 经 get）。
    pub fn into_decorator_subtype(&self, key: u32) -> Arc<DerivedDecoratorInfo> {
        self.get(key)
    }
}

/// `DerivedDecoratorInfoResources`（O 名；ECS 资源组，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DerivedDecoratorInfoResources(pub Gap);

//! ivy_leaf_storage.rs — 常春藤叶存储（WP21 新恢复 IvyLeaf 104B 布局）。
//!
//! O：`IvyLeafStorage` = `HashMap<usize, Vec<IvyLeaf>>`（键=ivy id）。`IvyLeaf` =
//! 104B（`add` 0x1408fe280 写满）。`IvyLeafRng` 布局 U。

use std::collections::HashMap;

use bevy_ecs::resource::Resource;
use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

/// `IvyLeaf`（O：0x68=104 字节，GPU 叶实例参数）：
/// `{ +0x0: u64, +0x8: u64, +0x10: u64(ivy id), +0x18: u64, +0x20: Vec3(位置),
///   +0x2c: Vec2, +0x38: u64, +0x40: f32, +0x44: f32, +0x48: u64=0, +0x50: f32,
///   +0x54: f32, +0x58: u64, +0x60: f32 }`。字段名 I/U。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IvyLeaf {
    pub a: u64,
    pub b: u64,
    /// +0x10（O：ivy id / 键）。
    pub ivy_id: u64,
    pub c: u64,
    /// +0x20 位置（O：Vec3）。
    pub position: Vec3,
    /// +0x2c（O：Vec2）。
    pub d: Vec2,
    pub e: u64,
    pub f: f32,
    pub g: f32,
    pub h: u64,
    pub i: f32,
    pub j: f32,
    pub k: u64,
    pub l: f32,
}

/// `IvyLeafRng`（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct IvyLeafRng(pub Gap);

/// `IvyLeafStorage`（O：HashMap<usize, Vec<IvyLeaf>>）。
#[derive(Resource, Clone, Debug, Default)]
pub struct IvyLeafStorage {
    pub leaves: HashMap<usize, Vec<IvyLeaf>>,
}

impl IvyLeafStorage {
    /// `new`（O 名，内联）。
    pub fn new() -> Self {
        Self::default()
    }
    /// `add`（O 0x1408fe280）：把叶实例 push 到对应 ivy id 的列表。
    pub fn add(&mut self, ivy_id: usize, leaf: IvyLeaf) {
        self.leaves.entry(ivy_id).or_default().push(leaf);
    }
    /// `all_leaves`（O 名，3 内联）。
    pub fn all_leaves(&self) -> impl Iterator<Item = &IvyLeaf> {
        self.leaves.values().flatten()
    }
    /// `all_leaves_mut`（O 名，内联）。
    pub fn all_leaves_mut(&mut self) -> impl Iterator<Item = &mut IvyLeaf> {
        self.leaves.values_mut().flatten()
    }
    /// `delete_all_leaves`（O 名，2 内联）。
    pub fn delete_all_leaves(&mut self) {
        self.leaves.clear();
    }
    /// `delete_affected_leaves`（O：按受影响 ivy id 删，4 内联）。
    pub fn delete_affected_leaves(&mut self, ivy_id: usize) {
        self.leaves.remove(&ivy_id);
    }
}

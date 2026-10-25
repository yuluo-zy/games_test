//! `resources::tree_storage` — 树/植被实例存储（B 骨架 `resources/tree_storage.rs`）。
//!
//! 反编译（Ghidra）：
//!   - `TreeStorage::insert`  0x140a38560  hashbrown `insert<u64,…>` + `next_index = max(next_index, index+1)`
//!   - `TreeStorage::get`     0x140a38460  先判 hashbrown bucket_mask（`+0x18`）==0 → None，否则表内查找
//!   - `TreeStorage::remove`  0x140a385a0  hashbrown `remove_entry`
//!   - `TreeStorage::next_index` 0x140a38600 `next_index += 1`
//!   - `impl$11/impl$12::register_required_components` 0x140a3d320 / 0x140a3da10（bevy 组件注册）
//! 布局裁决：`+0x18` 是 hashbrown RawTable 的 bucket_mask（空表=0 → get 返回 None）；
//! `+0x30` 是 `next_index`。map 值类型有冲突（tree-tool 反汇编 8B {prefab_index,entity} vs
//! enriched callee `ref_<system_clutter::autoclutter::AutoClutterObject>`）→ **typed gap**。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;
use glam::Vec3;
use hashbrown::HashMap;

use crate::typed_gap::Gap;

/// 树索引 → 记录（O 布局：hashbrown map + `next_index u64`）。
/// 骨架行：Default 内联于 `system_tree::impl$0::build`(:33) / `forest::impl$0::build`(:100)。
#[derive(Resource, Default)]
pub struct TreeStorage {
    /// 树 index → 记录。反编译 get 经 `hash_one<RandomState, ref GlobalId>` → 键为
    /// `GlobalId` = u64 新类型（O）。
    map: HashMap<u64, TreeStorageEntry>,
    /// 单调分配的下一个索引（`+0x30`，O）。
    next_index: u64,
}

/// 单棵已放置树的记录值。
///
/// **冲突登记（typed gap）**：tree-tool 早前恢复（reva 0x140a38460/0x140a38560/0x140a385a0）
/// 为 `{prefab_index u32 @0, entity u32 @4}`（Move 分支低/高 32 位读取）；
/// enriched `insert` callee 签名记为 `ref_<system_clutter::autoclutter::AutoClutterObject>`。
/// 本实现取 8B 候选并注明冲突，不声称是作者原字段。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeStorageEntry {
    /// prefab 查询表下标（tree-tool 证据；O/I）。
    pub prefab_index: u32,
    /// 预制体实体号（tree-tool 证据；O/I）。
    pub entity: u32,
}

impl TreeStorage {
    /// `TreeStorage::insert`（行 66，O）：登记 index 并推进 `next_index = max(next_index, index+1)`。
    /// 返回占用状态：槽位先前已存在返回 1，否则 0（tree-tool 证据，I）。
    pub fn insert(&mut self, index: u64) -> u64 {
        let prev = self.map.insert(
            index,
            TreeStorageEntry {
                prefab_index: 0,
                entity: 0,
            },
        );
        self.next_index = self.next_index.max(index + 1);
        u64::from(prev.is_some())
    }

    /// `TreeStorage::get`（行 58，O）：bucket_mask==0 → None。
    pub fn get(&self, index: u64) -> Option<&TreeStorageEntry> {
        self.map.get(&index)
    }

    /// `TreeStorage::get` 的可变变体（I：move 分支更新记录用）。
    pub fn get_mut(&mut self, index: u64) -> Option<&mut TreeStorageEntry> {
        self.map.get_mut(&index)
    }

    /// `TreeStorage::remove`（行 72，O）。
    pub fn remove(&mut self, index: u64) -> Option<TreeStorageEntry> {
        self.map.remove(&index)
    }

    /// `TreeStorage::iter`（行 54，O：内联于 `wall_eraser_bulldoze`）。
    pub fn iter(&self) -> impl Iterator<Item = (u64, &TreeStorageEntry)> + '_ {
        self.map.iter().map(|(k, v)| (*k, v))
    }

    /// `TreeStorage::next_index`（行 77，O）：`next_index += 1`。
    /// 返回语义（旧值或新值）U；本实现取自增后的新值。
    pub fn next_index(&mut self) -> u64 {
        self.next_index += 1;
        self.next_index
    }

    /// 当前 `next_index`（I：查询只读）。
    pub fn peek_next_index(&self) -> u64 {
        self.next_index
    }
}

/// 森林树组件（O 名；`forest::impl$0::build` 注册，字段 U）。
#[derive(Component, Default)]
pub struct ForestTree(pub Gap);

/// 鸟巢/树栖点组件（O 名；`BirdHaven::new` 内联于 `systems/prefab.rs::create_prefab_rt` :147，
/// `system_critters::wren` 的 `Query<(ref BirdHaven, ref PrefabInstanceData)>` 消费，字段 U）。
#[derive(Component, Default)]
pub struct BirdHaven(pub Gap);

impl BirdHaven {
    /// `BirdHaven::new`（行 147，O 名；字段 U）。
    pub fn new() -> Self {
        BirdHaven(Gap)
    }
}

/// 可交互森林放置资源（O 名；`clear` 内联于 `startup/clearing::{init_clearing,uninit_clearing}` :110）。
#[derive(Resource, Default)]
pub struct InteractableForestPlacements {
    _gap: Gap,
}

impl InteractableForestPlacements {
    /// `InteractableForestPlacements::clear`（行 110，O）。
    pub fn clear(&mut self) {
        self._gap = Gap;
    }
}

/// 可交互森林树记录（O 名；字段 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct InteractableForestTree(pub Gap);

/// 隐藏墙边树的命令（O 名；`system_tree::hide_trees::hide_trees` 消费，payload U）。
#[derive(Event, Clone, Debug, Default)]
pub struct HideTreesNextToWallCmd(pub Gap);

/// 隐藏区域内树的命令（O 名；payload U）。
#[derive(Event, Clone, Debug, Default)]
pub struct HideTreesinAreaCmd(pub Gap);

/// 移动树的命令（O 名；payload 由 tree-tool 反汇编 0x142267430 定：事件槽 0x20，20B：
/// index u64 @+0，position Vec3 @+8）。
#[derive(Event, Clone, Copy, Debug)]
pub struct MoveTreeCmd {
    /// 目标树索引（TreeStorage hash 键）。
    pub index: u64,
    /// 新位置。
    pub position: Vec3,
}

/// 组件注册（骨架 `impl$11::register_required_components` 行 116 / `impl$12` 行 131，O 名）。
pub fn register_required_components() {
    // bevy `register_required_components` 内部为组件依赖注册；具体注册对象 U。
    let _ = core::marker::PhantomData::<(ForestTree, BirdHaven)>;
}

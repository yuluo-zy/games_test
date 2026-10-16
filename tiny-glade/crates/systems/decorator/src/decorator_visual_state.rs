//! `decorator_visual_state.rs` — 装饰视觉状态资源（实体关联状态）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x1408119e0/
//! 0x140811c30/0x140811e80-*.txt`）：
//! - `PersistentVisualEntities::entities` 0x1408119d0（Ghidra NotFound =
//!   小访问器；返回内嵌 map 引用）；
//! - `DecoratorPersistentVisualState::entities` 0x14067dd80（ICF 同址
//!   DecoratorId::new；恒等/访问器）；
//! - `add_decorator_visual_entity` 0x1408119e0（impl$4）/0x140811c30
//!   （impl$5）：HashMap<u32, 0x28B 条目> SIMD 探针，条目 = `{id u32, pad,
//!   Vec<Entity>}`——entry().or_default().push（O 结构）；
//! - `seal_persistent_entities` 0x140811e80（O）：条目标志位 `+0x00` 非 1 →
//!   置 1；已置 1 → fmt panic **"The persistent visuals are already sealed"**
//!   （0x142a8ecd0，file decorator_visual_state.rs 0x142a8ece0）；
//! - `remove_persistent_entities`（行 90 内联）：HashMap::remove；
//! - `DecoratorTransientVisualState::drain`（行 46 内联）：HashMap::drain。
//!
//! core 对接：无 core 依赖（纯 systems 状态资源）；实体↔装饰关联靠
//! DecoratorId 组件（RE-DECORATOR-STORAGE §2 实体机制一致——本资源存实体
//! 列表，不生成实体）。

use bevy_ecs::prelude::*;
use std::collections::HashMap;

/// 持久视觉实体集合（HashMap<u32 DecoratorId, Vec<Entity>>；O 布局
/// 0x28B 条目 = {id u32, pad, Vec{cap,ptr,len}}）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PersistentVisualEntities {
    /// decorator id → 实体列表。
    pub entities: HashMap<u32, Vec<Entity>>,
}

impl PersistentVisualEntities {
    /// `entities` 0x1408119d0（O 访问器；Ghidra NotFound = 小函数）。
    pub fn entities(&self) -> &HashMap<u32, Vec<Entity>> {
        &self.entities
    }
}

/// 持久视觉条目（0x20B 值；布局 O，字段语义 U——[`seal_persistent_entities`]
/// 只操作标志字节）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PersistentVisualEntry {
    /// +0x00：seal 标志（1 = 已封存；seal panic 面）。
    pub sealed: u8,
    /// +0x08/+0x10/+0x18：载荷（U：候选实体/变换句柄）。
    pub payload: [u64; 3],
}

/// 装饰持久视觉状态（HashMap<u32, PersistentVisualEntry>；O 条目 0x28B）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorPersistentVisualState {
    /// decorator id → 持久视觉条目。
    pub entries: HashMap<u32, PersistentVisualEntry>,
}

impl DecoratorPersistentVisualState {
    /// `entities` 0x14067dd80（ICF 同址 DecoratorId::new；恒等/访问器）。
    pub fn entities(&self) -> &HashMap<u32, PersistentVisualEntry> {
        &self.entries
    }

    /// `add_decorator_visual_entity` 0x140811c30（impl$5，O 结构）。
    pub fn add_decorator_visual_entity(&mut self, id: u32, entry: PersistentVisualEntry) {
        self.entries.insert(id, entry);
    }

    /// `remove_persistent_entities`（行 90 内联，O）：移除装饰的持久条目。
    pub fn remove_persistent_entities(&mut self, id: u32) -> Option<PersistentVisualEntry> {
        self.entries.remove(&id)
    }

    /// `seal_persistent_entities` 0x140811e80（O）：封存条目；已封存 panic
    /// "The persistent visuals are already sealed"。
    pub fn seal_persistent_entities(&mut self, id: u32) {
        let entry = self.entries.get_mut(&id);
        let Some(entry) = entry else { return };
        if entry.sealed != 1 {
            entry.sealed = 1;
            return;
        }
        panic!("The persistent visuals are already sealed");
    }
}

/// 装饰瞬时视觉状态（HashMap<u32, Vec<Entity>>；行 46 drain 面 O）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorTransientVisualState {
    /// decorator id → 瞬时实体列表。
    pub entities: HashMap<u32, Vec<Entity>>,
}

impl DecoratorTransientVisualState {
    /// `drain`（行 46 内联，O）：排空全部瞬时实体。
    pub fn drain(&mut self) -> std::collections::hash_map::Drain<'_, u32, Vec<Entity>> {
        self.entities.drain()
    }
}

/// 装饰视觉状态（持久 + 瞬时；`seal` 面 O）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorVisualState {
    /// 持久视觉（seal 语义）。
    pub persistent: DecoratorPersistentVisualState,
    /// 瞬时视觉（drain 语义）。
    pub transient: DecoratorTransientVisualState,
}

impl DecoratorVisualState {
    /// `seal_persistent_entities`（0x140811e80 面）。
    pub fn seal_persistent_entities(&mut self, id: u32) {
        self.persistent.seal_persistent_entities(id);
    }

    /// `remove_persistent_entities`（行 90 内联）。
    pub fn remove_persistent_entities(&mut self, id: u32) -> Option<PersistentVisualEntry> {
        self.persistent.remove_persistent_entities(id)
    }
}

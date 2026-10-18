//! `decorator_visual_state.rs` — 装饰视觉状态资源（实体关联状态）。
//!
//! 机器码证据（2026-10-17 fresh Ghidra shared 反编译，
//! `evidence/restorer-state-decorator/decomp/*.c`）：
//! - `PersistentVisualEntities::entities` 0x1408119d0（PDB void() 占位 + 小访问
//!   器；PDB `enum2$<PersistentVisualEntities>` + 消费方
//!   `from_iter<Entity, FlatMap<IntoIter<&PersistentVisualEntities>,
//!   &[Entity], cart_entities::closure_env$0>>` 证明该类型含
//!   `Vec<Entity>` 字段（entities 访问器返回该切片）；
//! - `impl$4::add_decorator_visual_entity` 0x1408119e0（O 结构）：
//!   `HashMap<u32, PersistentVisualEntities>` 探针 → 命中且 seal 标志
//!   （entry+0x08 首字节，bit0）== 1 → panic_fmt；否则
//!   `entry(id).or_default().push(entity)`（Vec grow_one）；
//!   未命中 → 插入空条目（`{key, pad, value 0x20B}`，0x28B 桶）；
//! - `impl$5::add_decorator_visual_entity` 0x140811c30（O，fresh 机器码复核）：
//!   **瞬态实体表** `HashMap<u32, Vec<Entity>>` 的
//!   `entry(id).or_default().push(entity)`。调用点 0x1412ac662 为
//!   `MOV EDX,ESI; MOV R8,RBX; CALL`（RBX = 本次生成实体）；函数体按 **8 B 步长**
//!   写入 Vec（`*(u64*)(vec + len*8) = in_R8`）。Ghidra 在该函数上的
//!   `Vec<DisplayChildAutoClutterInfo>` 类型串与机器码矛盾：该类型的真实
//!   `grow_one` 步长是 **0x60**（0x14099ca60 `imul 0x60`），不可能用 `*8` 写入。
//!   wave4 reviewer FIX-5：autoclutter 表结论作废。
//! - `DecoratorVisualState::seal_persistent_entities` 0x140811e80（O 结构）：
//!   查 id；条目 seal 标志 `!=1` → 置 1 并 return；`==1` → panic_fmt
//!   **"The persistent visuals are already sealed"**（0x142a8ecd0，
//!   file decorator_visual_state.rs 0x142a8ece0）；
//! - `DecoratorPersistentVisualState::entities` 0x14067dd80（ICF 同址
//!   DecoratorId::new；恒等/访问器）；
//! - `remove_persistent_entities`（行 90 内联）：HashMap::remove；
//! - `DecoratorTransientVisualState::drain`（行 46 内联）：HashMap::drain。
//!
//! 结构与 `clear_out_visual_entities`（0x1407f9260）的瞬态 drain
//! `HashMap<u32, Vec<Entity>>` 对应；`impl$5` 就是这张瞬态实体表的 add
//! （同形状、同 8 B 元素）。autoclutter 子项表（`DisplayChildAutoClutterInfo`，
//! 0x60 B）是另一张表，由 `add_door_autoclutter` 的消费面维护，**不由本函数写入**。

use bevy_ecs::prelude::*;
use std::collections::HashMap;

/// 持久视觉实体集合（**HashMap<u32, Vec<Entity>> 的值**；B 0x20B：
/// `{seal u8 @0, pad, Vec<Entity> @+0x08}`；桶 0x28B = `{key u32, pad,
/// 本值}`）。PDB `enum2$`，消费方展平 `&[Entity]`。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVisualEntities {
    /// +0x00：seal 标志（1 = 已封存；seal panic 面）。
    pub sealed: u8,
    /// +0x08：实体列表（访问器返回切片；消费方 cart_entities 展平）。
    pub entities: Vec<Entity>,
}

impl PersistentVisualEntities {
    /// `entities` 0x1408119d0（O 访问器；返回 Vec 切片引用）。
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}

/// 装饰持久视觉状态（**HashMap<u32, PersistentVisualEntities>**；O 桶 0x28B）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorPersistentVisualState {
    /// decorator id → 持久视觉条目。
    pub entries: HashMap<u32, PersistentVisualEntities>,
}

impl DecoratorPersistentVisualState {
    /// `entities` 0x14067dd80（ICF 同址 DecoratorId::new；恒等/访问器）。
    pub fn entities(&self) -> &HashMap<u32, PersistentVisualEntities> {
        &self.entries
    }

    /// `impl$4::add_decorator_visual_entity` 0x1408119e0（O 结构）：
    /// 命中已封存条目 → panic；否则 push 实体（缺失则建空条目）。
    pub fn add_decorator_visual_entity(&mut self, id: u32, entity: Entity) {
        let entry = self.entries.entry(id).or_default();
        if entry.sealed & 1 != 0 {
            panic!("The persistent visuals are already sealed");
        }
        entry.entities.push(entity);
    }

    /// `remove_persistent_entities`（行 90 内联，O）：移除装饰的持久条目。
    pub fn remove_persistent_entities(&mut self, id: u32) -> Option<PersistentVisualEntities> {
        self.entries.remove(&id)
    }

    /// `seal_persistent_entities` 0x140811e80（O）：封存条目；已封存 panic
    /// "The persistent visuals are already sealed"。
    pub fn seal_persistent_entities(&mut self, id: u32) {
        let Some(entry) = self.entries.get_mut(&id) else {
            return;
        };
        if entry.sealed != 1 {
            entry.sealed = 1;
            return;
        }
        panic!("The persistent visuals are already sealed");
    }
}

/// 装饰瞬时视觉实体状态（行 46 drain 面 O；
/// **drain `HashMap<u32, Vec<Entity>>`**，clear_out_visual_entities 消费；
/// `impl$5::add_decorator_visual_entity`（0x140811c30）向同一张表 push）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorTransientVisualState {
    /// decorator id → 瞬时实体列表。
    pub entities: HashMap<u32, Vec<Entity>>,
}

impl DecoratorTransientVisualState {
    /// `impl$5::add_decorator_visual_entity`（0x140811c30，O；fresh 机器码）：
    /// `HashMap<u32, Vec<Entity>>::entry(id).or_default().push(entity)`。
    ///
    /// 证据：调用点 0x1412ac662 `MOV EDX,ESI; MOV R8,RBX`（id/实体）；
    /// 尾部 `grow_one<usize>` + `*(u64*)(vec + len*8) = in_R8`（8 B 元素）。
    pub fn add_decorator_visual_entity(&mut self, id: u32, entity: Entity) {
        self.entities.entry(id).or_default().push(entity);
    }

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
    pub fn remove_persistent_entities(&mut self, id: u32) -> Option<PersistentVisualEntities> {
        self.persistent.remove_persistent_entities(id)
    }
}

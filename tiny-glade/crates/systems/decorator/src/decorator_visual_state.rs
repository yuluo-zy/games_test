//! `decorator_visual_state.rs` — 装饰视觉状态资源（实体关联状态推进）。
//!
//! 机器码证据（2026-10-17 fresh Ghidra/ReVa 反编译，
//! `evidence/country-core-deep/systems-recovery/groupD/decomp/decomp-0x1408119*.txt` /
//! `decomp-0x140811c30-visual_add_entity_2.txt` / `decomp-0x140811e80-visual_seal.txt`）：
//!
//! - `PersistentVisualEntities::entities` 0x1408119d0（O）：`Vec<Entity>::as_slice`
//!   访问器（PDB 内联链 `Vec::as_slice` → `Deref` → `RawVec::ptr`）；
//! - `impl$4::add_decorator_visual_entity` 0x1408119e0（O）：持久表
//!   `HashMap<u32, PersistentVisualEntities>` 探针 → 命中且 sealed `bit0 == 1`
//!   → panic；否则 `entry.or_default().entities.push(entity)`
//!   （`grow_one<usize>`，8 B 元素）。表项 0x28 B = `{key u32, pad, sealed u8,
//!   pad, Vec<Entity>}`（值 0x20 B）；
//! - `impl$5::add_decorator_visual_entity` 0x140811c30（O，fresh 机器码复核）：
//!   **瞬时表** `HashMap<u32, Vec<Entity>>` 的 `entry.or_default().push(entity)`。
//!   调用点 0x1412ac662 `MOV EDX,ESI; MOV R8,RBX`；函数体按 **8 B 步长**写 Vec
//!   （`*(u64*)(vec + len*8) = in_R8`）。Ghidra 在该函数上给出的
//!   `Vec<DisplayChildAutoClutterInfo>` 类型串与机器码矛盾（该类型 grow_one
//!   步长 0x60，不可能用 *8 写），**不采用**；
//! - `DecoratorVisualState::seal_persistent_entities` 0x140811e80（O）：表空/查无
//!   id → 静默返回；命中且标志 `!= 1` → 写 1 返回；命中且 `== 1` → panic_fmt
//!   "The persistent visuals are already sealed"（0x142a8ecd0，
//!   file 0x142a8ece0）；
//! - `DecoratorPersistentVisualState::entities` 0x14067dd80（ICF 同址
//!   DecoratorId::new；访问器）；
//! - `DecoratorTransientVisualState::drain` 行 46 inline（O）：`HashMap::drain`
//!   （宿主 `clear_out_visual_entities` 0x1407f9260 逐实体 despawn）；
//! - `DecoratorVisualState::remove_persistent_entities` 行 90 inline（O）：
//!   `HashMap::remove`；
//! - `impl$11::clone` / `impl$13::eq` 行 114（O）：资源派生 Clone/PartialEq。
//!
//! # 状态推进语义
//!
//! 每帧（宿主调度，见 `lib.rs`）：
//! 1. `clear_out_visual_entities` 0x1407f9260 对 [`DecoratorTransientVisualState`]
//!    `drain` 并 despawn 全部瞬时实体 → 瞬时表回到空；
//! 2. 视觉生成族按装饰 id 调 [`DecoratorPersistentVisualState::add_decorator_visual_entity`]
//!    累积本帧持久实体；
//! 3. 生成结束后按 id 调 [`DecoratorPersistentVisualState::seal_persistent_entities`]
//!    封存（同一 id 再次 add 或重复 seal → panic）；下帧如需重建，先
//!    `remove_persistent_entities` 再 add。
//!
//! **持久表不在帧清系统里清除**（reviewer-state 复核 2026-10-17）；
//! 清除只发生在 `remove_persistent_entities` / 显式重建。
//!
//! # U 登记
//!
//! - `ClothInfo` 的 PDB 字段声明仍未知；2026-10-20 的有界 B 原证已从两窗口
//!   producer、180 B Option<Box<ClothInfo>> equality、434 B typed iterator fold
//!   和全局 owner 生命周期恢复下方语义字段。它不是 WallFlagCloth 或 Nani
//!   ClothlineCloth。原生分配 80 B/align16 不作为宿主 raw ABI 承诺。
//! - 同一资源首 Vec 由 fullvisual 0x140827910 每帧清长度后，从所有当前蓝图
//!   typed Box 按值克隆（发生在 differential gate 前）；clothline 原系统随后
//!   读取它。当前窗口 producer/clothline/卸载宿主仍需各自接通，字段本身不代表
//!   已有非空真实场景输入。证据见本任务 cloth-contract-v1/DESIGN_MANIFEST_V1.json。
//! - 持久表 sealed 字段的精确宽度：seal 写入为 8 B、add 读 bit0、seal 判 `!=1`；
//!   本实现以 `u8` 承载已观察语义（值域 {0,1}），宽度差异不影响判别。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::WallId;
use country_core::utils::components::transform::Transform;
use std::collections::HashMap;

/// 窗口蓝图提供的布绳候选值；由同一全局 VisualState 首 Vec 持有。
///
/// B 0x1407ee313 / 0x1407f7e93 创建 `Box<ClothInfo>`；有效字段为 Transform
/// （rotation@0, translation@0x10, scale@0x1c）、WallId@0x30、两个 float
/// @0x38/0x3c、DecoratorId@0x40。原分配 80 B 对齐 16；不复制未知 padding。
/// `dimensions` 保持原 pair 顺序，未推定原字段名或 width/height 次序。
///
/// B equality 0x140825090 对所有 12 个 float 使用 ordered equality：NaN 不等，
/// 正负零相等；没有 Some Box 指针身份快捷路径。值克隆没有嵌套所有权。
#[derive(Clone, Debug, PartialEq)]
pub struct ClothInfo {
    pub transform: Transform,
    pub wall: WallId,
    pub dimensions: [f32; 2],
    pub decorator_id: DecoratorId,
}

/// 持久视觉实体集合（B 表值 0x20 B：`{sealed u8 @0, pad, Vec<Entity> @+0x08}`；
/// 桶 0x28 B = `{key u32, pad, 本值}`）。PDB `enum2$`，消费方展平 `&[Entity]`。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVisualEntities {
    /// +0x00：seal 标志（bit0 = 已封存；add/seal 的 panic 门）。
    pub sealed: u8,
    /// +0x08：实体列表（`entities` 访问器返回切片；消费方 cart_entities 展平）。
    pub entities: Vec<Entity>,
}

impl PersistentVisualEntities {
    /// `entities` 0x1408119d0（O 访问器；`Vec::as_slice`）。
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    /// seal 标志 bit0（O：`impl$4` 的 `byte & 1` 门）。
    #[inline]
    pub fn is_sealed(&self) -> bool {
        self.sealed & 1 != 0
    }

    /// 实体数（`Vec::len` 访问面）。
    #[inline]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// 实体列表为空。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

/// 装饰持久视觉状态（**HashMap<u32, PersistentVisualEntities>**；O 桶 0x28 B）。
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct DecoratorPersistentVisualState {
    /// decorator id → 持久视觉条目。
    pub entries: HashMap<u32, PersistentVisualEntities>,
}

impl DecoratorPersistentVisualState {
    /// `entities` 0x14067dd80（ICF 同址 DecoratorId::new；恒等/访问器）。
    pub fn entities(&self) -> &HashMap<u32, PersistentVisualEntities> {
        &self.entries
    }

    /// 单 id 持久条目（访问面）。
    #[inline]
    pub fn get(&self, id: u32) -> Option<&PersistentVisualEntities> {
        self.entries.get(&id)
    }

    /// 单 id 实体切片（`entities` 访问器 + `PersistentVisualEntities::entities`）。
    pub fn entities_of(&self, id: u32) -> Option<&[Entity]> {
        self.entries.get(&id).map(PersistentVisualEntities::entities)
    }

    /// 已封存 id 集合（判定面；按 `is_sealed` 过滤）。
    pub fn sealed_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries
            .iter()
            .filter(|(_id, entry)| entry.is_sealed())
            .map(|(id, _entry)| *id)
    }

    /// `impl$4::add_decorator_visual_entity` 0x1408119e0（O 结构）：
    /// 命中已封存条目 → panic；否则 push 实体（缺失则建空条目）。
    ///
    /// O：`entry.or_default()` + `grow_one<usize>` + `*(u64*)(ptr+len*8)=entity`。
    pub fn add_decorator_visual_entity(&mut self, id: u32, entity: Entity) {
        let entry = self.entries.entry(id).or_default();
        // O：`*(byte *)(entry + 0) & 1 != 0` → panic（0x142a8ecd0）。
        if entry.is_sealed() {
            panic!("The persistent visuals are already sealed");
        }
        entry.entities.push(entity);
    }

    /// `remove_persistent_entities`（行 90 内联，O）：移除装饰的持久条目。
    ///
    /// 返回被移除的条目（含实体表），供调用方 despawn/转移。
    pub fn remove_persistent_entities(&mut self, id: u32) -> Option<PersistentVisualEntities> {
        self.entries.remove(&id)
    }

    /// `seal_persistent_entities` 0x140811e80（O）：封存条目；已封存 panic
    /// "The persistent visuals are already sealed"。
    ///
    /// O：表空或 id 不存在时静默返回（`param_1+0x30 == 0` / 探针耗尽）。
    pub fn seal_persistent_entities(&mut self, id: u32) {
        let Some(entry) = self.entries.get_mut(&id) else {
            return;
        };
        // O：`*(int *)(entry+flags) != 1` → 写 1 返回；`== 1` → panic。
        if entry.sealed != 1 {
            entry.sealed = 1;
            return;
        }
        panic!("The persistent visuals are already sealed");
    }
}

/// 装饰瞬时视觉实体状态（行 46 drain 面 O；
/// **drain `HashMap<u32, Vec<Entity>>`**，`clear_out_visual_entities` 消费；
/// `impl$5::add_decorator_visual_entity`（0x140811c30）向同一张表 push）。
#[derive(Resource, Clone, Debug, Default, PartialEq)]
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

    /// 单 id 实体切片（访问面）。
    pub fn entities_of(&self, id: u32) -> Option<&[Entity]> {
        self.entities.get(&id).map(Vec::as_slice)
    }

    /// `drain`（行 46 内联，O）：排空全部瞬时实体。
    pub fn drain(&mut self) -> std::collections::hash_map::Drain<'_, u32, Vec<Entity>> {
        self.entities.drain()
    }
}

/// 同一装饰视觉状态 owner：布绳候选首 Vec、持久表和瞬时表。
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct DecoratorVisualState {
    /// B 首 Vec<ClothInfo>：plugin 初始为空；fullvisual 每帧 clear 后重填。
    /// clear 不触及另外两张实体表；glade unload 原路径替换整个 Default owner。
    pub cloth_infos: Vec<ClothInfo>,
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

    /// 持久实体访问面（`entities` 0x14067dd80 / `PersistentVisualEntities::entities`）。
    pub fn persistent_entities_of(&self, id: u32) -> Option<&[Entity]> {
        self.persistent.entities_of(id)
    }

    /// 瞬态实体访问面（`impl$5` 表）。
    pub fn transient_entities_of(&self, id: u32) -> Option<&[Entity]> {
        self.transient.entities_of(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(raw: u32) -> Entity {
        Entity::from_raw(raw)
    }

    fn cloth_info(lanes: [f32; 12]) -> ClothInfo {
        ClothInfo {
            transform: Transform {
                rotation: glam::Quat::from_array([lanes[0], lanes[1], lanes[2], lanes[3]]),
                translation: glam::Vec3::new(lanes[4], lanes[5], lanes[6]),
                scale: glam::Vec3::new(lanes[7], lanes[8], lanes[9]),
            },
            wall: WallId(0x1234_5678_90ab_cdef),
            dimensions: [lanes[10], lanes[11]],
            decorator_id: DecoratorId(37),
        }
    }

    #[test]
    fn cloth_value_clone_and_all_identity_fields_participate_in_eq() {
        let value = cloth_info([0.1, 0.2, 0.3, 0.4, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(value, value.clone());
        let mut other = value.clone();
        other.wall = WallId(value.wall.0 + 1);
        assert_ne!(value, other);
        let mut other = value.clone();
        other.decorator_id = DecoratorId(value.decorator_id.0 + 1);
        assert_ne!(value, other);
        for lane in 0..2 {
            let mut other = value.clone();
            other.dimensions[lane] += 1.0;
            assert_ne!(value, other);
        }
    }

    #[test]
    fn cloth_eq_rejects_nan_in_every_native_compared_float_lane() {
        for lane in 0..12 {
            let mut lanes = [1.0; 12];
            lanes[lane] = f32::NAN;
            let value = cloth_info(lanes);
            // The original Some/Some helper compares values even for one Box.
            assert_ne!(value, value.clone(), "native float lane {lane}");
        }
    }

    #[test]
    fn cloth_eq_treats_positive_and_negative_zero_as_equal() {
        let positive = cloth_info([0.0; 12]);
        for lane in 0..12 {
            let mut lanes = [0.0; 12];
            lanes[lane] = -0.0;
            assert_eq!(positive, cloth_info(lanes), "native float lane {lane}");
        }
    }

    #[test]
    fn same_global_owner_keeps_entity_maps_and_independent_cloth_values() {
        let mut world = World::new();
        world.init_resource::<DecoratorVisualState>();
        let mut source = Box::new(cloth_info([1.0; 12]));
        {
            let mut visual = world.resource_mut::<DecoratorVisualState>();
            assert!(visual.cloth_infos.is_empty());
            assert!(visual.persistent.entries.is_empty());
            assert!(visual.transient.entities.is_empty());
            visual.persistent.add_decorator_visual_entity(37, entity(10));
            visual.transient.add_decorator_visual_entity(37, entity(20));
            visual.cloth_infos.push((*source).clone());
            source.dimensions[0] = 9.0;
            assert_eq!(visual.cloth_infos[0].dimensions, [1.0, 1.0]);
            // Shared-state unit boundary only; actual fullvisual owns its clear/copy call.
            visual.cloth_infos.clear();
            visual.cloth_infos.push((*source).clone());
        }
        let visual = world.resource::<DecoratorVisualState>();
        assert_eq!(visual.cloth_infos.len(), 1);
        assert_eq!(visual.cloth_infos[0].dimensions, [9.0, 1.0]);
        assert_eq!(visual.persistent_entities_of(37), Some([entity(10)].as_slice()));
        assert_eq!(visual.transient_entities_of(37), Some([entity(20)].as_slice()));
    }

    #[test]
    fn persistent_add_before_seal_then_push() {
        let mut state = DecoratorPersistentVisualState::default();
        state.add_decorator_visual_entity(1, entity(10));
        state.add_decorator_visual_entity(1, entity(11));
        state.add_decorator_visual_entity(2, entity(12));
        assert_eq!(state.entities_of(1), Some([entity(10), entity(11)].as_slice()));
        assert_eq!(state.entities_of(2), Some([entity(12)].as_slice()));
        assert_eq!(state.entries[&1].len(), 2);
        assert!(!state.entries[&1].is_sealed());
    }

    #[test]
    #[should_panic(expected = "The persistent visuals are already sealed")]
    fn persistent_add_after_seal_panics() {
        let mut state = DecoratorPersistentVisualState::default();
        state.add_decorator_visual_entity(1, entity(10));
        state.seal_persistent_entities(1);
        state.add_decorator_visual_entity(1, entity(11));
    }

    #[test]
    #[should_panic(expected = "The persistent visuals are already sealed")]
    fn double_seal_panics() {
        let mut state = DecoratorPersistentVisualState::default();
        state.add_decorator_visual_entity(1, entity(10));
        state.seal_persistent_entities(1);
        state.seal_persistent_entities(1);
    }

    #[test]
    fn seal_unknown_id_is_silent_noop() {
        let mut state = DecoratorPersistentVisualState::default();
        state.seal_persistent_entities(42);
        assert!(state.entries.is_empty());
    }

    #[test]
    fn remove_allows_rebuild_and_resets_seal() {
        let mut state = DecoratorPersistentVisualState::default();
        state.add_decorator_visual_entity(1, entity(10));
        state.seal_persistent_entities(1);
        let removed = state.remove_persistent_entities(1).expect("entry");
        assert_eq!(removed.entities, vec![entity(10)]);
        assert_eq!(removed.sealed, 1);
        // 重建后是未封存条目，可再次 add/seal。
        state.add_decorator_visual_entity(1, entity(20));
        state.seal_persistent_entities(1);
        assert_eq!(state.entities_of(1), Some([entity(20)].as_slice()));
    }

    #[test]
    fn sealed_ids_reports_only_sealed_entries() {
        let mut state = DecoratorPersistentVisualState::default();
        state.add_decorator_visual_entity(1, entity(10));
        state.add_decorator_visual_entity(2, entity(11));
        state.seal_persistent_entities(2);
        let mut ids: Vec<u32> = state.sealed_ids().collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![2]);
    }

    #[test]
    fn transient_add_and_drain_clears_only_transient() {
        let mut visual = DecoratorVisualState::default();
        visual
            .persistent
            .add_decorator_visual_entity(1, entity(10));
        visual.transient.add_decorator_visual_entity(1, entity(20));
        visual.transient.add_decorator_visual_entity(1, entity(21));
        visual.transient.add_decorator_visual_entity(2, entity(22));
        assert_eq!(visual.transient_entities_of(1).map(|s| s.len()), Some(2));
        let drained: Vec<(u32, Vec<Entity>)> = visual.transient.drain().collect();
        assert_eq!(drained.len(), 2);
        assert!(visual.transient.entities.is_empty());
        // 持久表不受 drain 影响。
        assert_eq!(
            visual.persistent_entities_of(1),
            Some([entity(10)].as_slice())
        );
    }

    #[test]
    fn drain_empty_is_noop() {
        let mut state = DecoratorTransientVisualState::default();
        assert_eq!(state.drain().count(), 0);
    }

    #[test]
    fn visual_state_clone_and_eq_match_derived_contract() {
        let mut a = DecoratorVisualState::default();
        a.persistent.add_decorator_visual_entity(1, entity(10));
        a.transient.add_decorator_visual_entity(1, entity(20));
        let b = a.clone();
        assert_eq!(a, b);
        let mut c = a.clone();
        c.transient.entities.clear();
        assert_ne!(a, c);
    }
}

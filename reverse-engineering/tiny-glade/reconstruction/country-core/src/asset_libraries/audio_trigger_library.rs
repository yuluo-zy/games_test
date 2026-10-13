//! `asset_libraries/audio_trigger_library.rs` —— 音频触发盒库（A3 逐函数核对版）。
//!
//! 原版只有 3 个过程（O）：
//!  * `AudioTriggerCollection::boxes` 0x14157ce40（67 B）
//!  * `AudioTriggerLibrary::get_triggers` 0x14157ce90（42 B）
//!  * `impl$3::from` 0x140833250（3 B，`mov eax, ecx; ret`）= `AudioTriggerHandle` 与 u32 的恒等转换；PDB 把它与
//!    `DecoratorRank::new` 折叠（ICF），不需要单独的 Rust 函数。
//!
//! 数据结构（O）：
//!  * `AudioTriggerCollection` = **`slotmap::SlotMap<AudioTriggerHandle, Aabb3>`**，0x20 B：`slots: Vec<Slot<Aabb3>>`
//!    @0（cap/ptr/len）、`free_head: u32` @0x18、`num_elems: u32` @0x1c。槽 = 28 B（`Aabb3` 24 B + `version: u32` @0x18，
//!    version 为奇数 = 占用；槽 0 是 slotmap 的哨兵）。
//!    证据：`boxes` 返回的迭代器 `{ptr + 0x1c(非空时跳过哨兵槽), end = ptr + len*0x1c, 起始下标 = (len != 0), num_left =
//!    [self+0x1c]}`；唯一调用方 `trigger_audio_system`（PDB 名被 ICF 折叠为 `handplaced_tree_animation`，调用点
//!    0x1422798de）逐槽读 `[slot+0x18] & 1` 判占用、`DecoratorRank::new(index)` 取键、再把 `[slot+0..0x18]` 当
//!    min/max 喂 `Aabb3::intersect_segment`。
//!  * `AudioTriggerLibrary` = `[AudioTriggerCollection; AudioTriggerKind::COUNT]`，**0x20 B**（O：
//!    `insert_resource<AudioTriggerLibrary>` 的 `call_once` 搬 0x20 B；dealloc 大小 = cap × 28、对齐 4）。
//!    `get_triggers(kind)` 只在 `kind == 0` 时返回库首地址，故 `AudioTriggerKind::COUNT == 1`（O：断言被编译成
//!    `test rdx, rdx; jne panic`，即只有 kind = 0 合法）。
//!  * `get_triggers` 的 panic 文案（O）：`assertion failed: kind != AudioTriggerKind::COUNT`（源码行 85 列 9）。
//!
//! 旧版的重大错误：盒是 28 B 带 `flags` 字段的自造结构（那 28 B 其实是 slotmap 槽，version 在槽里）；
//! `AudioTriggerCollectionHandle` 与“集合 → 盒”的二级 slotmap 纯属臆造（库就是一个集合）；`get_triggers` 按键取
//! `Option`（原版是对 kind 的断言 + 直接返回引用）；`boxes()` 返回 values 迭代器（原版返回带键的 `Iter`）。
//!
//! 仍为 I：`AudioTriggerKind` 唯一合法变体的名字（O 只知道它是 0）；`add_trigger / update_trigger_shape /
//! remove_trigger / clear` 在原版没有独立符号（内联进 `spawn_prefabs` 行 55、`update_prefabs` 行 63、
//! `despawn_prefab_contents` 行 70、`uninit_clearing`），这里按 slotmap 的 `insert / get_mut / remove / clear` 写出。

use bevy_ecs::resource::Resource;
use slotmap::{new_key_type, SlotMap};

use crate::utils_contract::Aabb3;

new_key_type! {
    /// 单个触发盒的键（O：PDB 名 `AudioTriggerHandle`，slotmap 键）。
    pub struct AudioTriggerHandle;
}

/// 触发盒类别（O：只有 `kind != COUNT` 的断言；`COUNT == 1`，I：唯一合法变体名）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum AudioTriggerKind {
    Primary = 0,
    COUNT = 1,
}

/// 一类触发盒（O：`SlotMap<AudioTriggerHandle, Aabb3>`，0x20 B）。
#[derive(Debug, Default)]
pub struct AudioTriggerCollection {
    pub triggers: SlotMap<AudioTriggerHandle, Aabb3>,
}

impl AudioTriggerCollection {
    /// O 0x14157ce40：`self.triggers.iter()`（带键、跳过哨兵槽、只产出占用槽）。
    pub fn boxes(&self) -> slotmap::basic::Iter<'_, AudioTriggerHandle, Aabb3> {
        self.triggers.iter()
    }
}

/// 音频触发盒库（O：0x20 B 资源，`startup::load_audio` 里 `insert_resource`）。
#[derive(Resource, Debug, Default)]
pub struct AudioTriggerLibrary {
    collections: [AudioTriggerCollection; AudioTriggerKind::COUNT as usize],
}

impl AudioTriggerLibrary {
    /// O 0x14157ce90：`assert!(kind != AudioTriggerKind::COUNT)`，返回对应集合（COUNT == 1 ⇒ 恒为库首）。
    pub fn get_triggers(&self, kind: AudioTriggerKind) -> &AudioTriggerCollection {
        assert!(kind != AudioTriggerKind::COUNT);
        &self.collections[kind as usize]
    }

    /// I（内联于 `spawn_prefabs` 行 55）：登记一个盒，返回键。
    pub fn add_trigger(&mut self, kind: AudioTriggerKind, shape: Aabb3) -> AudioTriggerHandle {
        assert!(kind != AudioTriggerKind::COUNT);
        self.collections[kind as usize].triggers.insert(shape)
    }

    /// I（内联于 `update_prefabs` 行 63）：更新盒形状；键失效返回 false。
    pub fn update_trigger_shape(&mut self, kind: AudioTriggerKind, handle: AudioTriggerHandle, shape: Aabb3) -> bool {
        assert!(kind != AudioTriggerKind::COUNT);
        match self.collections[kind as usize].triggers.get_mut(handle) {
            Some(slot) => {
                *slot = shape;
                true
            }
            None => false,
        }
    }

    /// I（内联于 `despawn_prefab_contents` 行 70）：移除盒。
    pub fn remove_trigger(&mut self, kind: AudioTriggerKind, handle: AudioTriggerHandle) -> Option<Aabb3> {
        assert!(kind != AudioTriggerKind::COUNT);
        self.collections[kind as usize].triggers.remove(handle)
    }

    /// I（内联于 `uninit_clearing` 行 23/76）：清空全部盒。
    pub fn clear(&mut self) {
        for c in &mut self.collections {
            c.triggers.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn aabb(i: f32) -> Aabb3 {
        Aabb3 { min: Vec3::splat(i), max: Vec3::splat(i + 1.0) }
    }

    #[test]
    fn layout_matches_slotmap_resource() {
        // O：insert_resource<AudioTriggerLibrary> 搬 0x20 B（真 slotmap 的 SlotMap = Vec 24 + 2×u32）；
        // 槽 = 28 B（Aabb3 24 + version 4）。测试台的 slotmap 占位多一个空闲表，故不断言库整体大小。
        assert_eq!(core::mem::size_of::<AudioTriggerLibrary>(), core::mem::size_of::<AudioTriggerCollection>());
        assert_eq!(core::mem::size_of::<Aabb3>(), 24);
        assert_eq!(AudioTriggerKind::COUNT as usize, 1);
    }

    #[test]
    fn get_triggers_for_valid_kind_returns_the_only_collection() {
        let mut lib = AudioTriggerLibrary::default();
        let h = lib.add_trigger(AudioTriggerKind::Primary, aabb(1.0));
        let c = lib.get_triggers(AudioTriggerKind::Primary);
        assert!(core::ptr::eq(c, &lib.collections[0]));
        assert_eq!(c.boxes().count(), 1);
        assert_eq!(c.boxes().next().map(|(k, _)| k), Some(h));
    }

    #[test]
    #[should_panic(expected = "assertion failed: kind != AudioTriggerKind::COUNT")]
    fn get_triggers_rejects_count() {
        AudioTriggerLibrary::default().get_triggers(AudioTriggerKind::COUNT);
    }

    #[test]
    fn boxes_is_keyed_and_skips_vacant_slots() {
        let mut lib = AudioTriggerLibrary::default();
        let a = lib.add_trigger(AudioTriggerKind::Primary, aabb(1.0));
        let b = lib.add_trigger(AudioTriggerKind::Primary, aabb(2.0));
        let c = lib.add_trigger(AudioTriggerKind::Primary, aabb(3.0));
        assert_eq!(lib.remove_trigger(AudioTriggerKind::Primary, b), Some(aabb(2.0)));
        let got: Vec<_> = lib.get_triggers(AudioTriggerKind::Primary).boxes().map(|(k, v)| (k, *v)).collect();
        assert_eq!(got, vec![(a, aabb(1.0)), (c, aabb(3.0))]);
    }

    #[test]
    fn empty_library_has_no_boxes() {
        assert_eq!(AudioTriggerLibrary::default().get_triggers(AudioTriggerKind::Primary).boxes().count(), 0);
    }

    #[test]
    fn update_shape_replaces_in_place_and_rejects_stale_handle() {
        let mut lib = AudioTriggerLibrary::default();
        let h = lib.add_trigger(AudioTriggerKind::Primary, aabb(1.0));
        assert!(lib.update_trigger_shape(AudioTriggerKind::Primary, h, aabb(5.0)));
        assert_eq!(*lib.get_triggers(AudioTriggerKind::Primary).boxes().next().unwrap().1, aabb(5.0));
        lib.remove_trigger(AudioTriggerKind::Primary, h);
        assert!(!lib.update_trigger_shape(AudioTriggerKind::Primary, h, aabb(6.0)));
        assert_eq!(lib.remove_trigger(AudioTriggerKind::Primary, h), None);
    }

    #[test]
    fn clear_removes_everything() {
        let mut lib = AudioTriggerLibrary::default();
        lib.add_trigger(AudioTriggerKind::Primary, aabb(1.0));
        lib.add_trigger(AudioTriggerKind::Primary, aabb(2.0));
        lib.clear();
        assert_eq!(lib.get_triggers(AudioTriggerKind::Primary).boxes().count(), 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: kind != AudioTriggerKind::COUNT")]
    fn add_rejects_count() {
        AudioTriggerLibrary::default().add_trigger(AudioTriggerKind::COUNT, aabb(0.0));
    }
}

//! 音频触发盒注册表。带版本的 slotmap 键管理 AABB 生命周期。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

use bevy_ecs::resource::Resource;
use slotmap::{SlotMap, new_key_type};

use crate::utils_contract::Aabb3;

new_key_type! {
    /// 带版本的触发盒身份，删除后旧键失效。
    pub struct AudioTriggerHandle;
}

/// 触发盒类别，COUNT 为边界哨兵。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum AudioTriggerKind {
    Primary = 0,
    COUNT = 1,
}

/// 一类触发盒的拥有集合。
#[derive(Debug, Default)]
pub struct AudioTriggerCollection {
    pub triggers: SlotMap<AudioTriggerHandle, Aabb3>,
}

impl AudioTriggerCollection {
    /// 遍历有效触发盒及其带版本键。
    pub fn boxes(&self) -> slotmap::basic::Iter<'_, AudioTriggerHandle, Aabb3> {
        self.triggers.iter()
    }
}

/// 按业务种类管理触发盒，不负责播放声音。
#[derive(Resource, Debug, Default)]
pub struct AudioTriggerLibrary {
    collections: [AudioTriggerCollection; AudioTriggerKind::COUNT as usize],
}

impl AudioTriggerLibrary {
    /// 读取指定种类的触发盒集合。
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
    pub fn update_trigger_shape(
        &mut self,
        kind: AudioTriggerKind,
        handle: AudioTriggerHandle,
        shape: Aabb3,
    ) -> bool {
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
    pub fn remove_trigger(
        &mut self,
        kind: AudioTriggerKind,
        handle: AudioTriggerHandle,
    ) -> Option<Aabb3> {
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
        Aabb3 {
            min: Vec3::splat(i),
            max: Vec3::splat(i + 1.0),
        }
    }

    #[test]
    fn layout_matches_slotmap_resource() {
        // O：insert_resource<AudioTriggerLibrary> 搬 0x20 B（真 slotmap 的 SlotMap = Vec 24 + 2×u32）；
        // 槽 = 28 B（Aabb3 24 + version 4）。测试台的 slotmap 占位多一个空闲表，故不断言库整体大小。
        assert_eq!(
            core::mem::size_of::<AudioTriggerLibrary>(),
            core::mem::size_of::<AudioTriggerCollection>()
        );
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
        assert_eq!(
            lib.remove_trigger(AudioTriggerKind::Primary, b),
            Some(aabb(2.0))
        );
        let got: Vec<_> = lib
            .get_triggers(AudioTriggerKind::Primary)
            .boxes()
            .map(|(k, v)| (k, *v))
            .collect();
        assert_eq!(got, vec![(a, aabb(1.0)), (c, aabb(3.0))]);
    }

    #[test]
    fn empty_library_has_no_boxes() {
        assert_eq!(
            AudioTriggerLibrary::default()
                .get_triggers(AudioTriggerKind::Primary)
                .boxes()
                .count(),
            0
        );
    }

    #[test]
    fn update_shape_replaces_in_place_and_rejects_stale_handle() {
        let mut lib = AudioTriggerLibrary::default();
        let h = lib.add_trigger(AudioTriggerKind::Primary, aabb(1.0));
        assert!(lib.update_trigger_shape(AudioTriggerKind::Primary, h, aabb(5.0)));
        assert_eq!(
            *lib.get_triggers(AudioTriggerKind::Primary)
                .boxes()
                .next()
                .unwrap()
                .1,
            aabb(5.0)
        );
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
        assert_eq!(
            lib.get_triggers(AudioTriggerKind::Primary).boxes().count(),
            0
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: kind != AudioTriggerKind::COUNT")]
    fn add_rejects_count() {
        AudioTriggerLibrary::default().add_trigger(AudioTriggerKind::COUNT, aabb(0.0));
    }
}

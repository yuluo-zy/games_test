//! asset_libraries/audio_trigger_library.rs — 音频触发器空间库（trigger→sound 映射）。
//!
//! PDB 命名实体（O）：`AudioTriggerLibrary`、`AudioTriggerCollection`、
//! `AudioTriggerHandle`、`AudioTriggerCollectionHandle`。
//!
//! 反汇编证据（O）：
//! - `AudioTriggerCollection::boxes`（0x14157ce40）：返回 28 字节盒条目切片
//!   `{ptr=base+len*0x1c, end, is_some, stride=0x1c}`；盒条目 `@+0x18` bit0 = enabled。
//! - `AudioTriggerLibrary::get_triggers`（0x14157ce90）：按键取集合（空 panic）。
//! - 内联宿主：`prefab::spawn_prefabs` 行 55（add_trigger）、`update_prefabs` 行 63
//!   （update_trigger_shape）、`despawn_prefab_contents` 行 70（remove_trigger）、
//!   `hot_reload_prefabs` 行 55、`clearing::uninit_clearing` 行 23/76（clear）。
//! - 消费方：`systems/audio/trigger_audio::trigger_audio_system`（`boxes()` 行 27）。
//! - 装配：`startup::load_audio` 行 15/42（`insert_resource<AudioTriggerLibrary>`）。
//!
//! 语义（I）：关卡 prefab 带音频触发器盒；`trigger_audio_system` 用相机段与盒相交判定
//! 响度。见 contracts/audio-trigger.md。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use slotmap::{new_key_type, SlotMap};

use crate::typed_gap::Gap;

new_key_type! {
    /// 单个触发盒的键（O：slotmap）。
    pub struct AudioTriggerHandle;
    /// 一个实体触发盒集合的键（O：slotmap）。
    pub struct AudioTriggerCollectionHandle;
}

/// 一个音频触发盒（28 字节，I：aabb min/max + flags；`@+0x18` bit0 = enabled，O）。
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioTriggerBox {
    /// aabb min（I）。
    pub min: glam::Vec3,
    /// aabb max（I）。
    pub max: glam::Vec3,
    /// flags u32（`@+0x18` bit0 = enabled，O）。
    pub flags: u32,
}

/// 单个实体的触发盒集合（O 名；内部容器布局 U）。
#[derive(Debug, Default)]
pub struct AudioTriggerCollection {
    /// 盒表（I：`HashMap<AudioTriggerHandle, AudioTriggerBox>`，slotmap 键）。
    pub boxes: slotmap::SlotMap<AudioTriggerHandle, AudioTriggerBox>,
}

impl AudioTriggerCollection {
    /// `AudioTriggerCollection::boxes`（0x14157ce40，O）：28 字节盒条目切片迭代。
    pub fn boxes(&self) -> impl Iterator<Item = &AudioTriggerBox> {
        self.boxes.values()
    }

    /// `AudioTriggerCollection::clear`（O）。
    pub fn clear(&mut self) {
        self.boxes.clear();
    }
}

/// 音频触发器库（Resource，O；`load_audio` 装配）。
#[derive(Resource, Debug, Default)]
pub struct AudioTriggerLibrary {
    /// 实体→集合表（I：slotmap 键索引）。
    pub collections: SlotMap<AudioTriggerCollectionHandle, AudioTriggerCollection>,
    /// 其余（U）。
    pub _gap: Gap,
}

impl AudioTriggerLibrary {
    /// `add_trigger`（行 55，O；`spawn_prefabs` 内联）。
    pub fn add_trigger(
        &mut self,
        _collection: AudioTriggerCollectionHandle,
        _handle: AudioTriggerHandle,
        _shape: &AudioTriggerBox,
    ) {
        // 写入盒表（语义 U）。
    }

    /// `update_trigger_shape`（行 63，O；`update_prefabs` 内联）。
    pub fn update_trigger_shape(
        &mut self,
        _collection: AudioTriggerCollectionHandle,
        _handle: AudioTriggerHandle,
        _shape: &AudioTriggerBox,
    ) {
    }

    /// `remove_trigger`（行 70，O；`despawn_prefab_contents` 内联）。
    pub fn remove_trigger(
        &mut self,
        _collection: AudioTriggerCollectionHandle,
        _handle: AudioTriggerHandle,
    ) {
    }

    /// `get_triggers`（0x14157ce90，O）：按键取集合。
    pub fn get_triggers(
        &self,
        collection: AudioTriggerCollectionHandle,
    ) -> Option<&AudioTriggerCollection> {
        self.collections.get(collection)
    }

    /// `clear`（O；`uninit_clearing` 行 23/76 内联）。
    pub fn clear(&mut self) {
        self.collections.clear();
    }
}

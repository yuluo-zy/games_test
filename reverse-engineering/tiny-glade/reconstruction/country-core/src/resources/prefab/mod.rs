//! `resources::prefab` — prefab 库/目录状态（B 版本骨架对齐，WP13）。
//!
//! 范围（合同见 `contracts/prefab.md`）：
//!   - `PrefabLibrary` = `HashMap<PrefabName, Prefab>`（hashbrown RandomState，O）
//!   - `add` 0x1408e2540 / `get<&PrefabName>` 0x141552d70 / `has<&PrefabNameRef>`
//!     0x141552ff0 / `iter`
//!   - `Prefab` / `PrefabPart` / `PrefabInstance` / `AudioTriggerShape`
//!   - `PrefabsModifiedCmd` 事件、`PrefabWatchPlugin`
//!
//! 未证字段一律 typed gap。

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use hashbrown::HashMap;

use crate::typed_gap::Gap;

/// 外部 `asset::prefab::{PrefabName, PrefabNameRef}`：只有一份定义，在
/// `resources::render::mesh_atlas_library`（`PrefabName(Arc<str>)`、`PrefabNameRef(str)`）。
/// O：旧占位的 “allocation 含 16B header、`as_name_ref` = 首指针 +16” 正是 `Arc<str>` 的
/// (strong, weak) 两个 usize 头，`as_name_ref` B=0x141556130 即 `PrefabName -> &PrefabNameRef`，
/// 因此这里统一为 re-export，不再保留 `&[u8]` 借用视图与 `OwnedPrefabName` 副本。
pub use crate::resources::render::mesh_atlas_library::{PrefabName, PrefabNameRef};

/// `HashMap<PrefabName, _>::get(&PrefabNameRef)` 的借用查找（O：`has<&PrefabNameRef>` 0x141552ff0）。
/// 两者按 `str` 内容哈希/比较，故 `Equivalent` 成立。
impl hashbrown::Equivalent<PrefabName> for PrefabNameRef {
    fn equivalent(&self, key: &PrefabName) -> bool {
        self.as_str() == key.as_name_ref().as_str()
    }
}

/// prefab 库值（字段布局 U）。
#[derive(Clone, Debug, Default)]
pub struct Prefab {
    pub parts: Vec<PrefabPart>,
}

/// prefab 组成部件（drop_in_place 出现；字段 U）。
#[derive(Clone, Debug, Default)]
pub struct PrefabPart(pub Gap);

/// prefab 实例组件（ECS Component；字段 U）。
#[derive(Component, Clone, Debug, Default)]
pub struct PrefabInstance(pub Gap);

/// prefab 实例 GPU 数据组件（WP20 `update_instance_data` 消费；O 名，字段 U）。
#[derive(Component, Clone, Debug, Default)]
pub struct PrefabInstanceData(pub Gap);

/// 音频触发形状（与 `audio_trigger_library::AudioTrigger` 成对存储；字段 U）。
#[derive(Clone, Debug, Default)]
pub struct AudioTriggerShape(pub Gap);

/// prefab 库资源（O：`HashMap<PrefabName, Prefab>` + RandomState）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PrefabLibrary {
    pub inner: HashMap<PrefabName, Prefab>,
}

impl PrefabLibrary {
    /// O：`add` 0x1408e2540 = `HashMap::insert`；被替换旧值逐 `PrefabPart` drop。
    pub fn add(&mut self, name: PrefabName, prefab: Prefab) {
        self.inner.insert(name, prefab);
    }
    /// O：`get<&PrefabName>` 0x141552d70 = 名字 hash 查找 → `&Prefab`。
    pub fn get(&self, name: &PrefabName) -> Option<&Prefab> {
        self.inner.get(name)
    }
    /// O：`has<&PrefabNameRef>` 0x141552ff0 = 名字 hash 查找 → bool。
    pub fn has(&self, name: &PrefabNameRef) -> bool {
        self.inner.contains_key(name)
    }
    /// O：`iter`（遍历 entry）。
    pub fn iter(&self) -> impl Iterator<Item = (&PrefabName, &Prefab)> {
        self.inner.iter()
    }
}

/// O：`PrefabsModifiedCmd`（库变更事件）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrefabsModifiedCmd(pub Gap);

/// O：`PrefabWatchPlugin`（watch 目录/触发策略 U）。
pub struct PrefabWatchPlugin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_add_get_has_share_one_name_type() {
        let mut lib = PrefabLibrary::default();
        let name = PrefabName::new("house/a");
        lib.add(name.clone(), Prefab::default());
        assert!(lib.get(&name).is_some());
        assert!(lib.has(PrefabNameRef::new("house/a")), "借用名按内容命中");
        assert!(!lib.has(PrefabNameRef::new("house/b")));
        assert_eq!(lib.iter().count(), 1);
    }
}

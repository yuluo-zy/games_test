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

/// prefab 名（外部 `asset::prefab::PrefabName` 的借用投影：allocation 含 16B header，
/// byte_len 来自原 +8；O：候选 TG-a3f23bd62617630cba `as_name_ref` B=0x141556130）。
/// 这是借用视图，不实现 Arc/所有权/编码。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrefabName<'a> {
    pub allocation: &'a [u8],
    pub byte_len: usize,
}

impl<'a> PrefabName<'a> {
    /// O：原首指针 +16、长度不变。
    pub fn as_name_ref(&self) -> &'a [u8] {
        &self.allocation[16..][..self.byte_len]
    }
}

/// `PrefabNameRef`（外部 asset crate 的引用形态；布局 U）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrefabNameRef<'a> {
    pub name: &'a [u8],
}

impl<'a> From<PrefabName<'a>> for PrefabNameRef<'a> {
    fn from(name: PrefabName<'a>) -> Self {
        Self {
            name: name.as_name_ref(),
        }
    }
}

/// 库内部自有名键（O：键 = `PrefabName` 的字节投影；借用视图不能作长生命周期键，
/// 故库内按名字字节存储，语义等价，见合同 prefab.md）。
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OwnedPrefabName {
    pub name_bytes: Box<[u8]>,
}

impl<'a> From<PrefabName<'a>> for OwnedPrefabName {
    fn from(name: PrefabName<'a>) -> Self {
        Self {
            name_bytes: name.as_name_ref().to_vec().into_boxed_slice(),
        }
    }
}

impl<'a> From<PrefabNameRef<'a>> for OwnedPrefabName {
    fn from(name: PrefabNameRef<'a>) -> Self {
        Self {
            name_bytes: name.name.to_vec().into_boxed_slice(),
        }
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

/// prefab 库资源（O：`HashMap<PrefabName, Prefab>` + RandomState；键以
/// `OwnedPrefabName` 承载名字字节）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PrefabLibrary {
    pub inner: HashMap<OwnedPrefabName, Prefab>,
}

impl PrefabLibrary {
    /// O：`add` 0x1408e2540 = `HashMap::insert`；被替换旧值逐 `PrefabPart` drop。
    pub fn add<'a>(&mut self, name: PrefabName<'a>, prefab: Prefab) {
        self.inner.insert(name.into(), prefab);
    }
    /// O：`get<&PrefabName>` 0x141552d70 = 名字 hash 查找 → `&Prefab`。
    pub fn get<'a>(&self, name: &PrefabName<'a>) -> Option<&Prefab> {
        self.inner.get(&OwnedPrefabName::from(*name))
    }
    /// O：`has<&PrefabNameRef>` 0x141552ff0 = 名字 hash 查找 → bool。
    pub fn has<'a>(&self, name: &PrefabNameRef<'a>) -> bool {
        self.inner.contains_key(&OwnedPrefabName::from(*name))
    }
    /// O：`iter`（遍历 entry）。
    pub fn iter(&self) -> impl Iterator<Item = (&OwnedPrefabName, &Prefab)> {
        self.inner.iter()
    }
}

/// O：`PrefabsModifiedCmd`（库变更事件）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrefabsModifiedCmd(pub Gap);

/// O：`PrefabWatchPlugin`（watch 目录/触发策略 U）。
pub struct PrefabWatchPlugin;

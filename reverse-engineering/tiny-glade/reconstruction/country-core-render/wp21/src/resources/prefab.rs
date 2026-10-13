//! prefab.rs — 预制体库（WP21；扩展共享 crate `resources/prefab/mod.rs`（wp13））。
//!
//! O：`PrefabLibrary` = `HashMap<PrefabName, Prefab>`；`add` 0x8e2540（Prefab 含
//! `Vec<PrefabPart>`）、`get<PrefabName>` 0x1552d70、`has<PrefabNameRef>` 0x1552ff0、
//! `iter`、`PrefabInstance`（0x58B，impl$7::register_required_components 0x8e5d10）。

use std::collections::HashMap;

use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `PrefabName`（O：资产命名；布局 U）。
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrefabName(pub String);

/// `PrefabPart`（O：prefab 部件；drop 逐 part）。
#[derive(Clone, Debug, Default)]
pub struct PrefabPart(pub Gap);

/// `Prefab`（O：prefab 实体 = 部件列表）。
#[derive(Clone, Debug, Default)]
pub struct Prefab {
    pub parts: Vec<PrefabPart>,
}

/// `PrefabLibrary`（O：HashMap<PrefabName, Prefab>）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PrefabLibrary {
    pub prefabs: HashMap<PrefabName, Prefab>,
}

impl PrefabLibrary {
    /// `add`（O 0x8e2540）。
    pub fn add(&mut self, name: PrefabName, prefab: Prefab) {
        self.prefabs.insert(name, prefab);
    }
    /// `get`（O 0x1552d70）。
    pub fn get(&self, name: &PrefabName) -> Option<&Prefab> {
        self.prefabs.get(name)
    }
    /// `has`（O 0x1552ff0）。
    pub fn has(&self, name: &str) -> bool {
        self.prefabs.contains_key(&PrefabName(name.to_string()))
    }
    /// `iter`（O 名，2 内联）。
    pub fn iter(&self) -> impl Iterator<Item = (&PrefabName, &Prefab)> {
        self.prefabs.iter()
    }
}

/// `PrefabInstance`（O：ECS 组件；0x58B，注册 0x8e5d10）。
#[derive(Clone, Debug, Default)]
pub struct PrefabInstance(pub Gap);

/// `PrefabsModifiedCmd`（O：prefab 修改命令）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct PrefabsModifiedCmd(pub Gap);

/// `PrefabWatchPlugin`（O：prefab 热加载插件，U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct PrefabWatchPlugin;

/// `AudioTriggerShape`（O：音频触发形状；U）。
#[derive(Clone, Debug, Default)]
pub struct AudioTriggerShape(pub Gap);

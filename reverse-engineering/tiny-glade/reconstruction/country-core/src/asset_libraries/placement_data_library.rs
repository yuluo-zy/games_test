//! placement_data_library.rs — 放置数据注册表（O）。
//!
//! 源 `asset_libraries/placement_data_library.rs`；实体
//! `AssetPlacementDataLibrary` / `InstancedDrawFromPlacementData`。
//! `AssetPlacementDataLibrary::new`（0x140867700，两张 RandomState 表）/ `load`
//! （0x1409365d0，`std::path::PathBuf` 泛型）/ `add`（内联）/ `clear`（内联于
//! uninit_clearing）。放置数据为装饰物实例化配置（scatter/autoclutter 用）。

#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 放置数据条目（O：`InstancedDrawFromPlacementData`；U 布局）。
#[derive(Clone, Debug, Default)]
pub struct PlacementData {
    pub path: PathBuf,
    pub _payload: Gap,
}

/// 放置数据注册表（O：`Res<AssetPlacementDataLibrary>`；HashMap<PathBuf,
/// PlacementData>，O `new` 两张 hash 表）。
#[derive(Resource, Default)]
pub struct AssetPlacementDataLibrary {
    pub entries: HashMap<PathBuf, PlacementData>,
}

impl AssetPlacementDataLibrary {
    /// `new`（O 0x140867700）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `load<PathBuf>`（O 0x1409365d0）：解析放置数据 JSON/文件并入表
    /// （`impl$1::load::closure$0` 内联）。
    pub fn load(&mut self, path: PathBuf) -> Result<(), ()> {
        self.entries.insert(path.clone(), PlacementData { path, _payload: Gap });
        Ok(())
    }

    /// `add`（O：`load` 内联）。
    pub fn add(&mut self, data: PlacementData) {
        self.entries.insert(data.path.clone(), data);
    }

    /// `clear`（O：`uninit_clearing` 内联）。
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

//! image_formation.rs — 画面形成配置（WP21）。
//!
//! O：`ImageFormationConfigs`（三档 0xc0B config + kind u8；`switch_kind` 0x8aea90 写
//! +0xc0、`iter_non_default_mut` 0x8aea60 迭代、`default` 0x8ae7f0）。每档 config 名称
//! "default" + Vec<7 项>。`ArtisticConfig`/`CameraLensConfig` 子类型（serde）。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// `ImageFormationConfig`（O：0xc0B；名称 + Vec<7 项>，项布局 U）。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageFormationConfig {
    /// 名称（O：8B "default" 0x61666564/0x746c7561）。
    pub name: [u8; 8],
    /// 参数项（O：Vec 7 项；+0x8 ptr/cap/len 布局 O）。
    pub params: Vec<Gap>,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl Default for ImageFormationConfig {
    fn default() -> Self {
        Self {
            name: *b"default\0",
            params: (0..7).map(|_| Gap::default()).collect(),
            _gap: Gap::default(),
        }
    }
}

/// `ImageFormationConfigs`（O：三档 + 当前 kind）。
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct ImageFormationConfigs {
    pub configs: [ImageFormationConfig; 3],
    /// 当前档（O：+0xc0 字节，`switch_kind` 写）。
    pub kind: u8,
}

impl Default for ImageFormationConfigs {
    fn default() -> Self {
        Self {
            configs: std::array::from_fn(|_| ImageFormationConfig::default()),
            kind: 0,
        }
    }
}

impl ImageFormationConfigs {
    /// `initial`（O 0x67dd80 折叠）。
    pub fn initial() -> Self {
        Self::default()
    }
    /// `current`（O 名，7 内联）。
    pub fn current(&self) -> &ImageFormationConfig {
        &self.configs[self.kind as usize % 3]
    }
    /// `current_mut`（O 0x8aea50）。
    pub fn current_mut(&mut self) -> &mut ImageFormationConfig {
        &mut self.configs[self.kind as usize % 3]
    }
    /// `get`（O 0x8aea80）。
    pub fn get(&self, i: usize) -> Option<&ImageFormationConfig> {
        self.configs.get(i)
    }
    /// `iter_non_default_mut`（O 0x8aea60：三档迭代）。
    pub fn iter_non_default_mut(&mut self) -> impl Iterator<Item = &mut ImageFormationConfig> {
        self.configs.iter_mut()
    }
    /// `switch_kind`（O 0x8aea90）。
    pub fn switch_kind(&mut self, kind: u8) {
        self.kind = kind;
    }
}

/// `ArtisticConfig`（O 名；serde，U）。
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArtisticConfig(pub Gap);

/// `CameraLensConfig`（O 名；serde，U）。
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CameraLensConfig(pub Gap);

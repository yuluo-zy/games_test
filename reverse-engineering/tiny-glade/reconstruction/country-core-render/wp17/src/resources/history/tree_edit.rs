//! `resources::history::tree_edit` — 树编辑记录（WP17）。
//!
//! O（反编译 missing_prefab 0x1409e6210 / TreeCanopyMaxCullDistance::get_dist；
//! ReplayCmd 判别 8 负载 `Arc<Option<TreeEdit>>`；观察 JSON Tree Place 598 /
//! Move 11：Place(index + tree{id, position, scale, rotation_degrees,
//! prefab_name, is_preplaced})、Move(index/from/to)。serde 手写
//! serialize 0x17c2600 区 / deserialize 0x17c2f60）：

use crate::typed_gap::Gap;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// 树记录（O：观察 `tree` 字段）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TreeRecord {
    /// O：`id`（u64）。
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub position: Vec3,
    #[serde(default)]
    pub scale: f32,
    #[serde(default)]
    pub rotation_degrees: Vec3,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefab_name: Option<String>,
    #[serde(default)]
    pub is_preplaced: bool,
    /// U：额外字段。
    #[serde(default)]
    pub aux: Gap,
}

/// 树编辑记录（O：Place / Move）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TreeEdit {
    /// O：Place（index + tree.id 保留为不同字段）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<TreeRecord>,
    /// O：Move（index + from/to）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec3>,
    /// U：判别/载荷。
    #[serde(default)]
    pub aux: Gap,
}

impl TreeEdit {
    /// O：`missing_prefab` 0x1409e6210：无 prefab 时的处理。
    pub fn missing_prefab(_prefab: &str) -> Self {
        Self::default()
    }
}

/// 树冠最大剔除距离（O：`get_dist`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TreeCanopyMaxCullDistance(pub f32);

impl TreeCanopyMaxCullDistance {
    pub fn get_dist(&self) -> f32 {
        self.0
    }
}

//! terrain_editing.rs — 地形编辑数据（WP8 恢复）。
//!
//! 源：B skeleton `resources/terrain_editing.rs`。方法行号：`new`@64、`get_mut_stroke`@75、
//! `get_stroke`@79、`get_last_mut_stroke`@83、`last_stroke_id`@87、`new_stroke`@91、`set_stroke`@110、`clear`@120。
//! 反编译：`new_stroke`@0x140924a10：`last_stroke_id < 0x3fffff` 则 +1 并
//!   `HashMap<TerrainStrokeId, TerrainStroke>::insert`，否则 None。
//!   `get_last_mut_stroke`@0x140924900：按 `last_stroke_id` 查（0x30B 桶步长）。
//!   ECS：`ResMut<TerrainEditingData>`（O）。serde Compact/Pretty/Value（O 名）。
//! prior `tg-terrain::stroke.rs` 逐条恢复计数/插入规则（O 原机器码案例）。

#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::resources::terrain_stroke::TerrainStroke;
use crate::typed_gap::Gap;

/// 笔画 id 计数上限（O：`new_stroke` 中 `last_stroke_id < 0x3fffff`）。
pub const ID_COUNTER_LIMIT: u32 = 0x3fffff;

/// 笔画 id（O 名；u32 newtype，prior `tg-terrain::schema` serde transparent）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TerrainStrokeId(pub u32);

/// 地形编辑数据资源（O 名；`ResMut<TerrainEditingData>`）。
/// 字段观察（O）：`strokes: HashMap<TerrainStrokeId, TerrainStroke>`、`last_stroke_id: u32`。
/// serde：原文件派生 Serialize/Deserialize（save schema，O 名）；Gap 非 serde 故此处仅保留 Clone/Debug。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainEditingData {
    pub strokes: HashMap<TerrainStrokeId, TerrainStroke>,
    pub last_stroke_id: u32,
    /// typed gap：其它字段（宽度/模式等）U。
    pub _gap: Gap,
}

impl TerrainEditingData {
    /// 新建（O 名；`new`@64）。
    pub fn new() -> Self {
        Self::default()
    }

    /// 当前计数（O 名；`last_stroke_id`@87）。
    pub fn last_stroke_id(&self) -> TerrainStrokeId {
        TerrainStrokeId(self.last_stroke_id)
    }

    /// 新建笔画：计数超限返回 None，否则 +1 并插入（O：`new_stroke`@0x140924a10，行 91）。
    pub fn new_stroke(&mut self, stroke: TerrainStroke) -> Option<TerrainStrokeId> {
        if self.last_stroke_id >= ID_COUNTER_LIMIT {
            return None;
        }
        self.last_stroke_id += 1;
        let id = TerrainStrokeId(self.last_stroke_id);
        self.strokes.insert(id, stroke);
        Some(id)
    }

    /// 取末笔画可变引用（O：`get_last_mut_stroke`@0x140924900，行 83）。
    pub fn get_last_mut_stroke(&mut self) -> Option<&mut TerrainStroke> {
        let id = TerrainStrokeId(self.last_stroke_id);
        self.strokes.get_mut(&id)
    }

    /// 取指定笔画可变引用（O 名；`get_mut_stroke`@75）。
    pub fn get_mut_stroke(&mut self, id: TerrainStrokeId) -> Option<&mut TerrainStroke> {
        self.strokes.get_mut(&id)
    }

    /// 取指定笔画（O 名；`get_stroke`@79）。
    pub fn get_stroke(&self, id: TerrainStrokeId) -> Option<&TerrainStroke> {
        self.strokes.get(&id)
    }

    /// 置入笔画（O 名；`set_stroke`@110）。
    pub fn set_stroke(&mut self, id: TerrainStrokeId, stroke: TerrainStroke) {
        self.strokes.insert(id, stroke);
    }

    /// 清空（O 名；`clear`@120）。
    pub fn clear(&mut self) {
        self.strokes.clear();
        self.last_stroke_id = 0;
    }
}

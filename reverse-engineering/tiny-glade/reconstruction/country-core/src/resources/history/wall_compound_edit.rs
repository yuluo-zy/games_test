//! `resources::history::wall_compound_edit` — 墙复合历史编辑（WP17）。
//!
//! O（反编译 CompoundEditBuilder::new_edit_sequence 0x140a82b20 / next
//! 0x140a82b40 / build 0x140a82d50 / panic_deprecated_operation 0x140a83120；
//! `EditType::new_height_changed` 0x140a83fb0 写 0x108B 负载 + Arc refcount
//! `0x8000000000000002`；观察 JSON 墙命令组 `[[{Operation}…],…]`）：
//!   - `WallHistoryEdit`：墙历史编辑操作（enum；观察 10+ 内层操作）。
//!   - `ConcurrentEdits`：`Vec<Vec<Arc<WallHistoryEdit>>>`（组数组）。
//!   - `WallHistoryCompoundEdit`：`new`（0x108B）；serde 手写。
//!   - `CompoundEditBuilder`：new_edit_sequence / next / build / next_empty /
//!     with_concurrent；`wall_deleted`（0x140a876a0，wp12 记录）。

use crate::resources::walls::WallId;
use crate::typed_gap::Gap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 预制形状（O：`MorphPremadeShape::{before,after}` 的 serde 载荷，观察
/// starting-builds `WallNewSystem`/`Wall` 历史 JSON 80+85 条）。
///
/// 外部标签枚举：`{"Circle":{...}}` / `{"Rectangle":{...}}`。
/// - `Circle { center: [f32;2], radius: f32 }`
/// - `Rectangle { center: [f32;2], length: f32, width: f32,
///   orientation: [f32;2] }`
///
/// 运行时内存布局 U；这里按 serde schema 合同恢复（JSON 字段全覆盖）。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PremadeShape {
    /// 圆形：center 数组 + radius。
    Circle {
        center: [f32; 2],
        radius: f32,
    },
    /// 矩形：center/orientation 数组 + length/width。
    Rectangle {
        center: [f32; 2],
        length: f32,
        width: f32,
        orientation: [f32; 2],
    },
}

/// `MorphPremadeShape`（O：观察 JSON 字段 `wall_index/before/after`，
/// 80 条 WallNewSystem + 85 条 Wall 记录，`present_in_every_observed_object`）。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MorphPremadeShape {
    /// O：`wall_index`（JSON integer）。
    pub wall_index: u32,
    /// O：变更前形状。
    pub before: PremadeShape,
    /// O：变更后形状。
    pub after: PremadeShape,
}

/// 墙历史编辑操作（O：enum2$；观察 JSON 内层操作）。未证负载字段 U。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum WallHistoryEdit {
    /// CreateWall（wall_index / wall_height / elevation / color_id / after）。
    #[default]
    CreateWall,
    /// WallChanged（wall_index / before / after / operation_ty）。
    WallChanged,
    /// DeleteWall（wall_index / wall_height / color_id / before）。
    DeleteWall,
    /// AdjustHeight（wall_index / before / after）。
    AdjustHeight,
    /// MorphPremadeShape（`wall_index / before / after`；O schema 见
    /// [`MorphPremadeShape`]）。
    MorphPremadeShape(MorphPremadeShape),
    /// ChangeStyle / ChangeElevation / ChangeArchHeight / MorphFreehandWall /
    /// ChangeIsEnclosedState / ChangeFreehandHeightType / ChangeArchType。
    MorphFreehandWall,
    /// 未证操作（观察 open）。
    Other(Gap),
}

/// 并发编辑组（O：`[[{Operation}…],…]` 组数组；元素 `Vec<Arc<WallHistoryEdit>>`）。
/// serde 原为手写 visitor（`_::impl$0`）；此处不 derive（Arc 非 serde）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConcurrentEdits(pub Vec<Vec<Arc<WallHistoryEdit>>>);

/// 墙复合历史编辑（O：`new`；0x108B 负载；serde 手写 `_::impl$0` visitor 族）。
#[derive(Clone, Debug, PartialEq)]
pub struct WallHistoryCompoundEdit {
    /// O：有序 `ConcurrentEdits` 组。
    pub concurrent_groups: ConcurrentEdits,
    /// U：额外负载（elevation/curve 等）。
    pub aux: Gap,
}

impl Default for WallHistoryCompoundEdit {
    fn default() -> Self {
        Self {
            concurrent_groups: ConcurrentEdits::default(),
            aux: Gap,
        }
    }
}

impl WallHistoryCompoundEdit {
    /// O：`new`（0x108B 负载构建）。
    pub fn new(groups: ConcurrentEdits) -> Self {
        Self {
            concurrent_groups: groups,
            aux: Gap,
        }
    }
    /// O：`impl$5::fmt` 0x140a87970（Display）。
    pub fn describe(&self) -> String {
        format!(
            "WallHistoryCompoundEdit({} groups)",
            self.concurrent_groups.0.len()
        )
    }
}

/// 复合编辑构建器（O：new_edit_sequence 0x140a82b20 / next 0x140a82b40 /
/// build 0x140a82d50；内部状态 U）。
#[derive(Clone, Debug, Default)]
pub struct CompoundEditBuilder {
    /// U：进行中的组/编辑序列。
    pub state: Gap,
}

impl CompoundEditBuilder {
    /// O：`new_edit_sequence` 0x140a82b20。
    pub fn new_edit_sequence() -> Self {
        Self { state: Gap }
    }
    /// O：`next_empty`（当前组是否为空）。
    pub fn next_empty(&self) -> bool {
        true
    }
    /// O：`with_concurrent`（加入并发组）。
    pub fn with_concurrent(&mut self, _group: Vec<Arc<WallHistoryEdit>>) {}
    /// O：`next` 0x140a82b40（推进到下一组）。
    pub fn next(&mut self) {}
    /// O：`build` 0x140a82d50 → `Option<WallHistoryCompoundEdit>`。
    pub fn build(&self) -> Option<WallHistoryCompoundEdit> {
        Some(WallHistoryCompoundEdit::default())
    }
    /// O：`wall_deleted`（0x140a876a0；wp12 记录 Delete(snapshot) 组）。
    pub fn wall_deleted(&mut self, _wall: WallId) {}
    /// O：`panic_deprecated_operation` 0x140a83120。
    pub fn panic_deprecated_operation() -> ! {
        panic!("Deprecated wall compound edit operation")
    }
}

#[cfg(test)]
mod morph_premade_shape_tests {
    use super::*;

    #[test]
    fn morph_premade_shape_serde_schema_matches_observed_json() {
        let edit = WallHistoryEdit::MorphPremadeShape(MorphPremadeShape {
            wall_index: 7,
            before: PremadeShape::Rectangle {
                center: [1.0, 2.0],
                length: 3.0,
                width: 4.0,
                orientation: [1.0, 0.0],
            },
            after: PremadeShape::Circle {
                center: [5.0, 6.0],
                radius: 2.5,
            },
        });
        let v: serde_json::Value = serde_json::to_value(&edit).unwrap();
        let m = &v["MorphPremadeShape"];
        assert_eq!(m["wall_index"], 7);
        assert_eq!(m["before"]["Rectangle"]["center"][0], 1.0);
        assert_eq!(m["before"]["Rectangle"]["length"], 3.0);
        assert_eq!(m["before"]["Rectangle"]["width"], 4.0);
        assert_eq!(m["before"]["Rectangle"]["orientation"][1], 0.0);
        assert_eq!(m["after"]["Circle"]["radius"], 2.5);
        let back: WallHistoryEdit = serde_json::from_value(v).unwrap();
        assert_eq!(back, edit);
    }
}

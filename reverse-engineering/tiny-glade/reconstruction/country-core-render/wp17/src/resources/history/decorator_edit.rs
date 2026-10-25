//! `resources::history::decorator_edit` — 装饰编辑记录（WP17）。
//!
//! O（反编译 convert_before_after_states_to_changes 0x140b31220 /
//! send_state_diff_events 0x14017bdae0 / impl$1::from 0x140b31240 /
//! is_default_chimney_smoke 0x1408c5680 / is_default_stair_width 0x140b3d590；
//! 观察 JSON `decorators.diffs` 427 批 / 523 记录变化；target
//! {Wall:数字}/"Terrain"/"Stairs"）：
//!   - `DecoratorHistoryEdit`：装饰编辑记录（ReplayCmd 判别 6/7 负载）。
//!   - `DecoratorHistoryDiff`：装饰 diff（`[target, {before, after}]`）。
//!   - `send_state_diff_events<closure_env$0>`：diff → 状态事件
//!     （replay_pop_bucket 的 restore_decorators 闭包）。
//!   - `convert_before_after_states_to_changes`：before/after → 变更集。

use crate::resources::history::Gap as HistoryDiffGap;
use crate::typed_gap::Gap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 装饰目标（O：观察 {Wall:数字}/"Terrain"/"Stairs"）。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecoratorTarget {
    Wall(u64),
    Terrain,
    Stairs,
    /// 未证目标（观察 open）。
    Other(Gap),
}

impl Default for DecoratorTarget {
    fn default() -> Self {
        DecoratorTarget::Terrain
    }
}

/// 装饰记录（O：观察字段 dst/rank/seed/ty/color_id/allow_connection/
/// chimney_h_offset/h_offset/clv/stair_width/railing/ladder_color）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DecoratorRecord {
    /// O：附着目标（Wall anchor/coord、Roof mesh_uv、Terrain pos_xz/yaw、
    /// Stair graph_edge/uv/yaw）。
    #[serde(default)]
    pub dst: Gap,
    #[serde(default)]
    pub rank: u32,
    #[serde(default)]
    pub seed: u32,
    #[serde(default)]
    pub ty: Gap,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_id: Option<u64>,
    #[serde(default)]
    pub aux: Gap,
}

/// 装饰历史编辑记录（O：ReplayCmd 判别 6/7 负载 `Arc<Option<..>>`；serde 手写）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DecoratorHistoryEdit {
    #[serde(default)]
    pub before: HashMap<u64, DecoratorRecord>,
    #[serde(default)]
    pub after: HashMap<u64, DecoratorRecord>,
    #[serde(default)]
    pub aux: Gap,
}

/// 装饰历史 diff（O：`[target, {before:{id:record}, after:{id:record}}]`；
/// replay_edit → ReplayBucket、replay_pop_bucket 消费）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DecoratorHistoryDiff {
    #[serde(default)]
    pub diffs: Vec<(DecoratorTarget, BeforeAfterState)>,
}

/// before/after 状态对（O：观察 before 357 条 / after 485 条）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BeforeAfterState {
    #[serde(default)]
    pub before: HashMap<u64, DecoratorRecord>,
    #[serde(default)]
    pub after: HashMap<u64, DecoratorRecord>,
}

impl DecoratorHistoryEdit {
    /// O：`convert_before_after_states_to_changes` 0x140b31220。
    pub fn convert_before_after_states_to_changes(&self) -> Vec<Gap> {
        Vec::new()
    }
}

/// O：`send_state_diff_events<C>` 0x14017bdae0：把 diff 状态变更事件交给闭包 C
/// （replay_pop_bucket::closure_env$0 宿主）。
pub fn send_state_diff_events<C>(_diff: &DecoratorHistoryDiff, _apply: C)
where
    C: FnMut(),
{
}

/// O：`is_default_chimney_smoke` 0x1408c5680。
pub fn is_default_chimney_smoke(_record: &DecoratorRecord) -> bool {
    false
}
/// O：`is_default_stair_width` 0x140b3d590。
pub fn is_default_stair_width(_record: &DecoratorRecord) -> bool {
    false
}
/// O：`is_default_lantern_brightness`。
pub fn is_default_lantern_brightness(_record: &DecoratorRecord) -> bool {
    false
}

/// 装饰存储历史 diff 边界（decorator_archivist 消费；U）。
pub type DecoratorStorageHistoryDiff = HistoryDiffGap;

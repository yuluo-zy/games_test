//! Live history uses the recovered transaction owner directly. Edit tags are
//! verified against original B Debug dispatch 0x140a842c0 / 0x142ad88d0.
//! Variants whose payloads remain unresolved are not represented by empty data.
pub mod auto_clutter_edit;
pub mod color_edit;
pub mod decorator_edit;
pub mod path_edit;
pub mod replay_cmd_buffer;
pub mod stairs_history_edit;
pub mod tree_edit;

use crate::recovered_building::{
    building_history_transactions::RoofHistoryEdit, history_transactions::HistoryTransactions,
    wall_history_edit::WallCompoundHistoryEdit,
};
use crate::resources::raster_stroke::RasterStroke;
use crate::systems::terrain::deferred_terrain_history::TerrainEdit;
use std::sync::Arc;

pub use replay_cmd_buffer::{HistoryReplayError, ReplayBucket, ReplayBuffer, ReplayCommand};
pub type History<S = ()> = HistoryTransactions<EditType, S>;

/// B `HistoryIdentity`（u64 新类型；Identity 存档节载荷 = 裸 u64 十进制数字，
/// `to_json_string<HistoryIdentity>` 0x140b385e0）。`generate` 0x140a84220 = fastrand u64。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct HistoryIdentity(pub u64);
impl HistoryIdentity {
    /// `generate` 0x140a84220：fastrand u64 全范围。
    pub fn generate() -> Self {
        Self(fastrand::u64(..))
    }
    /// `mutate`（行 102 内联）：重随机。
    pub fn mutate(&mut self) {
        self.0 = fastrand::u64(..);
    }
}

#[derive(Clone, Debug)]
pub enum EditType {
    /// tag 0（E1：visit_str 0x140b3b630 权威 15 变体）。
    Tree(Arc<tree_edit::TreeEdit>),
    Path(Arc<RasterStroke>),
    Cobblestone1(Arc<RasterStroke>),
    Cobblestone2(Arc<RasterStroke>),
    Cobblestone3(Arc<RasterStroke>),
    Water(Arc<RasterStroke>),
    Garden(Arc<RasterStroke>),
    Wall(Arc<WallCompoundHistoryEdit>),
    Roof(Arc<RoofHistoryEdit>),
    Color(Arc<color_edit::ColorEdit>),
    DecoratorV5(Arc<decorator_edit::DecoratorHistoryEdit>),
    Decorator(Arc<decorator_edit::DecoratorHistoryEdit>),
    AutoClutter(Arc<auto_clutter_edit::AutoClutterEdit>),
    Stairs(Arc<stairs_history_edit::StairsHistoryEdit>),
    /// 地形历史（`history_record_terrain` 0x140ac9770 O；**原版 EditType visit_str
    /// 0x140b3b630 无此名**——地形记录机制待 E 补挖，tag 待定位，不伪造）。
    Terrain(Arc<TerrainEdit>),
}
impl From<RoofHistoryEdit> for EditType {
    fn from(edit: RoofHistoryEdit) -> Self {
        Self::Roof(Arc::new(edit))
    }
}
impl EditType {
    pub const fn native_tag(&self) -> u64 {
        match self {
            // E1（RE-EDITTYPE.md）：15 变体 tag 0..14 权威表。
            Self::Tree(_) => 0,
            Self::Path(_) => 1,
            Self::Cobblestone1(_) => 2,
            Self::Cobblestone2(_) => 3,
            Self::Cobblestone3(_) => 4,
            Self::Water(_) => 5,
            Self::Garden(_) => 6,
            Self::Wall(_) => 7,
            Self::Roof(_) => 8,
            Self::Color(_) => 9,
            Self::DecoratorV5(_) => 10,
            Self::Decorator(_) => 11,
            Self::AutoClutter(_) => 13,
            Self::Stairs(_) => 14,
            // 地形：原版无此 EditType 名（tag 12 为 4 字符未解名）；保留登记待 E 定位。
            Self::Terrain(_) => 15,
        }
    }
}

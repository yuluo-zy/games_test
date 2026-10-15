//! Live history uses the recovered transaction owner directly. Edit tags are
//! verified against original B Debug dispatch 0x140a842c0 / 0x142ad88d0.
//! Variants whose payloads remain unresolved are not represented by empty data.
pub mod auto_clutter_edit;
pub mod replay_cmd_buffer;

use crate::recovered_building::{
    building_history_transactions::RoofHistoryEdit, history_transactions::HistoryTransactions,
    wall_history_edit::WallCompoundHistoryEdit,
};
use crate::resources::raster_stroke::RasterStroke;
use std::sync::Arc;

pub use replay_cmd_buffer::{HistoryReplayError, ReplayBucket, ReplayBuffer, ReplayCommand};
pub type History<S = ()> = HistoryTransactions<EditType, S>;

#[derive(Clone, Debug)]
pub enum EditType {
    Path(Arc<RasterStroke>),
    Cobblestone1(Arc<RasterStroke>),
    Cobblestone2(Arc<RasterStroke>),
    Cobblestone3(Arc<RasterStroke>),
    Water(Arc<RasterStroke>),
    Garden(Arc<RasterStroke>),
    Wall(Arc<WallCompoundHistoryEdit>),
    Roof(Arc<RoofHistoryEdit>),
    AutoClutter(Arc<auto_clutter_edit::AutoClutterEdit>),
}
impl From<RoofHistoryEdit> for EditType {
    fn from(edit: RoofHistoryEdit) -> Self {
        Self::Roof(Arc::new(edit))
    }
}
impl EditType {
    pub const fn native_tag(&self) -> u64 {
        match self {
            Self::Path(_) => 1,
            Self::Cobblestone1(_) => 2,
            Self::Cobblestone2(_) => 3,
            Self::Cobblestone3(_) => 4,
            Self::Water(_) => 5,
            Self::Garden(_) => 6,
            Self::Wall(_) => 7,
            Self::Roof(_) => 8,
            Self::AutoClutter(_) => 13,
        }
    }
}

//! `systems::history::replay_water` — 水面回放（WP17）。
//!
//! O（PDB 骨架 + enriched 接口）：
//!   - `replay_water` = `replay_raster<WaterRasterStagingSsbo, WorldRaster<512>,
//!     replay_water::closure_env$2>`，签名 `(RasterReplayResources,
//!     EventWriter<WaterDrawEvent>, ResMut<WaterRaster>,
//!     EventWriter<RecalculateHeightMapCmd>, Res<ScheduledTerrainReadback>)`
//!     （宿主 @0xae1c50/0x14dfcd0；编译实例 @0x9f4650）。
//!   - 命令生成闭包 `closure$1::closure_env$0`（`generate_replay_cmds_from_edit`
//!     宿主 @0x1444c70）。

use crate::resources::history::path_edit::RasterEdit;
use crate::resources::history::replay_cmd_buffer::ReplayCmd;
use crate::systems::history::replay_raster::{RasterCmdFilter, RasterReplayResources, RasterStateBoundary};
use crate::typed_gap::Gap;
use std::sync::Arc;

/// 水面命令过滤（O：判别 0/1/2 中水面的 RasterEdit 变体；U 判别歧义）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WaterFilter;

impl RasterCmdFilter for WaterFilter {
    fn filter<'a>(&self, cmd: &'a ReplayCmd) -> Option<&'a Arc<RasterEdit>> {
        match cmd {
            ReplayCmd::Path(e) => Some(e),
            ReplayCmd::Cobblestone1(_)
            | ReplayCmd::Cobblestone2(_)
            | ReplayCmd::Terrain(_)
            | ReplayCmd::Wall(_)
            | ReplayCmd::Roof(_)
            | ReplayCmd::Decorator(_)
            | ReplayCmd::DecoratorSecondary(_)
            | ReplayCmd::Tree(_)
            | ReplayCmd::AutoClutter(_)
            | ReplayCmd::Stairs(_)
            | ReplayCmd::Unknown(_) => None,
        }
    }
}

/// 水面绘制事件（O：`resources::water::WaterDrawEvent`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaterDrawEvent;

/// O：`replay_water`（replay_raster<Water> 实例；@0x1409f4650）。
pub fn replay_water(
    resources: &mut RasterReplayResources,
    state: &mut RasterStateBoundary,
    out_draw_events: &mut Vec<Gap>,
) {
    let filter = WaterFilter;
    crate::systems::history::replay_raster::replay_raster(resources, state, &filter, out_draw_events);
}

/// O：`replay_water::closure$1`（EditType Water → ReplayCmd 生成；
/// `generate_replay_cmds_from_edit` 宿主 @0x1444c70）。
pub fn generate_water_replay_cmds(edit: &RasterEdit, forward: bool) -> Vec<ReplayCmd> {
    let mut cmds = Vec::new();
    cmds.push(ReplayCmd::Path(Arc::new(edit.clone())));
    let _ = forward;
    cmds
}

/// 水面光栅边界（O：`ResMut<WaterRaster>`）。
#[derive(Clone, Debug, Default)]
pub struct WaterRasterBoundary(pub RasterStateBoundary);

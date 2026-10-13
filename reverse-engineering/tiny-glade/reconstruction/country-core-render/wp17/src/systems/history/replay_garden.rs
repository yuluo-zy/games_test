//! `systems::history::replay_garden` — 花园回放（WP17）。
//!
//! O（PDB 骨架 + enriched 接口）：
//!   - `replay_garden` = `replay_raster<GardenRasterStagingSsbo,
//!     WorldRaster<512>, replay_garden::closure_env$2>`，签名
//!     `(RasterReplayResources, EventWriter<GardenDrawEvent>,
//!     ResMut<GardenRaster>)`（宿主 @0x14c7090/0x17689f0；编译实例
//!     @0x9f6ff0）。
//!   - 命令生成闭包 `closure$1::closure_env$0`（`generate_replay_cmds_from_edit`
//!     宿主 @0x143d2c0）。

use crate::resources::history::path_edit::RasterEdit;
use crate::resources::history::replay_cmd_buffer::ReplayCmd;
use crate::systems::history::replay_raster::{RasterCmdFilter, RasterReplayResources, RasterStateBoundary};
use crate::typed_gap::Gap;
use std::sync::Arc;

/// 花园命令过滤（O：判别 0/1/2 中花园的 RasterEdit 变体；U 判别歧义）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenFilter;

impl RasterCmdFilter for GardenFilter {
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

/// 花园绘制事件（O：`systems::garden::GardenDrawEvent`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GardenDrawEvent;

/// O：`replay_garden`（replay_raster<Garden> 实例；@0x1409f6ff0）。
pub fn replay_garden(
    resources: &mut RasterReplayResources,
    state: &mut RasterStateBoundary,
    out_draw_events: &mut Vec<Gap>,
) {
    let filter = GardenFilter;
    crate::systems::history::replay_raster::replay_raster(resources, state, &filter, out_draw_events);
}

/// O：`replay_garden::closure$1`（EditType Garden → ReplayCmd 生成；
/// `generate_replay_cmds_from_edit` 宿主 @0x143d2c0）。
pub fn generate_garden_replay_cmds(edit: &RasterEdit, forward: bool) -> Vec<ReplayCmd> {
    let mut cmds = Vec::new();
    cmds.push(ReplayCmd::Path(Arc::new(edit.clone())));
    let _ = forward;
    cmds
}

/// 花园光栅边界（O：`ResMut<GardenRaster>`）。
#[derive(Clone, Debug, Default)]
pub struct GardenRasterBoundary(pub RasterStateBoundary);

//! `systems::history::replay_path` — 路径回放（WP17）。
//!
//! O（PDB 骨架 + enriched 接口）：
//!   - `replay_path` 系统实例化 `replay_raster<*RasterStagingSsbo,
//!     WorldRaster<512>, replay_path::closure_env$1<T>>`，签名
//!     `(RasterReplayResources, ResMut<PathRaster/Cobblestone*Raster>,
//!     Commands)`（Path/C1/C2/C3 4 个：宿主 0xae1620/0xae2130/0xae1a40/
//!     0xae1830）。
//!   - `PathFilter` 系列：DirtPathFilter / Cobblestone1PathFilter /
//!     Cobblestone2PathFilter / Cobblestone3PathFilter。
//!   - `generate_path_replay_cmds` 0x14016286d0：Path/Cobblestone edit →
//!     ReplayCmd（closure$0 @0x14475a0；`generate_replay_cmds_from_edit`
//!     宿主 @0x14423e0）。

use crate::resources::history::path_edit::RasterEdit;
use crate::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCmd};
use crate::systems::history::replay_raster::{RasterCmdFilter, RasterStateBoundary};
use crate::typed_gap::Gap;
use std::sync::Arc;

/// 路径过滤类型标记（O：`closure_env$1<T>` 的 T）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DirtPathFilter;
#[derive(Clone, Copy, Debug, Default)]
pub struct Cobblestone1PathFilter;
#[derive(Clone, Copy, Debug, Default)]
pub struct Cobblestone2PathFilter;
#[derive(Clone, Copy, Debug, Default)]
pub struct Cobblestone3PathFilter;

/// 路径领域命令过滤（O：判别 0/1/2 = RasterEdit 系列）。
pub struct PathFilter<T> {
    /// 目标判别（O：判别字节；0=Path, 1=C1, 2=C2/C3）。
    pub discrim: u8,
    pub _marker: std::marker::PhantomData<T>,
}

impl<T> PathFilter<T> {
    pub fn new(discrim: u8) -> Self {
        Self {
            discrim,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> RasterCmdFilter for PathFilter<T> {
    fn filter<'a>(&self, cmd: &'a ReplayCmd) -> Option<&'a Arc<RasterEdit>> {
        match cmd {
            ReplayCmd::Path(e) if self.discrim == 0 => Some(e),
            ReplayCmd::Cobblestone1(e) if self.discrim == 1 => Some(e),
            ReplayCmd::Cobblestone2(e) if self.discrim == 2 => Some(e),
            _ => None,
        }
    }
}

/// O：`replay_path`（replay_raster<Path> 实例；@0x1409febf0 编译实例）。
pub fn replay_path_dirt(
    resources: &mut crate::systems::history::replay_raster::RasterReplayResources,
    state: &mut RasterStateBoundary,
    out_draw_events: &mut Vec<Gap>,
) {
    replay_raster_path_inner(resources, state, &PathFilter::<DirtPathFilter>::new(0), out_draw_events);
}

pub fn replay_path_cobblestone1(
    resources: &mut crate::systems::history::replay_raster::RasterReplayResources,
    state: &mut RasterStateBoundary,
    out_draw_events: &mut Vec<Gap>,
) {
    replay_raster_path_inner(resources, state, &PathFilter::<Cobblestone1PathFilter>::new(1), out_draw_events);
}

pub fn replay_path_cobblestone2(
    resources: &mut crate::systems::history::replay_raster::RasterReplayResources,
    state: &mut RasterStateBoundary,
    out_draw_events: &mut Vec<Gap>,
) {
    replay_raster_path_inner(resources, state, &PathFilter::<Cobblestone2PathFilter>::new(2), out_draw_events);
}

pub fn replay_path_cobblestone3(
    resources: &mut crate::systems::history::replay_raster::RasterReplayResources,
    state: &mut RasterStateBoundary,
    out_draw_events: &mut Vec<Gap>,
) {
    replay_raster_path_inner(resources, state, &PathFilter::<Cobblestone3PathFilter>::new(2), out_draw_events);
}

fn replay_raster_path_inner<F: RasterCmdFilter>(
    resources: &mut crate::systems::history::replay_raster::RasterReplayResources,
    state: &mut RasterStateBoundary,
    filter: &F,
    out_draw_events: &mut Vec<Gap>,
) {
    crate::systems::history::replay_raster::replay_raster(
        resources,
        state,
        filter,
        out_draw_events,
    );
}

/// O：`generate_path_replay_cmds` 0x14016286d0：Path/Cobblestone edit →
/// ReplayCmd 列表（ReplayCmd::from(RasterEdit)）。
pub fn generate_path_replay_cmds(edit: &RasterEdit, forward: bool) -> Vec<ReplayCmd> {
    let mut cmds = Vec::new();
    // O：`ReplayCmd::from(RasterEdit)`（impl$0..from 内联；判别由领域决定）。
    cmds.push(ReplayCmd::Path(Arc::new(edit.clone())));
    let _ = forward;
    cmds
}

/// 路径领域 buffer（O：`ResMut<PathRaster>` 边界）。
#[derive(Clone, Debug, Default)]
pub struct PathRasterBoundary(pub RasterStateBoundary);

pub type PathBufferAccess = ReplayBuffer;

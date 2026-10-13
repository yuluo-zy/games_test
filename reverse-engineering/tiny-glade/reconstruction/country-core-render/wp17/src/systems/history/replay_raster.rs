//! `systems::history::replay_raster` — 光栅回放泛型系统（WP17）。
//!
//! O（PDB 骨架：`RasterReplayCmd::forward` 0x1401700430 / `backward`
//! 0x1401700620 / `forward_instant`/`backward_instant`（74 宿主点）/`new`
//! （149 宿主点）；`RasterReplayResources` SystemParam bundle（get_param
//! 0x140ae5830/0x1401507610、init_state、validate_param、impl$8::default）；
//! 反编译 replay_raster<Path> 0x1409febf0）：
//!   - 泛型系统 `replay_raster<T, R, F>`：T=StagingSsbo、R=WorldRaster<512>、
//!     F=命令过滤闭包。六个实例化（Path/C1/C2/C3/Garden/Water）。
//!   - 过滤闭包把 ReplayCmd → Option<&RasterEdit>；undo 时 clear_buffer<512>
//!     + 重绘存储 stroke；redo 时 rasterize_stroke + Commands::send_event<…>；
//!     DirtyTiles::rasterize_world_space_stroke 写 dirty tiles。
//!   - instant/动画节奏。

use crate::resources::history::path_edit::RasterEdit;
use crate::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCmd};
use crate::systems::history::replay_pop_bucket::remove_done_commands;
use crate::typed_gap::Gap;
use std::sync::Arc;

/// 命令过滤闭包 trait（O：`replay_path::closure_env$1<T>` / `replay_water::
/// closure_env$2` / `replay_garden::closure_env$2`）。
pub trait RasterCmdFilter {
    /// 若命令属于本领域光栅编辑，返回其 RasterEdit 引用。
    fn filter<'a>(&self, cmd: &'a ReplayCmd) -> Option<&'a Arc<RasterEdit>>;
}

/// 光栅回放资源（O：`RasterReplayResources` SystemParam bundle；含 ReplayBuffer
/// + GameTime + StagingSsbo；精确字段 U）。
#[derive(Clone, Debug, Default)]
pub struct RasterReplayResources {
    pub buffer: ReplayBuffer,
    pub dt: f32,
    /// U：staging SSBO / WorldRaster 访问。
    pub aux: Gap,
}

impl RasterReplayResources {
    /// O：`impl$8::default`。
    pub fn default_resource() -> Self {
        Self::default()
    }
    /// O：`_::impl$0::get_param`（0x140ae5830/0x1401507610）。
    pub fn get_param(&mut self) {}
    /// O：`init_state` / `validate_param`。
    pub fn init_state(&mut self) {}
    pub fn validate_param(&mut self) {}
}

/// 光栅回放命令（O：`RasterReplayCmd`；负载 `Arc<RasterEdit>`）。
#[derive(Clone, Debug, Default)]
pub struct RasterReplayCmd {
    pub edit: Option<Arc<RasterEdit>>,
    pub forward: bool,
    pub instant: bool,
    pub duration: f32,
    pub elapsed: f32,
    pub undo: bool,
    /// U：started(+0x19) / redo(+0x1a) 标志。
    pub aux: Gap,
}

impl RasterReplayCmd {
    /// O：`new`（149 宿主点）。
    pub fn new(edit: Arc<RasterEdit>, forward: bool) -> Self {
        Self {
            edit: Some(edit),
            forward,
            instant: false,
            duration: 0.2,
            elapsed: 0.0,
            undo: !forward,
            aux: Gap,
        }
    }
    /// O：`forward` 0x1401700430。
    pub fn forward(&mut self, edit: Arc<RasterEdit>) {
        *self = Self::new(edit, true);
    }
    /// O：`backward` 0x1401700620。
    pub fn backward(&mut self, edit: Arc<RasterEdit>) {
        *self = Self::new(edit, false);
    }
    /// O：`forward_instant` / `backward_instant`（74 宿主点）。
    pub fn forward_instant(&mut self, edit: Arc<RasterEdit>) {
        let mut c = Self::new(edit, true);
        c.instant = true;
        *self = c;
    }
    pub fn backward_instant(&mut self, edit: Arc<RasterEdit>) {
        let mut c = Self::new(edit, false);
        c.instant = true;
        *self = c;
    }
}

/// 光栅域状态边界（O：`WorldRaster<512>` + StagingSsbo；apply 入口 U）。
#[derive(Clone, Debug, Default)]
pub struct RasterStateBoundary(pub Gap);

impl RasterStateBoundary {
    /// O：`clear_buffer<512>`。
    pub fn clear_buffer(&mut self) {}
    /// O：`rasterize_stroke<512>`（域内重绘）。
    pub fn rasterize_stroke(&mut self, _points: &[glam::Vec3]) {}
    /// O：`DirtyTiles::rasterize_world_space_stroke`。
    pub fn rasterize_world_space_stroke(&mut self, _points: &[glam::Vec3]) {}
}

/// O：`replay_raster<T, R, F>` 泛型系统。骨架实现：过滤前桶命令，逐条按
/// instant/动画节奏应用；undo 清缓冲重绘，redo 画编辑 stroke。
pub fn replay_raster<F: RasterCmdFilter>(
    resources: &mut RasterReplayResources,
    state: &mut RasterStateBoundary,
    filter: &F,
    out_draw_events: &mut Vec<Gap>,
) {
    let Some(bucket) = resources.buffer.front_mut_bucket() else {
        return;
    };
    let mut done = Vec::new();
    for (i, cmd) in bucket.commands.iter_mut().enumerate() {
        let Some(edit) = filter.filter(cmd) else {
            continue;
        };
        let _ = edit;
        // O：undo → clear + 重绘存储 stroke；redo → rasterize_stroke +
        // Commands::send_event（PathDrawEvent 等）。
        if state_matches_undo(cmd) {
            state.clear_buffer();
        } else {
            state.rasterize_stroke(&[]);
            let _ = out_draw_events.push(Gap);
        }
        state.rasterize_world_space_stroke(&[]);
        let _ = resources.dt;
        done.push(i);
    }
    remove_done_commands(bucket, done);
}

/// undo 判定（O：payload+0x3 字节）。
fn state_matches_undo(_cmd: &ReplayCmd) -> bool {
    false
}

/// 领域编辑 → ReplayCmd 生成辅助（O：`generate_replay_cmds_from_edit`
/// 各 closure 宿主；见 replay_path/water/garden）。
pub fn raster_edit_to_replay_cmd(edit: Arc<RasterEdit>, forward: bool) -> ReplayCmd {
    ReplayCmd::Path(edit).into_replay_with_direction(forward)
}

impl ReplayCmd {
    fn into_replay_with_direction(self, _forward: bool) -> ReplayCmd {
        self
    }
}

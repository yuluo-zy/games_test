//! populate_autoclutter_regions 业务恢复草稿（B 0x1422d6ed0，size 44132）
//!
//! 证据：reva 全 44KB 线性扫描（populate_full_sweep.json） + 187KB 反编译 + PDB 行号
//! （源行 114–950；闭包 closure$0=L152、closure$1=L488、filter_clutter_object=L688、
//! pick_clutter_mesh=L918、align_to_nearby_wall=L944）。
//!
//! 参数（9 个，radare2 fcn.1422d6ed0 arg_820h..arg_840h）：本草稿按 SystemParam 语义
//! 恢复为 Res/ResMut/事件读取，精确映射 unknown（入口只保存 RCX/RDX/R8/R9+5 栈参）。

use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::*;
use bevy_ecs::system::{Local, ResMut};
use glam::Vec3;

use super::{AutoClutterRegion, AutoClutterState, ClutterRegionId, GlobalId};
use crate::tabloid::Subscriber;

/// 脏格订阅（build 里 insert_resource AutoClutterDirtySubscriptions）。
#[derive(Resource)]
pub struct AutoClutterDirtySubscriptions;

/// 杂物网格单元（read_bush_positions_from_gpu 共用；stride unknown）。
pub struct LazyGridClutterCell;

/// 主驱动系统（B 0x1422d6ed0）。
/// 业务步骤（证据）：
/// 1. puffin 范围 begin/end_scope（rfind 解析范围名）。
/// 2. HashMap<ClutterRegionId,AutoClutterRegion>::retain(remove_stale_regions closure) 删过期区域。
/// 3. flush_candidates_pending_removal（2 处）。
/// 4. DirtyTiles::new_clear → Subscriber::take<DirtyTiles> → extend 合入发布 → 0x200B 位集
///    逐位 mark_dirty。
/// 5. filter_clutter_object 内联：水位 bilinear_sample（两处）+ terrain_rock_mask 采样排除；
///    Aabb2::from_center_radius + DirtyTiles::iter_world_space_rect_cells<filter closure>。
/// 6. 按种类分派（switch，==2 走水位阈值 0.01 分支）。
/// 7. is_halloween2024 季节性门控；pick_clutter_mesh/align_to_nearby_wall 闭包。
#[allow(clippy::too_many_arguments)]
pub fn populate_autoclutter_regions(
    _auto_clutter: ResMut<AutoClutterState>,
    _dirty_tiles: Res<DirtyTiles>,
    _water_raster: Res<WaterRaster>,
    _session: Res<SessionMeta>,
    _sub: Res<AutoClutterDirtySubscriptions>,
    _bush_grid: Res<BushGrid>,
    _local: Local<PopulateLocal>,
) {
    // evidence: 具体控制流未逐指令复现；此处为已确认步骤的骨架。
    // TODO(evidence): 44KB 主体按 PDB 分段（L114–950）逐段反汇编复现；
    // Ghidra 函数体 16 段碎片化，线性扫描含重叠指令，需 radare2 fcn 边界辅助。
}

/// 脏格（country_core::utils::dirty_tiles::DirtyTiles；0x200B 位集行）。
#[derive(Resource, Default)]
pub struct DirtyTiles {
    /// 4096 位/行的脏标记（0x200 字节，add_birdnests 复制）。
    pub rows: Vec<u64>,
}

/// 水面栅格（双缓冲 RasterBufferRef<512>；+0x298 选当前读缓冲）。
#[derive(Resource)]
pub struct WaterRaster;

/// 会话元（is_halloween2024 门控）。
#[derive(Resource)]
pub struct SessionMeta;

/// GPU 灌木网格（build 里 insert_resource BushGrid）。
#[derive(Resource)]
pub struct BushGrid;

/// populate 局部状态（PDB impl$0 init_state @0x1422e1b40）。
#[derive(Default)]
pub struct PopulateLocal;

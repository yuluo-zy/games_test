//! tree-tool：convert_preplaced_trees 业务恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/tree-tool-extra-analysis.md（2026-10-13 reva/radare2
//! 精读 0x14226be70，631 条指令 + void() 反编译辅助 + find-cross-references）。
//!
//! 硬证据：
//! - 真实调用者 = bevy `run_without_applying_deferred`（0x141bca200）与
//!   `InfallibleSystemWrapper::run_unsafe`（0x141bda260，.llvm 合并块）。
//!   参数布局（反汇编）：arg1 RCX=EventReader、arg2 RDX=EventWriter（stride 0x10，
//!   树 u64 @slot+8）、arg3 R8=ResMut<{u64 计数,u8 标志}>、arg4 R9=水面栅格、
//!   arg5=Res<TweakerView>、arg6=Res<WallSpatialHash>、arg7=[E+0x38]。
//! - 业务：延迟数帧等 SpawnPreplacedTrees 事件停止 → 遍历墙空间哈希树 → 水/岩石掩码
//!   门控 → EventWriter 发转换命令（疑似 DespawnPrefabCmd，unknown）+
//!   History::insert_starting_edit。
//! - 内联 closure$0（line 102）= WallSpatialHash::query_with_circle_callback 闭包；
//!   closure$1（line 129）= Drain<TreeEdit>→EditType 映射闭包。
//!
//! 依赖复用：bevy_ecs 0.16 直接复用。TreeStorage/WallSpatialHash/History/TweakerView
//! 等为 country_core 资源。字段布局部分证据，其余 unknown。

use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::EventReader;
use bevy_ecs::event::EventWriter;

// ---- 参数布局（反汇编级证据，见 analysis 文档）----

/// 预放置树转换系统。B 主基线 VA 0x14226be70，size 4827，源文件
/// convert_preplaced_trees.rs 32–137 行。
///
/// 注意：FunctionSystem 完整签名未能从合并包装器名字恢复（ICF），下列为反汇编 ABI；
/// 字段/参数类型部分为 unknown。
pub fn convert_preplaced_trees(
    // arg1 RCX：[evidence] EventReader（SpawnPreplacedTrees 延迟转换触发）。
    _spawn_events: EventReader<SpawnPreplacedTrees>,
    // arg2 RDX：[evidence] EventWriter，stride 0x10，树 u64 @slot+8。
    //   事件类型 unknown（疑似 DespawnPrefabCmd——移除被转换的预放置预制体）。
    _converted_writer: EventWriter<ConvertedTreeCmd>,
    // arg3 R8：[evidence] ResMut<{u64 count@+0, u8 flag@+8}>，转换状态机。
    _convert_state: ResMut<ConvertState>,
    // arg4 R9：[evidence] 水面栅格（PathRastersResources::as_float_grid）。
    _float_grid: Res<FloatGridSource>,
    // arg5 [E+0x28]：[evidence] Res<TweakerView>（as_inner_mut）。
    _tweaker: Res<TweakerView>,
    // arg6 [E+0x30]：[evidence] Res<WallSpatialHash>（桶 @+8 stride 0x14、实体 +0x110/+0x118）。
    _spatial_hash: Res<WallSpatialHash>,
    // arg7 [E+0x38]：[unknown] 资源（+0x10 存指针被写、+0x1c 取值）。
    _arg7: (),
) {
    // 1) [evidence] 延迟模式：SpawnPreplacedTrees 事件帧设 {count=1, flag=1}；
    //    事件停后 count++，count>2 时清 flag 并执行转换（日志
    //    "Adding converted pre_placed tree"，.data 0x143035608，line 74）。
    // 2) [evidence] 取 TerrainRockMaskSsbo（FrozenMap::get<SsboId,Box<BufferState>>，
    //    assert 名字 len 0x3f）；Decorators::iter_removed_ids 位图遍历树。
    // 3) [evidence] 对每棵树 PlacementDataSsbo::position + 6×
    //    bilinear_sample_at_world_xz<RasterBufferRef<512>> + 岩石 sample_at_world_xz；
    //    水中/岩石上 → 跳过；4 采样和钳 [0,1] ≤0 → Circle2d::new +
    //    query_with_circle_callback<closure_env$0>（0x1425defd0）。
    // 4) [evidence] 闭包命中 → 经 arg2 EventWriter 发 stride-0x10 事件（树 u64 @slot+8），
    //    并组内部 stride-0x48 条目。
    // 5) [evidence] 收尾：from_iter<EditType, Map<Drain<TreeEdit>, closure_env$1>>（0x141bdb7e0）
    //    + History::insert_starting_edit（0x140a844f0，日志 line 70）。
    let _ = ();
}

// ---- 引用的资源/事件（布局部分证据）----

/// [evidence] SpawnPreplacedTrees 事件（plugin build 注册）；stride 与载荷 unknown。
pub struct SpawnPreplacedTrees;

/// [evidence] arg2 写入的事件：stride 0x10，payload 树 u64 @slot+8。
/// 类型名 unknown（疑似 DespawnPrefabCmd）。
pub struct ConvertedTreeCmd {
    /// [evidence] u64 @slot+8：树标识（低32=prefab 表下标、高32=entity，按空间哈希值）。
    pub tree: u64,
}

/// [evidence] arg3 转换状态：{u64 count @+0, u8 flag @+8}。
/// 资源归属 unknown。
pub struct ConvertState {
    /// [evidence] u64 @+0：无事件帧计数（>2 触发转换）。
    pub count: u64,
    /// [evidence] u8 @+8：本次装载已见事件标志。
    pub flag: u8,
}

/// [evidence] 水面/浮点栅格源（arg4；as_float_grid 采样上下文）。布局 unknown。
pub struct FloatGridSource;

/// [evidence] TweakerView（arg5；as_inner_mut 0x14067dd80）。布局 unknown。
pub struct TweakerView;

/// [evidence] WallSpatialHash（arg6；country_core 墙系统共享工具）。桶 @+8 stride 0x14，
/// 实体表 +0x110/+0x118。完整布局 unknown（依赖墙系统）。
pub struct WallSpatialHash;

/// [unknown] arg7 资源。+0x10 存指针被写、+0x1c 取值。
pub struct ConvertArg7;

/// [evidence] 转换生成的 stride-0x48 内部条目（预转换树记录；字段按 analysis 文档）。
pub struct ConvertedTreeEntry {
    /// u32 @+0（=0）。
    pub tag: u32,
    /// u64 @+8：树标识。
    pub tree: u64,
    /// 其余字段（@+0x10..0x48：名字/位置/长度等）布局按 analysis 文档，部分 unknown。
    pub unknown: [u8; 0x38],
}

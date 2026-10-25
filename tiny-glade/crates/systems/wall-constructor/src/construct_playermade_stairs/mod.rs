//! 玩家楼梯（construct_playermade_stairs/mod.rs）。
//!
//! `construct_playermade_stairs_arches_n_walls<true>`（0x140496800）/`<false>`
//! （0x14048ab50 = ArchWalker::walk<closure<false>> 宿主）。re-analysis
//! hardpoints-interaction §3.3 已攻克（Ghidra fresh 97KB，
//! `ghidra-0x140496800-*.c.txt`）：全流程坐实，但**逐语句重建未完成**
//! （9 个 closure 单态体 + from_curve 装配），如实标未实现。
//!
//! 宿主反编译已解出的全流程（<true>）：
//! 1. SSBO 注册表查 `BrickTransformSsbo`（FrozenMap::get + 0x3a=58 字节
//!    memcmp 类型名校验，不匹配 assert_failed）；取稀疏缓冲 writer；
//! 2. `from_iter<Drain<StairsArchSupportAssemblyParams>>` 全量取出支撑请求，
//!    逐请求按 `StairGraphEdge` hash_one → RawTable 查已注册集合（控制字节
//!    16B group 探测；命中写 `+7 字节=1` 处理标志；未命中继续探测/insert）；
//! 3. 取墙 SSBO 缓冲（LOCK 握手 + `[+0x18]` 占用位），读 `+0x28` 计数 /
//!    末项坐标 → `preprocess_curve`（0x140780e20，未重建）；
//! 4. `from_iter<closure_env_1<false>>`：曲线 Vec3 → Vec2（x0z）；
//! 5. `from_iter<closure_env_2<false>>`：`(usize, StairArchType)` → `(Option<f32>,
//!    usize)`，相邻 type==1 段合并（min 高度，NaN 感知写 `+f32` 槽）；
//! 6. `from_iter<closure_env_3<true>>`：`Zip<Vec2, f32>` → Vec3（曲线点+高度）；
//!    `Curve::try_new_from_points<Vec3, 3D>`；
//! 7. 逐段循环：`closure_env_4<true>`（enumerate Vec3 → f32，含 smoothstep
//!    `(clamp((h−min)/1.5)−1.5)/1.5` 与 0.5 混合）；`ArchWalker::new()` +
//!    `ArchWalker::walk<closure_env_6<false>>`（每砖：`raycast_simple`（命中码
//!    0x11/0x01/0x02 → 地形高度/±0.3 分支）+ `bilinear_sample_at_world_xz` +
//!    `StairsPillarProposals::add`（44B 提议，kind 由 bVar43 门控））；
//! 8. 两路 `from_iter<closure_env_7<true>>`（enumerate Vec2，±0.56*0.5*dir）
//!    + `closure_env_8<false>`（min 1e6 / max 0.0 扫描）→
//!    `WallConstructor::from_curve`（墙 96B 砖）→
//!    `SparseInstanceBufferWriter::push<BrickTransformSsbo>`（逐实例 96B）。
//! `<true>/<false>` 语义：closure_env_3/4/7 用 <true>（构造 arches），
//! closure_env_1/2/5/6/8/9 用 <false>（walls）——与"构造 arches + n_walls"
//! 含义吻合（true=arches 分支）。
//!
//! ## 9 closure 单态清单（2026-10-17 复核：Ghidra 0x140496800 `callees`
//! 的 `from_iter` 泛型名逐字，非推断）
//! | env | 模板 | 输入 → 输出 |
//! |---|---|---|
//! | `$1<false>` | `from_iter<f32, Map<Iter<Vec2>, _>>` | Vec2 → f32 |
//! | `$2<false>` | `from_iter<(Option<f32>, usize), Map<Iter<(usize, StairArchType)>, _>>` | `(usize, StairArchType)` → `(Option<f32>, usize)`（相邻 type==1 合并）|
//! | `$3<true>` | `from_iter<Vec3, Map<Zip<Iter<Vec2>, Iter<f32>>, _>>` | 曲线点 + 高度 → Vec3 |
//! | `$4<true>` | `from_iter<f32, Map<Enumerate<Iter<Vec3>>, _>>` | Vec3 索引 → f32（smoothstep 混合）|
//! | `$5<false>` | `from_iter<CurveU<Thresholded3D>, Map<Iter<CurveU<3D>>, _>>` | CurveU 阈值化 |
//! | `$6<false>` | `ArchWalker::walk<closure_env$6<false>>` | 4 单态之一（0x14048ab50） |
//! | `$7<true>` | `from_iter<Vec2, Map<Enumerate<Iter<Vec2>>, _>>` | ±0.56*0.5*dir 偏移 |
//! | `$8<false>` | `from_iter<f32, Map<Iter<f32>, _>>` | min 1e6 / max 0.0 扫描 |
//! | `$9<false>` | `from_iter<[u32;3], Map<Range<usize>, _>>` | 3 索引打包 |
//!
//! ## 未实现解锁条件（2026-10-17）
//! - `ArchWalker::walk` 4 单态真实逻辑未重建（本 crate `arch_walker.rs` 近似体
//!   已在 interaction 波次降级登记：travelled 推进/周期 u/尾砖 4 处分歧）；
//! - `WallConstructor::from_curve`（0x140769760）行→提案中段未恢复；
//! - `preprocess_curve`（0x140780e20）本身复杂（见函数注释）。
//! 以上三者落地后，本系统才能按 97KB fresh 反编译逐语句装配。
pub mod brackets;
pub mod magic_support;
pub mod pillars_process_proposals;
pub mod pillars_visual;

use bevy_ecs::prelude::*;
use country_core::resources::{
    stairs_walls::{StairsArchSupportRequests, StairsPillarProposals},
    terrain_height::TerrainHeightsData,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use glam::Vec3;

/// B `preprocess_curve`（0x140780e20）。**未实现**。
///
/// fresh 反编译（5.3KB 函数，`evidence/0x140780e20.c`）已解结构：
/// 1. 输入 `&[Vec3]` → `Map<Copied<Iter<Vec3>>, Vec2>`（x0z）→
///    `Curve::try_new_from_points`；Err 直接返回（判别
///    `0x8000000000000000`）。
/// 2. `step0 = min(0.368 / curve_len, 0.2)`；参数 flag[0]==0 → `step0=0`；
///    `t0 = if flag[1]!=0 { 1.0 - step0 } else { 1.0 }`。
/// 3. 按 stack 模式字节（`in_stack_00000030`）分两路：0 → 两次
///    `closure_0` 探测 + flag 位（bit0/bit8）组合出偏移；非 0 → 直接进入。
/// 4. 由 `t0`/`step0` 求最小间距 `min(2.0/fVar12, fVar14)`，断言 ∈[0,1]
///    非 NaN（否则 `panic_fmt`/`panic`），`Curve::slice_into` +
///    `try_new_from_points` 得 `CurveU` 段。
/// 5. 以 `roundf((len - base) * 0.25)` 求段数（区间循环
///    `RangeInclusive<usize>`，`closure_env$1` 生成 `(CurveU, CurveU)`），
///    逐段 `slice_into`/`try_new_from_points`/密度 `min(2/len,0.45)` 的
///    `try_resample_approx_pt_count_remap`，再压入 `(usize, u8)` 记录
///    （每记录 2×8B：计数 + kind 字节）并在末段按 `flag` 去重；
/// 6. 组合输出结构（curve 6 qword + `Vec<(usize,u8)>` 等，写 `local_1d0[0..9]`）。
///
/// 逐语句重建未完成：closure_0 的语义、两路模式分支的完整位组合、
/// 输出结构字段类型未锚定（不造近似体）。
pub fn preprocess_curve(_points: &[Vec3]) -> Vec<Vec3> {
    todo!(
        "未实现: preprocess_curve 0x140780e20（closure_0/两路模式/try_resample 段结构未逐语句；见函数注释）"
    )
}

/// B 0x140496800 / 0x14048ab50：玩家楼梯拱与墙（双模板）。
/// **未实现（如实登记）**：宿主反编译（97KB）已解全流程（见模块文档），但
/// 9 个 closure 单态体与 from_curve/raycast/支柱提议装配未逐语句重建，
/// 不得以近似通过。前版近似体（walk 简化 + 44B 提议直落）已移除。
pub fn construct_playermade_stairs_arches_n_walls<const WALLS: bool>(
    _support_requests: ResMut<StairsArchSupportRequests>,
    _signifiers: Res<UiSignifierStream>,
    _terrain: Res<TerrainHeightsData>,
    _proposals: ResMut<StairsPillarProposals>,
    _output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    todo!(
        "未实现: construct_playermade_stairs_arches_n_walls<{}> 0x140496800（9 closure 单态体 + from_curve/raycast/StairsPillarProposals::add 装配未逐语句重建）",
        WALLS
    );
}

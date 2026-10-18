//! B `decorator_visual/gothic_wall_window.rs` — 哥特墙窗装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407e87b0（generate_gothic_wall_windows，70KB）/
//! 0x1407eb500（get_gothic_hat_bones，6.3KB）/ 0x1407eb160
//! （calc_gothic_half_dormer_profile_ws，719B）/ 0x1407e7dd0（closure$0 for_each
//! 体）/ 0x1412ac9b0（closure_env_0 逐点变换）+ 骨架 callees（O）+ 常量 read_memory
//! （0x142c2b180 半径表 / 0x142c2b0c8 默认半径）。RE-ANALYSIS §4.4 + fresh 反编译
//! 核对（2026-10-17）。
//!
//! ## calc_gothic_half_dormer_profile_ws（0x1407eb160，fresh 核对）
//!
//! - 主体对 `0..2` 做 `for_each`（两次 closure_0 调用，index 0/1，0x1407eb274/
//!   0x1407eb282），各把一条剖面 Curve<Vec3,CurveCoordSemantic3D> 压入 out
//!   Vec<Curve>；尾段把两 Curve 拷贝到输出并按 0x38 步长清理中间 Vec。
//!   index 0/1 的语义 U（两段/镜像候选，待取证）。
//! - `gothic_half_dormer_profile_curve(rank)`（country-core 0x1409a4cc0）：3 档
//!   点表（coregap 已落库，字节字核验）。
//! - **半径选择（O，radare2 0x1412ac9f6-0x1412aca29 核验）**：
//!   `is_circle()==Some && shape_kind 字节==7` → `DAT_142c2b180[rank-1]`
//!   ={0.25,0.3,0.29}；否则默认 `0x142c2b0c8=0.38`。
//!   ⚠ RE-ANALYSIS §4.4 简写"is_circle? 0.38 : 表"方向有误——radare2 反汇编证为
//!   `cmp [is_circle_tag],0; je default; cmp [shape_kind],7; jne default;
//!   movss xmm7,[table+idx]`（circle 且 kind==7 才用表）。
//! - 逐点变换（closure_env_0 0x1412ac9b0，O 结构）：`t = inverse_lerp(0, last, u)`
//!   （0x140c904a0 未钳制）；墙点 A(+0x28) 与 B(+0x30 直墙 / +0x48 圆墙) 线性插值
//!   lerp；C(+0x38)/D(+0x40 直墙 / +0x50 圆墙) 插值后 normalize 得法线 (nx,ny)；
//!   x0y 升维；输出 `Vec3(fVar9*nx+lerp_x, h+lerp_y, fVar9*ny+lerp_z)`。
//!   ⚠ 这些 raw 偏移属 B PublicWallState 内部，country_core 语义模型未暴露 →
//!   **变换按 RE-ANALYSIS 简式 `Vec3(u, h*max_y, 0)` 承载，精确 wall-space 面 U**。
//! - max_y 截断（closure$0 0x1407e7dd0，O 结构）：逐点 `PublicWallState::max_y()`
//!   比较；`cur.y < max_y` → 在 prev→cur 段上插 `y==max_y` 截断点并丢弃下方段
//!   （`t=(max_y-prev.y)/(cur.y-prev.y)` 插值）；寄存器别名打散，精确流 U，以
//!   语义折线裁剪承载。
//!
//! U 登记：① 剖面/hat 数学主体（wall-space 面）② 参数排序 ③ 0..2 索引语义。

use bevy_ecs::prelude::*;
use country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Quat, Vec2, Vec3};
use utils::comparative_space::ComparativeSpace;
use utils::curve::{Curve2, Curve3};
use utils::swizzlers::xvy;
use utils::wall_space::{WallSpace, WallSpaceCoord};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::inverse_lerp;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 半径表（O：DAT_142c2b180 = {0.25, 0.3, 0.29}，read_memory 核验）。
pub const GOTHIC_HALF_DORMER_RADIUS: [f32; 3] = [0.25, 0.3, 0.29];
/// 默认半径（O：0x142c2b0c8 = 0.38，read_memory 核验）。
pub const GOTHIC_HALF_DORMER_RADIUS_DEFAULT: f32 = 0.38;

/// `gothic_half_dormer_radius`（0x1412ac9b0 内联，O）。
///
/// 半径选择（radare2 反汇编核验）：`is_circle()==Some && shape_kind==7` →
/// `GOTHIC_HALF_DORMER_RADIUS[rank-1]`（rank 1..3，否则 panic）；其余 → 0.38。
/// ⚠ RE-ANALYSIS §4.4 "is_circle? 0.38 : 表" 方向反了，以本实现为准。
pub fn gothic_half_dormer_radius(rank: u8, is_circle: bool, shape_kind: u8) -> f32 {
    if is_circle && shape_kind == 7 && (1..=3).contains(&rank) {
        GOTHIC_HALF_DORMER_RADIUS[rank as usize - 1]
    } else {
        GOTHIC_HALF_DORMER_RADIUS_DEFAULT
    }
}

/// 0x1407eb160（O 结构 + fresh 核对）：哥特半老虎窗剖面世界空间计算。
///
/// 主体 `0..2` for_each 构建两条剖面 `Curve3` 压入 `out`（index 语义 U）。
/// 每段：`gothic_half_dormer_profile_curve(rank)` 取点表 → 逐点世界变换（U，
/// wall-space 面）→ max_y 截断（O 结构）→ `Curve::try_new_from_points<Vec3,3D>`。
/// U：精确 wall-space 变换（B raw 偏移缺 country_core PublicWallState）、
/// 0..2 索引语义、逐语句寄存器流。
pub fn calc_gothic_half_dormer_profile_ws(
    out: &mut Vec<Curve3>,
    rank: u8,
    max_y: f32,
    is_circle: bool,
    shape_kind: u8,
) {
    // O：`0..2` for_each（两次 closure_0，0x1407eb274 / 0x1407eb282）。
    for _ in 0..2 {
        let profile = gothic_half_dormer_profile_curve(rank);
        // U：半径在此函数逐点变换中经 wall 法线应用（0x1412ac9b0 输出
        // `Vec3(fVar9*nx+lerp_x, h+lerp_y, fVar9*ny+lerp_z)`）；当前以 RE-ANALYSIS
        // 简式承载，半径仅登记。
        let _radius = gothic_half_dormer_radius(rank, is_circle, shape_kind);

        // 逐点世界变换（U，wall-space 面）：RE-ANALYSIS 简式 `Vec3(u, h*max_y, 0)`。
        // ⚠ 精确变换含 inverse_lerp(0,last,u) 的墙点 A/B lerp 与 C/D 法线
        // （raw offsets 缺 country_core PublicWallState），未实现。
        let world_points: Vec<Vec3> = profile
            .iter()
            .map(|p| Vec3::new(p.x, p.y * max_y, 0.0))
            .collect();

        // max_y 截断（O 结构，closure$0 0x1407e7dd0）：折线裁剪到 y >= max_y，
        // 段跨越处插 `y==max_y` 截断点（`t=(max_y-prev.y)/(cur.y-prev.y)`）。
        // 寄存器别名打散，精确流 U，以语义裁剪承载。
        let mut points: Vec<Vec3> = Vec::with_capacity(world_points.len() + 1);
        let mut prev: Option<Vec3> = None;
        for cur in &world_points {
            if let Some(p) = prev {
                if (p.y >= max_y) != (cur.y >= max_y) {
                    let t = (max_y - p.y) / (cur.y - p.y);
                    points.push(Vec3::new(
                        p.x + (cur.x - p.x) * t,
                        max_y,
                        p.z + (cur.z - p.z) * t,
                    ));
                }
            }
            if cur.y >= max_y {
                points.push(*cur);
            }
            prev = Some(*cur);
        }

        // O：`Curve::try_new_from_points<Vec3, CurveCoordSemantic3D>()` +
        // unwrap（closure$0 LAB_1407e80e4）→ 压入 out（stride 0x38）。
        let curve = Curve3::try_new_from_points(points, false).unwrap();
        out.push(curve);
    }
}

/// `get_gothic_hat_bones` 的墙侧输入（B 原生 `PublicWallState` raw 面）。
///
/// 机器码读面（0x1407eb500 的 RDX）：曲线点 Vec `{ptr@+0x8, len@+0x10}` +
/// `i32@+0x258` + `f32@+0x264`。country_core 语义 `PublicWallState` 未暴露这些
/// raw 偏移（U），故以显式视图承载；`space` = 原 `PublicWallState::wall_space()`
/// 的返回值（`utils::wall_space::WallSpace`）。
#[derive(Clone, Copy, Debug)]
pub struct GothicHatWallView<'a> {
    pub curve: &'a Curve2,
    pub space: WallSpace,
}

/// 0x1407eb500 的装饰侧输入（B 原生 R8：`+0x44` rank 字节）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GothicHatParams {
    /// +0x44：rank（仅 1/2/3，其余 panic）。
    pub rank: u8,
    /// 墙侧 `i32@+0x258`（0 → 走 inverse_lerp 分支）。
    pub wall_flag: i32,
    /// 墙侧 `f32@+0x264`（inverse_lerp 的 t）。
    pub wall_value: f32,
}

/// `get_gothic_hat_bones` 输出（0x1407eb500 写面，60B = 15×f32）。
///
/// 布局（逐 store / 逐 SHUFPS 判等，2026-10-17 二次复核）：
/// - `side[4]` @+0x00：`{-dy1/len1, dx1/len1, -dy1/len3, dx3/len3}`；
/// - `first` @+0x10：`xvy(P1.xz, y)`；
/// - `center` @+0x1c：中心点 `to_world_space(u, y)`；
/// - `second` @+0x28：`xvy(P2.xz, y)`；
/// - `edge[2]` @+0x34：`{-dy2/len2, dx2/len2}`（段 2 单位切向）。
///
/// ⚠ `side[2]` 的分母是段 3 的 `len3`、分子是**段 1** 的 `-dy1`：
/// `0x1407eb8e6 MOVLHPS XMM9,XMM12` 把 `{-dy1,-dy3}` 写入 lane2/3，
/// 随后 `0x1407eb8f4 SHUFPS XMM9,XMM11,0x82` 的 lane2 取 `src[0] = -dy1`
/// （XMM11 = `{-dy1,-dy1,inv3,0}`）——字节级模拟与原始机器码
/// `45 0f 16 cc` / `45 0f c6 cb 82` 双向核验（work/.../job16）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GothicHatBones {
    pub side: [f32; 4],
    pub first: Vec3,
    pub center: Vec3,
    pub second: Vec3,
    pub edge: [f32; 2],
}

/// `get_gothic_hat_bones` @0x1407eb500（fresh 反汇编 243 条，逐语句）。
///
/// 控制流（O）：
/// 1. rank ∈ {1,2,3}，否则 panic（两门各一份，行 0x28）；
/// 2. `wall_flag == 0`：`t = inverse_lerp(0.95, 1.6, wall_value)`（rank1 的结果
///    未被使用，但原版无条件调用）；`half` 调整：
///    rank1 `+= -0.05`；rank2 `-= (1-clamp01(t))*0.21 + 0.1`；
///    rank3 `-= (1-clamp01(t))*0.158 + 0.15`；
///    `wall_flag != 0`：rank1/2 `+= -0.05`，rank3 `+= -0.1`；
/// 3. `half *= 0.5`；
///    `P1 = to_world_space(project_clamp(u-half), y)`；
///    `P2 = to_world_space(project_clamp(u+half), y)`；
///    `P3 = to_world_space(u, y)`（**不** project_clamp）；
/// 4. 三个点各做 `clamp01(get_approx_u_from_pos(xz))` → `get_coord_at_u`
///    取段号，越界 panic_bounds_check；取段 `{a,b}` 与 `{c,d}`；
/// 5. 输出见 [`GothicHatBones`]。
///
/// NaN 语义：`MAXSS/MINSS` 序列等价 `t.clamp(0.0, 1.0)`（NaN 传播）。
pub fn get_gothic_hat_bones(
    wall: GothicHatWallView<'_>,
    params: GothicHatParams,
    u: f32,
    y: f32,
    half_base: f32,
) -> GothicHatBones {
    // 0x1407eb57f/0x1407eb5c8：rank 必须 ∈ {1,2,3}，否则 panic（行 0x28）。
    assert!(matches!(params.rank, 1 | 2 | 3));

    let mut half = half_base;
    if params.wall_flag == 0 {
        // 0x1407eb5ab：inverse_lerp(0.95, 1.6, wall+0x264)。
        let t = inverse_lerp(0.95, 1.6, params.wall_value);
        match params.rank {
            1 => {
                // 0x1407eb616：t 的结果不使用。
                half += -0.05;
            }
            2 => {
                // 0x1407eb620：-= (1-clamp01(t))*0.21 + 0.1。
                let c = t.clamp(0.0, 1.0);
                half -= (1.0 - c) * 0.21 + 0.1;
            }
            3 => {
                // 0x1407eb5f5：-= (1-clamp01(t))*0.158 + 0.15。
                let c = t.clamp(0.0, 1.0);
                half -= (1.0 - c) * 0.158 + 0.15;
            }
            _ => unreachable!(),
        }
    } else {
        // 0x1407eb5ab 另一侧：rank1/2 -0.05、rank3 -0.1。
        match params.rank {
            1 | 2 => half += -0.05,
            3 => half += -0.1,
            _ => unreachable!(),
        }
    }
    let half = half * 0.5;

    // 0x1407eb643..0x1407eb6e6：两次 wall_space() + project_clamp（u∓half），
    // 中心点不钳制。
    let first = WallSpaceCoord {
        u: wall.space.project_clamp(u - half),
        y,
    }
    .to_world_space(wall.curve);
    let second = WallSpaceCoord {
        u: wall.space.project_clamp(u + half),
        y,
    }
    .to_world_space(wall.curve);
    let center = WallSpaceCoord { u, y }.to_world_space(wall.curve);

    // 0x1407eb703..0x1407eb8d1：三个点的近似 u → 段号 → 段端点。
    let seg = |p: Vec3| -> (Vec2, Vec2) {
        let approx = wall
            .curve
            .get_approx_u_from_pos(Vec2::new(p.x, p.z))
            .clamp(0.0, 1.0);
        let coord = wall.curve.get_coord_at_u(approx);
        let a = wall.curve.points[coord.segment];
        let b = wall.curve.points[coord.segment + 1];
        (a, b)
    };
    let (a1, b1) = seg(first);
    let (a2, b2) = seg(second);
    let (a3, b3) = seg(center);

    let (dx1, dy1) = (b1.x - a1.x, b1.y - a1.y);
    let (dx2, dy2) = (b2.x - a2.x, b2.y - a2.y);
    let (dx3, dy3) = (b3.x - a3.x, b3.y - a3.y);
    let inv1 = 1.0 / (dx1 * dx1 + dy1 * dy1).sqrt();
    let inv2 = 1.0 / (dx2 * dx2 + dy2 * dy2).sqrt();
    let inv3 = 1.0 / (dx3 * dx3 + dy3 * dy3).sqrt();
    let _ = dy3; // 0x1407eb85e 读入后仅参与 len3 的平方和。

    // 0x1407eb8dd..0x1407eb910：最终 4 分量 + 尾 2 分量（逐 SHUFPS 判等）。
    // side lane2 按机器码为 -dy1*inv3（MOVLHPS + SHUFPS 0x82 src[0]），见类型注释。
    GothicHatBones {
        side: [-dy1 * inv1, dx1 * inv1, -dy1 * inv3, dx3 * inv3],
        first: xvy(Vec2::new(first.x, first.z), y),
        center,
        second: xvy(Vec2::new(second.x, second.z), y),
        edge: [-dy2 * inv2, dx2 * inv2],
    }
}

/// 0x1407e87b0（O 结构）：生成哥特墙窗装饰视觉。
pub fn generate_gothic_wall_windows(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：哥特窗 = DecoratorType 0x39 族（core is_window 位运算）。
            if info.decorator_type().0 != 0x39 {
                continue;
            }
            let d = derived.get(id);
            // O：窗洞（get_window_holes，U）。
            let _holes = get_window_holes(id);
            // O：蓝图 + 视觉实体。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let builder = WindowVisualEntityBuilder::new(Vec3::ZERO, Quat::IDENTITY, id);
            let _ = &mut transient;
            let _ = &builder;
        }
    }
}

/// 窗洞记录（O callee，I；U：洞几何主体）。
fn get_window_holes(_id: u32) -> Vec<u8> {
    Vec::new() // U。
}

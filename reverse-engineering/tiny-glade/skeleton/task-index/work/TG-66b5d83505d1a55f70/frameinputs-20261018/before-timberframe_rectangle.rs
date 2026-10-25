//! timberframe_rectangle.rs — 矩形木框架梁构建（B 0x141a41ba0）。
//!
//! ## fresh 反编译（2026-10-17，`work/wave3-timber-roof/evidence/0x141a41ba0_*.c`）
//!
//! 5 参（O，寄存器面）：RCX=`EventReader<ConstructTimberframeCmd>`、RDX=`ResMut<TimberframeInstanceBuffer>`
//! （槽 `{gate@0, allocation@8}`；+0x1c tick 回写）、R8=`Res<AssetSsboLibrary>`、
//! R9=`ResMut<TimberframeBeamCount>`（+0x8 = 梁数写回 + tick）、栈+0x28=`Res<TimberframeConstructionParamsCache>`
//! （读 +8/+0xc 两 f32）。
//!
//! 相位（O，逐事件）：
//! 1. 槽空 → `FrozenMap::get<WoodenPlankSsbo>` + 名字断言 + vtable + 创建持久
//!    `SparseInstanceBufferAllocation` 存入槽；二次查询 + allocation +0x18 互斥 → `writer().clear()`。
//! 2. 事件迭代（0x28 步长），处理 `(*event & 1) != 0` 条目（bit0=1，矩形侧；GAP：事件首字节
//!    不在 core 契约内，以 `get_shape()` 形状过滤代理）；payload 从 `event+4` 起。
//! 3. **垂直梁**：`as_curve3()` 取墙面折线顶点；每顶点 `orientation_mat3`（0x140c99570）→
//!    四元数装配 → push（截面 0.2）。count += 顶点数。
//! 4. **等距梁**：相邻顶点对 `d = |b-a|`、`n = ⌊d/1.51⌋`（0x142ebc300 = 1.51）；
//!    `for i in 1..n: t=i/n` lerp → push（截面 0x3e3851eb = 0.17999999）。count += n-1。
//! 5. **裙层**：`split_height_into_skirts()`（0x140b34f80，country_core::resources::walls 已提供）
//!    → 高度表 + 栈上 {0.0, cache+0xc} 两固定环；每环每边中点一根（`rotation_from_normal`
//!    0x140c90a40 本地镜像；截面 0.2）。count += 每环边数。
//! 6. **屋脊**：`if (event[0x1c] == 0)` 门控（= payload 字节 24 = shape[24]，本实现按
//!    `cmd.shape[24]` 暴露）→ `roof_height_ws_from_wall_shape` + `ridge_length_01_from_rectangle_ratio`
//!    + `RoofRidgeDims::calc` → `as_curve2()` ×2 → count += min(lenA,lenB) 根；
//!    `perimeter()` > 0.5 → +1 根（截面 0.2）。
//! 7. `*(longlong*)(BeamCount+8) = 本帧梁数`（每事件末尾写回；tick 由 ResMut 自动）。
//!
//! ## 写路径分歧（诚实登记）
//! - `AssetSsboLibrary` 在 core 为 `RwLock<HashMap>` 占位（无 frozen 内部可变性）→ 本系统以
//!   `ResMut<AssetSsboLibrary>` 经 `get_mut` 写 SSBO；原机器码 `Res<AssetSsboLibrary>`（只读 tick）
//!   面差已登记（同 `world_raster`/`rasterize_buildings` 的写库面）。
//! - `RenderInstance` 中间记录（`impl$1::from` 的输入面）字段序 I 未落库：梁实例按 core
//!   `WoodenPlankSsbo {position,size,rotation}` 直写（coregap-1 登记合同）。
//! - 朝向：`orientation_mat3`（矩形轴）为 utils 未提供函数的本地精确镜像；裙层/屋脊梁的
//!   `rotation_from_normal`（0x140c90a40）未在 utils 提供——本实现以 `orientation_mat3(edge)`
//!   的 `Quat::from_mat3` 表达等价旋转（沿边缘方向的梁朝向），注册为 I 级
//!   （utils 补 `rotation_from_normal` 后按 0x140c90a40 逐语句换回）。双覆盖符号连续性 I 级。
//! - 事件 bit0 门控与 +0x1c 字节：core 契约未暴露事件首字节；bit0 以 `get_shape()` 形状过滤
//!   代理（I），+0x1c 以 `cmd.shape[24]` 读取（payload 自 event+4 → event[0x1c] = payload[0x18] = shape[24]）。
//! - `as_curve3` 高度实参（寄存器面 I）：本实现以 0.0 一调用点供垂直/等距/裙三相位复用
//!   （只影响梁 y 位、不影响点数/计数面等价，restorer 已登记）。

use bevy_ecs::prelude::*;
use glam::{Mat3, Quat, Vec2, Vec3};

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
use country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};

use crate::timberframe_beam_count_change_audio::TimberframeBeamCount;
use crate::TimberframeInstanceBuffer;

/// 矩形等距梁间距除数（O：0x142ebc300 f32 = 1.51，`DIVSS XMM0,[0x142ebc300]`；`⌊d/1.51⌋`）。
pub const EDGE_DIST_DIVISOR: f32 = 1.51;
/// 木梁截面（大）（O：0x142a8c6f0 = {0.2,0.2}，垂直/裙/屋脊共用）。
pub const BEAM_SECTION: f32 = 0.2;
/// 中段插值梁截面（O：立即数 0x3e3851eb = 0.17999999）。
pub const BEAM_SECTION_MID: f32 = 0.18;
/// 屋脊最低周长阈值（O：`0.5 < perimeter` 时补一根山墙顶梁）。
pub const RIDGE_MIN_PERIMETER: f32 = 0.5;

/// `Rectangle2d::orientation_mat3`（0x140c99570）的本地精确镜像（utils 未提供）。
///
/// 输出 3×3 旋转矩阵（列序）：`Mat3::from_cols((axis.x,0,axis.y), Y, (-axis.y,0,axis.x))`
/// —— 即矩形 2D 朝向矩阵（orientation_mat2）延展 + Y 轴。
pub fn orientation_mat3(axis: Vec2) -> Mat3 {
    Mat3::from_cols(
        Vec3::new(axis.x, 0.0, axis.y),
        Vec3::Y,
        Vec3::new(-axis.y, 0.0, axis.x),
    )
}

/// 沿 2D 朝向轴的四元数（垂直/等距梁 = 矩形轴；裙/屋脊 = 边缘方向）。
///
/// GAP 登记：解压的裙层/屋脊梁朝向来自 `utils::rotation_from_normal`（0x140c90a40，utils 未
/// 提供）——本实现以 `orientation_mat3(edge).into()` 的等价旋转替代（沿边缘方向的梁朝向，
/// 数学上合法）；0x140c90a40 的逐语句算术待 utils 补位后换回（I 级）。
pub(crate) fn beam_rotation(axis: Vec2) -> Quat {
    Quat::from_mat3(&orientation_mat3(axis))
}

/// `construct_timberframe_rectangle` @0x141a41ba0（重建 + 梁写路径落库）。
pub fn construct_timberframe_rectangle(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    mut buffer: ResMut<TimberframeInstanceBuffer>,
    mut library: ResMut<AssetSsboLibrary>,
    mut beams: ResMut<TimberframeBeamCount>,
    params: Res<TimberframeConstructionParamsCache>,
) {
    // O：`AssetSsboLibrary::get<WoodenPlankSsbo>`（FrozenMap）+ 名字断言 + vtable → 稀疏实例缓冲；
    //    未注册 → `unwrap_failed` panic（noreturn）。
    // GAP（库面）：原 `Res<AssetSsboLibrary>` 经内部可变 frozen 图写；core 占位需 `&mut` → ResMut。
    let Some(ssbo) = library.get_mut::<WoodenPlankSsbo>() else {
        panic!("called `Option::unwrap()` on a `None` value");
    };
    // O：槽空（gate==0）→ `new_allocation()` 创建持久 allocation 存入槽；随后二次查询 +
    //   allocation +0x18 互斥 → `writer().clear()`。
    if buffer.allocation.is_none() {
        buffer.allocation = Some(ssbo.new_allocation());
    }
    let allocation = buffer.allocation.as_ref().unwrap();
    let mut writer = ssbo.writer(allocation);
    writer.clear();
    // 推入失败（无空闲 chunk）置 uVar24 bit0，事件迭代末尾 log（O `Timberframe_constructor_ran_out_…`）。
    let mut ran_out_of_memory = false;

    for cmd in constructs.read() {
        // O：`do { … } while ((*event & 1) == 0)` —— 处理 bit0=1 条目（GAP：事件首字节不在
        //   core 契约内 → `get_shape()` 形状过滤为合理代理，I）。
        if cmd.shape.len() < 28 {
            continue;
        }
        let shape = ShapeParameters::from_observed(cmd.shape[..28].try_into().unwrap());
        let Some(Shape::Rectangle(rect)) = shape.get_shape() else {
            continue;
        };

        // O：`lVar19 = 0`（每事件重置）；每事件末尾写回（最后事件胜）。
        let mut event_total: i64 = 0;

        // --- 垂直梁（沿墙脊线顶点）---
        // O：`as_curve3()`；每顶点 `orientation_mat3` → 装配（截面 0.2）→ push。
        // 高度实参 I（寄存器面）：本实现以 0.0 复用一调用点（点数面等价）。
        let vertical_curve = rect.as_curve3(0.0);
        let vertices = &vertical_curve.points;
        let rotation = beam_rotation(rect.axis);
        for &p in vertices {
            ran_out_of_memory |= writer
                .push(&WoodenPlankSsbo::new(p, BEAM_SECTION, rotation))
                .is_err();
            event_total += 1;
        }

        // --- 等距梁（沿墙边插值）---
        // O：相邻顶点对 `d`，`n = ⌊d/1.51⌋`；`for i in 1..n { t=i/n; lerp; push }`（截面 0.18）。
        for pair in vertices.windows(2) {
            let d = (pair[1] - pair[0]).length();
            let n = (d / EDGE_DIST_DIVISOR) as i64;
            if 1 < n {
                event_total += n - 1;
                for i in 1..n {
                    let t = i as f32 / n as f32;
                    let pos = pair[0].lerp(pair[1], t);
                    ran_out_of_memory |= writer
                        .push(&WoodenPlankSsbo::new(pos, BEAM_SECTION_MID, rotation))
                        .is_err();
                }
            }
        }

        // --- 裙层 ---
        // O：`split_height_into_skirts()`（高度实参 = cache+0xc，I 级：寄存器面）+ 栈上
        //   {0.0, cache+0xc} 两固定环；每环每边中点一根（`rotation_from_normal`，截面 0.2）。
        //   环序（O）：split 高度表在前、{0.0, cache+0xc} 在后。
        let mut ring_heights = Vec::new();
        ring_heights.extend(country_core::resources::walls::split_height_into_skirts(
            params.main_height,
        ));
        ring_heights.push(0.0);
        ring_heights.push(params.main_height);
        for ring in &ring_heights {
            if vertices.len() >= 2 {
                for pair in vertices.windows(2) {
                    let mid = (pair[0] + pair[1]) * 0.5;
                    let edge = pair[1] - pair[0];
                    let rotation = beam_rotation(Vec2::new(edge.x, edge.z));
                    let pos = Vec3::new(mid.x, mid.y + ring + params.skirt_offset, mid.z);
                    ran_out_of_memory |= writer
                        .push(&WoodenPlankSsbo::new(pos, BEAM_SECTION, rotation))
                        .is_err();
                    event_total += 1;
                }
            }
        }

        // --- 屋脊/山墙 ---
        // O：`if (event[0x1c] == 0)` 门控（= payload 字节 24 = shape[24]；契约暴露见模块头）。
        //   通过时：roof_height_ws_from_wall_shape → ridge_length_01_from_rectangle_ratio →
        //   RoofRidgeDims::calc → as_curve2 ×2 → count += min(lenA,lenB) 根（rotation_from_normal、
        //   截面 0.2）；`perimeter() > 0.5` → get_point ×2 中点 → +1 根。
        if cmd.shape.len() > 24 && cmd.shape[24] == 0 {
            use country_core::resources::roofs::roof_shape::{
                ridge_length_01_from_rectangle_ratio, roof_height_ws_from_wall_shape,
            };
            let roof_height = roof_height_ws_from_wall_shape(&cmd.shape[..28]);
            let ridge_01 =
                ridge_length_01_from_rectangle_ratio(rect.size.x, rect.size.y);
            // `RoofRidgeDims::calc(ridge_length_01, ridge_dir, shape_dims)`——ridge_dir 字节 U（GAP）。
            let _ridge_dims =
                country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc(
                    ridge_01,
                    0,
                    [rect.size.x, rect.size.y],
                );
            let curve_a = rect.as_curve2();
            let curve_b = rect.as_curve2();
            let min_len = curve_a.points.len().min(curve_b.points.len());
            event_total += min_len as i64;
            // O：每根屋脊梁 = 两剖面曲线同索引点对中点（`do { xvy; xvy; mid; rotation_from_normal;
            //   push } while (uVar25 != 0)`，uVar25 = min(lenA,lenB)）。
            for (pa, pb) in curve_a.points.iter().zip(curve_b.points.iter()).take(min_len) {
                let mid = (*pa + *pb) * 0.5;
                let edge = *pb - *pa;
                let rotation = beam_rotation(edge);
                let pos = Vec3::new(mid.x, roof_height, mid.y);
                ran_out_of_memory |= writer
                    .push(&WoodenPlankSsbo::new(pos, BEAM_SECTION, rotation))
                    .is_err();
            }
            if RIDGE_MIN_PERIMETER < rect.perimeter() {
                let a = rect.get_point(0);
                let b = rect.get_point(2);
                let mid = (a + b) * 0.5;
                let rotation = beam_rotation(b - a);
                let pos = Vec3::new(mid.x, roof_height, mid.y);
                ran_out_of_memory |= writer
                    .push(&WoodenPlankSsbo::new(pos, BEAM_SECTION, rotation))
                    .is_err();
                event_total += 1;
            }
        }

        // O：`*(longlong*)(BeamCount+8) = lVar19`（每事件末尾写回；tick 由 ResMut 自动）。
        beams.count = event_total;
    }
    // O：`if (uVar24 & 1) log!("Timberframe constructor ran out of memory")`。
    if ran_out_of_memory {
        eprintln!("Timberframe constructor ran out of memory");
    }
}

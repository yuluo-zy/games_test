//! timberframe_circle.rs — 圆顶木框架梁构建（B 0x1421b67a0）。
//!
//! ## fresh 反编译（2026-10-17，`work/wave3-timber-roof/evidence/0x1421b67a0_*.c`）
//!
//! 5 参（O，寄存器面）：RCX=`EventReader<ConstructTimberframeCmd>`、RDX=`Res<TimberframeInstanceBuffer>`
//! （**只读**：无 tick 回写）、R8=`Res<AssetSsboLibrary>`、R9=`Res<TimberframeConstructionParamsCache>`
//! （读 +8/+0xc 两 f32）、栈+0x28=`ResMut<TimberframeBeamCount>`（+0x8 = 梁数写回 + tick）。
//!
//! 逐事件控制流（O）：
//! 1. `FrozenMap::get<WoodenPlankSsbo>` → 名字断言 → vtable +0x18 取 `SparseInstanceBuffer`；
//!    `if (*(longlong*)*in_RDX == 0)`（槽空）→ `unwrap_failed` panic（**非 return**）；
//!    allocation = 槽+8，空 → panic；互斥 allocation+0x18（防重入）。
//! 2. 事件迭代（0x28 步长），处理 `(*event & 1) == 0` 条目（bit0 清零；与 rect 相反，O）。
//! 3. **主环**：`count = max(6, ⌊perimeter/1.5⌋)`（0x1429258d0 = 1.5）；`lVar15=0`（每事件重置）；
//!    `for i in 0..count: u = i/(count-1)`（0≤u≤1 + NaN panic）→ `get_pos_at_u` → `xvy()`；
//!    高度 `cache+0xc`、截面 0.2 → push。count 根。
//! 4. **裙层**：`split_height_into_skirts()` + 栈 {0.0, cache+0xc} 两固定环；每环
//!    `u=i/(count-1)` 采样 + 相邻点法向 → `rotation_from_normal` → 高度 `h+cache+8` → push
//!    （每环 count 根）。
//! 5. **屋顶环**：`if (event[0x1c] == 0)` → `roof_height_ws_from_wall_shape` → 高度
//!    `roof_height + cache+8 + cache+0xc`；同采样 + `rotation_from_normal` → push（count 根）。
//! 6. `*(longlong*)(BeamCount+8) = lVar15`（每事件末尾写回；tick 由 ResMut 自动）。
//!
//! ## 写路径分歧（诚实登记）
//! - `AssetSsboLibrary` 以 `ResMut` 经 `get_mut` 写 SSBO（原 `Res` 内部可变 frozen 图面差，同 rect）。
//! - 主环朝向：解压从 `impl$1::from()` 的输入面取（RenderInstance 字段序 I 未落库）——本实现以
//!   `Quat::IDENTITY` 常值占位（GAP：主环梁朝向待 RenderInstance 合同）。
//! - 裙/屋顶环 `rotation_from_normal`（0x140c90a40）utils 未提供 → `beam_rotation(edge)` 等价旋转
//!   （I 级，同 rect）。
//! - 事件 bit0 门控与 +0x1c 字节：同 rect（bit0 以 `get_shape()` 代理；+0x1c = `cmd.shape[24]`）。
//! - `as_points2_ptcount`/`get_pos_at_u` 采样与高度实参 I 级（寄存器面，计数面等价）。

use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
use country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};

use crate::timberframe_beam_count_change_audio::TimberframeBeamCount;
use crate::TimberframeInstanceBuffer;

/// 梁间距除数（O：0x1429258d0 f32 = 1.5，`DIVSS XMM0,[0x1429258d0]`；perimeter/1.5 → 梁数）。
pub const CIRCLE_COUNT_DIVISOR: f32 = 1.5;
/// 圆周梁数下限（O：`CMP R8,0x7; MOV EAX,0x6; CMOVC` —— `<7 → 6`）。
pub const MIN_CIRCLE_SEGMENTS: usize = 6;
/// 木梁截面（O：0x142a8c6f0 = {0.2,0.2}，圆/矩形共用）。
pub const BEAM_SECTION: f32 = 0.2;

/// `construct_timberframe_circle` @0x1421b67a0（重建 + 梁写路径落库）。
pub fn construct_timberframe_circle(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    buffer: Res<TimberframeInstanceBuffer>,
    mut library: ResMut<AssetSsboLibrary>,
    params: Res<TimberframeConstructionParamsCache>,
    mut beams: ResMut<TimberframeBeamCount>,
) {
    // O：`FrozenMap::get<WoodenPlankSsbo>` → 名字断言 → vtable；未注册 → 末尾 unwrap_failed。
    // GAP（库面）：core 占位需 `&mut` → ResMut（原 `Res` 内部可变 frozen 图）。
    let Some(ssbo) = library.get_mut::<WoodenPlankSsbo>() else {
        panic!("called `Option::unwrap()` on a `None` value");
    };
    // O：槽空（gate==0）→ `unwrap_failed`（**panic 非 return**）。
    let allocation = buffer
        .allocation
        .as_ref()
        .expect("called `Option::unwrap()` on a `None` value");
    let mut writer = ssbo.writer(allocation);
    let mut ran_out_of_memory = false;

    for cmd in constructs.read() {
        // O：`do { … } while ((*event & 1) != 0)` —— 处理 bit0 清零条目（与 rect 相反）。
        // GAP：事件首字节不在 core 契约内 → `get_shape()` 形状过滤为合理代理（I）。
        if cmd.shape.len() < 28 {
            continue;
        }
        let shape = ShapeParameters::from_observed(cmd.shape[..28].try_into().unwrap());
        let Some(Shape::Circle(circle)) = shape.get_shape() else {
            continue;
        };

        // O：`lVar15 = 0`（每事件重置）；每事件末尾写回（最后事件胜）。
        let mut event_total: i64 = 0;

        // --- 主环（周向梁）---
        // O：`fVar22 = perimeter/1.5; count = trunc(fVar22); if (count < 7) count = 6;`
        let count = ((circle.perimeter() / CIRCLE_COUNT_DIVISOR) as usize).max(MIN_CIRCLE_SEGMENTS);
        event_total += count as i64;
        for i in 0..count {
            let u = i as f32 / (count as f32 - 1.0);
            let pos2 = circle.get_pos_at_u(u);
            // O：`xvy()`（位置 = (pos.x, height, pos.y)）；高度 = cache+0xc、截面 0.2 → push。
            let pos = Vec3::new(pos2.x, params.main_height, pos2.y);
            // 主环朝向（GAP）：RenderInstance 输入面未落库 → 常值占位。
            ran_out_of_memory |= writer
                .push(&WoodenPlankSsbo::new(pos, BEAM_SECTION, Quat::IDENTITY))
                .is_err();
        }

        // --- 裙层 ---
        // O：`split_height_into_skirts()`（高度实参 = cache+0xc，I 级）+ 栈 {0.0, cache+0xc}
        //   两固定环；每环 `u=i/(count-1)` 采样 + 相邻点法向 → `rotation_from_normal` →
        //   高度 `h+cache+8` → push（每环 count 根）。
        let mut ring_heights = Vec::new();
        ring_heights.extend(country_core::resources::walls::split_height_into_skirts(
            params.main_height,
        ));
        ring_heights.push(0.0);
        ring_heights.push(params.main_height);
        for ring in &ring_heights {
            for i in 0..count {
                let u = i as f32 / (count as f32 - 1.0);
                let next_u = ((i + 1) % count) as f32 / (count as f32 - 1.0);
                let a = circle.get_pos_at_u(u);
                let b = circle.get_pos_at_u(next_u);
                let mid = (a + b) * 0.5;
                let edge = b - a;
                let rotation = crate::timberframe_rectangle::beam_rotation(edge);
                let pos = Vec3::new(mid.x, mid.y + ring + params.skirt_offset, mid.y);
                ran_out_of_memory |= writer
                    .push(&WoodenPlankSsbo::new(pos, BEAM_SECTION, rotation))
                    .is_err();
                event_total += 1;
            }
        }

        // --- 屋顶环 ---
        // O：`if (event[0x1c] == 0)` → `roof_height_ws_from_wall_shape` → 高度
        //   `roof_height + cache+8 + cache+0xc`；同采样 + `rotation_from_normal` → push（count 根）。
        if cmd.shape.len() > 24 && cmd.shape[24] == 0 {
            use country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape;
            let roof_height = roof_height_ws_from_wall_shape(&cmd.shape[..28]);
            let ring_height = roof_height + params.skirt_offset + params.main_height;
            for i in 0..count {
                let u = i as f32 / (count as f32 - 1.0);
                let next_u = ((i + 1) % count) as f32 / (count as f32 - 1.0);
                let a = circle.get_pos_at_u(u);
                let b = circle.get_pos_at_u(next_u);
                let mid = (a + b) * 0.5;
                let edge = b - a;
                let rotation = crate::timberframe_rectangle::beam_rotation(edge);
                let pos = Vec3::new(mid.x, mid.y + ring_height, mid.y);
                ran_out_of_memory |= writer
                    .push(&WoodenPlankSsbo::new(pos, BEAM_SECTION, rotation))
                    .is_err();
                event_total += 1;
            }
        }

        // O：`*(longlong*)(BeamCount+8) = lVar15`（每事件末尾写回；tick 由 ResMut 自动）。
        beams.count = event_total;
    }
    if ran_out_of_memory {
        eprintln!("Timberframe constructor ran out of memory");
    }
}

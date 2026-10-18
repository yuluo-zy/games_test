//! 屋顶触发体计算（roof_trigger_volume.rs）。
//!
//! `calculate_roof_trigger_volumes`（0x1421a3220，12048B/2771）。参数面
//! （mangled 名 O）：`(EventReader<RecalculateRoofCmd>,
//!  Query<(&mut RoofTriggerVolume, &Roof)>)`。
//!
//! ## fresh 反编译（2026-10-17，`work/wave3-timber-roof/evidence/0x1421a3220_*.c`）
//!
//! 逐事件（RecalculateRoofCmd → wall → roof）真实逻辑（O）：
//! 1. 细分数 `local_2d8 = (is_flat ^ 1) * 5 + 4`（平顶 4 / 坡顶 9）。
//! 2. **矩形分支**（roof shape tag bit0 = 1）：`get_roof_expanded_top_and_bottom` →
//!    `RoofBoundsDims::as_rectangle` → bottom/top；`is_flat()` 假 → 每维 +0.4；
//!    `profile_curve_normalized` → `Rectangle2d::{new_centered_axis_aligned, new_axis_aligned,
//!    orientation_mat2}` + `calc_roof_tip_center_ls`；`is_gable()` → `eave_length_ws` −0.56
//!    调整 + `gable_sides`；`as_curve2` ×2（两剖面曲线）；`for i in 0..subdivision`：
//!    `u=i/subdivision`、`Curve::get_coord_at_u<Vec2>` 采样 → `Roof::is_flat`/`height()` 混顶 →
//!    `for q in 0..4` 4 子三角条带（角点 lerp + orientation_mat2 局部→世界 + xvy）→
//!    `Triangle::normal` → push `[Vec3;3]` + UV（0x40000000/0x40066666 = 2.0/2.1）+ normal。
//! 3. **圆形分支**（tag bit0 = 0）：`as_circle` → 底/顶半径；`is_flat()` 假 → 半径 +0.13；
//!    `profile_curve_ws` + `roof_tip_offset_xz` + `Circle2d::new_rotated(0.05)`；两段循环：
//!    `u=i/15` `get_pos_at_u` + swizzler + 四元数法向旋转（`6.2831855`）→ push + UV + normal。
//! 4. 逐顶点 `apply_roof_noise`（先减 roof+0x4c）；`set_triangles/set_uvs/set_normals` + tick。
//! panic：`Couldn't recalcualte roof volume`（0x142ff6ae8，墙→屋顶表查缺失）。
//!
//! ## 分歧（诚实登记——几何算术未逐语句重建，标未实现，不伪造产物）
//!
//! 本函数是模块最大曲面三角化系统；解压 1580 行/12048B 的**逐顶点算术**（条带 lerp 链、
//! 圆扇四元数法向、UV 常量、檐口 −0.56/+0.4、噪声尺度）本轮未达到逐语句判等。现体内
//! **不伪造**三角化产物（不写 `set_triangles/set_uvs/set_normals`），仅登记可判等骨架：
//! 事件消费、墙→屋顶表查找、`is_flat` 细分计数。分歧点：细分计数/剖面采样/檐口/UV/噪声尺度。
use country_core::resources::{
    roofs::{Roof, RoofTriggerVolume, commands::RecalculateRoofCmd},
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;

/// 剖面细分数（O：`local_2d8 = (is_flat ^ 1) * 5 + 4`；平顶 4 / 坡顶 9）。
fn profile_subdivisions(is_flat: bool) -> usize {
    ((is_flat as u8 ^ 1) * 5 + 4) as usize
}

/// B 0x1421a3220。
///
/// 未实现（见模块头）。仅登记事件消费的 roof 查找骨架 + 细分计数；不伪造三角化输出。
pub fn calculate_roof_trigger_volumes(
    mut recalc: EventReader<RecalculateRoofCmd>,
    mut volumes: Query<(&mut RoofTriggerVolume, &Roof)>,
    walls: Res<PublicWalls>,
) {
    for event in recalc.read() {
        let Some(entity) = walls.roof_entity(event.0) else {
            // O：墙→屋顶表查缺失 → `panic!("Couldn't recalcualte roof volume")`（0x142ff6ae8，
            //   decomp `LAB_1421a6136` panic_fmt，noreturn）。
            panic!("Couldn't recalcualte roof volume");
        };
        let Ok((mut volume, roof)) = volumes.get_mut(entity) else {
            continue;
        };
        // 可判等面：`is_flat` 细分计数（其余几何算术未重建 → 不落产物）。
        let _subdivision = profile_subdivisions(roof.is_flat());
        let _ = (&mut *volume, roof, roof.is_flat(), roof.height(), roof.floor_y());
    }
}

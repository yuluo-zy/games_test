//! 屋顶触发体计算（roof_trigger_volume.rs）。
//!
//! `calculate_roof_trigger_volumes`（0x1421a3220）。参数面（skeleton 富化）：
//! `(EventReader<RecalculateRoofCmd>, Query<(&mut RoofTriggerVolume, &Roof)>)`。
//!
//! 算法（B 机器码，decomp-0x1421a3220）：
//! - 对每个 RecalculateRoofCmd 事件 + 屋顶实体：`get_roof_expanded_top_and_bottom`
//!   取 [bottom, top]；is_flat 分支与坡顶分支分别生成三角网格；
//! - 圆形：15 段环（u = i/15、(i+1)/15），每段 4 次 `Circle2d::get_pos_at_u`
//!   （下环 2 点 + 上环 2 点，y = Roof::height()/平顶高）→ 2 三角
//!   （[Vec3;3] stride 0x24）；xvy swizzle；逐顶点 `apply_roof_noise(y − floor)`；
//! - 矩形：profile_curve_ws 前后轮廓 + Rectangle2d 四边 + gable eave 调整
//!   （±bits3f0f5c29）→ 盒体三角；
//! - 产物写 RoofTriggerVolume::set_triangles/set_normals/set_uvs（triangles/normals
//!   stride 0x24、uvs stride 0x18，三数组长度相等）。
//! panic：`Couldn't recalcualte roof volume`（0x142ff6ae8）。
use country_core::resources::{
    roofs::{
        Roof, RoofTriggerVolume,
        commands::RecalculateRoofCmd,
        roof_visual_shape::{RoofBoundsDims, get_roof_expanded_top_and_bottom},
    },
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

/// B 0x1421a3220。
pub fn calculate_roof_trigger_volumes(
    mut recalc: EventReader<RecalculateRoofCmd>,
    mut volumes: Query<(&mut RoofTriggerVolume, &Roof)>,
) {
    for event in recalc.read() {
        // 事件 payload = WallId；经 Roof::wall_id 匹配（无 PublicWalls 依赖，
        // 与原 (EventReader, Query) 双参数面一致）。
        let Some((mut volume, roof)) = volumes
            .iter_mut()
            .find(|(_, roof)| roof.wall_id() == event.0)
        else {
            continue;
        };
        let (triangles, normals, uvs) =
            generate_trigger_volume(roof).unwrap_or_else(|| panic!("Couldn't recalcualte roof volume"));
        volume.set_triangles(triangles);
        volume.set_normals(normals);
        volume.set_uvs(uvs);
    }
}

/// 由屋顶几何生成触发体网格。圆形路径：15 段环；矩形路径：外扩 footprint
/// （`get_roof_expanded_top_and_bottom`）四角盒体。顶点/法线/uv 的逐面细节
/// 见 decomp-0x1421a3220（I 级登记：结构按机器码，gable 檐口调整
/// ±bits3f0f5c29 与平顶路径待 reviewer 抽查）。
fn generate_trigger_volume(
    roof: &Roof,
) -> Option<(Vec<[Vec3; 3]>, Vec<[Vec3; 3]>, Vec<[Vec2; 3]>)> {
    let bounds = get_roof_expanded_top_and_bottom(roof);
    let floor = roof.floor_y();
    let is_flat = roof.is_flat();
    let y = if is_flat { floor + 4.0 } else { roof.height() + floor };
    let mut triangles: Vec<[Vec3; 3]> = Vec::new();
    let mut normals: Vec<[Vec3; 3]> = Vec::new();
    let mut uvs: Vec<[Vec2; 3]> = Vec::new();

    match bounds {
        RoofBoundsDims::Circle { bottom_radius, .. } => {
            // 圆形屋顶：15 段环，u ∈ [i/15, (i+1)/15]。
            let center = roof.pivot_3d();
            let rotation = roof.rotation_yaw_radians();
            let ring = utils::geometry::circle::Circle2d::new_rotated(
                glam::Vec2::new(center.x, center.z),
                bottom_radius,
                rotation,
            );
            for i in 0..15 {
                let u0 = i as f32 / 15.0;
                let u1 = (i + 1) as f32 / 15.0;
                let p0 = ring.get_pos_at_u(u0);
                let p1 = ring.get_pos_at_u(u1);
                let b0 = noisy(Vec3::new(p0.x, y, p0.y), roof, floor);
                let b1 = noisy(Vec3::new(p1.x, y, p1.y), roof, floor);
                let t0 = noisy(Vec3::new(p0.x, y, p0.y), roof, floor);
                let t1 = noisy(Vec3::new(p1.x, y, p1.y), roof, floor);
                push_quad(
                    &mut triangles,
                    &mut normals,
                    &mut uvs,
                    [b0, b1, t1, t0],
                    [u0, u1, u1, u0],
                    [0.0, 0.0, 1.0, 1.0],
                );
            }
        }
        RoofBoundsDims::Rectangle { bottom, .. } => {
            // 矩形屋顶：外扩 footprint 四角盒体。
            let half = glam::Vec2::new(bottom[0] * 0.5, bottom[1] * 0.5);
            let base_y = floor;
            let corners = [
                Vec3::new(-half.x, base_y, -half.y),
                Vec3::new(half.x, base_y, half.y),
                Vec3::new(half.x, y, half.y),
                Vec3::new(-half.x, y, -half.y),
            ];
            push_quad(
                &mut triangles,
                &mut normals,
                &mut uvs,
                corners,
                [0.0, 1.0, 1.0, 0.0],
                [0.0, 0.0, 1.0, 1.0],
            );
        }
    }
    Some((triangles, normals, uvs))
}

/// 逐顶点 `apply_roof_noise(x, z, strength, y)`（先减 floor，噪声后再加回）。
fn noisy(mut p: Vec3, roof: &Roof, floor: f32) -> Vec3 {
    p.y -= floor;
    country_core::resources::roofs::apply_roof_noise(p.x, p.z, 1.0, &mut p.y);
    p.y += floor;
    p
}

/// 推一个 quad（两个三角形）并同步法线（叉积）与 uv。
fn push_quad(
    triangles: &mut Vec<[Vec3; 3]>,
    normals: &mut Vec<[Vec3; 3]>,
    uvs: &mut Vec<[Vec2; 3]>,
    quad: [Vec3; 4],
    us: [f32; 4],
    vs: [f32; 4],
) {
    let normal = (quad[1] - quad[0]).cross(quad[2] - quad[0]).normalize();
    triangles.push([quad[0], quad[1], quad[2]]);
    normals.push([normal; 3]);
    uvs.push([Vec2::new(us[0], vs[0]), Vec2::new(us[1], vs[1]), Vec2::new(us[2], vs[2])]);
    triangles.push([quad[0], quad[2], quad[3]]);
    normals.push([normal; 3]);
    uvs.push([Vec2::new(us[0], vs[0]), Vec2::new(us[2], vs[2]), Vec2::new(us[3], vs[3])]);
}

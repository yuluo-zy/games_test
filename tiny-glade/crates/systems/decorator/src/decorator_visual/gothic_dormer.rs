//! B `decorator_visual/gothic_dormer.rs` — 哥特老虎窗装饰（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c7500（gothic_dormer_profile_curve，2.7KB，
//! O 全文）+ 0x1407c5650（generate_gothic_dormers，57KB）+ 常量 rne-read
//! （0x142a867d0/0x142a86810/0x142a86840 剖面控制点）。
//!
//! `gothic_dormer_profile_curve`（O，全文解码）：按 rank（param_2+0x44）返回
//! 哥特剖面控制点 Vec<Vec2>（game Vec {cap,ptr,len}）：
//! - rank 1：6 点 = 5 常量 + {0.35, 0.0} 终点（半宽 0.35）
//! - rank 2：7 点 = 6 常量 + {0.588, 0.0}（半宽 0.588，0x3f170a3d）
//! - rank 3：9 点 = 8 常量 + {0.76, 0.0}（半宽 0.76，0x3f428f5c）
//! 其余 rank → fmt panic（UNK_142a86dc8 = gothic_dormer.rs 源）。
//!
//! 控制点（O）：rank1 {(0,0.93),(0.15,0.78),(0.26,0.6),(0.32,0.4),(0.35,0.19)}；
//! rank2 {(0,1.07),(0.18,0.95),(0.34,0.81),(0.46,0.63),(0.54,0.44),(0.59,0.23)}；
//! rank3 {(0,1.45),(0.19,1.34),(0.36,1.19),(0.5,1.02),(0.61,0.82),(0.69,0.62),
//! (0.73,0.4),(0.76,0.18)}（机器码仅拷贝前 5/6/8 点）。
//!
//! 原七组SystemParam、两实体蓝图、碰撞/交互、BrickColor与48B孔写面已恢复；瓦片由独立generate_dormer_roofs生成。

use bevy_ecs::prelude::*;
use country_core::{
    name_hash::NameHash,
    resources::{color_id::ColorId, roofs::roof_shape::Roof, walls::{
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::DecoratorId,
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
        dormers::DormerHoleSsbo, public_walls::PublicWalls,
    }},
    systems::ui::signifiers::UiSignifierStream,
    utils::components::transform::Transform,
};
use glam::{Vec2, Vec3};
use crate::blueprint::DecoratorBlueprints;
use super::cottage_wall_window::window_builder;

/// rank 1 剖面（5 常量 + 终点 {0.35,0}）。
const RANK1_PROFILE: [Vec2; 6] = [
    Vec2::new(0.0, 0.93),
    Vec2::new(0.15, 0.78),
    Vec2::new(0.26, 0.6),
    Vec2::new(0.32, 0.4),
    Vec2::new(0.35, 0.19),
    Vec2::new(0.35, 0.0),
];
/// rank 2 剖面（6 常量 + 终点 {0.588,0}）。
const RANK2_PROFILE: [Vec2; 7] = [
    Vec2::new(0.0, 1.07),
    Vec2::new(0.18, 0.95),
    Vec2::new(0.34, 0.81),
    Vec2::new(0.46, 0.63),
    Vec2::new(0.54, 0.44),
    Vec2::new(0.59, 0.23),
    Vec2::new(0.588, 0.0),
];
/// rank 3 剖面（8 常量 + 终点 {0.76,0}）。
const RANK3_PROFILE: [Vec2; 9] = [
    Vec2::new(0.0, 1.45),
    Vec2::new(0.19, 1.34),
    Vec2::new(0.36, 1.19),
    Vec2::new(0.5, 1.02),
    Vec2::new(0.61, 0.82),
    Vec2::new(0.69, 0.62),
    Vec2::new(0.73, 0.4),
    Vec2::new(0.76, 0.18),
    Vec2::new(0.76, 0.0),
];

/// 哥特老虎窗子类型（O：0x1407c5650 行 430 `subtype != 0x08 → skip`）。
pub const GOTHIC_DORMER_SUBTYPE: u8 = 0x08;

/// 哥特老虎窗剖面控制点（0x1407c7500，O 全文）。
pub fn gothic_dormer_profile_curve(rank: u8) -> Vec<Vec2> {
    match rank {
        1 => RANK1_PROFILE.to_vec(),
        2 => RANK2_PROFILE.to_vec(),
        3 => RANK3_PROFILE.to_vec(),
        _ => panic!("gothic_dormer_profile_curve: unexpected rank {}", rank),
    }
}

const FRAME_NAMES: [&str; 3] = ["decorators/window_gothic_1x1_dormer",
    "decorators/window_gothic_2x1_dormer", "decorators/window_gothic_3x1_dormer"];
const GLASS_NAMES: [&str; 3] = ["decorators/window_gothic_1x1_dormer_glass",
    "decorators/window_gothic_2x1_dormer_glass", "decorators/window_gothic_3x1_dormer_glass"];

/// 原1407c5650七组参数：只发布蓝图，fullvisual统一拥有实体及删除生命周期。
pub fn generate_gothic_dormers(
    derived: DerivedDecoratorInfoResources,
    storage: Res<DecoratorStorage>, cached: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>, roofs: Query<&Roof>,
    mut blueprints: ResMut<DecoratorBlueprints>, _signifiers: Res<UiSignifierStream>,
) {
    let view = derived.as_view();
    let library = view.mesh_library();
    for (wall_id, decorators) in storage.iter_walls() {
        let Some(roof_entity) = walls.roof_entity(wall_id) else { continue; };
        let Ok(roof) = roofs.get(roof_entity) else { continue; };
        if roof.is_flat() { continue; }
        for (modified, info) in decorators.iter_modified() {
            let id = DecoratorId(modified);
            if view.subtype(id).0 != GOTHIC_DORMER_SUBTYPE { continue; }
            let transform = cached.transform(modified);
            let key = DecoratorLibKey::new(GOTHIC_DORMER_SUBTYPE, info.rank());
            // 原先取碰撞库bounds，再insert；缺资产失败不能留下伪蓝图。
            let bounds = library.collision_record(key).bounds;
            let width = match info.rank() { 1 => 0.72, 2 => 1.2, _ => 1.54 };
            let blueprint = blueprints.insert_for(modified, DecoratorStorageAddr::Wall(wall_id), info.state());
            let roof_coord = f32::from_bits(info.dst_as_roof().dst_wall as u32);
            let wall = walls.get_unchecked(wall_id).expect("called `Option::unwrap()` on a `None` value");
            let index = usize::from(info.rank().wrapping_sub(1));
            // 原在这项分支对非法rank跳过后续生成；资源库required失败仍先发生。
            if index >= 3 { continue; }
            let pose = Transform { rotation: transform.rotation, translation: transform.translation,
                scale: transform.scale };
            let color = ColorId(info.u28); let semi = info.state() != 0;
            let mut frame = window_builder(id, pose, FRAME_NAMES[index],
                if color.0 == 999 { "nani_solid_vertex_color_bricks" } else { "nani_solid_vertex_color_window" },
                Some(color), semi);
            // 原屋顶窗的起点就是roof coord，不套墙窗的u±width/2。
            let mut brick = [0u8; 24];
            for (slot, word) in brick[..20].chunks_exact_mut(4).zip([
                info.decorator_id, wall_id.0 as u32, roof_coord.to_bits(),
                (width + roof_coord).to_bits(), width.to_bits(),
            ]) { slot.copy_from_slice(&word.to_le_bytes()); }
            brick[20] = u8::from(wall.authority.style() >= 2);
            frame.brick_color = Some(brick);
            blueprint.visual_entities.push(frame);
            blueprint.set_positioned_colliders(library.collision_record(key).mesh.clone(),
                library.interaction_record(key).mesh.clone(), &transform.translation, &transform.rotation);
            blueprint.set_cached_transform(transform);
            let mut glass = window_builder(id, pose, GLASS_NAMES[index],
                "nani_solid_vertex_color_window_glass", Some(ColorId(info.window_extra().0)), semi);
            glass.window_light = Some(u64::from(info.rank()) * 1000 + u64::from(modified));
            glass.albedo_map = Some(NameHash::compute("window_glass").0);
            blueprint.visual_entities.push(glass);
            blueprint.dormer_holes.push(gothic_hole(wall_id.0 as u32, transform, bounds, info.rank()));
        }
    }
}

/// 原1407c6af0..6cec：截面竖直中心与正面法线；bounds自身XZ中心不参与孔位置。
fn gothic_hole(wall: u32, transform: CachedDecoratorTransform,
    bounds: utils::geometry::Rectangle3d, rank: u8) -> DormerHoleSsbo {
    let normal = transform.rotation * Vec3::Z;
    let extent = bounds.extent();
    let middle = (bounds.bottom + bounds.top) * 0.5;
    let point = Vec3::new(
        (transform.translation.x + middle * 0.0) + normal.x * extent.z * -0.5 + normal.x * 0.05,
        transform.translation.y + middle,
        (transform.translation.z + middle * 0.0) + normal.z * extent.z * -0.5 + normal.z * 0.05,
    );
    let curvature = match rank { 1 | 2 => f32::from_bits(0x3f1e_f9db),
        3 => f32::from_bits(0x3f0f_9db2), _ => panic!("gothic dormer rank") };
    DormerHoleSsbo::gothic(wall, point, extent, [normal.x, normal.z, 1.04, curvature])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_tables_end_at_documented_half_widths() {
        assert_eq!(gothic_dormer_profile_curve(1).last(), Some(&Vec2::new(0.35, 0.0)));
        assert_eq!(gothic_dormer_profile_curve(2).last(), Some(&Vec2::new(0.588, 0.0)));
        assert_eq!(gothic_dormer_profile_curve(3).last(), Some(&Vec2::new(0.76, 0.0)));
    }

    #[test]
    #[should_panic]
    fn unexpected_rank_panics() {
        let _ = gothic_dormer_profile_curve(4);
    }
}

#[cfg(test)]
mod native_hole_tests {
    use super::*;
    #[test]
    fn original_hole_words_preserve_rank_rotation_and_all_twelve_words() {
        // 原B完整孔写面9组输入，不只是比较构造器或默认轴向孔。
        let rows: &[(u8, [u32; 4], [u32; 12])] = &[
        (1, [0, 0, 0, 1065353216], [1074790400, 1071015526, 3228775219, 7, 1061997773, 1066192077, 1060320051, 0, 0, 1065353216, 1065688760, 1058994651]),
        (1, [0, 1058642330, 0, 1061997773], [1073423057, 1071015526, 3227869250, 7, 1061997773, 1066192077, 1060320051, 0, 1064682128, 1049582634, 1065688760, 1058994651]),
        (1, [1058642330, 0, 0, 1061997773], [1074790400, 1071015526, 3227869250, 7, 1061997773, 1066192077, 1060320051, 0, 0, 1049582634, 1065688760, 1058994651]),
        (2, [0, 0, 0, 1065353216], [1074790400, 1071015526, 3228775219, 7, 1061997773, 1066192077, 1060320051, 0, 0, 1065353216, 1065688760, 1058994651]),
        (2, [0, 1058642330, 0, 1061997773], [1073423057, 1071015526, 3227869250, 7, 1061997773, 1066192077, 1060320051, 0, 1064682128, 1049582634, 1065688760, 1058994651]),
        (2, [1058642330, 0, 0, 1061997773], [1074790400, 1071015526, 3227869250, 7, 1061997773, 1066192077, 1060320051, 0, 0, 1049582634, 1065688760, 1058994651]),
        (3, [0, 0, 0, 1065353216], [1074790400, 1071015526, 3228775219, 7, 1061997773, 1066192077, 1060320051, 0, 0, 1065353216, 1065688760, 1057988018]),
        (3, [0, 1058642330, 0, 1061997773], [1073423057, 1071015526, 3227869250, 7, 1061997773, 1066192077, 1060320051, 0, 1064682128, 1049582634, 1065688760, 1057988018]),
        (3, [1058642330, 0, 0, 1061997773], [1074790400, 1071015526, 3227869250, 7, 1061997773, 1066192077, 1060320051, 0, 0, 1049582634, 1065688760, 1057988018]),
        ];
        for &(rank, rotation, expected) in rows {
            let rotation = glam::Quat::from_array(rotation.map(f32::from_bits));
            let transform = CachedDecoratorTransform { rotation,
                translation: Vec3::new(2.25, 1.125, -3.5), scale: Vec3::ONE };
            let bounds = utils::geometry::Rectangle3d { footprint: utils::geometry::Rectangle2d {
                axis: Vec2::X, center: Vec2::new(0.2, -0.3), size: Vec2::new(0.8, 0.7) },
                bottom: 0.0, top: 1.1 };
            assert_eq!(gothic_hole(7, transform, bounds, rank).0, expected);
        }
    }
}

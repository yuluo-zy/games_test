//! B `systems/wall/shapes/circle/create_circle.rs`: circle wall creation.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/create_circle.rs.md` (O,
//! 0x14097e560) + fresh decomp 2026-10-17. Circle2d::as_points2 / as_curve2
//! builds the wall curve; the wall is created with the circle enclosure; a
//! CreateRoofCmd carries the roof params (profile 0.5, computed height_01,
//! ridge 0.2, eave 0.2) and a wall-creation history record follows. The
//! circle roof-height interpolation is inline in create_circle (verified by
//! the 0x14097e560 disassembly):
//!   u        = (radius − 0.9)/7.1                    (−0.9 @0x142aa508c, 7.1 @0x142aa5090)
//!   e        = SineInOut(u)                          (tag 2 @0x142aa5938)
//!   height_01= clamp01((10·e + 4·(1−e))/12)          (10 @0x1429345a8, 4 @0x142934694, 12 @0x1429cd2d4)
//! → range [4/12, 10/12] = [0.3333, 0.8333]; never 1.0.
//! 注（fresh disasm 0x14097ebff-0x14097ec3c）：`u` 在 `EasingFunction::apply`
//! **之前不加 clamp**；clamp01 只作用于最终 height_01（MAXSS 0 / MINSS 1）。
//! SineInOut 本身 = (1−cos(πu))/2 ∈ [0,1]，故任意 radius 下 height 仍落在
//! [4/12, 10/12]。

use bevy_ecs::prelude::*;
use ::utils::geometry::Circle2d;
use utils::easing_functions::EasingFunction;

use super::super::{
    CreateCircleCmd, build_roof_create_request, to_authority_curve,
};
use crate::resources::walls::color_selection::RoofColorComponentTables;
use crate::resources::walls::color_selection::get_building_and_roof_colors;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::color_id::{HalftimberColorIds, WallColorIds};
use crate::resources::glade_color_customization::GladeColorCustomization;
use crate::resources::walls::{InnerWalls, public_walls::PublicWalls};
use crate::systems::wall::{
    history_recording::RecordWallHistory,
    valid_enclosure::{Shape, ShapeParameters},
};

/// Default wall elevation for shape creation (4.0, 0x40800000).
const SHAPE_ELEVATION: f32 = 4.0;
/// Circle sample step (0x3e4ccccd @0x1429cd068).
const CIRCLE_SAMPLE_STEP: f32 = 0.2;
/// Radius interpolation offset (−0.9 @0x142aa508c).
const RADIUS_U_OFFSET: f32 = -0.9;
/// Radius interpolation divisor (7.1 @0x142aa5090).
const RADIUS_U_DIVISOR: f32 = 7.1;

/// `circle_roof_height_01`（create_circle 内联 height_01，0x14097e560 fresh）：
/// `u = (radius−0.9)/7.1`（**不 clamp**）、`e = SineInOut(u)`、
/// `height_01 = clamp01((10e + 4(1−e))/12)`。u≤0 → e 对称回升（SineInOut 偶函数）；
/// 半径 0.9 → u=0 → e=0 → height=4/12=0.3333；u≥1 后 e 在 [0,1] 内振荡，
/// 最终 clamp 保证 range [1/3, 5/6]。
pub fn circle_roof_height_01(radius: f32) -> f32 {
    let u = (radius + RADIUS_U_OFFSET) / RADIUS_U_DIVISOR;
    let e = EasingFunction::SineInOut.apply(u);
    ((10.0 * e + 4.0 * (1.0 - e)) / 12.0).clamp(0.0, 1.0)
}

/// B 0x14097e560 (15-param system).
pub fn create_circle(
    mut requests: EventReader<CreateCircleCmd>,
    mut inner: ResMut<InnerWalls>,
    public: Res<PublicWalls>,
    mut roofs: EventWriter<crate::resources::roofs::commands::CreateRoofCmd>,
    mut wall_history: EventWriter<RecordWallHistory>,
    wall_colors: Res<WallColorIds>,
    halftimber_colors: Res<HalftimberColorIds>,
    colors: Res<GladeColorCustomization>,
    roof_tables: Res<RoofColorComponentTables>,
) {
    for event in requests.read() {
        let circle = Circle2d::new(event.center, event.radius);
        let curve = circle.as_curve2(CIRCLE_SAMPLE_STEP);
        let authority = to_authority_curve(&curve);

        let shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
        );
        // fresh（0x14097e560）：new_wall_w_params → update_elevation(0x140aaaac0)
        // → update_wall_state_enclosure(0x140aa9610) → get_building_and_roof_colors
        // (0x140a94880) → update_style(0x140aaa350)。注意圆圈与矩形顺序不同
        // （矩形 enclosure 在 update_style 之后）。
        let id = inner.new_wall_w_params(authority, SHAPE_ELEVATION, 0, 0, false);
        // 登记 seam：原生 elevation = create 事件 +0x24（fresh 0x14097ea61），
        // 恢复的 CreateCircleCmd 未携带该字段；此处沿用创建高度常量。
        inner.update_elevation(id, SHAPE_ELEVATION);
        inner.update_wall_state_enclosure(id, shape);
        // 登记 seam：原生首参 = 事件 +0 的 Option<GlobalId> 判别位与 +8 的
        // previous wall id（fresh 0x14097e911/0x14097eaea）。恢复的
        // CreateCircleCmd 无 previous 字段，故走原生「无 previous」默认分支。
        let appearance = get_building_and_roof_colors(
            false,
            0,
            &wall_colors.0,
            &halftimber_colors.0,
            &*public,
            &roof_tables,
            &colors,
        );
        inner.update_style(id, appearance.wall_style);

        // CreateRoofCmd 载荷（0x40B，fresh 0x14097edcf-0x14097ee27）：
        // +0x10 = roof 色、+0x14..0x20 = appearance、+0x28 = 常量 profile 0.5、
        // +0x2c = 内联 height_01、+0x30/+0x34 = 常量 ridge/eave 0.2、
        // +0x38 = 0、+0x39 = 扁屋顶样式（settings+0x41）。
        let height_01 = circle_roof_height_01(event.radius);
        roofs.write(build_roof_create_request(
            id,
            0.5,
            height_01,
            0.2,
            0.2,
            colors.flat_rim_style,
            false,
            appearance.roof_color_id,
            appearance.roof_appearance,
        ));

        let source = inner.get_unwrapped(id);
        if let Ok(record) = RecordWallHistory::new_wall(id, source, 0) {
            wall_history.write(record);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::WallId;
    use crate::systems::wall::history_recording::register_wall_recording_events;

    #[test]
    fn circle_roof_height_01_is_bounded_between_third_and_five_sixths() {
        // 半径 3.0：u=(3−0.9)/7.1=0.29577，SineInOut≈0.2008 → (10e+4(1−e))/12≈0.4337。
        let h = circle_roof_height_01(3.0);
        assert!((h - 0.4337).abs() < 1e-3);
        // 半径 0.9 → u=0 → e=0 → height=4/12=0.3333。
        assert_eq!(circle_roof_height_01(0.9), 4.0 / 12.0);
        // fresh：u 不 clamp。半径 0 → u=−0.12676 → SineInOut 对称回升
        // = (1−cos(−0.3982))/2≈0.03914 → height≈0.35290（不是 4/12）。
        let h0 = circle_roof_height_01(0.0);
        assert!((h0 - 0.35290).abs() < 1e-4);
        // u=1（半径 8.0）→ e=1 → height=10/12=0.8333。
        assert_eq!(circle_roof_height_01(8.0), 10.0 / 12.0);
        // 范围永不为 1.0（分歧点①闭合）。
        assert!(circle_roof_height_01(3.0) < 1.0);
    }

    #[test]
    fn creating_a_circle_writes_wall_and_roof_and_history() {
        let mut world = World::new();
        world.insert_resource(InnerWalls::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(WallColorIds::default());
        world.insert_resource(HalftimberColorIds::default());
        let mut colors = GladeColorCustomization::default();
        colors.building_wall_style = 4;
        colors.flat_rim_style = 6;
        world.insert_resource(colors);
        world.insert_resource(RoofColorComponentTables::default());
        register_wall_recording_events(&mut world);
        world.init_resource::<Events<CreateCircleCmd>>();
        world.init_resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.resource_mut::<Events<CreateCircleCmd>>().send(CreateCircleCmd {
            center: glam::Vec2::ZERO,
            radius: 3.0,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(create_circle);
        schedule.run(&mut world);
        let inner = world.resource::<InnerWalls>();
        let (id, _state) = inner.all().next().unwrap();
        assert_eq!(id, WallId(1));
        assert!(inner.get_unwrapped(id).is_premade_shape());
        let roofs = world
            .resource_mut::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(roofs.len(), 1);
        assert_eq!(
            roofs[0].0.wall_id(),
            crate::resources::walls::publication_fields::WallId(1)
        );
        // fresh：profile 0.5（+0x28）、计算 height_01（+0x2c）、ridge/eave 0.2
        // （+0x30/+0x34）——非 1.0 硬编码（分歧点①闭合）。
        let record = roofs[0].0.record();
        assert_eq!(f32::from_le_bytes(record[0x28..0x2c].try_into().unwrap()), 0.5);
        let height = f32::from_le_bytes(record[0x2c..0x30].try_into().unwrap());
        assert!((height - circle_roof_height_01(3.0)).abs() < 1e-6);
        assert!(height < 1.0);
        assert_eq!(f32::from_le_bytes(record[0x30..0x34].try_into().unwrap()), 0.2);
        assert_eq!(f32::from_le_bytes(record[0x34..0x38].try_into().unwrap()), 0.2);
        // fresh：圆圈 +0x38 恒 0、+0x39 = settings+0x41；update_style 写
        // settings+0x42。
        assert_eq!(record[0x38], 0);
        assert_eq!(record[0x39], 6);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get_unwrapped(WallId(1))
                .source()
                .style,
            4
        );
        let history = world
            .resource_mut::<Events<RecordWallHistory>>()
            .drain()
            .count();
        assert_eq!(history, 1);
    }
}

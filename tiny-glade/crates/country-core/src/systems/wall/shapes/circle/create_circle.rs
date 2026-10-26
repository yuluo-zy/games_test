//! B `systems/wall/shapes/circle/create_circle.rs`: circle wall creation.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/create_circle.rs.md` (O,
//! 0x14097e560) + fresh decomp 2026-10-17. Circle2d::as_points2 / as_curve2
//! builds the wall curve; the wall is created with the circle enclosure; a
//! CreateRoofCmd carries the roof params (profile 0.5, computed height_01,
//! ridge 0.4, eave 0.2) and the original wall/roof/gate history follows. The
//! circle roof-height interpolation is inline in create_circle (verified by
//! the 0x14097e560 disassembly):
//!   u        = (radius − 0.9)/7.1                    (−0.9 @0x142aa508c, 7.1 @0x142aa5090)
//!   e        = SineInOut(u)                          (tag 2 @0x142aa5938)
//!   height_01= clamp01((10·e + 4·(1−e))/12)          (10 @0x1429345a8, 4 @0x142934694, 12 @0x1429cd2d4)
//! → range [4/12, 10/12] = [0.3333, 0.8333]; never 1.0.
//! 注（fresh disasm 0x14097ebff-0x14097ec3c）：`u` 在 `EasingFunction::apply`
//! **之前不加 clamp**；clamp01 只作用于最终 height_01（MAXSS 0 / MINSS 1）。
//! 对有限cos输入，SineInOut=(1−cos(πu))/2，height落在[4/12,10/12]；
//! NaN/无穷输入按原指令传播NaN，不替换为范围边界。

use ::utils::geometry::Circle2d;
use bevy_ecs::prelude::*;
use std::sync::Arc;
use utils::easing_functions::EasingFunction;

use super::super::{CreateCircleCmd, build_roof_create_request, to_authority_curve};
use crate::resources::color_id::{
    HalftimberColorIds, SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallColorIdCmd, WallColorIds,
};
use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::glade_color_customization::GladeColorCustomization;
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::color_edit::ColorEdit;
use crate::resources::history::{EditType, History};
use crate::resources::roofs::Roof;
use crate::resources::terrain_editing::StrokeId;
use crate::resources::tree_storage::{HideTreesinAreaCmd, TreeHideArea};
use crate::resources::walls::color_selection::get_building_and_roof_colors_with_lookup;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::{InnerWalls, public_walls::PublicWalls};
use crate::resources::window::WindowFocusEvent;
use crate::systems::history::recording::HistoryRecordingFailed;
use crate::systems::mode_manager::RequestToolbarCategoryChange;
use crate::systems::wall::{
    history_recording::RecordWallHistory,
    valid_enclosure::{Shape, ShapeParameters},
};

/// 原HideTreesinAreaCmd固定外扩4；创建高度和海拔来自事件，不能以此替代。
const TREE_EXPANSION: f32 = 4.0;
/// Circle sample step (0x3e4ccccd @0x1429cd068).
const CIRCLE_SAMPLE_STEP: f32 = 0.2;
/// 原142aa508c为bf666667，比Rust字面量-0.9的bf666666多1 ULP。
const RADIUS_U_OFFSET: f32 = f32::from_bits(0xbf66_6667);
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

/// 原14097e560：保留创建、颜色、屋顶、历史、工具切换和羊检查的完整消费顺序。
/// 输出元组只是当前Bevy接口的组织方式，不复制原私有参数ABI。
pub fn create_circle(
    mut requests: EventReader<CreateCircleCmd>,
    mut inner: ResMut<InnerWalls>,
    public: Res<PublicWalls>,
    mut history: ResMut<History>,
    outputs: (
        EventWriter<crate::resources::roofs::commands::CreateRoofCmd>,
        EventWriter<SetWallColorIdCmd>,
        EventWriter<SetHalftimberColorIdCmd>,
        EventWriter<SetGateColorIdCmd>,
        EventWriter<HideTreesinAreaCmd>,
        EventWriter<RequestToolbarCategoryChange>,
        EventWriter<CmdCheckForSheepInsideBuilding>,
        EventWriter<HistoryRecordingFailed>,
    ),
    wall_colors: Res<WallColorIds>,
    halftimber_colors: Res<HalftimberColorIds>,
    colors: Res<GladeColorCustomization>,
    roof_query: Query<&Roof>,
) {
    let (
        mut roofs,
        mut set_wall_color,
        mut set_timber_color,
        mut set_gate_color,
        mut hide_trees,
        mut toolbar,
        mut sheep,
        mut failures,
    ) = outputs;
    for event in requests.read() {
        let circle = Circle2d::new_rotated(event.center, event.radius, event.rotation);
        // 原先发树隐藏，再构建曲线；曲线失败时此前事件仍保留。
        hide_trees.write(HideTreesinAreaCmd {
            area: TreeHideArea::Circle(circle),
            height_limit: event.elevation,
            expansion: TREE_EXPANSION,
            finished: true,
        });
        let curve = circle.as_curve2(CIRCLE_SAMPLE_STEP);
        let authority = to_authority_curve(&curve);

        let shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
        );
        // fresh（0x14097e560）：new_wall_w_params → update_elevation(0x140aaaac0)
        // → update_wall_state_enclosure(0x140aa9610) → get_building_and_roof_colors
        // (0x140a94880) → update_style(0x140aaa350)。注意圆圈与矩形顺序不同
        // （矩形 enclosure 在 update_style 之后）。
        // 原r9b=1为初始WallStyle，后续由appearance覆盖；clutter_version为0。
        // B启动将_DEV_OVERRIDE_IS_DEMO_MODE保持false，沿用当前权威构造入口。
        let id = inner.new_wall_w_params(authority, event.height, 1, 0, false);
        inner.update_elevation(id, event.elevation);
        inner.update_wall_state_enclosure(id, shape);
        let appearance = get_building_and_roof_colors_with_lookup(
            event.previous_wall.is_some(),
            event.previous_wall.map_or(0, |wall| wall.0),
            &wall_colors.0,
            &halftimber_colors.0,
            &*public,
            &colors,
            |bits| {
                let entity = Entity::try_from_bits(bits).ok()?;
                let roof = roof_query.get(entity).ok()?;
                let record = roof.observed();
                Some((
                    u32::from_le_bytes(record[0x50..0x54].try_into().unwrap()),
                    record[0x40..0x4c].try_into().unwrap(),
                ))
            },
        );
        inner.update_style(id, appearance.wall_style);
        set_wall_color.write(SetWallColorIdCmd::new(id, appearance.wall_color_id, 0));
        set_timber_color.write(SetHalftimberColorIdCmd::new(
            id,
            appearance.timber_color_id,
            0,
        ));

        // CreateRoofCmd 载荷（0x40B，fresh 0x14097edcf-0x14097ee27）：
        // +0x10 = roof 色、+0x14..0x20 = appearance、+0x28 = 常量 profile 0.5、
        // +0x2c = 内联height_01、+0x30/+0x34 = 常量ridge=0.4/eave=0.2、
        // +0x38 = 0、+0x39 = 扁屋顶样式（settings+0x41）。
        let mut height_01 = circle_roof_height_01(event.radius);
        let mut roof_style = colors.flat_rim_style.native_tag();
        if let Some(previous) = event.previous_wall {
            if let Some(entity) = public.roof_entity(previous) {
                let bits = entity.to_bits();
                if bits >> 32 != 0 {
                    if let Ok(component) = roof_query.get(entity) {
                        roof_style = component.observed()[0x3d];
                        let pitch = component.height_01();
                        // 有序<=；NaN保持计算高度，style仍继承。
                        if pitch <= 0.1 {
                            height_01 = pitch;
                        }
                    }
                }
            }
        }
        roofs.write(build_roof_create_request(
            id,
            0.5,
            height_01,
            0.4,
            0.2,
            roof_style,
            false,
            appearance.roof_color_id,
            appearance.roof_appearance,
        ));

        let record =
            RecordWallHistory::new_wall(id, inner.get_unwrapped(id), appearance.wall_color_id)
                .expect("原圆形创建必须具有可录制的完整墙快照");
        record_creation_history(&mut history, EditType::Wall(record.0), &mut failures);
        let mut presence = [0; 44];
        presence[..4].copy_from_slice(&appearance.roof_color_id.to_le_bytes());
        presence[0x18..0x1c].copy_from_slice(&0.5_f32.to_le_bytes());
        presence[0x1c..0x20].copy_from_slice(&height_01.to_le_bytes());
        presence[0x20..0x24].copy_from_slice(&0.4_f32.to_le_bytes());
        presence[0x24..0x28].copy_from_slice(&0.2_f32.to_le_bytes());
        presence[0x29] = roof_style;
        record_creation_history(
            &mut history,
            RoofHistoryEdit::create(id.0, presence).into(),
            &mut failures,
        );
        if colors.gates.0 != 0 {
            set_gate_color.write(SetGateColorIdCmd::new(id, colors.gates.0, 0));
            record_creation_history(
                &mut history,
                EditType::Color(Arc::new(ColorEdit::Gate {
                    wall_id: id,
                    from: StrokeId(0),
                    to: StrokeId(colors.gates.0),
                })),
                &mut failures,
            );
        }
        toolbar.write(RequestToolbarCategoryChange(WindowFocusEvent(7)));
        sheep.write(CmdCheckForSheepInsideBuilding { wall: id });
    }
}

/// 沿用当前History的显式错误出口；不得把录制失败变成静默丢弃。
fn record_creation_history(
    history: &mut History,
    edit: EditType,
    failures: &mut EventWriter<HistoryRecordingFailed>,
) {
    if let Err(error) = history.add(edit.clone()) {
        failures.write(HistoryRecordingFailed { edit, error });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::WallId;

    #[test]
    fn circle_roof_parameters_match_original_instructions() {
        // The original calls the platform `cosf`; the fixtures bridged it to the host libm,
        // and NaN results carry the host's sign (Darwin positive, Windows UCRT negative).
        // Each platform compares against original-binary results made with its own libm.
        #[cfg(windows)]
        const FIXTURE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/shape-create-circle-roof-parameters.windows-ucrt.ron"
        ));
        #[cfg(not(windows))]
        const FIXTURE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/shape-create-circle-roof-parameters.ron"
        ));
        let rows: Vec<(u32, u32)> = ron::from_str(FIXTURE).unwrap();
        for (case, (radius, expected)) in rows.into_iter().enumerate() {
            let actual = circle_roof_height_01(f32::from_bits(radius)).to_bits();
            assert_eq!(
                actual, expected,
                "原circle参数case{case} radius={radius:08x}"
            );
        }
    }

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

    fn creation_world() -> World {
        let mut world = World::new();
        world.insert_resource(InnerWalls::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(WallColorIds::default());
        world.insert_resource(HalftimberColorIds::default());
        let mut colors = GladeColorCustomization::default();
        colors.building_wall_style = crate::resources::walls::inner_wall_state::WallStyle(4);
        colors.flat_rim_style = crate::resources::roofs::roof_shape::FlatRoofRimStyle::NoRim;
        colors.gates = crate::resources::color_id::ColorId(9);
        world.insert_resource(colors);
        let mut history: History = History::empty_glade();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        world.init_resource::<Events<CreateCircleCmd>>();
        world.init_resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>();
        world.init_resource::<Events<SetWallColorIdCmd>>();
        world.init_resource::<Events<SetHalftimberColorIdCmd>>();
        world.init_resource::<Events<SetGateColorIdCmd>>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<RequestToolbarCategoryChange>>();
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        world.init_resource::<Events<HistoryRecordingFailed>>();
        world
    }

    #[test]
    fn invalid_circle_preserves_original_tree_event_before_curve_failure() {
        let mut world = creation_world();
        world
            .resource_mut::<Events<CreateCircleCmd>>()
            .send(CreateCircleCmd {
                previous_wall: None,
                center: glam::Vec2::ZERO,
                radius: f32::NAN,
                rotation: 0.4,
                height: 3.5,
                elevation: 1.25,
            });
        let mut schedule = Schedule::default();
        schedule.add_systems(create_circle);
        let failure =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(failure.is_err());
        assert_eq!(world.resource::<Events<HideTreesinAreaCmd>>().len(), 1);
        assert_eq!(world.resource::<InnerWalls>().all().count(), 0);
        assert!(
            world
                .resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
                .is_empty()
        );
        assert!(
            world
                .resource::<History>()
                .pending()
                .unwrap()
                .edits
                .is_empty()
        );
    }

    #[test]
    fn creating_a_circle_writes_wall_and_roof_and_history() {
        let mut world = creation_world();
        world
            .resource_mut::<Events<CreateCircleCmd>>()
            .send(CreateCircleCmd {
                previous_wall: None,
                center: glam::Vec2::ZERO,
                radius: 3.0,
                rotation: 0.7,
                height: 6.5,
                elevation: 2.25,
            });
        let mut schedule = Schedule::default();
        schedule.add_systems(create_circle);
        schedule.run(&mut world);
        let inner = world.resource::<InnerWalls>();
        let (id, _state) = inner.all().next().unwrap();
        assert_eq!(id, WallId(1));
        assert!(inner.get_unwrapped(id).is_premade_shape());
        assert_eq!(inner.get_unwrapped(id).source().extra, 6.5);
        assert_eq!(inner.get_unwrapped(id).source().terrain_floor, 2.25);
        let shape = inner.get_unwrapped(id).source().shape.bytes();
        assert_eq!(
            f32::from_le_bytes(shape[0x10..0x14].try_into().unwrap()),
            0.7
        );
        let roofs = world
            .resource_mut::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(roofs.len(), 1);
        assert_eq!(
            roofs[0].0.wall_id(),
            crate::resources::walls::publication_fields::WallId(1)
        );
        // 原profile=0.5、计算height_01、ridge=0.4、eave=0.2。
        let record = roofs[0].0.record();
        assert_eq!(
            f32::from_le_bytes(record[0x28..0x2c].try_into().unwrap()),
            0.5
        );
        let height = f32::from_le_bytes(record[0x2c..0x30].try_into().unwrap());
        assert!((height - circle_roof_height_01(3.0)).abs() < 1e-6);
        assert!(height < 1.0);
        assert_eq!(
            f32::from_le_bytes(record[0x30..0x34].try_into().unwrap()),
            0.4
        );
        assert_eq!(
            f32::from_le_bytes(record[0x34..0x38].try_into().unwrap()),
            0.2
        );
        // fresh：圆圈 +0x38 恒 0、+0x39 = settings+0x41；update_style 写
        // settings+0x42。
        assert_eq!(record[0x38], 0);
        assert_eq!(record[0x39], 1);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get_unwrapped(WallId(1))
                .source()
                .style,
            4
        );
        let history = world.resource::<History>().pending().unwrap();
        assert_eq!(
            history
                .edits
                .iter()
                .map(EditType::native_tag)
                .collect::<Vec<_>>(),
            vec![7, 8, 9]
        );
        assert!(
            matches!(&history.edits[2], EditType::Color(edit) if matches!(edit.as_ref(), ColorEdit::Gate { wall_id: WallId(1), from: StrokeId(0), to: StrokeId(9) }))
        );
        let hidden = world
            .resource_mut::<Events<HideTreesinAreaCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(hidden.len(), 1);
        assert_eq!(
            (
                hidden[0].height_limit,
                hidden[0].expansion,
                hidden[0].finished
            ),
            (2.25, 4.0, true)
        );
        assert!(matches!(hidden[0].area, TreeHideArea::Circle(circle) if circle.rotation == 0.7));
        assert_eq!(world.resource::<Events<SetWallColorIdCmd>>().len(), 1);
        assert_eq!(world.resource::<Events<SetHalftimberColorIdCmd>>().len(), 1);
        assert_eq!(world.resource::<Events<SetGateColorIdCmd>>().len(), 1);
        assert_eq!(
            world
                .resource::<Events<RequestToolbarCategoryChange>>()
                .iter_current_update_events()
                .next()
                .unwrap()
                .0
                .0,
            7
        );
        assert_eq!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .iter_current_update_events()
                .next()
                .unwrap()
                .wall,
            id
        );
        assert!(
            world
                .resource::<Events<HistoryRecordingFailed>>()
                .is_empty()
        );
        // 重跑系统不重复消费原事件，也不重复录制或创建墙。
        schedule.run(&mut world);
        assert_eq!(world.resource::<InnerWalls>().all().count(), 1);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            3
        );
    }
}

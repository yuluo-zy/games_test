//! B `systems/wall/shapes/rectangle/create_rectangle.rs`: rectangle wall
//! creation.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/create_rectangle.rs.md`
//! (O, 0x140a93790) + fresh decomp 2026-10-17. Rectangle2d::as_curve2 builds
//! the smoothstep-cornered curve; the wall is created with the rectangle
//! enclosure; a CreateRoofCmd carries the roof params (profile 0.5, computed
//! height_01, ridge_length_01, eave 0.2) and a wall-creation history record
//! follows. The ridge/height interpolation is inline in create_rectangle and
//! is identical to `roof_shape::ridge_length_01_from_rectangle_ratio`
//! (0x1408e3b60):
//!   height_u = (max_dim - 2.25)/12.75，进入SineInOut前不钳制。
//!   height_01= clamp01((6·SineInOut(height_u) + 3·(1-SineInOut(height_u)))/12)
//!   smooth   = smoothstep(clamp01((max_dim - 2.25)/5.25))
//!   ridge    = smoothstep(clamp01((1.5·smooth + max(w/h,h/w) - 1.5)/1.5))
//!             (1.5 @0x1429258d0, -1.5 @0x1429df180; SineInOut tag2 @0x142aa5938)

use ::utils::geometry::Rectangle2d;
use bevy_ecs::prelude::*;
use std::sync::Arc;
use utils::easing_functions::EasingFunction;

use super::super::{CreateRectangleCmd, build_roof_create_request, to_authority_curve};
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

/// 原树隐藏外扩固定4；创建高度及海拔必须独立读取事件。
const TREE_EXPANSION: f32 = 4.0;
/// Ridge-length ratio offset (-2.25, 0xc0100000 @0x142aa5084).
const RIDGE_RATIO_OFFSET: f32 = -2.25;
/// Ridge-length ratio divisor (5.25 @0x142aa50dc).
const RIDGE_RATIO_DIVISOR: f32 = 5.25;
/// 原140a93e8c使用独立常量142aa5088，而非ridge的5.25。
const HEIGHT_RATIO_DIVISOR: f32 = 12.75;
/// Old-roof flat-pitch threshold (0.1, 0x3dcccccd @0x142925984). Native:
/// `UCOMISS 0.1,pitch; JC skip` → override only when `pitch <= 0.1` and
/// ordered (NaN skips).
const INHERITED_FLAT_PITCH: f32 = 0.1;
/// Roof profile constant at record `+0x28` (0.5, 0x3f000000).
const ROOF_PROFILE_01: f32 = 0.5;
/// Roof eave constant at record `+0x34` (0.2, 0x3e4ccccd).
const ROOF_EAVE_01: f32 = 0.2;

/// `ridge_length_01_from_rectangle_ratio` (create_rectangle inline ridge):
/// aspect ratio drives the roof ridge.
///
/// fresh（0x140a93790 内联 + 0x1408e3b60）：`u = clamp01((max_dim−2.25)/5.25)`、
/// `smooth = smoothstep(u)`、`blend = (1.5·smooth + max(w/h,h/w) − 1.5)/1.5`、
/// 返回 `smoothstep(clamp01(blend))`。边界：`max_dim ≤ 2.25 → u=0 → smooth=0`，
/// 此时 ridge 仅由纵横比驱动（`ratio>1.5` 才非零）；正方形 → ridge=0。
pub fn ridge_length_01_from_rectangle_ratio(dimensions: glam::Vec2) -> f32 {
    let x = dimensions.x;
    let y = dimensions.y;
    let ratio_xz = x / y;
    let ratio_zx = y / x;
    let ratio = f32::max(ratio_zx, ratio_xz);
    let max_dim = f32::max(x, y);
    let u = ((max_dim + RIDGE_RATIO_OFFSET) / RIDGE_RATIO_DIVISOR).clamp(0.0, 1.0);
    let smooth = (3.0 - (u + u)) * (u * u);
    let blended = (1.5 * smooth + ratio - 1.5) / 1.5;
    let t = blended.clamp(0.0, 1.0);
    (3.0 - (t + t)) * (t * t)
}

/// `rectangle_roof_height_01`（create_rectangle 内联 height_01）：
/// `u = (max_dim−2.25)/12.75`（不先钳制）、`e = SineInOut(u)`、
/// `height_01 = clamp01((6e + 3(1−e))/12)`（低 6/高 3/除 12，常量
/// 0x142934690/0x1429cd078/0x1429cd2d4）。（1+e）/4 ∈ [0.25, 0.5]。
pub fn rectangle_roof_height_01(dimensions: glam::Vec2) -> f32 {
    let max_dim = f32::max(dimensions.x, dimensions.y);
    let u = (max_dim + RIDGE_RATIO_OFFSET) / HEIGHT_RATIO_DIVISOR;
    let e = EasingFunction::SineInOut.apply(u);
    ((6.0 * e + 3.0 * (1.0 - e)) / 12.0).clamp(0.0, 1.0)
}

/// 原140a93790：原事件、墙、配色、屋顶、历史及创建反馈完整消费链。
pub fn create_rectangle(
    mut requests: EventReader<CreateRectangleCmd>,
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
        let rect = Rectangle2d::new(event.center, event.dimensions, event.orientation);
        hide_trees.write(HideTreesinAreaCmd {
            area: TreeHideArea::Rectangle(rect),
            height_limit: event.elevation,
            expansion: TREE_EXPANSION,
            finished: true,
        });
        let curve = rect.as_curve2();
        let authority = to_authority_curve(&curve);

        let shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_record(),
        );
        // fresh（0x140a93790）：new_wall_w_params → update_elevation(0x140aaaac0)
        // → get_building_and_roof_colors(0x140a94880) → update_style(0x140aaa350)
        // → update_wall_state_enclosure(0x140aa9610)。
        let id = inner.new_wall_w_params(authority, event.height, 1, 0, false);
        inner.update_elevation(id, event.elevation);
        // 原生首参 = 事件 +0 的 `== 1` 判别（rectangle），previous id = 事件 +8
        // （fresh 0x140a93ed2/0x140a93edb）。CreateRectangleCmd 现携带
        // `previous_wall`（None 即原生判别位为 0 的默认分支）。
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
        inner.update_wall_state_enclosure(id, shape);

        // 旧屋顶继承（fresh 0x140a93ed2-0x140a94099）：事件判别 1 且 previous
        // id 存在、roofs 探针命中、entity generation != 0、dense roof 记录链
        // 命中时：style = component+0x3d；pitch = component+0x30，且
        // `pitch <= 0.1`（有序；NaN 跳过）时 height_01 改为 pitch。
        let mut roof_style = colors.flat_rim_style.native_tag();
        let mut height_01 = rectangle_roof_height_01(event.dimensions);
        if let Some(previous) = event.previous_wall {
            if let Some(entity_bits) = public.roof_entity(previous).map(|e| e.to_bits()) {
                if entity_bits >> 32 != 0 {
                    if let Ok(component) = roof_query.get(Entity::from_bits(entity_bits)) {
                        roof_style = component.observed()[0x3d];
                        let pitch = component.height_01();
                        if pitch <= INHERITED_FLAT_PITCH {
                            height_01 = pitch;
                        }
                    }
                }
            }
        }

        // fresh（0x140a93790）：屋顶参数 = 常量 profile 0.5（+0x28）、内联
        // height_01（+0x2c）、ridge_length_01（+0x30）、常量 eave 0.2（+0x34）。
        // +0x38 = ridge_dir 布尔（width < length）、+0x39 = 屋顶样式
        // （默认 settings+0x41，命中旧屋顶时 component+0x3d；fresh
        // 0x140a93ece/0x140a94081/0x140a94166）；+0x10/+0x14..0x20 来自
        // get_building_and_roof_colors（roof 色 / appearance）。
        let ridge_length_01 = ridge_length_01_from_rectangle_ratio(event.dimensions);
        let ridge_dir = event.dimensions.x < event.dimensions.y;
        roofs.write(build_roof_create_request(
            id,
            ROOF_PROFILE_01,
            height_01,
            ridge_length_01,
            ROOF_EAVE_01,
            roof_style,
            ridge_dir,
            appearance.roof_color_id,
            appearance.roof_appearance,
        ));

        // ev_new_wall（原生 0x140a83300 内部自行 History::add Wall edit）。
        let record =
            RecordWallHistory::new_wall(id, inner.get_unwrapped(id), appearance.wall_color_id)
                .expect("原矩形创建必须具有可录制的完整墙快照");
        record_creation_history(&mut history, EditType::Wall(record.0), &mut failures);

        // History::add #1 — EditType::Roof（原生 tag 8 @0x140a9451d），
        // RoofHistoryEdit tag 0 = create（0x140a944d1）。presence 逐字段
        // （payload = data+0x10，presence = payload[4..48]）：
        //   [+0x00] roof color id   (0x140a944d5/0x944db)
        //   [+0x18] profile 0.5     (0x140a944ea)
        //   [+0x1c] height_01       (0x140a944f1)
        //   [+0x20] ridge_length_01 (0x140a944f7)
        //   [+0x24] eave 0.2        (0x140a944fd)
        //   [+0x28] ridge_dir       (0x140a94504)
        //   [+0x29] roof style      (0x140a9450f)
        let mut presence = [0u8; 44];
        presence[0x00..0x04].copy_from_slice(&appearance.roof_color_id.to_le_bytes());
        presence[0x18..0x1c].copy_from_slice(&ROOF_PROFILE_01.to_le_bytes());
        presence[0x1c..0x20].copy_from_slice(&height_01.to_le_bytes());
        presence[0x20..0x24].copy_from_slice(&ridge_length_01.to_le_bytes());
        presence[0x24..0x28].copy_from_slice(&ROOF_EAVE_01.to_le_bytes());
        presence[0x28] = u8::from(ridge_dir);
        presence[0x29] = roof_style;
        record_creation_history(
            &mut history,
            RoofHistoryEdit::create(id.0, presence).into(),
            &mut failures,
        );

        // History::add #2 — EditType::Color（原生 tag 9 @0x140a94617），
        // gate = settings.gates != 0；原target=4即ColorEdit::Gate，使用现有
        // typed变体写同一History，不能进入旧RecordColorHistory空消费路径。
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

/// 当前History接口返回显式错误，沿用真实错误事件而非静默吞掉编辑。
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
    fn rectangle_roof_parameters_match_original_instructions() {
        // See the circle test: NaN results carry the bridged platform libm's sign.
        #[cfg(windows)]
        const FIXTURE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/shape-create-rectangle-roof-parameters.windows-ucrt.ron"
        ));
        #[cfg(not(windows))]
        const FIXTURE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/shape-create-rectangle-roof-parameters.ron"
        ));
        let rows: Vec<(u32, u32, u32, u32, u8)> = ron::from_str(FIXTURE).unwrap();
        let mut canonical_nan_sign_boundaries = 0;
        for (case, (x, y, height, ridge, direction)) in rows.into_iter().enumerate() {
            let dimensions = glam::Vec2::new(f32::from_bits(x), f32::from_bits(y));
            assert_eq!(
                rectangle_roof_height_01(dimensions).to_bits(),
                height,
                "原rectangle高度case{case} {x:08x}/{y:08x}"
            );
            let actual_ridge = ridge_length_01_from_rectangle_ratio(dimensions).to_bits();
            // 独立MULSS(0,inf)/DIVSS(0,0)实验证明：x86新生canonical NaN
            // 为ffc00000，ARM为7fc00000。只单列这一符号差异，其他payload仍严格。
            if actual_ridge != ridge
                && actual_ridge & 0x7fff_ffff == 0x7fc0_0000
                && ridge & 0x7fff_ffff == 0x7fc0_0000
            {
                canonical_nan_sign_boundaries += 1;
            } else {
                assert_eq!(
                    actual_ridge, ridge,
                    "原rectangle脊长case{case} {x:08x}/{y:08x}"
                );
            }
            assert_eq!(
                u8::from(dimensions.x < dimensions.y),
                direction,
                "原rectangle方向case{case}"
            );
        }
        eprintln!(
            "rectangle原夹具：canonical NaN符号平台边界{canonical_nan_sign_boundaries}例，其余逐位一致"
        );
    }

    #[test]
    fn real_canvas_creation_inherits_records_publishes_and_releases_previews() {
        use crate::resources::walls::{WallState, publication_source::TerrainHeightGrid512};
        use crate::systems::mode_manager::{AppMode, WallShape};
        use crate::systems::wall::shapes::rectangle::ui_create_rectangle::tests::{
            interact, validation_world, wake_preview_factories,
        };
        use crate::systems::wall::shapes::{ShapePlacement, circle};
        use bevy_ecs::schedule::IntoScheduleConfigs;

        // 真组件、缺组件、过期实体、NaN/阈值pitch与未开始历史录制分别验收。
        for (rectangle, roof_case) in [false, true]
            .into_iter()
            .flat_map(|shape| (0..7).map(move |case| (shape, case)))
        {
            let mode = AppMode::Wall(if rectangle {
                WallShape::Rectangle
            } else {
                WallShape::Circle
            });
            let mut world = validation_world(mode);
            let terrain = TerrainHeightGrid512 {
                encoded: Arc::from(vec![0; 512 * 512]),
                minimum_y: 0.0,
                maximum_y: 0.0,
                sampling_fix: false,
            };
            // 初始世界含真实已发布墙及Roof组件，继承不再靠伪造密集表。
            let old_circle = utils::geometry::Circle2d::new(glam::Vec2::ZERO, 2.0);
            let mut inner = InnerWalls::default();
            let previous = inner.new_wall_w_params(
                to_authority_curve(&old_circle.as_curve2(0.2)),
                3.0,
                2,
                0,
                false,
            );
            inner.update_wall_state_enclosure(
                previous,
                ShapeParameters::from_shape(Shape::Circle(old_circle)).to_snapshot(),
            );
            let mut public = PublicWalls::default();
            public.consume_with_paths(&mut inner, &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
            assert!(public.get(previous).is_some());
            let mut roof_record = [0; 88];
            roof_record[..8].copy_from_slice(&previous.0.to_le_bytes());
            let old_pitch = match roof_case {
                1 => 0.100_001,
                4 => f32::from_bits(0x7fc0_1234),
                5 => 0.1,
                _ => 0.05,
            };
            roof_record[0x30..0x34].copy_from_slice(&old_pitch.to_le_bytes());
            roof_record[0x3d] = 0;
            roof_record[0x40..0x4c].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
            roof_record[0x50..0x54].copy_from_slice(&55_u32.to_le_bytes());
            let roof = if roof_case == 2 {
                world.spawn_empty().id()
            } else {
                world.spawn(Roof::from_observed(roof_record)).id()
            };
            if roof_case == 3 {
                assert!(world.despawn(roof));
            }
            public.associate_roof(previous, roof);
            world.insert_resource(inner);
            world.insert_resource(public);
            let mut wall_colors = WallColorIds::default();
            wall_colors.0[previous.0 as usize] = 21;
            let mut timber_colors = HalftimberColorIds::default();
            timber_colors.0[previous.0 as usize] = 23;
            world.insert_resource(wall_colors);
            world.insert_resource(timber_colors);
            let mut colors = GladeColorCustomization::default();
            colors.flat_rim_style = crate::resources::roofs::roof_shape::FlatRoofRimStyle::NoRim;
            world.insert_resource(colors);
            let mut history: History = History::empty_glade();
            if roof_case != 6 {
                history.start_input_recording(0).unwrap();
            }
            world.insert_resource(history);
            world.init_resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>();
            world.init_resource::<Events<SetWallColorIdCmd>>();
            world.init_resource::<Events<SetHalftimberColorIdCmd>>();
            world.init_resource::<Events<SetGateColorIdCmd>>();
            world.init_resource::<Events<RequestToolbarCategoryChange>>();
            world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
            world.init_resource::<Events<HistoryRecordingFailed>>();
            let mut placement = *world.resource::<ShapePlacement>().observed();
            placement[0x20..0x28].copy_from_slice(&previous.0.to_le_bytes());
            placement[0x40..0x44].copy_from_slice(&9.25_f32.to_le_bytes());
            world.insert_resource(ShapePlacement::from_observed(placement));
            let mut schedule = Schedule::default();
            let canvas = if rectangle {
                schedule.add_systems(
                    (
                        super::super::ui_create_rectangle::ui_create_rectangle,
                        create_rectangle,
                    )
                        .chain_ignore_deferred(),
                );
                "wall_rectangle_canvas"
            } else {
                schedule.add_systems(
                    (
                        circle::ui_create_circle::ui_create_circle,
                        circle::create_circle::create_circle,
                    )
                        .chain_ignore_deferred(),
                );
                "wall_circle_canvas"
            };
            schedule.run(&mut world);
            interact(&mut world, canvas, 3);
            schedule.run(&mut world);
            interact(&mut world, canvas, 1);
            schedule.run(&mut world);
            let created = WallId(previous.0 + 1);
            let authority = world.resource::<InnerWalls>().get_unwrapped(created);
            assert_eq!(
                (
                    authority.source().extra,
                    authority.source().terrain_floor,
                    authority.source().style
                ),
                (9.25, 5.0, 2)
            );
            let saved = serde_json::to_string(authority).unwrap();
            let restored: WallState = serde_json::from_str(&saved).unwrap();
            assert_eq!(restored.source().extra, 9.25);
            assert_eq!(restored.source().terrain_floor, 5.0);
            assert_eq!(
                restored.source().shape.bytes(),
                authority.source().shape.bytes()
            );
            let requests = world
                .resource_mut::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
                .drain()
                .collect::<Vec<_>>();
            assert_eq!(requests.len(), 1);
            let request = requests[0].0.record();
            let inherited = !matches!(roof_case, 2 | 3);
            let fallback_height = if rectangle {
                rectangle_roof_height_01(glam::Vec2::splat(6.0))
            } else {
                circle::create_circle::circle_roof_height_01(3.0)
            };
            let expected_height = if inherited && old_pitch <= 0.1 {
                old_pitch
            } else {
                fallback_height
            };
            assert_eq!(
                f32::from_le_bytes(request[0x2c..0x30].try_into().unwrap()).to_bits(),
                expected_height.to_bits(),
                "shape={rectangle} roof_case={roof_case}"
            );
            assert_eq!(request[0x39], if inherited { 0 } else { 1 });
            assert_eq!(
                u32::from_le_bytes(request[0x10..0x14].try_into().unwrap()),
                if inherited { 55 } else { 0 }
            );
            assert_eq!(
                &request[0x14..0x20],
                if inherited {
                    &roof_record[0x40..0x4c]
                } else {
                    &[0; 12]
                }
            );
            if roof_case == 6 {
                assert!(world.resource::<History>().pending().is_none());
                let failures = world
                    .resource_mut::<Events<HistoryRecordingFailed>>()
                    .drain()
                    .collect::<Vec<_>>();
                assert_eq!(
                    failures
                        .iter()
                        .map(|failure| failure.edit.native_tag())
                        .collect::<Vec<_>>(),
                    vec![7, 8]
                );
            } else {
                assert_eq!(
                    world
                        .resource::<History>()
                        .pending()
                        .unwrap()
                        .edits
                        .iter()
                        .map(EditType::native_tag)
                        .collect::<Vec<_>>(),
                    vec![7, 8]
                );
                world
                    .resource_mut::<History>()
                    .finish_input_recording()
                    .unwrap();
                assert_eq!(world.resource::<History>().ledger().entries.len(), 1);
                assert!(
                    world
                        .resource::<Events<HistoryRecordingFailed>>()
                        .is_empty()
                );
            }
            world.resource_scope(|world, mut public: Mut<PublicWalls>| {
                public.consume_with_paths(&mut world.resource_mut::<InnerWalls>(), &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
                assert!(public.get(created).is_some());
                world.resource_mut::<InnerWalls>().remove(created);
                public.consume_with_paths(&mut world.resource_mut::<InnerWalls>(), &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
                assert!(public.get(created).is_none());
                assert!(public.get(previous).is_some());
                assert!(public.last_delta.removed.contains(&created));
            });
            // 共用真实signifier建立、动画标记、实体删除与TransientMesh释放链。
            // TestDevice边界只验证CPU资源所有权，不冒称真实GPU画面验收。
            wake_preview_factories(&mut world);
        }
    }

    #[test]
    fn square_rectangle_has_no_ridge_and_elongated_has_one() {
        // 原生（fresh）：正方形 max_dim>2.25 但 ratio=1 → blend=(1.5·smooth+1−1.5)/1.5<0
        // → ridge=0；长条（6×4）ratio=1.5 → blend=smooth → ridge>0。
        let square = ridge_length_01_from_rectangle_ratio(glam::Vec2::new(4.0, 4.0));
        assert_eq!(square, 0.0);
        let elongated = ridge_length_01_from_rectangle_ratio(glam::Vec2::new(6.0, 4.0));
        assert!((elongated - 0.8977).abs() < 1e-3);
        // 边界 max_dim≤2.25 → u=0 → smooth=0 → blend=(ratio−1.5)/1.5；ratio=1<1.5 → 0。
        assert_eq!(
            ridge_length_01_from_rectangle_ratio(glam::Vec2::new(2.0, 2.0)),
            0.0
        );
    }

    #[test]
    fn rectangle_roof_height_01_is_bounded_between_quarter_and_half() {
        let h = rectangle_roof_height_01(glam::Vec2::new(6.0, 4.0));
        assert!((h - 0.29967).abs() < 1e-4);
        assert_eq!(rectangle_roof_height_01(glam::Vec2::splat(2.25)), 0.25);
        // 只有高度插值到15时u=1；大尺寸继续按原SineInOut振荡。
        assert_eq!(rectangle_roof_height_01(glam::Vec2::new(15.0, 1.0)), 0.5);
        assert!(rectangle_roof_height_01(glam::Vec2::new(1.0, 1.0)) > 0.25);
    }

    #[test]
    fn creating_a_rectangle_writes_wall_and_roof_and_history() {
        let mut world = World::new();
        world.insert_resource(InnerWalls::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(WallColorIds::default());
        world.insert_resource(HalftimberColorIds::default());
        let mut colors = GladeColorCustomization::default();
        colors.building_wall_style = crate::resources::walls::inner_wall_state::WallStyle(3);
        colors.flat_rim_style = crate::resources::roofs::roof_shape::FlatRoofRimStyle::NoRim;
        colors.gates = crate::resources::color_id::ColorId(11);
        world.insert_resource(colors);
        let mut history: History = History::empty_glade();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        world.init_resource::<Events<CreateRectangleCmd>>();
        world.init_resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>();
        world.init_resource::<Events<SetWallColorIdCmd>>();
        world.init_resource::<Events<SetHalftimberColorIdCmd>>();
        world.init_resource::<Events<SetGateColorIdCmd>>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<RequestToolbarCategoryChange>>();
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        world.init_resource::<Events<HistoryRecordingFailed>>();
        world
            .resource_mut::<Events<CreateRectangleCmd>>()
            .send(CreateRectangleCmd {
                center: glam::Vec2::ZERO,
                dimensions: glam::Vec2::new(6.0, 4.0),
                orientation: glam::Vec2::X,
                previous_wall: None,
                height: 7.25,
                elevation: 1.75,
            });
        let mut schedule = Schedule::default();
        schedule.add_systems(create_rectangle);
        schedule.run(&mut world);
        let inner = world.resource::<InnerWalls>();
        let (id, _state) = inner.all().next().unwrap();
        assert_eq!(id, WallId(1));
        assert!(inner.get_unwrapped(id).is_premade_shape());
        assert_eq!(inner.get_unwrapped(id).source().extra, 7.25);
        assert_eq!(inner.get_unwrapped(id).source().terrain_floor, 1.75);
        // fresh：update_style 写入 colors.building_wall_style（settings+0x42）。
        let inherited_style = inner.get_unwrapped(id).source().style;
        let roofs = world
            .resource_mut::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(roofs.len(), 1);
        assert_eq!(inherited_style, 3);
        let record = roofs[0].0.record();
        assert_eq!(record[0x39], 1); // 扁屋顶样式 = settings+0x41
        assert_eq!(record[0x38], 0); // 6>4 → ridge_dir=false
        // History::add #1 = EditType::Roof / RoofHistoryEdit tag 0 (create).
        let history = world.resource::<History>().pending().unwrap();
        assert_eq!(
            history
                .edits
                .iter()
                .map(EditType::native_tag)
                .collect::<Vec<_>>(),
            vec![7, 8, 9]
        );
        let EditType::Roof(roof_edit) = &history.edits[1] else {
            panic!("屋顶历史顺序错误")
        };
        assert_eq!(roof_edit.tag(), 0);
        assert_eq!(roof_edit.wall_id(), 1);
        assert_eq!(
            u32::from_le_bytes(roof_edit.payload()[4..8].try_into().unwrap()),
            u32::from_le_bytes(record[0x10..0x14].try_into().unwrap())
        );
        // History::add #2 = EditType::Color gate `settings.gates != 0`,
        // payload {+0x08=4, +0x10=wall id, +0x18=0, +0x1c=gates}.
        assert!(
            matches!(&history.edits[2], EditType::Color(edit) if matches!(edit.as_ref(), ColorEdit::Gate { wall_id: WallId(1), from: StrokeId(0), to: StrokeId(11) }))
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
            (1.75, 4.0, true)
        );
        assert_eq!(world.resource::<Events<SetWallColorIdCmd>>().len(), 1);
        assert_eq!(world.resource::<Events<SetHalftimberColorIdCmd>>().len(), 1);
        assert_eq!(world.resource::<Events<SetGateColorIdCmd>>().len(), 1);
        assert_eq!(
            world
                .resource::<Events<CmdCheckForSheepInsideBuilding>>()
                .len(),
            1
        );
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
        assert!(
            world
                .resource::<Events<HistoryRecordingFailed>>()
                .is_empty()
        );
        schedule.run(&mut world);
        assert_eq!(world.resource::<InnerWalls>().all().count(), 1);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            3
        );
    }
}

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
//!   u        = clamp01((max_dim - 2.25)/5.25)   (5.25 @0x142aa50dc)
//!   height_01= clamp01((6·SineInOut(u) + 3·(1-SineInOut(u)))/12)  (= (1+e)/4)
//!   smooth   = smoothstep(u)
//!   ridge    = smoothstep(clamp01((1.5·smooth + max(w/h,h/w) - 1.5)/1.5))
//!             (1.5 @0x1429258d0, -1.5 @0x1429df180; SineInOut tag2 @0x142aa5938)

use bevy_ecs::prelude::*;
use ::utils::geometry::Rectangle2d;
use utils::easing_functions::EasingFunction;

use super::super::{CreateRectangleCmd, build_roof_create_request, to_authority_curve};
use crate::resources::walls::color_selection::RoofColorComponentTables;
use crate::resources::walls::color_selection::get_building_and_roof_colors;
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::color_id::{HalftimberColorIds, WallColorIds};
use crate::resources::glade_color_customization::GladeColorCustomization;
use crate::resources::roofs::commands::RecordRoofHistory;
use crate::resources::walls::{InnerWalls, public_walls::PublicWalls};
use crate::systems::wall::{
    history_recording::{ColorHistoryRecord, RecordColorHistory, RecordWallHistory},
    valid_enclosure::{Shape, ShapeParameters},
};

/// Default wall elevation for shape creation (4.0, 0x40800000).
const SHAPE_ELEVATION: f32 = 4.0;
/// Ridge-length ratio offset (-2.25, 0xc0100000 @0x142aa5084).
const RIDGE_RATIO_OFFSET: f32 = -2.25;
/// Ridge-length ratio divisor (5.25 @0x142aa50dc).
const RIDGE_RATIO_DIVISOR: f32 = 5.25;
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
    let smooth = (3.0 - 2.0 * u) * u * u;
    let blended = (1.5 * smooth + ratio - 1.5) / 1.5;
    let t = blended.clamp(0.0, 1.0);
    (3.0 - 2.0 * t) * t * t
}

/// `rectangle_roof_height_01`（create_rectangle 内联 height_01）：
/// `u = clamp01((max_dim−2.25)/5.25)`、`e = SineInOut(u)`、
/// `height_01 = clamp01((6e + 3(1−e))/12)`（低 6/高 3/除 12，常量
/// 0x142934690/0x1429cd078/0x1429cd2d4）。（1+e）/4 ∈ [0.25, 0.5]。
pub fn rectangle_roof_height_01(dimensions: glam::Vec2) -> f32 {
    let max_dim = f32::max(dimensions.x, dimensions.y);
    let u = ((max_dim + RIDGE_RATIO_OFFSET) / RIDGE_RATIO_DIVISOR).clamp(0.0, 1.0);
    let e = EasingFunction::SineInOut.apply(u);
    ((6.0 * e + 3.0 * (1.0 - e)) / 12.0).clamp(0.0, 1.0)
}

/// B 0x140a93790 (12-param system).
pub fn create_rectangle(
    mut requests: EventReader<CreateRectangleCmd>,
    mut inner: ResMut<InnerWalls>,
    public: Res<PublicWalls>,
    mut roofs: EventWriter<crate::resources::roofs::commands::CreateRoofCmd>,
    mut wall_history: EventWriter<RecordWallHistory>,
    mut roof_history: EventWriter<RecordRoofHistory>,
    mut color_history: EventWriter<RecordColorHistory>,
    wall_colors: Res<WallColorIds>,
    halftimber_colors: Res<HalftimberColorIds>,
    colors: Res<GladeColorCustomization>,
    roof_tables: Res<RoofColorComponentTables>,
) {
    for event in requests.read() {
        let rect = Rectangle2d::new(event.center, event.dimensions, event.orientation);
        let curve = rect.as_curve2();
        let authority = to_authority_curve(&curve);

        let shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_record(),
        );
        // fresh（0x140a93790）：new_wall_w_params → update_elevation(0x140aaaac0)
        // → get_building_and_roof_colors(0x140a94880) → update_style(0x140aaa350)
        // → update_wall_state_enclosure(0x140aa9610)。
        let id = inner.new_wall_w_params(authority, SHAPE_ELEVATION, 0, 0, false);
        // 登记 seam：原生 elevation = create 事件 +0x2c（fresh 0x140a93c39），
        // 恢复的 CreateRectangleCmd 未携带该字段；此处沿用创建高度常量。
        inner.update_elevation(id, SHAPE_ELEVATION);
        // 原生首参 = 事件 +0 的 `== 1` 判别（rectangle），previous id = 事件 +8
        // （fresh 0x140a93ed2/0x140a93edb）。CreateRectangleCmd 现携带
        // `previous_wall`（None 即原生判别位为 0 的默认分支）。
        let appearance = get_building_and_roof_colors(
            event.previous_wall.is_some(),
            event.previous_wall.map_or(0, |wall| wall.0),
            &wall_colors.0,
            &halftimber_colors.0,
            &*public,
            &roof_tables,
            &colors,
        );
        inner.update_style(id, appearance.wall_style);
        inner.update_wall_state_enclosure(id, shape);

        // 旧屋顶继承（fresh 0x140a93ed2-0x140a94099）：事件判别 1 且 previous
        // id 存在、roofs 探针命中、entity generation != 0、dense roof 记录链
        // 命中时：style = component+0x3d；pitch = component+0x30，且
        // `pitch <= 0.1`（有序；NaN 跳过）时 height_01 改为 pitch。
        let mut roof_style = colors.flat_rim_style;
        let mut height_01 = rectangle_roof_height_01(event.dimensions);
        if let Some(previous) = event.previous_wall {
            if let Some(entity_bits) = public.roof_entity(previous).map(|e| e.to_bits()) {
                if entity_bits >> 32 != 0 {
                    if let Some(component) = roof_tables.lookup(entity_bits) {
                        roof_style = component.style();
                        let pitch = component.pitch_01();
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
        let source = inner.get_unwrapped(id);
        if let Ok(record) = RecordWallHistory::new_wall(id, source, 0) {
            wall_history.write(record);
        }

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
        roof_history.write(RecordRoofHistory(RoofHistoryEdit::create(id.0, presence)));

        // History::add #2 — EditType::Color（原生 tag 9 @0x140a94617），
        // gate = settings.gates != 0（@0x140a94541），payload {+0x08 = 4,
        // +0x10 = wall id, +0x18 = 0, +0x1c = settings.gates}。Owner =
        // `RecordColorHistory`；`EditType::Color(Arc<ColorEdit>)` 映射待
        // ColorEdit 内存布局对齐（见 ColorHistoryRecord 登记）。
        if colors.gates.0 != 0 {
            color_history.write(RecordColorHistory(ColorHistoryRecord {
                kind: 4,
                wall: id,
                from: 0,
                to: colors.gates.0,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::WallId;
    use crate::systems::wall::history_recording::register_wall_recording_events;

    #[test]
    fn square_rectangle_has_no_ridge_and_elongated_has_one() {
        // 原生（fresh）：正方形 max_dim>2.25 但 ratio=1 → blend=(1.5·smooth+1−1.5)/1.5<0
        // → ridge=0；长条（6×4）ratio=1.5 → blend=smooth → ridge>0。
        let square = ridge_length_01_from_rectangle_ratio(glam::Vec2::new(4.0, 4.0));
        assert_eq!(square, 0.0);
        let elongated = ridge_length_01_from_rectangle_ratio(glam::Vec2::new(6.0, 4.0));
        assert!((elongated - 0.8977).abs() < 1e-3);
        // 边界 max_dim≤2.25 → u=0 → smooth=0 → blend=(ratio−1.5)/1.5；ratio=1<1.5 → 0。
        assert_eq!(ridge_length_01_from_rectangle_ratio(glam::Vec2::new(2.0, 2.0)), 0.0);
    }

    #[test]
    fn rectangle_roof_height_01_is_bounded_between_quarter_and_half() {
        let h = rectangle_roof_height_01(glam::Vec2::new(6.0, 4.0));
        assert!((h - 0.4529).abs() < 1e-3);
        // max_dim≤2.25 → u=0 → e=0 → height=3/12=0.25。
        assert_eq!(rectangle_roof_height_01(glam::Vec2::new(1.0, 1.0)), 0.25);
        // 极大 max_dim → u=1 → e=1 → height=6/12=0.5。
        assert_eq!(rectangle_roof_height_01(glam::Vec2::new(1000.0, 1.0)), 0.5);
    }

    #[test]
    fn creating_a_rectangle_writes_wall_and_roof_and_history() {
        let mut world = World::new();
        world.insert_resource(InnerWalls::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(WallColorIds::default());
        world.insert_resource(HalftimberColorIds::default());
        let mut colors = GladeColorCustomization::default();
        colors.building_wall_style = 3;
        colors.flat_rim_style = 5;
        colors.gates = crate::resources::color_id::ColorId(11);
        world.insert_resource(colors);
        world.insert_resource(RoofColorComponentTables::default());
        register_wall_recording_events(&mut world);
        world.init_resource::<Events<CreateRectangleCmd>>();
        world.init_resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<RecordRoofHistory>>();
        world.resource_mut::<Events<CreateRectangleCmd>>().send(CreateRectangleCmd {
            center: glam::Vec2::ZERO,
            dimensions: glam::Vec2::new(6.0, 4.0),
            orientation: glam::Vec2::X,
            previous_wall: None,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(create_rectangle);
        schedule.run(&mut world);
        let inner = world.resource::<InnerWalls>();
        let (id, _state) = inner.all().next().unwrap();
        assert_eq!(id, WallId(1));
        assert!(inner.get_unwrapped(id).is_premade_shape());
        // fresh：update_style 写入 colors.building_wall_style（settings+0x42）。
        let inherited_style = inner.get_unwrapped(id).source().style;
        let roofs = world
            .resource_mut::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(roofs.len(), 1);
        assert_eq!(inherited_style, 3);
        let record = roofs[0].0.record();
        assert_eq!(record[0x39], 5); // 扁屋顶样式 = settings+0x41
        assert_eq!(record[0x38], 0); // 6>4 → ridge_dir=false
        // History::add #1 = EditType::Roof / RoofHistoryEdit tag 0 (create).
        let roof_edits = world
            .resource_mut::<Events<RecordRoofHistory>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(roof_edits.len(), 1);
        assert_eq!(roof_edits[0].0.tag(), 0);
        assert_eq!(roof_edits[0].0.wall_id(), 1);
        assert_eq!(
            u32::from_le_bytes(roof_edits[0].0.payload()[4..8].try_into().unwrap()),
            u32::from_le_bytes(record[0x10..0x14].try_into().unwrap())
        );
        // History::add #2 = EditType::Color gate `settings.gates != 0`,
        // payload {+0x08=4, +0x10=wall id, +0x18=0, +0x1c=gates}.
        let color_edits = world
            .resource_mut::<Events<RecordColorHistory>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(color_edits.len(), 1);
        assert_eq!(color_edits[0].0.kind, 4);
        assert_eq!(color_edits[0].0.wall, id);
        assert_eq!(color_edits[0].0.from, 0);
        assert_eq!(color_edits[0].0.to, 11);
        assert_eq!(&color_edits[0].0.payload()[0x10..0x18], &1u64.to_le_bytes());
    }
}

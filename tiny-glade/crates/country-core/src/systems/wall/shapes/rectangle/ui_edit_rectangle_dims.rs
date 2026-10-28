//! B `systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs`: rectangle
//! dimension editing.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs.md`
//! (O, 0x140ad6250 / do_ui_edit_rectangle_dims 0x140ad6e00). Dragging an edge
//! moves it along its normal and writes the new dims into the UiState80
//! (variant 1) plus an EditRectangleCmd. The exact projection constants are
//! 待取证; Rectangle2d::expand_along_edge is reused.

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::super::EditRectangleCmd;
use super::finish_edit_rectangle::RectangleEditUiState;
use crate::resources::walls::InnerWalls;
use crate::systems::wall::valid_enclosure::ShapeParameters;

fn edge_widget_id(
    wall: crate::resources::walls::WallId,
    edge: usize,
) -> crate::systems::ui::resources::ui_state::WidgetId {
    crate::systems::ui::resources::ui_state::WidgetId(
        ::utils::calculate_hash(&(wall.0, edge as u64)) ^ 0x448d3066df154a46,
    )
}

pub fn distance_to_edge(rect: ::utils::geometry::Rectangle2d, edge: usize, cursor: Vec2) -> f32 {
    let (point, _) = rect.get_edge_pts(edge);
    (cursor - point).dot(rect.get_edge_normal(edge))
}

/// B UiRectangleDimsEdit::widget geometry. The selected edge samples the
/// published 3D curve and segment heights; ordinary edges clip the wall volume.
fn edge_widget_geometry(
    wall: &crate::resources::walls::public_wall_state::PublicWallState,
    edge: usize,
    selected: bool,
    trigger: Option<&crate::resources::walls::wall_trigger_volume::WallTriggerVolume>,
) -> crate::systems::ui::resources::widget_params::WidgetGeometry {
    use crate::systems::ui::resources::widget_params::WidgetGeometry;
    let rect = ShapeParameters::from_snapshot(wall.shape()).as_rectangle();
    if !selected {
        return trigger
            .filter(|t| t.is_valid())
            .map(|t| {
                WidgetGeometry::TriangleMesh(t.edit_shape_tris_in_range(Some([
                    rect.get_point_u(edge),
                    rect.get_point_u(edge + 1),
                ])))
            })
            .unwrap_or(WidgetGeometry::Empty);
    }
    let points = &wall
        .prepared_path
        .as_ref()
        .expect("rectangle edge requires prepared path")
        .positions;
    let sample = |point| {
        let coord = ::utils::curve::approx_curve_coord(points, point, |p| Vec2::new(p[0], p[2]));
        let a = coord.segment;
        let b = a + 1;
        let t = coord.t;
        let blend = |x: f32, y: f32| y * t + x * (1.0 - t);
        (
            blend(points[a][1], points[b][1]),
            blend(wall.segment_heights[a], wall.segment_heights[b]),
        )
    };
    let (a, b) = rect.get_edge_pts(edge);
    let (bottom_a, height_a) = sample(a);
    let (bottom_b, height_b) = sample(b);
    let bottom = bottom_a.max(bottom_b);
    let height = height_a.max(height_b);
    let y = bottom + (height - bottom) * 0.5;
    let normal = rect.get_edge_normal(edge);
    let n = glam::Vec3::new(normal.x, 0.0, normal.y);
    let mid = a * 0.5 + b * 0.5;
    let rotation = ::utils::rotation_from_normal(n);
    let position = glam::Vec3::new(mid.x, y, mid.y) + n * (0.63 * 0.5) + rotation * glam::Vec3::Z;
    let transform = crate::utils::components::transform::Transform {
        translation: position,
        rotation,
        scale: glam::Vec3::ONE,
    };
    WidgetGeometry::TriangleMesh(super::ui_edit_rectangle_orientation::box_trigger_volume(
        1.0, 0.5, 2.0, &transform,
    ))
}

/// Original producer event: active intersection, selected wall and edge.
#[derive(Event, Clone, Debug)]
pub struct DoUiEditRectangleDims {
    pub intersection: [u8; 56],
    pub wall: crate::resources::walls::WallId,
    pub rectangle: ::utils::geometry::Rectangle2d,
    pub edge: u64,
    pub phase: u8,
}

#[derive(Clone, Debug)]
pub struct RectangleActiveEdge {
    pub curve: ::utils::curve::Curve2,
    pub widget: crate::systems::ui::resources::ui_state::WidgetId,
    pub position: glam::Vec3,
}
#[derive(Resource, Default)]
pub struct UiEditRectangleActiveWidgetResource(pub Option<RectangleActiveEdge>);

fn write_rectangle(bytes: &mut [u8], rect: ::utils::geometry::Rectangle2d) {
    bytes.copy_from_slice(
        &crate::systems::wall::valid_enclosure::Shape::Rectangle(rect).to_record()[4..],
    );
}

fn read_rectangle(bytes: &[u8]) -> ::utils::geometry::Rectangle2d {
    let mut record = [0; 28];
    record[..4].copy_from_slice(&1u32.to_le_bytes());
    record[4..].copy_from_slice(bytes);
    ShapeParameters::from_observed(record).as_rectangle()
}

/// B do_ui_edit_rectangle_dims; ShowPinned is pending its shared owner.
pub fn do_ui_edit_rectangle_dims(
    mut input: EventReader<DoUiEditRectangleDims>,
    (public, cursor, window, camera, border): (
        Res<crate::resources::walls::public_walls::PublicWalls>,
        Res<crate::resources::mouse_terrain_raycast::CursorPositionSS>,
        Res<crate::resources::window::WindowSize>,
        Res<crate::systems::camera_rig::main_camera::MainCamera>,
        Res<crate::resources::glade_border::GladeBorder>,
    ),
    (mut wip, mut slider, mut archivist): (
        ResMut<RectangleEditUiState>,
        Local<crate::systems::ui::resources::slider3d::Slider3dState>,
        ResMut<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    ),
    (mut edits, mut finish, mut trees, mut display, mut feedback, mut pins): (
        EventWriter<EditRectangleCmd>,
        EventWriter<super::super::FinishEditRectangleCmd>,
        EventWriter<crate::resources::tree_storage::HideTreesinAreaCmd>,
        EventWriter<crate::resources::glade_border::DisplayOutOfBorder>,
        EventWriter<crate::resources::glade_border::FeedbackInputOutOfBorder>,
        EventWriter<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>,
    ),
) {
    use crate::resources::glade_border::{
        BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder,
    };
    use crate::resources::tree_storage::{HideTreesinAreaCmd, TreeHideArea};
    use crate::systems::ui::resources::slider3d::Slider3dMatrices;
    let Some(event) = input.read().last() else {
        return;
    };
    if u32::from_le_bytes(event.intersection[..4].try_into().unwrap()) == 2 {
        return;
    }
    let wall = public
        .get(event.wall)
        .expect("rectangle dimension edit wall missing");
    let edge = event.edge as usize;
    let rect = event.rectangle;
    let normal = rect.get_edge_normal(edge);
    let f = |at| f32::from_le_bytes(event.intersection[at..at + 4].try_into().unwrap());
    let hit = glam::Vec3::new(f(0x24), f(0x28), f(0x2c));
    match event.phase.wrapping_sub(2).min(3) {
        1 => {
            archivist.decorator_cull_allow_any();
            wip.0[..4].copy_from_slice(&2u32.to_le_bytes());
            wip.0[4..8].copy_from_slice(&hit.y.to_le_bytes());
            wip.0[8..16].copy_from_slice(&event.wall.0.to_le_bytes());
            wip.0[16..24].copy_from_slice(&event.edge.to_le_bytes());
            write_rectangle(&mut wip.0[24..48], rect);
            write_rectangle(&mut wip.0[48..72], rect);
            let (a, b) = rect.get_edge_pts(edge);
            let e = b - a;
            let p = Vec2::new(hit.x, hit.z);
            let denominator = (e.y * e.y + e.x * e.x).max(1.0e-20);
            let t = ((p.y - a.y) * e.y + (p.x - a.x) * e.x) / denominator;
            let q = a * (1.0 - t) + b * t;
            let offset = normal * (p - q).dot(normal);
            wip.0[72..76].copy_from_slice(&offset.x.to_le_bytes());
            wip.0[76..80].copy_from_slice(&offset.y.to_le_bytes());
        }
        2 => {
            pins.write(crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::Rectangle {
                rectangle: rect, bottom: wall.authority.elevation(), top: wall.max_y(), exclude_wall: Some(event.wall),
            });
            trees.write(HideTreesinAreaCmd {
                area: TreeHideArea::Rectangle(rect),
                height_limit: wall.authority.elevation(),
                expansion: 1.5,
                finished: false,
            });
        }
        3 => {
            finish.write(super::super::FinishEditRectangleCmd { wall: event.wall });
            trees.write(HideTreesinAreaCmd {
                area: TreeHideArea::Rectangle(rect),
                height_limit: wall.authority.elevation(),
                expansion: 1.5,
                finished: true,
            });
        }
        _ => {}
    }
    if u32::from_le_bytes(wip.0[..4].try_into().unwrap()) != 2 {
        return;
    }
    let direction = glam::Vec3::new(normal.x, 0.0, normal.y);
    let dimension = if edge == 1 || edge == 3 {
        rect.size.y
    } else {
        rect.size.x
    };
    let matrices = Slider3dMatrices {
        second: camera.read_matrix(0),
        gap: [0; 0x40],
        first: camera.read_matrix(0x80),
    };
    let Some((_, scalar)) = slider.update(
        event.phase,
        hit,
        direction,
        dimension,
        cursor.0,
        window.as_dims(),
        &matrices,
        [2.25, 15.0],
    ) else {
        return;
    };
    let mut candidate = rect;
    candidate.expand_along_edge(edge, scalar - dimension, 2.25, 15.0);
    let current = read_rectangle(&wip.0[48..72]);
    if !border.is_shape_inside(BorderShape::Rectangle(candidate)) {
        display.write(DisplayOutOfBorder);
        feedback.write(FeedbackInputOutOfBorder);
        let BorderShape::Rectangle(fitted) = border.search_lerped_shape_that_fits_into_glade(
            BorderShape::Rectangle(current),
            BorderShape::Rectangle(candidate),
        ) else {
            unreachable!()
        };
        candidate = fitted;
    }
    if (candidate.center - current.center).length() > 0.0 {
        write_rectangle(&mut wip.0[48..72], candidate);
        let before = read_rectangle(&wip.0[24..48]);
        let height = f32::from_le_bytes(wip.0[4..8].try_into().unwrap());
        edits.write(EditRectangleCmd {
            wall: event.wall,
            center: candidate.center,
            dimensions: candidate.size,
            orientation: candidate.axis,
            position: glam::Vec3::new(before.center.x, height, before.center.y),
            movement: (scalar - dimension).abs(),
            rotating: false,
        });
    }
}

/// Native producer: register the candidate edge widgets and send the active
/// intersection. The downstream Slider3d transaction is implemented separately.
pub fn ui_edit_rectangle_dims(
    mode: Res<crate::systems::mode_manager::AppMode>,
    _signifiers: Res<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    ui: Res<crate::systems::ui::resources::ui_state::UiState>,
    ray: Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
    public: Res<crate::resources::walls::public_walls::PublicWalls>,
    entities: Res<crate::resources::walls::public_walls::WallEntities>,
    triggers: Query<&crate::resources::walls::wall_trigger_volume::WallTriggerVolume>,
    mut camera_moved: EventReader<crate::systems::camera_rig::main_camera::MainCameraMoved>,
    mut active: ResMut<UiEditRectangleActiveWidgetResource>,
    mut output: EventWriter<DoUiEditRectangleDims>,
    mut commands: Commands,
) {
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::widget_params::WidgetParams;
    use ::utils::geometry::{Aabb2, Ray2};
    active.0 = None;
    let moved = camera_moved.read().count() != 0;
    let selected = match *mode {
        AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) => {
            Some(crate::resources::walls::WallId(id))
        }
        AppMode::Editing(EditSubject::None) => None,
        _ => return,
    };
    for (wall_id, _) in public.all() {
        if selected.is_some_and(|id| id != wall_id) {
            continue;
        }
        let wall = public.get(wall_id).expect("iterated wall");
        if wall.shape().tag() != 1 {
            continue;
        }
        let rect = ShapeParameters::from_snapshot(wall.shape()).as_rectangle();
        let (enter, exit) = Aabb2::from_points(rect.as_points2()).intersect_ray_interval(Ray2 {
            origin: Vec2::new(ray.origin.x, ray.origin.z),
            direction: Vec2::new(ray.dir.x, ray.dir.z),
        });
        let ray_hits = 0.0 <= exit && enter <= exit;
        for edge in 0..4 {
            let id = edge_widget_id(wall_id, edge);
            let is_selected = selected == Some(wall_id);
            if !is_selected
                && !ray_hits
                && !(ui.active_widget() == id && ui.hover_byte() != 0 && ui.intersection_tag() != 2)
            {
                continue;
            }
            let trigger = entities
                .wall_entity(wall_id)
                .and_then(|entity| triggers.get(entity).ok());
            let geometry = edge_widget_geometry(wall, edge, is_selected, trigger);
            let mut params = WidgetParams::default().geometry(geometry);
            params.occlude_by_world = !is_selected;
            let enabled = u32::from(!moved);
            for (i, (slot, kind)) in params
                .sounds
                .slots
                .iter_mut()
                .zip([1u32, 2, 3, 3])
                .enumerate()
            {
                slot[..4].copy_from_slice(&(kind * enabled).to_le_bytes());
                params.fields[[3, 5, 6, 7][i]] = kind * enabled;
            }
            let context = ui.add_with_id(id, params);
            if moved
                && !is_selected
                && !(ui.hover_byte() > 1 && ui.active_widget() == id && ui.click_byte() != 2)
            {
                continue;
            }
            if !context.is_active() {
                continue;
            }
            if !is_selected && ui.hover_byte() == 1 && ui.click_byte() != 0 {
                commands.queue(move |world: &mut World| {
                    world.send_event(crate::systems::wall::wall_highlight::HighlightWallCmd {
                        wall: wall_id,
                        ty: 2,
                    });
                });
            }
            let intersection = context.intersecting();
            let phase = if ui.hover_byte() == 1 {
                2
            } else {
                context.dragged()
            };
            output.write(DoUiEditRectangleDims {
                intersection,
                wall: wall_id,
                rectangle: rect,
                edge: edge as u64,
                phase,
            });
            if !is_selected {
                let (a, b) = rect.get_edge_pts(edge);
                let mid = (a + b) * 0.5;
                let curve = ::utils::curve::Curve2::try_new(vec![a - mid, b - mid], false)
                    .expect("edge curve");
                let y = ui.intersection_point().y;
                active.0 = Some(RectangleActiveEdge {
                    curve,
                    widget: id,
                    position: glam::Vec3::new(mid.x, y, mid.y),
                });
            }
        }
    }
}

#[cfg(test)]
mod native_widget_tests {
    use super::*;
    use crate::resources::walls::public_wall_state::PublicWallState;
    use crate::resources::walls::wall_trigger_volume::WallTriggerVolume;
    use crate::resources::walls::{WallState, publication_source::TerrainHeightGrid512};
    use crate::systems::ui::resources::widget_params::WidgetGeometry;
    use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    use utils::geometry::Rectangle2d;

    #[test]
    fn widget_id_matches_original_hash_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-dims-widget-hash-fixture.json"))).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let wall = u64::from_str_radix(case["wall"].as_str().unwrap(), 16).unwrap();
            let edge = case["edge"].as_u64().unwrap() as usize;
            let expected = u64::from_str_radix(case["widget_id"].as_str().unwrap(), 16).unwrap();
            assert_eq!(
                edge_widget_id(crate::resources::walls::WallId(wall), edge).0,
                expected
            );
        }
    }

    #[test]
    fn selected_and_unselected_geometry_uses_published_wall_and_trigger() {
        let rect = Rectangle2d::new(Vec2::ZERO, Vec2::splat(4.0), Vec2::X);
        let mut authority = WallState::new(
            crate::systems::wall::shapes::to_authority_curve(&rect.as_curve2()),
            4.0,
            0,
            0,
            false,
        );
        authority.snapshot.source.shape =
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_snapshot();
        authority.snapshot.source.curve_kind_tag = 1;
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let mut wall = PublicWallState::collect(&authority, &grid).unwrap();
        let paths =
            crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
                std::array::from_fn(|_| std::sync::Arc::from(vec![0u8; 512 * 512])),
                true,
            )
            .unwrap();
        wall.prepare_paths(&paths).unwrap();
        let mut trigger = WallTriggerVolume::default();
        trigger.recompute(&wall).unwrap();
        assert!(matches!(
            edge_widget_geometry(&wall, 0, false, None),
            WidgetGeometry::Empty
        ));
        let WidgetGeometry::TriangleMesh(tris) =
            edge_widget_geometry(&wall, 0, false, Some(&trigger))
        else {
            panic!("range mesh")
        };
        assert!(!tris.is_empty());
        let WidgetGeometry::TriangleMesh(box_tris) =
            edge_widget_geometry(&wall, 0, true, Some(&trigger))
        else {
            panic!("selected box")
        };
        assert_eq!(box_tris.len(), 12);

        use crate::resources::walls::{
            InnerWalls,
            public_walls::{PublicWalls, WallEntities, WallPublicationDirty, WallSpatialHash},
        };
        use crate::systems::mode_manager::{AppMode, EditSubject};
        let mut inner = InnerWalls::new();
        let id = inner.new_wall_from_curve(authority);
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &grid,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        public.get_mut(id).unwrap().prepare_paths(&paths).unwrap();
        let mut world = World::new();
        world.init_resource::<Events<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
        world.insert_resource(inner);
        world.insert_resource(public);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(id.0)));
        world.init_resource::<WallEntities>();
        world.insert_resource(crate::systems::ui::resources::ui_state::UiState::at_startup());
        world
            .init_resource::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>();
        world.insert_resource(crate::resources::mouse_terrain_raycast::CameraCursorRay {
            origin: glam::Vec3::new(0.0, 1.0, 10.0),
            dir: -glam::Vec3::Z,
        });
        world.init_resource::<UiEditRectangleActiveWidgetResource>();
        world.init_resource::<Events<DoUiEditRectangleDims>>();
        world.init_resource::<Events<crate::systems::camera_rig::main_camera::MainCameraMoved>>();
        let mut producer = Schedule::default();
        producer.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        producer.add_systems(ui_edit_rectangle_dims);
        producer.run(&mut world);
        let widget = edge_widget_id(id, 0);
        let params = world
            .resource::<crate::systems::ui::resources::ui_state::UiState>()
            .widget(widget)
            .unwrap();
        let Some(WidgetGeometry::TriangleMesh(mesh)) = &params.geometry else {
            panic!("selected edge box")
        };
        let center = mesh
            .iter()
            .flat_map(|t| t.points)
            .fold(glam::Vec3::ZERO, |sum, p| sum + p)
            / (mesh.len() * 3) as f32;
        let inverse = glam::Mat4::from_translation(center + glam::Vec3::Z * 10.0);
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse.inverse());
        camera.write_matrix(0xc0, &inverse);
        world.insert_resource(camera);
        world.insert_resource(crate::resources::window::WindowSize::new(1280, 720));
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(
            Vec2::new(640.0, 360.0),
        ));
        world.insert_resource(crate::resources::read_mouse_depth_value::MouseDepthValue {
            view_depth: 100.0,
            ..Default::default()
        });
        world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
        world.init_resource::<crate::input::action_set::ActionSetStates>();
        world.init_resource::<Events<crate::resources::window::WindowFocusEvent>>();
        world.init_resource::<Events<crate::resources::events::EscapeMenuJustOpened>>();
        world.init_resource::<Events<crate::resources::events::ShutdownGameCmd>>();
        world.init_resource::<Events<crate::systems::brush_preview::OverwriteBrushPreview>>();
        world.init_resource::<Events<crate::resources::mouse_capture::OsCursorVisibilityCmd>>();
        world.init_resource::<Events<crate::systems::ui::resources::widget_params::PlayUiSound>>();
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut input = Schedule::default();
        input.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        input.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        input.run(&mut world);
        producer.run(&mut world);
        let events: Vec<_> = world
            .resource_mut::<Events<DoUiEditRectangleDims>>()
            .drain()
            .collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].wall, id);
        assert_eq!(events[0].edge, 0);
        assert_eq!(events[0].phase, 2);
        // Feed the actual picker intersection into the transaction system.
        world.init_resource::<RectangleEditUiState>();
        world.init_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
        world
            .init_resource::<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>(
            );
        world.insert_resource(crate::resources::glade_border::GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
            ),
        ));
        world.init_resource::<Events<EditRectangleCmd>>();
        world.init_resource::<Events<super::super::super::FinishEditRectangleCmd>>();
        world.init_resource::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>();
        world.init_resource::<Events<crate::resources::glade_border::DisplayOutOfBorder>>();
        world.init_resource::<Events<crate::resources::glade_border::FeedbackInputOutOfBorder>>();
        let mut transaction = Schedule::default();
        transaction.add_systems(do_ui_edit_rectangle_dims);
        world.insert_resource(
            crate::resources::history::History::<()>::after_observed_future_clear(),
        );
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustPressed;
        input.run(&mut world);
        let mut history_start = Schedule::default();
        history_start
            .add_systems(crate::systems::history::recording::history_start_recording::<()>);
        history_start.run(&mut world);
        assert!(
            world
                .resource::<crate::resources::history::History<()>>()
                .is_recording()
        );
        producer.run(&mut world);
        transaction.run(&mut world);
        assert_eq!(
            u32::from_le_bytes(
                world.resource::<RectangleEditUiState>().0[..4]
                    .try_into()
                    .unwrap()
            ),
            2
        );
        assert_eq!(
            read_rectangle(&world.resource::<RectangleEditUiState>().0[24..48]),
            rect
        );
        world
            .resource_mut::<crate::resources::mouse_terrain_raycast::CursorPositionSS>()
            .0
            .x += 100.0;
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::Held;
        input.run(&mut world);
        producer.run(&mut world);
        transaction.run(&mut world);
        assert!(
            world
                .resource_mut::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>()
                .drain()
                .any(|event| !event.finished)
        );
        assert!(!world.resource::<Events<EditRectangleCmd>>().is_empty());
        world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
        world.init_resource::<Events<crate::systems::audio::PlayBuildingAudio>>();
        world.init_resource::<bevy_time::Time>();
        let mut apply = Schedule::default();
        apply.add_systems(super::super::edit_rectangle::edit_rectangle);
        apply.run(&mut world);
        let changed = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(id)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert_ne!(changed, rect);
        world.resource_mut::<Events<EditRectangleCmd>>().clear();
        world.init_resource::<Events<super::super::finish_edit_rectangle::OnRectangleEdited>>();
        world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding>>();
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustReleased;
        input.run(&mut world);
        producer.run(&mut world);
        transaction.run(&mut world);
        let mut finish_schedule = Schedule::default();
        finish_schedule
            .add_systems(super::super::finish_edit_rectangle::finish_edit_rectangle::<()>);
        finish_schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<crate::resources::history::History<()>>()
                .pending()
                .unwrap()
                .edits
                .len(),
            1
        );
        assert!(
            world
                .resource_mut::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>()
                .drain()
                .any(|event| event.finished)
        );
        world.init_resource::<crate::resources::stairs::stairs_state::StairsState>();
        world.init_resource::<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
        world
            .init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>>();
        world.init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>>();
        world
            .init_resource::<Events<crate::systems::history::recording::FinishInputRecordingCmd>>();
        world.init_resource::<Events<crate::systems::backend::input::input_record_replay::InputReplayCommand>>();
        let mut history_finish = Schedule::default();
        history_finish
            .add_systems(crate::systems::history::recording::history_finish_recording_system::<()>);
        history_finish.run(&mut world);
        assert!(
            !world
                .resource::<crate::resources::history::History<()>>()
                .is_recording()
        );
        world.insert_resource(
            crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                grid.clone(),
            ),
        );
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
        world.init_resource::<crate::resources::time::GameTime>();
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 1.0;
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.init_resource::<Events<crate::systems::mode_manager::HistoryCommand>>();
        world.init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>();
        world.init_resource::<Events<crate::systems::history::history_process::NewReplayCommandEvent>>();
        world.init_resource::<Events<crate::systems::audio::PlayReplayAudio>>();
        world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
        world.init_resource::<Events<crate::systems::history::HistoryReplayFailed>>();
        world
            .init_resource::<Events<crate::systems::history::replay_wall::WallHistoryReplayAudio>>(
            );
        world
            .init_resource::<Events<crate::systems::history::replay_wall::WallHistoryRestoreColor>>(
            );
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        world
            .init_resource::<Events<crate::systems::wall::adjust_wall_height::AdjustWallHeightCmd>>(
            );
        world.init_resource::<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();
        let mut replay = Schedule::default();
        replay.add_systems(
            (
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::pop_empty_replay_bucket,
            )
                .chain(),
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::UNDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        let restored = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(id)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert!(
            (restored.center - rect.center).length() < 0.001
                && (restored.size - rect.size).length() < 0.001
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        let redone = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(id)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert!(
            (redone.center - changed.center).length() < 0.001
                && (redone.size - changed.size).length() < 0.001
        );
        let before = world.resource::<RectangleEditUiState>().0;
        let mut missing = events[0].clone();
        missing.wall = crate::resources::walls::WallId(u64::MAX);
        missing.phase = 3;
        world.send_event(missing);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| transaction.run(&mut world)))
                .is_err()
        );
        assert_eq!(world.resource::<RectangleEditUiState>().0, before);
        assert_eq!(
            world
                .resource_mut::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>()
                .drain()
                .count(),
            0
        );
        assert_eq!(
            world
                .resource_mut::<Events<EditRectangleCmd>>()
                .drain()
                .count(),
            0
        );
        let entity = world.spawn(trigger).id();
        world.resource_mut::<WallEntities>().0.insert(id, entity);
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::None);
        world.insert_resource(crate::systems::ui::resources::ui_state::UiState::at_startup());
        world.init_resource::<Events<crate::systems::wall::wall_highlight::HighlightWallCmd>>();
        producer.run(&mut world);
        let params = world
            .resource::<crate::systems::ui::resources::ui_state::UiState>()
            .widget(widget)
            .unwrap();
        let Some(WidgetGeometry::TriangleMesh(mesh)) = &params.geometry else {
            panic!("ordinary edge mesh")
        };
        let center = mesh
            .iter()
            .flat_map(|t| t.points)
            .fold(glam::Vec3::ZERO, |sum, p| sum + p)
            / (mesh.len() * 3) as f32;
        let inverse = glam::Mat4::from_translation(center + glam::Vec3::Z * 10.0);
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse.inverse());
        camera.write_matrix(0xc0, &inverse);
        world.insert_resource(camera);
        input.run(&mut world);
        producer.run(&mut world);
        let highlighted: Vec<_> = world
            .resource_mut::<Events<crate::systems::wall::wall_highlight::HighlightWallCmd>>()
            .drain()
            .collect();
        assert_eq!(highlighted.len(), 1);
        assert_eq!((highlighted[0].wall, highlighted[0].ty), (id, 2));
        assert!(
            world
                .resource::<UiEditRectangleActiveWidgetResource>()
                .0
                .is_some()
        );
    }
}

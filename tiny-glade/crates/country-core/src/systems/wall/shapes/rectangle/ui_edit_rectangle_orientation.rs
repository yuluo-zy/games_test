//! B `systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs`:
//! rectangle orientation editing.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs.md`
//! (O, 0x140ad8000 / do_ui_edit_rectangle_rotate 0x140ad8be0). The rotation
//! angle uses the fast 8-coefficient acos polynomial; the sign comes from the
//! cross product. Wip variant 1 stores the original and current rectangles.
//! ShowPinned and host schedule ordering remain open.

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::finish_edit_rectangle::RectangleEditUiState;
use utils::geometry::intersect::Triangle;

/// B 0x14083d3f0: geometry for a corner rotation widget.
fn corner_widget_geometry(
    wall: &crate::resources::walls::public_wall_state::PublicWallState,
    corner: usize,
    selected: bool,
) -> crate::systems::ui::resources::widget_params::WidgetGeometry {
    use crate::systems::ui::resources::widget_params::WidgetGeometry;
    let rect = crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(wall.shape())
        .as_rectangle();
    let point = rect.get_point(corner);
    let points = &wall
        .prepared_path
        .as_ref()
        .expect("rectangle corner requires prepared path")
        .positions;
    let coord = ::utils::curve::approx_curve_coord(points, point, |p| Vec2::new(p[0], p[2]));
    let i = coord.segment;
    let t = coord.t;
    let blend = |a: f32, b: f32| b * t + a * (1.0 - t);
    let y = blend(points[i][1], points[i + 1][1]) + 1.0;
    if selected {
        let [a, b] = rect.get_edge_dirs_from_corner_id(corner);
        let pairs = [(a, Vec2::new(a.y, -a.x)), (b, Vec2::new(-b.y, b.x))];
        let mut triangles = Vec::with_capacity(24);
        for (along, across) in pairs {
            let p = point + along * f32::from_bits(0xbdccccc8) + across * 0.7;
            let transform = crate::utils::components::transform::Transform {
                translation: Vec3::new(p.x, y + 0.1, p.y),
                rotation: ::utils::xz_rotation_bad(along),
                scale: Vec3::ONE,
            };
            triangles.extend(box_trigger_volume(0.7, 0.5, 1.9, &transform));
        }
        WidgetGeometry::TriangleMesh(triangles)
    } else {
        let height =
            (wall.segment_heights[i + 1] - wall.segment_heights[i]) * t + wall.segment_heights[i];
        let normal = rect.get_corner_normal(corner);
        let center = point - normal * 0.3;
        let transform = crate::utils::components::transform::Transform {
            translation: Vec3::new(center.x, y + 0.5 * (height - y), center.y),
            rotation: glam::Quat::from_mat3(&rect.orientation_mat3()),
            scale: Vec3::ONE,
        };
        WidgetGeometry::TriangleMesh(box_trigger_volume(1.5, height - y, 1.5, &transform))
    }
}

/// B 0x140ad9f10: skewed box trigger volume.
///
/// Two input records are 4 `Vec3` corners each (0x30 B; fresh disasm copies
/// `+0x0`, `+0x8`, `+0xc`, `+0x14`, `+0x18`, `+0x20`, `+0x24`, `+0x2c`
/// verbatim). The output is 0xf0 B: eight corner positions (arg3's four then
/// arg2's four) followed by 36 `u32` triangle indices, each offset by `base`.
/// Every index is emitted with `LEA r9d,[r9+n]`, i.e. 32-bit wrapping.
pub fn skewed_box_trigger_vol(
    loop_a: &[Vec3; 4],
    loop_b: &[Vec3; 4],
    base: u32,
) -> ([Vec3; 8], [u32; 36]) {
    let vertices = [
        loop_b[0], loop_b[1], loop_b[2], loop_b[3], loop_a[0], loop_a[1], loop_a[2], loop_a[3],
    ];
    let b = base;
    let indices = [
        b,
        b.wrapping_add(1),
        b.wrapping_add(2),
        b.wrapping_add(2),
        b.wrapping_add(3),
        b,
        b.wrapping_add(4),
        b.wrapping_add(5),
        b.wrapping_add(6),
        b.wrapping_add(6),
        b.wrapping_add(7),
        b.wrapping_add(4),
        b.wrapping_add(3),
        b.wrapping_add(2),
        b.wrapping_add(6),
        b.wrapping_add(6),
        b.wrapping_add(7),
        b.wrapping_add(3),
        b.wrapping_add(1),
        b,
        b.wrapping_add(4),
        b.wrapping_add(4),
        b.wrapping_add(5),
        b.wrapping_add(1),
        b.wrapping_add(2),
        b.wrapping_add(1),
        b.wrapping_add(5),
        b.wrapping_add(5),
        b.wrapping_add(6),
        b.wrapping_add(2),
        b,
        b.wrapping_add(3),
        b.wrapping_add(7),
        b.wrapping_add(7),
        b.wrapping_add(4),
        b,
    ];
    (vertices, indices)
}

/// 原 B 140ada0e0：按六个面的原顶点与绕序生成盒体的十二个真实三角形。
/// 维度可为负、零或非有限值；原入口不修正这些输入。
pub fn box_trigger_volume(
    width: f32,
    height: f32,
    depth: f32,
    transform: &crate::utils::components::transform::Transform,
) -> Vec<Triangle> {
    let x0 = width * -0.5;
    let x1 = width * 0.5;
    let y0 = height * -0.5;
    let y1 = height * 0.5;
    let z0 = depth * -0.5;
    let z1 = depth * 0.5;
    // 原表保留每个面的独立顶点；不能改为八点拓扑后重排三角形。
    let positions = [
        Vec3::new(x0, y0, z1),
        Vec3::new(x1, y0, z1),
        Vec3::new(x1, y1, z1),
        Vec3::new(x0, y1, z1),
        Vec3::new(x0, y1, z0),
        Vec3::new(x1, y1, z0),
        Vec3::new(x1, y0, z0),
        Vec3::new(x0, y0, z0),
        Vec3::new(x1, y0, z0),
        Vec3::new(x1, y1, z0),
        Vec3::new(x1, y1, z1),
        Vec3::new(x1, y0, z1),
        Vec3::new(x0, y0, z1),
        Vec3::new(x0, y1, z1),
        Vec3::new(x0, y1, z0),
        Vec3::new(x0, y0, z0),
        Vec3::new(x1, y1, z0),
        Vec3::new(x0, y1, z0),
        Vec3::new(x0, y1, z1),
        Vec3::new(x1, y1, z1),
        Vec3::new(x1, y0, z1),
        Vec3::new(x0, y0, z1),
        Vec3::new(x0, y0, z0),
        Vec3::new(x1, y0, z0),
    ];
    const INDICES: [usize; 36] = [
        0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ];
    box_trigger_triangles(&INDICES, &transform.compute_matrix(), &positions)
}

/// 原内部映射闭包保留分量乘加顺序，第三点的最后一次加法操作数相反。
fn box_trigger_triangles(
    indices: &[usize],
    transform: &glam::Mat4,
    positions: &[Vec3; 24],
) -> Vec<Triangle> {
    let x_axis = transform.x_axis.truncate();
    let y_axis = transform.y_axis.truncate();
    let z_axis = transform.z_axis.truncate();
    let translation = transform.w_axis.truncate();
    indices
        .chunks(3)
        .map(|chunk| {
            let point = |slot: usize| {
                let local = positions[chunk[slot]];
                let xy = y_axis * local.y + x_axis * local.x;
                let xyz = z_axis * local.z + xy;
                if slot == 2 {
                    translation + xyz
                } else {
                    xyz + translation
                }
            };
            Triangle::new(point(0), point(1), point(2))
        })
        .collect()
}

#[derive(Event, Clone, Debug)]
pub struct DoUiEditRectangleRotate {
    pub intersection: [u8; 56],
    pub wall: crate::resources::walls::WallId,
    pub rectangle: ::utils::geometry::Rectangle2d,
    pub corner: u64,
    pub phase: u8,
}

#[derive(Resource, Default)]
pub struct ClosestCornerCache {
    pub corner: u64,
    pub dirty: bool,
}

fn corner_widget_id(
    wall: crate::resources::walls::WallId,
    corner: u64,
) -> crate::systems::ui::resources::ui_state::WidgetId {
    crate::systems::ui::resources::ui_state::WidgetId(
        ::utils::calculate_hash(&(wall.0, corner)) ^ 0xa8f64de710aa4d92,
    )
}

fn corner_cache_dirty(hover: u8, active_matches: bool, dragged: u8) -> bool {
    hover < 2 || !active_matches || dragged == 2
}

fn rotate_angle(a: Vec2, b: Vec2) -> f32 {
    a.normalize().angle_to(b.normalize())
}

fn smooth_rotation_delta(angle: f32, dt: f32) -> f32 {
    let smooth = angle * (1.0 - (-20.0 * dt).exp());
    let limited = if angle.is_nan() {
        f32::NAN
    } else {
        angle.abs().min(2.0 * dt).copysign(angle)
    };
    if smooth.abs() > limited.abs() {
        smooth
    } else {
        limited
    }
}

/// B 0x140ad8000: register corner widgets and forward the real intersection.
pub fn ui_edit_rectangle_orientation(
    mode: Res<crate::systems::mode_manager::AppMode>,
    ui: Res<crate::systems::ui::resources::ui_state::UiState>,
    ray: Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
    camera: Res<crate::systems::camera_rig::main_camera::MainCamera>,
    public: Res<crate::resources::walls::public_walls::PublicWalls>,
    mut camera_moved: EventReader<crate::systems::camera_rig::main_camera::MainCameraMoved>,
    mut cache: ResMut<ClosestCornerCache>,
    mut output: EventWriter<DoUiEditRectangleRotate>,
    mut commands: Commands,
) {
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::widget_params::WidgetParams;
    use crate::systems::wall::valid_enclosure::ShapeParameters;
    use ::utils::geometry::{Aabb2, Ray2};
    if mode.native_tag() >= 6 {
        cache.dirty = true;
        return;
    }
    let moved = camera_moved.read().count() != 0;
    let selected = match *mode {
        AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) => {
            Some(crate::resources::walls::WallId(id))
        }
        AppMode::Editing(EditSubject::None) => None,
        _ => return,
    };
    for (id, _) in public.all() {
        if selected.is_some_and(|wall| wall != id) {
            continue;
        }
        let wall = public.get(id).expect("iterated wall");
        if wall.shape().tag() != 1 {
            continue;
        }
        let rect = ShapeParameters::from_snapshot(wall.shape()).as_rectangle();
        let is_selected = selected == Some(id);
        if is_selected && cache.dirty {
            let eye = camera.eye_position();
            let eye = Vec2::new(eye.x, eye.z);
            let mut best = f32::INFINITY;
            for corner in 0..4 {
                let distance = (rect.get_point(corner) - eye).length();
                if distance < best {
                    best = distance;
                    cache.corner = corner as u64;
                }
            }
        }
        let (enter, exit) = Aabb2::from_points(rect.as_points2()).intersect_ray_interval(Ray2 {
            origin: Vec2::new(ray.origin.x, ray.origin.z),
            direction: Vec2::new(ray.dir.x, ray.dir.z),
        });
        let hits = 0.0 <= exit && enter <= exit;
        for corner in 0..4u64 {
            if is_selected && corner != cache.corner {
                continue;
            }
            let widget = corner_widget_id(id, corner);
            if !is_selected
                && !hits
                && !(ui.active_widget() == widget
                    && ui.hover_byte() != 0
                    && ui.intersection_tag() != 2)
            {
                continue;
            }
            let geometry = corner_widget_geometry(wall, corner as usize, is_selected);
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
            let context = ui.add_with_id(widget, params);
            cache.dirty = corner_cache_dirty(
                ui.hover_byte(),
                ui.active_widget() == widget,
                context.dragged(),
            );
            if moved
                && !is_selected
                && !(ui.hover_byte() > 1 && ui.active_widget() == widget && ui.click_byte() != 2)
            {
                continue;
            }
            if !context.is_active() {
                continue;
            }
            if !is_selected && ui.hover_byte() == 1 && ui.click_byte() == 1 {
                commands.queue(move |world: &mut World| {
                    world.send_event(crate::systems::wall::wall_highlight::HighlightWallCmd {
                        wall: id,
                        ty: 2,
                    });
                });
            }
            let phase = if ui.hover_byte() == 1 {
                2
            } else {
                context.dragged()
            };
            output.write(DoUiEditRectangleRotate {
                intersection: context.intersecting(),
                wall: id,
                rectangle: rect,
                corner,
                phase,
            });
        }
    }
}

/// B 0x140ad8be0; ShowPinned is pending restoration of its shared owner.
pub fn do_ui_edit_rectangle_rotate(
    mut input: EventReader<DoUiEditRectangleRotate>,
    (public, ray, border, time): (
        Res<crate::resources::walls::public_walls::PublicWalls>,
        Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
        Res<crate::resources::glade_border::GladeBorder>,
        Res<crate::resources::time::GameTime>,
    ),
    mut wip: ResMut<RectangleEditUiState>,
    (mut edits, mut finish, mut trees, mut display, mut feedback, mut pins): (
        EventWriter<super::super::EditRectangleCmd>,
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
    use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    let Some(event) = input.read().last() else {
        return;
    };
    if u32::from_le_bytes(event.intersection[..4].try_into().unwrap()) == 2 {
        return;
    }
    let wall = public
        .get(event.wall)
        .expect("rectangle rotate wall missing");
    let rect = event.rectangle;
    let corner = event.corner as usize;
    let f = |at| f32::from_le_bytes(event.intersection[at..at + 4].try_into().unwrap());
    let hit_y = f(0x28);
    let intersect = |height: f32| {
        let t = (height - ray.origin.y) / ray.dir.y;
        Vec2::new(ray.origin.x + t * ray.dir.x, ray.origin.z + t * ray.dir.z)
    };
    let write_rect =
        |bytes: &mut [u8], value| bytes.copy_from_slice(&Shape::Rectangle(value).to_record()[4..]);
    let read_rect = |bytes: &[u8]| {
        let mut record = [0u8; 28];
        record[..4].copy_from_slice(&1u32.to_le_bytes());
        record[4..].copy_from_slice(bytes);
        ShapeParameters::from_observed(record).as_rectangle()
    };
    match event.phase.wrapping_sub(2).min(3) {
        1 => {
            let angle = rotate_angle(
                rect.get_point(corner) - rect.center,
                intersect(hit_y) - rect.center,
            );
            wip.0[..4].copy_from_slice(&1u32.to_le_bytes());
            wip.0[4..8].copy_from_slice(&hit_y.to_le_bytes());
            wip.0[8..12].copy_from_slice(&angle.to_le_bytes());
            wip.0[16..24].copy_from_slice(&event.wall.0.to_le_bytes());
            wip.0[24..32].copy_from_slice(&event.corner.to_le_bytes());
            write_rect(&mut wip.0[32..56], rect);
            write_rect(&mut wip.0[56..80], rect);
        }
        2 => {
            assert_eq!(
                u32::from_le_bytes(wip.0[..4].try_into().unwrap()),
                1,
                "rectangle rotate requires active Wip"
            );
            let corner = u64::from_le_bytes(wip.0[24..32].try_into().unwrap()) as usize;
            let edit_wall = crate::resources::walls::WallId(u64::from_le_bytes(
                wip.0[16..24].try_into().unwrap(),
            ));
            let height = f32::from_le_bytes(wip.0[4..8].try_into().unwrap());
            let grab = f32::from_le_bytes(wip.0[8..12].try_into().unwrap());
            let current = read_rect(&wip.0[56..80]);
            let angle = rotate_angle(
                current.get_point(corner) - current.center,
                intersect(height) - current.center,
            ) - grab;
            let angle = smooth_rotation_delta(angle, time.delta);
            let rotate = |axis: Vec2, angle: f32| {
                let (sin, cos) = angle.sin_cos();
                Vec2::new(axis.x * cos - axis.y * sin, axis.x * sin + axis.y * cos)
            };
            let mut candidate = current;
            candidate.axis = rotate(current.axis, angle);
            if !border.is_shape_inside(BorderShape::Rectangle(candidate)) {
                display.write(DisplayOutOfBorder);
                feedback.write(FeedbackInputOutOfBorder);
                let mut axis = current.axis;
                let mut step = 0.0;
                while step < angle {
                    step += 0.1;
                    let mut trial = current;
                    trial.axis = rotate(current.axis, step);
                    if !border.is_shape_inside(BorderShape::Rectangle(trial)) {
                        break;
                    }
                    axis = trial.axis;
                }
                candidate.axis = axis;
            }
            pins.write(crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::Rectangle {
                rectangle: rect, bottom: wall.authority.elevation(), top: wall.max_y(), exclude_wall: Some(event.wall),
            });
            trees.write(HideTreesinAreaCmd {
                area: TreeHideArea::Rectangle(rect),
                height_limit: wall.authority.elevation(),
                expansion: 1.5,
                finished: false,
            });
            if (candidate.axis - current.axis).length() > 0.0 {
                let movement = (candidate.get_point(corner) - current.get_point(corner)).length();
                write_rect(&mut wip.0[56..80], candidate);
                let before = read_rect(&wip.0[32..56]);
                edits.write(super::super::EditRectangleCmd {
                    wall: edit_wall,
                    center: candidate.center,
                    dimensions: candidate.size,
                    orientation: candidate.axis,
                    position: Vec3::new(before.center.x, height, before.center.y),
                    movement,
                    rotating: true,
                });
            }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner_cache_marks_other_widget_and_hover_dirty() {
        assert!(corner_cache_dirty(1, true, 3));
        assert!(corner_cache_dirty(2, false, 3));
        assert!(corner_cache_dirty(2, true, 2));
        assert!(!corner_cache_dirty(2, true, 3));
    }

    #[test]
    fn rotation_angle_matches_original_instruction_fragment() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-rotate-fast-acos-fragment-fixture.json"))).unwrap();
        let inputs = [
            (Vec2::X, Vec2::X),
            (Vec2::X, Vec2::Y),
            (Vec2::X, -Vec2::Y),
            (Vec2::X, Vec2::new(-1.0, 1.0)),
            (Vec2::X, -Vec2::X),
            (Vec2::ZERO, Vec2::X),
            (Vec2::new(f32::NAN, 0.0), Vec2::X),
            (Vec2::new(1.0, -0.0), Vec2::new(1.0, -0.0)),
            (Vec2::new(3.0, 4.0), Vec2::new(5.0, 7.0)),
            (Vec2::new(3.0, 7.0), Vec2::new(-9.0, 4.0)),
            (Vec2::splat(1.0e-20), Vec2::new(1.0e-20, -1.0e-20)),
        ];
        for (case, (a, b)) in fixture["cases"].as_array().unwrap().iter().zip(inputs) {
            let expected = u32::from_str_radix(case["angle_bits"].as_str().unwrap(), 16).unwrap();
            assert_eq!(rotate_angle(a, b).to_bits(), expected, "{}", case["name"]);
        }
    }

    #[test]
    fn corner_widget_uses_published_path_for_both_geometries() {
        use crate::resources::walls::public_wall_state::PublicWallState;
        use crate::resources::walls::{WallState, publication_source::TerrainHeightGrid512};
        use crate::systems::ui::resources::widget_params::WidgetGeometry;
        use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let rect = ::utils::geometry::Rectangle2d::new(Vec2::ZERO, Vec2::splat(4.0), Vec2::X);
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
        for corner in 0..4 {
            let WidgetGeometry::TriangleMesh(selected) =
                corner_widget_geometry(&wall, corner, true)
            else {
                panic!("selected mesh")
            };
            let WidgetGeometry::TriangleMesh(ordinary) =
                corner_widget_geometry(&wall, corner, false)
            else {
                panic!("ordinary mesh")
            };
            assert_eq!(selected.len(), 24);
            assert_eq!(ordinary.len(), 12);
            assert!(
                selected
                    .iter()
                    .chain(&ordinary)
                    .flat_map(|t| t.points)
                    .all(|p| p.is_finite())
            );
        }
    }

    #[test]
    fn rotate_schedule_captures_wip_and_releases() {
        use crate::resources::walls::{
            InnerWalls, WallState,
            public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash},
            publication_source::TerrainHeightGrid512,
        };
        use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let rect = ::utils::geometry::Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(4.0));
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
        let mut walls = InnerWalls::new();
        let wall = walls.new_wall_from_curve(authority);
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut walls,
            &grid,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        let paths =
            crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
                std::array::from_fn(|_| std::sync::Arc::from(vec![0u8; 512 * 512])),
                true,
            )
            .unwrap();
        public.get_mut(wall).unwrap().prepare_paths(&paths).unwrap();
        let mut world = World::new();
        world.init_resource::<Events<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
        world.insert_resource(public);
        world.insert_resource(walls);
        world.insert_resource(crate::resources::mouse_terrain_raycast::CameraCursorRay {
            origin: Vec3::new(-2.0, 10.0, -2.0),
            dir: -Vec3::Y,
        });
        world.insert_resource(crate::resources::glade_border::GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(100.0)),
            ),
        ));
        world.init_resource::<crate::resources::time::GameTime>();
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 0.1;
        world.init_resource::<RectangleEditUiState>();
        world.init_resource::<Events<DoUiEditRectangleRotate>>();
        world.insert_resource(crate::systems::mode_manager::AppMode::Editing(
            crate::systems::mode_manager::EditSubject::Wall(wall.0),
        ));
        world.insert_resource(crate::systems::ui::resources::ui_state::UiState::at_startup());
        world.insert_resource(crate::systems::camera_rig::main_camera::MainCamera::new(
            1.0,
        ));
        world.init_resource::<ClosestCornerCache>();
        world.init_resource::<Events<crate::systems::camera_rig::main_camera::MainCameraMoved>>();
        let mut producer = Schedule::default();
        producer.add_systems(ui_edit_rectangle_orientation);
        producer.run(&mut world);
        let widget = corner_widget_id(wall, 0);
        assert!(
            world
                .resource::<crate::systems::ui::resources::ui_state::UiState>()
                .widget(widget)
                .is_some()
        );
        let params = world
            .resource::<crate::systems::ui::resources::ui_state::UiState>()
            .widget(widget)
            .unwrap();
        let geometry = params.geometry.as_ref().unwrap();
        let crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(mesh) =
            geometry
        else {
            panic!("corner mesh")
        };
        let center = mesh[..12]
            .iter()
            .flat_map(|t| t.points)
            .fold(Vec3::ZERO, |a, p| a + p)
            / 36.0;
        let view = glam::Mat4::look_at_rh(center + Vec3::Y * 10.0, center, Vec3::Z).inverse();
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &view.inverse());
        camera.write_matrix(0xc0, &view);
        world.insert_resource(camera);
        world.insert_resource(crate::resources::window::WindowSize::new(100, 100));
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(
            Vec2::splat(50.0),
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
        let mut picker = Schedule::default();
        picker.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        picker.run(&mut world);
        assert_eq!(
            world
                .resource::<crate::systems::ui::resources::ui_state::UiState>()
                .active_widget(),
            widget
        );
        producer.run(&mut world);
        let picked: Vec<_> = world
            .resource_mut::<Events<DoUiEditRectangleRotate>>()
            .drain()
            .collect();
        assert_eq!(picked.len(), 1);
        assert_eq!(picked[0].phase, 2);
        assert_eq!(picked[0].wall, wall);
        world.init_resource::<Events<super::super::super::EditRectangleCmd>>();
        world.init_resource::<Events<super::super::super::FinishEditRectangleCmd>>();
        world.init_resource::<Events<crate::resources::tree_storage::HideTreesinAreaCmd>>();
        world.init_resource::<Events<crate::resources::glade_border::DisplayOutOfBorder>>();
        world.init_resource::<Events<crate::resources::glade_border::FeedbackInputOutOfBorder>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(do_ui_edit_rectangle_rotate);
        world.insert_resource(
            crate::resources::history::History::<()>::after_observed_future_clear(),
        );
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustPressed;
        picker.run(&mut world);
        let mut start_history = Schedule::default();
        start_history
            .add_systems(crate::systems::history::recording::history_start_recording::<()>);
        start_history.run(&mut world);
        producer.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(
            u32::from_le_bytes(
                world.resource::<RectangleEditUiState>().0[..4]
                    .try_into()
                    .unwrap()
            ),
            1
        );
        world
            .resource_mut::<crate::resources::mouse_terrain_raycast::CameraCursorRay>()
            .origin = Vec3::new(2.0, 10.0, -2.0);
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::Held;
        picker.run(&mut world);
        producer.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<Events<super::super::super::EditRectangleCmd>>()
                .len(),
            1
        );
        world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
        world.init_resource::<Events<crate::systems::audio::PlayBuildingAudio>>();
        world.init_resource::<bevy_time::Time>();
        let mut apply = Schedule::default();
        apply.add_systems(super::super::edit_rectangle::edit_rectangle);
        apply.run(&mut world);
        let changed = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert_ne!(changed.axis, rect.axis);
        world.init_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
        world
            .init_resource::<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>(
            );
        world.init_resource::<Events<super::super::finish_edit_rectangle::OnRectangleEdited>>();
        world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding>>();
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustReleased;
        picker.run(&mut world);
        producer.run(&mut world);
        schedule.run(&mut world);
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
        world.insert_resource(
            crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                grid.clone(),
            ),
        );
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
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
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 1.0;
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
        let before = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert!(
            (before.axis - rect.axis).length() < 0.001
                && (before.center - rect.center).length() < 0.001
                && (before.size - rect.size).length() < 0.001
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        let after = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_rectangle();
        assert!(
            (after.axis - changed.axis).length() < 0.001
                && (after.center - changed.center).length() < 0.001
                && (after.size - changed.size).length() < 0.001
        );
        // A later event may identify another published wall/corner; the edit
        // command still targets the Wip wall and uses its saved corner.
        let other_state = WallState::new(
            crate::systems::wall::shapes::to_authority_curve(&rect.as_curve2()),
            4.0,
            0,
            0,
            false,
        );
        let other = world
            .resource_mut::<InnerWalls>()
            .new_wall_from_curve(other_state);
        world
            .resource_mut::<InnerWalls>()
            .update_wall_state_enclosure(
                other,
                ShapeParameters::from_shape(Shape::Rectangle(rect)).to_snapshot(),
            );
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.consume_with_paths(
                &mut world.resource_mut::<InnerWalls>(),
                &grid,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
        });
        world
            .resource_mut::<Events<super::super::super::EditRectangleCmd>>()
            .clear();
        world
            .resource_mut::<crate::resources::mouse_terrain_raycast::CameraCursorRay>()
            .origin = Vec3::new(-2.0, 10.0, -2.0);
        let mut cross = DoUiEditRectangleRotate {
            intersection: [0; 56],
            wall,
            rectangle: rect,
            corner: 0,
            phase: 3,
        };
        world.send_event(cross.clone());
        schedule.run(&mut world);
        world
            .resource_mut::<crate::resources::mouse_terrain_raycast::CameraCursorRay>()
            .origin = Vec3::new(2.0, 10.0, -2.0);
        cross.wall = other;
        cross.corner = 1;
        cross.phase = 4;
        world.send_event(cross);
        schedule.run(&mut world);
        let commands: Vec<_> = world
            .resource_mut::<Events<super::super::super::EditRectangleCmd>>()
            .drain()
            .collect();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].wall, wall);
        assert!(commands[0].orientation.y.abs() > 0.01);
    }

    #[test]
    fn skewed_box_volume_keeps_corner_order_and_fixed_index_pattern() {
        let a = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let b = [
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
        ];
        let (vertices, indices) = skewed_box_trigger_vol(&a, &b, 10);
        // arg3 (loop_b) first, then arg2 (loop_a).
        assert_eq!(vertices[0], b[0]);
        assert_eq!(vertices[3], b[3]);
        assert_eq!(vertices[4], a[0]);
        assert_eq!(vertices[7], a[3]);
        // First and last triangles.
        assert_eq!(&indices[..3], &[10, 11, 12]);
        assert_eq!(&indices[33..36], &[17, 14, 10]);
        // 12 triangles over 8 vertices, all within [base, base+8).
        assert_eq!(indices.len(), 36);
        assert!(indices.iter().all(|&i| (10..18).contains(&i)));
        // 32-bit wrapping like the native LEA.
        let (_, wrapped) = skewed_box_trigger_vol(&a, &b, u32::MAX);
        assert_eq!(wrapped[1], 0);
    }

    #[test]
    fn box_trigger_volume_transforms_triangle_positions() {
        let mut positions = [Vec3::ZERO; 24];
        positions[0] = Vec3::new(1.0, 2.0, 3.0);
        positions[1] = Vec3::new(4.0, 5.0, 6.0);
        positions[2] = Vec3::new(7.0, 8.0, 9.0);
        let shifted = glam::Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let triangles = box_trigger_triangles(&[0, 1, 2], &shifted, &positions);
        assert_eq!(triangles.len(), 1);
        assert_eq!(triangles[0].points[0], Vec3::new(11.0, 2.0, 3.0));
        assert_eq!(triangles[0].points[2], Vec3::new(17.0, 8.0, 9.0));
        // Two chunks -> two triangles.
        let two = box_trigger_triangles(&[0, 1, 2, 2, 1, 0], &glam::Mat4::IDENTITY, &positions);
        assert_eq!(two.len(), 2);
        assert_eq!(two[1].points[0], Vec3::new(7.0, 8.0, 9.0));
    }

    #[test]
    #[should_panic]
    fn box_trigger_volume_rejects_out_of_range_vertex() {
        let positions = [Vec3::ZERO; 24];
        let _ = box_trigger_triangles(&[0, 1, 24], &glam::Mat4::IDENTITY, &positions);
    }
    #[test]
    fn box_trigger_volume_has_all_native_faces_and_srt() {
        use crate::utils::components::transform::Transform;
        // 原24点/36索引独立标量lift在尺寸2/4/6下的十二个三角形。
        let expected = [
            [[-1., -2., 3.], [1., -2., 3.], [1., 2., 3.]],
            [[1., 2., 3.], [-1., 2., 3.], [-1., -2., 3.]],
            [[-1., 2., -3.], [1., 2., -3.], [1., -2., -3.]],
            [[1., -2., -3.], [-1., -2., -3.], [-1., 2., -3.]],
            [[1., -2., -3.], [1., 2., -3.], [1., 2., 3.]],
            [[1., 2., 3.], [1., -2., 3.], [1., -2., -3.]],
            [[-1., -2., 3.], [-1., 2., 3.], [-1., 2., -3.]],
            [[-1., 2., -3.], [-1., -2., -3.], [-1., -2., 3.]],
            [[1., 2., -3.], [-1., 2., -3.], [-1., 2., 3.]],
            [[-1., 2., 3.], [1., 2., 3.], [1., 2., -3.]],
            [[1., -2., 3.], [-1., -2., 3.], [-1., -2., -3.]],
            [[-1., -2., -3.], [1., -2., -3.], [1., -2., 3.]],
        ];
        let triangles = box_trigger_volume(2., 4., 6., &Transform::identity());
        assert_eq!(triangles.len(), 12);
        for (actual, points) in triangles.iter().zip(expected) {
            assert_eq!(actual.points, points.map(Vec3::from_array));
        }
        let transform = Transform {
            rotation: glam::Quat::from_xyzw(0., 1., 0., 0.),
            translation: Vec3::new(10., 20., 30.),
            scale: Vec3::new(2., 3., 4.),
        };
        let triangles = box_trigger_volume(2., 4., 6., &transform);
        for (actual, points) in triangles.iter().zip(expected) {
            assert_eq!(
                actual.points,
                points.map(|[x, y, z]| Vec3::new(10. - 2. * x, 20. + 3. * y, 30. - 4. * z))
            );
        }
    }

    #[test]
    fn box_trigger_volume_keeps_reflection_degeneracy_and_nonfinite_lanes() {
        use crate::utils::components::transform::Transform;
        let identity = Transform::identity();
        let positive = box_trigger_volume(2., 4., 6., &identity);
        let reflected = box_trigger_volume(-2., 4., 6., &identity);
        for (actual, original) in reflected.iter().zip(&positive) {
            assert_eq!(
                actual.points,
                original.points.map(|p| Vec3::new(-p.x, p.y, p.z))
            );
        }
        let flat = box_trigger_volume(0., 4., 6., &identity);
        assert_eq!(flat.len(), 12);
        assert!(flat.iter().flat_map(|t| t.points).all(|p| p.x == 0.));
        let nonfinite = box_trigger_volume(f32::INFINITY, 4., 6., &identity);
        assert_eq!(nonfinite.len(), 12);
        assert!(
            nonfinite
                .iter()
                .flat_map(|t| t.points)
                .all(|p| !p.is_finite())
        );
    }

    #[test]
    #[should_panic]
    fn box_trigger_triangle_mapper_keeps_short_chunk_failure() {
        let _ = box_trigger_triangles(&[0, 1], &glam::Mat4::IDENTITY, &[Vec3::ZERO; 24]);
    }
}

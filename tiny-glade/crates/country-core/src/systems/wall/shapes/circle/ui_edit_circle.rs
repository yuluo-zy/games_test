//! B `systems/wall/shapes/circle/ui_edit_circle.rs` — `ui_edit_circle` (VA `0x14165d460`, 30 params).
//!
//! Spec §6. The original Normal and focused-circle registration paths use the
//! existing wall trigger, UiState and projection owners. Typed Local fields
//! retain the observed idle/start data. Held/release, decorator transactions
//! and full-circle factories are still partial; registration is not leaf acceptance.

use bevy_ecs::prelude::*;

use glam::Vec2;

use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder};
use crate::resources::history::History;
use crate::resources::mouse_terrain_raycast::CursorPositionSS;
use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
use crate::resources::time::GameTime;
use crate::resources::tree_storage::HideTreesinAreaCmd;
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use crate::resources::walls::public_walls::{PublicWalls, WallEntities};
use crate::resources::walls::wall_trigger_volume::WallTriggerVolume;
use crate::resources::walls::{InnerWalls, WallId, WallState};
use crate::resources::window::WindowSize;
use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};
use crate::systems::camera_rig::main_camera::{MainCamera, MainCameraMoved};
use crate::systems::mode_manager::{AppMode, EditSubject};
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter;
use crate::systems::wall::wall_highlight::HighlightWallCmd;

/// Radius-change history threshold (0x3c23d70a @0x142960674) — S5 release decision.
pub const CHANGE_THRESHOLD: f32 = 0.01;
/// B 0x14165f5ac passes 0.5 in XMM2 to `Circle2d::as_curve2`.
const CIRCLE_SAMPLE_STEP: f32 = 0.5;
/// S2 radius bounds (native 0.9 / 8.0).
pub const CIRCLE_MIN_RADIUS: f32 = f32::from_bits(0x3f66_6667);
pub const CIRCLE_MAX_RADIUS: f32 = 8.0;

/// S2 radius clamp: `clamp(0.9, 8.0)`.
pub fn clamp_circle_radius(radius: f32) -> f32 {
    radius.clamp(CIRCLE_MIN_RADIUS, CIRCLE_MAX_RADIUS)
}

/// S5 release decision: `|Δr| > 0.01` → commit; unordered restores.
pub fn circle_radius_changed(previous: f32, current: f32) -> bool {
    let delta = (current - previous).abs();
    delta > CHANGE_THRESHOLD
}

/// S4 apply: `Circle2d::new(center, r).as_curve2(0.5)` → `update_wall_state_curve` →
/// `update_wall_state_enclosure` (circle snapshot). Returns false if the curve update fails.
fn apply_circle_radius(
    inner: &mut InnerWalls,
    wall: WallId,
    circle: ::utils::geometry::Circle2d,
) -> bool {
    let curve = circle.as_curve2(CIRCLE_SAMPLE_STEP);
    inner.update_wall_state_curve(
        wall,
        crate::systems::wall::shapes::to_authority_curve(&curve),
    );
    let snapshot = crate::resources::walls::publication_fields::WallShapeSnapshot::from_observed(
        crate::systems::wall::valid_enclosure::ShapeParameters::from_shape(
            crate::systems::wall::valid_enclosure::Shape::Circle(circle),
        )
        .to_record(),
    );
    inner.update_wall_state_enclosure(wall, snapshot)
}

/// B `AnimatedCircleSignifierResource` (size 64): circle / rectangle / freehand signifier
/// state shared by `signifier_edit_circle_arrow` (0x140b35120) and `..._line` (0x140a64a40).
/// Registration default only writes `+0x38 = 2` (Freehand); the other fields are UNKNOWN
/// and zeroed here (not fabricated business defaults).
#[repr(C)]
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug)]
pub struct AnimatedCircleSignifierResource {
    /// +0x00..0x18: registration only writes `+0x38`; these bytes are UNKNOWN (zeroed here).
    pub unknown_00: [u8; 0x18],
    /// +0x18 published Circle2d radius (legacy field name retained).
    pub angle_base: f32,
    /// +0x1c..0x20 published Circle2d rotation bytes.
    pub pad_1c: [u8; 0x4],
    /// +0x20 centre/origin `Vec3` (x@0x20, y@0x24, z@0x28).
    pub position: glam::Vec3,
    /// +0x2c axis normal `Vec3` (x@0x2c, y@0x30, z@0x34).
    pub normal: glam::Vec3,
    /// +0x38 interaction: 0 = hover, 1 = active drag, 2 = absent.
    pub tag: u8,
    /// +0x39..0x40 padding.
    pub pad_39: [u8; 0x7],
}

impl Default for AnimatedCircleSignifierResource {
    fn default() -> Self {
        Self {
            unknown_00: [0; 0x18],
            angle_base: 0.0,
            pad_1c: [0; 0x4],
            position: glam::Vec3::ZERO,
            normal: glam::Vec3::ZERO,
            tag: 2,
            pad_39: [0; 0x7],
        }
    }
}

/// B `ui_edit_circle::CircleGrabParams` Local: optional starting circle at
/// native +4 and radial projection offset at +0x14. The offset is initialized
/// only by the start branch; an absent owned projection must not be read as 0.
/// This safe representation does not claim the native Local's default bytes.
#[derive(Default, Debug)]
pub struct CircleGrabParams {
    pub circle: Option<::utils::geometry::Circle2d>,
    pub radius_offset: Option<f32>,
}

/// B `CircleSignifierParams`: the radial XZ direction and world Y written
/// before projection. Native case 0/1 populate all three floats before use;
/// None models that no initialized projection has been supplied yet.
#[derive(Default, Debug)]
pub struct CircleSignifierParams {
    pub radial_axis: Option<(Vec2, f32)>,
}

/// B 0x141660880: owned full-circle mesh and post-build topology command.
pub fn show_full_circle_signifier(
    stream: &mut UiSignifierStream,
    wall: WallId,
    circle: ::utils::geometry::Circle2d,
    y: f32,
    line_width: f32,
) {
    use crate::systems::ui::signifiers::signifier_factory::{
        SignifierEntityBuilder, SignifierFactory, SignifierMesh,
    };
    let name = format!("circle{wall:?}_edit_{circle:?}_{y}{line_width}");
    let mut factory = SignifierFactory::from_closure(
        &name,
        std::sync::Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Points(
                circle.as_points3(0.5, y).collect(),
            ))
            .animation_duration(0.0)
            .custom_shader("vertex_color_shader")
            .build(meshes, shaders, commands);
            commands.entity(entity).insert(
                crate::render::pass_builder::draw_list::DrawTopology::Alt(line_width),
            );
            vec![entity]
        }),
    );
    factory.marker = 0x142c824f8;
    stream.write_factory(&factory);
}

/// B `ui_edit_circle` (`0x14165d460`, 30 params, PDB order).
///
/// Native registration and idle/start projection are restored; held radius,
/// border events, factories and the release transaction remain under review.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn ui_edit_circle<S: Send + Sync + 'static>(
    // Keep logical PDB order while grouping within Bevy's supported arity.
    (app_mode, mut inner, public, entities, trigger_volumes, archival, border, ui_state): (
        Res<AppMode>,
        ResMut<InnerWalls>,
        Res<PublicWalls>,
        Res<WallEntities>,
        Query<&WallTriggerVolume>,
        DecoratorArchivalResources,
        Res<GladeBorder>,
        Res<UiState>,
    ), // 1..8
    (
        mut signifiers,
        mut grab,
        mut signifier_params,
        mut animated,
        cursor_ss,
        window,
        camera,
        mut archivist,
    ): (
        ResMut<UiSignifierStream>,
        Local<CircleGrabParams>,
        Local<CircleSignifierParams>,
        ResMut<AnimatedCircleSignifierResource>,
        Res<CursorPositionSS>,
        Res<WindowSize>,
        Res<MainCamera>,
        ResMut<LiveDecoratorArchivist>,
    ), // 9..16
    (
        mut storage,
        game_time,
        time,
        mut camera_moved,
        mut out_of_border,
        mut feedback_out_of_border,
        mut highlight,
        mut hint,
    ): (
        ResMut<DecoratorStorage>,
        Res<GameTime>,
        Res<bevy_time::Time>,
        EventReader<MainCameraMoved>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
        EventWriter<HighlightWallCmd>,
        EventWriter<NotifyHintSystemShapeEditWasPerformed>,
    ), // 17..24
    (
        mut sheep_check,
        mut building_audio,
        mut material_audio,
        mut hide_trees,
        mut show_pins,
        mut history,
    ): (
        EventWriter<CmdCheckForSheepInsideBuilding>,
        EventWriter<PlayBuildingAudio>,
        EventWriter<RequestWallMaterialAudio>,
        EventWriter<HideTreesinAreaCmd>,
        EventWriter<ShowPinnedDecoClutter>,
        ResMut<History<S>>,
    ), // 25..30
) {
    use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams};
    use crate::systems::wall::valid_enclosure::ShapeParameters;

    let tag = app_mode.native_tag();
    animated.tag = 2;
    if tag > 5 {
        return;
    }
    let camera_has_moved = camera_moved.read().count() != 0;
    let mut selection = None;
    if tag == 0 {
        // Native Normal-mode entry registers the actual circle wall mesh. It
        // does not require an already selected wall or an injected edit event.
        for (wall, _) in public.all() {
            let published = public.get(wall).expect("iterated public wall");
            if published.shape().tag() != 0 {
                continue;
            }
            let geometry = entities
                .wall_entity(wall)
                .and_then(|entity| trigger_volumes.get(entity).ok())
                .filter(|trigger| trigger.is_valid())
                .map(|trigger| WidgetGeometry::TriangleMesh(trigger.edit_shape_tris()))
                .unwrap_or(WidgetGeometry::Empty);
            let mut params = WidgetParams::default().geometry(geometry);
            for (slot, kind) in params.sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
                slot[..4].copy_from_slice(&kind.to_le_bytes());
            }
            let context = ui_state.add(&format!("edit_circle_wall_{wall}"), params);
            if context.is_active() {
                highlight.write(HighlightWallCmd { wall, ty: 2 });
                let circle = ShapeParameters::from_snapshot(published.shape()).as_circle();
                selection = Some((wall, context.id, circle, ui_state.intersection_point().y));
                break;
            }
        }
    } else if let AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) = *app_mode {
        let wall = WallId(id);
        let Some(published) = public.get(wall) else {
            return;
        };
        if published.shape().tag() != 0 {
            return;
        }
        let circle = ShapeParameters::from_snapshot(published.shape()).as_circle();
        let y = published.max_y() - 0.5 * published.geometry.metrics.constrained_extra;
        let preview = ::utils::geometry::Circle2d {
            radius: circle.radius + 0.5,
            ..circle
        };
        let mut params = WidgetParams::default().geometry(WidgetGeometry::SphereCloud {
            centers: preview.as_points3(0.5, y).collect(),
            radius: 0.5,
        });
        params.occlude_by_world = false;
        let enabled = u32::from(!camera_has_moved);
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
        let context = ui_state.add(&format!("edit_circle_wall_focused_{wall}"), params);
        if context.is_active() {
            selection = Some((wall, context.id, circle, y));
        } else {
            show_full_circle_signifier(&mut signifiers, wall, preview, y, 3.0);
        }
    }
    let Some((wall, widget, circle, y)) = selection else {
        return;
    };
    let phase = ui_state
        .widget_context(widget)
        .dragged()
        .wrapping_sub(2)
        .min(3);
    if phase <= 1 {
        let intersection = ui_state.intersection_point();
        let offset = Vec2::new(intersection.x, intersection.z) - circle.center;
        let inverse_length = 1.0 / (offset.x * offset.x + offset.y * offset.y).sqrt();
        let direction = offset * inverse_length;
        signifier_params.radial_axis = Some((direction, y));
        grab.circle = Some(circle);
        if phase == 1 {
            use crate::utils::project_cursor_onto_ray::{
                ProportionalScreenSpace, ProportionalScreenSpaceCoord,
            };
            let screen = ProportionalScreenSpace::from_window_size(window.as_dims());
            let coordinate = ProportionalScreenSpaceCoord(
                screen.transform_screen_space(cursor_ss.0.x, cursor_ss.0.y),
            );
            let origin = glam::Vec3::new(circle.center.x, y, circle.center.y);
            let projected = coordinate.project_onto_ray(
                &screen,
                &camera.read_matrix(0x80),
                &camera.read_matrix(0),
                ::utils::geometry::Ray {
                    origin,
                    direction: glam::Vec3::new(direction.x, 0.0, direction.y),
                },
                [0.0, f32::MAX],
            );
            let delta = projected - origin;
            grab.radius_offset = Some(
                ((delta.z * delta.z + delta.y * delta.y) + delta.x * delta.x).sqrt()
                    - circle.radius,
            );
            archivist.archive_current_decorators_in_storages_with_resources(
                &[
                    DecoratorStorageAddr::Wall(wall),
                    DecoratorStorageAddr::Terrain,
                ],
                &mut storage,
                &archival,
                "edit_circle",
            );
            archivist.decorator_cull_allow_wall(wall, &mut storage);
            if let Some(terrain) = storage.get(DecoratorStorageAddr::Terrain) {
                archivist
                    .culling
                    .allowed_ids
                    .extend(terrain.decorators().keys().copied());
            }
        }
    } else if phase == 2 {
        use crate::resources::glade_border::BorderShape;
        use crate::utils::project_cursor_onto_ray::{
            ProportionalScreenSpace, ProportionalScreenSpaceCoord,
        };
        let (Some((direction, projection_y)), Some(offset)) =
            (signifier_params.radial_axis, grab.radius_offset)
        else {
            return;
        };
        let screen = ProportionalScreenSpace::from_window_size(window.as_dims());
        let coordinate = ProportionalScreenSpaceCoord(
            screen.transform_screen_space(cursor_ss.0.x, cursor_ss.0.y),
        );
        let origin = glam::Vec3::new(circle.center.x, projection_y, circle.center.y);
        let projected = coordinate.project_onto_ray(
            &screen,
            &camera.read_matrix(0x80),
            &camera.read_matrix(0),
            ::utils::geometry::Ray {
                origin,
                direction: glam::Vec3::new(direction.x, 0.0, direction.y),
            },
            [0.0, f32::MAX],
        );
        let delta = projected - origin;
        let published = public.get(wall).expect("selected wall remains published");
        hide_trees.write(HideTreesinAreaCmd {
            area: crate::resources::tree_storage::TreeHideArea::Circle(circle),
            height_limit: published.authority.elevation(),
            expansion: 1.5,
            finished: false,
        });
        show_pins.write(ShowPinnedDecoClutter::Circle {
            circle,
            flag: false,
            bottom: published.authority.elevation(),
            top: published.max_y(),
            exclude_wall: Some(wall),
        });
        let target = clamp_circle_radius(
            ((delta.z * delta.z + delta.y * delta.y) + delta.x * delta.x).sqrt() - offset,
        );
        if target != circle.radius {
            let mut candidate = circle;
            candidate.radius =
                ::utils::time_continuous_lerp(circle.radius, target, game_time.delta, 10.0);
            if !border.is_shape_inside(BorderShape::Circle(candidate)) {
                out_of_border.write(DisplayOutOfBorder);
                feedback_out_of_border.write(FeedbackInputOutOfBorder);
                candidate = match border.0 {
                    BorderShape::Rectangle(_) => match border
                        .search_lerped_shape_that_fits_into_glade(
                            BorderShape::Circle(circle),
                            BorderShape::Circle(candidate),
                        ) {
                        BorderShape::Circle(c) => c,
                        _ => unreachable!(),
                    },
                    BorderShape::Circle(boundary) => {
                        candidate.radius =
                            boundary.radius - boundary.center.distance(candidate.center);
                        candidate
                    }
                };
            }
            if (candidate.radius - circle.radius).abs() > 0.5 * game_time.delta {
                apply_circle_radius(&mut inner, wall, candidate);
                material_audio.write(RequestWallMaterialAudio {
                    id: wall.0,
                    position: origin,
                    kind: 4,
                    flag: 0,
                });
                let value = (candidate.radius - circle.radius).abs() / time.delta_secs() / 20.0;
                let value = if value < 0.0 { 0.0 } else { value };
                let value = if value > 1.0 { 1.0 } else { value };
                building_audio.write(PlayBuildingAudio {
                    id: wall.0,
                    position: origin,
                    editing_speed: value,
                    kind: 0,
                });
            }
        }
    } else if phase == 3 {
        {
            let previous = grab.circle.expect("circle release without grab");
            if !circle_radius_changed(previous.radius, circle.radius) {
                archivist.extract_and_restore_to_storage(&mut storage);
            } else {
                use crate::resources::history::EditType;
                use crate::resources::history::wall_edit::{
                    CompoundEditBuilder, MorphPremadeShapeEdit, WallHistoryEdit,
                };
                use crate::systems::wall::valid_enclosure::Shape;
                let current = inner
                    .get(wall)
                    .expect("circle edit wall missing")
                    .get_enclosure_params()
                    .expect("circle edit wall is not premade");
                let after = ShapeParameters::from_snapshot(current);
                assert_eq!(current.tag(), 0, "circle edit wall is not a circle");
                let current_circle = after.as_circle();
                if let Some(decorators) = storage.wall_decorators_mut(wall) {
                    let ids: Vec<_> = decorators.iter_maybe_mut().map(|(id, _)| id).collect();
                    for id in ids {
                        decorators.modified_ids.insert(id);
                        let info = decorators.decorators.get_mut(&id).unwrap();
                        match info.dst_tag() {
                            0 | 1 => {
                                let p = Vec2::new(
                                    f32::from_bits((info.dst_payload >> 32) as u32),
                                    f32::from_bits((info.u18 >> 32) as u32),
                                );
                                let p = current_circle.closest_point_on_perimeter(p);
                                info.dst_payload = (info.dst_payload & 0xffff_ffff)
                                    | ((p.x.to_bits() as u64) << 32);
                                info.u18 =
                                    (info.u18 & 0xffff_ffff) | ((p.y.to_bits() as u64) << 32);
                            }
                            2 => {}
                            _ => panic!("invalid circle wall decorator destination"),
                        }
                    }
                }
                let mut builder = CompoundEditBuilder::default();
                builder.new_edit_sequence();
                builder.next(WallHistoryEdit::MorphPremadeShape(
                    MorphPremadeShapeEdit::new(
                        crate::resources::walls::publication_fields::WallId(wall.0),
                        ShapeParameters::from_shape(Shape::Circle(previous)),
                        after,
                    ),
                ));
                history
                    .add(EditType::Wall(std::sync::Arc::new(builder.build())))
                    .expect("circle edit history recording");
                hint.write(NotifyHintSystemShapeEditWasPerformed);
                sheep_check.write(CmdCheckForSheepInsideBuilding { wall });
            }
            let published = public.get(wall).expect("selected wall remains published");
            hide_trees.write(HideTreesinAreaCmd {
                area: crate::resources::tree_storage::TreeHideArea::Circle(circle),
                height_limit: published.authority.elevation(),
                expansion: 1.5,
                finished: true,
            });
        }
    }
    let (direction, signifier_y) = signifier_params
        .radial_axis
        .expect("circle signifier was initialized by hover or grab");
    if tag == 0 {
        animated.unknown_00[..8].copy_from_slice(&widget.0.to_le_bytes());
        animated.unknown_00[8..16].copy_from_slice(&wall.0.to_le_bytes());
        animated.unknown_00[16..20].copy_from_slice(&circle.center.x.to_le_bytes());
        animated.unknown_00[20..24].copy_from_slice(&circle.center.y.to_le_bytes());
        animated.angle_base = circle.radius;
        animated.pad_1c = circle.rotation.to_le_bytes();
        let point = circle.center + direction * circle.radius;
        animated.position = glam::Vec3::new(point.x, signifier_y, point.y);
        animated.normal = glam::Vec3::new(direction.x, 0.0, direction.y);
        animated.tag = u8::from(ui_state.widget_context(widget).dragged() != 2);
    } else {
        show_full_circle_signifier(
            &mut signifiers,
            wall,
            ::utils::geometry::Circle2d {
                radius: circle.radius + 0.5,
                ..circle
            },
            signifier_y,
            5.0,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_circle_system_registers_normal_and_focused_geometry_without_injected_edit_events() {
        use crate::resources::walls::public_walls::{WallPublicationDirty, WallSpatialHash};
        use crate::resources::walls::{
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512,
        };
        use crate::systems::collision::world::RaycastWorld;
        use crate::systems::ui::resources::{ui_state::WidgetId, widget_params::WidgetGeometry};
        use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let circle = ::utils::geometry::Circle2d::new_rotated(Vec2::new(2.0, 3.0), 2.0, 0.4);
        let mut source = WallState::new(
            crate::systems::wall::shapes::to_authority_curve(&circle.as_curve2(0.2)),
            4.0,
            0,
            0,
            false,
        );
        source.snapshot.source.shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
        );
        source.snapshot.source.curve_kind_tag = 0;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(source);
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &grid,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        let published = public.get(wall).unwrap();
        let expected_y = published.max_y() - 0.5 * published.geometry.metrics.constrained_extra;
        let mut world = World::new();
        world.insert_resource(AppMode::Editing(EditSubject::None));
        world.insert_resource(inner);
        world.insert_resource(public);
        world.init_resource::<WallEntities>();
        world.init_resource::<RaycastWorld>();
        world.insert_resource(GladeBorder(
            crate::resources::glade_border::BorderShape::Circle(::utils::geometry::Circle2d::new(
                Vec2::ZERO,
                65.0,
            )),
        ));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<AnimatedCircleSignifierResource>();
        world.insert_resource(CursorPositionSS(Vec2::ZERO));
        world.insert_resource(WindowSize::new(1280, 720));
        world.insert_resource(MainCamera::new(1280.0 / 720.0));
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<GameTime>();
        world.insert_resource(bevy_time::Time::<()>::default());
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<HighlightWallCmd>>();
        world.init_resource::<Events<NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        world.init_resource::<Events<PlayBuildingAudio>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<ShowPinnedDecoClutter>>();
        world.insert_resource(History::<()>::after_observed_future_clear());
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        let mut schedule = Schedule::default();
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        schedule.add_systems(ui_edit_circle::<()>);
        for step in 0..3 {
            world.resource::<UiState>().clear_widgets();
            if step != 0 {
                *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::Wall(wall.0));
            }
            if step == 2 {
                world
                    .resource_mut::<Events<MainCameraMoved>>()
                    .send(MainCameraMoved);
            }
            schedule.run(&mut world);
            let name = if step == 0 {
                format!("edit_circle_wall_{wall}")
            } else {
                format!("edit_circle_wall_focused_{wall}")
            };
            let widget = world
                .resource::<UiState>()
                .widget(WidgetId::from_name(&name))
                .expect("circle widget must be registered from production resources");
            if step == 0 {
                assert!(
                    matches!(widget.geometry, Some(WidgetGeometry::Empty)),
                    "missing valid trigger produces Empty, not a fabricated sphere"
                );
                assert!(widget.occlude_by_world);
            } else {
                let Some(WidgetGeometry::SphereCloud { centers, radius }) = &widget.geometry else {
                    panic!("focused circle uses perimeter spheres");
                };
                assert_eq!(*radius, 0.5);
                assert!(!centers.is_empty());
                for p in centers {
                    assert_eq!(p.y, expected_y);
                    assert!((Vec2::new(p.x, p.z).distance(circle.center) - 2.5).abs() < 0.0001);
                }
                assert!(!widget.occlude_by_world);
            }
            let expected = if step == 2 { [0u32; 4] } else { [1, 2, 3, 3] };
            for (slot, kind) in widget.sounds.slots.iter().zip(expected) {
                assert_eq!(u32::from_le_bytes(slot[..4].try_into().unwrap()), kind);
            }
            assert!(world.resource::<Events<HighlightWallCmd>>().is_empty());
            assert!(world.resource::<History<()>>().pending().is_none());
        }
        let previews = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(previews.len(), 2);
        let crate::systems::ui::signifiers::UiSignifierEntry::Factory(factory) = &previews[0]
        else {
            panic!("full circle factory")
        };
        assert_eq!(
            factory.name(),
            Some(
                format!(
                    "circle{wall:?}_edit_{:?}_{expected_y}3",
                    ::utils::geometry::Circle2d {
                        radius: circle.radius + 0.5,
                        ..circle
                    }
                )
                .as_str()
            )
        );
        assert_eq!(factory.marker, 0x142c824f8);
        let meshes = crate::asset_libraries::mesh_library::AssetMeshLibrary::default();
        let shaders = crate::asset_libraries::shader_library::AssetShaderLibrary::new();
        shaders.add_raster(
            crate::render::shader::RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &crate::render::test_device::TestDevice(1),
            ),
            false,
        );
        let mut spawned = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &spawned);
        let entity = factory.wake(&meshes, &shaders, &mut commands)[0];
        drop(commands);
        queue.apply(&mut spawned);
        let animation = spawned
            .get::<crate::systems::ui::signifiers::animation::SignifierAnimation>(entity)
            .unwrap();
        assert_eq!(
            (animation.flag_8, animation.flag_9, animation.duration),
            (0, 1, 0.0)
        );
        assert_eq!(
            *spawned
                .get::<crate::render::pass_builder::draw_list::DrawTopology>(entity)
                .unwrap(),
            crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0)
        );

        // Prepare the actual published wall from four supplied raster buffers,
        // then recompute its real trigger. No triangle or UiInteraction is seeded.
        let paths =
            crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
                std::array::from_fn(|_| std::sync::Arc::from(vec![0u8; 512 * 512])),
                true,
            )
            .unwrap();
        let mut trigger = WallTriggerVolume::default();
        {
            let mut published = world.resource_mut::<PublicWalls>();
            let state = published.get_mut(wall).unwrap();
            state.prepare_paths(&paths).unwrap();
            trigger.recompute(state).unwrap();
        }
        assert!(trigger.is_valid());
        assert!(!trigger.edit_shape_tris().is_empty());
        let entity = world.spawn(trigger).id();
        world.resource_mut::<WallEntities>().0.insert(wall, entity);
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::None);
        world.insert_resource(UiState::at_startup());
        let inverse_view = glam::Mat4::from_translation(glam::Vec3::new(3.0, 2.0, 10.0));
        let mut camera = MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse_view.inverse());
        camera.write_matrix(0xc0, &inverse_view);
        world.insert_resource(camera);
        world.insert_resource(CursorPositionSS(Vec2::new(640.0, 360.0)));
        world.init_resource::<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
        world.insert_resource(crate::resources::read_mouse_depth_value::MouseDepthValue {
            view_depth: 100.0,
            ..Default::default()
        });
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
        let mut input_schedule = Schedule::default();
        input_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        input_schedule.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        schedule.run(&mut world);
        let selected = WidgetId::from_name(&format!("edit_circle_wall_{wall}"));
        let widget = world.resource::<UiState>().widget(selected).unwrap();
        assert!(
            matches!(&widget.geometry, Some(WidgetGeometry::TriangleMesh(tris)) if !tris.is_empty())
        );
        input_schedule.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), selected);
        assert_eq!(world.resource::<UiState>().hover_byte(), 1);
        schedule.run(&mut world);
        let animated_hover = *world.resource::<AnimatedCircleSignifierResource>();
        assert_eq!(animated_hover.tag, 0);
        assert_eq!(animated_hover.angle_base, circle.radius);
        assert_eq!(&animated_hover.unknown_00[..8], &selected.0.to_le_bytes());
        assert_eq!(&animated_hover.unknown_00[8..16], &wall.0.to_le_bytes());
        world.init_resource::<crate::asset_libraries::mesh_library::AssetMeshLibrary>();
        let mut consumers = Schedule::default();
        consumers.add_systems((
            super::super::signifiers::edit_arrow::signifier_edit_circle_arrow,
            super::super::signifiers::edit_line::signifier_edit_circle_line,
        ));
        consumers.run(&mut world);
        assert_eq!(
            world
                .resource_mut::<UiSignifierStream>()
                .take_entries()
                .len(),
            2
        );
        let highlighted: Vec<_> = world
            .resource_mut::<Events<HighlightWallCmd>>()
            .drain()
            .collect();
        assert_eq!(highlighted.len(), 1);
        assert_eq!((highlighted[0].wall, highlighted[0].ty), (wall, 2));
        assert!(world.resource::<History<()>>().pending().is_none());

        // Drive the start phase through the actual input owner, then verify the
        // wall and terrain snapshots and the wall culling allow-list.
        use crate::input::action_set::DigitalActionPhase;
        use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage
                .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
                .insert(DecoratorId(11), DecoratorInfo::default());
            let mut roof = DecoratorInfo::default();
            roof.u20 = 2 << 16;
            storage
                .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
                .insert(DecoratorId(13), roof);
            storage
                .get_or_insert_default(DecoratorStorageAddr::Terrain)
                .insert(DecoratorId(12), DecoratorInfo::default());
            storage
                .decorator_to_storage
                .insert(DecoratorId(11), DecoratorStorageAddr::Wall(wall));
            storage
                .decorator_to_storage
                .insert(DecoratorId(12), DecoratorStorageAddr::Terrain);
            storage
                .decorator_to_storage
                .insert(DecoratorId(13), DecoratorStorageAddr::Wall(wall));
            storage
                .get_mut(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .modified_ids
                .clear();
        }
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = DigitalActionPhase::JustPressed;
        input_schedule.run(&mut world);
        let mut start = Schedule::default();
        start.add_systems(crate::systems::history::recording::history_start_recording::<()>);
        start.run(&mut world);
        assert!(world.resource::<History<()>>().is_recording());
        assert_eq!(
            world
                .resource::<UiState>()
                .widget_context(selected)
                .dragged(),
            3
        );
        schedule.run(&mut world);
        assert_eq!(world.resource::<AnimatedCircleSignifierResource>().tag, 1);
        let archived = world.resource::<LiveDecoratorArchivist>();
        assert!(
            archived
                .extract_before_after(DecoratorStorageAddr::Wall(wall))
                .is_some()
        );
        assert!(
            archived
                .extract_before_after(DecoratorStorageAddr::Terrain)
                .is_some()
        );
        assert!(archived.is_decorator_cull_allowed(11));
        assert!(archived.is_decorator_cull_allowed(12));
        assert!(
            world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .decorators()
                .contains_key(&11)
        );
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = DigitalActionPhase::Held;
        world.resource_mut::<GameTime>().delta = 0.1;
        world.resource_mut::<CursorPositionSS>().0.x = 1100.0;
        input_schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<UiState>()
                .widget_context(selected)
                .dragged(),
            4
        );
        schedule.run(&mut world);
        let edited = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .unwrap();
        let edited_circle = ShapeParameters::from_snapshot(edited).as_circle();
        assert_ne!(edited_circle.radius.to_bits(), circle.radius.to_bits());
        assert_eq!(
            world
                .resource_mut::<Events<RequestWallMaterialAudio>>()
                .drain()
                .count(),
            1
        );
        // Publish the authoritative held update before releasing, as the real
        // frame chain does; release must record the old and new shapes.
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            let mut inner = world.resource_mut::<InnerWalls>();
            public.consume_with_paths(
                &mut inner,
                &grid,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
        });
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = DigitalActionPhase::JustReleased;
        input_schedule.run(&mut world);
        assert!(
            world
                .resource::<UiState>()
                .widget_context(selected)
                .dragged()
                < 2
        );
        schedule.run(&mut world);
        let pending = world.resource::<History<()>>();
        let edits = &pending.pending().unwrap().edits;
        assert!(
            matches!(&edits[0], crate::resources::history::EditType::Wall(edit) if matches!(&edit.groups[0][0], crate::resources::history::wall_edit::WallHistoryEdit::MorphPremadeShape(_)))
        );
        let info = world
            .resource::<DecoratorStorage>()
            .get(DecoratorStorageAddr::Wall(wall))
            .unwrap()
            .decorators()[&11];
        assert!(
            world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .modified_ids
                .contains(&13)
        );
        let pos = Vec2::new(
            f32::from_bits((info.dst_payload >> 32) as u32),
            f32::from_bits((info.u18 >> 32) as u32),
        );
        assert!((pos.distance(edited_circle.center) - edited_circle.radius).abs() < 0.0001);
        assert_eq!(
            world
                .resource_mut::<Events<CmdCheckForSheepInsideBuilding>>()
                .drain()
                .count(),
            1
        );
        assert_eq!(
            world
                .resource_mut::<Events<NotifyHintSystemShapeEditWasPerformed>>()
                .drain()
                .count(),
            1
        );
        assert!(
            world
                .resource_mut::<Events<HideTreesinAreaCmd>>()
                .drain()
                .any(|event| event.finished)
        );
        // Complete the canonical recording and replay owners, including the
        // nonempty decorator diff. An edit pending in History is not a commit.
        crate::systems::history::register_history_events(&mut world);
        world.init_resource::<crate::resources::walls::decorator::DecoratorIdGen>();
        world.init_resource::<crate::resources::walls::decorator::DeferredDecoratorOpEvents>();
        let mut finish = Schedule::default();
        finish
            .add_systems(crate::systems::history::recording::history_finish_recording_system::<()>);
        finish.run(&mut world);
        let history = world.resource::<History<()>>();
        assert!(!history.is_recording());
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(
            history.ledger().entries[0]
                .auxiliary
                .decorator_diff
                .as_ref()
                .is_some_and(|diff| !diff.is_empty())
        );
        world.insert_resource(
            crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                grid,
            ),
        );
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.resource_mut::<GameTime>().delta = 1.0;
        let mut replay = Schedule::default();
        #[cfg(feature = "native-physics")]
        replay.add_systems(
            (
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::replay_decorator::replay_decorator,
                crate::systems::history::replay_pop_bucket_system,
            )
                .chain(),
        );
        #[cfg(not(feature = "native-physics"))]
        replay.add_systems(
            (
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::replay_decorator::replay_decorator,
                crate::systems::history::replay_pop_bucket_cpu_system,
            )
                .chain(),
        );
        for (command, expected_circle, expected_decorator) in [
            (
                crate::systems::mode_manager::HistoryCommand::UNDO,
                circle,
                DecoratorInfo::default(),
            ),
            (
                crate::systems::mode_manager::HistoryCommand::REDO,
                edited_circle,
                info,
            ),
        ] {
            world.send_event(command);
            for _ in 0..400 {
                world.resource_mut::<DecoratorStorage>().mark_mutable();
                replay.run(&mut world);
            }
            let shape = ShapeParameters::from_snapshot(
                world
                    .resource::<InnerWalls>()
                    .get(wall)
                    .unwrap()
                    .get_enclosure_params()
                    .unwrap(),
            )
            .as_circle();
            assert_eq!(shape.center, expected_circle.center);
            assert_eq!(shape.radius.to_bits(), expected_circle.radius.to_bits());
            assert_eq!(shape.rotation.to_bits(), expected_circle.rotation.to_bits());
            let restored = world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .decorators()[&11];
            assert_eq!(restored, expected_decorator);
            assert!(
                world
                    .resource::<crate::resources::history::ReplayBuffer>()
                    .buckets
                    .is_empty()
            );
            assert!(
                world
                    .resource::<Events<crate::systems::history::HistoryReplayFailed>>()
                    .is_empty()
            );
        }
        // A release with an uninitialized Local is an original panic, not a no-op.
        world.resource_mut::<UiState>().replace_interaction(
            crate::systems::ui::resources::ui_state::UiInteraction {
                native_tag: 2,
                native_phase: 1,
                widget: selected,
            },
        );
        let mut fresh_schedule = Schedule::default();
        fresh_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        fresh_schedule.add_systems(ui_edit_circle::<()>);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || fresh_schedule.run(&mut world)
            ))
            .is_err()
        );
    }

    #[test]
    fn locals_and_resource_defaults() {
        let grab = CircleGrabParams::default();
        let _ = &grab;
        let signifier = CircleSignifierParams::default();
        let _ = &signifier;
        // AnimatedCircleSignifierResource default tag = 2 (Freehand) per registration.
        assert_eq!(AnimatedCircleSignifierResource::default().tag, 2);
        assert_eq!(std::mem::size_of::<AnimatedCircleSignifierResource>(), 64);
    }

    #[test]
    fn s0_numeric_helpers_clamp_and_threshold() {
        // S2 clamp bounds 0.9 / 8.0.
        assert_eq!(clamp_circle_radius(0.1).to_bits(), 0x3f66_6667);
        assert_eq!(clamp_circle_radius(100.0), 8.0);
        assert_eq!(clamp_circle_radius(4.0), 4.0);
        // S5 threshold 0.01 (unordered follows the native restore branch).
        assert!(!circle_radius_changed(2.0, 2.005));
        assert!(circle_radius_changed(2.0, 2.02));
        assert!(!circle_radius_changed(f32::NAN, 2.0));
    }

    #[test]
    fn circle_clamp_matches_executed_original_sse_fixtures_by_bits() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-analysis-20261028-clamp-fixture.json"
        ))).unwrap();
        assert_eq!(fixture["executed"], true);
        assert_eq!(
            fixture["original_B_sha256"],
            "cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323"
        );
        let cases = fixture["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 18);
        for case in cases {
            let input = u32::from_str_radix(case["input_bits"].as_str().unwrap(), 16).unwrap();
            let expected = u32::from_str_radix(case["output_bits"].as_str().unwrap(), 16).unwrap();
            assert_eq!(
                clamp_circle_radius(f32::from_bits(input)).to_bits(),
                expected,
                "original SSE input {input:08x}"
            );
        }
    }

    #[test]
    fn s4_apply_circle_updates_curve_and_enclosure_radius() {
        use crate::resources::roofs::geometry::surface::Curve2;
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 2.0, 0, 0, false));
        let circle = ::utils::geometry::Circle2d::new_rotated(Vec2::new(1.0, 1.0), 3.0, 0.4);
        let applied = apply_circle_radius(&mut inner, wall, circle);
        assert!(applied, "apply_circle_radius should update the wall curve");
        let params = inner
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .expect("circle enclosure params");
        let radius = crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(params)
            .as_circle()
            .radius;
        assert!((radius - 3.0).abs() < 1e-3, "enclosure radius {radius}");
        assert_eq!(
            crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(params)
                .as_circle()
                .rotation,
            0.4
        );
        for radius in [CIRCLE_MIN_RADIUS, 3.0, CIRCLE_MAX_RADIUS] {
            let input = ::utils::geometry::Circle2d::new_rotated(Vec2::new(1.0, 1.0), radius, 0.4);
            assert!(apply_circle_radius(&mut inner, wall, input));
            let expected = input.as_curve2(0.5);
            let actual = &inner.get(wall).unwrap().snapshot.source.curve.points;
            assert_eq!(actual.len(), expected.points.len());
            for (a, b) in actual.iter().zip(&expected.points) {
                assert_eq!(a[0].to_bits(), b.x.to_bits());
                assert_eq!(a[1].to_bits(), b.y.to_bits());
            }
        }
    }
}

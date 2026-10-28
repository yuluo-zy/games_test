//! B circle rotation interaction, 0x1409ac910, with owned gizmo factories.
use crate::resources::history::History;
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use crate::resources::walls::{InnerWalls, WallId, public_walls::PublicWalls};
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

#[derive(Default)]
pub struct RotateCircleCache {
    pub gizmo_offset: f32,
    pub cursor_angle: f32,
    pub initial_rotation: f32,
    pub grab: Vec3,
    pub state: u8,
}
#[derive(Default)]
pub struct CurrentAngleGizmo {
    pub active: bool,
    pub angle: ::utils::circular_f32::CircularF32,
}

/// Update both authoritative curve and enclosure using the native 0.5 step.
pub fn rotate_circle_wall(
    inner: &mut InnerWalls,
    wall: WallId,
    mut circle: ::utils::geometry::Circle2d,
    angle: f32,
) {
    circle.rotation = angle;
    inner.update_wall_state_curve(
        wall,
        super::super::to_authority_curve(&circle.as_curve2(0.5)),
    );
    inner.update_wall_state_enclosure(
        wall,
        ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot(),
    );
}

pub fn ui_rotate_circle<S: Send + Sync + 'static>(
    (mode, public, mut inner, camera, window, ray, ui, time): (
        Res<crate::systems::mode_manager::AppMode>,
        Res<PublicWalls>,
        ResMut<InnerWalls>,
        Res<crate::systems::camera_rig::main_camera::MainCamera>,
        Res<crate::resources::window::WindowSize>,
        Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
        Res<crate::systems::ui::resources::ui_state::UiState>,
        Res<crate::resources::time::GameTime>,
    ),
    (mut cache, mut angle_gizmo, mut storage, mut archivist, archival, mut history): (
        Local<RotateCircleCache>,
        Local<CurrentAngleGizmo>,
        ResMut<DecoratorStorage>,
        ResMut<LiveDecoratorArchivist>,
        DecoratorArchivalResources,
        ResMut<History<S>>,
    ),
    (mut rotated, mut roof, mut hint, mut audio, mut ui_sound, mut pins, mut signifiers): (
        EventWriter<crate::resources::events::OnCircleRotated>,
        EventWriter<crate::resources::roofs::commands::RecalculateRoofCmd>,
        EventWriter<crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,
        EventWriter<crate::systems::audio::PlayBuildingAudio>,
        EventWriter<crate::systems::ui::resources::widget_params::PlayUiSound>,
        EventWriter<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>,
        ResMut<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    ),
) {
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams};
    let selected = match *mode {
        AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) => WallId(id),
        _ => {
            angle_gizmo.active = false;
            return;
        }
    };
    let wall = public.get(selected).expect("circle rotation wall missing");
    if wall.shape().tag() != 0 {
        angle_gizmo.active = false;
        return;
    }
    let circle = ShapeParameters::from_snapshot(wall.shape()).as_circle();
    let mut best = 1.0e7f32;
    let mut point = Vec3::from_array(
        *wall
            .geometry
            .outline
            .first()
            .expect("circle rotation curve is empty"),
    );
    for p in wall.geometry.outline.iter() {
        let p = Vec3::from_array(*p);
        let screen = crate::utils::world_space_to_screen_space(
            p,
            window.width as f32,
            window.height as f32,
            &camera.read_matrix(0x80),
            &camera.read_matrix(0),
        );
        if screen.x < best {
            best = screen.x;
            point = p;
        }
    }
    let d = Vec2::new(point.x, point.z) - circle.center;
    let normal = d / (d.y * d.y + d.x * d.x).sqrt();
    let direction = Vec3::new(normal.x, 0.0, normal.y);
    let gizmo = point + direction;
    let transform = crate::utils::components::transform::Transform {
        translation: gizmo + direction * 0.3,
        rotation: ::utils::xz_rotation_bad(normal),
        scale: Vec3::ONE,
    };
    let mesh = super::super::rectangle::ui_edit_rectangle_orientation::box_trigger_volume(
        3.5, 0.5, 1.5, &transform,
    );
    let mut params = WidgetParams::default().geometry(WidgetGeometry::TriangleMesh(mesh));
    params.occlude_by_world = false;
    for (slot, kind) in params.sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
        slot[..4].copy_from_slice(&kind.to_le_bytes());
    }
    let context = ui.add(&format!("rotate_circle_{selected}"), params);
    let phase = context.dragged().wrapping_sub(2).min(3);
    let intersect = |height: f32| {
        let t = (height - ray.origin.y) / ray.dir.y;
        ray.origin + ray.dir * t
    };
    let polar = |p: Vec2| p.y.atan2(p.x);
    match phase {
        1 => {
            archivist.archive_current_decorators_in_storages_with_resources(
                &[DecoratorStorageAddr::Wall(selected)],
                &storage,
                &archival,
                "circle rotate",
            );
            archivist.decorator_cull_allow_wall(selected, &storage);
            let grab = intersect(gizmo.y);
            let cursor = polar(Vec2::new(grab.x, grab.z) - circle.center);
            cache.gizmo_offset = polar(Vec2::new(gizmo.x, gizmo.z) - circle.center) - cursor;
            cache.cursor_angle = cursor;
            cache.initial_rotation =
                ::utils::circular_f32::CircularF32::new(circle.rotation).as_f32();
            cache.grab = grab;
            cache.state = 0;
        }
        2 => {
            let grab = intersect(cache.grab.y);
            let angle = ::utils::circular_f32::CircularF32::new(
                cache.initial_rotation - cache.cursor_angle
                    + polar(Vec2::new(grab.x, grab.z) - circle.center),
            )
            .as_f32();
            if angle != circle.rotation {
                let distance =
                    ::utils::circular_f32::CircularF32::new(angle).distance(circle.rotation);
                let speed = distance / time.delta / 15.0;
                let speed = if speed >= 0.0 { speed } else { 0.0 };
                let speed = if speed <= 1.0 { speed } else { 1.0 };
                audio.write(crate::systems::audio::PlayBuildingAudio {
                    id: selected.0,
                    position: Vec3::new(circle.center.x, gizmo.y, circle.center.y),
                    editing_speed: speed,
                    kind: 2,
                });
                rotate_circle_wall(&mut inner, selected, circle, angle);
                if let Some(entity) = public.roof_entity(selected) {
                    roof.write(crate::resources::roofs::commands::RecalculateRoofCmd {
                        entity,
                        wall: selected,
                        kind: 2,
                    });
                }
            }
            rotated.write(crate::resources::events::OnCircleRotated(selected));
            pins.write(
                crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter::Circle {
                    circle,
                    flag: true,
                    bottom: wall.authority.elevation(),
                    top: wall.max_y(),
                    exclude_wall: Some(selected),
                },
            );
        }
        3 => {
            use crate::resources::history::{
                EditType,
                wall_edit::{MorphPremadeShapeEdit, WallCompoundHistoryEdit, WallHistoryEdit},
            };
            rotated.write(crate::resources::events::OnCircleRotated(selected));
            let mut before = circle;
            before.rotation = cache.initial_rotation;
            let edit = WallHistoryEdit::MorphPremadeShape(MorphPremadeShapeEdit::new(
                crate::resources::walls::publication_fields::WallId(selected.0),
                ShapeParameters::from_shape(Shape::Circle(before)),
                ShapeParameters::from_shape(Shape::Circle(circle)),
            ));
            history
                .add(EditType::Wall(std::sync::Arc::new(
                    WallCompoundHistoryEdit {
                        groups: vec![vec![edit]],
                    },
                )))
                .expect("circle rotate history recording");
            hint.write(crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed);
            cache.state = 1;
        }
        _ => {}
    }
    let target = if phase == 2 {
        let projected = intersect(cache.grab.y);
        ::utils::circular_f32::CircularF32::new(
            cache.gizmo_offset + polar(Vec2::new(projected.x, projected.z) - circle.center),
        )
    } else {
        ::utils::circular_f32::CircularF32::new(polar(Vec2::new(gizmo.x, gizmo.z) - circle.center))
    };
    let was_active = angle_gizmo.active;
    let previous_angle = angle_gizmo.angle;
    if was_active {
        angle_gizmo.angle = if phase == 2 {
            target
        } else {
            angle_gizmo
                .angle
                .lerp(target.as_f32(), 1.0 - (-10.0 * time.delta).exp())
        };
    } else {
        angle_gizmo.active = true;
        angle_gizmo.angle = target;
    }
    if was_active && cache.state == 1 {
        let distance = previous_angle.distance(angle_gizmo.angle.as_f32());
        if !(distance > 0.01) {
            ui_sound.write(crate::systems::ui::resources::widget_params::PlayUiSound {
                kind: 0x23,
                ..Default::default()
            });
            cache.state = 2;
        } else {
            let value = ::utils::scalar::inverse_lerp(0.4, 0.0, distance);
            let value = if value >= 0.0 { value } else { 0.0 };
            let value = if value <= 1.0 { value } else { 1.0 };
            let a = angle_gizmo.angle.as_f32();
            let position = Vec3::new(
                circle.center.x + (circle.radius + 1.0) * a.cos(),
                gizmo.y,
                circle.center.y + (circle.radius + 1.0) * a.sin(),
            );
            let mut payload = [0u8; 20];
            payload[..4].copy_from_slice(&value.to_le_bytes());
            for (i, v) in position.to_array().into_iter().enumerate() {
                payload[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
            }
            ui_sound.write(crate::systems::ui::resources::widget_params::PlayUiSound {
                kind: 0x22,
                _payload: payload,
            });
        }
    }
    let a = angle_gizmo.angle.as_f32();
    let position = Vec3::new(
        circle.center.x + (circle.radius + 1.0) * a.cos(),
        gizmo.y,
        circle.center.y + (circle.radius + 1.0) * a.sin(),
    );
    let delta = Vec2::new(position.x, position.z) - circle.center;
    let normal = delta / (delta.y * delta.y + delta.x * delta.x).sqrt();
    let rotation = ::utils::xz_rotation_bad(normal);
    let interaction = ui.interaction();
    let scale = if interaction.native_tag != 0
        && interaction.widget == context.id
        && interaction.native_phase != 2
    {
        1.0f32
    } else {
        0.8f32
    };
    for (tag, mesh) in [(0, "corner_arrow"), (1, "corner_arrow_outline")] {
        use crate::systems::ui::signifiers::signifier_factory::{
            SignifierEntityBuilder, SignifierFactory, SignifierMesh,
        };
        let name = format!("circle_{selected:?}_rotate_{position:?}{scale}");
        let mut factory = SignifierFactory::from_closure(
            &name,
            std::sync::Arc::new(move |meshes, shaders, commands| {
                vec![
                    SignifierEntityBuilder::new(SignifierMesh::Named(mesh.into()))
                        .position(position)
                        .rotation(rotation)
                        .scale(scale)
                        .set_ui_pass(tag)
                        .animation_duration(0.0)
                        .custom_shader("vertex_color_shader")
                        .build(meshes, shaders, commands),
                ]
            }),
        );
        factory.marker = 0x142abddc0;
        signifiers.write_factory(&factory);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::{
        WallState,
        public_walls::{WallPublicationDirty, WallSpatialHash},
        publication_source::TerrainHeightGrid512,
    };
    use crate::systems::mode_manager::{AppMode, EditSubject};
    use crate::systems::ui::resources::ui_state::{UiState, WidgetId};
    #[test]
    fn polar_angles_match_original_wrapper_with_host_ucrt_bridge() {
        // m01-polar-fragment-fixture.json: original PE wrapper in Unicorn,
        // imports bridged to host ucrtbase, not a whole-game execution.
        for (x, z, bits) in [
            (1.0f32, 0.0f32, 0x00000000),
            (0.0, 1.0, 0x3fc90fdb),
            (-1.0, 0.0, 0x40490fdb),
            (0.0, -1.0, 0xbfc90fdb),
            (0.0, 0.0, 0x00000000),
            (-0.0, 0.0, 0x40490fdb),
            (0.0, -0.0, 0x80000000),
            (-0.0, -0.0, 0xc0490fdb),
            (f32::INFINITY, 1.0, 0x00000000),
        ] {
            assert_eq!(z.atan2(x).to_bits(), bits);
        }
        assert!(1.0f32.atan2(f32::NAN).is_nan());
    }
    #[test]
    fn real_circle_rotation_picker_updates_wall_and_records_history() {
        let circle = ::utils::geometry::Circle2d::new(Vec2::ZERO, 4.0);
        let mut state = WallState::new(
            super::super::super::to_authority_curve(&circle.as_curve2(0.5)),
            4.0,
            0,
            0,
            false,
        );
        state.snapshot.source.shape =
            ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot();
        state.snapshot.source.curve_kind_tag = 0;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(state);
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
        let mut world = World::new();
        world.init_resource::<Events<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
        world
            .init_resource::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>();
        world.insert_resource(inner);
        world.insert_resource(public);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature="native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.insert_resource(History::<()>::after_observed_future_clear());
        world.init_resource::<crate::resources::time::GameTime>();
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 0.1;
        world.init_resource::<Events<crate::resources::events::OnCircleRotated>>();
        world.init_resource::<Events<crate::resources::roofs::commands::RecalculateRoofCmd>>();
        world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<crate::systems::audio::PlayBuildingAudio>>();
        world.insert_resource(crate::resources::mouse_terrain_raycast::CameraCursorRay {
            origin: Vec3::new(-4.0, 10.0, 0.0),
            dir: -Vec3::Y,
        });
        world.insert_resource(crate::systems::camera_rig::main_camera::MainCamera::new(
            1.0,
        ));
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
        let mut producer = Schedule::default();
        producer.add_systems(ui_rotate_circle::<()>);
        producer.run(&mut world);
        let entries = world
            .resource_mut::<crate::systems::ui::signifiers::signifier_create::UiSignifierStream>()
            .take_entries();
        assert_eq!(entries.len(), 2);
        let meshes = crate::asset_libraries::mesh_library::AssetMeshLibrary::default();
        for name in ["corner_arrow", "corner_arrow_outline"] {
            meshes.add(crate::asset_libraries::mesh_library::MeshAsset::new(
                crate::render::mesh::Mesh::new(name),
                Some(name.to_owned()),
            ));
        }
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
        let mut spawned_ids = Vec::new();
        for entry in &entries {
            let crate::systems::ui::signifiers::UiSignifierEntry::Factory(factory) = entry else {
                panic!("circle rotate factory")
            };
            assert_eq!(factory.marker, 0x142abddc0);
            assert!(
                factory
                    .name()
                    .unwrap()
                    .starts_with(&format!("circle_{wall:?}_rotate_"))
            );
            let mut commands = Commands::new(&mut queue, &spawned);
            spawned_ids.push(factory.wake(&meshes, &shaders, &mut commands)[0]);
        }
        queue.apply(&mut spawned);
        for (index, entity) in spawned_ids.into_iter().enumerate() {
            let animation = spawned
                .get::<crate::systems::ui::signifiers::animation::SignifierAnimation>(entity)
                .unwrap();
            assert_eq!(animation.duration, 0.0);
            assert_eq!(animation.flag_8, 0);
            assert_eq!(animation.flag_9, 1);
            assert_eq!(
                spawned
                    .get::<crate::utils::components::transform::Transform>(entity)
                    .unwrap()
                    .scale,
                Vec3::ZERO
            );
            assert_eq!(
                spawned
                    .get::<crate::systems::render_loop::draw_passes::draw_lists::UiPass>(entity)
                    .is_some(),
                index == 1
            );
        }
        assert!(
            world
                .resource::<Events<crate::systems::ui::resources::widget_params::PlayUiSound>>()
                .is_empty()
        );
        let widget = WidgetId::from_name(&format!("rotate_circle_{wall}"));
        let params = world.resource::<UiState>().widget(widget).unwrap();
        let Some(crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(mesh)) =
            &params.geometry
        else {
            panic!("circle gizmo")
        };
        let center = mesh
            .iter()
            .flat_map(|t| t.points)
            .fold(Vec3::ZERO, |a, p| a + p)
            / (mesh.len() * 3) as f32;
        let inv = glam::Mat4::from_translation(center + Vec3::Z * 10.0);
        world
            .resource_mut::<crate::systems::camera_rig::main_camera::MainCamera>()
            .write_matrix(0x80, &inv.inverse());
        world
            .resource_mut::<crate::systems::camera_rig::main_camera::MainCamera>()
            .write_matrix(0xc0, &inv);
        let mut picker = Schedule::default();
        picker.add_systems(
            (
                crate::systems::ui::hover_intersection::hover_intersection,
                crate::systems::ui::ui_update::ui_update,
            )
                .chain_ignore_deferred(),
        );
        picker.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        producer.run(&mut world);
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustPressed;
        picker.run(&mut world);
        let mut start = Schedule::default();
        start.add_systems(crate::systems::history::recording::history_start_recording::<()>);
        start.run(&mut world);
        producer.run(&mut world);
        assert!(
            world
                .resource::<LiveDecoratorArchivist>()
                .extract_before_after(DecoratorStorageAddr::Wall(wall))
                .is_some()
        );
        world
            .resource_mut::<crate::resources::mouse_terrain_raycast::CameraCursorRay>()
            .origin = Vec3::new(0.0, 10.0, -4.0);
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::Held;
        picker.run(&mut world);
        producer.run(&mut world);
        let changed = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_circle();
        assert_ne!(changed.rotation, circle.rotation);
        assert_eq!(
            world
                .resource_mut::<Events<crate::resources::events::OnCircleRotated>>()
                .drain()
                .count(),
            1
        );
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.consume_with_paths(
                &mut world.resource_mut::<InnerWalls>(),
                &grid,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            )
        });
        world
            .resource_mut::<crate::input::action_set::ActionSetStates>()
            .digital[0]
            .phase = crate::input::action_set::DigitalActionPhase::JustReleased;
        picker.run(&mut world);
        producer.run(&mut world);
        assert_eq!(
            world
                .resource::<History<()>>()
                .pending()
                .unwrap()
                .edits
                .len(),
            1
        );
        assert!(
            world
                .resource_mut::<Events<crate::systems::ui::resources::widget_params::PlayUiSound>>()
                .drain()
                .any(|event| event.kind == 0x22 || event.kind == 0x23)
        );
        assert_eq!(world.resource_mut::<Events<crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>>().drain().count(),1);
        world.init_resource::<crate::resources::stairs::stairs_state::StairsState>();
        world.init_resource::<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
        world
            .init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>>();
        world.init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>>();
        world
            .init_resource::<Events<crate::systems::history::recording::FinishInputRecordingCmd>>();
        world.init_resource::<Events<crate::systems::backend::input::input_record_replay::InputReplayCommand>>();
        let mut finish = Schedule::default();
        finish
            .add_systems(crate::systems::history::recording::history_finish_recording_system::<()>);
        finish.run(&mut world);
        world.insert_resource(
            crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain(
                grid,
            ),
        );
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world
            .resource_mut::<crate::resources::time::GameTime>()
            .delta = 1.0;
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
        let before = ShapeParameters::from_snapshot(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap(),
        )
        .as_circle();
        assert!((before.rotation - circle.rotation).abs() < 0.001);
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
        .as_circle();
        assert!((after.rotation - changed.rotation).abs() < 0.001);
    }
}

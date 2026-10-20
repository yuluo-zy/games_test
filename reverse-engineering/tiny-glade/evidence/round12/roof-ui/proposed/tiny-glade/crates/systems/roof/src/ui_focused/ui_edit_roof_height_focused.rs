//! Full original focused roof height UI1421ad440/5393B and factory1421ae960.
//! Uses canonical input, owned widget geometry, archival collision, roof edits
//! and History. The actual builder remains the shared owning implementation.
use crate::ui_edit_roof::{
    OriginalWallHeight, RoofParamsCacheResource, crossing_wall_height, record_wall_height,
    roof_widget_sounds,
};
use bevy_ecs::{prelude::*, system::SystemParam};
use country_core::{
    input::action_set::ActionSetStates,
    resources::{
        history::{EditType, History, building_transactions::BuildingEdit},
        mouse_terrain_raycast::CursorPositionSS,
        roofs::{
            Roof,
            commands::EditRoofCmd,
            geometry::{
                edit_rules::{RoofEditEvent, ScalarEdit, TipEdit},
                ridge::RidgeGeometry,
            },
            roof_ridge::RoofRidgeDims,
        },
        walls::{
            WallId,
            decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            inner_walls::InnerWalls,
            public_walls::PublicWalls,
        },
        window::WindowSize,
    },
    systems::{
        camera_rig::MainCamera,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{UiSounds, WidgetParams, WidgetVolumeShape},
            },
            signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
        },
        wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume,
    },
    utils::project_cursor_onto_ray::{ProportionalScreenSpace, ProportionalScreenSpaceCoord},
};
use glam::{Mat4, Vec3};
use std::sync::Arc;
use utils::geometry::Ray;

#[path = "native_helpers.rs"]
mod native_helpers;
use native_helpers::{
    BOX_INDICES, box_positions, height_from_projection, squared_distance, xz_rotation_bad,
};

/// Native Local<HeightGrabParams> is the one f32 distance offset written at
///1421adf41. Mouse position and base roof height are not invented local fields.
#[derive(Default, Debug)]
pub struct HeightGrabParams {
    pub distance_offset: f32,
}

#[derive(SystemParam)]
pub struct FocusedRoofInput<'w> {
    mode: Res<'w, AppMode>,
    public: Res<'w, PublicWalls>,
    camera: Res<'w, MainCamera>,
    cursor: Res<'w, CursorPositionSS>,
    window: Res<'w, WindowSize>,
    actions: Res<'w, ActionSetStates>,
    ui: Res<'w, UiState>,
}

pub fn ui_edit_roof_ridge_and_height_focused(
    input: FocusedRoofInput,
    roofs: Query<&Roof>,
    mut grab: Local<HeightGrabParams>,
    mut original_height: Local<OriginalWallHeight>,
    mut cache: ResMut<RoofParamsCacheResource>,
    mut stream: ResMut<UiSignifierStream>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    storage: Res<DecoratorStorage>,
    archival: DecoratorArchivalResources,
    mut walls: ResMut<InnerWalls>,
    mut history: ResMut<History>,
    mut edit: EventWriter<EditRoofCmd>,
) {
    let selected = match *input.mode {
        AppMode::Editing(EditSubject::Wall(wall) | EditSubject::Roof(wall)) => WallId(wall),
        _ => return,
    };
    let Some(entity) = input.public.roof_entity(selected) else {
        return;
    };
    let roof = roofs
        .get(entity)
        .expect("focused roof entity must contain Roof");
    let center = match RoofRidgeDims::from(roof)
        .to_world_space(roof)
        .expect("original roof ridge geometry")
    {
        RidgeGeometry::Point(p) => Vec3::from_array(p),
        RidgeGeometry::Segment([a, b]) => (Vec3::from_array(a) + Vec3::from_array(b)) * 0.5,
    };
    let arrow_position = center + Vec3::new(0.0, 0.75, 0.0);
    let right = input.camera.camera_matrices().eye_right();
    let reciprocal = 1.0 / (right.x * right.x + right.z * right.z).sqrt();
    let rotation = xz_rotation_bad(-right.z * reciprocal, right.x * reciprocal);
    let box_transform =
        Mat4::from_rotation_translation(rotation, arrow_position + Vec3::new(0.0, 0.5, 0.0));
    let triangles =
        box_trigger_volume(&BOX_INDICES, &box_transform, &box_positions(0.75, 1.0, 0.5));
    // Original ActionSetStates+10/+14 is digital slot2 (camera_rotate_mode),
    // with native click at+0/+4 independently proved by ui_update140b2e660.
    let sounds = if input.actions.digital[2].pressed() {
        UiSounds::empty()
    } else {
        roof_widget_sounds()
    };
    let widget = input.ui.add(
        &format!("roof_edit_{}", roof.wall_id()),
        WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
            .triangle_mesh(triangles)
            .sounds(sounds)
            .occlude_by_world(false),
    );
    let phase = widget.into_drag_state();
    let scale = if widget.hovered() != 0 || phase.is_some() {
        1.2
    } else {
        1.0
    };
    // Original four arguments and four literal pieces142ff7c48. Quat/Vec3
    // use original glam DebugTuple implementations (1421adc34/50), while
    // f32 uses Display. Config names participate in entity lifetime identity.
    let name = format!("height_{selected}_arrow_{rotation:?}_{arrow_position:?}{scale}");
    let factory = SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("brush_arrow_flat_single_up")
                    .animation_duration(0.0)
                    .rotation(rotation)
                    .scale(scale * 1.5)
                    .position(arrow_position)
                    .build(meshes, shaders, commands),
            ]
        }),
    );
    stream.write_factory(&factory);

    let wall = roof.wall_id();
    let origin = Vec3::new(center.x, roof.floor_y(), center.z);
    let project = || {
        let screen = ProportionalScreenSpace::from_window_size(input.window.as_dims());
        let xy = screen.transform_screen_space(input.cursor.0.x, input.cursor.0.y);
        ProportionalScreenSpaceCoord(xy).project_onto_ray(
            &screen,
            &input.camera.read_matrix(0x80),
            &input.camera.read_matrix(0),
            Ray {
                origin,
                direction: Vec3::Y,
            },
            [0.0, f32::MAX],
        )
    };
    match phase.0 {
        2 => {}
        3 => {
            // Both lengths are unsigned Euclidean lengths in the original.
            grab.distance_offset = squared_distance(project(), origin).sqrt()
                - squared_distance(center, origin).sqrt();
            archivist.archive_current_decorators_at_address(
                DecoratorStorageAddr::Wall(wall),
                &storage,
                &archival,
                "edit_roof",
            );
            archivist.decorator_cull_allow_wall(wall, &storage);
            original_height.0 = walls
                .get(wall)
                .expect("focused roof requires wall")
                .snapshot
                .source
                .extra;
            cache.remember(roof, None);
        }
        4 => {
            let height = height_from_projection(project(), origin, grab.distance_offset);
            // Preserve the original subtract/add rounding before crossing tests.
            let previous = roof.height_01();
            let next = (height - previous) + previous;
            if let Some(height) = crossing_wall_height(original_height.0, previous, next) {
                walls.update_wall_state_height(wall, height);
            }
            edit.write(EditRoofCmd(RoofEditEvent {
                profile: ScalarEdit {
                    mode: 0,
                    value: 0.0,
                },
                height: ScalarEdit {
                    mode: 1,
                    value: height,
                },
                ridge: ScalarEdit {
                    mode: 0,
                    value: 0.0,
                },
                eave: ScalarEdit {
                    mode: 0,
                    value: 0.0,
                },
                wall_id: wall.0,
                tip: TipEdit {
                    mode: 0,
                    value: glam::Vec2::ZERO,
                },
                source_flag: 0,
            }));
        }
        _ => {
            if let Some(before) = &cache.before {
                if before.differs(roof) {
                    let BuildingEdit::Roof(visual) = before.visual_record(roof).edit() else {
                        unreachable!("visual record owner")
                    };
                    let _ = history.add(EditType::Roof(Arc::new(visual)));
                    let now = walls
                        .get(wall)
                        .expect("focused roof finish requires wall")
                        .snapshot
                        .source
                        .extra;
                    if original_height.0 != now {
                        record_wall_height(&mut history, wall, original_height.0, now);
                    }
                }
            }
        }
    }
}

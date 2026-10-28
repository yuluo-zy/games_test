//! B `systems/ui_move_shapes.rs` — `ui_move_shapes` (VA `0x14097a060`, 12832 B, 30 params).
//!
//! Spec §3. The case helpers have limited unit coverage; the production cache
//! producer, complete edit history and event payloads remain under review.
//! `move_gizmo_and_trigger_vol` restores widget registration and real factories;
//! selected-wall/no-interaction entry is tested. Drag state and transactions
//! are still partial and must not be inferred from those bounded checks.

use bevy_ecs::prelude::*;

use crate::input::action_set::ActionSetStates;
use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::events::OnShapeMoved;
use crate::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder};
use crate::resources::history::History;
use crate::resources::mouse_terrain_raycast::CameraCursorRay;
use crate::resources::roofs::Roof;
use crate::resources::shape_intersections::ShapeIntersections;
use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use crate::resources::time::GameTime;
use crate::resources::tree_storage::HideTreesinAreaCmd;
use crate::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use crate::resources::walls::decorator_storage::DecoratorStorage;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::{InnerWalls, WallId};
use crate::resources::window::AudioCursorMoved;
use crate::systems::audio::{PlayBuildingAudio, PlayWallUiSound, RequestWallMaterialAudio};
use crate::systems::backend::input::mod_key::Modifiers;
use crate::systems::collision::world::RaycastWorld;
use crate::systems::mode_manager::{AppMode, EditSubject};
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter;
use crate::systems::wall::shapes::shape_placement::{RoofRidgeSnap, snap_to_roof_ridge};
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// B `MoveShapeEntry` (per hashbrown bucket, stride `0x2a8`, negative offsets). Field names
/// synthesized from the offsets; the key (`WallId`) is the map key.
#[derive(Clone, Debug)]
pub struct MoveShapeEntry {
    /// `-0x290` u64; `< 2 → assert panic`.
    pub state: u64,
    /// `-0x270` f32; `<= 0.0 → assert panic`: source curve length at public +0x30.
    pub activation: f32,
    /// `-0x2a0 [u8; 0x280]` authoritative shape record (only `+0x30`/`+0x280` read).
    pub shape_rec: [u8; 0x280],
    /// `-0x48` u32 jump-table index (`0x142ab7514`).
    pub variant: u32,
    /// In-transit moved curve (native `entry-0x53` points). Typed field: written by case1
    /// (deferred), consumed by case3 commit. `None` = no move recorded.
    pub curve: Option<crate::resources::roofs::geometry::surface::Curve2>,
    /// Commit sentinel (native `entry[-0x20] = 0x8000…`): case3 sets it once.
    pub committed: bool,
    /// case1 decorator triples `{DecoratorId u32, d[-0x34]-cx f32, d[-0x2c]-cz f32}` written into
    /// the shape record's Vec payload (`+0x08` ptr); typed here so the Vec is owned rather than
    /// modelled by a raw pointer.
    pub decorator_triples: Vec<(u32, f32, f32)>,
}

impl Default for MoveShapeEntry {
    fn default() -> Self {
        Self {
            state: 0,
            activation: 0.0,
            shape_rec: [0; 0x280],
            variant: 0,
            curve: None,
            committed: false,
            decorator_triples: Vec::new(),
        }
    }
}

/// Single native grab working record. Public walls are not stored in this Local.
#[derive(Default, Debug)]
pub struct MoveShapeCache {
    pub entry: MoveShapeEntry,
    pub initial_shape: Option<crate::systems::wall::valid_enclosure::ShapeParameters>,
    pub hover_snap: bool,
}

/// B case1 drag (`code_0x14097ad4c..b046`): archive/cull wall+terrain decorators, collect the
/// moved-shape working record into the shape `rec` (`param_6` = entry value), and stash the
/// decorator triples. `entry[-X] ≡ shape_rec[0x2a0-X]`; ShapeParameters at `shape_rec[0x258..0x274]`.
#[allow(clippy::too_many_arguments)]
fn case1_drag(
    entry: &mut MoveShapeEntry,
    wall: WallId,
    ray: &CameraCursorRay,
    terrain: &TerrainHeightsData<TerrainHeightSsbo>,
    storage: &mut DecoratorStorage,
    archivist: &mut LiveDecoratorArchivist,
    archival: &DecoratorArchivalResources,
) {
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    use crate::resources::walls::publication_fields::WallShapeSnapshot;
    use crate::systems::wall::valid_enclosure::ShapeParameters;

    // step1 archive "move_shape" (wall+terrain); step2 cull allow wall.
    archivist.archive_current_decorators_in_storages_with_resources(
        &[
            DecoratorStorageAddr::Wall(wall),
            DecoratorStorageAddr::Terrain,
        ],
        storage,
        archival,
        "move_shape",
    );
    archivist.decorator_cull_allow_wall(wall, storage);
    if let Some(terrain) = storage.get(DecoratorStorageAddr::Terrain) {
        archivist
            .culling
            .allowed_ids
            .extend(terrain.decorators().keys().copied());
    }

    // ShapeParameters (28B) at shape_rec[0x258..0x274]; center() per tag.
    let sp_bytes: [u8; 28] = entry.shape_rec[0x258..0x274].try_into().unwrap();
    let sp = ShapeParameters::from_snapshot(&WallShapeSnapshot::from_observed(sp_bytes));
    let center = sp.center();

    // step3 collect decorator triples {id, d[-0x34]-cx, d[-0x2c]-cz}; d[+0x1c]=hi(u18), d[+0x24]=u24.
    let mut triples: Vec<(u32, f32, f32)> = Vec::new();
    if let Some(decorators) = storage.wall_decorators_mut(wall) {
        for (id, info) in decorators.iter_maybe_mut() {
            match info.dst_tag() {
                0 | 1 => {
                    let dx = f32::from_bits((info.dst_payload >> 32) as u32) - center.x;
                    let dz = f32::from_bits((info.u18 >> 32) as u32) - center.y;
                    triples.push((id, dx, dz));
                }
                2 => {}
                _ => panic!("invalid move shape wall decorator destination"),
            }
        }
    }

    // anchor = Vec3(center.x, terrain_height, center.y).
    let terrain_y = terrain.sample_at_world_xz(center.x, center.y);
    let anchor = glam::Vec3::new(center.x, terrain_y, center.y);

    // +0x18: cursor-ray ∩ y=anchor.y → world xz − anchor xz.
    let t = (anchor.y - ray.origin.y) / ray.dir.y;
    let projected = glam::Vec2::new(ray.origin.x + t * ray.dir.x, ray.origin.z + t * ray.dir.z);
    let delta = projected - glam::Vec2::new(anchor.x, anchor.z);

    // step5 write the working record (`param_6` = shape_rec value).
    let rec = &mut entry.shape_rec;
    rec[0x00..0x04].copy_from_slice(&(triples.capacity() as u32).to_le_bytes());
    rec[0x04..0x08].copy_from_slice(&0u32.to_le_bytes());
    rec[0x08..0x10].copy_from_slice(&0u64.to_le_bytes()); // ptr (Vec owned on the entry)
    rec[0x10..0x14].copy_from_slice(&(triples.len() as u32).to_le_bytes());
    rec[0x14..0x18].copy_from_slice(&0u32.to_le_bytes());
    rec[0x18..0x1c].copy_from_slice(&delta.x.to_le_bytes());
    rec[0x1c..0x20].copy_from_slice(&delta.y.to_le_bytes());
    // +0x20..0x3c = 28B ShapeParameters copy (param_6+0x20..0x3b).
    let sp_copy: [u8; 28] = rec[0x258..0x274].try_into().unwrap();
    rec[0x20..0x3c].copy_from_slice(&sp_copy);
    rec[0x3c..0x40].copy_from_slice(&anchor.y.to_le_bytes());

    entry.decorator_triples = triples;
}

/// B `ui_move_shapes` (`0x14097a060`, 30 params, PDB order).
///
/// SKELETON: mode gate, cache-empty/entry probe, and the `0x142ab7514` 4-way dispatch are
/// wired; the case bodies + `digital[..]` decision are `TODO(evidence)`.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn ui_move_shapes<S: Send + Sync + 'static>(
    (
        app_mode,
        public,
        mut inner,
        roofs,
        camera_cursor_ray,
        cursor_terrain,
        mut audio_cursor,
        raycast_world,
    ): (
        Res<AppMode>,
        Res<PublicWalls>,
        ResMut<InnerWalls>,
        Query<&Roof>,
        Res<CameraCursorRay>,
        Res<CursorTerrainRaycast>,
        EventReader<AudioCursorMoved>,
        Res<RaycastWorld>,
    ),
    (
        mut decorator_storage,
        mut archivist,
        ui_state,
        mut signifiers,
        mut cache,
        mut history,
        actions,
        mut material_audio,
    ): (
        ResMut<DecoratorStorage>,
        ResMut<LiveDecoratorArchivist>,
        Res<UiState>,
        ResMut<UiSignifierStream>,
        Local<MoveShapeCache>,
        ResMut<History<S>>,
        Res<ActionSetStates>,
        EventWriter<RequestWallMaterialAudio>,
    ),
    (
        mut building_audio,
        mut wall_ui_sound,
        mut on_shape_moved,
        mut sheep_check,
        mut hide_trees,
        mut show_pins,
        archival,
        terrain,
    ): (
        EventWriter<PlayBuildingAudio>,
        EventWriter<PlayWallUiSound>,
        EventWriter<OnShapeMoved>,
        EventWriter<CmdCheckForSheepInsideBuilding>,
        EventWriter<HideTreesinAreaCmd>,
        EventWriter<ShowPinnedDecoClutter>,
        DecoratorArchivalResources,
        Res<TerrainHeightsData<TerrainHeightSsbo>>,
    ),
    (
        border,
        mut out_of_border,
        mut feedback_out_of_border,
        time,
        modifiers,
        intersections,
        spatial,
    ): (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
        Res<GameTime>,
        Res<Modifiers>,
        Res<ShapeIntersections>,
        Res<WallSpatialHash>,
    ),
) {
    // Mode gate (`/…/*param_1 <= 5 && (==1||==2)/`): only Editing(Wall/Roof) proceed.
    let tag = app_mode.native_tag();
    if tag > 5 {
        return;
    }
    let wall = match *app_mode {
        AppMode::Editing(EditSubject::Wall(id)) | AppMode::Editing(EditSubject::Roof(id)) => {
            WallId(id)
        }
        _ => return,
    };
    // Native param_2's maps are PublicWalls records and roof associations, not
    // maps inside the Local. Register the real selected wall before examining
    // the drag phase, including the first frame with an empty Local.
    let Some(shape) = public.get(wall) else {
        return;
    };
    if shape.shape().tag() >= 2 {
        return;
    }
    let parameters =
        crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(shape.shape());
    let center = parameters.center();
    let terrain_y = terrain.sample_at_world_xz(center.x, center.y);
    let anchor = glam::Vec3::new(
        center.x,
        shape.authority.elevation().max(terrain_y),
        center.y,
    );
    let rotation = if parameters.tag() == 1 {
        glam::Quat::from_mat3(&parameters.as_rectangle().orientation_mat3())
    } else {
        glam::Quat::IDENTITY
    };
    let context = move_gizmo_and_trigger_vol(
        anchor,
        rotation,
        &actions,
        &ui_state,
        wall,
        shape,
        terrain_y,
        &mut signifiers,
    );
    let variant = context.dragged().wrapping_sub(2).min(3);
    if variant == 0 {
        return;
    }
    // Jump table `0x142ab7514`: 0→return, 1→case1, 2→case2, 3→case3, else native halt_baddata.
    match variant {
        0 => {}
        1 => {
            // case1 drag: archive/cull decorators + write the move-shape working record.
            cache.entry.shape_rec[0x258..0x274].copy_from_slice(&parameters.to_record());
            cache.initial_shape = Some(parameters);
            case1_drag(
                &mut cache.entry,
                wall,
                &camera_cursor_ray,
                &terrain,
                &mut decorator_storage,
                &mut archivist,
                &archival,
            );
        }
        2 => {
            use crate::resources::glade_border::BorderShape;
            use crate::resources::tree_storage::TreeHideArea;
            use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
            if cache.initial_shape.is_none() {
                return;
            }
            let read =
                |at| f32::from_le_bytes(cache.entry.shape_rec[at..at + 4].try_into().unwrap());
            let offset = glam::Vec2::new(read(0x18), read(0x1c));
            let plane_y = read(0x3c);
            let mut target = if shape.authority.elevation() != 0.0 {
                let t = (plane_y - camera_cursor_ray.origin.y) / camera_cursor_ray.dir.y;
                glam::Vec2::new(
                    camera_cursor_ray.origin.x + t * camera_cursor_ray.dir.x,
                    camera_cursor_ray.origin.z + t * camera_cursor_ray.dir.z,
                ) - offset
            } else {
                (if cursor_terrain.tag & 1 != 0 {
                    glam::Vec2::new(cursor_terrain.pos.x, cursor_terrain.pos.z)
                } else {
                    glam::Vec2::ZERO
                }) - offset
            };
            let was_snapped = cache.hover_snap;
            cache.hover_snap = false;
            if !modifiers.disable_snapping() {
                if let Some(selected_roof) = public
                    .roof_entity(wall)
                    .and_then(|entity| roofs.get(entity).ok())
                {
                    let projection = shape.building_projection();
                    let mut top =
                        terrain.sample_at_world_xz(target.x, target.y) + projection.float_0x284();
                    if roof_has_snap_height(selected_roof.height_01()) {
                        top += selected_roof.height();
                    }
                    let hit = raycast_world.raycast_simple_excluding_wall(
                        glam::Vec3::new(target.x, top - 0.5, target.y),
                        -glam::Vec3::Y,
                        0x202,
                        wall,
                    );
                    if matches!(hit.handle[0] as u8, 1 | 2) {
                        let hit_wall = WallId(hit.handle[1]);
                        if let (Some(hit_public), Some(hit_roof)) = (
                            public.get(hit_wall),
                            public
                                .roof_entity(hit_wall)
                                .and_then(|entity| roofs.get(entity).ok()),
                        ) {
                            let hit_top = hit_roof.floor_y() + hit_roof.height();
                            if projection.float_0x288() <= hit_top
                                && hit_public.authority.elevation() <= top
                            {
                                let at = glam::Vec3::new(hit.position.x, hit_top, hit.position.z);
                                let point = match snap_to_roof_ridge(hit_roof, at, 1.0) {
                                    RoofRidgeSnap::Point(p) => Some(p),
                                    RoofRidgeSnap::Segment { a, b, t } => {
                                        crate::utils::debug_viz::DebugLine::new(vec![
                                            a + glam::Vec3::Y * 0.2,
                                            b + glam::Vec3::Y * 0.2,
                                        ])
                                        .line_width(3.0)
                                        .set_ui_pass(true)
                                        .dashed()
                                        .show(&mut signifiers);
                                        Some(a * (1.0 - t) + b * t)
                                    }
                                    RoofRidgeSnap::None => None,
                                };
                                if let Some(point) = point {
                                    target = glam::Vec2::new(point.x, point.z);
                                    cache.hover_snap = true;
                                    crate::utils::debug_viz::DebugLine::new(vec![
                                        glam::Vec3::new(point.x, top.max(point.y), point.z),
                                        glam::Vec3::new(point.x, anchor.y, point.z),
                                    ])
                                    .line_width(3.0)
                                    .set_ui_pass(true)
                                    .dashed()
                                    .show(&mut signifiers);
                                    if !was_snapped {
                                        wall_ui_sound.write(PlayWallUiSound {
                                            kind: 3,
                                            selector: 0,
                                            value: 0.0,
                                            position: anchor,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let mut candidate = match parameters.get_shape().expect("premade move shape") {
                Shape::Circle(mut c) => {
                    c.center = target;
                    Shape::Circle(c)
                }
                Shape::Rectangle(mut r) => {
                    r.center = target;
                    Shape::Rectangle(r)
                }
            };
            let border_shape = match candidate {
                Shape::Circle(c) => BorderShape::Circle(c),
                Shape::Rectangle(r) => BorderShape::Rectangle(r),
            };
            let inside = border.is_shape_inside(border_shape);
            let pins = match candidate {
                Shape::Circle(circle) => ShowPinnedDecoClutter::Circle {
                    circle,
                    flag: false,
                    bottom: shape.authority.elevation(),
                    top: shape.max_y(),
                    exclude_wall: Some(wall),
                },
                Shape::Rectangle(rectangle) => ShowPinnedDecoClutter::Rectangle {
                    rectangle,
                    bottom: shape.authority.elevation(),
                    top: shape.max_y(),
                    exclude_wall: Some(wall),
                },
            };
            show_pins.write(pins);
            let area = match candidate {
                Shape::Circle(c) => TreeHideArea::Circle(c),
                Shape::Rectangle(r) => TreeHideArea::Rectangle(r),
            };
            hide_trees.write(HideTreesinAreaCmd {
                area,
                height_limit: shape.authority.elevation(),
                expansion: 1.5,
                finished: false,
            });
            if !inside {
                out_of_border.write(DisplayOutOfBorder);
                feedback_out_of_border.write(FeedbackInputOutOfBorder);
                candidate = match (border.0, candidate) {
                    (BorderShape::Rectangle(_), Shape::Circle(c)) => {
                        let old = parameters.as_circle();
                        match border.search_lerped_shape_that_fits_into_glade(
                            BorderShape::Circle(old),
                            BorderShape::Circle(c),
                        ) {
                            BorderShape::Circle(fitted) => Shape::Circle(fitted),
                            _ => unreachable!(),
                        }
                    }
                    (BorderShape::Circle(boundary), Shape::Circle(mut c)) => {
                        let d = c.center - boundary.center;
                        let direction = d / (d.x * d.x + d.y * d.y).sqrt();
                        let limited = boundary.center + direction * (boundary.radius - c.radius);
                        let current = parameters.center();
                        c.center = (limited - current)
                            * ::utils::time_continuous_lerp_factor(time.delta, 100.0)
                            + current;
                        Shape::Circle(c)
                    }
                    (BorderShape::Rectangle(_), Shape::Rectangle(r)) => {
                        match border.search_lerped_shape_that_fits_into_glade(
                            BorderShape::Rectangle(parameters.as_rectangle()),
                            BorderShape::Rectangle(r),
                        ) {
                            BorderShape::Rectangle(fitted) => Shape::Rectangle(fitted),
                            _ => unreachable!(),
                        }
                    }
                    (BorderShape::Circle(boundary), Shape::Rectangle(mut r)) => {
                        r.center = fit_rectangle_center_in_circle(
                            boundary,
                            parameters.as_rectangle(),
                            r.center,
                            time.delta,
                        );
                        Shape::Rectangle(r)
                    }
                };
            }
            let final_center = ShapeParameters::from_shape(candidate).center();
            on_shape_moved.write(OnShapeMoved {
                offsets: cache.entry.decorator_triples.clone(),
                wall,
                center: final_center,
            });
            let mut state = inner.get(wall).expect("move shape wall missing").clone();
            let curve = match candidate {
                Shape::Circle(c) => c.as_curve2(0.5),
                Shape::Rectangle(r) => r.as_curve2(),
            };
            state.snapshot.source.curve = crate::systems::wall::shapes::to_authority_curve(&curve);
            let snapshot = ShapeParameters::from_shape(candidate).to_snapshot();
            state.snapshot.source.curve_kind_tag = snapshot.tag();
            state.snapshot.source.shape = snapshot;
            inner.update_wall_state(wall, state);
            if inside && audio_cursor.read().last().is_some() {
                material_audio.write(RequestWallMaterialAudio {
                    id: wall.0,
                    position: anchor,
                    kind: 4,
                    flag: 0,
                });
                let speed = (cursor_terrain.speed / 250.0).max(0.0).min(1.0);
                building_audio.write(PlayBuildingAudio {
                    id: wall.0,
                    position: anchor,
                    editing_speed: speed,
                    kind: 4,
                });
                if cache.hover_snap {
                    building_audio.write(PlayBuildingAudio {
                        id: wall.0,
                        position: anchor,
                        editing_speed: speed,
                        kind: 5,
                    });
                }
            }
        }
        3 => {
            use crate::resources::history::{
                EditType,
                wall_edit::{CompoundEditBuilder, MorphPremadeShapeEdit, WallHistoryEdit},
            };
            use crate::resources::tree_storage::TreeHideArea;
            on_shape_moved.write(OnShapeMoved {
                offsets: cache.entry.decorator_triples.clone(),
                wall,
                center: parameters.center(),
            });
            sheep_check.write(CmdCheckForSheepInsideBuilding { wall });
            if let Some(before) = cache.initial_shape.take().filter(|shape| shape.tag() != 2) {
                let mut builder = CompoundEditBuilder::default();
                builder.new_edit_sequence();
                builder.next(WallHistoryEdit::MorphPremadeShape(
                    MorphPremadeShapeEdit::new(
                        crate::resources::walls::publication_fields::WallId(wall.0),
                        before,
                        parameters,
                    ),
                ));
                history
                    .add(EditType::Wall(std::sync::Arc::new(builder.build())))
                    .expect("move shape history recording");
            }
            let area = if parameters.tag() == 0 {
                TreeHideArea::Circle(parameters.as_circle())
            } else {
                TreeHideArea::Rectangle(parameters.as_rectangle())
            };
            hide_trees.write(HideTreesinAreaCmd {
                area,
                height_limit: shape.authority.elevation(),
                expansion: 1.5,
                finished: true,
            });
        }
        _ => {
            // Native `0x14097ffb0` = halt_baddata (out of the 4-entry table); unreachable by contract.
        }
    }
}

/// `ActionSetStates.digital[2].pressed()` = `phase < 3` — the mouse Rotate binding
/// (`DIGITAL_TARGETS (37,2)`, camera_rotate_mode) read inline by `move_gizmo_and_trigger_vol`
/// (0x14097D382 `[rax+0x10]`; 0x14097D3EF `cmp ebx,3; setae`).
pub fn camera_rotate_pressed(actions: &ActionSetStates) -> bool {
    (actions.digital[2].phase as u8) < 3
}

/// The native unordered comparison follows the height-present branch.
fn roof_has_snap_height(height: f32) -> bool {
    !(height <= 0.1)
}

fn fit_rectangle_center_in_circle(
    boundary: ::utils::geometry::Circle2d,
    published: ::utils::geometry::Rectangle2d,
    target: glam::Vec2,
    delta: f32,
) -> glam::Vec2 {
    let d = target - boundary.center;
    let direction = d / (d.x * d.x + d.y * d.y).sqrt();
    let mut centered = published;
    centered.center = boundary.center;
    let (a, b) = centered.get_edge_pts(centered.get_closest_edge_id(target));
    let cast = |origin| {
        boundary
            .intersect_ray(::utils::geometry::Ray2 { origin, direction })
            .map_or(0.0, |hit| hit.0)
    };
    let limited = boundary.center + direction * cast(a).min(cast(b));
    (limited - published.center) * ::utils::time_continuous_lerp_factor(delta, 100.0)
        + published.center
}

pub fn move_gizmo_and_trigger_vol<'a>(
    anchor: glam::Vec3,
    rotation: glam::Quat,
    actions: &ActionSetStates,
    ui_state: &'a UiState,
    wall: WallId,
    shape: &crate::resources::walls::public_wall_state::PublicWallState,
    signifier_y: f32,
    signifiers: &mut UiSignifierStream,
) -> crate::systems::ui::resources::ui_state::UiWidgetContext<'a> {
    use crate::systems::ui::resources::widget_params::{WidgetGeometry, WidgetParams};
    use crate::systems::ui::signifiers::signifier_factory::{
        SignifierEntityBuilder, SignifierFactory, SignifierMesh,
    };
    use crate::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
    use crate::utils::components::transform::Transform;
    use std::sync::Arc;

    let t = ((shape.simplified_curve_2d().length - 0.5) / 2.5).clamp(0.0, 1.0);
    let smooth = (3.0 - 2.0 * t) * t * t;
    let base = (1.0 - smooth) * 0.5 + smooth;
    let triangles = box_trigger_volume(
        base * 1.5,
        0.5,
        base * 1.5,
        &Transform {
            rotation,
            translation: anchor,
            scale: glam::Vec3::ONE,
        },
    );
    let released = u32::from(!camera_rotate_pressed(actions));
    let mut params = WidgetParams::default().geometry(WidgetGeometry::TriangleMesh(triangles));
    params.fields = [0, 8, 0, 0, 0, 2 * released, 3 * released, 3 * released];
    // Native slots at +0x18, +0x30, +0x48, +0x60 are also represented by
    // typed UiSounds, which the current input consumer reads directly.
    for (slot, kind) in
        params
            .sounds
            .slots
            .iter_mut()
            .zip([0u32, 2 * released, 3 * released, 3 * released])
    {
        slot[..4].copy_from_slice(&kind.to_le_bytes());
    }
    params.occlude_by_world = false;
    let context = ui_state.add(&format!("move_shape{wall}"), params);

    if shape.authority.elevation() > 0.0 {
        let start = glam::Vec3::new(anchor.x, signifier_y, anchor.z);
        let name = format!("move_shape_line{wall:?}{anchor:?}{start}{anchor}");
        let mut factory = SignifierFactory::from_closure(
            &name,
            Arc::new(move |meshes, shaders, commands| {
                let entity =
                    SignifierEntityBuilder::new(SignifierMesh::Points(vec![start, anchor]))
                        .animation_duration(0.0)
                        .custom_shader("dashed_line")
                        .build(meshes, shaders, commands);
                let d = start - anchor;
                commands
                    .entity(entity)
                    .insert(crate::components::DashedLineLength(
                        ((d.z * d.z + d.y * d.y) + d.x * d.x).sqrt(),
                    ));
                vec![entity]
            }),
        );
        factory.marker = 0x142ab6d90;
        signifiers.write_factory(&factory);
    }

    let interaction = ui_state.interaction();
    let matched = interaction.widget == context.id
        && match interaction.native_tag {
            0 => false,
            1 => interaction.native_phase != 0,
            _ => interaction.native_phase != 2,
        };
    let scale = base * if matched { 1.2 } else { 1.0 };
    for (tag, mesh_name) in [(0, "move_shape"), (1, "move_shape_outline")] {
        let name = format!("move_shape{wall:?}{anchor:?}{mesh_name}{scale}{rotation:?}");
        let mut factory = SignifierFactory::from_closure(
            &name,
            Arc::new(move |meshes, shaders, commands| {
                vec![
                    SignifierEntityBuilder::new(SignifierMesh::Named(mesh_name.to_owned()))
                        .rotation(rotation)
                        .position(anchor)
                        .scale(scale * 1.5)
                        .set_ui_pass(tag)
                        .animation_duration(0.0)
                        .custom_shader("vertex_color_shader")
                        .build(meshes, shaders, commands),
                ]
            }),
        );
        factory.marker = 0x142ab6e28;
        signifiers.write_factory(&factory);
    }
    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_shapes_real_system_registers_selected_circle_without_seeding_local_cache() {
        use crate::resources::walls::public_walls::WallPublicationDirty;
        use crate::resources::walls::{
            WallState, publication_fields::WallShapeSnapshot,
            publication_source::TerrainHeightGrid512,
        };
        use crate::systems::ui::resources::ui_state::WidgetId;
        use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        use bevy_ecs::system::SystemState;

        let circle = utils::geometry::Circle2d::new(glam::Vec2::new(2.0, 3.0), 2.0);
        let mut source = WallState::new(
            crate::systems::wall::shapes::to_authority_curve(&circle.as_curve2(0.2)),
            20.0,
            0,
            0,
            false,
        );
        source.snapshot.source.shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
        );
        source.snapshot.source.curve_kind_tag = 0;
        source.snapshot.source.terrain_floor = 1.0;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(source);
        let grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let spatial = WallSpatialHash::default();
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &grid,
            None,
            &WallPublicationDirty::default(),
            &spatial,
        );
        let before = inner.get(wall).unwrap().source().curve.clone();
        let mut world = World::new();
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(public);
        world.insert_resource(inner);
        world.insert_resource(CameraCursorRay {
            origin: glam::Vec3::new(2.0, 10.0, 3.0),
            dir: -glam::Vec3::Y,
        });
        world.init_resource::<CursorTerrainRaycast>();
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<RaycastWorld>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(History::<()>::after_observed_future_clear());
        world.init_resource::<ActionSetStates>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<PlayBuildingAudio>>();
        world.init_resource::<Events<PlayWallUiSound>>();
        world.init_resource::<Events<OnShapeMoved>>();
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<ShowPinnedDecoClutter>>();
        world.init_resource::<TerrainHeightsData<TerrainHeightSsbo>>();
        world.insert_resource(GladeBorder(
            crate::resources::glade_border::BorderShape::Circle(utils::geometry::Circle2d::new(
                glam::Vec2::ZERO,
                65.0,
            )),
        ));
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<GameTime>();
        world.init_resource::<Modifiers>();
        world.init_resource::<ShapeIntersections>();
        world.insert_resource(spatial);
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();

        // Grouping only adapts Bevy's tuple arity; it calls the actual production
        // function with its resources and default Local, not a copied algorithm.
        let mut params: SystemState<(
            (
                Res<AppMode>,
                Res<PublicWalls>,
                ResMut<InnerWalls>,
                Query<&Roof>,
                Res<CameraCursorRay>,
                Res<CursorTerrainRaycast>,
                EventReader<AudioCursorMoved>,
                Res<RaycastWorld>,
            ),
            (
                ResMut<DecoratorStorage>,
                ResMut<LiveDecoratorArchivist>,
                Res<UiState>,
                ResMut<UiSignifierStream>,
                Local<MoveShapeCache>,
                ResMut<History<()>>,
                Res<ActionSetStates>,
                EventWriter<RequestWallMaterialAudio>,
            ),
            (
                EventWriter<PlayBuildingAudio>,
                EventWriter<PlayWallUiSound>,
                EventWriter<OnShapeMoved>,
                EventWriter<CmdCheckForSheepInsideBuilding>,
                EventWriter<HideTreesinAreaCmd>,
                EventWriter<ShowPinnedDecoClutter>,
                DecoratorArchivalResources,
                Res<TerrainHeightsData<TerrainHeightSsbo>>,
            ),
            (
                Res<GladeBorder>,
                EventWriter<DisplayOutOfBorder>,
                EventWriter<FeedbackInputOutOfBorder>,
                Res<GameTime>,
                Res<Modifiers>,
                Res<ShapeIntersections>,
                Res<WallSpatialHash>,
            ),
        )> = SystemState::new(&mut world);
        let (a, b, c, d) = params.get_mut(&mut world);
        ui_move_shapes::<()>(a, b, c, d);
        params.apply(&mut world);
        let widget = WidgetId::from_name(&format!("move_shape{wall}"));
        let registration = world
            .resource::<UiState>()
            .widget(widget)
            .expect("first frame must register its move widget");
        assert_eq!(registration.volume.center, glam::Vec3::ZERO); // TriangleMesh owns the world coordinates.
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 3);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points,
            before.points
        );
        assert!(world.resource::<Events<OnShapeMoved>>().is_empty());
        assert!(world.resource::<History<()>>().pending().is_none());
        use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
        use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
        let mut info = DecoratorInfo::default();
        info.dst_payload = (1.0f32.to_bits() as u64) << 32;
        info.u18 = (2.0f32.to_bits() as u64) << 32;
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(11), info);
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(DecoratorId(12), DecoratorInfo::default());
        let inverse_view = glam::Mat4::from_translation(glam::Vec3::new(2.0, 1.0, 10.0));
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse_view.inverse());
        camera.write_matrix(0xc0, &inverse_view);
        world.insert_resource(camera);
        world.insert_resource(crate::resources::window::WindowSize::new(1280, 720));
        world.insert_resource(crate::resources::mouse_terrain_raycast::CursorPositionSS(
            glam::Vec2::new(640.0, 360.0),
        ));
        world.insert_resource(crate::resources::read_mouse_depth_value::MouseDepthValue {
            view_depth: 100.0,
            ..Default::default()
        });
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
        let mut schedule = Schedule::default();
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        schedule.add_systems(ui_move_shapes::<()>);
        input.run(&mut world);
        assert_eq!(world.resource::<UiState>().active_widget(), widget);
        schedule.run(&mut world);
        world.resource_mut::<ActionSetStates>().digital[0].phase =
            crate::input::action_set::DigitalActionPhase::JustPressed;
        input.run(&mut world);
        assert_eq!(
            world.resource::<UiState>().widget_context(widget).dragged(),
            3
        );
        let mut history_start = Schedule::default();
        history_start
            .add_systems(crate::systems::history::recording::history_start_recording::<()>);
        history_start.run(&mut world);
        assert!(world.resource::<History<()>>().is_recording());
        schedule.run(&mut world);
        let archivist = world.resource::<LiveDecoratorArchivist>();
        assert!(
            archivist
                .extract_before_after(DecoratorStorageAddr::Wall(wall))
                .is_some()
        );
        assert!(
            archivist
                .extract_before_after(DecoratorStorageAddr::Terrain)
                .is_some()
        );
        assert!(archivist.is_decorator_cull_allowed(11));
        assert!(archivist.is_decorator_cull_allowed(12));
        world.resource_mut::<ActionSetStates>().digital[0].phase =
            crate::input::action_set::DigitalActionPhase::Held;
        world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(4.0, 10.0, 4.0);
        world.resource_mut::<CursorTerrainRaycast>().speed = 125.0;
        world.send_event(AudioCursorMoved);
        input.run(&mut world);
        schedule.run(&mut world);
        let held = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .unwrap();
        assert_eq!(
            ShapeParameters::from_snapshot(held).center(),
            glam::Vec2::new(4.0, 4.0)
        );
        let audio: Vec<_> = world
            .resource_mut::<Events<PlayBuildingAudio>>()
            .drain()
            .collect();
        assert_eq!(audio.len(), 1);
        assert_eq!((audio[0].kind, audio[0].editing_speed), (4, 0.5));
        assert_eq!(
            world
                .resource_mut::<Events<RequestWallMaterialAudio>>()
                .drain()
                .count(),
            1
        );
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
        *world.resource_mut::<GladeBorder>() =
            GladeBorder(crate::resources::glade_border::BorderShape::Circle(
                utils::geometry::Circle2d::new(glam::Vec2::ZERO, 6.0),
            ));
        world.resource_mut::<GameTime>().delta = 1.0;
        world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(20.0, 10.0, 0.0);
        input.run(&mut world);
        schedule.run(&mut world);
        let bounded = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .unwrap();
        assert!(
            (ShapeParameters::from_snapshot(bounded).center() - glam::Vec2::new(4.0, 0.0)).length()
                < 0.0001
        );
        assert_eq!(
            world
                .resource_mut::<Events<DisplayOutOfBorder>>()
                .drain()
                .count(),
            1
        );
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
        world.resource_mut::<ActionSetStates>().digital[0].phase =
            crate::input::action_set::DigitalActionPhase::JustReleased;
        input.run(&mut world);
        schedule.run(&mut world);
        let events: Vec<_> = world
            .resource_mut::<Events<OnShapeMoved>>()
            .drain()
            .collect();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].wall, wall);
        assert_eq!(events[0].offsets, vec![(11, -1.0, -1.0)]);
        assert_eq!(events[0].center, glam::Vec2::new(4.0, 4.0));
        assert!((events[1].center - glam::Vec2::new(4.0, 0.0)).length() < 0.0001);
        assert_eq!(events[2].center, events[1].center);
        assert!(
            matches!(&world.resource::<History<()>>().pending().unwrap().edits[0], crate::resources::history::EditType::Wall(edit) if matches!(&edit.groups[0][0], crate::resources::history::wall_edit::WallHistoryEdit::MorphPremadeShape(_)))
        );
        assert_eq!(
            world
                .resource_mut::<Events<CmdCheckForSheepInsideBuilding>>()
                .drain()
                .count(),
            1
        );
        assert!(
            world
                .resource_mut::<Events<HideTreesinAreaCmd>>()
                .drain()
                .any(|e| e.finished)
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
        assert!(!world.resource::<History<()>>().is_recording());
        assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
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
        let restored = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .unwrap();
        assert!(
            (ShapeParameters::from_snapshot(restored).center() - circle.center).length() < 0.001
        );
        world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
        for _ in 0..400 {
            replay.run(&mut world);
        }
        let redone = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .unwrap();
        assert!(
            (ShapeParameters::from_snapshot(redone).center() - glam::Vec2::new(4.0, 0.0)).length()
                < 0.001
        );
        #[cfg(feature = "native-physics")]
        {
            use crate::systems::collision::world::{
                ColliderMeshInstance, RaycastGeoSource, RaycastMeshData, upsert_collider,
            };
            use std::{collections::HashMap, sync::Arc};
            let other_circle = utils::geometry::Circle2d::new(glam::Vec2::new(6.5, 0.0), 2.0);
            let mut other = WallState::new(
                crate::systems::wall::shapes::to_authority_curve(&other_circle.as_curve2(0.5)),
                2.0,
                0,
                0,
                false,
            );
            other.snapshot.source.shape =
                ShapeParameters::from_shape(Shape::Circle(other_circle)).to_snapshot();
            other.snapshot.source.curve_kind_tag = 0;
            let other_wall = world
                .resource_mut::<InnerWalls>()
                .new_wall_from_curve(other);
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
            let mut roof_bytes = [0u8; 88];
            roof_bytes[0x4c..0x50].copy_from_slice(&5.0f32.to_le_bytes());
            roof_bytes[0x0c..0x10].copy_from_slice(&6.5f32.to_le_bytes());
            let other_roof = world.spawn(Roof::from_observed(roof_bytes)).id();
            let selected_roof = world.spawn(Roof::from_observed([0; 88])).id();
            world
                .resource_mut::<PublicWalls>()
                .associate_roof(wall, selected_roof);
            world
                .resource_mut::<PublicWalls>()
                .associate_roof(other_wall, other_roof);
            world
                .resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>()
                .set_height_range(0.0, 0.0);
            let mut instances: HashMap<u32, ColliderMeshInstance> = HashMap::new();
            {
                let mut collision = world.resource_mut::<RaycastWorld>();
                for (key, id, y) in [(1, wall, 10.0), (2, other_wall, 5.0)] {
                    let mut source = RaycastGeoSource::Slot1;
                    source.0[8..16].copy_from_slice(&id.0.to_le_bytes());
                    upsert_collider(
                        &mut collision,
                        &mut instances,
                        key,
                        source,
                        Arc::new(RaycastMeshData::new_cuboid_at(
                            glam::Vec3::splat(2.0),
                            glam::Vec3::ZERO,
                        )),
                        glam::Vec3::new(6.0, y, 0.0),
                        glam::Quat::IDENTITY,
                    );
                }
                collision.flush_colliders();
            }
            *world.resource_mut::<GladeBorder>() =
                GladeBorder(crate::resources::glade_border::BorderShape::Circle(
                    utils::geometry::Circle2d::new(glam::Vec2::ZERO, 65.0),
                ));
            world.insert_resource(UiState::at_startup());
            let inverse = glam::Mat4::from_translation(glam::Vec3::new(4.0, 1.0, 10.0));
            let mut camera =
                crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
            camera.write_matrix(0, &glam::Mat4::IDENTITY);
            camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
            camera.write_matrix(0x80, &inverse.inverse());
            camera.write_matrix(0xc0, &inverse);
            world.insert_resource(camera);
            world.resource_mut::<ActionSetStates>().digital[0].phase =
                crate::input::action_set::DigitalActionPhase::Idle;
            world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(4.0, 10.0, 0.0);
            schedule.run(&mut world);
            input.run(&mut world);
            schedule.run(&mut world);
            world.resource_mut::<ActionSetStates>().digital[0].phase =
                crate::input::action_set::DigitalActionPhase::JustPressed;
            input.run(&mut world);
            schedule.run(&mut world);
            world.resource_mut::<ActionSetStates>().digital[0].phase =
                crate::input::action_set::DigitalActionPhase::Held;
            world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(6.0, 10.0, 0.0);
            input.run(&mut world);
            schedule.run(&mut world);
            let snapped = world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap();
            assert_eq!(
                ShapeParameters::from_snapshot(snapped).center(),
                glam::Vec2::new(6.5, 0.0)
            );
            let sound: Vec<_> = world
                .resource_mut::<Events<PlayWallUiSound>>()
                .drain()
                .collect();
            assert_eq!(sound.len(), 1);
            assert_eq!(sound[0].kind, 3);
            input.run(&mut world);
            schedule.run(&mut world);
            assert!(
                world.resource::<Events<PlayWallUiSound>>().is_empty(),
                "continued snap must not repeat the transition sound"
            );
            world.resource_mut::<Modifiers>().flags = 1;
            input.run(&mut world);
            schedule.run(&mut world);
            let unsnapped = world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap();
            assert_eq!(
                ShapeParameters::from_snapshot(unsnapped).center(),
                glam::Vec2::new(6.0, 0.0)
            );
            world.resource_mut::<Modifiers>().flags = 0;
            world
                .resource_mut::<PublicWalls>()
                .get_mut(other_wall)
                .unwrap()
                .authority
                .snapshot
                .source
                .terrain_floor = 30.0;
            input.run(&mut world);
            schedule.run(&mut world);
            let separated = world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap();
            assert_eq!(
                ShapeParameters::from_snapshot(separated).center(),
                glam::Vec2::new(6.0, 0.0)
            );
            world
                .resource_mut::<PublicWalls>()
                .get_mut(other_wall)
                .unwrap()
                .authority
                .snapshot
                .source
                .terrain_floor = 0.0;
            let mut rectangle_roof = roof_bytes;
            rectangle_roof[8..12].copy_from_slice(&1u32.to_le_bytes());
            rectangle_roof[0x0c..0x10].copy_from_slice(&1.0f32.to_le_bytes());
            rectangle_roof[0x10..0x14].copy_from_slice(&0.0f32.to_le_bytes());
            rectangle_roof[0x14..0x18].copy_from_slice(&6.5f32.to_le_bytes());
            rectangle_roof[0x18..0x1c].copy_from_slice(&0.0f32.to_le_bytes());
            rectangle_roof[0x1c..0x20].copy_from_slice(&4.0f32.to_le_bytes());
            rectangle_roof[0x20..0x24].copy_from_slice(&2.0f32.to_le_bytes());
            rectangle_roof[0x34..0x38].copy_from_slice(&1.0f32.to_le_bytes());
            let roof = Roof::from_observed(rectangle_roof);
            let hit_top = roof.floor_y() + roof.height();
            let RoofRidgeSnap::Segment { a, b, t } =
                snap_to_roof_ridge(&roof, glam::Vec3::new(6.0, hit_top, 0.0), 1.0)
            else {
                panic!("fixture must have a ridge segment")
            };
            let expected = a * (1.0 - t) + b * t;
            *world.get_mut::<Roof>(other_roof).unwrap() = roof;
            let factories_before = world.resource::<UiSignifierStream>().committed_len();
            input.run(&mut world);
            schedule.run(&mut world);
            let segment = world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap();
            assert_eq!(
                ShapeParameters::from_snapshot(segment).center(),
                glam::Vec2::new(expected.x, expected.z)
            );
            assert_eq!(
                world
                    .resource_mut::<Events<PlayWallUiSound>>()
                    .drain()
                    .count(),
                1
            );
            assert_eq!(
                world.resource::<UiSignifierStream>().committed_len() - factories_before,
                5,
                "three gizmo factories and both ridge lines"
            );
            world.get_mut::<Roof>(selected_roof).unwrap().observed_mut()[0x30..0x34]
                .copy_from_slice(&f32::NAN.to_le_bytes());
            input.run(&mut world);
            schedule.run(&mut world);
            let nan_roof = world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .get_enclosure_params()
                .unwrap();
            assert_eq!(
                ShapeParameters::from_snapshot(nan_roof).center(),
                glam::Vec2::new(6.0, 0.0)
            );
        }
        // Exercise the rectangle/circular-border branch through the same
        // picker, press and held production systems, without roof snapping.
        let rect = utils::geometry::Rectangle2d::new(
            glam::Vec2::ZERO,
            glam::Vec2::splat(2.0),
            glam::Vec2::X,
        );
        {
            let mut inner = world.resource_mut::<InnerWalls>();
            let mut state = inner.get(wall).unwrap().clone();
            state.snapshot.source.curve =
                crate::systems::wall::shapes::to_authority_curve(&rect.as_curve2());
            state.snapshot.source.shape =
                ShapeParameters::from_shape(Shape::Rectangle(rect)).to_snapshot();
            state.snapshot.source.curve_kind_tag = 1;
            inner.update_wall_state(wall, state);
        }
        world.resource_scope(|world, mut public: Mut<PublicWalls>| {
            public.remove_roof(wall);
            let mut inner = world.resource_mut::<InnerWalls>();
            public.consume_with_paths(
                &mut inner,
                &grid,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
        });
        *world.resource_mut::<GladeBorder>() =
            GladeBorder(crate::resources::glade_border::BorderShape::Circle(
                utils::geometry::Circle2d::new(glam::Vec2::ZERO, 6.0),
            ));
        world.insert_resource(UiState::at_startup());
        let inverse = glam::Mat4::from_translation(glam::Vec3::new(0.0, 1.0, 10.0));
        let mut camera = crate::systems::camera_rig::main_camera::MainCamera::new(1280.0 / 720.0);
        camera.write_matrix(0, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x40, &glam::Mat4::IDENTITY);
        camera.write_matrix(0x80, &inverse.inverse());
        camera.write_matrix(0xc0, &inverse);
        world.insert_resource(camera);
        world.resource_mut::<ActionSetStates>().digital[0].phase =
            crate::input::action_set::DigitalActionPhase::Idle;
        world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(0.0, 10.0, 0.0);
        schedule.run(&mut world);
        input.run(&mut world);
        schedule.run(&mut world);
        world.resource_mut::<ActionSetStates>().digital[0].phase =
            crate::input::action_set::DigitalActionPhase::JustPressed;
        input.run(&mut world);
        schedule.run(&mut world);
        world.resource_mut::<ActionSetStates>().digital[0].phase =
            crate::input::action_set::DigitalActionPhase::Held;
        world.resource_mut::<CameraCursorRay>().origin = glam::Vec3::new(20.0, 10.0, 0.0);
        input.run(&mut world);
        schedule.run(&mut world);
        let moved = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .get_enclosure_params()
            .unwrap();
        let expected = fit_rectangle_center_in_circle(
            utils::geometry::Circle2d::new(glam::Vec2::ZERO, 6.0),
            rect,
            glam::Vec2::new(20.0, 0.0),
            1.0,
        );
        assert_eq!(
            ShapeParameters::from_snapshot(moved).as_rectangle().center,
            expected
        );
    }

    #[test]
    fn move_gizmo_registers_native_box_and_camera_rotation_sound_switches() {
        use crate::input::action_set::DigitalActionPhase;
        use crate::resources::walls::WallState;
        use crate::resources::walls::public_wall_state::PublicWallState;
        use crate::resources::walls::publication_source::TerrainHeightGrid512;
        use crate::systems::ui::resources::widget_params::WidgetGeometry;
        use glam::{Quat, Vec3};

        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let anchor = Vec3::new(4.0, 2.0, -3.0);
        // These source lengths select the native low/middle/high smoothstep
        // values. Wall height is deliberately unrelated to all three values.
        for (length, half_width) in [(0.5, 0.375), (1.75, 0.5625), (3.0, 0.75)] {
            let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
                vec![[0.0, 0.0], [length, 0.0]],
                false,
            )
            .unwrap();
            let state = WallState::new(curve, 20.0, 0, 0, false);
            let shape = PublicWallState::collect(&state, &terrain).unwrap();
            for phase in [
                DigitalActionPhase::JustPressed,
                DigitalActionPhase::Held,
                DigitalActionPhase::Repeating,
                DigitalActionPhase::JustReleased,
                DigitalActionPhase::Idle,
            ] {
                let ui = UiState::at_startup();
                let mut actions = ActionSetStates::default();
                actions.digital[2].phase = phase;
                let mut stream = UiSignifierStream::default();
                let context = move_gizmo_and_trigger_vol(
                    anchor,
                    Quat::IDENTITY,
                    &actions,
                    &ui,
                    WallId(17),
                    &shape,
                    1.0,
                    &mut stream,
                );
                assert_eq!(
                    context.id,
                    crate::systems::ui::resources::ui_state::WidgetId::from_name(
                        "move_shapeWallId(17)"
                    )
                );
                let params = ui
                    .widget(context.id)
                    .expect("registration also occurs during camera rotation");
                assert!(!params.occlude_by_world);
                assert_eq!(params.flags.0, 1);
                let expected = if (phase as u8) < 3 {
                    [0, 0, 0, 0]
                } else {
                    [0, 2, 3, 3]
                };
                for (slot, kind) in params.sounds.slots.iter().zip(expected) {
                    assert_eq!(u32::from_le_bytes(slot[..4].try_into().unwrap()), kind);
                }
                let Some(WidgetGeometry::TriangleMesh(triangles)) = &params.geometry else {
                    panic!("move widget must use the original triangle mesh");
                };
                assert_eq!(triangles.len(), 12);
                let mut min = Vec3::splat(f32::INFINITY);
                let mut max = Vec3::splat(f32::NEG_INFINITY);
                for p in triangles.iter().flat_map(|t| t.points) {
                    min = min.min(p);
                    max = max.max(p);
                }
                assert_eq!(min, anchor - Vec3::new(half_width, 0.25, half_width));
                assert_eq!(max, anchor + Vec3::new(half_width, 0.25, half_width));
            }
        }
    }

    #[test]
    fn move_gizmo_factories_keep_native_names_hover_scale_and_activation_points() {
        use crate::asset_libraries::{
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::AssetShaderLibrary,
        };
        use crate::render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice};
        use crate::resources::walls::{
            WallState, public_wall_state::PublicWallState, publication_source::TerrainHeightGrid512,
        };
        use crate::systems::ui::{
            resources::ui_state::{UiInteraction, WidgetId},
            signifiers::UiSignifierEntry,
        };
        use crate::utils::components::transform::Transform;
        use glam::{Quat, Vec3};

        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[0.0, 0.0], [4.0, 0.0]],
            false,
        )
        .unwrap();
        let mut state = WallState::new(curve, 20.0, 0, 0, false);
        state.snapshot.source.terrain_floor = 2.5;
        let shape = PublicWallState::collect(&state, &terrain).unwrap();
        let meshes = AssetMeshLibrary::new();
        for name in ["move_shape", "move_shape_outline"] {
            meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
        }
        let shaders = AssetShaderLibrary::new();
        for name in ["vertex_color_shader", "dashed_line"] {
            shaders.add_raster(
                RasterShaderProgram::new_draw(name, Default::default(), &TestDevice(1)),
                false,
            );
        }
        let anchor = Vec3::new(4.0, 2.0, -3.0);
        let start = Vec3::new(4.0, 1.0, -3.0);
        let rotation = Quat::from_rotation_y(0.5);
        let wall = WallId(17);
        let widget = WidgetId::from_name("move_shapeWallId(17)");
        for (tag, phase, matched, scale) in [
            (0, 1, true, 1.0),
            (1, 0, true, 1.0),
            (1, 1, true, 1.2),
            (2, 2, true, 1.0),
            (2, 0, true, 1.2),
            (2, 3, true, 1.2),
            (2, 3, false, 1.0),
        ] {
            let ui = UiState::new(UiInteraction {
                native_tag: tag,
                native_phase: phase,
                widget: if matched { widget } else { WidgetId(0) },
            });
            let mut stream = UiSignifierStream::default();
            move_gizmo_and_trigger_vol(
                anchor,
                rotation,
                &ActionSetStates::default(),
                &ui,
                wall,
                &shape,
                1.0,
                &mut stream,
            );
            let entries = stream.take_entries();
            assert_eq!(
                entries.len(),
                3,
                "positive elevation emits the two-point line before both axes"
            );
            let mut world = World::new();
            let mut queue = bevy_ecs::world::CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            for (i, entry) in entries.iter().enumerate() {
                let UiSignifierEntry::Factory(factory) = entry else {
                    panic!("real factory required");
                };
                let expected = if i == 0 {
                    format!("move_shape_line{wall:?}{anchor:?}{start}{anchor}")
                } else {
                    let mesh = ["move_shape", "move_shape_outline"][i - 1];
                    format!("move_shape{wall:?}{anchor:?}{mesh}{scale}{rotation:?}")
                };
                assert_eq!(factory.name(), Some(expected.as_str()));
                assert_eq!(factory.wake(&meshes, &shaders, &mut commands).len(), 1);
            }
            drop(commands);
            queue.apply(&mut world);
            // Native workers keep flag8=0 and duration=0: creation starts at
            // zero scale; the real animation stage establishes the target.
            for (transform, animation) in world
                .query::<(
                    &Transform,
                    &crate::systems::ui::signifiers::SignifierAnimation,
                )>()
                .iter(&world)
            {
                assert_eq!(transform.scale, Vec3::ZERO);
                assert_eq!(
                    (animation.flag_8, animation.flag_9, animation.duration),
                    (0, 1, 0.0)
                );
            }
            use bevy_ecs::system::RunSystemOnce;
            world.insert_resource(UiState::at_startup());
            world.insert_resource(GameTime::default());
            world
                .run_system_once(crate::systems::ui::signifiers::animation::signifier_animation_t)
                .unwrap();
            world
                .run_system_once(crate::systems::ui::signifiers::animation::signifier_animation)
                .unwrap();
            let transforms: Vec<_> = world.query::<&Transform>().iter(&world).copied().collect();
            assert_eq!(transforms.len(), 3);
            let axes: Vec<_> = transforms
                .iter()
                .filter(|t| t.translation == anchor)
                .collect();
            assert_eq!(axes.len(), 2);
            for axis in axes {
                assert_eq!(axis.rotation, rotation);
                assert_eq!(axis.scale, Vec3::splat(scale * 1.5));
            }
            let lengths: Vec<_> = world
                .query::<&crate::components::DashedLineLength>()
                .iter(&world)
                .map(|x| x.0)
                .collect();
            assert_eq!(lengths, vec![1.0]);
        }
    }

    #[test]
    fn cache_default_has_empty_grab() {
        let cache = MoveShapeCache::default();
        assert!(cache.entry.decorator_triples.is_empty());
    }

    #[test]
    fn snap_height_gate_preserves_unordered_branch() {
        assert!(!roof_has_snap_height(0.1));
        assert!(roof_has_snap_height(f32::from_bits(0x3dccccce)));
        assert!(roof_has_snap_height(f32::NAN));
    }

    #[test]
    fn camera_rotate_reads_digital_slot_2_phase() {
        use crate::input::action_set::{ActionSetStates, DigitalActionPhase};
        let mk = |phase: DigitalActionPhase| {
            let mut actions = ActionSetStates::default();
            actions.digital[2].phase = phase;
            actions
        };
        assert!(camera_rotate_pressed(&mk(DigitalActionPhase::JustPressed)));
        assert!(camera_rotate_pressed(&mk(DigitalActionPhase::Held)));
        assert!(camera_rotate_pressed(&mk(DigitalActionPhase::Repeating)));
        assert!(!camera_rotate_pressed(&mk(
            DigitalActionPhase::JustReleased
        )));
        assert!(!camera_rotate_pressed(&mk(DigitalActionPhase::Idle)));
    }

    #[test]
    fn case1_drag_writes_working_record_and_decorator_triples() {
        use crate::resources::mouse_terrain_raycast::CameraCursorRay;
        use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
        use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
        use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
        use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
        use crate::systems::collision::world::RaycastWorld;
        use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        use bevy_ecs::system::RunSystemOnce;

        #[derive(bevy_ecs::resource::Resource)]
        struct TestEntry(MoveShapeEntry);

        let wall = WallId(7);
        // Preset ShapeParameters (circle center (2,3), r 4) at shape_rec[0x258..0x274].
        let circle = ::utils::geometry::Circle2d::new(glam::Vec2::new(2.0, 3.0), 4.0);
        let sp = ShapeParameters::from_shape(Shape::Circle(circle)).to_record();
        let mut entry = MoveShapeEntry {
            state: 2,
            activation: 1.0,
            variant: 1,
            ..MoveShapeEntry::default()
        };
        entry.shape_rec[0x258..0x274].copy_from_slice(&sp);

        let mut world = World::new();
        world.insert_resource(TestEntry(entry));
        world.insert_resource(CameraCursorRay {
            origin: glam::Vec3::new(2.0, 10.0, 3.0),
            dir: glam::Vec3::new(0.0, -1.0, 0.0),
        });
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::default());
        let mut storage = DecoratorStorage::default();
        let mut info = DecoratorInfo::default();
        info.dst_payload = (1.0f32.to_bits() as u64) << 32; // Info+0x14
        info.u18 = (2.0f32.to_bits() as u64) << 32; // Info+0x1c
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(11), info);
        let mut compound = info;
        compound.u20 = 1 << 16;
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(12), compound);
        let mut roof = info;
        roof.u20 = 2 << 16;
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(13), roof);
        world.insert_resource(storage);
        world.insert_resource(LiveDecoratorArchivist::default());
        world.insert_resource(RaycastWorld::default());
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();

        world
            .run_system_once(
                move |ray: Res<CameraCursorRay>,
                      terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
                      mut storage: ResMut<DecoratorStorage>,
                      mut archivist: ResMut<LiveDecoratorArchivist>,
                      archival: DecoratorArchivalResources,
                      mut entry: ResMut<TestEntry>| {
                    case1_drag(
                        &mut entry.0,
                        wall,
                        &ray,
                        &terrain,
                        &mut storage,
                        &mut archivist,
                        &archival,
                    );
                },
            )
            .unwrap();

        let e = &world.resource::<TestEntry>().0;
        // Vec header len == 1 (one wall decorator).
        let len = u32::from_le_bytes(e.shape_rec[0x10..0x14].try_into().unwrap());
        assert_eq!(len, 2, "wall and compound decorators only");
        // +0x20..0x3c is a verbatim copy of the 28B ShapeParameters.
        assert_eq!(&e.shape_rec[0x20..0x3c], &e.shape_rec[0x258..0x274]);
        // Decorator triples: {id, d[-0x34]-cx, d[-0x2c]-cz} = {11, 1.0-2.0, 2.0-3.0}.
        assert_eq!(e.decorator_triples.len(), 2);
        for &(id, dx, dz) in &e.decorator_triples {
            assert!([11, 12].contains(&id));
            assert!((dx - (1.0 - 2.0)).abs() < 1e-5, "dx {dx}");
            assert!((dz - (2.0 - 3.0)).abs() < 1e-5, "dz {dz}");
        }
        // +0x3c = anchor.y = terrain height sampled at center (default grid min).
        let anchor_y = f32::from_le_bytes(e.shape_rec[0x3c..0x40].try_into().unwrap());
        let expected_y =
            TerrainHeightsData::<TerrainHeightSsbo>::default().sample_at_world_xz(2.0, 3.0);
        assert_eq!(anchor_y, expected_y);
        for tag in [3u32, 4] {
            let mut invalid = DecoratorInfo::default();
            invalid.u20 = tag << 16;
            world
                .resource_mut::<DecoratorStorage>()
                .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
                .insert(DecoratorId(99), invalid);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                world
                    .run_system_once(
                        move |ray: Res<CameraCursorRay>,
                              terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
                              mut storage: ResMut<DecoratorStorage>,
                              mut archivist: ResMut<LiveDecoratorArchivist>,
                              archival: DecoratorArchivalResources,
                              mut entry: ResMut<TestEntry>| {
                            case1_drag(
                                &mut entry.0,
                                wall,
                                &ray,
                                &terrain,
                                &mut storage,
                                &mut archivist,
                                &archival,
                            );
                        },
                    )
                    .unwrap();
            }));
            assert!(result.is_err(), "destination tag {tag} must panic");
            world
                .resource_mut::<DecoratorStorage>()
                .get_mut(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .remove(DecoratorId(99));
        }
    }
}

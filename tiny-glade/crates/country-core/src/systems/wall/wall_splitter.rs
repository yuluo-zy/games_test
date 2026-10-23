//! B `systems/wall/wall_splitter.rs`: `wall_splitter` (0x140a74c20).
//!
//! While a wall is being edited (`AppMode::Editing(EditSubject::Wall(id))`) this
//! system offers the "scissors" interaction on that wall:
//!
//! 1. every frame it registers a mesh-volume UI widget `split_wall_{WallId}`
//!    made of two triangles per wall segment (bottom edge, terrain-relative top
//!    edge). Nothing else happens until that widget is the active widget
//!    (`UiWidgetContext::is_active`);
//! 2. while active it projects the cursor hit onto the mapped wall outline,
//!    keeps the split at least `0.5` world units away from both wall ends,
//!    and shows a vertical line plus the `ui_scissors` mesh at the split point;
//! 3. the widget's click byte (`UiState` +0x19, 2 when the widget is not the
//!    active one) selects the action: 3 (drag started) resets the eraser
//!    archivist and caches the original wall; 2 and 4 do nothing; every other
//!    value (0, 1, 5..) splits the wall in two.
//!
//! Parameter surface, original order: `Res<AppMode>`, `Res<PublicWalls>`,
//! the raycast/collider pair (one derived SystemParam struct in the original),
//! `Res<UiState>`, `UiSignifierStream`, `WallEraserArchivist`, `InnerWalls`,
//! `DecoratorStorage`, `DecoratorArchivist`, `History`, `WallColorIds`,
//! `HalftimberColorIds`, terrain heights and the four event writers
//! (`SetWallColorIdCmd`, `SetHalftimberColorIdCmd`, `RequestAppModeChange`,
//! `PlayWallAudio`, again one derived struct in the original).
use std::sync::Arc;

use bevy_ecs::{error::Result, prelude::*, system::SystemParam};
use glam::{Mat3, Quat, Vec2, Vec3};
use utils::{curve::Curve3, geometry::intersect::Triangle};

use crate::{
    asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    components::drawable::DrawableMeshBundle,
    resources::{
        color_id::{
            HalftimberColorIds, SetHalftimberColorIdCmd, SetWallColorIdCmd, WallColorIds,
        },
        history::{EditType, History, color_edit::ColorEdit, wall_edit::WallHistoryMetadata},
        history::wall_edit::WallAuthoritySnapshot,
        roofs::geometry::surface::Curve2,
        terrain_editing::StrokeId,
        terrain_height::TerrainHeightsData,
        walls::{
            InnerWalls, WallId, WallState,
            decorator_archivist::LiveDecoratorArchivist,
            decorator_storage::{DecoratorStorage, Decorators},
            public_walls::PublicWalls,
            publication_fields::WallShapeSnapshot,
            publication_source::RawWallPublicationSource,
        },
    },
    systems::{
        audio::PlayWallAudio,
        mode_manager::{AppMode, EditSubject, RequestAppModeChange},
        render_loop::draw_passes::draw_lists::{DontCastShadow, UiPass},
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{WidgetParams, WidgetVolumeShape},
            },
            signifiers::{
                animation::SignifierAnimation,
                signifier_create::UiSignifierStream,
                signifier_factory::SignifierFactory,
            },
        },
        wall::eraser::archivist::{ErasedRange, WallEraserArchivist},
    },
    utils::{components::transform::Transform, debug_viz::DebugLine},
};

/// Original `DEV_OVERRIDE_IS_DEMO_MODE` (0x143634d90; `tiny_glade::main` always
/// writes 0). Feeds the new walls' metadata byte +0x69 (`demo ? 0 : 5`).
const DEV_OVERRIDE_IS_DEMO_MODE: bool = false;

/// The split keeps this many world units between the cut and either wall end
/// (`0.5 / curve length`, original constant 0x142925870).
const MIN_END_DISTANCE: f32 = 0.5;

/// `PlayWallAudio` kind written once the split is finished (slot +0x08 = 0x15).
const SPLIT_AUDIO_KIND: u8 = 0x15;

/// The scissors mesh and its shader (both closures of the original use the
/// 19-byte shader name at 0x142a9948c).
const SCISSORS_MESH: &str = "ui_scissors";
const SIGNIFIER_SHADER: &str = "vertex_color_shader";

/// Raycast world and decorator collider maps: one derived SystemParam struct in
/// the original, only forwarded to the archivist's collider payload rebuild
/// (a registered gap of the local archivist).
#[derive(SystemParam)]
pub struct SplitCollisionParams<'w> {
    #[allow(dead_code)]
    raycast: Res<'w, crate::systems::collision::world::RaycastWorld>,
    #[cfg(feature = "native-physics")]
    #[allow(dead_code)]
    colliders: Res<'w, crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>,
}

/// The four event writers of the original's last (derived) parameter, in order.
#[derive(SystemParam)]
pub struct SplitEventWriters<'w> {
    wall_colors: EventWriter<'w, SetWallColorIdCmd>,
    halftimber_colors: EventWriter<'w, SetHalftimberColorIdCmd>,
    app_mode: EventWriter<'w, RequestAppModeChange>,
    wall_audio: EventWriter<'w, PlayWallAudio>,
}

/// Explicit error paths (original: panics inside the eraser archivist).
#[derive(Debug, PartialEq)]
pub enum WallSplitterError {
    /// `add_split_event` needs the eraser's rewire map (map1) and child walls
    /// (map3) to contain the wall (original: `expect` panic). `cache_if_original_wall`
    /// (Start) now creates the map3 entry like 0x14092ee80, so this only fires when Split
    /// runs without a preceding Start; it is refused *before* any state is touched.
    EraserSplitTargetMissing(WallId),
    /// The wall was removed by a different system between the widget being
    /// active and the click being handled.
    HistoryNotRecording,
}

impl std::fmt::Display for WallSplitterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EraserSplitTargetMissing(wall) => write!(
                f,
                "wall_splitter: eraser archivist has no split target for {wall}; \
                 cache_if_original_wall did not register its child wall (unclosed)"
            ),
            Self::HistoryNotRecording => {
                write!(f, "wall_splitter: History::add without an active recording")
            }
        }
    }
}

impl std::error::Error for WallSplitterError {}

/// `utils::xz_projective_rotation_bad` (0x140c911d0, Vec3 -> Quat): the XZ part
/// of `dir` is normalised (fallback `(1, 0)` when the reciprocal length is not a
/// positive finite number), the basis is `x = (fz, 0, -fx)`, `y = Y`,
/// `z = (fx, 0, fz)` and the rotation is the standard matrix-to-quaternion
/// conversion.
pub fn xz_projective_rotation_bad(dir: Vec3) -> Quat {
    let reciprocal = 1.0 / (dir.z * dir.z + dir.x * dir.x).sqrt();
    let (fx, fz) = if reciprocal > 0.0 && reciprocal.is_finite() {
        (dir.x * reciprocal, dir.z * reciprocal)
    } else {
        (1.0, 0.0)
    };
    let lanes = Vec3::new(fz * 1.0 - 0.0, fx * 0.0 - fz * 0.0, 0.0 * 0.0 - 1.0 * fx);
    let length = (lanes.z * lanes.z + lanes.x * lanes.x + lanes.y * lanes.y).sqrt();
    let right = Vec3::new(lanes.x / length, lanes.y / length, lanes.z / length);
    Quat::from_mat3(&Mat3::from_cols(right, Vec3::Y, Vec3::new(fx, 0.0, fz)))
}

/// Volume of the `split_wall_*` widget: two triangles per segment of the zipped
/// (mapped outline point, relative height) sequence. Bottom triangle
/// `(p_prev, p_cur, top_prev)`, top triangle `(p_cur, top_cur, top_prev)` where
/// `top = (x, relative_height, z)`.
pub fn splitter_triangles(outline: &[[f32; 3]], relative_heights: &[f32]) -> Vec<Triangle> {
    // Original: `points.len().checked_sub(..)` unwrap before allocating the
    // `(2 * n - 2)` triangle buffer; an empty outline is a hard failure.
    assert!(!outline.is_empty(), "split wall outline is empty");
    let mut triangles = Vec::with_capacity(outline.len() * 2 - 2);
    let mut previous: Option<(Vec3, f32)> = None;
    for (&point, &height) in outline.iter().zip(relative_heights.iter()) {
        let current = (Vec3::from_array(point), height);
        if let Some((prev_point, prev_height)) = previous {
            let top_prev = Vec3::new(prev_point.x, prev_height, prev_point.z);
            let top_cur = Vec3::new(current.0.x, current.1, current.0.z);
            triangles.push(Triangle::new(prev_point, current.0, top_prev));
            triangles.push(Triangle::new(current.0, top_cur, top_prev));
        }
        previous = Some(current);
    }
    triangles
}

/// Result of projecting the cursor hit onto the wall.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplitPoint {
    /// Normalised wall coordinate of the cut (kept `0.5` units off both ends).
    pub u: f32,
    /// Point on the mapped outline.
    pub position: Vec3,
    /// `position` with y replaced by the interpolated relative height.
    pub top: Vec3,
    /// Wall tangent rotation (scissors orientation).
    pub rotation: Quat,
}

/// The per-frame hover maths (0x140a755c2..0x140a758a0).
pub fn compute_split_point(
    curve: &Curve3,
    relative_heights: &[f32],
    wall_length: f32,
    hit: Vec3,
) -> SplitPoint {
    let coord = curve.get_approx_coord_from_pos(Vec2::new(hit.x, hit.z));
    let (segment, t) = (coord.segment, coord.t);
    assert!(segment < curve.points_u.len());
    assert!(segment + 1 < curve.points_u.len());
    assert!((0.0..=1.0).contains(&t), "assertion failed: (0.0..=1.0).contains(&t)");
    // utils lerp(u[segment], u[segment + 1], t)
    let u0 = curve.points_u[segment];
    let u1 = curve.points_u[segment + 1];
    let u = u0 + (u1 - u0) * t;

    let margin = MIN_END_DISTANCE / wall_length;
    let curve_u = |value: f32| {
        assert!(
            (0.0..=1.0).contains(&value),
            "Tried to init CurveU from an invalid u: {value}"
        );
        value
    };
    let lower = curve_u(margin);
    assert!(!lower.is_nan(), "assertion failed: !u.is_nan()");
    let upper = curve_u(1.0 - lower);
    // `maxss`/`minss` with the original NaN behaviour: a NaN `u` becomes `lower`.
    let u = if u.is_nan() { lower } else { u.max(lower) };
    let u = if u.is_nan() { upper } else { u.min(upper) };

    let at = curve.get_coord_at_u(u);
    let position = curve.get_pos_at_coord(at);
    assert!(segment < relative_heights.len());
    assert!(segment + 1 < relative_heights.len());
    let a = relative_heights[segment];
    let b = relative_heights[segment + 1];
    let height = t * (b - a) + a;
    let top = Vec3::new(position.x, height, position.z);

    let tangent_coord = curve.get_coord_at_u(u);
    let delta = curve.points[tangent_coord.segment + 1] - curve.points[tangent_coord.segment];
    let reciprocal = 1.0 / (delta.y * delta.y + delta.x * delta.x + delta.z * delta.z).sqrt();
    let direction = Vec3::new(reciprocal * delta.x, reciprocal * delta.y, reciprocal * delta.z);
    SplitPoint {
        u,
        position,
        top,
        rotation: xz_projective_rotation_bad(direction),
    }
}

/// The `ui_scissors` signifier (closure of 0x140a77dd0): the library mesh at
/// `top + (0, 0.1, 0)` with the tangent rotation. The shader is
/// `vertex_color_shader`; a missing mesh or shader spawns nothing.
fn scissors_factory(name: &str, rotation: Quat, top: Vec3) -> SignifierFactory {
    SignifierFactory::from_closure(
        name,
        Arc::new(
            move |meshes: &AssetMeshLibrary,
                  shaders: &AssetShaderLibrary,
                  commands: &mut Commands| {
                let Some(mesh) = meshes.try_get_handle_by_name(SCISSORS_MESH) else {
                    log::warn!("scissors mesh not registered: {SCISSORS_MESH}");
                    return Vec::new();
                };
                let Some(shader) = shaders.get_raster_handle_by_name(SIGNIFIER_SHADER) else {
                    log::warn!("signifier shader not registered: {SIGNIFIER_SHADER}");
                    return Vec::new();
                };
                let transform = Transform {
                    rotation,
                    translation: Vec3::new(top.x + 0.0, top.y + 0.1, top.z + 0.0),
                    scale: Vec3::ONE,
                };
                let entity = commands
                    .spawn((
                        DrawableMeshBundle {
                            mesh,
                            shader,
                            transform,
                        },
                        DontCastShadow,
                            SignifierAnimation {
                            flag_8: 1,
                            duration: 0.0,
                            value: 1.0,
                            t: 1.0,
                            eased: 1.0,
                            ..Default::default()
                        },
                        UiPass,
                    ))
                    .id();
                vec![entity]
            },
        ),
    )
}

/// `DecoratorArchivist::archive_current_decorators_in_storage` 0x1413e7230 for
/// a wall address: archive the wall's bucket, or an empty bucket when the
/// storage has none.
fn archive_wall(archivist: &mut LiveDecoratorArchivist, storage: &DecoratorStorage, wall: WallId) {
    match storage.per_wall.get(&wall) {
        Some(decorators) => archivist.archive_storage_decorators(0, decorators, Some(wall)),
        None => archivist.archive_storage_decorators(0, &Decorators::default(), Some(wall)),
    }
}

/// State of one half of a split wall (0x140a76210..0x140a762d4): the sliced
/// curve, shape tag 2, `height`, and everything else from the removed wall
/// except style (collapses to `style == 1`), source +0x65 (the public row's
/// copy) and metadata +0x69 (demo-derived).
fn piece_state(removed: &WallState, curve: Curve2, height: f32, public_flag_65: u8) -> WallState {
    let mut shape = [0u8; 28];
    shape[..4].copy_from_slice(&2u32.to_le_bytes());
    let shape = WallShapeSnapshot::from_observed(shape);
    let source = RawWallPublicationSource {
        curve,
        curve_kind_tag: shape.tag(),
        terrain_floor: removed.elevation(),
        shape,
        style: u8::from(removed.style() == 1),
        extra: height,
        special: false,
        observed_path_scalar: removed.arch_height(),
        clutter_version: public_flag_65,
        observed_source_flag_64: removed.arch_type(),
    };
    let metadata = WallHistoryMetadata {
        source_60_bits: removed.snapshot().metadata.source_60_bits,
        source_66: Some(removed.freehand_ty()),
        source_67: removed.removed_supports(),
        source_69: if DEV_OVERRIDE_IS_DEMO_MODE { 0 } else { 5 },
    };
    WallState::from_snapshot(WallAuthoritySnapshot { source, metadata }, removed.freehand_ty())
}

fn input_curve(curve: &Curve2) -> utils::curve::Curve2 {
    utils::curve::Curve2::from_cached_parts(
        curve.points.iter().copied().map(Vec2::from_array).collect(),
        curve.points_u.clone(),
        curve.length,
    )
}

fn authority_curve(curve: &utils::curve::Curve2) -> Curve2 {
    Curve2 {
        points: curve.points.iter().map(|p| p.to_array()).collect(),
        points_u: curve.points_u.clone(),
        length: curve.length,
    }
}

/// Original 0x140a74c20; see the module documentation.
pub fn wall_splitter<S: Send + Sync + 'static>(
    app_mode: Res<AppMode>,
    public_walls: Res<PublicWalls>,
    _collision: SplitCollisionParams,
    ui_state: Res<UiState>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut eraser: ResMut<WallEraserArchivist>,
    mut inner_walls: ResMut<InnerWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    mut history: ResMut<History<S>>,
    wall_colors: Res<WallColorIds>,
    halftimber_colors: Res<HalftimberColorIds>,
    terrain: Res<TerrainHeightsData>,
    mut events: SplitEventWriters,
) -> Result {
    // Native AppMode tag 1 = Editing(Wall(id)).
    let AppMode::Editing(EditSubject::Wall(edited)) = *app_mode else {
        return Ok(());
    };
    let wall = WallId(edited);
    // The original walks every public wall id and skips all but the edited one.
    if public_walls.get_unchecked(wall).is_none() {
        return Ok(());
    }
    let public = public_walls
        .get(wall)
        .expect("PublicWalls::get after the id was found");
    // Premade shapes (tag < 2) and walls up to one unit long are not splittable.
    if public.shape().tag() < 2 || public.simplified_curve_2d().length <= 1.0 {
        return Ok(());
    }

    let outline = &public.geometry.outline;
    let triangles = splitter_triangles(outline, &public.geometry.relative_heights);
    let widget_name = format!("split_wall_{wall}");
    let widget = ui_state.add(
        &widget_name,
        WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
            .mesh_volume(triangles)
            .occlude_by_world(false),
    );
    if !widget.is_active() {
        return Ok(());
    }

    // ---- hover: split point, vertical line and scissors ----
    let hit = ui_state.intersection_point();
    let curve = Curve3::from_cached_parts(
        outline.iter().copied().map(Vec3::from_array).collect(),
        public.geometry.outline_u.clone(),
        public.geometry.mapped_curve_length,
    );
    let split = compute_split_point(
        &curve,
        &public.geometry.relative_heights,
        public.simplified_curve_2d().length,
        hit,
    );
    // Name and closure body of the line differ from the original's
    // `format!("split_wall{wall:?}{pos:?}")` factory (registered).
    DebugLine::new(vec![split.position, split.top]).show(&mut signifiers);
    let scissors_name = format!("split_wall_scissors{wall:?}{:?}", split.position);
    signifiers.write_factory(&scissors_factory(&scissors_name, split.rotation, split.top));

    inner_walls.get_unwrapped(wall);

    // ---- click byte -> action (`cVar - 2` as u8, > 2 collapses to 3) ----
    // 2 unless the widget is dragged/clicked (`UiState +0x18 > 1` and id match).
    let click = widget.dragged();
    let action = match click.wrapping_sub(2) {
        0 | 2 => return Ok(()),
        1 => SplitAction::Start,
        _ => SplitAction::Split,
    };

    match action {
        SplitAction::Start => {
            *eraser = WallEraserArchivist::new();
            archive_wall(&mut archivist, &storage, wall);
            archivist.decorator_cull_allow_wall(wall, &storage);
            let snapshot = inner_walls.get_unwrapped(wall).snapshot().clone();
            eraser.cache_if_original_wall(wall, &snapshot, ErasedRange { low: 0.0, high: 1.0 });
            Ok(())
        }
        SplitAction::Split => {
            if !eraser.has_split_target(wall) {
                return Err(WallSplitterError::EraserSplitTargetMissing(wall).into());
            }
            // Color table: source +0x65 of the removed wall selects it.
            let removed = inner_walls.remove(wall);
            let halftimber = removed.source_flag_65() != 0;
            let color_id = if halftimber {
                halftimber_colors.0[wall.0 as usize]
            } else {
                wall_colors.0[wall.0 as usize]
            };
            let removed_curve = input_curve(&removed.source().curve);
            // Cut position on the removed wall's own (2D) curve.
            let at = curve.get_coord_at_u(split.u);
            let cut = curve.get_pos_at_coord(at);
            let split_u = removed_curve.get_approx_u_from_pos(Vec2::new(cut.x, cut.z));
            // `maxss(0, u)` then `minss(1, ..)`: NaN propagates.
            let split_u = if 0.0 > split_u { 0.0 } else { split_u };
            let split_u = if 1.0 < split_u { 1.0 } else { split_u };

            let mut pieces: Vec<(u64, f32, f32)> = Vec::with_capacity(2);
            for (low, high) in [(0.0, split_u), (split_u, 1.0)] {
                let slice = removed_curve.crashy_slice(low, high);
                let height = if removed.is_premade_shape() || removed.freehand_ty() == 0 {
                    removed.height()
                } else {
                    let mut highest = -2.5f32;
                    for point in &slice.points {
                        highest = highest.max(terrain.sample_at_world_xz(point.x, point.y));
                    }
                    public.max_y() - highest
                };
                let state = piece_state(
                    &removed,
                    authority_curve(&slice),
                    height,
                    public.authority.source_flag_65(),
                );
                let id = inner_walls.new_wall_from_curve(state);
                assert!(low <= high, "assertion failed: u_start <= u_end");
                pieces.push((id.0, low, high));

                // The new wall's pre-split decorator bucket is empty.
                archivist.archive_storage_decorators(0, &Decorators::default(), Some(id));
                if halftimber {
                    events
                        .halftimber_colors
                        .write(SetHalftimberColorIdCmd::new(id, color_id, 0));
                    add_history(
                        &mut history,
                        ColorEdit::Halftimber {
                            wall_id: id,
                            from: StrokeId(color_id),
                            to: StrokeId(color_id),
                        },
                    )?;
                } else {
                    events
                        .wall_colors
                        .write(SetWallColorIdCmd::new(id, color_id, 0));
                    add_history(
                        &mut history,
                        ColorEdit::Wall {
                            wall_id: id,
                            from: StrokeId(color_id),
                            to: StrokeId(color_id),
                        },
                    )?;
                }
            }

            eraser.add_split_event_pieces(wall, &pieces);
            archive_wall(&mut archivist, &storage, wall);
            archivist.decorator_cull_allow_wall(wall, &storage);
            let children: Vec<(WallId, f32, f32)> = pieces
                .iter()
                .map(|&(id, low, high)| (WallId(id), low, high))
                .collect();
            storage.on_wall_split(wall, &removed_curve, &children);
            let compound = eraser.finish(&wall_colors, &halftimber_colors);
            history
                .add(EditType::Wall(Arc::new(compound)))
                .map_err(|_| WallSplitterError::HistoryNotRecording)?;

            events
                .app_mode
                .write(RequestAppModeChange(AppMode::Editing(EditSubject::None)));
            // The widget must still be the active one (original `Option::unwrap`).
            assert!(
                widget.is_active(),
                "called `Option::unwrap()` on a `None` value"
            );
            events.wall_audio.write(PlayWallAudio {
                kind: SPLIT_AUDIO_KIND,
                value: 0.0,
                position: ui_state.intersection_point(),
                volume: 1.0,
                flag: 0,
            });
            Ok(())
        }
    }
}

enum SplitAction {
    Start,
    Split,
}

fn add_history<S>(history: &mut History<S>, edit: ColorEdit) -> std::result::Result<(), WallSplitterError> {
    history
        .add(EditType::Color(Arc::new(edit)))
        .map_err(|_| WallSplitterError::HistoryNotRecording)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resources::walls::publication_source::TerrainHeightGrid512,
        systems::{
            audio::PlayWallAudio,
            ui::resources::ui_state::{UiInteraction, WidgetId},
        },
    };
    use bevy_ecs::system::RunSystemOnce;

    fn terrain_grid() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        }
    }

    struct Fixture {
        world: World,
        wall: WallId,
    }

    fn widget_name(wall: WallId) -> String {
        format!("split_wall_{wall}")
    }

    /// `click`: the UiState click byte; `hover`: hover byte (0 = inactive).
    fn fixture(points: &[[f32; 2]], editing: bool, hover: u8, click: u8) -> Fixture {
        let mut world = World::new();
        let mut inner = InnerWalls::default();
        let curve = Curve2::try_new(points.to_vec(), false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 4.0, 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &terrain_grid());
        world.insert_resource(public);
        world.insert_resource(inner);
        world.insert_resource(if editing {
            AppMode::Editing(EditSubject::Wall(wall.0))
        } else {
            AppMode::default()
        });
        let mut ui = UiState::new(UiInteraction {
            native_tag: hover,
            native_phase: click,
            widget: WidgetId::from_name(&widget_name(wall)),
        });
        ui.set_intersection_tag(0);
        ui.set_intersection_point(Vec3::new(5.0, 0.0, 0.0));
        world.insert_resource(ui);
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(WallEraserArchivist::new());
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(5).unwrap();
        world.insert_resource(history);
        world.insert_resource(WallColorIds::new());
        world.insert_resource(HalftimberColorIds::default());
        world.init_resource::<TerrainHeightsData>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<Events<SetWallColorIdCmd>>();
        world.init_resource::<Events<SetHalftimberColorIdCmd>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        world.init_resource::<Events<PlayWallAudio>>();
        Fixture { world, wall }
    }

    fn run(f: &mut Fixture) -> Result {
        f.world
            .run_system_once(wall_splitter::<()>)
            .expect("system parameters are available")
    }

    const LONG: [[f32; 2]; 2] = [[0.0, 0.0], [10.0, 0.0]];

    #[test]
    fn outside_wall_edit_mode_registers_nothing() {
        let mut f = fixture(&LONG, false, 2, 3);
        run(&mut f).unwrap();
        let id = WidgetId::from_name(&widget_name(f.wall));
        assert!(f.world.resource::<UiState>().widget(id).is_none());
    }

    #[test]
    fn walls_up_to_one_unit_long_are_not_splittable() {
        let mut f = fixture(&[[0.0, 0.0], [1.0, 0.0]], true, 2, 3);
        run(&mut f).unwrap();
        let id = WidgetId::from_name(&widget_name(f.wall));
        assert!(f.world.resource::<UiState>().widget(id).is_none());
    }

    #[test]
    fn inactive_widget_registers_mesh_volume_only() {
        let mut f = fixture(&LONG, true, 0, 3);
        run(&mut f).unwrap();
        let id = WidgetId::from_name(&widget_name(f.wall));
        let params = f.world.resource::<UiState>().widget(id).expect("widget");
        assert_eq!(params.volume.tag, 2);
        let points = f.world.resource::<PublicWalls>().get(f.wall).unwrap().geometry.outline.len();
        assert!(points > 2, "the mapped outline is resampled");
        let geometry = params.resolved_geometry();
        let crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(triangles)
            = geometry.as_ref() else { panic!("splitter must register complete triangle mesh") };
        assert_eq!(triangles.len(), 2 * (points - 1));
        assert!(!params.occlude_by_world);
        assert_eq!(f.world.resource::<UiSignifierStream>().committed_len(), 0);
        assert!(f.world.resource::<InnerWalls>().contains(f.wall));
    }

    #[test]
    fn active_hover_shows_line_and_scissors_without_editing() {
        // hover byte 1: active, but the drag state is None (click byte reads 2).
        let mut f = fixture(&LONG, true, 1, 3);
        run(&mut f).unwrap();
        assert_eq!(f.world.resource::<UiSignifierStream>().committed_len(), 2);
        assert!(!f.world.resource::<WallEraserArchivist>().has_anything_been_affected());
        assert!(f.world.resource::<InnerWalls>().contains(f.wall));
    }

    #[test]
    fn click_bytes_two_and_four_do_nothing() {
        for click in [2u8, 4] {
            let mut f = fixture(&LONG, true, 2, click);
            run(&mut f).unwrap();
            assert!(!f.world.resource::<WallEraserArchivist>().has_anything_been_affected());
            assert!(f.world.resource::<InnerWalls>().contains(f.wall));
            assert_eq!(f.world.resource::<Events<RequestAppModeChange>>().len(), 0);
        }
    }

    #[test]
    fn drag_start_resets_eraser_and_caches_original_wall() {
        let mut f = fixture(&LONG, true, 2, 3);
        // A previous session's cache is dropped by the reset.
        f.world
            .resource_mut::<WallEraserArchivist>()
            .cache_if_original_wall(
                WallId(99),
                &WallState::new(
                    Curve2::try_new(LONG.to_vec(), false).unwrap(),
                    1.0,
                    0,
                    0,
                    false,
                )
                .snapshot()
                .clone(),
                ErasedRange { low: 0.0, high: 1.0 },
            );
        run(&mut f).unwrap();
        let eraser = f.world.resource::<WallEraserArchivist>();
        assert!(eraser.affected_walls().eq([f.wall]));
        assert_eq!(f.world.resource::<LiveDecoratorArchivist>().all_storages().count(), 1);
        assert!(f.world.resource::<InnerWalls>().contains(f.wall));
    }

    #[test]
    fn finished_click_without_eraser_split_target_is_an_explicit_error() {
        let mut f = fixture(&LONG, true, 2, 1);
        let error = run(&mut f).expect_err("split must be refused");
        assert!(error.to_string().contains("no split target"));
        // Refused before any state is touched.
        assert!(f.world.resource::<InnerWalls>().contains(f.wall));
        assert_eq!(f.world.resource::<InnerWalls>().all().count(), 1);
        assert_eq!(f.world.resource::<Events<RequestAppModeChange>>().len(), 0);
        assert!(f.world.resource::<History<()>>().pending().unwrap().edits.is_empty());
    }

    #[test]
    fn triangles_cover_each_segment_with_bottom_and_top_triangles() {
        let outline = [[0.0, 1.0, 0.0], [2.0, 1.5, 0.0], [4.0, 2.0, 1.0]];
        let heights = [3.0, 3.5, 4.0];
        let tris = splitter_triangles(&outline, &heights);
        assert_eq!(tris.len(), 4);
        assert_eq!(
            tris[0].points,
            [Vec3::new(0.0, 1.0, 0.0), Vec3::new(2.0, 1.5, 0.0), Vec3::new(0.0, 3.0, 0.0)]
        );
        assert_eq!(
            tris[1].points,
            [Vec3::new(2.0, 1.5, 0.0), Vec3::new(2.0, 3.5, 0.0), Vec3::new(0.0, 3.0, 0.0)]
        );
        assert_eq!(tris[3].points[1], Vec3::new(4.0, 4.0, 1.0));
        // zip stops at the shorter sequence.
        assert_eq!(splitter_triangles(&outline, &heights[..2]).len(), 2);
    }

    #[test]
    fn split_point_keeps_half_a_unit_from_both_ends() {
        let curve = Curve3::try_new_from_points(vec![Vec3::new(0.0, 1.0, 0.0), Vec3::new(10.0, 1.0, 0.0)], false)
            .unwrap();
        let heights = [3.0, 5.0];
        let middle = compute_split_point(&curve, &heights, 10.0, Vec3::new(5.0, 0.0, 0.0));
        assert!((middle.u - 0.5).abs() < 1e-6);
        assert_eq!(middle.position, Vec3::new(5.0, 1.0, 0.0));
        // relative height uses the cursor's own coordinate, not the clamped one.
        assert_eq!(middle.top, Vec3::new(5.0, 4.0, 0.0));
        let near_start = compute_split_point(&curve, &heights, 10.0, Vec3::new(0.2, 0.0, 0.0));
        assert!((near_start.u - 0.05).abs() < 1e-6);
        assert!((near_start.position.x - 0.5).abs() < 1e-5);
        let near_end = compute_split_point(&curve, &heights, 10.0, Vec3::new(9.9, 0.0, 0.0));
        assert!((near_end.u - 0.95).abs() < 1e-6);
    }

    #[test]
    fn rotation_follows_the_wall_tangent_in_the_xz_plane() {
        let along_x = xz_projective_rotation_bad(Vec3::X);
        assert!(along_x.abs_diff_eq(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), 1e-6));
        // A vertical tangent falls back to (1, 0) in XZ.
        assert_eq!(xz_projective_rotation_bad(Vec3::Y), along_x);
        let along_z = xz_projective_rotation_bad(Vec3::Z);
        assert!(along_z.abs_diff_eq(Quat::IDENTITY, 1e-6));
    }

    #[test]
    fn piece_state_collapses_style_and_uses_demo_derived_metadata() {
        let mut state = WallState::new(
            Curve2::try_new(LONG.to_vec(), false).unwrap(),
            4.0,
            2,
            0,
            false,
        );
        state.snapshot.source.style = 2;
        let piece = piece_state(&state, state.source().curve.clone(), 1.5, 7);
        assert_eq!(piece.style(), 0);
        assert_eq!(piece.height(), 1.5);
        assert_eq!(piece.source_flag_65(), 7);
        assert_eq!(piece.snapshot().metadata.source_69, 5);
        assert_eq!(piece.source().curve_kind_tag, 2);
        state.snapshot.source.style = 1;
        assert_eq!(piece_state(&state, state.source().curve.clone(), 1.0, 0).style(), 1);
    }
}

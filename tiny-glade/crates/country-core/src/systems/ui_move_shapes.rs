//! B `systems/ui_move_shapes.rs` — `ui_move_shapes` (VA `0x14097a060`, 12832 B, 30 params).
//!
//! Spec §3. This file lands the evidence-backed **structure**: the true 30-param FnSystem
//! signature (PDB order), the `MoveShapeCache`/`MoveShapeEntry` types, and the 4-way jump
//! dispatch (`0x142ab7514`). The **case bodies** (case1 rectangle/line drag, case2 hover/snap,
//! case3 commit) and the `ActionSetStates.digital[..]` slot/phase decision are `TODO(evidence)`
//! — the analysis line is still resolving the concrete digital slots and the per-push writer
//! binding. Nothing here is claimed complete.

use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::input::action_set::ActionSetStates;
use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::events::OnShapeMoved;
use crate::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder};
use crate::resources::history::History;
use crate::resources::mouse_terrain_raycast::CameraCursorRay;
use crate::resources::roofs::Roof;
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
use crate::resources::shape_intersections::ShapeIntersections;
use crate::systems::audio::{PlayBuildingAudio, PlayWallUiSound, RequestWallMaterialAudio};
use crate::systems::backend::input::mod_key::Modifiers;
use crate::systems::collision::world::RaycastWorld;
use crate::systems::mode_manager::{AppMode, EditSubject};
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter;
use crate::systems::wall::shapes::shape_placement::{nearest_proposal, snap_to_roof_ridge, SnapProposal};
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use crate::utils::debug_viz::{DebugColor, DebugLine, DebugSphere};

/// B `MoveShapeEntry` (per hashbrown bucket, stride `0x2a8`, negative offsets). Field names
/// synthesized from the offsets; the key (`WallId`) is the map key.
#[derive(Clone, Debug)]
pub struct MoveShapeEntry {
    /// `-0x290` u64; `< 2 → assert panic`.
    pub state: u64,
    /// `-0x270` f32; `<= 0.0 → assert panic` (also `shape_rec+0x30` height source).
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

/// B `MoveShapeCache` (`Local`). Native holds multiple hashbrown SwissTables + hashers; here
/// modelled as the two `WallId → MoveShapeEntry` maps decoded from the offsets (map1 @+0x00,
/// map3 @+0x30). Empty (both maps empty) = native `items == 0` → skip queries.
#[derive(Default, Debug)]
pub struct MoveShapeCache {
    /// Primary map (`HashMap<WallId, MoveShapeEntry>`).
    pub map1: HashMap<WallId, MoveShapeEntry>,
    /// Secondary map (`HashMap<WallId, MoveShapeEntry>`).
    pub map3: HashMap<WallId, MoveShapeEntry>,
}

impl MoveShapeCache {
    fn get(&self, wall: WallId) -> Option<&MoveShapeEntry> {
        self.map1.get(&wall).or_else(|| self.map3.get(&wall))
    }
    fn is_empty(&self) -> bool {
        self.map1.is_empty() && self.map3.is_empty()
    }
}

/// Outcome of the case3 commit (variant == 3); the system writes these events after the helper
/// mutates `InnerWalls`/`History` (kept out of the helper so it stays unit-testable).
struct CommitMoveOutcome {
    emit_on_shape_moved: bool,
    emit_show_pins: bool,
}

/// B case3 commit (`code_0x14097af78`, recon 849-987). Applies the in-transit moved curve to
/// `InnerWalls`, records one Wall `StateChange` compound edit (native `edit_type = 7`), and
/// reports the follow-up events. Native step 3 (0x30 ring row) → `OnShapeMoved`; step 8 (0x18
/// row) → `ShowPinnedDecoClutter` (offset layout known; per-push target still under review).
fn commit_move_shape<S: Send + Sync + 'static>(
    entry: &mut MoveShapeEntry,
    wall: WallId,
    inner: &mut InnerWalls,
    history: &mut History<S>,
) -> CommitMoveOutcome {
    // 1: update_wall_state(curve) once (native skips when the commit sentinel is already set).
    if !entry.committed {
        if let Some(curve) = entry.curve.clone() {
            inner.update_wall_state_curve(wall, curve);
        }
        entry.committed = true;
    }
    // 5-7: CompoundEditBuilder::new_edit_sequence → next → build → History::add(type 7).
    // Native case3 single edit is `ChangeSupports` (variant disc 0x8000…000b; payload
    // {wall@+8, supports=1}) — NOT `StateChange`.
    let mut builder = crate::resources::history::wall_edit::CompoundEditBuilder::default();
    builder.new_edit_sequence();
    builder.next(crate::resources::history::wall_edit::WallHistoryEdit::ChangeSupports(
        crate::resources::history::wall_edit::WallValueEdit {
            wall_index: wall.0,
            before: true,
            after: true,
        },
    ));
    let compound = builder.build();
    let _ = history.add(crate::resources::history::EditType::Wall(std::sync::Arc::new(compound)));
    CommitMoveOutcome {
        emit_on_shape_moved: true,
        emit_show_pins: true,
    }
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
        &[DecoratorStorageAddr::Wall(wall), DecoratorStorageAddr::Terrain],
        storage,
        archival,
        "move_shape",
    );
    archivist.decorator_cull_allow_wall(wall, storage);

    // ShapeParameters (28B) at shape_rec[0x258..0x274]; center() per tag.
    let sp_bytes: [u8; 28] = entry.shape_rec[0x258..0x274].try_into().unwrap();
    let sp = ShapeParameters::from_snapshot(&WallShapeSnapshot::from_observed(sp_bytes));
    let center = sp.center();

    // step3 collect decorator triples {id, d[-0x34]-cx, d[-0x2c]-cz}; d[+0x1c]=hi(u18), d[+0x24]=u24.
    let mut triples: Vec<(u32, f32, f32)> = Vec::new();
    if let Some(decorators) = storage.wall_decorators_mut(wall) {
        for (id, info) in decorators.iter_maybe_mut() {
            let dx = f32::from_bits((info.u18 >> 32) as u32) - center.x;
            let dz = f32::from_bits(info.u24) - center.y;
            triples.push((id, dx, dz));
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
    app_mode: Res<AppMode>,                                   // 1
    public: Res<PublicWalls>,                                 // 2
    mut inner: ResMut<InnerWalls>,                            // 3
    roofs: Query<&Roof>,                                      // 4
    camera_cursor_ray: Res<CameraCursorRay>,                  // 5
    cursor_terrain: Res<CursorTerrainRaycast>,                // 6
    mut audio_cursor: EventReader<AudioCursorMoved>,          // 7
    raycast_world: Res<RaycastWorld>,                         // 8
    mut decorator_storage: ResMut<DecoratorStorage>,          // 9
    mut archivist: ResMut<LiveDecoratorArchivist>,            // 10
    ui_state: Res<UiState>,                                   // 11
    mut signifiers: ResMut<UiSignifierStream>,                // 12
    mut cache: Local<MoveShapeCache>,                         // 13
    mut history: ResMut<History<S>>,                          // 14
    actions: Res<ActionSetStates>,                            // 15
    mut material_audio: EventWriter<RequestWallMaterialAudio>, // 16
    mut building_audio: EventWriter<PlayBuildingAudio>,       // 17
    mut wall_ui_sound: EventWriter<PlayWallUiSound>,          // 18
    mut on_shape_moved: EventWriter<OnShapeMoved>,            // 19
    mut sheep_check: EventWriter<CmdCheckForSheepInsideBuilding>, // 20
    mut hide_trees: EventWriter<HideTreesinAreaCmd>,          // 21
    mut show_pins: EventWriter<ShowPinnedDecoClutter>,        // 22
    archival: DecoratorArchivalResources,                     // 23
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,      // 24
    border: Res<GladeBorder>,                                 // 25
    mut out_of_border: EventWriter<DisplayOutOfBorder>,       // 26
    mut feedback_out_of_border: EventWriter<FeedbackInputOutOfBorder>, // 27
    time: Res<GameTime>,                                      // 28
    modifiers: Res<Modifiers>,                                // 29
    intersections: Res<ShapeIntersections>,                   // 30
    // SEAM (non-PDB top-level param): CF maps native `param_2[4]` to `WallSpatialHash`; case2
    // (hover-snap) queries it via `nearest_proposal`. PublicWalls is NOT extended.
    spatial: Res<WallSpatialHash>,
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
    // AudioCursorMoved: consume the channel for this frame.
    let _ = audio_cursor.read().last();

    // Cache empty → public tail (`move_gizmo_and_trigger_vol`).
    if cache.is_empty() {
        // TODO(evidence): move_gizmo_and_trigger_vol public tail (0x14097d280, §2).
        return;
    }
    let Some((variant, state, activation)) = cache
        .get(wall)
        .map(|entry| (entry.variant, entry.state, entry.activation))
    else {
        // TODO(evidence): public tail gizmo when the wall is not in the cache.
        return;
    };
    // Native assertions (`state>=2`, `activation>0`).
    assert!(state >= 2, "assertion failed: MoveShapeEntry state >= 2");
    assert!(activation > 0.0, "assertion failed: MoveShapeEntry activation > 0");

    // Jump table `0x142ab7514`: 0→return, 1→case1, 2→case2, 3→case3, else native halt_baddata.
    match variant {
        0 => {}
        1 => {
            // case1 drag: archive/cull decorators + write the move-shape working record.
            let entry = if cache.map1.contains_key(&wall) {
                cache.map1.get_mut(&wall)
            } else {
                cache.map3.get_mut(&wall)
            };
            if let Some(entry) = entry {
                case1_drag(
                    entry,
                    wall,
                    &camera_cursor_ray,
                    &terrain,
                    &mut decorator_storage,
                    &mut archivist,
                    &archival,
                );
            }
        }
        2 => {
            // case2 hover-snap: candidate segment → SnapProposal → roof-ridge offset →
            // DebugSphere + DebugLine signifiers (NO InnerWalls write).
            let cursor_world = cursor_terrain.pos;
            if let Some((proposal, snap_point)) =
                hover_snap(&public, &spatial, cursor_world, |entity| roofs.get(entity).ok())
            {
                let sphere = DebugSphere {
                    color: DebugColor::White,
                    radius: 0.2,
                    center: snap_point,
                    alpha: 1.0,
                    solid: false,
                };
                sphere.show(&mut *signifiers);
                let line = DebugLine::new(vec![
                    snap_point,
                    snap_point + glam::Vec3::new(proposal.normal.x, 0.0, proposal.normal.y),
                ]);
                line.show(&mut *signifiers);
            }
        }
        3 => {
            // case3 commit: update wall state → StateChange compound edit → History::add(type 7)
            // → OnShapeMoved (0x30 row) + ShowPinnedDecoClutter (0x18 row).
            let entry = if cache.map1.contains_key(&wall) {
                cache.map1.get_mut(&wall)
            } else {
                cache.map3.get_mut(&wall)
            };
            if let Some(entry) = entry {
                let outcome = commit_move_shape(entry, wall, &mut inner, &mut history);
                if outcome.emit_on_shape_moved {
                    on_shape_moved.write(OnShapeMoved(wall));
                }
                if outcome.emit_show_pins {
                    show_pins.write(ShowPinnedDecoClutter::default());
                }
            }
        }
        _ => {
            // Native `0x14097ffb0` = halt_baddata (out of the 4-entry table); unreachable by contract.
        }
    }

    // Public tail: move gizmo / trigger volume (§2). Reads `ActionSetStates.digital[2]`
    // (camera-rotate, phase<3) inside the helper. anchor/rotation/focus come from case1/2
    // (deferred); placeholder values here, return ignored until those land.
    let _ = move_gizmo_and_trigger_vol(
        glam::Vec3::ZERO,
        glam::Quat::IDENTITY,
        0,
        activation,
        activation,
        &*actions,
    );
}

/// `ActionSetStates.digital[2].pressed()` = `phase < 3` — the mouse Rotate binding
/// (`DIGITAL_TARGETS (37,2)`, camera_rotate_mode) read inline by `move_gizmo_and_trigger_vol`
/// (0x14097D382 `[rax+0x10]`; 0x14097D3EF `cmp ebx,3; setae`).
pub fn camera_rotate_pressed(actions: &ActionSetStates) -> bool {
    (actions.digital[2].phase as u8) < 3
}

/// case2 hover-snap core (§3): candidate segment → `SnapProposal` → roof-ridge height offset.
/// Read-only (no `InnerWalls`). `resolve_roof` yields the candidate wall's roof (if any).
fn hover_snap<'a>(
    public: &PublicWalls,
    spatial: &WallSpatialHash,
    cursor_world: glam::Vec3,
    resolve_roof: impl Fn(Entity) -> Option<&'a Roof>,
) -> Option<(SnapProposal, glam::Vec3)> {
    let proposal = nearest_proposal(public, spatial, cursor_world)?;
    let mut point = proposal.point;
    if let Some(entity) = public.roof_entity(proposal.wall) {
        if let Some(roof) = resolve_roof(entity) {
            let tag = public
                .get(proposal.wall)
                .map(|state| state.shape().tag())
                .unwrap_or(0);
            let offset = match tag {
                2 => -6.0f32, // freehand
                1 => -8.0f32, // rectangle
                _ => 0.0,
            };
            // Roof-ridge snap (visual) — result consumed by the gizmo signifier only.
            let _ = snap_to_roof_ridge(roof, proposal.point, 0.1);
            let _ = roof.height();
            point = glam::Vec3::new(point.x, point.y + offset, point.z);
        }
    }
    Some((proposal, point))
}

/// B `move_gizmo_and_trigger_vol` (`0x14097d280`, §2). Constant handle + trigger volume.
/// Implemented: camera-rotate gate (`digital[2].phase<3`) and the smoothstep `base` from the
/// shape height. TODO(evidence): `box_trigger_volume(base*1.5, 0.5, base*1.5, Transform)`,
/// `UiState::add` widget, and the `move_shape` / `move_shape_outline` axis signifier records.
#[allow(clippy::too_many_arguments)]
pub fn move_gizmo_and_trigger_vol(
    anchor: glam::Vec3,
    rotation: glam::Quat,
    focus_count: u32,
    shape_height: f32,
    activation: f32,
    actions: &ActionSetStates,
) -> Vec<[[f32; 3]; 3]> {
    let _ = (anchor, rotation, focus_count, activation);
    // Camera-rotate held → suppress the move gizmo while rotating the camera.
    let camera_rotate = camera_rotate_pressed(actions);
    // base = smoothstep((h + (-0.5)) / 2.5): §2 constants -0.5/2.5/3.0/0.5/1.0.
    let t = ((shape_height - 0.5) / 2.5).clamp(0.0, 1.0);
    let smooth = (3.0 - 2.0 * t) * t * t;
    let base = (1.0 - smooth) * 0.5 + smooth;
    let _ = base;
    if camera_rotate {
        return Vec::new();
    }
    // TODO(evidence): box_trigger_volume(base*1.5, 0.5, base*1.5, Transform); UiState::add;
    // axis signifier records (move_shape / move_shape_outline).
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_default_is_empty_and_lookup_none() {
        let cache = MoveShapeCache::default();
        assert!(cache.is_empty());
        assert!(cache.get(WallId(1)).is_none());
    }

    #[test]
    fn cache_entry_roundtrips() {
        let mut cache = MoveShapeCache::default();
        cache.map1.insert(
            WallId(7),
            MoveShapeEntry {
                state: 2,
                activation: 1.0,
                variant: 3,
                ..MoveShapeEntry::default()
            },
        );
        assert!(!cache.is_empty());
        let entry = cache.get(WallId(7)).unwrap();
        assert_eq!(entry.state, 2);
        assert_eq!(entry.variant, 3);
    }

    #[test]
    fn case3_commit_updates_wall_and_records_undoable_history() {
        use crate::resources::history::History;
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{InnerWalls, WallState};

        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 2.0, 0, 0, false));
        let mut history = History::<()>::after_observed_future_clear();
        history.start_input_recording(0).unwrap();

        // In-transit moved curve (as case1 would have written).
        let moved = Curve2::try_new(vec![[0.0, 0.0], [6.0, 0.0]], false).unwrap();
        let mut entry = MoveShapeEntry {
            state: 2,
            activation: 1.0,
            variant: 3,
            curve: Some(moved),
            ..MoveShapeEntry::default()
        };
        let outcome = commit_move_shape(&mut entry, wall, &mut inner, &mut history);
        assert!(entry.committed, "case3 sets the commit sentinel");
        assert!(outcome.emit_on_shape_moved && outcome.emit_show_pins);
        // History holds one committed-able Wall `ChangeSupports` edit (undoable entry).
        let pending = history.pending().expect("recording pending");
        assert!(!pending.edits.is_empty(), "case3 records a Wall edit");
        let crate::resources::history::EditType::Wall(compound) = &pending.edits[0] else {
            panic!("expected EditType::Wall")
        };
        assert!(
            matches!(
                &compound.groups[0][0],
                crate::resources::history::wall_edit::WallHistoryEdit::ChangeSupports(_)
            ),
            "case3 single edit must be ChangeSupports"
        );
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
        assert!(!camera_rotate_pressed(&mk(DigitalActionPhase::JustReleased)));
        assert!(!camera_rotate_pressed(&mk(DigitalActionPhase::Idle)));
    }

    #[test]
    fn case2_hover_snap_finds_candidate_near_wall() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::public_walls::{PublicWalls, WallPublicationDirty};
        use crate::resources::walls::publication_source::TerrainHeightGrid512;
        use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialSegment;
        use crate::resources::walls::{InnerWalls, WallState};

        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 2.0, 0, 0, false));
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        };
        let mut spatial = WallSpatialHash::default();
        let mut public = PublicWalls::default();
        public.consume_with_paths(
            &mut inner,
            &terrain,
            None,
            &WallPublicationDirty::default(),
            &spatial,
        );
        // Populate the spatial index from the published outline.
        let outline = public.get(wall).unwrap().geometry.outline.clone();
        for seg in outline.windows(2) {
            spatial.add_segment(WallSpatialSegment {
                endpoints: [[seg[0][0], seg[0][2]], [seg[1][0], seg[1][2]]],
                source_u: [0.0, 1.0],
                relative_heights: [0.0, 0.0],
                terrain_heights: [0.0, 0.0],
                wall,
            });
        }
        let cursor = glam::Vec3::new(2.0, 2.0, 0.0);
        let snapped = hover_snap(&public, &spatial, cursor, |_| None);
        assert!(snapped.is_some(), "expected a snap candidate near the wall");
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
        info.u18 = (1.0f32.to_bits() as u64) << 32; // field @+0x1c = 1.0
        info.u24 = 2.0f32.to_bits(); // field @+0x24 = 2.0
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(11), info);
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
        assert_eq!(len, 1, "one decorator triple");
        // +0x20..0x3c is a verbatim copy of the 28B ShapeParameters.
        assert_eq!(&e.shape_rec[0x20..0x3c], &e.shape_rec[0x258..0x274]);
        // Decorator triples: {id, d[-0x34]-cx, d[-0x2c]-cz} = {11, 1.0-2.0, 2.0-3.0}.
        assert_eq!(e.decorator_triples.len(), 1);
        let (id, dx, dz) = e.decorator_triples[0];
        assert_eq!(id, 11);
        assert!((dx - (1.0 - 2.0)).abs() < 1e-5, "dx {dx}");
        assert!((dz - (2.0 - 3.0)).abs() < 1e-5, "dz {dz}");
        // +0x3c = anchor.y = terrain height sampled at center (default grid min).
        let anchor_y = f32::from_le_bytes(e.shape_rec[0x3c..0x40].try_into().unwrap());
        let expected_y = TerrainHeightsData::<TerrainHeightSsbo>::default()
            .sample_at_world_xz(2.0, 3.0);
        assert_eq!(anchor_y, expected_y);
    }
}

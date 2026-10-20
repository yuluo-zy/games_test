//! B `systems/wall/merge_walls.rs`: `merge_walls` (0x140999920).
//!
//! Consumes one `MergeWallsCmd`, removes both source walls from `InnerWalls`,
//! creates the concatenated merged wall, hands its colour to the wall or
//! half-timber colour owner, archives/migrates decorators, and parks the
//! history payload in `PendingWallHistory::merge_walls` for
//! `history_record_merge_walls`.
use bevy_ecs::{prelude::*, system::SystemParam};
use glam::{Vec2, Vec3};
use utils::curve::Curve2 as InputCurve;

use crate::{
    resources::{
        color_id::{
            HalftimberColorIds, SetHalftimberColorIdCmd, SetWallColorIdCmd, WallColorIds,
        },
        history::wall_edit::{WallAuthoritySnapshot, WallHistoryMetadata},
        roofs::geometry::surface::Curve2,
        terrain_height::TerrainHeightsData,
        walls::{
            InnerWalls, WallId, WallState,
            decorator_archivist::LiveDecoratorArchivist,
            decorator_storage::{Decorators, DecoratorStorage},
            publication_fields::WallShapeSnapshot,
            publication_source::RawWallPublicationSource,
        },
    },
    systems::{
        audio::PlayWallUiSound,
        collision::world::RaycastWorld,
        wall::deferred_wall_history::{
            PendingAdjustHeight, PendingMergeWalls, PendingWallHistory,
        },
    },
};

/// The original's second parameter is one derived SystemParam struct holding
/// the raycast world and the decorator collider maps; both are only forwarded to
/// `archive_current_decorators_in_storage` (collider payload rebuild, a
/// registered gap of the local archivist). `DecoratorPhysicsColliders` exists
/// only with `native-physics`.
#[derive(SystemParam)]
pub struct ArchiveCollisionParams<'w> {
    #[allow(dead_code)]
    raycast: Res<'w, RaycastWorld>,
    #[cfg(feature = "native-physics")]
    #[allow(dead_code)]
    colliders: Res<'w, crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>,
}

/// Original `DEV_OVERRIDE_IS_DEMO_MODE` (0x143634d90; `tiny_glade::main` always
/// writes 0). Feeds the merged wall's metadata byte +0x69 (`demo ? 0 : 5`).
const DEV_OVERRIDE_IS_DEMO_MODE: bool = false;

/// `PlayWallUiSound` kind written at the merge junction (instance +0x08 = 4).
const MERGE_SOUND_KIND: u8 = 4;

/// Original `merge_walls::MergeWallsCmd` (EventInstance 0x28 B: id @+0, then
/// wall_a @+0x08, wall_b @+0x10, f32 @+0x18, u32 @+0x1c, u8 @+0x20, u8 @+0x21).
/// Only `wall_a`, `wall_b` and `wall_a_original_color_id` are named by the
/// original assertion text; the other three names are descriptive.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct MergeWallsCmd {
    pub wall_a: WallId,
    pub wall_b: WallId,
    /// Height of wall A before it was matched to wall B (becomes the `from`
    /// of the recorded AdjustHeight and the initial height of `wall_state_a`).
    pub wall_a_original_height: f32,
    pub wall_a_original_color_id: u32,
    /// Event +0x18: reverse wall A's points before concatenation.
    pub reverse_a: bool,
    /// Event +0x19: keep wall B's point order; false reverses it.
    pub keep_b_direction: bool,
}

/// `DecoratorArchivist::archive_current_decorators_in_storage` 0x1413e7230 for
/// a wall address: archive the wall's current bucket, or an empty bucket when
/// the storage has none. (Collider payload rebuild is the archivist's
/// registered collision-domain gap.)
fn archive_wall(archivist: &mut LiveDecoratorArchivist, storage: &DecoratorStorage, wall: WallId) {
    match storage.per_wall.get(&wall) {
        Some(decorators) => archivist.archive_storage_decorators(0, decorators, Some(wall)),
        None => archivist.archive_storage_decorators(0, &Decorators::default(), Some(wall)),
    }
}

/// 0x140b2bc00 `decorator_cull_allow_wall(archivist, wall, storage)`: folds the
/// wall bucket's decorator ids into the archivist's allowed-cull set (+0x30).
/// A missing bucket does nothing.
fn cull_allow_wall(archivist: &mut LiveDecoratorArchivist, storage: &DecoratorStorage, wall: WallId) {
    if let Some(decorators) = storage.per_wall.get(&wall) {
        archivist
            .culling
            .allowed_ids
            .extend(decorators.decorators().keys().copied());
    }
}

/// Points of `state` as input vectors, reversed when `reverse`.
fn oriented_points(state: &WallState, reverse: bool) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = state
        .source()
        .curve
        .points
        .iter()
        .copied()
        .map(Vec2::from_array)
        .collect();
    if reverse {
        points.reverse();
    }
    points
}

/// Merged wall state (0x14099a2b2..0x14099a31c): curve from the joined points,
/// shape tag 2 (no enclosure), everything else from wall B except style, which
/// collapses to `(B.style == 1)`, and metadata +0x69 which is demo-derived.
fn merged_state(curve: Curve2, b: &WallState) -> WallState {
    let mut shape = [0u8; 28];
    shape[..4].copy_from_slice(&2u32.to_le_bytes());
    let shape = WallShapeSnapshot::from_observed(shape);
    let source = RawWallPublicationSource {
        curve,
        curve_kind_tag: shape.tag(),
        terrain_floor: b.elevation(),
        shape,
        style: u8::from(b.style() == 1),
        extra: b.height(),
        special: false,
        observed_path_scalar: b.arch_height(),
        clutter_version: b.source_flag_65(),
        observed_source_flag_64: b.arch_type(),
    };
    let mut metadata = b.snapshot().metadata;
    metadata = WallHistoryMetadata {
        source_60_bits: metadata.source_60_bits,
        source_66: Some(b.freehand_ty()),
        source_67: metadata.source_67,
        source_69: if DEV_OVERRIDE_IS_DEMO_MODE { 0 } else { 5 },
    };
    WallState::from_snapshot(WallAuthoritySnapshot { source, metadata }, b.freehand_ty())
}

/// Original 0x140999920. At most one `MergeWallsCmd` per run (`ev_count < 2`
/// assertion). Parameter surface in original order: terrain heights, the
/// raycast/collider pair (one derived SystemParam struct passed to the
/// archivist), archivist, decorator storage, pending history, inner walls,
/// merge reader, wall-sound writer, wall colour writer, wall colours,
/// half-timber colour writer, half-timber colours.
pub fn merge_walls(
    terrain: Res<TerrainHeightsData>,
    _collision: ArchiveCollisionParams,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    mut storage: ResMut<DecoratorStorage>,
    mut pending_wall_history: ResMut<PendingWallHistory>,
    mut inner_walls: ResMut<InnerWalls>,
    mut merges: EventReader<MergeWallsCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut wall_color_cmds: EventWriter<SetWallColorIdCmd>,
    wall_colors: Res<WallColorIds>,
    mut halftimber_color_cmds: EventWriter<SetHalftimberColorIdCmd>,
    halftimber_colors: Res<HalftimberColorIds>,
) {
    let mut ev_count = 0;
    for ev in merges.read() {
        ev_count += 1;
        assert!(ev_count < 2, "assertion failed: ev_count < 2");
        assert!(ev.wall_a != ev.wall_b, "assertion failed: ev.wall_a != ev.wall_b");

        // Wall A is saved with the pre-merge height; wall B is saved as is.
        let mut wall_state_a = inner_walls.remove(ev.wall_a);
        wall_state_a.snapshot.source.extra = ev.wall_a_original_height;
        let wall_state_b = inner_walls.remove(ev.wall_b);

        // Wall B's source +0x65 byte selects which colour table/owner is used.
        let halftimber = wall_state_b.source_flag_65() != 0;
        let colors = if halftimber {
            &halftimber_colors.0
        } else {
            &wall_colors.0
        };
        let merged_wall_color = colors[ev.wall_b.0 as usize];

        // Equal (non-NaN) heights record nothing; otherwise wall A is retargeted
        // to B's height and the change is recorded as AdjustHeight.
        let b_height = wall_state_b.height();
        let adjust_height = if ev.wall_a_original_height == b_height {
            None
        } else {
            wall_state_a.snapshot.source.extra = b_height;
            Some(PendingAdjustHeight {
                wall: ev.wall_a,
                from: ev.wall_a_original_height,
                to: b_height,
            })
        };

        // Join: B first (reversed unless keep_b_direction), A second (reversed
        // when reverse_a); A's last point is popped and becomes the junction.
        let b_points = oriented_points(&wall_state_b, !ev.keep_b_direction);
        let mut points = oriented_points(&wall_state_a, ev.reverse_a);
        let junction = points.pop().unwrap();
        points.extend(b_points);
        let input = InputCurve::try_new_from_points(points, false).unwrap();
        let curve = Curve2 {
            points: input.points.iter().map(|p| p.to_array()).collect(),
            points_u: input.points_u.clone(),
            length: input.length,
        };
        let merged_id = inner_walls.new_wall_from_curve(merged_state(curve, &wall_state_b));

        if halftimber {
            halftimber_color_cmds.write(SetHalftimberColorIdCmd::new(merged_id, merged_wall_color, 0));
        } else {
            wall_color_cmds.write(SetWallColorIdCmd::new(merged_id, merged_wall_color, 0));
        }

        archive_wall(&mut archivist, &storage, ev.wall_a);
        cull_allow_wall(&mut archivist, &storage, ev.wall_a);
        archive_wall(&mut archivist, &storage, ev.wall_b);
        cull_allow_wall(&mut archivist, &storage, ev.wall_b);
        archive_wall(&mut archivist, &storage, merged_id);
        storage.on_walls_merged(ev.wall_a, ev.wall_b, merged_id);

        assert!(
            pending_wall_history
                .merge_walls
                .replace(PendingMergeWalls {
                    adjust_height,
                    wall_a: ev.wall_a,
                    wall_b: ev.wall_b,
                    wall_state_a,
                    wall_state_b,
                    merged_wall_id: merged_id,
                    merged_color_id: merged_wall_color,
                    wall_color_a: ev.wall_a_original_color_id,
                    wall_color_b: merged_wall_color,
                })
                .is_none(),
            "assertion failed: pending_wall_history.merge_walls.replace(PendingMergeWalls {{ .. }}).is_none()"
        );

        // x0y(junction) lifted by the terrain height (the *0.0 terms are kept).
        let height = terrain.sample_at_world_xz(junction.x, junction.y);
        sounds.write(PlayWallUiSound {
            kind: MERGE_SOUND_KIND,
            selector: 0,
            value: 0.0,
            position: Vec3::new(
                junction.x + height * 0.0,
                0.0 + height,
                junction.y + height * 0.0,
            ),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resources::walls::{
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::DecoratorStorageAddr,
        },
        systems::wall::{
            deferred_wall_history::history_record_merge_walls,
            history_recording::{RecordWallHistory, record_wall_history},
            shapes::to_authority_curve,
        },
    };

    fn state(points: &[(f32, f32)], height: f32, style: u8, flag65: u8) -> WallState {
        let points = points.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
        let curve = InputCurve::try_new_from_points(points, false).unwrap();
        WallState::new(to_authority_curve(&curve), height, style, flag65, false)
    }

    struct Fixture {
        world: World,
        a: WallId,
        b: WallId,
    }

    fn fixture(a: WallState, b: WallState) -> Fixture {
        let mut world = World::new();
        let mut inner = InnerWalls::default();
        let wa = inner.new_wall_from_curve(a);
        let wb = inner.new_wall_from_curve(b);
        world.insert_resource(inner);
        let mut colors = WallColorIds::new();
        colors.0[wa.0 as usize] = 11;
        colors.0[wb.0 as usize] = 22;
        world.insert_resource(colors);
        let mut halftimber = HalftimberColorIds::default();
        halftimber.0[wb.0 as usize] = 33;
        world.insert_resource(halftimber);
        world.init_resource::<TerrainHeightsData>();
        world.init_resource::<RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<PendingWallHistory>();
        world.init_resource::<Events<MergeWallsCmd>>();
        world.init_resource::<Events<PlayWallUiSound>>();
        world.init_resource::<Events<SetWallColorIdCmd>>();
        world.init_resource::<Events<SetHalftimberColorIdCmd>>();
        Fixture { world, a: wa, b: wb }
    }

    fn cmd(f: &Fixture, height: f32, reverse_a: bool, keep_b: bool) -> MergeWallsCmd {
        MergeWallsCmd {
            wall_a: f.a,
            wall_b: f.b,
            wall_a_original_height: height,
            wall_a_original_color_id: 5,
            reverse_a,
            keep_b_direction: keep_b,
        }
    }

    fn run(f: &mut Fixture, cmds: &[MergeWallsCmd]) {
        for c in cmds {
            f.world.resource_mut::<Events<MergeWallsCmd>>().send(*c);
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(merge_walls);
        schedule.run(&mut f.world);
    }

    fn pts(world: &World, id: WallId) -> Vec<[f32; 2]> {
        world.resource::<InnerWalls>().get_unwrapped(id).source().curve.points.clone()
    }

    #[test]
    fn joins_a_then_b_dropping_a_junction_point_and_parks_history() {
        let mut f = fixture(
            state(&[(0., 0.), (1., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(2., 0.), (3., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let c = cmd(&f, 3.9, false, true);
        run(&mut f, &[c]);
        let inner = f.world.resource::<InnerWalls>();
        assert!(!inner.contains(f.a) && !inner.contains(f.b));
        let merged = WallId(3);
        assert_eq!(inner.max_id(), 3);
        assert_eq!(
            pts(&f.world, merged),
            vec![[0., 0.], [1., 0.], [2., 0.], [3., 0.], [4., 0.]]
        );
        let m = inner.get_unwrapped(merged);
        assert_eq!(m.source().shape.tag(), 2);
        assert_eq!(m.snapshot().metadata.source_69, 5);
        let pending = f.world.resource::<PendingWallHistory>().merge_walls.clone().unwrap();
        assert!(pending.adjust_height.is_none());
        assert_eq!((pending.wall_a, pending.wall_b, pending.merged_wall_id), (f.a, f.b, merged));
        assert_eq!(
            (pending.merged_color_id, pending.wall_color_a, pending.wall_color_b),
            (22, 5, 22)
        );
        assert_eq!(pending.wall_state_a.height(), 3.9);
        // Wall colour owner receives the merged id with wall B's colour.
        let cmds: Vec<_> = f
            .world
            .resource::<Events<SetWallColorIdCmd>>()
            .iter_current_update_events()
            .map(|c| (c.wall_id(), c.color, c.custom))
            .collect();
        assert_eq!(cmds, vec![(merged, 22, 0)]);
        assert!(f.world.resource::<Events<SetHalftimberColorIdCmd>>().is_empty());
        // Sound at the popped junction (2, 0), kind 4, lifted by the terrain height.
        let sound = f.world.resource::<Events<PlayWallUiSound>>().iter_current_update_events().next().copied().unwrap();
        assert_eq!(sound.kind, 4);
        let h = f.world.resource::<TerrainHeightsData>().sample_at_world_xz(2.0, 0.0);
        assert_eq!(sound.position, Vec3::new(2.0 + h * 0.0, 0.0 + h, 0.0 + h * 0.0));
    }

    #[test]
    fn reverse_flags_orient_both_walls() {
        // reverse_a: A -> [(2,0),(1,0),(0,0)], junction (0,0) dropped; B reversed
        // (keep_b_direction=false): [(-2,0),(-1,0),(0,0)].
        let mut f = fixture(
            state(&[(0., 0.), (1., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(0., 0.), (-1., 0.), (-2., 0.)], 3.9, 0, 0),
        );
        let c = cmd(&f, 3.9, true, false);
        run(&mut f, &[c]);
        assert_eq!(
            pts(&f.world, WallId(3)),
            vec![[2., 0.], [1., 0.], [-2., 0.], [-1., 0.], [0., 0.]]
        );
        let sound = f.world.resource::<Events<PlayWallUiSound>>().iter_current_update_events().next().copied().unwrap();
        assert_eq!((sound.position.x, sound.position.z), (0.0, 0.0));
    }

    #[test]
    fn height_mismatch_retargets_wall_a_and_records_adjust_height() {
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 1.0, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let c = cmd(&f, 1.0, false, true);
        run(&mut f, &[c]);
        let pending = f.world.resource::<PendingWallHistory>().merge_walls.clone().unwrap();
        assert_eq!(
            pending.adjust_height,
            Some(PendingAdjustHeight { wall: f.a, from: 1.0, to: 3.9 })
        );
        assert_eq!(pending.wall_state_a.height(), 3.9);
        assert_eq!(f.world.resource::<InnerWalls>().get_unwrapped(WallId(3)).height(), 3.9);
    }

    #[test]
    fn nan_original_height_counts_as_mismatch() {
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 1.0, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let c = cmd(&f, f32::NAN, false, true);
        run(&mut f, &[c]);
        let pending = f.world.resource::<PendingWallHistory>().merge_walls.clone().unwrap();
        assert!(pending.adjust_height.unwrap().from.is_nan());
        assert_eq!(pending.wall_state_a.height(), 3.9);
    }

    #[test]
    fn wall_b_flag_selects_halftimber_colour_owner_and_style_collapses() {
        // flag65 != 0 -> HalftimberColorIds / SetHalftimberColorIdCmd; style 2 -> 0.
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 2, 1),
        );
        let c = cmd(&f, 3.9, false, true);
        run(&mut f, &[c]);
        let cmds: Vec<_> = f
            .world
            .resource::<Events<SetHalftimberColorIdCmd>>()
            .iter_current_update_events()
            .map(|c| (c.wall_id(), c.color))
            .collect();
        assert_eq!(cmds, vec![(WallId(3), 33)]);
        assert!(f.world.resource::<Events<SetWallColorIdCmd>>().is_empty());
        let m = f.world.resource::<InnerWalls>().get_unwrapped(WallId(3)).clone();
        assert_eq!((m.style(), m.source_flag_65()), (0, 1));
        // Style 1 (skirts) survives.
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 1, 0),
        );
        let c = cmd(&f, 3.9, false, true);
        run(&mut f, &[c]);
        assert_eq!(f.world.resource::<InnerWalls>().get_unwrapped(WallId(3)).style(), 1);
    }

    #[test]
    fn decorators_are_archived_culled_and_migrated() {
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let info = |id: u32| DecoratorInfo { dst_wall: 7, decorator_id: id, ..DecoratorInfo::default() };
        {
            let mut storage = f.world.resource_mut::<DecoratorStorage>();
            storage.apply_decorator_place(DecoratorStorageAddr::Wall(f.a), DecoratorId(1), &info(1));
            storage.apply_decorator_place(DecoratorStorageAddr::Wall(f.b), DecoratorId(2), &info(2));
        }
        let c = cmd(&f, 3.9, false, true);
        run(&mut f, &[c]);
        let archivist = f.world.resource::<LiveDecoratorArchivist>();
        // a, b and the (empty) merge target are archived before the migration.
        assert_eq!(archivist.walls.len(), 3);
        assert!(archivist.is_decorator_cull_allowed(1) && archivist.is_decorator_cull_allowed(2));
        let storage = f.world.resource::<DecoratorStorage>();
        assert!(storage.get(DecoratorStorageAddr::Wall(f.a)).is_none());
        assert_eq!(storage.get(DecoratorStorageAddr::Wall(WallId(3))).unwrap().decorators.len(), 2);
    }

    #[test]
    #[should_panic(expected = "ev_count < 2")]
    fn a_second_command_in_one_run_panics() {
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let c = cmd(&f, 3.9, false, true);
        run(&mut f, &[c, c]);
    }

    #[test]
    #[should_panic(expected = "ev.wall_a != ev.wall_b")]
    fn merging_a_wall_with_itself_panics() {
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let mut c = cmd(&f, 3.9, false, true);
        c.wall_b = c.wall_a;
        run(&mut f, &[c]);
    }

    #[test]
    #[should_panic(expected = "pending_wall_history.merge_walls.replace")]
    fn an_unconsumed_pending_merge_panics_after_replacing_it() {
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 3.9, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let c = cmd(&f, 3.9, false, true);
        run(&mut f, &[c]);
        // Second run: re-create two walls and merge again without draining.
        let (a, b) = {
            let mut inner = f.world.resource_mut::<InnerWalls>();
            (
                inner.new_wall_from_curve(state(&[(0., 0.), (2., 0.)], 3.9, 0, 0)),
                inner.new_wall_from_curve(state(&[(2., 0.), (4., 0.)], 3.9, 0, 0)),
            )
        };
        let c2 = MergeWallsCmd { wall_a: a, wall_b: b, ..c };
        f.world.resource_mut::<WallColorIds>().0[b.0 as usize] = 1;
        f.world.resource_mut::<Events<MergeWallsCmd>>().clear();
        run(&mut f, &[c2]);
    }

    #[test]
    fn history_record_emits_adjust_height_then_delete_delete_create() {
        use crate::resources::history::{EditType, History, wall_edit::WallHistoryEdit};
        let mut f = fixture(
            state(&[(0., 0.), (2., 0.)], 1.0, 0, 0),
            state(&[(2., 0.), (4., 0.)], 3.9, 0, 0),
        );
        let c = cmd(&f, 1.0, false, true);
        run(&mut f, &[c]);
        f.world.init_resource::<Events<RecordWallHistory>>();
        f.world.init_resource::<Events<crate::systems::history::recording::HistoryRecordingFailed>>();
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(12).unwrap();
        f.world.insert_resource(history);
        let mut schedule = Schedule::default();
        schedule.add_systems((history_record_merge_walls, record_wall_history::<()>).chain());
        schedule.run(&mut f.world);
        schedule.run(&mut f.world); // drained: nothing is recorded twice
        assert!(f.world.resource::<PendingWallHistory>().merge_walls.is_none());
        let edits = &f.world.resource::<History<()>>().pending().unwrap().edits;
        assert_eq!(edits.len(), 2);
        let EditType::Wall(adjust) = &edits[0] else { panic!("wall edit") };
        assert!(matches!(
            adjust.groups[0][0],
            WallHistoryEdit::AdjustHeight { from, to, .. } if from == 1.0 && to == 3.9
        ));
        let EditType::Wall(merge) = &edits[1] else { panic!("wall edit") };
        assert_eq!(merge.groups.len(), 3);
        let WallHistoryEdit::Delete { wall, color_id, restored_height, source } = &merge.groups[0][0] else { panic!("delete a") };
        assert_eq!((wall.0, *color_id, *restored_height, source.source.extra), (f.a.0, 5, 3.9, 3.9));
        let WallHistoryEdit::Delete { wall, color_id, .. } = &merge.groups[1][0] else { panic!("delete b") };
        assert_eq!((wall.0, *color_id), (f.b.0, 22));
        let WallHistoryEdit::Create { wall, color_id, restored_height, source, .. } = &merge.groups[2][0] else { panic!("create") };
        assert_eq!((wall.0, *color_id, *restored_height), (3, 22, 3.9));
        assert_eq!(source.source.curve.points.len(), 3);
    }

    #[test]
    fn history_record_without_pending_merge_records_nothing() {
        let mut world = World::new();
        world.init_resource::<PendingWallHistory>();
        world.init_resource::<InnerWalls>();
        world.init_resource::<Events<RecordWallHistory>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(history_record_merge_walls);
        schedule.run(&mut world);
        assert!(world.resource::<Events<RecordWallHistory>>().is_empty());
    }
}

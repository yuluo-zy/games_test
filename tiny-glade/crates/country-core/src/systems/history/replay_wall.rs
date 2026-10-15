//! Original wall replay groups applied to the one live wall authority.
use crate::{
    recovered_building::{
        wall_height_command::AdjustWallHeightRequest,
        wall_history_edit::{
            WallAuthoritySnapshot, WallHistoryEdit, WallReplayAudio, WallRestoreColorRequest,
        },
        wall_history_replay::{WallReplayError, advance_wall_groups_observed},
        wall_state::WallId as RecoveredWallId,
    },
    resources::{
        history::{HistoryReplayError, ReplayBuffer, ReplayCommand},
        time::GameTime,
        walls::{InnerWalls, WallId, WallState},
    },
    systems::{
        history::HistoryReplayFailed,
        wall::{
            adjust_wall_height::AdjustWallHeightCmd,
            wall_state::publish_public_wall_state::WallPublicationTerrain,
        },
    },
};
use bevy_ecs::prelude::*;
use std::collections::HashMap;

/// Actual audio/color owners consume these effects; writing an event is not
/// claimed as having played sound or installed a material.
#[derive(Event, Clone, Debug)]
pub struct WallHistoryReplayAudio(pub WallReplayAudio);
#[derive(Event, Clone, Debug)]
pub struct WallHistoryRestoreColor(pub WallRestoreColorRequest);

pub fn register_wall_replay_events(world: &mut World) {
    world.init_resource::<Events<WallHistoryReplayAudio>>();
    world.init_resource::<Events<WallHistoryRestoreColor>>();
    crate::systems::wall::adjust_wall_height::register_wall_height_events(world);
}

fn command_wall(edit: &WallHistoryEdit) -> RecoveredWallId {
    match edit {
        WallHistoryEdit::Create { wall, .. }
        | WallHistoryEdit::Delete { wall, .. }
        | WallHistoryEdit::StateChange { wall, .. }
        | WallHistoryEdit::EnclosureChange { wall, .. } => *wall,
    }
}

fn restored_state(
    snapshot: &WallAuthoritySnapshot,
    wall: RecoveredWallId,
    known_types: &HashMap<RecoveredWallId, u8>,
) -> Result<WallState, WallReplayError> {
    let ty = snapshot
        .metadata
        .source_66
        .or_else(|| known_types.get(&wall).copied())
        .ok_or(WallReplayError::MissingSource66(wall))?;
    Ok(WallState::from_snapshot(snapshot.clone(), ty))
}

/// Visit only the front bucket. The existing verified group interpreter owns
/// the same-wall barrier; finished adjacent disjoint groups may run together.
/// The temporary projection is discarded each command, and effects are written
/// back to InnerWalls immediately. It is not a second persistent wall map.
pub fn replay_wall(
    time: Res<GameTime>,
    terrain: Res<WallPublicationTerrain>,
    mut replay: ResMut<ReplayBuffer>,
    mut inner: ResMut<InnerWalls>,
    mut heights: EventWriter<AdjustWallHeightCmd>,
    mut audio: EventWriter<WallHistoryReplayAudio>,
    mut colors: EventWriter<WallHistoryRestoreColor>,
    mut failures: EventWriter<HistoryReplayFailed>,
) {
    let dt = time.delta * replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    for command in &mut bucket.commands {
        let ReplayCommand::Wall(groups) = command else {
            continue;
        };
        let mut sources: HashMap<_, _> = groups
            .iter()
            .flatten()
            .filter_map(|command| {
                let id = command_wall(&command.edit);
                inner
                    .get(WallId(id.0))
                    .map(|state| (id, state.snapshot().clone()))
            })
            .collect();
        let mut known_types: HashMap<_, _> = sources
            .iter()
            .map(|(&id, state)| (id, state.metadata.source_66.expect("core wall source +66")))
            .collect();
        // An unsupported portable snapshot must neither partially edit live
        // walls nor silently finish its command. Commit only after validation.
        let mut next_groups = groups.clone();
        let mut steps = Vec::new();
        let result = advance_wall_groups_observed(
            &mut next_groups,
            &mut sources,
            dt,
            &terrain.0,
            |effect, sources| {
                let mut states = HashMap::new();
                let instant = effect.instantaneous.as_ref();
                if let Some(wall) = instant.and_then(|value| value.removed_source_notification) {
                    known_types.remove(&wall);
                }
                if let Some(wall) = instant.and_then(|value| value.new_source_notification) {
                    // A restored row must carry its own observed byte, even
                    // when another row with this ID existed earlier this frame.
                    if sources[&wall].metadata.source_66.is_none() {
                        return Err(WallReplayError::MissingSource66(wall));
                    }
                }
                let changed = effect
                    .updated_source_notifications
                    .iter()
                    .copied()
                    .chain(instant.and_then(|value| value.new_source_notification))
                    .chain(instant.and_then(|value| value.updated_source_notification));
                for wall in changed {
                    if let Some(snapshot) = sources.get(&wall) {
                        let state = restored_state(snapshot, wall, &known_types)?;
                        known_types.insert(wall, state.freehand_ty());
                        states.insert(wall, state);
                    }
                }
                steps.push(states);
                Ok(())
            },
        );
        let outcome = match result {
            Ok(result) => result,
            Err(error) => {
                failures.write(HistoryReplayFailed {
                    command: None,
                    error: HistoryReplayError::Wall(error),
                });
                continue;
            }
        };
        *groups = next_groups;
        for (effect, states) in outcome.effects.into_iter().zip(steps) {
            for wall in effect.updated_source_notifications {
                if let Some(state) = states.get(&wall) {
                    inner.update_wall_state(WallId(wall.0), state.clone());
                }
            }
            if let Some(request) = effect.height_request {
                let mut payload = [0; 40];
                request.write_observed_fields(&mut payload);
                heights.write(AdjustWallHeightCmd(AdjustWallHeightRequest::from_observed(
                    &payload,
                )));
            }
            if let Some(value) = effect.audio {
                audio.write(WallHistoryReplayAudio(value));
            }
            if let Some(instant) = effect.instantaneous {
                if let Some(wall) = instant.removed_source_notification {
                    inner.remove(WallId(wall.0));
                }
                if let Some(wall) = instant.new_source_notification {
                    inner.new_wall_with_id(WallId(wall.0), states[&wall].clone());
                }
                if let Some(wall) = instant.updated_source_notification {
                    if let Some(state) = states.get(&wall) {
                        inner.update_wall_state(WallId(wall.0), state.clone());
                    }
                }
                if let Some(value) = instant.audio {
                    audio.write(WallHistoryReplayAudio(value));
                }
                if let Some(mut value) = instant.color_request {
                    // Both portable fields represented the same native +65.
                    value.observed_special_owner =
                        inner.get_unwrapped(WallId(value.wall.0)).is_special();
                    colors.write(WallHistoryRestoreColor(value));
                }
            }
        }
    }
    bucket
        .commands
        .retain(|command| !matches!(command, ReplayCommand::Wall(groups) if groups.is_empty()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        recovered_building::{
            wall_collection::TerrainHeightGrid512,
            wall_history_edit::{WallCompoundHistoryEdit, record_deleted_wall, record_new_wall},
            wall_history_replay::expand_wall_compound,
        },
        resources::history::ReplayBucket,
        systems::history::{ReplayBucketFinished, pop_empty_replay_bucket},
    };

    fn fixture() -> (World, Schedule, WallState) {
        let mut world = World::new();
        register_wall_replay_events(&mut world);
        world.init_resource::<InnerWalls>();
        world.init_resource::<Events<HistoryReplayFailed>>();
        world.init_resource::<Events<ReplayBucketFinished>>();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.insert_resource(GameTime {
            delta: 0.2,
            ..Default::default()
        });
        world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        }));
        let curve =
            system_roof::surface::Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut state = WallState::new(curve, 2.0, 3, 0, false);
        state.snapshot.metadata.source_66 = Some(1);
        state.snapshot.metadata.source_60_bits = 0x3f123456;
        let mut schedule = Schedule::default();
        schedule.add_systems((replay_wall, pop_empty_replay_bucket).chain());
        (world, schedule, state)
    }

    fn push(
        world: &mut World,
        index: usize,
        edit: &WallCompoundHistoryEdit,
        undo: bool,
        instant: bool,
    ) {
        world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
            entry_index: index,
            undo,
            commands: vec![ReplayCommand::Wall(
                expand_wall_compound(edit, undo, instant).unwrap(),
            )],
        });
    }

    #[test]
    fn replay_restores_real_authority_and_honors_group_and_bucket_boundaries() {
        let (mut world, mut schedule, state) = fixture();
        let id = RecoveredWallId(9);
        let edit = record_new_wall(id, state.snapshot(), 7).unwrap();
        let delete = record_deleted_wall(id, state.snapshot(), 7);
        world.resource_mut::<InnerWalls>().set_max_id(70);
        push(&mut world, 0, &edit, false, false);
        push(&mut world, 1, &delete, false, true);
        schedule.run(&mut world);
        let walls = world.resource::<InnerWalls>();
        assert_eq!(walls.max_id(), 70);
        assert_eq!(walls.get_unwrapped(WallId(9)).freehand_ty(), 1);
        assert_eq!(walls.get_unwrapped(WallId(9)).source().curve.length, 0.2);
        assert_eq!(world.resource::<ReplayBuffer>().len(), 2);
        // The same-wall create/growth boundary requires a second invocation.
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get_unwrapped(WallId(9))
                .source()
                .curve
                .length,
            0.2
        );
        schedule.run(&mut world);
        let length = world
            .resource::<InnerWalls>()
            .get_unwrapped(WallId(9))
            .source()
            .curve
            .length;
        assert!(length > 0.2 && length < 4.0);
        schedule.run(&mut world);
        let restored = world.resource::<InnerWalls>().get_unwrapped(WallId(9));
        assert_eq!(restored.source().curve.length, 4.0);
        assert_eq!(restored.snapshot().metadata, state.snapshot().metadata);
        assert_eq!(world.resource::<ReplayBuffer>().len(), 1);
        // Completion pops only the old outer bucket; the delete runs next tick.
        schedule.run(&mut world);
        assert!(!world.resource::<InnerWalls>().contains(WallId(9)));
        assert!(world.resource::<ReplayBuffer>().is_empty());
        assert!(world.resource::<Events<HistoryReplayFailed>>().is_empty());
    }

    #[test]
    fn disjoint_adjacent_groups_keep_intermediate_create_before_later_delete() {
        let (mut world, mut schedule, state) = fixture();
        let create = |id| WallHistoryEdit::Create {
            wall: RecoveredWallId(id),
            source: state.snapshot().clone(),
            color_id: 0,
            restored_height: 2.0,
            restored_elevation: 0.0,
        };
        let edit = WallCompoundHistoryEdit {
            groups: vec![
                vec![create(9)],
                vec![create(10)],
                vec![WallHistoryEdit::Delete {
                    wall: RecoveredWallId(9),
                    source: state.snapshot().clone(),
                    color_id: 0,
                }],
            ],
        };
        push(&mut world, 0, &edit, false, true);
        schedule.run(&mut world);
        let walls = world.resource::<InnerWalls>();
        assert!(!walls.contains(WallId(9)));
        assert!(walls.contains(WallId(10)));
        assert!(walls.changes().created.contains(&WallId(9)));
        assert!(walls.changes().removed.contains(&WallId(9)));
        assert!(world.resource::<ReplayBuffer>().is_empty());
        assert_eq!(world.resource::<Events<WallHistoryRestoreColor>>().len(), 2);
    }

    #[test]
    fn incomplete_import_cannot_create_or_finish_a_wall_replay() {
        let (mut world, mut schedule, mut state) = fixture();
        state.snapshot.metadata.source_66 = None;
        let edit = record_deleted_wall(RecoveredWallId(9), state.snapshot(), 7);
        // Directly inserted commands still validate, even bypassing expansion's
        // early error that protects the history cursor.
        push(&mut world, 0, &edit, true, true);
        schedule.run(&mut world);
        assert!(!world.resource::<InnerWalls>().contains(WallId(9)));
        assert_eq!(world.resource::<ReplayBuffer>().len(), 1);
        assert_eq!(world.resource::<Events<HistoryReplayFailed>>().len(), 1);
    }
}

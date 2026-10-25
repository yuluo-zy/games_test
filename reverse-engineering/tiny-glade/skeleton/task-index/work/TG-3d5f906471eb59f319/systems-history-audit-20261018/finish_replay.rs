//! B finish_replay 0x1408b36a0: preserve active replay until both queues are
//! empty and undo/redo are released, or an explicit byte1==3 stop arrives.
//! The independent HistoryCmd reader and sheep unit event match the PDB.
use crate::{
    meta_commander::MetaCommander,
    input::action_set::MainActionSet,
    resources::{history::ReplayBuffer, toolbar::{Toolbar, ToolbarCategory}},
    systems::{
        history::PendingHistoryCommands,
        mode_manager::{AppMode, HistoryCommand, RestoreAppTo},
    },
};
use bevy_ecs::prelude::*;

/// MetaCommander 队首的回放自然结束哨兵 tag（finish_replay 0x1408b36a0
/// 检查 `!= -0x7fffffffffffffff`，即 `== 0x8000000000000001`）。
pub const REPLAY_FINISHED_SENTINEL: u64 = 0x8000_0000_0000_0001;

/// Original payload-free sheep command. The critter owner consumes this event.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct CmdUnfreezeSheep;

/// Queue-level natural completion. finish_replay also checks live undo/redo
/// state and permits the original explicit stop command to override this gate.
pub fn is_replay_naturally_finished(
    mode: AppMode,
    replay: &ReplayBuffer,
    pending: &PendingHistoryCommands,
) -> bool {
    mode.native_tag() == 13 && replay.is_empty() && pending.0.is_empty()
}

/// Original last-stage replay completion, with its own HistoryCommand cursor.
pub fn finish_replay(
    mut mode: ResMut<AppMode>,
    restore: Res<RestoreAppTo>,
    mut toolbar: ResMut<Toolbar>,
    mut commander: ResMut<MetaCommander>,
    replay: Res<ReplayBuffer>,
    pending: Res<PendingHistoryCommands>,
    actions: Res<MainActionSet>,
    mut commands: EventReader<HistoryCommand>,
    mut unfreeze_sheep: EventWriter<CmdUnfreezeSheep>,
) {
    let was_replay = mode.native_tag() == 13;
    if was_replay {
        if pending.0.len() == 1 && pending.0.front().is_some_and(|command| command.bytes[0] == 1) {
            unfreeze_sheep.write(CmdUnfreezeSheep);
        }
        // `any` intentionally stops the reader after the first explicit stop.
        let interrupted = commands.read().any(|command| command.bytes[1] == 3);
        let released = actions.digital[16].released() && actions.digital[17].released();
        if !interrupted && !(is_replay_naturally_finished(*mode, &replay, &pending) && released) {
            return;
        }
        let restore_to = restore.0;
        *mode = restore_to;
        // Original tools select their full mode; editing/CloneClutter select
        // or cycle the Editing category while preserving restored AppMode.
        let tag = restore_to.native_tag();
        if (6..=19).contains(&tag) {
            toolbar.switch_mode(&restore_to);
        } else {
            // B +0xf8 advances only when category7 was already selected.
            toolbar.advance_category(ToolbarCategory::Editing);
        }
    }
    if commander
        .peek()
        .is_some_and(|command| command.tag == REPLAY_FINISHED_SENTINEL)
    {
        if was_replay {
            log::info!("History replay has naturally finished. Unblocking game commander.");
        } else {
            log::info!("History replay was not active. Unblocking game commander.");
        }
        commander.consume();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        input::action_set::DigitalActionPhase,
        meta_commander::MetaCommand,
        resources::history::{ReplayBucket, ReplayBuffer},
        systems::mode_manager::EditSubject,
    };

    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(AppMode::Replay);
        world.insert_resource(RestoreAppTo(AppMode::Editing(EditSubject::None)));
        world.init_resource::<Toolbar>();
        world.init_resource::<MetaCommander>();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.init_resource::<PendingHistoryCommands>();
        world.init_resource::<MainActionSet>();
        world.init_resource::<Events<HistoryCommand>>();
        world.init_resource::<Events<CmdUnfreezeSheep>>();
        world.resource_mut::<MetaCommander>().extend_script([MetaCommand {
            tag: REPLAY_FINISHED_SENTINEL, payloads: [0; 6],
        }]);
        let mut schedule = Schedule::default();
        schedule.add_systems(finish_replay);
        (world, schedule)
    }

    #[test]
    fn active_bucket_and_held_undo_keep_replay_and_commander_blocked() {
        let (mut world, mut schedule) = fixture();
        world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
            entry_index: 0, undo: false, commands: vec![], timing: Vec::new(),
        });
        schedule.run(&mut world);
        assert_eq!(*world.resource::<AppMode>(), AppMode::Replay);
        assert!(!world.resource::<MetaCommander>().is_empty());
        world.resource_mut::<ReplayBuffer>().pop_front_bucket();
        world.resource_mut::<MainActionSet>().digital[16].phase = DigitalActionPhase::Held;
        schedule.run(&mut world);
        assert_eq!(*world.resource::<AppMode>(), AppMode::Replay);
        world.resource_mut::<MainActionSet>().digital[16].phase = DigitalActionPhase::JustReleased;
        schedule.run(&mut world);
        assert_eq!(*world.resource::<AppMode>(), AppMode::Editing(EditSubject::None));
        assert!(world.resource::<MetaCommander>().is_empty());
    }

    #[test]
    fn explicit_stop_overrides_pending_work_without_consuming_the_queue() {
        let (mut world, mut schedule) = fixture();
        world.resource_mut::<PendingHistoryCommands>().0.push_back(HistoryCommand { bytes: [1, 0, 0, 0] });
        world.resource_mut::<Events<HistoryCommand>>().write(HistoryCommand { bytes: [0, 3, 0, 3] });
        schedule.run(&mut world);
        assert_ne!(*world.resource::<AppMode>(), AppMode::Replay);
        assert_eq!(world.resource::<PendingHistoryCommands>().0.len(), 1);
        assert_eq!(world.resource::<Events<CmdUnfreezeSheep>>().len(), 1);
        assert!(world.resource::<MetaCommander>().is_empty());
    }
}

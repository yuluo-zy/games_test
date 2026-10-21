//! B timelapse replay: original start/interrupt/finish/cleanup lifecycle.
//! The queue retains session fences and the history-completion sentinel until
//! their real consumers release them. Registration belongs to the gameplay app.
use crate::{
    meta_commander::{MetaCommand, MetaCommander},
    resources::{history::History, session::ActiveSession},
    systems::{
        mode_manager::{AppMode, RestoreAppTo},
        ui::resources::widget_params::PlayUiSound,
    },
};
use bevy_ecs::prelude::*;

/// Native byte tags. Names describe observed transitions; original variant
/// spellings are unavailable. B run_game@1405a146f explicitly supplies tag2.
#[repr(u8)]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimelapseState {
    Running = 0,
    Restoring = 1,
    #[default]
    Inactive = 2,
}
impl TimelapseState {
    /// B14092e6b0: cmp byte[rcx],2; setne al.
    pub fn is_active(self) -> bool {
        self != Self::Inactive
    }
}

/// Original payload is one f32; EventInstance stride16 includes the EventId.
/// B140af9eef multiplies the last unread value by the original constant5.0.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct StartTimelapseReplayCmd(pub f32);

/// Original history_process arg8 writes EventId only, stride8.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InterruptTimelapse;

/// B140a8a5ab copies the current state byte to the finished event. Running
/// means natural completion; Restoring means an interrupt restored the session.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimelapseReplayFinished(pub TimelapseState);

/// This is the canonical original two-word Option<f32> resource, separate
/// from action input and hold acceleration. No duplicate live override owner.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ReplaySpeedMultiplierOverride(pub Option<f32>);

/// B140a8a310 + specialized check_if_can_switch@140af9e00. It always drains
/// the unread start batch and acts on its last payload, including failure.
#[allow(clippy::too_many_arguments)]
pub fn start_timelapse(
    mut requested: EventReader<StartTimelapseReplayCmd>,
    history: Res<History>,
    mut commander: ResMut<MetaCommander>,
    active: Res<ActiveSession>,
    mode: Res<AppMode>,
    mut restore: ResMut<RestoreAppTo>,
    mut sounds: EventWriter<PlayUiSound>,
    mut speed: ResMut<ReplaySpeedMultiplierOverride>,
    mut state: ResMut<TimelapseState>,
) {
    let Some(request) = requested.read().last().copied() else {
        return;
    };
    if history.is_recording() {
        sounds.write(PlayUiSound::variant4());
        return;
    }
    restore.0 = *mode;
    // Original specialized extend<[MetaCommand;2]>140b59be0 appends both in
    // this order. The fence/load/replay owners retain their normal lifecycle.
    commander.extend_script([
        MetaCommand::load_session(active.id, true),
        MetaCommand::scalar(1, 0),
    ]);
    speed.0 = Some(request.0 * 5.0);
    *state = TimelapseState::Running;
}

/// B140a8a460. Register after replay Pop and finish_replay in numeric set41,
/// with the original resource_exists::<ActiveSession> condition. Do not use
/// queue emptiness as an input substitute for TimelapseState::is_active.
pub fn finish_timelapse(
    mut interrupts: EventReader<InterruptTimelapse>,
    mut finished: EventWriter<TimelapseReplayFinished>,
    mut speed: ResMut<ReplaySpeedMultiplierOverride>,
    mut state: ResMut<TimelapseState>,
    mut commander: ResMut<MetaCommander>,
    active: Res<ActiveSession>,
) {
    // Original read().count/clear consumes ALL interrupts even when inactive.
    let interrupted = interrupts.read().count() != 0;
    // B140a8a4ba..4c2 marks this ResMut changed before testing its byte, even when
    // inactive. Preserve that publication to other ECS consumers.
    state.set_changed();
    if !state.is_active() {
        return;
    }
    if interrupted && *state == TimelapseState::Running {
        commander.extend_script([MetaCommand::load_session(active.id, false)]);
        *state = TimelapseState::Restoring;
    }
    if commander.is_empty() {
        speed.0 = None;
        finished.write(TimelapseReplayFinished(*state));
    }
}

/// B140a8a5d0, independent PostUpdate reader. It does not restore AppMode:
/// the original load/replay owners and RestoreAppTo perform that transition.
pub fn cleanup_timelapse(
    mut state: ResMut<TimelapseState>,
    mut finished: EventReader<TimelapseReplayFinished>,
) {
    if finished.read().last().is_some() {
        log::info!("The timelapse has finished");
        *state = TimelapseState::Inactive;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::session::{SessionId, SessionMeta};

    fn fixture() -> (World, Schedule, Schedule) {
        let mut world = World::new();
        world.insert_resource(History::<crate::resources::history::MaybePendingSnapshotId>::empty_glade());
        world.insert_resource(ActiveSession::new(
            SessionId(uuid::Uuid::from_bytes([37; 16])),
            SessionMeta::default(),
        ));
        world.init_resource::<AppMode>();
        world.init_resource::<RestoreAppTo>();
        world.init_resource::<MetaCommander>();
        world.insert_resource(TimelapseState::Inactive);
        world.init_resource::<ReplaySpeedMultiplierOverride>();
        world.init_resource::<Events<StartTimelapseReplayCmd>>();
        world.init_resource::<Events<InterruptTimelapse>>();
        world.init_resource::<Events<TimelapseReplayFinished>>();
        world.init_resource::<Events<PlayUiSound>>();
        let mut schedule = Schedule::default();
        schedule.add_systems((start_timelapse, finish_timelapse).chain());
        let mut post = Schedule::default();
        post.add_systems(cleanup_timelapse);
        (world, schedule, post)
    }

    #[test]
    fn original_batch_order_override_and_finish_payload_lifecycle() {
        let (mut world, mut schedule, mut post) = fixture();
        world
            .resource_mut::<Events<StartTimelapseReplayCmd>>()
            .send_batch([StartTimelapseReplayCmd(2.0), StartTimelapseReplayCmd(3.5)]);
        schedule.run(&mut world);
        assert_eq!(*world.resource::<TimelapseState>(), TimelapseState::Running);
        assert_eq!(
            world.resource::<ReplaySpeedMultiplierOverride>().0,
            Some(17.5)
        );
        let id = world.resource::<ActiveSession>().id;
        let load = world.resource_mut::<MetaCommander>().consume().unwrap();
        assert_eq!(load, MetaCommand::load_session(id, true));
        assert_eq!(
            world.resource_mut::<MetaCommander>().consume(),
            Some(MetaCommand::scalar(1, 0))
        );
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplaySpeedMultiplierOverride>().0, None);
        let mut cursor = world
            .resource::<Events<TimelapseReplayFinished>>()
            .get_cursor();
        assert_eq!(
            cursor
                .read(world.resource::<Events<TimelapseReplayFinished>>())
                .copied()
                .collect::<Vec<_>>(),
            vec![TimelapseReplayFinished(TimelapseState::Running)]
        );
        post.run(&mut world);
        assert!(!world.resource::<TimelapseState>().is_active());
    }

    #[test]
    fn recording_rejects_the_drained_batch_without_replacing_restore_or_queue() {
        let (mut world, mut schedule, _) = fixture();
        world
            .resource_mut::<History>()
            .start_input_recording(0)
            .unwrap();
        world.resource_mut::<RestoreAppTo>().0 = AppMode::Path;
        world
            .resource_mut::<Events<StartTimelapseReplayCmd>>()
            .send_batch([StartTimelapseReplayCmd(1.0), StartTimelapseReplayCmd(2.0)]);
        schedule.run(&mut world);
        assert_eq!(
            *world.resource::<TimelapseState>(),
            TimelapseState::Inactive
        );
        assert_eq!(world.resource::<RestoreAppTo>().0, AppMode::Path);
        assert!(world.resource::<MetaCommander>().is_empty());
        assert_eq!(world.resource::<ReplaySpeedMultiplierOverride>().0, None);
        assert_eq!(world.resource::<Events<PlayUiSound>>().len(), 1);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<PlayUiSound>>().len(), 1);
    }

    #[test]
    fn interrupt_appends_restore_once_and_waits_for_owned_fence_completion() {
        let (mut world, mut schedule, mut post) = fixture();
        world
            .resource_mut::<Events<StartTimelapseReplayCmd>>()
            .send(StartTimelapseReplayCmd(1.0));
        schedule.run(&mut world);
        world
            .resource_mut::<Events<InterruptTimelapse>>()
            .send(InterruptTimelapse);
        schedule.run(&mut world);
        assert_eq!(
            *world.resource::<TimelapseState>(),
            TimelapseState::Restoring
        );
        let id = world.resource::<ActiveSession>().id;
        let mut q = world.resource_mut::<MetaCommander>();
        assert_eq!(q.consume(), Some(MetaCommand::load_session(id, true)));
        assert_eq!(q.consume(), Some(MetaCommand::scalar(1, 0)));
        let restore = q.consume().unwrap();
        assert_eq!(restore, MetaCommand::load_session(id, false));
        let next = q.begin_session_load(restore);
        let fence = match &next.on_load[0] {
            crate::resources::session::OnLoadEntry::Fence(fence) => fence.clone(),
            _ => panic!("actual on-load fence owner missing"),
        };
        drop(q);
        world
            .resource_mut::<Events<InterruptTimelapse>>()
            .send_batch([InterruptTimelapse, InterruptTimelapse]);
        schedule.run(&mut world);
        assert!(
            world
                .resource::<Events<TimelapseReplayFinished>>()
                .is_empty()
        );
        assert_eq!(
            world.resource::<MetaCommander>().peek().unwrap().payloads[2],
            fence.as_payload()
        );
        fence.unblock();
        // Actual commander normally observes the now-unblocked fence in set1.
        let outcome = world.resource_mut::<MetaCommander>().update().unwrap();
        assert!(
            matches!(outcome, Some(crate::meta_commander::CommanderOutcome::SessionLoadWait { fence, .. }) if !fence.is_blocked())
        );
        schedule.run(&mut world);
        let mut cursor = world
            .resource::<Events<TimelapseReplayFinished>>()
            .get_cursor();
        assert_eq!(
            cursor
                .read(world.resource::<Events<TimelapseReplayFinished>>())
                .copied()
                .collect::<Vec<_>>(),
            vec![TimelapseReplayFinished(TimelapseState::Restoring)]
        );
        post.run(&mut world);
        assert_eq!(
            *world.resource::<TimelapseState>(),
            TimelapseState::Inactive
        );
    }
}

//! Original B command queue and the session/save command path.
//!
//! Evidence is bound in this claim's META-COMMANDER-CONTRACT.md. The public
//! seven-word carrier remains compatible with the current recovered on-load
//! queue. Rust owns session fences separately; integer identities are never
//! converted back into pointers. Unsupported owned-payload commands remain
//! queued and return an error instead of being silently treated as completed.
use crate::{
    app_loading_state::AppLoadingState,
    render::render_temporal_state::RenderTemporalState,
    resources::{
        game_clock::GameClock,
        rt::{RtBvhBuilderGlueResource, RtWorld},
        session::{NextSession, SessionId},
        shutdown::ShutdownGame,
    },
    systems::{mode_manager::AppMode, save::save_game::SaveGameCmd},
};
use ahash::RandomState;
use bevy_ecs::prelude::{Commands, Res, ResMut};
use hashbrown::HashMap;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MetaCommand {
    pub tag: u64,
    pub payloads: [u64; 6],
}
impl MetaCommand {
    pub const EMPTY_SENTINEL: u64 = 0x8000_0000_0000_0014;
    pub const SESSION_LOAD_MARKER: u64 = 0x8000_0000_0000_0000;
    pub const fn scalar(kind: u8, value: u64) -> Self {
        Self {
            tag: Self::SESSION_LOAD_MARKER | kind as u64,
            payloads: [value, 0, 0, 0, 0, 0],
        }
    }
    /// Original serde LoadSession{id,ignore_snapshot}: UUID bytes at +8/+16,
    /// owned fence at +24, ignore_snapshot at +32. No active/latest fallback.
    pub fn load_session(id: SessionId, ignore_snapshot: bool) -> Self {
        let bytes = id.0.into_bytes();
        Self {
            tag: Self::SESSION_LOAD_MARKER,
            payloads: [
                u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                u64::from_le_bytes(bytes[8..].try_into().unwrap()),
                0,
                u64::from(ignore_snapshot),
                0,
                0,
            ],
        }
    }
    pub fn session_id(self) -> SessionId {
        let mut bytes = [0; 16];
        bytes[..8].copy_from_slice(&self.payloads[0].to_le_bytes());
        bytes[8..].copy_from_slice(&self.payloads[1].to_le_bytes());
        SessionId(uuid::Uuid::from_bytes(bytes))
    }
    pub const fn decoded(self) -> u64 {
        self.tag ^ Self::SESSION_LOAD_MARKER
    }
}

#[derive(Clone, Debug)]
pub struct MetaCommandFence(pub Arc<Mutex<(bool, bool)>>);
impl MetaCommandFence {
    pub fn new_pair_blocked() -> Self {
        Self(Arc::new(Mutex::new((true, true))))
    }
    pub fn unblock(&self) {
        self.0.lock().expect("fence poisoned").0 = false;
    }
    pub fn is_blocked(&self) -> bool {
        self.0.lock().expect("fence poisoned").0
    }
    /// Identity only. Consumers must retain this owned fence, never dereference it.
    pub fn as_payload(&self) -> u64 {
        Arc::as_ptr(&self.0) as usize as u64
    }
}
impl PartialEq for MetaCommandFence {
    fn eq(&self, rhs: &Self) -> bool {
        Arc::ptr_eq(&self.0, &rhs.0)
    }
}
impl Eq for MetaCommandFence {}

#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct MetaCommander {
    commands: VecDeque<MetaCommand>,
    // Profiling commands are not accepted until their real sink consumer is
    // connected. This is retained for the existing is_profiling API only.
    profiler_sinks: HashMap<String, (), RandomState>,
    session_fences: HashMap<u64, MetaCommandFence, RandomState>,
}
impl Default for MetaCommander {
    fn default() -> Self {
        Self::new()
    }
}
impl MetaCommander {
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            profiler_sinks: HashMap::with_hasher(RandomState::new()),
            session_fences: HashMap::with_hasher(RandomState::new()),
        }
    }
    pub fn peek(&self) -> Option<MetaCommand> {
        self.commands.front().copied()
    }
    pub fn consume(&mut self) -> Option<MetaCommand> {
        self.commands.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    pub fn undo_consume(&mut self, command: MetaCommand) {
        self.commands.push_front(command);
    }
    pub fn extend_script(&mut self, commands: impl IntoIterator<Item = MetaCommand>) {
        self.commands.extend(commands);
    }
    pub fn is_profiling(&self) -> bool {
        !self.profiler_sinks.is_empty()
    }

    /// One dispatch step. The ECS consumer loops until the queue is empty or
    /// an original wait branch returns. The previous one-command/frame behavior
    /// incorrectly delayed ResetTime/Freeze/SaveAndQuit behind unrelated work.
    pub fn update(&mut self) -> anyhow::Result<Option<CommanderOutcome>> {
        let Some(command) = self.peek() else {
            return Ok(None);
        };
        let kind = command.decoded();
        if command.tag == MetaCommand::EMPTY_SENTINEL {
            self.consume();
            return Ok(None);
        }
        // These cases require owned paths/recordings/profiler sinks, screenshot
        // rendering or critter controls. Retain the command and report the exact
        // unsupported operation; do not synthesize a PathBuf or swallow it.
        if matches!(kind, 8 | 9 | 12 | 13 | 14 | 15 | 16) || kind >= 20 {
            anyhow::bail!(
                "MetaCommand {} ({}) has no connected canonical consumer",
                kind,
                command_name(kind)
            );
        }
        if kind == 0
            && command.payloads[2] != 0
            && !self.session_fences.contains_key(&command.payloads[2])
        {
            anyhow::bail!("LoadSession fence has no owned queue entry");
        }
        let command = self.consume().unwrap();
        let outcome = match kind {
            0 if command.payloads[2] == 0 => CommanderOutcome::SessionLoad { original: command },
            0 => CommanderOutcome::SessionLoadWait {
                original: command,
                fence: self.session_fences[&command.payloads[2]].clone(),
            },
            1 => CommanderOutcome::WaitHistoryReplay { original: command },
            2 => CommanderOutcome::WaitAsync {
                original: command,
                frames: None,
            },
            3 => CommanderOutcome::WaitAsync {
                original: command,
                frames: Some(command.payloads[0]),
            },
            4 => CommanderOutcome::ResetTime,
            5 => CommanderOutcome::WaitFrames {
                original: command,
                remaining: command.payloads[0],
            },
            // Original from_secs_f32 failure panics after consuming the command.
            6 => CommanderOutcome::FixTimestep {
                duration: Duration::from_secs_f32(f32::from_bits(command.payloads[0] as u32)),
            },
            7 => CommanderOutcome::DisableInput,
            10 => CommanderOutcome::Freeze { paused: true },
            11 => CommanderOutcome::Freeze { paused: false },
            17 => CommanderOutcome::SaveAndQuit,
            18 => CommanderOutcome::QuitWithoutSaving,
            19 => CommanderOutcome::QuitProcess {
                status: command.payloads[0] as i32,
            },
            _ => unreachable!(),
        };
        Ok(Some(outcome))
    }

    /// Begin the original two-pass load fence. Caller has already checked AppMode.
    pub fn begin_session_load(&mut self, mut original: MetaCommand) -> NextSession {
        let fence = MetaCommandFence::new_pair_blocked();
        original.payloads[2] = fence.as_payload();
        let mut next = NextSession::load(original.session_id())
            .ignore_snapshot(original.payloads[3] as u8 != 0);
        next.append_fence_release(fence.clone());
        self.session_fences.insert(fence.as_payload(), fence);
        self.undo_consume(original);
        next
    }

    /// Legacy source callers must supply the original carrier. Never replace its
    /// UUID/ignore_snapshot with zeros when installing or retrying the fence.
    pub fn reenqueue_session_load(&mut self, original: MetaCommand, fence: &MetaCommandFence) {
        let mut original = original;
        original.payloads[2] = fence.as_payload();
        self.session_fences
            .insert(fence.as_payload(), fence.clone());
        self.undo_consume(original);
    }
}

#[derive(Clone, Debug)]
pub enum CommanderOutcome {
    SessionLoad {
        original: MetaCommand,
    },
    SessionLoadWait {
        original: MetaCommand,
        fence: MetaCommandFence,
    },
    WaitHistoryReplay {
        original: MetaCommand,
    },
    WaitAsync {
        original: MetaCommand,
        frames: Option<u64>,
    },
    ResetTime,
    WaitFrames {
        original: MetaCommand,
        remaining: u64,
    },
    FixTimestep {
        duration: Duration,
    },
    DisableInput,
    Freeze {
        paused: bool,
    },
    SaveAndQuit,
    QuitWithoutSaving,
    QuitProcess {
        status: i32,
    },
}

fn command_name(kind: u64) -> &'static str {
    const NAMES: [&str; 20] = [
        "LoadSession",
        "WaitHistoryReplay",
        "WaitAsync",
        "WaitAsyncMaxFrames",
        "ResetTime",
        "WaitFrames",
        "FixTimestep",
        "DisableInput",
        "StartInputReplay",
        "Screenshot",
        "Freeze",
        "Unfreeze",
        "StartProfiling",
        "FinishProfiling",
        "ReportGpuUtilization",
        "ReportMemoryUse",
        "FloodWithDeadlyNeurotoxin",
        "SaveAndQuit",
        "QuitWithoutSaving",
        "QuitProcess",
    ];
    NAMES.get(kind as usize).copied().unwrap_or("unknown")
}

/// Canonical gameplay resources, registered at original numeric set1. No
/// resource_exists gates or fake active/latest session are introduced.
pub fn commander_update_system(
    mut commander: ResMut<MetaCommander>,
    app_mode: Res<AppMode>,
    rt_glue: Res<RtBvhBuilderGlueResource>,
    rt_world: Res<RtWorld>,
    mut next_state: ResMut<bevy_state::state::NextState<AppLoadingState>>,
    mut time: ResMut<GameClock>,
    mut temporal: ResMut<RenderTemporalState>,
    mut save: bevy_ecs::event::EventWriter<SaveGameCmd>,
    mut commands: Commands,
) -> anyhow::Result<()> {
    while let Some(outcome) = commander.update()? {
        match outcome {
            CommanderOutcome::SessionLoad { original } => {
                if matches!(app_mode.native_tag(), 13 | 14) {
                    commander.undo_consume(original);
                    return Ok(());
                }
                let request = commander.begin_session_load(original);
                next_state.set(AppLoadingState::Reload);
                commands.insert_resource(request);
                return Ok(());
            }
            CommanderOutcome::SessionLoadWait { original, fence } => {
                if fence.is_blocked() {
                    commander.undo_consume(original);
                    return Ok(());
                }
                commander.session_fences.remove(&original.payloads[2]);
            }
            CommanderOutcome::WaitHistoryReplay { original } => {
                commander.undo_consume(original);
                return Ok(());
            }
            CommanderOutcome::WaitAsync {
                mut original,
                frames,
            } => {
                if rt_world.has_pending_changes() || !rt_glue.is_idle() {
                    if frames != Some(0) {
                        original.tag = MetaCommand::SESSION_LOAD_MARKER | 3;
                        original.payloads[0] = frames.map_or(300, |remaining| remaining - 1);
                        commander.undo_consume(original);
                        return Ok(());
                    }
                    log::warn!("WaitAsyncMaxFrames expired while async work remains");
                }
            }
            CommanderOutcome::ResetTime => {
                *time = GameClock {
                    ran: true,
                    ..GameClock::default()
                };
                temporal.frame_index = 0;
            }
            CommanderOutcome::WaitFrames {
                mut original,
                remaining,
            } => {
                if remaining != 0 {
                    original.payloads[0] = remaining - 1;
                    commander.undo_consume(original);
                    return Ok(());
                }
            }
            CommanderOutcome::FixTimestep { duration } => {
                commands.insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(duration));
            }
            CommanderOutcome::DisableInput => {
                // B14092010e writes niche3, exactly the current canonical
                // DisableInput variant. None4 would resume live translation.
                let mut disabled = crate::systems::backend::input::input_record_replay::InputRecordReplayState::default();
                disabled.mode = crate::input::InputRecordReplayMode::DisableInput;
                commands.insert_resource(disabled);
            }
            CommanderOutcome::Freeze { paused } => time.paused = paused,
            CommanderOutcome::SaveAndQuit => {
                save.write(SaveGameCmd);
                commands.insert_resource(ShutdownGame);
            }
            CommanderOutcome::QuitWithoutSaving => {
                commands.insert_resource(ShutdownGame);
            }
            CommanderOutcome::QuitProcess { status } => std::process::exit(status),
        }
    }
    Ok(())
}

pub fn configure_session_switch_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(commander_update_system);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_session_command_preserves_uuid_ignore_and_fence_across_wait() {
        let id = SessionId(uuid::Uuid::from_bytes([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]));
        let original = MetaCommand::load_session(id, true);
        assert_eq!(original.payloads[0], 0x0807_0605_0403_0201);
        assert_eq!(original.payloads[1], 0x100f_0e0d_0c0b_0a09);
        let mut q = MetaCommander::new();
        let next = q.begin_session_load(original);
        assert_eq!(next.id, id);
        assert!(next.ignore_snapshot);
        let Some(CommanderOutcome::SessionLoadWait {
            original: retry,
            fence,
        }) = q.update().unwrap()
        else {
            panic!()
        };
        assert!(fence.is_blocked());
        assert_eq!(retry.session_id(), id);
        assert_eq!(retry.payloads[3], 1);
        q.undo_consume(retry);
        match &next.on_load[0] {
            crate::resources::session::OnLoadEntry::Fence(f) => f.unblock(),
            _ => panic!(),
        }
        let Some(CommanderOutcome::SessionLoadWait { fence, .. }) = q.update().unwrap() else {
            panic!()
        };
        assert!(!fence.is_blocked());
    }
    #[test]
    fn unsupported_original_screenshot_is_retained_instead_of_creating_empty_directory() {
        let mut q = MetaCommander::new();
        let cmd = MetaCommand::scalar(9, 0);
        q.extend_script([cmd, MetaCommand::scalar(17, 0)]);
        assert!(q.update().unwrap_err().to_string().contains("Screenshot"));
        assert_eq!(q.peek(), Some(cmd));
    }
    #[test]
    fn original_scalar_dispatch_distinguishes_fix_timestep_and_save_quit() {
        let mut q = MetaCommander::new();
        q.extend_script([
            MetaCommand::scalar(6, (0.125f32).to_bits() as u64),
            MetaCommand::scalar(17, 0),
            MetaCommand::scalar(18, 0),
        ]);
        assert!(
            matches!(q.update().unwrap(), Some(CommanderOutcome::FixTimestep { duration }) if duration == Duration::from_millis(125))
        );
        assert!(matches!(
            q.update().unwrap(),
            Some(CommanderOutcome::SaveAndQuit)
        ));
        assert!(matches!(
            q.update().unwrap(),
            Some(CommanderOutcome::QuitWithoutSaving)
        ));
        assert!(q.update().unwrap().is_none());
    }
    #[test]
    fn foreign_session_fence_integer_is_never_dereferenced_or_consumed() {
        let mut q = MetaCommander::new();
        let mut cmd = MetaCommand::load_session(SessionId(uuid::Uuid::nil()), false);
        cmd.payloads[2] = 0xfeed_bad;
        q.extend_script([cmd]);
        assert!(q.update().is_err());
        assert_eq!(q.peek(), Some(cmd));
    }
}

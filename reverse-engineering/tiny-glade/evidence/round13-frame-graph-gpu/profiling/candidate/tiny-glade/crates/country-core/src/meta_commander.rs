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
    path::{Path, PathBuf},
    fs::File,
    io::BufWriter,
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

/// Owned payload projection of the original externally tagged serde variants.
/// Queue tokens are identities in `MetaCommander`, never native pointers.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum OwnedMetaCommand {
    StartProfiling { id: String },
    FinishProfiling { id: String, #[serde(default)] output: Option<PathBuf> },
    Screenshot { file: PathBuf },
}
impl OwnedMetaCommand {
    fn kind(&self) -> u8 {
        match self { Self::StartProfiling { .. } => 12,
            Self::FinishProfiling { .. } => 13, Self::Screenshot { .. } => 9 }
    }
}

#[derive(Clone)]
struct ProfilerSink {
    sink: puffin::FrameSinkId,
    frames: Arc<Mutex<puffin::FrameView>>,
}
impl std::fmt::Debug for ProfilerSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfilerSink").field("sink", &self.sink).finish_non_exhaustive()
    }
}

#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct MetaCommander {
    commands: VecDeque<MetaCommand>,
    // B stores (FrameSinkId, Arc<Mutex<FrameView>>) per original String ID.
    profiler_sinks: HashMap<String, ProfilerSink, RandomState>,
    owned_commands: HashMap<u64, OwnedMetaCommand, RandomState>,
    next_owned_command: u64,
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
            owned_commands: HashMap::with_hasher(RandomState::new()),
            next_owned_command: 1,
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

    /// Append an original owned-payload variant to the existing command deque.
    /// The payload survives front requeue until its actual consumer takes it.
    pub fn enqueue_owned(&mut self, payload: OwnedMetaCommand) {
        let token = self.next_owned_command;
        self.next_owned_command = token.checked_add(1).expect("owned MetaCommand identity exhausted");
        let command = MetaCommand::scalar(payload.kind(), token);
        assert!(self.owned_commands.insert(token, payload).is_none());
        self.commands.push_back(command);
    }
    pub fn extend_owned_script(&mut self, commands: impl IntoIterator<Item = OwnedMetaCommand>) {
        for command in commands { self.enqueue_owned(command); }
    }
    fn owned_front(&self, command: MetaCommand) -> anyhow::Result<&OwnedMetaCommand> {
        self.owned_commands.get(&command.payloads[0])
            .filter(|payload| u64::from(payload.kind()) == command.decoded())
            .ok_or_else(|| anyhow::anyhow!("MetaCommand {} has no owned payload", command_name(command.decoded())))
    }
    /// gather_screenshot_requests consumes this same queue after Update set1.
    pub fn peek_owned_screenshot(&self) -> anyhow::Result<Option<&Path>> {
        let Some(command) = self.peek().filter(|command| command.decoded() == 9) else { return Ok(None); };
        match self.owned_front(command)? {
            OwnedMetaCommand::Screenshot { file } => Ok(Some(file.as_path())),
            _ => unreachable!(),
        }
    }
    /// Native gather clones the path first, then consumes before publishing.
    pub fn consume_owned_screenshot(&mut self) -> anyhow::Result<Option<PathBuf>> {
        if self.peek_owned_screenshot()?.is_none() { return Ok(None); }
        let command = self.consume().expect("peeked screenshot");
        let Some(OwnedMetaCommand::Screenshot { file }) = self.owned_commands.remove(&command.payloads[0]) else { unreachable!() };
        Ok(Some(file))
    }
    fn start_profiling(&mut self, id: String) {
        log::info!("Start profiling {id}");
        puffin::set_scopes_on(true);
        let mut global = puffin::GlobalProfiler::lock();
        let frames = Arc::new(Mutex::new(puffin::FrameView::default()));
        let writer = frames.clone();
        let sink = global.add_sink(Box::new(move |frame| {
            writer.lock().expect("profiling FrameView poisoned").add_frame(frame);
        }));
        // Preserve original insert semantics: replacing an ID drops the old
        // map value; it does not unregister the previous GlobalProfiler sink.
        drop(self.profiler_sinks.insert(id, ProfilerSink { sink, frames }));
    }
    fn finish_profiling(&mut self, id: String, output: Option<PathBuf>) {
        log::info!("Finish profiling {id}");
        let capture = self.profiler_sinks.remove(&id).expect("No such profiling ID");
        {
            let mut global = puffin::GlobalProfiler::lock();
            drop(global.remove_sink(capture.sink).expect("No such frame sink"));
        }
        // B disables global scopes even if another named capture remains.
        puffin::set_scopes_on(false);
        if let Some(output) = output {
            let frames = Arc::try_unwrap(capture.frames)
                .unwrap_or_else(|_| panic!("profiling FrameView still has owners"))
                .into_inner().expect("profiling FrameView poisoned");
            log::info!("writing profiler output to {}", output.display());
            // Original OpenOptions creates/truncates the requested file; no
            // parent directory is synthesized by FinishProfiling.
            let file = File::create(&output).expect("writing profiler output");
            let mut writer = BufWriter::with_capacity(0x2000, file);
            frames.save_to_writer(&mut writer).expect("writing profiler output");
            // Match original BufWriter drop/flush behavior after serialization.
        }
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
        if matches!(kind, 9 | 12 | 13) {
            self.owned_front(command)?; // foreign carriers are retained, never dereferenced
            let command = self.consume().expect("peeked owned command");
            let payload = self.owned_commands.remove(&command.payloads[0]).expect("validated owned payload");
            match payload {
                OwnedMetaCommand::StartProfiling { id } => self.start_profiling(id),
                OwnedMetaCommand::FinishProfiling { id, output } => self.finish_profiling(id, output),
                payload @ OwnedMetaCommand::Screenshot { .. } => {
                    let OwnedMetaCommand::Screenshot { ref file } = payload else { unreachable!() };
                    let parent = file.parent().expect("Screenshot file has no parent");
                    std::fs::create_dir_all(parent).expect("creating screenshot directory");
                    self.owned_commands.insert(command.payloads[0], payload);
                    self.undo_consume(command);
                    // Original Screenshot prepares directories, requeues and
                    // stops here; render gather later consumes the same entry.
                    return Ok(None);
                }
            }
            return Ok(Some(CommanderOutcome::Continue));
        }
        // Remaining owned input/reports/critters retain their explicit boundary.
        if matches!(kind, 8 | 14 | 15 | 16) || kind >= 20 {
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
    /// A synchronous original command already completed; keep draining.
    Continue,
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
) -> bevy_ecs::error::Result {
    while let Some(outcome) = commander.update()? {
        match outcome {
            CommanderOutcome::Continue => {},
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

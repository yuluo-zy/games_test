from pathlib import Path
out=Path('reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/profiling/candidate');p=out/'tiny-glade/crates/country-core/src/meta_commander.rs';s=p.read_text()
s=s.replace('    collections::VecDeque,','    collections::VecDeque,\n    path::{Path, PathBuf},\n    fs::File,\n    io::BufWriter,')
s=s.replace('#[derive(bevy_ecs::resource::Resource, Clone, Debug)]\npub struct MetaCommander {','''/// Owned payload projection of the original externally tagged serde variants.
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
pub struct MetaCommander {''')
s=s.replace('''    // Profiling commands are not accepted until their real sink consumer is
    // connected. This is retained for the existing is_profiling API only.
    profiler_sinks: HashMap<String, (), RandomState>,''','''    // B stores (FrameSinkId, Arc<Mutex<FrameView>>) per original String ID.
    profiler_sinks: HashMap<String, ProfilerSink, RandomState>,
    owned_commands: HashMap<u64, OwnedMetaCommand, RandomState>,
    next_owned_command: u64,''')
s=s.replace('''            profiler_sinks: HashMap::with_hasher(RandomState::new()),''','''            profiler_sinks: HashMap::with_hasher(RandomState::new()),
            owned_commands: HashMap::with_hasher(RandomState::new()),
            next_owned_command: 1,''')
needle='''    pub fn is_profiling(&self) -> bool {
        !self.profiler_sinks.is_empty()
    }
'''
insert='''
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
'''
assert needle in s;s=s.replace(needle,needle+insert)
old='''        // These cases require owned paths/recordings/profiler sinks, screenshot
        // rendering or critter controls. Retain the command and report the exact
        // unsupported operation; do not synthesize a PathBuf or swallow it.
        if matches!(kind, 8 | 9 | 12 | 13 | 14 | 15 | 16) || kind >= 20 {'''
new='''        if matches!(kind, 9 | 12 | 13) {
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
        if matches!(kind, 8 | 14 | 15 | 16) || kind >= 20 {'''
assert old in s;s=s.replace(old,new)
s=s.replace('''pub enum CommanderOutcome {
''','''pub enum CommanderOutcome {
    /// A synchronous original command already completed; keep draining.
    Continue,
''')
s=s.replace('''        match outcome {
''','''        match outcome {
            CommanderOutcome::Continue => {},
''')
p.write_text(s)
p=out/'tiny-glade/crates/country-core/Cargo.toml';s=p.read_text();s=s.replace('log.workspace = true','log.workspace = true\npuffin = { workspace = true, features = ["serialization"] }');p.write_text(s)

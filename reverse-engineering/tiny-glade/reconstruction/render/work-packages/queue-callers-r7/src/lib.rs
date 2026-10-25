//! Private ensure_active_cb caller recovery at 0x140bebc90.
//! Creation and begin remain explicit backend boundaries; this is a CPU contract.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandSnapshot(pub [u8; 88]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordingQueue {
    Primary,
}

pub trait CommandBackend {
    fn create(&mut self, pool_family_index: u32) -> CommandSnapshot;
    fn begin(&mut self, command: &mut CommandSnapshot, queue: RecordingQueue);
}

/// Pool family index (device +0xad4) and recording queue tag are separate inputs.
/// On the observed normal path an existing vector is left untouched.
pub fn ensure_active_cb(
    commands: &mut Vec<CommandSnapshot>,
    pool_family_index: u32,
    backend: &mut impl CommandBackend,
) {
    if commands.is_empty() {
        let mut command = backend.create(pool_family_index);
        backend.begin(&mut command, RecordingQueue::Primary);
        commands.push(command);
    }
}

#[derive(Default)]
pub struct FixtureBackend {
    pub fill: u8,
    pub sequence: u64,
    pub events: Vec<String>,
}

impl CommandBackend for FixtureBackend {
    fn create(&mut self, family: u32) -> CommandSnapshot {
        self.events.push(format!("create:{family}"));
        CommandSnapshot([self.fill; 88])
    }

    fn begin(&mut self, command: &mut CommandSnapshot, queue: RecordingQueue) {
        assert_eq!(queue, RecordingQueue::Primary);
        self.events.push("begin:0".into());
        command.0[..8].copy_from_slice(&1u64.to_le_bytes());
        command.0[8..16].copy_from_slice(&self.sequence.to_le_bytes());
        command.0[16..24].fill(0);
        command.0[86] = 0;
    }
}

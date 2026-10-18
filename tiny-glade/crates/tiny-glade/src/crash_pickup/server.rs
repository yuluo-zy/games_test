//! `tiny_glade::crash_pickup::server` — crash server loop.
//!
//! B `server::run` (`0x1402aee80`) and the four `ServerHandler` callbacks:
//! `create_minidump_file` `0x1402ad880`, `on_minidump_created` `0x1402adae0`,
//! `on_message` `0x1402ae1f0`, plus client connect/disconnect handling.
//!
//! `minidumper::Server` supplies the transport; the event dispatch, minidump
//! naming and message routing below are recovered logic.

use super::{CrashPickupMessage, Error};
use crossbeam_channel::Receiver;
use std::path::PathBuf;

/// Events delivered by the transport backend.
#[derive(Debug)]
pub enum ServerEvent {
    ClientConnected(u64),
    ClientDisconnected(u64),
    Message { client: u64, payload: Vec<u8> },
    MinidumpCreated { client: u64, path: PathBuf },
    Shutdown,
}

/// Crash server state.
#[derive(Default)]
pub struct CrashServer {
    next_minidump: u64,
    minidumps: Vec<PathBuf>,
}

impl CrashServer {
    pub fn new() -> Self {
        Self::default()
    }

    /// B `server::run` (`0x1402aee80`).
    ///
    /// The original is a `minidumper::Server::run` event loop. It dispatches
    /// each backend event to the matching callback and stops on the transport's
    /// shutdown event.
    pub fn run(
        &mut self,
        events: Receiver<ServerEvent>,
        output: &dyn ServerOutput,
    ) -> Result<(), Error> {
        while let Ok(event) = events.recv() {
            match event {
                ServerEvent::ClientConnected(client) => self.on_client_connected(client, output),
                ServerEvent::ClientDisconnected(client) => {
                    self.on_client_disconnected(client, output);
                }
                ServerEvent::Message { client, payload } => {
                    self.on_message(client, &payload, output)?;
                }
                ServerEvent::MinidumpCreated { client, path } => {
                    self.on_minidump_created(client, path, output)?;
                }
                ServerEvent::Shutdown => break,
            }
        }
        Ok(())
    }

    /// `on_client_connected`: the original logs the client id; no state change.
    pub fn on_client_connected(&mut self, client: u64, output: &dyn ServerOutput) {
        output.log(&format!("crash client {client} connected"));
    }

    /// `on_client_disconnected`: logs and drops the client.
    pub fn on_client_disconnected(&mut self, client: u64, output: &dyn ServerOutput) {
        output.log(&format!("crash client {client} disconnected"));
    }

    /// B `create_minidump_file` (`0x1402ad880`).
    ///
    /// Fresh disassembly: create the directory (`DirBuilder::create`), format the
    /// file name with the two-piece literal `["", ".dmp"]`
    /// (`0x1429ad1e0` pieces; `0x1429ad1d8` = `".dmp"`) and a fresh v4 UUID,
    /// join it onto the directory, then `OpenOptions` with write/create/truncate.
    /// Returns the opened file together with its path.
    pub fn create_minidump_file(
        &mut self,
        directory: &std::path::Path,
        output: &dyn ServerOutput,
    ) -> Result<(PathBuf, std::fs::File), Error> {
        std::fs::create_dir_all(directory)?;
        let path = directory.join(format!("{}.dmp", uuid::Uuid::new_v4()));
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;
        output.open_minidump(&path);
        self.next_minidump += 1;
        Ok((path, file))
    }

    /// B `on_minidump_created` (`0x1402adae0`).
    ///
    /// Records the finished minidump and sends its path back to the client.
    pub fn on_minidump_created(
        &mut self,
        client: u64,
        path: PathBuf,
        output: &dyn ServerOutput,
    ) -> Result<(), Error> {
        self.minidumps.push(path.clone());
        output.send_path(client, &path);
        Ok(())
    }

    /// B `on_message` (`0x1402ae1f0`).
    ///
    /// `serde_json::from_slice::<CrashPickupMessage>` followed by dispatch. A
    /// malformed payload reaches `core::result::unwrap_failed` (the decompiled
    /// body jumps to it when the result tag is the `Err` niche; the message is
    /// `"crash pickup message"`, `0x1429acfd0`), i.e. the original panics instead
    /// of skipping the message.
    pub fn on_message(
        &mut self,
        client: u64,
        payload: &[u8],
        output: &dyn ServerOutput,
    ) -> Result<(), Error> {
        let message = serde_json::from_slice::<CrashPickupMessage>(payload).unwrap();
        output.handle_message(client, &message);
        Ok(())
    }

    pub fn minidumps(&self) -> &[PathBuf] {
        &self.minidumps
    }
}

/// Side effects needed by the server loop (minidumper server + logging).
pub trait ServerOutput: Send + Sync {
    fn log(&self, message: &str);
    fn open_minidump(&self, path: &std::path::Path);
    fn send_path(&self, client: u64, path: &std::path::Path);
    fn handle_message(&self, client: u64, message: &CrashPickupMessage);
}

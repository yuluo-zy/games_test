//! `tiny_glade::crash_pickup` — out-of-process crash pickup.
//!
//! Recovered public symbols: `CrashReporter::spawn_server` `0x14015aeb0`,
//! `ClientHandle::send_message` `0x14015ac60`, `client::start` `0x14048a170`,
//! `server::run` `0x1402aee80`.
//!
//! The IPC/minidump transport comes from `minidumper` / `crash_handler` in the
//! original and is external-SDK glue; it is represented by [`IpcClient`] and
//! [`CrashHandler`]. Payload serialisation, retry timing and lifecycle are real
//! recovered logic.

pub mod client;
pub mod crash_payload;
pub mod minidump_to_sentry;
pub mod server;

use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

pub use crash_payload::{CrashPickupMessage, RecoveredSave};

/// Crash-pickup failure.
#[derive(Debug)]
pub enum Error {
    Serialize(serde_json::Error),
    Io(std::io::Error),
    Connect(String),
    Server(String),
    Timeout,
    ClientExited,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialize(error) => write!(formatter, "serialize error: {error}"),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::Connect(message) => write!(formatter, "connect error: {message}"),
            Self::Server(message) => write!(formatter, "server error: {message}"),
            Self::Timeout => formatter.write_str("timed out waiting for crash server"),
            Self::ClientExited => formatter.write_str("crash client exited"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialize(error)
    }
}

/// Transport used by [`ClientHandle::send_message`].
pub trait IpcClient: Send + Sync {
    fn send_message_impl(&self, message: Vec<u8>) -> Result<(), String>;
}

/// Crash-handler attachment (Windows crash handler / minidump writer).
pub trait CrashHandler: Send + Sync {
    fn attach(&self) -> Result<(), String>;
}

/// B `tiny_glade::crash_pickup::ClientHandle`.
#[derive(Clone)]
pub struct ClientHandle {
    client: Arc<dyn IpcClient>,
}

impl ClientHandle {
    pub fn new(client: Arc<dyn IpcClient>) -> Self {
        Self { client }
    }

    /// B `ClientHandle::send_message` (`0x14015ac60`).
    ///
    /// Serialises the message with `serde_json` into `Vec<u8>`, then calls
    /// `minidumper::Client::send_message_impl`; serialisation failure is an
    /// `unwrap`/`expect` in the original (`core::result::unwrap_failed`).
    pub fn send_message(&self, message: CrashPickupMessage) -> Result<(), Error> {
        // 0x14015ac60: `serde_json::to_vec(...).unwrap()` — the failure path reaches
        // `core::result::unwrap_failed` with the generic `Result::unwrap` message
        // (`0x14295b9d0`, len 0x2b), i.e. a plain `unwrap`, not `expect`.
        let bytes = serde_json::to_vec(&message).unwrap();
        self.client
            .send_message_impl(bytes)
            .map_err(Error::Connect)
    }
}

/// B `tiny_glade::crash_pickup::CrashReporter`.
pub struct CrashReporter;

impl CrashReporter {
    /// B `CrashReporter::spawn_server` (`0x14015aeb0`).
    ///
    /// Fresh decompilation / disassembly (`0x14015aeb0`):
    /// 1. `socket = std::env::temp_dir().join(Uuid::new_v4().to_string())`
    ///    (`Uuid` `Display`, zero format pieces; non-UTF-8 → error tag 9 with the
    ///    message `"temp file path for crash reporter"`, `0x14295bc2f`, len 0x21).
    /// 2. `Command::new(current_exe())` with the four literal arguments
    ///    `--crash-reporter-server` (`0x14295bc50`),
    ///    the socket path,
    ///    `--crash-reporter-send-to-sentry` (`0x14295bc67`) and
    ///    `"true"`/`"false"` (`0x14295ba65`/`0x14295ba60`).
    /// 3. `Child::id()` then [`client::start`] with the socket path.
    ///
    /// `send_to_sentry` is the boolean that selects the last argument.
    pub fn spawn_server(send_to_sentry: bool, connect_timeout: std::time::Duration) -> Result<ClientHandle, Error> {
        let socket = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        let socket_string = socket
            .to_str()
            .ok_or_else(|| Error::Connect("temp file path for crash reporter".to_string()))?;

        let executable = std::env::current_exe()?;
        let child = std::process::Command::new(executable)
            .arg("--crash-reporter-server")
            .arg(socket_string)
            .arg("--crash-reporter-send-to-sentry")
            .arg(if send_to_sentry { "true" } else { "false" })
            .spawn()?;
        let _child_id = child.id();

        client::start(&socket, connect_timeout)
    }
}

/// Folder holding report artifacts produced by the server.
pub fn report_folder() -> PathBuf {
    std::env::temp_dir()
}

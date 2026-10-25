//! `tiny_glade::crash_pickup::client` — crash client bootstrap.
//!
//! B `client::start` (`0x14048a170`): retry `UnixStream::connect` in 50 ms steps
//! until the timeout, then spawn the message-queue reader thread and attach the
//! platform crash handler.

use super::{ClientHandle, Error, IpcClient};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Connect step in the original loop (`0x32` ms).
const CONNECT_STEP: Duration = Duration::from_millis(50);

/// Transport factory for one connection attempt.
pub type ConnectFn = dyn Fn(&Path) -> Result<Arc<dyn IpcClient>, String> + Send + Sync;

/// B `tiny_glade::crash_pickup::client::start` (`0x14048a170`).
///
/// Reconstructed retry loop (`0x14048a170`): the connect is attempted once
/// before the loop; on failure the elapsed counter is advanced by 50 ms and
/// *then* compared with the timeout, and only afterwards does the thread sleep:
/// ```text
/// loop {
///     match connect(path) {
///         Ok(client) => break,
///         Err(error) => {
///             waited += 50ms;
///             if timeout <= waited { return Err(error) }
///             sleep(50ms);
///         }
///     }
/// }
/// ```
/// After connecting the original spawns the reader thread
/// (`std::thread::spawn` on the `Client`) and calls `CrashHandler::attach`; a
/// failure to spawn/attach is returned as `Error::Server`.
pub fn start_with(
    connect: Arc<ConnectFn>,
    path: &Path,
    timeout: Duration,
) -> Result<ClientHandle, Error> {
    let mut waited = Duration::ZERO;
    let client = loop {
        match connect(path) {
            Ok(client) => break client,
            Err(message) => {
                waited += CONNECT_STEP;
                if timeout <= waited {
                    return Err(Error::Connect(message));
                }
                std::thread::sleep(CONNECT_STEP);
            }
        }
    };
    Ok(ClientHandle::new(client))
}

/// Default entry point used by [`super::CrashReporter::spawn_server`].
///
/// The original binds `minidumper::ipc::windows::UnixStream`; that SDK type is
/// unavailable without the external crate, so the transport is injected.
pub fn start(path: &Path, timeout: Duration) -> Result<ClientHandle, Error> {
    let connect: Arc<ConnectFn> = Arc::new(|_path| {
        Err("crash-pickup IPC transport not configured".to_string())
    });
    start_with(connect, path, timeout)
}

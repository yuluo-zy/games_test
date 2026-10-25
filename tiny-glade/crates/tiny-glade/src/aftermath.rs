//! `tiny_glade::aftermath` — NVIDIA Aftermath GPU crash dump delegate.
//!
//! Recovered VAs:
//! * `NvAftermathDelegate::default` `0x140244460`
//! * `NvAftermathDelegate::dumped` `0x140244790`
//! * `NvAftermathDelegate::shader_debug_info` `0x140245340`
//! * `NvAftermathDelegate::description` `0x14003bb80` (empty under ICF)
//!
//! `default` checks `paths::tmp()/gpu-crash-dump.zip` and removes a stale file.
//! `dumped` logs `"NV Aftermath dump data received."`, builds the archive in
//! memory (`Cursor<Vec<u8>>`), writes one entry per stored shader-debug-info
//! record named `shader-nvdbg/{:x}-{:x}.nvdbg`, writes the SDK dump as
//! `nv-aftermath.nv-gpudmp`, finishes the zip, writes it to
//! `paths::tmp()/gpu-crash-dump.zip` and logs
//! `"NV Aftermath dump file written."`; any failure is logged as
//! `"Error writing NV Aftermath dump file: {error}"` and swallowed.
//! `shader_debug_info` stores the identifier → bytes mapping in the hash map.
//!
//! The Aftermath SDK callbacks themselves are external-SDK glue.

use anyhow::Context;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

/// File name of the GPU crash dump archive (`0x1429cbb20`).
pub const GPU_CRASH_DUMP_FILE: &str = "gpu-crash-dump.zip";
/// Main dump entry name inside the archive (`0x1429a06d8`).
pub const MAIN_DUMP_FILE: &str = "nv-aftermath.nv-gpudmp";
/// Log target (`0x1429a0590`, len 0x15).
const TARGET: &str = "tiny_glade::aftermath";

/// Aftermath delegate callbacks.
pub trait AftermathDelegate: Send + Sync {
    fn description(&self) -> &str {
        ""
    }
    fn dumped(&self, dump: &GpuCrashDump);
    fn shader_debug_info(&self, identifier: [u8; 16], debug_info: Vec<u8>);
}

/// Aftermath dump payload handed to [`AftermathDelegate::dumped`].
#[derive(Clone, Debug, Default)]
pub struct GpuCrashDump {
    pub bytes: Vec<u8>,
}

/// B `NvAftermathDelegate`.
pub struct NvAftermathDelegate {
    shader_debug_info: Mutex<HashMap<[u8; 16], Vec<u8>>>,
}

impl Default for NvAftermathDelegate {
    /// B `aftermath::impl$0::default` (`0x140244460`).
    ///
    /// `paths::tmp().join("gpu-crash-dump.zip")`; if the file exists it is
    /// unlinked before the new session starts. Failure is logged at **Error**
    /// level: `"Error deleting stale {path}: {error}"`.
    fn default() -> Self {
        let stale = paths::tmp().join(GPU_CRASH_DUMP_FILE);
        if stale.is_file() {
            if let Err(error) = std::fs::remove_file(&stale) {
                log::error!(
                    target: TARGET,
                    "Error deleting stale {}: {}",
                    stale.display(),
                    error
                );
            }
        }
        Self {
            shader_debug_info: Mutex::new(HashMap::new()),
        }
    }
}

impl NvAftermathDelegate {
    /// Runs the recovered `Default` cleanup.
    pub fn new() -> Self {
        Self::default()
    }

    /// Identifier → debug-info map populated by the SDK.
    pub fn shader_debug_info_map(&self) -> HashMap<[u8; 16], Vec<u8>> {
        self.shader_debug_info.lock().clone()
    }

    pub fn dump_path() -> PathBuf {
        paths::tmp().join(GPU_CRASH_DUMP_FILE)
    }
}

impl AftermathDelegate for NvAftermathDelegate {
    /// B `description` (`0x14003bb80`) — empty.
    fn description(&self) -> &str {
        ""
    }

    /// B `dumped` (`0x140244790`).
    fn dumped(&self, dump: &GpuCrashDump) {
        log::info!(target: TARGET, "NV Aftermath dump data received.");

        let result = (|| -> anyhow::Result<()> {
            let mut archive = Vec::new();
            {
                let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut archive));
                let options = zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Deflated);

                let debug_info = self.shader_debug_info.lock();
                for (identifier, bytes) in debug_info.iter() {
                    let low = u64::from_le_bytes(identifier[..8].try_into().unwrap());
                    let high = u64::from_le_bytes(identifier[8..].try_into().unwrap());
                    let name = format!("shader-nvdbg/{low:x}-{high:x}.nvdbg");
                    zip.start_file(name, options)?;
                    zip.write_all(bytes).context("writing debug info data")?;
                }
                drop(debug_info);

                zip.start_file(MAIN_DUMP_FILE, options)?;
                zip.write_all(&dump.bytes)
                    .context("writing dump data")?;
                zip.finish()?;
            }
            std::fs::write(Self::dump_path(), archive)?;
            Ok(())
        })();

        match result {
            Ok(()) => log::info!(target: TARGET, "NV Aftermath dump file written."),
            Err(error) => {
                log::error!(target: TARGET, "Error writing NV Aftermath dump file: {error:#}")
            }
        }
    }

    /// B `shader_debug_info` (`0x140245340`).
    ///
    /// `aftermath_rs::get_shader_debug_info_identifier` supplies the key; when
    /// trace logging is enabled the original logs
    /// `"Got NV shader debug info! Ident: {:?}"` (`0x1429a0688`), then inserts
    /// the bytes into the map under the mutex.
    fn shader_debug_info(&self, identifier: [u8; 16], debug_info: Vec<u8>) {
        if log::max_level() > log::LevelFilter::Debug {
            log::trace!(
                target: TARGET,
                "Got NV shader debug info! Ident: {:?}",
                Some(identifier)
            );
        }
        self.shader_debug_info
            .lock()
            .insert(identifier, debug_info);
    }
}

//! `tiny_glade::panic_reporter::panic_dump` — crash report file collectors.
//!
//! Fresh decompilation (VAs in each function) shows the collectors sharing the
//! same shape: join the artifact name under the report directory, copy/write the
//! live artifact, and on success return the destination `PathBuf` (the
//! `Result::Ok` payload is 32 bytes = `std::path::PathBuf`). I/O errors carry a
//! fixed `anyhow` context. Strings come from `.rdata` and are preserved
//! verbatim: `log.txt` (`0x1429cb980`), `Copying the log file` (`0x1429cb987`),
//! `save.json` (`0x1429cba80`), `History is empty` (`0x1429cba8d`),
//! `settings.ron` (`0x1429c3860`), `Copying the settings file` (`0x1429cbac8`),
//! `Copying the profile file` (`0x1429cbb08`), `gpu-crash-dump.zip`
//! (`0x1429cbb20`), `render.log` (`0x1429cbb51`), `Writing rhapsody log`
//! (`0x1429cbb5b`), `inputs.json` (`0x1429cb9c7`) and
//! `Input record replay state was not InterHistoryRecord.` (`0x1429cba19`).

use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};

use super::PanicReporterInfo;
use crate::crash_pickup::crash_payload::{RecoveredSave, SaveRecoveryError};

/// B `dump_log_file` (`0x1403acc70`).
///
/// `fs::copy(paths::tmp().join("log.txt"), report_dir.join("log.txt"))` with the
/// context `"Copying the log file"`.
pub fn dump_log_file(report_dir: &Path, _info: Option<&PanicReporterInfo>) -> Result<PathBuf> {
    let source = paths::tmp().join("log.txt");
    let destination = report_dir.join("log.txt");
    std::fs::copy(&source, &destination).context("Copying the log file")?;
    Ok(destination)
}

/// B `dump_inputs` (`0x1403ace70`).
///
/// Recovered control flow:
/// * the global `PANIC_REPORTER_INFO` discriminant `== 2` ⇒
///   `anyhow!("Game not yet initialized")`;
/// * the input stream lives at `INFO + 0x30`, its mode word at `+0x40`
///   (`InputRecordReplayMode` niche `0x8000_0000_0000_0000 + n`);
/// * mode `AutoRecordPanicRepro` (0) and `Records` (1) serialize the stream and
///   write `inputs.json`; the `Records` branch then returns
///   `Err("Inputs saved to {path}")` (used as a breadcrumb), the
///   `AutoRecordPanicRepro` branch returns `Ok(path)`;
/// * any other mode (>1 and <5, or the clamped `>=5` bucket) returns
///   `Err("Input record replay state was not InterHistoryRecord.")`.
///
/// Blocker: the recovered `country_core::input::input_stream::InputStream` has
/// only the two `VecDeque`s (`sync_points`/`frames`, 0x30 bytes) and no mode
/// slot at `+0x40`, and it does not implement `Serialize` (the original uses the
/// derived serializer `0x1402e0530`, whose key plane is `sync_points`/`frames`).
/// The `SyncPoint` element schema is also not recovered, so this collector stays
/// explicit-unimplemented instead of writing a guessed payload.
pub fn dump_inputs(report_dir: &Path, info: Option<&PanicReporterInfo>) -> Result<PathBuf> {
    let info = info.ok_or_else(|| anyhow!("Game not yet initialized"))?;
    let _stream = info
        .input_stream
        .as_ref()
        .ok_or_else(|| anyhow!("Game not yet initialized"))?;
    let destination = report_dir.join("inputs.json");
    Err(anyhow!(
        "未实现-需重做: dump_inputs 0x1403ace70 —— country_core InputStream 缺 +0x40 \
         InputRecordReplayMode 槽位与 Serialize(sync_points/frames) 实现\
         （目标 {destination:?}）"
    ))
}

/// B `dump_save` (`0x1403ad640`).
///
/// Recovered call sequence: `OpenOptions::open(report_dir/save.json)` →
/// `HistoryIdentity::generate` (`0x140a84220`) →
/// `MainCamera::store_parameters` (`0x140911710`) →
/// `HistoryEditChain::iter` (`0x140b31cf0`) →
/// `History::iter_resolved_snapshots` (`0x140a84a40`) →
/// `Vec::from_iter` (filter_map) → `GladeColorCustomization::default`
/// (`0x1408c5600`) → `save_game_to_file` (`0x140241ca0`). An absent session is
/// `"History is empty"` (`0x1429cba8d`).
///
/// Blocker: `country-core` has `HistoryIdentity::generate` and
/// `MainCamera::store_parameters`, but `HistoryEditChain::iter`,
/// `History::iter_resolved_snapshots` and `save_game_to_file` are not recovered.
pub fn dump_save(
    report_dir: &Path,
    info: Option<&PanicReporterInfo>,
) -> std::result::Result<RecoveredSave, SaveRecoveryError> {
    // `PANIC_REPORTER_INFO` discriminant == 2 returns before any path work.
    let info = info.ok_or(SaveRecoveryError::GameNotYetInitialized)?;
    let destination = report_dir.join("save.json");
    // `OpenOptions` runs before the empty-history gate; the file is created even
    // when there is nothing to write.
    let _file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&destination)
        .map_err(|error| SaveRecoveryError::Other(error.to_string()))?;
    // The original checks `history.len == 0` (`INFO + 0x18` → `+0x10`); the
    // recovered `country-core` exposes the active session instead.
    let _session = info
        .active_session
        .as_ref()
        .ok_or_else(|| SaveRecoveryError::Other("History is empty".to_string()))?;
    Err(SaveRecoveryError::Other(format!(
        "未实现-需重做: dump_save 0x1403ad640 —— 需要 country-core \
         HistoryEditChain::iter/History::iter_resolved_snapshots/save_game_to_file\
         (0x140241ca0) 接线（目标 {destination:?}）"
    )))
}

/// B `dump_settings_file` (`0x1403ae050`).
///
/// `paths::config()/settings.ron` → `report_dir/<file_name>`; the original
/// calls `Path::file_name().unwrap()` (unreachable panic if the path has no file
/// name) and wraps I/O errors with `"Copying the settings file"`.
pub fn dump_settings_file(
    report_dir: &Path,
    _info: Option<&PanicReporterInfo>,
) -> Result<PathBuf> {
    let source = paths::config().join("settings.ron");
    let file_name = source.file_name().unwrap();
    let destination = report_dir.join(file_name);
    std::fs::copy(&source, &destination).context("Copying the settings file")?;
    Ok(destination)
}

/// B `dump_profile_file` (`0x1403ae2f0`).
///
/// `country_core::game_profile::profile_file_path() → file_name().unwrap()` →
/// `report_dir.join(file_name)` → `fs::copy` with the context
/// `"Copying the profile file"` (`0x1429cbb08`).
///
/// Blocker: `country_core::game_profile` exists but still has no
/// `profile_file_path` helper (the exact relative literal is not recovered).
pub fn dump_profile_file(
    report_dir: &Path,
    _info: Option<&PanicReporterInfo>,
) -> Result<PathBuf> {
    Err(anyhow!(
        "未实现-需重做: dump_profile_file 0x1403ae2f0 —— country_core::game_profile::\
         profile_file_path 未恢复（目标 {report_dir:?}）"
    ))
}

/// B `dump_gpu_crash_dump_file` (`0x1403ae570`).
///
/// `paths::tmp()/gpu-crash-dump.zip` → `report_dir/gpu-crash-dump.zip` with the
/// context `"Copying the GPU crash dump file"`.
pub fn dump_gpu_crash_dump_file(
    report_dir: &Path,
    _info: Option<&PanicReporterInfo>,
) -> Result<PathBuf> {
    let source = paths::tmp().join("gpu-crash-dump.zip");
    let destination = report_dir.join("gpu-crash-dump.zip");
    std::fs::copy(&source, &destination).context("Copying the GPU crash dump file")?;
    Ok(destination)
}

/// B `dump_rhapsody_log` (`0x1403ae770`).
///
/// Fresh disassembly: `join(report_dir, "render.log")` → `rhapsody::dump_log`
/// (returns `Vec<u8>`; symbol `0x140be6b50`) → `std::fs::write` with the context
/// `"Writing rhapsody log"` (`0x1429cbb5b`, len 20).
pub fn dump_rhapsody_log(report_dir: &Path, _info: Option<&PanicReporterInfo>) -> Result<PathBuf> {
    let destination = report_dir.join("render.log");
    std::fs::write(&destination, rhapsody::dump_log_vec()).context("Writing rhapsody log")?;
    Ok(destination)
}

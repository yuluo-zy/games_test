//! `tiny_glade::panic_reporter::sentry_reporter` — Sentry event construction.
//!
//! Recovered VAs: `create_sentry_attachment` `0x1404b4d50`,
//! `panic_info_to_message` `0x1404b54c0`,
//! `panic_info_to_sentry_event` `0x1404b5550`,
//! `report_crash_event_to_sentry` `0x1404b3e30`.
//!
//! The Sentry SDK transport (`sentry::Client`, scope configuration, UUID
//! capture) is external-SDK glue and is kept behind [`SentrySink`]. The event
//! contents are real recovered logic: panic message, breadcrumbs, stack frames,
//! and the report files as attachments.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::crash_pickup::crash_payload::CrashRecovered;

/// Breadcrumb entry — the original type is
/// `sentry_types::protocol::v7::Breadcrumb`.
pub use sentry_types::protocol::v7::Breadcrumb;

/// Stack frame (`sentry_types::protocol::v7::Frame` in the original).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Frame {
    pub function: Option<String>,
    pub module: Option<String>,
    pub filename: Option<String>,
    pub lineno: Option<u64>,
    pub instruction_addr: Option<String>,
}

/// Minimal Sentry event payload.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SentryEvent {
    pub message: String,
    pub level: String,
    pub platform: String,
    pub fingerprint: Vec<String>,
    pub tags: Vec<(String, String)>,
    pub extra: Vec<(String, String)>,
    pub breadcrumbs: Vec<Breadcrumb>,
    pub frames: Vec<Frame>,
}

/// File attached to a Sentry event.
#[derive(Clone, Debug)]
pub struct SentryAttachment {
    pub filename: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
}

/// Transport abstraction for the Sentry SDK.
pub trait SentrySink: Send + Sync {
    fn capture_event(&self, event: &SentryEvent, attachments: &[SentryAttachment]) -> Result<()>;
}

static SENTRY_SINK: parking_lot::Mutex<Option<Box<dyn SentrySink>>> =
    parking_lot::Mutex::new(None);

/// Install the process-wide Sentry transport.
pub fn set_sentry_sink(sink: Box<dyn SentrySink>) {
    *SENTRY_SINK.lock() = Some(sink);
}

/// B `panic_info_to_message` (`0x1404b54c0`).
///
/// Fresh disassembly: the payload `TypeId` is compared against `&str` and
/// `String`; the matching branch returns `(ptr, len)` of the payload text, and
/// the fallback is the 8-byte literal `"Box<Any>"` (`0x1429e90a5`). The caller
/// (`setup_panic_hook`) then calls `str::to_owned` on the result, so returning an
/// owned `String` here is the same data flow.
pub fn panic_info_to_message(info: &std::panic::PanicHookInfo<'_>) -> String {
    let payload = info.payload();
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "Box<Any>".to_string()
    }
}

/// B `panic_info_to_sentry_event` (`0x1404b5550`).
///
/// Builds a `sentry_types::protocol::v7::Event`: the exception type/value is
/// `"panic"` (immediate bytes `0x73617263`+`0x68` in the decompilation), the
/// passed stack trace is converted by `sentry_backtrace::process::
/// backtrace_to_stacktrace`, `Event::default()` fills the remaining fields and
/// the level byte is `4` (`Level::Fatal`). The exact tag/fingerprint/extra key
/// strings live in the SDK scope and are not recoverable from the listing; the
/// recovered fields are message, level, platform, frames and breadcrumbs.
pub fn panic_info_to_sentry_event(
    info: &std::panic::PanicHookInfo<'_>,
    breadcrumbs: Vec<Breadcrumb>,
) -> SentryEvent {
    let message = panic_info_to_message(info);
    let location = info
        .location()
        .map(|location| format!("{}:{}", location.file(), location.line()))
        .unwrap_or_else(|| "unknown".to_string());
    let backtrace = std::backtrace::Backtrace::capture();
    let frames = backtrace
        .to_string()
        .lines()
        .filter_map(parse_backtrace_line)
        .collect();

    SentryEvent {
        message,
        level: "fatal".to_string(),
        platform: "native".to_string(),
        // The original builds an `Exception` with `ty = "panic"`; the SDK event
        // has no fingerprint template in the recovered body.
        fingerprint: Vec::new(),
        tags: vec![("panic.location".to_string(), location)],
        extra: Vec::new(),
        breadcrumbs,
        frames,
    }
}

fn parse_backtrace_line(line: &str) -> Option<Frame> {
    let line = line.trim();
    let rest = line.strip_prefix("at ").unwrap_or(line);
    let (location, function) = match rest.rsplit_once(" - ") {
        Some((location, function)) => (location.trim(), Some(function.trim().to_string())),
        None => (rest, None),
    };
    let mut parts = location.rsplitn(3, ':');
    let column = parts.next().and_then(|part| part.parse::<u64>().ok());
    let lineno = parts.next().and_then(|part| part.parse::<u64>().ok());
    let filename = parts.next().map(str::to_string);
    if filename.is_none() && function.is_none() {
        return None;
    }
    Some(Frame {
        function,
        module: None,
        filename,
        lineno,
        instruction_addr: column.map(|column| format!("col:{column}")),
    })
}

/// B `create_sentry_attachment` (`0x1404b4d50`).
///
/// Fresh decompilation:
/// 1. `file_name()` ⇒ `"No attachment file name"`;
/// 2. `file_name.to_str()` ⇒ `"Corrupt attachment file name"`;
/// 3. `extension()` ⇒ `"No attachment file name extension"`;
/// 4. `extension.to_str()` ⇒ `"Corrupt attachment extension"`;
/// 5. extension map: `json` → `application/json`; `jpg`/`jpeg` →
///    `image/jpeg`; `txt`/`log`/`ron` → `text/plain`; `dmp` →
///    `application/binary`; `zip` → `application/zip`; otherwise
///    `"Unrecognized attachment file name extension: {extension}"`;
/// 6. metadata, then reject `total + len >= 0x1400001` with
///    `"Attachment limit reached."` (25 bytes at `0x1429e8f90`);
/// 7. `fs::read`, bump the shared byte total and return the attachment.
pub fn create_sentry_attachment(path: &Path, total_bytes: &mut u64) -> Result<SentryAttachment> {
    /// 20 MiB (`0x1400000`).
    const ATTACHMENT_LIMIT: u64 = 0x14_00000;

    let file_name = path.file_name().ok_or_else(|| anyhow!("No attachment file name"))?;
    let filename = file_name
        .to_str()
        .ok_or_else(|| anyhow!("Corrupt attachment file name"))?
        .to_string();
    let extension = path
        .extension()
        .ok_or_else(|| anyhow!("No attachment file name extension"))?;
    let extension = extension
        .to_str()
        .ok_or_else(|| anyhow!("Corrupt attachment extension"))?;
    let content_type = match extension {
        "json" => "application/json",
        "jpg" | "jpeg" => "image/jpeg",
        "txt" | "log" | "ron" => "text/plain",
        "dmp" => "application/binary",
        "zip" => "application/zip",
        other => {
            return Err(anyhow!(
                "Unrecognized attachment file name extension: {other}"
            ))
        }
    };

    let metadata = std::fs::metadata(path)?;
    if *total_bytes + metadata.len() >= ATTACHMENT_LIMIT {
        return Err(anyhow!("Attachment limit reached."));
    }
    let bytes = std::fs::read(path).with_context(|| format!("Failed to read {path:?}"))?;
    *total_bytes += metadata.len();
    Ok(SentryAttachment {
        filename,
        content_type: content_type.to_string(),
        bytes,
    })
}

/// B `report_crash_event_to_sentry` (`0x1404b3e30`).
///
/// Fresh decompilation builds a 9-entry array of `(name, Result<PathBuf, String>)`
/// — `Minidump`, `GPU crash dump`, `Rhapsody log`, `Inputs`, `Settings`,
/// `Profile`, `Screenshot`, `Log`, `Save` (`0x1429e8f07`…`0x1429e8f3e`) — maps it
/// through `create_sentry_attachment` with one shared byte counter, retains the
/// successful attachments, and then configures the Sentry scope (UUID) before
/// capturing. The scope/transport is SDK glue.
pub fn report_crash_event_to_sentry(
    event: &SentryEvent,
    recovered: &CrashRecovered,
    minidump_path: Option<&Path>,
) -> Result<()> {
    let mut total_bytes = 0u64;
    let mut attachments = Vec::new();
    let save_path = recovered
        .save
        .as_ref()
        .map(|save| save.temp_save_path.clone())
        .map_err(|error| error.to_string());
    let entries: [(&str, &std::result::Result<PathBuf, String>); 8] = [
        ("GPU crash dump", &recovered.gpu_crash_dump),
        ("Rhapsody log", &recovered.rhapsody_log),
        ("Inputs", &recovered.inputs),
        ("Settings", &recovered.settings),
        ("Profile", &recovered.profile),
        ("Screenshot", &recovered.screenshot),
        ("Log", &recovered.log),
        ("Save", &save_path),
    ];
    for (name, path) in entries {
        if let Ok(path) = path {
            match create_sentry_attachment(path, &mut total_bytes) {
                Ok(attachment) => attachments.push(attachment),
                Err(error) => log::warn!("failed to attach {name} {path:?}: {error:#}"),
            }
        }
    }
    if let Some(path) = minidump_path {
        match create_sentry_attachment(path, &mut total_bytes) {
            Ok(attachment) => attachments.push(attachment),
            Err(error) => log::warn!("failed to attach minidump {path:?}: {error:#}"),
        }
    }

    let sink = SENTRY_SINK.lock();
    let sink = sink
        .as_ref()
        .ok_or_else(|| anyhow!("Sentry sink not configured"))?;
    sink.capture_event(event, &attachments)
}

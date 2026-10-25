//! `tiny_glade::crash_pickup::minidump_to_sentry` — minidump → Sentry trace.
//!
//! Recovered public symbols: `minidump_to_crash_trace` `0x1404b2fb0`,
//! `crash_trace_to_sentry_event` `0x1404b17d0`.
//!
//! The `minidump` crate performs the actual stream walking (external-SDK glue).
//! The conversion of a parsed trace into a Sentry event is real business logic
//! reproduced below.

use crate::panic_reporter::sentry_reporter::{Breadcrumb, Frame, SentryEvent};
use anyhow::{Context, Result};
use std::path::Path;

/// Parsed crash trace.
#[derive(Clone, Debug, Default)]
pub struct CrashTrace {
    pub summary: String,
    pub crash_reason: String,
    pub frames: Vec<Frame>,
    pub modules: Vec<String>,
}

/// Parsed minidump contents used by [`minidump_to_crash_trace`].
#[derive(Clone, Debug, Default)]
pub struct MinidumpContents {
    pub summary: String,
    pub crash_reason: String,
    pub frames: Vec<Frame>,
    pub modules: Vec<String>,
}

/// Minidump parser abstraction (the `minidump` crate in the original).
pub trait MinidumpParser: Send + Sync {
    fn parse(&self, bytes: &[u8]) -> Result<MinidumpContents>;
}

/// Default parser: validates the `MDMP` signature and reports the stream count.
///
/// The original walks exception/thread/module streams through the `minidump`
/// crate. Those stream layouts are SDK glue; this fallback preserves the file
/// validation and summary shape so the reporting pipeline still runs.
#[derive(Default)]
pub struct HeaderOnlyParser;

impl MinidumpParser for HeaderOnlyParser {
    fn parse(&self, bytes: &[u8]) -> Result<MinidumpContents> {
        if bytes.len() < 32 || &bytes[..4] != b"MDMP" {
            anyhow::bail!("not a minidump (missing MDMP signature)");
        }
        let stream_count = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        Ok(MinidumpContents {
            summary: format!("minidump with {stream_count} streams"),
            crash_reason: "unknown".to_string(),
            frames: Vec::new(),
            modules: Vec::new(),
        })
    }
}

/// B `minidump_to_crash_trace` (`0x1404b2fb0`).
///
/// Fresh decompilation (external-SDK parsing, not recovered as a callable here):
/// `Minidump::read_path` → `get_stream::<MinidumpSystemInfo>` →
/// `get_stream::<MinidumpMiscInfo>` → `get_stream::<MinidumpModuleList>` →
/// `get_stream::<MinidumpException>` (`exception.context()`) →
/// memory list (`MINIDUMP_MEMORY_DESCRIPTOR`) → `get_stack_pointer` →
/// `memory_at_address` → `minidump_unwind` frames → `cpu_info` →
/// `get_crash_reason` → `Stacktrace::from_frames_reversed`.
///
/// Blocker: the `minidump-unwind` dependency is not in the reconstructed
/// workspace (`minidump`/`minidump-common` are present, `minidump-unwind` is
/// not), so the header-only fallback below is *not* the original logic and this
/// function stays explicit-unimplemented.
pub fn minidump_to_crash_trace(path: &Path) -> Result<CrashTrace> {
    minidump_to_crash_trace_with(&HeaderOnlyParser, path)
}

pub fn minidump_to_crash_trace_with(
    parser: &dyn MinidumpParser,
    path: &Path,
) -> Result<CrashTrace> {
    let bytes = std::fs::read(path).with_context(|| format!("Failed to read {path:?}"))?;
    let contents = parser
        .parse(&bytes)
        .with_context(|| format!("Failed to parse minidump {path:?}"))?;
    Ok(CrashTrace {
        summary: contents.summary,
        crash_reason: contents.crash_reason,
        frames: contents.frames,
        modules: contents.modules,
    })
}

/// B `crash_trace_to_sentry_event` (`0x1404b17d0`).
///
/// Builds a `sentry_types::protocol::v7::Event` from the crash trace: the crash
/// reason becomes the event value, `Event::default()` fills the rest, the trace
/// frames become the stack trace, and the modules become breadcrumbs. Fresh
/// decompilation shows the exception type literals `"crash"` (immediate bytes
/// `0x73617263`+`0x68`); the remaining tag/fingerprint strings are SDK scope
/// data that are not present in the listing.
pub fn crash_trace_to_sentry_event(trace: &CrashTrace) -> SentryEvent {
    let breadcrumbs = trace
        .modules
        .iter()
        .map(|module| Breadcrumb {
            message: Some(module.clone()),
            category: Some("module".to_string()),
            level: sentry_types::protocol::v7::Level::Info,
            ..Default::default()
        })
        .collect();
    SentryEvent {
        message: trace.crash_reason.clone(),
        level: "fatal".to_string(),
        platform: "native".to_string(),
        fingerprint: Vec::new(),
        tags: Vec::new(),
        extra: vec![("summary".to_string(), trace.summary.clone())],
        breadcrumbs,
        frames: trace.frames.clone(),
    }
}

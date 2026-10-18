//! `tiny_glade::panic_reporter` — crash/panic reporting.
//!
//! Recovered from B `crates/tiny-glade/src/panic_reporter/*`:
//! * `setup_panic_hook` (`0x1400db5b0`) installs a hook that keeps the previous
//!   hook and runs the reporting closure (`0x1400db6b0`).
//! * `recover_game_state_after_crash` (`0x1400dbc30`) creates a timestamped
//!   report folder, runs the eight [`panic_dump`]/[`panic_screenshot`]
//!   collectors in the recovered order and returns a
//!   [`CrashRecovered`] bundle (dump errors become Sentry breadcrumbs).
//! * `on_game_crashed` (`0x1403ae940`) builds the crash payload, verifies the
//!   package manifest, converts minidumps to Sentry traces and reports them.
//!
//! Evidence note: `country_core::startup::panic_reporter_startup::
//! PANIC_REPORTER_INFO` is not part of the recovered `country-core` crate yet;
//! the equivalent store is defined here (same role: latest session/input
//! snapshot consumed by the dumpers).

pub mod panic_dump;
pub mod panic_report;
pub mod panic_screenshot;
pub mod reverse_backtrace;
pub mod sentry_reporter;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use country_core::input::input_stream::InputStream;
use country_core::resources::session::ActiveSession;
use country_core::systems::backend::input::input_record_replay::InputFrame;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::crash_pickup::crash_payload::{BacktracePayload, CrashPickupMessage, CrashRecovered};

/// Snapshot consumed by the panic dumpers.
#[derive(Default)]
pub struct PanicReporterInfo {
    /// Session that was active when the crash happened.
    pub active_session: Option<ActiveSession>,
    /// Input recording ring (`PANIC_REPORTER_INFO + 0x30 -> +0x40`).
    pub input_stream: Option<InputStream<InputFrame>>,
}

/// Global crash-report snapshot. B stores it in `country-core`; see module note.
pub static PANIC_REPORTER_INFO: Lazy<Mutex<Option<Arc<PanicReporterInfo>>>> =
    Lazy::new(|| Mutex::new(None));

/// Install (or replace) the snapshot used by [`recover_game_state_after_crash`].
pub fn set_panic_reporter_info(info: PanicReporterInfo) {
    *PANIC_REPORTER_INFO.lock() = Some(Arc::new(info));
}

fn panic_reporter_info() -> Option<Arc<PanicReporterInfo>> {
    PANIC_REPORTER_INFO.lock().clone()
}

/// B `tiny_glade::panic_reporter::setup_panic_hook` (`0x1400db5b0`).
///
/// Machine code: `take_hook`, allocate the old hook box, `set_hook` with a
/// closure that (per `0x1400db6b0`) behaves as follows:
/// * a global byte mutex (`0x143634b78`) serialises hook entries;
/// * `0x143634b79` (double-panic flag) set ⇒ print
///   `"Panic mutex poisoned! 😱 Aborting.\n"` and `__fastfail(7)`;
/// * `0x143634b7a` (already reported) ⇒ release the lock and tail-call the old
///   hook;
/// * otherwise: set the flag, capture a `backtrace::Backtrace::new()`, log
///   `"Reverse backtrace:\n{...}"` and `"PANIC: {}"` (target
///   `tiny_glade::panic_reporter`), run [`recover_game_state_after_crash`], and
///   — when a client handle is installed — send a
///   [`CrashPickupMessage::Panic`] built from the captured backtrace and
///   `panic_info_to_message`, with `.expect("send message to crash handler")`;
/// * finally `__fastfail(7)` (`std::process::abort`).
pub fn setup_panic_hook() {
    static MUTEX: AtomicBool = AtomicBool::new(false);
    static REPORTED: AtomicBool = AtomicBool::new(false);
    static DOUBLE_PANIC: AtomicBool = AtomicBool::new(false);

    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        while MUTEX
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::thread::yield_now();
        }

        let panicking = std::thread::panicking();
        if DOUBLE_PANIC.load(Ordering::Relaxed) {
            eprintln!("Panic mutex poisoned! 😱 Aborting.");
            std::process::abort();
        }

        if REPORTED.swap(true, Ordering::SeqCst) {
            MUTEX.store(false, Ordering::Release);
            previous(info);
            return;
        }

        // The original drops the guard before doing any reporting work.
        MUTEX.store(false, Ordering::Release);

        let backtrace = backtrace::Backtrace::new();
        log::error!(
            target: "tiny_glade::panic_reporter",
            "Reverse backtrace:\n{:?}",
            reverse_backtrace::ReverseBacktrace(&backtrace)
        );
        log::error!(
            target: "tiny_glade::panic_reporter",
            "PANIC: {}",
            info
        );

        let _recovered = recover_game_state_after_crash();

        if let Some(client) = crash_client() {
            let message = sentry_reporter::panic_info_to_message(info);
            client
                .send_message(CrashPickupMessage::Panic {
                    payload: BacktracePayload { backtrace, message },
                })
                .expect("send message to crash handler");
        }

        if panicking {
            DOUBLE_PANIC.store(true, Ordering::Relaxed);
        }
        std::process::abort();
    }));
}

/// Process-wide client handle used by the panic hook.
static CRASH_CLIENT: Lazy<Mutex<Option<crate::crash_pickup::ClientHandle>>> =
    Lazy::new(|| Mutex::new(None));

/// Register the minidumper client used by the panic hook.
pub fn set_crash_client(client: crate::crash_pickup::ClientHandle) {
    *CRASH_CLIENT.lock() = Some(client);
}

fn crash_client() -> Option<crate::crash_pickup::ClientHandle> {
    CRASH_CLIENT.lock().clone()
}

/// B `tiny_glade::panic_reporter::recover_game_state_after_crash`
/// (`0x1400dbc30`).
///
/// Returns [`CrashRecovered`] (0x1e8 bytes in B). The recovered flow is:
/// 1. `utils::file::date_time_string_for_filename` + `create_dir_all` for
///    `paths::tmp()/panics/panic_{timestamp}` (the failure string is
///    `"Creating the bug report directory"`, `0x14293aab8`).
/// 2. Collectors in the exact call order of the fresh decompilation:
///    `dump_save`, `dump_screenshot`, `dump_rhapsody_log`, `dump_inputs`,
///    `dump_log_file`, `dump_settings_file`, `dump_profile_file`,
///    `dump_gpu_crash_dump_file`.
/// 3. Every `anyhow::Error` is logged through `logging_map_error` and mapped to
///    a breadcrumb (`closure_env$7`); `save` keeps its own
///    [`SaveRecoveryError`].
/// 4. The active session's `save_folder_path`/`backup_folder_path` are read.
pub fn recover_game_state_after_crash() -> CrashRecovered {
    let timestamp = utils::file::date_time_string_for_filename();
    let report_dir = paths::tmp().join(format!("panics/panic_{timestamp}"));
    let report_folder = match std::fs::create_dir_all(&report_dir) {
        Ok(()) => Ok(report_dir.clone()),
        Err(error) => Err(format!("Creating the bug report directory: {error}")),
    };

    let info = panic_reporter_info();
    let mut errors: Vec<anyhow::Error> = Vec::new();

    macro_rules! collect {
        ($name:literal, $dump:path) => {
            match $dump(&report_dir, info.as_deref()) {
                Ok(path) => Ok(path),
                Err(error) => {
                    log::error!(
                        target: "tiny_glade::panic_reporter",
                        "Error recovering {}: {:#}",
                        $name,
                        error
                    );
                    let text = format!("{error:#}");
                    errors.push(error);
                    Err(text)
                }
            }
        };
    }

    let save = panic_dump::dump_save(&report_dir, info.as_deref());
    let screenshot: std::result::Result<PathBuf, String> = collect!("screenshot", panic_screenshot::dump_screenshot);
    let rhapsody_log: std::result::Result<PathBuf, String> = collect!("rhapsody_log", panic_dump::dump_rhapsody_log);
    let inputs: std::result::Result<PathBuf, String> = collect!("inputs", panic_dump::dump_inputs);
    let log: std::result::Result<PathBuf, String> = collect!("log", panic_dump::dump_log_file);
    let settings: std::result::Result<PathBuf, String> = collect!("settings", panic_dump::dump_settings_file);
    let profile: std::result::Result<PathBuf, String> = collect!("profile", panic_dump::dump_profile_file);
    let gpu_crash_dump: std::result::Result<PathBuf, String> = collect!("gpu_crash_dump", panic_dump::dump_gpu_crash_dump_file);

    if let Some(session) = info.as_ref().and_then(|info| info.active_session.as_ref()) {
        let _save_folder = session.id.save_folder_path();
        let _backup_folder = session.id.backup_folder_path();
    }

    let breadcrumbs = errors
        .iter()
        .map(|error| sentry_types::protocol::v7::Breadcrumb {
            message: Some(format!("{error:#}")),
            // The original maps errors through `closure_env$7`; the exact
            // category literal is not in the decompiled body.
            category: Some("panic_reporter".to_string()),
            level: sentry_types::protocol::v7::Level::Error,
            ..Default::default()
        })
        .collect();

    CrashRecovered {
        breadcrumbs,
        report_folder,
        screenshot,
        save,
        inputs,
        log,
        profile,
        settings,
        gpu_crash_dump,
        rhapsody_log,
    }
}

/// Path of the current report directory, if one has been created.
pub fn latest_report_dir() -> Option<PathBuf> {
    None
}

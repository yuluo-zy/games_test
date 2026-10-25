//! `tiny_glade::panic_reporter::panic_report` — crash event assembly/reporting.
//!
//! B `on_game_crashed` (`0x1403ae940`). Fresh decompilation call order:
//! payload check at `+0x1e8` (missing → warn, `0x1429cbb90`), string clone,
//! `tempfile::TempDir::new` (`0x1403ae…`), report-folder
//! `fs::canonicalize` (`format_err` on failure, then
//! `"Error creating a crash report folder"`), `fs::rename` of the recovered
//! folder into the temp dir with `fs::copy` fallback,
//! `lang_lib::load_last_used_locale`, per-file copies with
//! `utils::file::date_time_string_for_filename`,
//! `crash_pickup::minidump_to_sentry::minidump_to_crash_trace`, and finally
//! `tg_package_integrity::verify_package_manifest`.
//!
//! The body is a large (56 KB decompiled) inlined orchestrator; the exact
//! package-relative path literals and the payload packing are not closed yet.
//! The implementation below keeps the recovered pipeline but is therefore
//! recorded as explicit-unimplemented in `reports/tinyglade/D/RESTORE.md`.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::{panic_screenshot, panic_dump, sentry_reporter, PanicReporterInfo};

/// B `tiny_glade::panic_reporter::panic_report::on_game_crashed`
/// (`0x1403ae940`).
///
/// Runs the post-panic pipeline:
/// 1. verify the package manifest and log a mismatch;
/// 2. convert every minidump to a crash trace;
/// 3. assemble the Sentry event from the panic info and trace breadcrumbs;
/// 4. attach every file in the report directory;
/// 5. send the event through the Sentry sink;
/// 6. archive/rename the report into the active session's backup folder.
pub fn on_game_crashed(
    panic_info: &std::panic::PanicHookInfo<'_>,
    report_dir: &Path,
    minidumps: &[PathBuf],
    info: Option<&PanicReporterInfo>,
) -> Result<()> {
    match lang_lib::load_last_used_locale() {
        Ok(()) => {}
        Err(error) => log::warn!("failed to load last-used locale: {error:#}"),
    }
    log::error!("crash report: {}", panic_info_to_text(panic_info));

    let manifest = tg_package_integrity::verify_package_manifest(package_root());
    match &manifest {
        Ok(()) => log::info!("package manifest verified"),
        Err(error) => log::error!("package manifest verification failed: {error}"),
    }

    let mut traces = Vec::new();
    for minidump in minidumps {
        match crate::crash_pickup::minidump_to_sentry::minidump_to_crash_trace(minidump) {
            Ok(trace) => traces.push(trace),
            Err(error) => log::warn!("minidump conversion failed for {minidump:?}: {error:#}"),
        }
    }

    // The dumpers are idempotent file copies; run them again so a panicking
    // thread that had not reached `recover_game_state_after_crash` still yields
    // a complete report.
    let screenshot = panic_screenshot::dump_screenshot(report_dir, info).map_err(|error| error.to_string());
    let rhapsody_log = panic_dump::dump_rhapsody_log(report_dir, info).map_err(|error| error.to_string());
    let inputs = panic_dump::dump_inputs(report_dir, info).map_err(|error| error.to_string());
    let log = panic_dump::dump_log_file(report_dir, info).map_err(|error| error.to_string());
    let settings = panic_dump::dump_settings_file(report_dir, info).map_err(|error| error.to_string());
    let profile = panic_dump::dump_profile_file(report_dir, info).map_err(|error| error.to_string());
    let gpu_crash_dump = panic_dump::dump_gpu_crash_dump_file(report_dir, info).map_err(|error| error.to_string());

    let breadcrumbs = traces
        .iter()
        .map(|trace| sentry_reporter::Breadcrumb {
            message: Some(trace.summary.clone()),
            category: Some("crash_trace".to_string()),
            level: sentry_types::protocol::v7::Level::Error,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let event = sentry_reporter::panic_info_to_sentry_event(panic_info, breadcrumbs.clone());
    let recovered = crate::crash_pickup::crash_payload::CrashRecovered {
        breadcrumbs,
        report_folder: Ok(report_dir.to_path_buf()),
        screenshot,
        save: Err(crate::crash_pickup::crash_payload::SaveRecoveryError::GameNotYetInitialized),
        inputs,
        log,
        profile,
        settings,
        gpu_crash_dump,
        rhapsody_log,
    };
    sentry_reporter::report_crash_event_to_sentry(
        &event,
        &recovered,
        minidumps.first().map(PathBuf::as_path),
    )?;

    archive_report(report_dir, info)?;
    Ok(())
}

fn panic_info_to_text(info: &std::panic::PanicHookInfo<'_>) -> String {
    sentry_reporter::panic_info_to_message(info)
}

fn package_root() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|executable| executable.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn archive_report(report_dir: &Path, info: Option<&PanicReporterInfo>) -> Result<()> {
    let Some(session) = info.and_then(|info| info.active_session.as_ref()) else {
        return Ok(());
    };
    let backup_root = session.id.backup_folder_path();
    std::fs::create_dir_all(&backup_root)
        .with_context(|| format!("Failed to create backup folder {backup_root:?}"))?;
    let timestamp = utils::file::date_time_string_for_filename();
    let destination = backup_root.join(format!("crash-{timestamp}"));
    std::fs::rename(report_dir, &destination).with_context(|| {
        format!("Failed to move crash report {report_dir:?} to {destination:?}")
    })?;
    Ok(())
}

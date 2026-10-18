//! `tiny_glade::integrity_report_main` — package integrity entry point.
//!
//! B `verify_package_integrity_and_show_report` (`0x1402552b0`):
//! `Instant::now` → `tg_package_integrity::verify_package_manifest`
//! (`0x1402552d8`), then a per-variant error log and finally — unconditionally —
//! the winit `EventLoop::new` + `tg_integrity_report::run_integrity_report_gui`
//! (`0x140255…`, call at decompiled line 703), after which the process exits.
//! The GUI crate was deleted from the reconstructed workspace (see
//! `upper-modules-plan/tiny-glade/TASKS.md`), so the display is kept behind
//! [`IntegrityReportGui`].
//!
//! Recovered strings: `"Package manifest check passed. Took {:?}"`
//! (`0x1429a3ba8`), `"Could not read {}: {}"` (`0x1429a3958`),
//! `"Incorrect files"` (`0x1429a3b20`) and `"Extra files"` (`0x1429a3b58`).

use std::path::PathBuf;
use std::time::Instant;

/// Window/report front-end (B `tg_integrity_report::run_integrity_report_gui`).
pub trait IntegrityReportGui {
    fn run_integrity_report_gui(&self, issues: &str);
}

/// B `tiny_glade::integrity_report_main::verify_package_integrity_and_show_report`.
///
/// The original has no return value: verification problems are logged per
/// variant and the report GUI is shown regardless of the result.
pub fn verify_package_integrity_and_show_report(gui: Option<&dyn IntegrityReportGui>) {
    let root = package_root();
    let started = Instant::now();
    let result = tg_package_integrity::verify_package_manifest(&root);

    let issues = match result {
        Ok(()) => {
            log::info!(
                "Package manifest check passed. Took {:?}",
                started.elapsed()
            );
            String::new()
        }
        Err(error) => {
            log_integrity_error(&root, &error);
            format!("{error:#?}")
        }
    };

    if let Some(gui) = gui {
        gui.run_integrity_report_gui(&issues);
    }
}

/// Per-variant logging. The exact `{}` argument for the read/parse variants is
/// the manifest path in the original; only the file-list variants name their
/// lists explicitly (`"Incorrect files"` / `"Extra files"`).
fn log_integrity_error(
    root: &std::path::Path,
    error: &tg_package_integrity::PackageVerificationError,
) {
    use tg_package_integrity::PackageVerificationError as Error;
    match error {
        Error::Files(issues) => {
            if !issues.corrupt.is_empty() {
                log::error!("Incorrect files: {:#?}", issues.corrupt);
            }
            if !issues.extra.is_empty() {
                log::error!("Extra files: {:#?}", issues.extra);
            }
            for path in &issues.missing {
                log::error!("Could not read {}: {}", path, "missing");
            }
        }
        other => log::error!("Could not read {}: {}", root.display(), other),
    }
}

fn package_root() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|executable| executable.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
}

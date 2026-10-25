//! Print machine-readable diagnostics and return a failing status for invalid packages.
use std::{env, process::ExitCode};
use tg_package_integrity::{PackageVerificationError, verify_package_manifest};

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let Some(root) = args.next().filter(|_| args.next().is_none()) else {
        eprintln!("usage: inspect_package <package-root>");
        return ExitCode::from(2);
    };
    let verification = verify_package_manifest(root);
    let success = verification.is_ok();
    let diagnostic = match verification {
        Ok(()) => serde_json::json!({"ok": true}),
        Err(PackageVerificationError::Files(issues)) => serde_json::json!({
            "ok": false, "error": "files", "missing": issues.missing,
            "corrupt": issues.corrupt, "extra": issues.extra,
        }),
        Err(PackageVerificationError::InvalidManifestChecksum) => {
            serde_json::json!({"ok": false, "error": "checksum"})
        }
        Err(PackageVerificationError::ManifestRead(message)) => {
            serde_json::json!({"ok": false, "error": "read", "message": message})
        }
        Err(PackageVerificationError::ManifestParse(message)) => {
            serde_json::json!({"ok": false, "error": "parse", "message": message})
        }
        Err(PackageVerificationError::InvalidManifestPath(path)) => {
            serde_json::json!({"ok": false, "error": "path", "path": path})
        }
        Err(PackageVerificationError::InvalidManifestEntry { path, reason }) => {
            serde_json::json!({"ok": false, "error": "entry", "path": path, "message": reason})
        }
        Err(PackageVerificationError::Filesystem(message)) => {
            serde_json::json!({"ok": false, "error": "filesystem", "message": message})
        }
        Err(PackageVerificationError::UnsupportedVersion(version)) => {
            serde_json::json!({"ok": false, "error": "version", "version": version})
        }
    };
    println!("{diagnostic:#}");
    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

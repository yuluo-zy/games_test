use std::{env, fs};
use tg_package_integrity::{
    MANIFEST_PATH, PackageManifest, PackageVerificationError, is_valid_manifest_checksum,
    verify_package_manifest,
};
fn main() {
    let root = env::args_os()
        .nth(1)
        .expect("usage: inspect_package <package-root>");
    let bytes = fs::read(std::path::Path::new(&root).join(MANIFEST_PATH)).expect("read manifest");
    let manifest: PackageManifest = serde_json::from_slice(&bytes).expect("parse manifest");
    let result = match verify_package_manifest(&root) {
        Ok(()) => serde_json::json!({"ok": true}),
        Err(PackageVerificationError::Files(issues)) => serde_json::json!({
            "ok": false, "error": "files", "missing": issues.missing, "corrupt": issues.corrupt, "extra": issues.extra
        }),
        Err(error) => serde_json::json!({"ok": false, "error": format!("{error:?}")}),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "candidate_execution": true, "original_function_execution": false,
            "manifest_checksum_valid": is_valid_manifest_checksum(&bytes),
            "version": manifest.version, "file_entries": manifest.files.len(), "result": result
        }))
        .unwrap()
    );
}

use std::{env, fs};
use tg_package_integrity::{
    MANIFEST_PATH, PackageManifest, PackageVerificationError, is_valid_manifest_checksum,
    verify_package_manifest,
};
fn main() {
    let root = env::args_os()
        .nth(1)
        .expect("usage: inspect_package <package-root>");
    // Run the original read -> checksum -> parse -> version -> files chain first.
    // Optional metadata must not panic or parse ungated bytes before this result.
    let result = match verify_package_manifest(&root) {
        Ok(()) => serde_json::json!({"ok": true}),
        Err(PackageVerificationError::Files(issues)) => serde_json::json!({
            "ok": false, "error": "files", "missing": issues.missing, "corrupt": issues.corrupt, "extra": issues.extra
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
        Err(PackageVerificationError::UnsupportedVersion(version)) => {
            serde_json::json!({"ok": false, "error": "version", "version": version})
        }
    };
    let bytes = fs::read(std::path::Path::new(&root).join(MANIFEST_PATH)).ok();
    let checksum = bytes.as_deref().map(is_valid_manifest_checksum);
    let manifest: Option<PackageManifest> = if checksum == Some(true) {
        bytes
            .as_deref()
            .and_then(|bytes| serde_json::from_slice(bytes).ok())
    } else {
        None
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "candidate_execution": true, "original_function_execution": false,
            "manifest_checksum_valid": checksum,
            "version": manifest.as_ref().map(|manifest| manifest.version),
            "file_entries": manifest.as_ref().map(|manifest| manifest.files.len()), "result": result
        }))
        .unwrap()
    );
}

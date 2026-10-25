use serde_json::{Value, json};
use std::{fs, path::Path};
use tg_package_integrity::{self as pi, PackageVerificationError as E};

fn result(root: &Path) -> Value {
    match pi::verify_package_manifest(root) {
        Ok(()) => json!({"kind":"ok"}),
        Err(E::Files(i)) => json!({"kind":"files","lists":[i.missing,i.corrupt,i.extra]}),
        Err(E::ManifestRead(s)) => json!({"kind":"read","message":s}),
        Err(E::ManifestParse(s)) => json!({"kind":"parse","message":s}),
        Err(E::UnsupportedVersion(v)) => json!({"kind":"version","version":v}),
        Err(E::InvalidManifestChecksum) => json!({"kind":"checksum"}),
    }
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    let root = Path::new(&args[2]);
    if args[1] == "inspect" {
        println!("{}", result(root));
        return;
    }
    let hash = blake3::hash(b"content").to_hex().to_string();
    let cases = [
        ("root-tmp-file", "tmp", "tmp"),
        ("root-tmp-listed", "tmp", "tmp"),
        ("uppercase-tmp", "Tmp/file", ""),
        ("known-case", "Case", "CASE"),
        ("separator-alias", "sub/file", "sub\\file"),
        ("dot-alias", "good", "./good"),
        ("self-dot-alias", "", "./build-info/manifest.json"),
        ("empty-file-name", "", ""),
        ("nul-file-name", "", "bad\0name"),
        ("directory-known", "sub/file", "sub"),
        ("unicode-and-surrogate", "中文🧱", "中文🧱"),
    ];
    fs::create_dir_all(root).unwrap();
    let mut records = Vec::new();
    for (name, disk_name, manifest_name) in cases {
        let disk = root.join(name);
        fs::create_dir_all(disk.join("build-info")).unwrap();
        if !disk_name.is_empty() {
            let path = disk.join(disk_name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, b"content").unwrap();
        }
        let mut files = vec![json!([pi::MANIFEST_PATH, "ignored"])];
        if name != "root-tmp-file" && name != "uppercase-tmp" {
            files.push(json!([
                manifest_name,
                if name == "root-tmp-listed" {
                    "bad"
                } else {
                    &hash
                }
            ]));
        }
        if name == "unicode-and-surrogate" {
            use std::os::windows::ffi::OsStringExt;
            let bad = std::ffi::OsString::from_wide(&[0xd800]);
            fs::write(disk.join(bad), b"invalid Unicode").unwrap();
            files.push(json!(["good", &hash]));
            fs::write(disk.join("good"), b"content").unwrap();
        }
        let source = json!({"version":1,"files":files}).to_string();
        let mut bytes = source.as_bytes().to_vec();
        let mut found = false;
        for nonce in 0..1_000_000_u32 {
            bytes.truncate(source.len());
            for bit in 0..20 {
                bytes.push(if nonce & (1 << bit) == 0 { b' ' } else { b'\t' });
            }
            if pi::is_valid_manifest_checksum(&bytes) {
                found = true;
                break;
            }
        }
        assert!(found);
        fs::write(disk.join(pi::MANIFEST_PATH), &bytes).unwrap();
        records.push(json!({"name":name,"bytes":String::from_utf8(bytes).unwrap(),"candidate":result(&disk)}));
    }
    fs::write(
        root.join("catalog.json"),
        serde_json::to_vec_pretty(&records).unwrap(),
    )
    .unwrap();
    println!("{}", json!({"fixture_count":records.len()}));
}

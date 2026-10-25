use serde_json::{json, Value};
use std::{fs, path::Path, time::Instant};
use tg_package_integrity::{self as pi, PackageVerificationError as E};
fn gated(source: &str) -> Vec<u8> {
    let mut bytes = source.as_bytes().to_vec();
    for n in 0..1_000_000u32 {
        bytes.truncate(source.len());
        for bit in 0..20 {
            bytes.push(if n & (1 << bit) == 0 { b' ' } else { b'\t' });
        }
        if pi::is_valid_manifest_checksum(&bytes) {
            return bytes;
        }
    }
    panic!("gate search exhausted")
}
fn result(root: &Path) -> Value {
    match pi::verify_package_manifest(root) {
        Ok(()) => json!({"kind":"ok"}),
        Err(E::InvalidManifestChecksum) => json!({"kind":"checksum"}),
        Err(E::ManifestRead(s)) => json!({"kind":"read","message":s}),
        Err(E::ManifestParse(s)) => json!({"kind":"parse","message":s}),
        Err(E::UnsupportedVersion(v)) => json!({"kind":"version","version":v}),
        Err(E::Files(i)) => {
            json!({"kind":"files","missing":i.missing,"corrupt":i.corrupt,"extra":i.extra})
        }
    }
}
#[repr(C)]
struct Mem {
    cb: u32,
    faults: u32,
    peak_ws: usize,
    ws: usize,
    peak_paged: usize,
    paged: usize,
    peak_nonpaged: usize,
    nonpaged: usize,
    pagefile: usize,
    peak_pagefile: usize,
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcess() -> isize;
    fn K32GetProcessMemoryInfo(p: isize, m: *mut Mem, s: u32) -> i32;
}
fn mem() -> (usize, usize) {
    let mut m: Mem = unsafe { std::mem::zeroed() };
    m.cb = std::mem::size_of::<Mem>() as u32;
    assert_ne!(
        unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut m, m.cb) },
        0
    );
    (m.ws, m.peak_ws)
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(&args[2]);
    match args[1].as_str() {
        "fixtures" => {
            fs::create_dir_all(root).unwrap();
            let self_row = r#"[["build-info/manifest.json","ignored"]]"#;
            let good = format!(r#"{{"version":1,"files":{self_row}}}"#);
            let cases: Vec<(&str, String)> = vec![
                ("success", good.clone()),
                (
                    "unknown-field",
                    format!(
                        r#"{{"version":1,"files":{self_row},"unknown":{{"nested":[null,true]}}}}"#
                    ),
                ),
                (
                    "reordered",
                    format!(r#"{{"files":{self_row},"version":1}}"#),
                ),
                ("sequence", format!(r#"[1,{self_row}]"#)),
                ("version-zero", r#"{"version":0,"files":[]}"#.into()),
                ("version-two", r#"{"version":2,"files":[]}"#.into()),
                (
                    "version-max-u32",
                    r#"{"version":4294967295,"files":[]}"#.into(),
                ),
                (
                    "version-overflow",
                    r#"{"version":4294967296,"files":[]}"#.into(),
                ),
                ("version-negative", r#"{"version":-1,"files":[]}"#.into()),
                ("version-float", r#"{"version":1.0,"files":[]}"#.into()),
                ("version-null", r#"{"version":null,"files":[]}"#.into()),
                ("version-string", r#"{"version":"1","files":[]}"#.into()),
                (
                    "duplicate-version",
                    r#"{"version":1,"version":1,"files":[]}"#.into(),
                ),
                (
                    "duplicate-files",
                    r#"{"version":1,"files":[],"files":[]}"#.into(),
                ),
                ("missing-version", r#"{"files":[]}"#.into()),
                ("missing-files", r#"{"version":1}"#.into()),
                ("capitalized-version", r#"{"Version":1,"files":[]}"#.into()),
                ("sequence-extra", format!(r#"[1,{self_row},true]"#)),
                ("sequence-short", r#"[1]"#.into()),
                ("tuple-short", r#"{"version":1,"files":[["a"]]}"#.into()),
                (
                    "tuple-extra",
                    r#"{"version":1,"files":[["a","b","c"]]}"#.into(),
                ),
                (
                    "file-object",
                    r#"{"version":1,"files":[{"name":"a","hash":"b"}]}"#.into(),
                ),
                ("file-number", r#"{"version":1,"files":[[1,"b"]]}"#.into()),
                ("hash-bool", r#"{"version":1,"files":[["a",false]]}"#.into()),
                ("parse-after-gate", "not json".into()),
                (
                    "schema-before-version",
                    r#"{"version":2,"files":[[1,"b"]]}"#.into(),
                ),
            ];
            let mut catalog = Vec::new();
            for (name, source) in cases {
                let path = root.join(name);
                fs::create_dir_all(path.join("build-info")).unwrap();
                let bytes = gated(&source);
                fs::write(path.join(pi::MANIFEST_PATH), &bytes).unwrap();
                catalog.push(json!({"name":name,"bytes":String::from_utf8(bytes).unwrap(),"candidate":result(&path)}));
            }
            let missing = root.join("read-missing");
            fs::create_dir_all(&missing).unwrap();
            catalog.push(json!({"name":"read-missing","candidate":result(&missing)}));
            let bad = root.join("checksum-before-parse");
            fs::create_dir_all(bad.join("build-info")).unwrap();
            fs::write(bad.join(pi::MANIFEST_PATH), b"not json").unwrap();
            catalog.push(json!({"name":"checksum-before-parse","candidate":result(&bad)}));
            let disk = root.join("ordered-files");
            fs::create_dir_all(disk.join("build-info")).unwrap();
            fs::write(disk.join("good"), b"content").unwrap();
            fs::write(disk.join("bad"), b"actual").unwrap();
            fs::write(disk.join("extra"), b"extra").unwrap();
            fs::create_dir_all(disk.join("tmp")).unwrap();
            fs::write(disk.join("tmp/ignored"), b"ignored").unwrap();
            fs::create_dir_all(disk.join("sub/tmp")).unwrap();
            fs::write(disk.join("sub/tmp/nested"), b"extra").unwrap();
            let hash = blake3::hash(b"content").to_hex().to_string();
            let mut files = vec![
                json!([pi::MANIFEST_PATH, "ignored"]),
                json!(["good", hash]),
                json!(["bad", "wrong"]),
                json!(["good", hash.to_uppercase()]),
            ];
            for i in 0..130 {
                files.push(json!([format!("missing-{i}"), "wrong"]));
            }
            files.push(json!(["missing-0", "wrong"]));
            files.push(json!([
                "build-info/manifest.json/../manifest.json",
                "wrong"
            ]));
            fs::write(
                disk.join(pi::MANIFEST_PATH),
                gated(&json!({"version":1,"files":files}).to_string()),
            )
            .unwrap();
            catalog.push(json!({"name":"ordered-files","candidate":result(&disk)}));
            fs::write(
                root.join("catalog.json"),
                serde_json::to_vec_pretty(&catalog).unwrap(),
            )
            .unwrap();
            println!("{}", json!({"fixture_count":catalog.len()}));
        }
        "inspect" => println!("{}", result(root)),
        "bench" => {
            let samples: usize = args[3].parse().unwrap();
            let base = mem().0;
            let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let flag = done.clone();
            let monitor = std::thread::spawn(move || {
                let mut ticks = 0;
                while !flag.load(std::sync::atomic::Ordering::Relaxed) {
                    if ticks % 10 == 0 {
                        eprintln!("live_memory: {:?}", mem());
                    }
                    ticks += 1;
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            });
            let mut results = Vec::new();
            for _ in 0..samples {
                let start = Instant::now();
                let outcome = result(root);
                results.push(
                    json!({"elapsed_ms":start.elapsed().as_secs_f64()*1000.,"result":outcome}),
                );
            }
            done.store(true, std::sync::atomic::Ordering::Relaxed);
            monitor.join().unwrap();
            let (ws, peak) = mem();
            println!(
                "{}",
                json!({"root":root,"baseline_ws":base,"ending_ws":ws,"process_peak_ws":peak,"runs":results,"profile":"release exact original dependency versions; OS cache not cleared"})
            );
        }
        "scan-stream" => {
 let bytes=fs::read(root.join(pi::MANIFEST_PATH)).unwrap();
 let manifest:pi::PackageManifest=serde_json::from_slice(&bytes).unwrap();
 for (name,_) in manifest.files.iter().take(80) {
  if name==pi::MANIFEST_PATH {continue;}
  eprintln!("starting {} bytes={}",name,fs::metadata(root.join(name)).map(|m|m.len()).unwrap_or(0));
  let started=Instant::now();let mut file=fs::File::open(root.join(name)).unwrap();let mut hasher=blake3::Hasher::new();hasher.update_reader(&mut file).unwrap();
  eprintln!("done {} ms={}",name,started.elapsed().as_secs_f64()*1000.);
 }
},
        "perf-fixture" => {
            let n: usize = args[3].parse().unwrap();
            let size: usize = args[4].parse().unwrap();
            fs::create_dir_all(root.join("build-info")).unwrap();
            let bytes = vec![42; size];
            let hash = blake3::hash(&bytes).to_hex().to_string();
            let mut files = vec![json!([pi::MANIFEST_PATH, "ignored"])];
            for i in 0..n {
                let name = format!("data-{i}");
                fs::write(root.join(&name), &bytes).unwrap();
                files.push(json!([name, hash]));
            }
            fs::write(
                root.join(pi::MANIFEST_PATH),
                gated(&json!({"version":1,"files":files}).to_string()),
            )
            .unwrap();
            println!(
                "{}",
                json!({"distinct_files":n,"bytes_per_file":size,"total_bytes":n*size})
            );
        }
        _ => panic!("unknown command"),
    }
}

//! Enforces rhapsody's internal layering.
//!
//! * Every top-level module sits at one place in [`LAYERS`] (bottom to top) and may only
//!   name modules below it: the module dependency graph is a DAG by construction.
//! * No module re-exports another top-level module's items (`pub use crate::other::...`).
//!   A path has one home; `mod.rs` files only flatten their own children.
//!
//! The scan is textual (no macro expansion) and ignores `#[cfg(test)]` code, comments and
//! `crate::` paths whose first segment is a root re-export such as `crate::Error`.
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

/// Bottom to top. A module may depend on modules listed before it.
const LAYERS: &[&str] = &[
    "error",       // error type and result categories
    "trace",       // opt-in submission tracing
    "log_ring",    // bounded in-memory log
    "format",      // Vulkan format tables
    "profile",     // CPU recording profile and command workload counters
    "sync",        // queue ids, sync points, timelines, device generation
    "accounting",  // memory budgets, host credit, allocator statistics
    "collections", // bounded containers and CPU pools
    "retire",      // retirement queue
    "native",      // instance, physical device, surface, access tables
    "tracking",    // GPU-use tracking and ranges
    "device",      // logical device, memory, queues, extension slots
    "resource",    // buffers, textures, samplers, managed handles
    "command",     // command buffers, barriers, descriptor sets, copies
    "pipeline",    // shaders and pipelines
    "present",     // swapchains
    "transfer",    // uploads
    "bindless",    // bindless heap and parameters
    "rg",          // render graph
];

fn src_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rank(module: &str) -> Option<usize> {
    LAYERS.iter().position(|name| *name == module)
}

fn top_module(root: &Path, file: &Path) -> String {
    let relative = file.strip_prefix(root).unwrap();
    let first = relative.components().next().unwrap().as_os_str().to_string_lossy();
    first.trim_end_matches(".rs").to_owned()
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

/// Source without comments and without `#[cfg(test)]` items.
fn production_text(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut kept = String::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        if line.trim() == "#[cfg(test)]" {
            let mut next = index + 1;
            while next < lines.len() && lines[next].trim_start().starts_with("#[") {
                next += 1;
            }
            if next < lines.len() && lines[next].trim_end().ends_with(';') {
                index = next + 1; // `mod tests;`
                continue;
            }
            let (mut depth, mut started) = (0i32, false);
            let mut end = next;
            while end < lines.len() {
                for ch in lines[end].chars() {
                    match ch {
                        '{' => {
                            depth += 1;
                            started = true;
                        }
                        '}' => depth -= 1,
                        _ => {}
                    }
                }
                end += 1;
                if started && depth <= 0 {
                    break;
                }
            }
            index = end;
            continue;
        }
        let code = line.split("//").next().unwrap_or("");
        kept.push_str(code);
        kept.push('\n');
        index += 1;
    }
    kept
}

fn leading_identifier(text: &str) -> &str {
    let text = text.trim_start();
    let end = text
        .find(|ch: char| !(ch.is_alphanumeric() || ch == '_'))
        .unwrap_or(text.len());
    &text[..end]
}

/// First segment after every `crate::` (expanding `crate::{a::b, c}` groups).
fn crate_roots(text: &str) -> Vec<String> {
    let mut roots = Vec::new();
    for (at, _) in text.match_indices("crate::") {
        let before = text[..at].chars().next_back();
        if before.is_some_and(|ch| ch.is_alphanumeric() || ch == '_' || ch == ':') {
            continue;
        }
        let rest = &text[at + "crate::".len()..];
        if let Some(group) = rest.strip_prefix('{') {
            let (mut depth, mut item_start) = (1i32, 0usize);
            for (offset, ch) in group.char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            roots.push(leading_identifier(&group[item_start..offset]).to_owned());
                            break;
                        }
                    }
                    ',' if depth == 1 => {
                        roots.push(leading_identifier(&group[item_start..offset]).to_owned());
                        item_start = offset + 1;
                    }
                    _ => {}
                }
            }
        } else {
            roots.push(leading_identifier(rest).to_owned());
        }
    }
    roots.retain(|root| !root.is_empty());
    roots
}

/// `crate::` roots named by `pub use` statements.
fn reexported_roots(text: &str) -> Vec<String> {
    let mut roots = Vec::new();
    for (at, _) in text.match_indices("pub") {
        let rest = &text[at..];
        let statement = ["pub use ", "pub(crate) use ", "pub(super) use "]
            .iter()
            .find_map(|prefix| rest.strip_prefix(prefix));
        if let Some(statement) = statement {
            let end = statement.find(';').unwrap_or(statement.len());
            roots.extend(crate_roots(&statement[..end]));
        }
    }
    roots
}

/// Files declared as `#[cfg(test)] mod name;`: test code is not part of the layering.
fn test_module_files(files: &[PathBuf]) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for file in files {
        let text = fs::read_to_string(file).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        let dir = if file.file_name().is_some_and(|name| name == "mod.rs") {
            file.parent().unwrap().to_owned()
        } else {
            file.with_extension("")
        };
        for (index, line) in lines.iter().enumerate() {
            if !(line.starts_with("#[cfg(test)]") || line.starts_with("#[cfg(all(test")) {
                continue;
            }
            let declaration = lines[index + 1..]
                .iter()
                .find(|next| !next.trim_start().starts_with("#["));
            let name = declaration
                .map(|next| next.trim())
                .and_then(|next| next.split_once("mod ").filter(|(prefix, _)| prefix.is_empty() || prefix.starts_with("pub")))
                .and_then(|(_, next)| next.strip_suffix(';'));
            if let Some(name) = name {
                found.push(dir.join(format!("{name}.rs")));
                found.push(dir.join(name).join("mod.rs"));
            }
        }
    }
    found
}

struct Scan {
    /// `(file, module, dependency)` for each cross-module path.
    dependencies: Vec<(String, String, String)>,
    /// `(file, module, re-exported module)`.
    reexports: Vec<(String, String, String)>,
    unknown: Vec<String>,
}

fn scan() -> Scan {
    let root = src_root();
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    files.sort();
    let tests = test_module_files(&files);
    files.retain(|file| !tests.contains(file));
    let mut scan = Scan {
        dependencies: Vec::new(),
        reexports: Vec::new(),
        unknown: Vec::new(),
    };
    for file in files {
        if file.file_name().is_some_and(|name| name == "lib.rs") {
            continue;
        }
        let module = top_module(&root, &file);
        if rank(&module).is_none() {
            scan.unknown.push(module.clone());
            continue;
        }
        let name = file.strip_prefix(&root).unwrap().display().to_string();
        let text = production_text(&fs::read_to_string(&file).unwrap());
        for dependency in crate_roots(&text) {
            if dependency != module && rank(&dependency).is_some() {
                scan.dependencies.push((name.clone(), module.clone(), dependency));
            }
        }
        for target in reexported_roots(&text) {
            if target != module && rank(&target).is_some() {
                scan.reexports.push((name.clone(), module.clone(), target));
            }
        }
    }
    scan.unknown.sort();
    scan.unknown.dedup();
    scan
}

#[test]
fn every_top_level_module_has_a_layer() {
    let scan = scan();
    assert!(
        scan.unknown.is_empty(),
        "place these modules in LAYERS (bottom to top): {:?}",
        scan.unknown
    );
}

#[test]
fn modules_only_depend_on_lower_layers() {
    let scan = scan();
    let mut violations: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for (file, module, dependency) in &scan.dependencies {
        if rank(dependency) > rank(module) {
            violations
                .entry((module.clone(), dependency.clone()))
                .or_default()
                .push(file.clone());
        }
    }
    let report: Vec<String> = violations
        .iter()
        .map(|((module, dependency), files)| {
            let mut files = files.clone();
            files.dedup();
            format!("`{module}` must not depend on `{dependency}` (in {files:?})")
        })
        .collect();
    assert!(report.is_empty(), "layering violations:\n{}", report.join("\n"));
}

#[test]
fn no_module_reexports_another_modules_items() {
    let scan = scan();
    let report: Vec<String> = scan
        .reexports
        .iter()
        .map(|(file, module, target)| format!("{file}: `{module}` re-exports `{target}`"))
        .collect();
    assert!(
        report.is_empty(),
        "cross-module re-exports (a path has one home):\n{}",
        report.join("\n")
    );
}

#[test]
fn the_scanner_sees_a_known_dependency() {
    // Guards the textual scan itself: `resource` builds on `device`.
    let scan = scan();
    assert!(scan
        .dependencies
        .iter()
        .any(|(_, module, dependency)| module == "resource" && dependency == "device"));
}

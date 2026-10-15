//! 原版 oracle 重放的公共读取/比对代码。
#![allow(dead_code)]
use std::path::PathBuf;
use std::process::Command;

fn evidence(relative: &str) -> Option<PathBuf> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../reverse-engineering/tiny-glade/evidence/render-recovery")
        .join(relative);
    path.exists().then_some(path)
}

/// 读一个 gz 的 jsonl，返回每行的 `(input 原文, 期望值原文)`；字符串值去引号，其余保留 JSON 文本。
fn load(relative: &str) -> Option<Vec<(serde_json::Value, serde_json::Value)>> {
    let path = evidence(relative)?;
    let text = if relative.ends_with(".gz") {
        let out = Command::new("gzip").arg("-dc").arg(&path).output().expect("gzip");
        assert!(out.status.success(), "gzip -dc {path:?}");
        String::from_utf8(out.stdout).expect("utf8")
    } else {
        std::fs::read_to_string(&path).expect("read")
    };
    // 两种承载：jsonl（每行一条）或整个 JSON 数组。
    let raw: Vec<serde_json::Value> = if relative.ends_with(".json") {
        serde_json::from_str(&text).expect("json array")
    } else {
        text.lines().map(|l| serde_json::from_str(l).expect("jsonl")).collect()
    };
    let rows = raw
        .into_iter()
        .map(|row| {
            let expected = row
                .get("original")
                .or_else(|| row.get("native"))
                .or_else(|| row.get("output"))
                .or_else(|| row.get("result"))
                .expect("expected field")
                .clone();
            (row.get("input").or_else(|| row.get("case")).expect("input field").clone(), expected)
        })
        .collect();
    Some(rows)
}

pub fn replay(name: &str, relative: &str, mut run: impl FnMut(&serde_json::Value) -> serde_json::Value) {
    std::panic::set_hook(Box::new(|_| {}));
    let Some(rows) = load(relative) else {
        eprintln!("skip {name}: evidence {relative} not present");
        return;
    };
    let mut bad = Vec::new();
    for (input, expected) in &rows {
        let got = run(input);
        if &got != expected && bad.len() < 8 {
            bad.push(format!("input={input} original={expected} got={got}"));
        } else if &got != expected {
            bad.push(String::new());
        }
    }
    let _ = std::panic::take_hook();
    let mismatches = bad.len();
    assert!(
        mismatches == 0,
        "{name}: {mismatches}/{} mismatches\n{}",
        rows.len(),
        bad.iter().filter(|s| !s.is_empty()).cloned().collect::<Vec<_>>().join("\n")
    );
    eprintln!("{name}: {} cases match", rows.len());
}

pub fn word(s: &str) -> u64 {
    u64::from_str_radix(s, 16).expect("hex word")
}
pub fn text(v: &serde_json::Value) -> String {
    v.as_str().expect("string").to_string()
}
pub fn jstr(s: impl Into<String>) -> serde_json::Value {
    serde_json::Value::String(s.into())
}
pub fn ok_or_panic<T>(r: Result<T, impl Sized>) -> &'static str {
    if r.is_ok() { "ok" } else { "panic" }
}


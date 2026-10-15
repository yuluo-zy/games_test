//! `startup_session.rs` — 新会话的起始存档目录与复制（B `startup::startup_session`）。
//!
//! 证据（`startup/ALGORITHMS.md` §1.2、§2.5）：
//! - 新会话复制的模板目录：`demo` 开关（0x143634d90）为真时 `assets/starting-builds/demo`（0x142c849cb），否则
//!   `assets/starting-builds/full`（0x142c849b0，长 0x1b），两个串已逐字节核对；
//! - `copy_dir_all<PathBuf, PathBuf>`（0x141740c00）：递归遍历目录，名为 `intro`（5 字符）的条目跳过，条目排序后逐项复制。
//!   "按名跳过、对文件与目录同样生效"是推断（I）：文档只写了"intro 子目录跳过"，没有说明文件。
//!
//! 未做：`load_game_session`（0x14166e7d0）本体——依赖会话资源、meta.json 写入与主题分发。
use std::fs;
use std::io;
use std::path::Path;

/// 起始存档模板目录（demo）。
pub const STARTING_BUILD_DEMO: &str = "assets/starting-builds/demo";
/// 起始存档模板目录（完整版）。
pub const STARTING_BUILD_FULL: &str = "assets/starting-builds/full";
/// 复制时跳过的条目名。
pub const SKIPPED_ENTRY: &str = "intro";

/// 按 demo 开关选择模板目录。
pub fn starting_build_dir(demo: bool) -> &'static str {
    if demo {
        STARTING_BUILD_DEMO
    } else {
        STARTING_BUILD_FULL
    }
}

/// `copy_dir_all`（0x141740c00）：递归复制 `src` 到 `dst`，跳过名为 `intro` 的条目，条目按路径排序。
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    fs::create_dir_all(dst)?;
    let mut entries = fs::read_dir(src)?.collect::<io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        if entry.file_name() == SKIPPED_ENTRY {
            continue;
        }
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn scratch(tag: &str) -> std::path::PathBuf {
        static N: AtomicU32 = AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!(
            "tg-startup-session-{tag}-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn template_directory_follows_the_demo_flag() {
        assert_eq!(starting_build_dir(true), "assets/starting-builds/demo");
        assert_eq!(starting_build_dir(false), "assets/starting-builds/full");
        assert_eq!(STARTING_BUILD_FULL.len(), 0x1b);
    }

    #[test]
    fn copies_nested_files_and_skips_intro_everywhere() {
        let root = scratch("copy");
        let src = root.join("src");
        fs::create_dir_all(src.join("a/b")).unwrap();
        fs::create_dir_all(src.join("intro")).unwrap();
        fs::create_dir_all(src.join("a/intro")).unwrap();
        fs::write(src.join("history.json"), "h").unwrap();
        fs::write(src.join("a/b/deep.txt"), "d").unwrap();
        fs::write(src.join("intro/skip.txt"), "x").unwrap();
        fs::write(src.join("a/intro/skip2.txt"), "x").unwrap();
        let dst = root.join("dst");
        copy_dir_all(&src, &dst).unwrap();
        assert_eq!(fs::read_to_string(dst.join("history.json")).unwrap(), "h");
        assert_eq!(fs::read_to_string(dst.join("a/b/deep.txt")).unwrap(), "d");
        assert!(!dst.join("intro").exists());
        assert!(!dst.join("a/intro").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn missing_source_is_an_error() {
        let root = scratch("missing");
        assert!(copy_dir_all(root.join("nope"), root.join("dst")).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}

//! `startup_session.rs` — 新会话的起始存档目录与复制（B `startup::startup_session`）。
//!
//! 证据（`startup/ALGORITHMS.md` §1.2、§2.5）：
//! - 新会话复制的模板目录：`demo` 开关（0x143634d90）为真时 `assets/starting-builds/demo`（0x142c849cb），否则
//!   `assets/starting-builds/full`（0x142c849b0，长 0x1b），两个串已逐字节核对；
//! - `copy_dir_all<PathBuf, PathBuf>`（0x141740c00）：递归遍历目录，名为 `intro`（5 字符）的条目跳过，条目排序后逐项复制。
//!   "按名跳过、对文件与目录同样生效"是推断（I）：文档只写了"intro 子目录跳过"，没有说明文件。
//!
//! `load_game_session`（0x14166e7d0）本体已落地为 13-param ECS 系统（① T2 契约
//! round4/contracts/t2-load_game_session-session-switch-CONTRACT.md）：会话切换核心
//! （continue_latest → 新建/继续 → 建目录/meta/起始 build → 载入 → 事件发射）。主题跳表、
//! attempt 精确字段、未消费 SystemParam 的面内消耗待取证，见函数 doc。
use std::fs;
use std::io;
use std::path::Path;

use crate::{
    game_profile::GameProfile,
    meta_commander::MetaCommander,
    resources::{
        glade_settings::GladeSettings,
        session::{
            ActiveSession, NewSessionStartedCmd, NextSession, SavedSessionList, SessionId,
            SessionLoadAttempt, SessionLoadedCmd, SessionMeta, SessionMetaSource,
        },
    },
    systems::{
        mode_manager::AppMode,
        save::{
            load_completion_checker::DisplayCorruptedSaveFileWarning, load_game::LoadGameManager,
            save_game::AutoSaveState,
        },
    },
};
use bevy_ecs::prelude::*;

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

/// `load_game_session`（0x14166e7d0，78KB）—— 会话切换系统（① T2 契约 §1/§2，O）。
///
/// 13-param SystemParam 面（PDB public-session-names + decomp 13 参核对，O）。每次游戏启动
/// 跑一次（`Local<bool>` 门；后续会话切换经 `NextSession` → `load_game_system`）。
///
/// 行为契约（§2，可直接落地部分）：
/// 1. 读会话尝试 `SessionLoadAttempt::continue_latest`（列表空 → 新会话，theme 默认 Autumn）。
/// 2. 新建（case2）：log "Starting a new game session" → 生成 `SessionId` → 建目录 →
///    写 meta.json（`SessionMeta::write_to_disk`，O）→ 起始 build 复制（demo/full）。
/// 3. 继续（case1）：log "Continuing game session {id}" → 就绪检查（`loading.tmp`
///    `check_load_completion` 0x140a9f810 族 + history.json 存在）→ 失败发损坏告警。
/// 4. 载入 meta.json → `ActiveSession`（失败 log "Failed to load session meta. Using
///    default. Error: " + default 回退，O）；载入 settings.json → `GladeSettings`。
/// 5. 事件发射：成功载入/新建 → `NewSessionStartedCmd` + `SessionLoadedCmd`；
///    载入失败 → `DisplayCorruptedSaveFileWarning`。
///
/// 面内消耗待取证（诚实登记，不伪造）：`GameProfile`/`AutoSaveState`/`LoadGameManager`/
/// `AppMode`/`MetaCommander` 在 78KB 内联中的精确消费；`NextSession`（原生 0xa0B 拷贝，
/// 本实现只读 kind 记日志）；`Commands`。主题跳表 0x142c84d24（case0 panic / 1/2 继续/
/// 新建）由本实现的 `SessionMetaSource` 二分表达；`SavedSessionList@+0x10` attempt
/// 字段精确语义（theme byte@+6）待取证（`SessionLoadAttempt::continue_latest` 已落地其
/// O 语义）。
#[allow(clippy::too_many_arguments)]
pub fn load_game_session(
    saved_list: Res<SavedSessionList>,
    mut active_session: Option<ResMut<ActiveSession>>,
    _game_profile: Res<GameProfile>,
    _auto_save: Res<AutoSaveState>,
    next_session: ResMut<NextSession>,
    _load_mgr: ResMut<LoadGameManager>,
    _app_mode: ResMut<AppMode>,
    mut glade_settings: ResMut<GladeSettings>,
    _meta_cmdr: ResMut<MetaCommander>,
    mut new_session_evt: EventWriter<NewSessionStartedCmd>,
    mut session_loaded: EventWriter<SessionLoadedCmd>,
    mut corrupt_warn: EventWriter<DisplayCorruptedSaveFileWarning>,
    _commands: Commands,
    mut ran: Local<bool>,
) {
    // O: 每次游戏启动只跑一次；会话切换的后续由 NextSession 链（load_game_system）处理。
    if *ran {
        return;
    }
    *ran = true;

    // 原生 0xa0B NextSession 拷贝（decomp-0x14166e7d0）；此处登记其 kind 消费（O 读面）。
    log::trace!("load_game_session: next session kind {:?}", next_session.kind);

    // 1. attempt（O：continue_latest，列表空 → 新会话，theme 默认 Autumn）。
    let attempt = SessionLoadAttempt::continue_latest(&saved_list);
    let source = if attempt.id.is_some() {
        SessionMetaSource::Existing
    } else {
        SessionMetaSource::New
    };
    let session_id = attempt.id.unwrap_or_else(SessionId::new);
    let folder = session_id.save_folder_path();

    match source {
        SessionMetaSource::New => {
            // case2（O）：'Starting a new game session' → 建目录 + meta.json + 起始 build。
            // ③ review：native 建目录/meta 写失败是 **panic**（'Creating folder for game
            // session' / 'Writing new session meta.json'）；本库以 log + corrupt_warn + return
            // 软失败适配（宿主库内合理，不崩主机）。
            log::info!("Starting a new game session");
            if let Err(error) = fs::create_dir_all(&folder) {
                log::error!("Creating folder for game session: {error}");
                corrupt_warn.write(DisplayCorruptedSaveFileWarning);
                return;
            }
            let meta = SessionMeta {
                glade: attempt.theme,
                ..Default::default()
            };
            if let Err(error) = meta.write_to_disk(session_id) {
                log::error!("Writing new session meta.json: {error}");
                corrupt_warn.write(DisplayCorruptedSaveFileWarning);
                return;
            }
            // 起始 build（O：demo 旗标 0x143634d90 宿主侧；本库默认 full，best-effort）。
            if let Err(error) = copy_dir_all(STARTING_BUILD_FULL, &folder) {
                log::warn!("Copying starting build failed: {error}");
            }
        }
        SessionMetaSource::Existing => {
            // case1（O）：'Latest game session '（continue_latest 路径）/
            // 'Continuing game session {id}' 双串——契约 §2.3；本库以 id 单串记录
            // （'Latest ' 前缀语义由 continue_latest 表达，行为等价）。
            log::info!("Continuing game session {}", session_id.0);
            // 就绪检查（O：check_load_completion 0x140a9f810 loading.tmp + history.json 存在）。
            if folder.join("loading.tmp").exists() {
                log::warn!(
                    "Session {} load is incomplete (loading.tmp present)",
                    session_id.0
                );
                corrupt_warn.write(DisplayCorruptedSaveFileWarning);
                return;
            }
            if !folder.join("history.json").exists() {
                log::warn!("Session {} is missing history.json", session_id.0);
                corrupt_warn.write(DisplayCorruptedSaveFileWarning);
                return;
            }
        }
    }

    // 4. 载入 meta.json → ActiveSession（O：失败 log + default 回退）。
    let meta = match SessionMeta::try_load_from_path(folder.join("meta.json")) {
        Ok(meta) => meta,
        Err(error) => {
            log::warn!("Failed to load session meta. Using default. Error: {error}");
            SessionMeta::default()
        }
    };
    if let Some(active) = active_session.as_mut() {
        active.id = session_id;
        active.meta = meta;
    }
    // 载入 settings.json → GladeSettings（GladeSettings::load 失败回退默认，O）。
    *glade_settings = GladeSettings::load(&folder);

    // 5. 事件发射（成功 → both；失败已在上方早退发 corrupt_warn）。
    new_session_evt.write(NewSessionStartedCmd);
    session_loaded.write(SessionLoadedCmd);
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

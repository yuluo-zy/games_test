//! `startup_session.rs` — 新会话的起始存档目录与复制（B `startup::startup_session`）。
//!
//! 证据（`startup/ALGORITHMS.md` §1.2、§2.5）：
//! - 新会话复制的模板目录：`demo` 开关（0x143634d90）为真时 `assets/starting-builds/demo`（0x142c849cb），否则
//!   `assets/starting-builds/full`（0x142c849b0，长 0x1b），两个串已逐字节核对；
//! - `copy_dir_all<PathBuf, PathBuf>`（0x141740c00）：**纯递归复制**——`create_dir_all` →
//!   `read_dir` → 目录条目递归 / 文件条目 `std::fs::copy`。fresh 反编译（2026-10-16 本批）
//!   disasm 全 CALL 清单**无排序、无 "intro" 比较**，旧实现的 sort_by_key + intro 跳过已删。
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
            SessionLoadedCmd, SessionMeta,
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

/// 按 demo 开关选择模板目录。
pub fn starting_build_dir(demo: bool) -> &'static str {
    if demo {
        STARTING_BUILD_DEMO
    } else {
        STARTING_BUILD_FULL
    }
}

/// Original startup_session @0x14166f861..0x141670239 chooses one initial
/// build directory; it never recursively copies the whole starting-build root.
/// First glade uses `intro`. Subsequent glades cycle through sorted directories
/// excluding `intro`. Selection does not create a session or mark loading done.
pub fn select_starting_build(
    demo: bool,
    new_glade_count: usize,
) -> std::io::Result<std::path::PathBuf> {
    select_starting_build_in(Path::new(starting_build_dir(demo)), new_glade_count)
}

pub fn select_starting_build_in(
    root: &Path,
    new_glade_count: usize,
) -> std::io::Result<std::path::PathBuf> {
    if new_glade_count == 0 {
        return Ok(root.join("intro"));
    }
    let mut builds = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.file_name().and_then(|name| name.to_str()) != Some("intro") {
            builds.push(path);
        }
    }
    // The original unwrap/panic boundary for an empty directory is retained:
    // returning a nonexistent or invented fallback build would hide bad assets.
    assert!(!builds.is_empty(), "No starting builds found");
    builds.sort();
    let index = (new_glade_count - 1) % builds.len();
    Ok(builds.swap_remove(index))
}

/// `copy_dir_all`（O 0x141740c00 fresh 反编译）：**纯递归复制**——`DirBuilder::create`
/// → `readdir` → 目录条目递归 `copy_dir_all` / 文件条目 `std::fs::copy`。原生**无排序、
/// 无 "intro" 跳过**（disasm 全 CALL 清单无 sort/intro 比较；旧实现 sort_by_key + intro
/// 跳过已按 fresh 反编译删去）。
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

// This corresponds to the original process-global demo byte. The host supplies
// the config-derived value before creating the async-load thread.
static STARTUP_DEMO_MODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn set_startup_demo_mode(demo: bool) {
    STARTUP_DEMO_MODE.store(demo, std::sync::atomic::Ordering::Relaxed);
}

/// B @14166e7d0: real 13-param request producer. NextSession is consumed and
/// removed, ActiveSession is inserted when absent, and the selected history
/// path is transferred into LoadGameManager. Domain application happens later.
#[allow(clippy::too_many_arguments)]
pub fn load_game_session(
    saved_list: Res<SavedSessionList>,
    mut active_session: Option<ResMut<ActiveSession>>,
    game_profile: Res<GameProfile>,
    auto_save: Res<AutoSaveState>,
    mut next_session: ResMut<NextSession>,
    mut load_mgr: ResMut<LoadGameManager>,
    mut app_mode: ResMut<AppMode>,
    mut glade_settings: ResMut<GladeSettings>,
    _meta_cmdr: ResMut<MetaCommander>,
    mut new_session_evt: EventWriter<NewSessionStartedCmd>,
    mut session_loaded: EventWriter<SessionLoadedCmd>,
    mut corrupt_warn: EventWriter<DisplayCorruptedSaveFileWarning>,
    mut commands: Commands,
) {
    use crate::resources::session::{GladeType, NextSessionKind};
    use crate::systems::save::{
        load_completion_checker::is_session_ok_to_load,
        load_game::{ActiveLoadOptions, LoadGameSource, PostGameLoadCommand},
    };
    commands.remove_resource::<NextSession>();
    // Original on_load conversion owns typed MetaCommand/PostGameLoadCommand
    // payloads. The existing raw-u64 adapter does not yet preserve all owners.
    assert!(
        next_session.on_load.is_empty(),
        "B startup_session @14166e7d0: owned on_load conversion is not restored"
    );
    let ignore_snapshot = next_session.ignore_snapshot;
    let requested = match next_session.kind {
        NextSessionKind::None => panic!("internal error: entered unreachable code"),
        NextSessionKind::LoadSavedGame => saved_list.sessions.first().map(|entry| entry.id),
        NextSessionKind::LoadOnLoad => Some(next_session.id),
    };
    let mut selected = requested;
    if let Some(id) = selected {
        if !is_session_ok_to_load(id, &auto_save.unsaved_files).is_ok() {
            corrupt_warn.write(DisplayCorruptedSaveFileWarning);
            selected = None;
        }
    }
    let demo = STARTUP_DEMO_MODE.load(std::sync::atomic::Ordering::Relaxed);
    let (session_id, history_path, is_new, snapshots_source) = match selected {
        Some(id) => {
            log::info!("Continuing game session {}", id.0);
            session_loaded.write(SessionLoadedCmd);
            (id, id.save_folder_path().join("history.json"), false, None)
        }
        None => {
            log::info!("Starting a new game session");
            let id = SessionId::new();
            fs::create_dir_all(id.save_folder_path()).expect("Creating folder for game session");
            let theme = saved_list
                .sessions
                .first()
                .and_then(|entry| {
                    SessionMeta::try_load_from_path(entry.id.save_folder_path().join("meta.json"))
                        .ok()
                })
                .map_or(GladeType::Autumn, |meta| meta.glade);
            SessionMeta {
                glade: theme,
                ..Default::default()
            }
            .write_to_disk(id)
            .expect("Writing new session meta.json");
            new_session_evt.write(NewSessionStartedCmd);
            let build = select_starting_build(demo, game_profile.new_glade_count)
                .expect("Selecting initial game state");
            let snapshots = build.join("snapshots");
            (id, build.join("history.json"), true, Some(snapshots))
        }
    };
    assert!(
        history_path.is_file(),
        "Failed to load game session or even create a new one"
    );
    if let Some(source) = snapshots_source.filter(|path| path.is_dir()) {
        // Original copy helper only copies the selected snapshot directory.
        // Its caller ignores the io result after dropping it.
        let _ = copy_dir_all(source, session_id.snapshots_folder_path());
    }
    let mut meta = SessionMeta::try_load_from_path(session_id.save_folder_path().join("meta.json"))
        .unwrap_or_else(|error| {
            log::warn!("Failed to load session meta. Using default. Error: {error}");
            SessionMeta::default()
        });
    if demo {
        meta.glade = GladeType::Summer;
    }
    // Original @1416713a8 joins assets/glade + selected theme, then calls
    // GladeSettings::load @1408bbbc0. It does not read settings from the save.
    *glade_settings = GladeSettings::load(meta.assets_root());
    if demo {
        glade_settings.default_building_color_id = 8;
        glade_settings.default_freeform_wall_color_id = 0;
        glade_settings.default_roof_color_id = 0;
        glade_settings.leaf_particle_count = 0;
    }
    let active = ActiveSession::new(session_id, meta);
    if let Some(existing) = active_session.as_mut() {
        **existing = active;
    } else {
        commands.insert_resource(active);
    }
    *app_mode = AppMode::LoadGame;
    load_mgr.begin(
        LoadGameSource::Path(history_path),
        ActiveLoadOptions {
            ignore_snapshot,
            request_reveal: is_new,
            new_session: is_new,
        },
        vec![PostGameLoadCommand::SwitchAppMode(AppMode::default())],
    );
    // The on_load Vec was asserted empty above; take still preserves ownership
    // transfer rather than leaving a duplicate request in the deferred world.
    let _ = std::mem::take(&mut next_session.on_load);
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
    fn copies_nested_files_and_directories_recursively() {
        let root = scratch("copy");
        let src = root.join("src");
        fs::create_dir_all(src.join("a/b")).unwrap();
        fs::create_dir_all(src.join("intro")).unwrap();
        fs::write(src.join("history.json"), "h").unwrap();
        fs::write(src.join("a/b/deep.txt"), "d").unwrap();
        fs::write(src.join("intro/note.txt"), "n").unwrap();
        let dst = root.join("dst");
        copy_dir_all(&src, &dst).unwrap();
        assert_eq!(fs::read_to_string(dst.join("history.json")).unwrap(), "h");
        assert_eq!(fs::read_to_string(dst.join("a/b/deep.txt")).unwrap(), "d");
        // 原生纯递归复制：不再跳过 "intro" 条目。
        assert_eq!(fs::read_to_string(dst.join("intro/note.txt")).unwrap(), "n");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn missing_source_is_an_error() {
        let root = scratch("missing");
        assert!(copy_dir_all(root.join("nope"), root.join("dst")).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}

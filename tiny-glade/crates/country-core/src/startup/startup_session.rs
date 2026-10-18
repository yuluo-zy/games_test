//! 会话启动请求：先准备磁盘与配置，再一次性发布活动会话和加载任务。
//! 初始动作与完成信号按所有权转交给 LoadGameManager，不使用裸指针回放。

use std::fs;
use std::io;
use std::path::Path;

use crate::{
    game_config::GameConfig,
    game_profile::GameProfile,
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

/// 首次选择 intro，后续按排序后的目录循环；不创建存档或提交加载状态。
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
    if builds.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "没有可用的起始场景",
        ));
    }
    builds.sort();
    let index = (new_glade_count - 1) % builds.len();
    Ok(builds.swap_remove(index))
}

/// 递归复制目录。错误直接交给调用方，避免把不完整快照发布成有效会话。
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

// 兼容尚未注入 GameConfig 的宿主；新宿主优先使用各自 World 中的 demo 配置。
static STARTUP_DEMO_MODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn set_startup_demo_mode(demo: bool) {
    STARTUP_DEMO_MODE.store(demo, std::sync::atomic::Ordering::Relaxed);
}

/// 消费一次会话请求。准备失败时保留请求，且不发布事件、活动会话或半成品任务。
#[allow(clippy::too_many_arguments)]
pub fn load_game_session(
    saved_sessions: Res<SavedSessionList>,
    mut active_session: Option<ResMut<ActiveSession>>,
    profile: Res<GameProfile>,
    auto_save: Res<AutoSaveState>,
    next_session: Option<ResMut<NextSession>>,
    mut load_manager: ResMut<LoadGameManager>,
    mut app_mode: ResMut<AppMode>,
    mut glade_settings: ResMut<GladeSettings>,
    config: Option<Res<GameConfig>>,
    mut new_session_events: EventWriter<NewSessionStartedCmd>,
    mut session_loaded_events: EventWriter<SessionLoadedCmd>,
    mut corrupted_save_events: EventWriter<DisplayCorruptedSaveFileWarning>,
    mut commands: Commands,
) -> bevy_ecs::error::Result {
    use crate::resources::session::NextSessionKind;
    use crate::systems::save::{
        load_completion_checker::is_session_ok_to_load,
        load_game::{ActiveLoadOptions, LoadGameSource},
    };
    let Some(mut request) = next_session else {
        return Ok(());
    };
    if request.kind == NextSessionKind::None {
        commands.remove_resource::<NextSession>();
        return Ok(());
    }
    // 已有请求尚未应用到世界时保留新请求，避免覆盖未完成的动作和信号。
    if load_manager.options.is_some() {
        return Ok(());
    }
    // 先完成文件和配置准备；出错时 request.on_load 仍由原请求持有。
    let requested_id = match request.kind {
        NextSessionKind::LoadSavedGame => saved_sessions.sessions.first().map(|session| session.id),
        NextSessionKind::LoadOnLoad => Some(request.id),
        NextSessionKind::None => unreachable!(),
    };
    let valid_session =
        requested_id.filter(|id| is_session_ok_to_load(*id, &auto_save.unsaved_files).is_ok());
    let demo = config.as_deref().map_or_else(
        || STARTUP_DEMO_MODE.load(std::sync::atomic::Ordering::Relaxed),
        |config| config.dev.demo,
    );
    let is_new = valid_session.is_none();
    let (session_id, history_path) = if let Some(session_id) = valid_session {
        (
            session_id,
            session_id.save_folder_path().join("history.json"),
        )
    } else {
        let build = select_starting_build(demo, profile.new_glade_count)?;
        let history_path = build.join("history.json");
        if !history_path.is_file() {
            return Err(format!("起始场景缺少历史文件：{}", history_path.display()).into());
        }
        let previous_theme = saved_sessions
            .sessions
            .first()
            .and_then(|session| {
                SessionMeta::try_load_from_path(session.id.save_folder_path().join("meta.json"))
                    .ok()
            })
            .map_or(crate::resources::session::GladeType::Autumn, |meta| {
                meta.glade
            });
        let session_id = SessionId::new();
        let meta = SessionMeta {
            glade: previous_theme,
            ..Default::default()
        };
        create_session_files(
            &session_id.save_folder_path(),
            &meta,
            &build.join("snapshots"),
        )?;
        (session_id, history_path)
    };
    if !history_path.is_file() {
        return Err(format!("会话历史文件不存在：{}", history_path.display()).into());
    }
    let mut metadata =
        SessionMeta::try_load_from_path(session_id.save_folder_path().join("meta.json"))
            .unwrap_or_else(|error| {
                log::warn!("无法读取会话元数据，使用默认设置：{error}");
                SessionMeta::default()
            });
    if demo {
        metadata.glade = crate::resources::session::GladeType::Summer;
    }
    let mut settings = GladeSettings::load(metadata.assets_root());
    if demo {
        settings.default_building_color_id = 8;
        settings.default_freeform_wall_color_id = 0;
        settings.default_roof_color_id = 0;
        settings.leaf_particle_count = 0;
    }
    let pending_actions = make_post_load_commands(std::mem::take(&mut request.on_load));
    load_manager.begin(
        LoadGameSource::Path(history_path),
        ActiveLoadOptions {
            ignore_snapshot: request.ignore_snapshot,
            request_reveal: is_new,
            new_session: is_new,
        },
        pending_actions,
    );
    *glade_settings = settings;
    let session = ActiveSession::new(session_id, metadata);
    if let Some(existing) = active_session.as_mut() {
        **existing = session;
    } else {
        commands.insert_resource(session);
    }
    *app_mode = AppMode::LoadGame;
    commands.remove_resource::<NextSession>();
    if requested_id.is_some() && valid_session.is_none() {
        corrupted_save_events.write(DisplayCorruptedSaveFileWarning);
    }
    if is_new {
        new_session_events.write(NewSessionStartedCmd);
    } else {
        session_loaded_events.write(SessionLoadedCmd);
    }
    Ok(())
}

/// 在同一存档分区的临时目录中完成元数据与快照，再用 rename 发布。
/// 复制失败时 TempDir 自动清理，新会话列表不会看到半成品目录。
fn create_session_files(
    destination: &Path,
    metadata: &SessionMeta,
    snapshots: &Path,
) -> anyhow::Result<()> {
    use anyhow::Context;
    let parent = destination.parent().context("存档目录没有父目录")?;
    fs::create_dir_all(parent)?;
    let staging = tempfile::Builder::new()
        .prefix(".new-session-")
        .tempdir_in(parent)?;
    metadata.write_to_path(staging.path().join("meta.json"))?;
    if snapshots.is_dir() {
        copy_dir_all(snapshots, staging.path().join("snapshots"))?;
    }
    fs::rename(staging.path(), destination).context("发布新会话目录")?;
    Ok(())
}

/// 按顺序移动请求中的动作。终止标记之后的动作不执行，最终模式切换始终排在末尾。
pub fn make_post_load_commands(
    entries: Vec<crate::resources::session::OnLoadEntry>,
) -> Vec<crate::systems::save::load_game::PostGameLoadCommand> {
    use crate::resources::session::{ON_LOAD_FENCE, OnLoadEntry};
    use crate::systems::save::load_game::PostGameLoadCommand;
    let mut commands = Vec::with_capacity(entries.len() + 1);
    for entry in entries {
        if matches!(
            entry,
            OnLoadEntry::Command {
                tag: ON_LOAD_FENCE,
                ..
            }
        ) {
            break;
        }
        commands.push(PostGameLoadCommand::SessionEntry(entry));
    }
    commands.push(PostGameLoadCommand::SwitchAppMode(AppMode::default()));
    commands
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

    #[test]
    fn later_builds_cycle_in_sorted_order_and_empty_roots_return_error() {
        let directory = tempfile::tempdir().unwrap();
        for name in ["z-last", "intro", "a-first"] {
            fs::create_dir(directory.path().join(name)).unwrap();
        }
        assert!(
            select_starting_build_in(directory.path(), 0)
                .unwrap()
                .ends_with("intro")
        );
        for (count, expected) in [(1, "a-first"), (2, "z-last"), (3, "a-first")] {
            assert!(
                select_starting_build_in(directory.path(), count)
                    .unwrap()
                    .ends_with(expected)
            );
        }
        let empty = tempfile::tempdir().unwrap();
        assert_eq!(
            select_starting_build_in(empty.path(), 1)
                .unwrap_err()
                .kind(),
            io::ErrorKind::NotFound
        );
    }

    #[test]
    fn staged_session_publishes_metadata_and_all_snapshots() {
        let directory = tempfile::tempdir().unwrap();
        let snapshots = directory.path().join("source");
        fs::create_dir(&snapshots).unwrap();
        fs::write(snapshots.join("sample.json"), b"snapshot").unwrap();
        let destination = directory.path().join("sessions/new");
        let metadata = SessionMeta::default();
        create_session_files(&destination, &metadata, &snapshots).unwrap();
        assert_eq!(
            SessionMeta::try_load_from_path(destination.join("meta.json")).unwrap(),
            metadata
        );
        assert_eq!(
            fs::read(destination.join("snapshots/sample.json")).unwrap(),
            b"snapshot"
        );
    }

    #[cfg(unix)]
    #[test]
    fn failed_snapshot_copy_leaves_no_half_created_session() {
        let directory = tempfile::tempdir().unwrap();
        let snapshots = directory.path().join("source");
        fs::create_dir(&snapshots).unwrap();
        std::os::unix::fs::symlink(directory.path().join("missing"), snapshots.join("bad-link"))
            .unwrap();
        let destination = directory.path().join("sessions/new");
        assert!(create_session_files(&destination, &SessionMeta::default(), &snapshots).is_err());
        assert!(!destination.exists());
        assert_eq!(
            fs::read_dir(destination.parent().unwrap()).unwrap().count(),
            0
        );
    }
}

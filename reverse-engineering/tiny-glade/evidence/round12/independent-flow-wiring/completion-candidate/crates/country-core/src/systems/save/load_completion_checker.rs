//! Original B completion observer140a9f810: consume LoadingGameComplete,
//! retain its sticky Local bit, and unlink loading.tmp once mode is neither
//! replay13 nor loading14. All unlink errors are dropped; mode_manager owns
//! the separate restore transition. The older pure filesystem-readiness helper
//! remains a compatibility API and must not be mistaken for this ECS consumer.
use crate::{resources::session::SessionId, systems::save::save_thread::UnsavedFileList};
use bevy_ecs::prelude::Resource;
use std::path::Path;

/// 会话可载入就绪度。原字节 tag：`Ok=0`、`DenyLoad=1`、`NotReady=2`（I；字段序 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum SessionLoadReadiness {
    #[default]
    Ok = 0,
    DenyLoad = 1,
    NotReady = 2,
}
impl SessionLoadReadiness {
    /// 0x140833230：`[rcx+0x10]==0` 折叠为 tag==0。
    pub fn is_ok(self) -> bool {
        self == Self::Ok
    }
    /// 0x140a9fa70：tag==1（与 `GladeType::is_olden` 同址折叠）。
    pub fn deny_load(self) -> bool {
        self == Self::DenyLoad
    }
}

/// `check_load_completion` 0x140a9f810：`loading.tmp` 仍存在 → 加载尚未完成，拒绝读入。
/// 路径根为全局 saves 目录；提供按目录的变体便于隔离测试。
pub fn check_load_completion(session: SessionId) -> SessionLoadReadiness {
    check_load_completion_in(&session.save_folder_path())
}

/// 按会话目录判定的 `check_load_completion`（O：save_folder + join "loading.tmp"）。
pub fn check_load_completion_in(session_folder: &Path) -> SessionLoadReadiness {
    if session_folder.join("loading.tmp").exists() {
        SessionLoadReadiness::DenyLoad
    } else {
        SessionLoadReadiness::Ok
    }
}

/// `is_session_ok_to_load` 0x140a9fa80：会话文件就绪检查。
///
/// **三值编码（O：fresh 反编译）**：unsaved 命中 `history.json` → **1**；`loading.tmp`
/// 缺失 → **0**；`loading.tmp` 存在 → **2**。旧实现把 unsaved 命中折叠为 `Ok(0)`，
/// 编码与原生不符（unsaved 命中是"有挂起保存可续"的**不同就绪值**）。
pub fn is_session_ok_to_load(
    session: SessionId,
    unsaved: &UnsavedFileList,
) -> SessionLoadReadiness {
    is_session_ok_to_load_in(&session.save_folder_path(), unsaved)
}

/// 按会话目录判定的 `is_session_ok_to_load`（三值编码，见上）。
pub fn is_session_ok_to_load_in(
    session_folder: &Path,
    unsaved: &UnsavedFileList,
) -> SessionLoadReadiness {
    let history = session_folder.join("history.json");
    if unsaved.contains_file(&history) {
        // 原生值 1（"unsaved 挂起可续"；与 Ok=0 不同编码）。
        return SessionLoadReadiness::DenyLoad;
    }
    if session_folder.join("loading.tmp").exists() {
        SessionLoadReadiness::NotReady
    } else {
        SessionLoadReadiness::Ok
    }
}

/// 原系统参数面里的损坏存档告警事件（PDB：`DisplayCorruptedSaveFileWarning`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DisplayCorruptedSaveFileWarning;

/// 原读档完成状态资源（PDB：`LoadingCompletionState`）。字段序 U，仅登记资源合同。
#[derive(Resource, Clone, Debug, Default)]
pub struct LoadingCompletionState {
    /// Canonical original Local bit: once a LoadingGameComplete was observed,
    /// later nonloading/nonreplay modes retry marker deletion every update.
    observed_complete: bool,
    /// Historical global readiness projection retained for the older tree
    /// adapter; it is not read by the original completion observer.
    /// 最近一次就绪度判定；游戏循环消费。
    pub readiness: SessionLoadReadiness,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readiness_tag_predicates() {
        assert!(SessionLoadReadiness::Ok.is_ok());
        assert!(!SessionLoadReadiness::DenyLoad.is_ok());
        assert!(SessionLoadReadiness::DenyLoad.deny_load());
        assert!(!SessionLoadReadiness::Ok.deny_load());
        assert_eq!(SessionLoadReadiness::default(), SessionLoadReadiness::Ok);
    }

    #[test]
    fn completion_checks_probe_the_session_folder() {
        use crate::systems::save::save_thread::UnsavedFileList;
        let root = tempfile::tempdir().unwrap();
        let folder = root.path().join("session");
        assert_eq!(check_load_completion_in(&folder), SessionLoadReadiness::Ok);
        std::fs::create_dir(&folder).unwrap();
        std::fs::write(folder.join("loading.tmp"), b"").unwrap();
        assert_eq!(
            check_load_completion_in(&folder),
            SessionLoadReadiness::DenyLoad
        );
        let unsaved = UnsavedFileList::default();
        assert_eq!(
            is_session_ok_to_load_in(&folder, &unsaved),
            SessionLoadReadiness::NotReady
        );
        std::fs::remove_file(folder.join("loading.tmp")).unwrap();
        assert_eq!(
            is_session_ok_to_load_in(&folder, &unsaved),
            SessionLoadReadiness::Ok
        );
        unsaved.add_path(folder.join("history.json"));
        // 三值编码：unsaved 命中 → 原生值 1（DenyLoad），非 Ok(0)。
        assert_eq!(
            is_session_ok_to_load_in(&folder, &unsaved),
            SessionLoadReadiness::DenyLoad
        );
    }
}

/// Original B140a9f810/608B, mounted in host Update set36.
/// Mode restoration belongs to mode_manager, not this marker observer. The
/// native unlink result is dropped for every error kind and the Local bit is
/// deliberately sticky, including across a later session.
pub fn check_load_completion_system(
    mode: bevy_ecs::prelude::Res<crate::systems::mode_manager::AppMode>,
    mut complete: bevy_ecs::prelude::EventReader<super::load_game::LoadingGameComplete>,
    mut local: bevy_ecs::prelude::Local<LoadingCompletionState>,
    session: bevy_ecs::prelude::Res<crate::resources::session::ActiveSession>,
) {
    if complete.read().last().is_some() {
        local.observed_complete = true;
    }
    if !local.observed_complete || matches!(mode.native_tag(), 13 | 14) {
        return;
    }
    let _ = std::fs::remove_file(session.id.save_folder_path().join("loading.tmp"));
}

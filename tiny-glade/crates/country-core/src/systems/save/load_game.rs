//! B `systems/save/load_game.rs` — 读档主系统。
//!
//! `load_game` 0x14146d030（13+ 段 SystemParam）：状态机（`LoadGameState::LoadGameData`）
//! → 写 `loading.tmp` → NextSessionKind 解码（1=parse_saved_game_data 含 upgrade 备份 /
//! 2=on_load 命令回放+内存快照）→ PostGameLoadCommand 回放（SwitchAppMode 忽略）→
//! 版本全局开关（centered_path = v>=0x1a；stairs checksum = v>0x1a）→ 旧版颜色 ID 升级
//! → `extract_max_wall_id_from_history` → `init_game_state_from_snapshot`。
//!
//! 2026-10-15 会话切换链（RE-SESSION-SWITCH）：commander_update case 0 经
//! `insert_resource<NextSession>` 触发 load_game；on_load 列表元素 0x20B {tag, payload}；
//! 两遍回放终止哨兵 = tag 0x8000000000000003（fence 是变体 0 载荷，非终止）；fence 释放
//! 走变体 2（unblock +0x10 + Arc drop）。MetaCommand 应用（commander_update case 0）与
//! `init_game_state_from_snapshot` 依赖游戏资源，由宿主/域恢复后接入；不伪造加载成功。
use crate::{
    meta_commander::MetaCommand,
    recovered_building::wall_history_edit::{WallCompoundHistoryEdit, WallHistoryEdit},
    resources::save_file::{
        LoadUpgradeFlags, LoadVersionGates, arrow_slit_record_applies, upgrade_arrow_slit_color_id,
        upgrade_window_record_color,
    },
    resources::session::{NextSession, OnLoadEntry, SessionId, ON_LOAD_FENCE, ON_LOAD_VARIANT_MASK},
};
use bevy_ecs::prelude::Resource;
use serde_json::Value;
use std::path::Path;

/// 读档状态（`LoadGameManager` 状态字节 +9）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum LoadGameState {
    /// 状态 0：发加载进度记录后置 2（多帧节拍）。
    #[default]
    Loading = 0,
    /// 状态 2：主流程。
    Loaded = 2,
}

/// 读档管理器资源（PDB：`load_game::LoadGameManager`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct LoadGameManager {
    pub state: LoadGameState,
    /// +0x4c 加载进度计数（可负）。
    pub progress_counter: i32,
    /// 已写入 `loading.tmp` 标记。
    pub wrote_loading_marker: bool,
}

/// 读档来源（PDB：`load_game::LoadGameSource`）。变体语义 I。
/// 由 load_game 系统消费；纯核心暂以 `NextSession` 的 id/kind 驱动，故标记待接。
#[allow(dead_code)] // Recovered contract awaiting the full load_game SystemParam wiring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoadGameSource {
    #[default]
    NewSession,
    ExistingSession(SessionId),
}

/// 解析后的存档数据（PDB：`load_game::ParsedGameData`）。
#[derive(Clone, Debug, Default)]
pub struct ParsedGameData {
    pub version: i32,
    pub backward_compatibility: Option<Value>,
    pub camera: Option<Value>,
    pub time_of_day: Option<Value>,
}

/// 读档后命令（PDB：`load_game::PostGameLoadCommand`）。
/// 变体 0 = SwitchAppMode（load_game 回放时忽略并 log）。第二遍回放已输出
/// `ReplayedOnLoadCommand::Apply`/`SwitchAppMode`；此枚举登记原 PDB 合同。
#[allow(dead_code)] // Recovered contract type; replayed commands surface as ReplayedOnLoadCommand.
#[derive(Clone, Debug)]
pub enum PostGameLoadCommand {
    SwitchAppMode,
    /// 变体 1：{tag, 载荷×2}；域载荷由所属模块提供。
    Apply { tag: u64, payload_a: Value, payload_b: Value },
    /// 变体 2+：Arc 负载 drop（I）。
    DropArc(Value),
}

/// `LoadingGameComplete` 事件（PDB：`load_game::LoadingGameComplete`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadingGameComplete;

/// `StartupRevealStarted` / `StartupRevealFinished` 事件（PDB 同名）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StartupRevealStarted;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StartupRevealFinished;

/// `parse_saved_game_data`（内联于 load_game 行 ~413，0x141470d00）：
/// 读档并解析节序 `Version → BackwardCompatibility → Camera → Time of day`；
/// 解析前先做 upgrade 备份。失败返回 "Couldn't load save file from " 语义错误。
pub fn parse_saved_game_data(path: &Path) -> anyhow::Result<ParsedGameData> {
    log::debug!("Making save file backup for upgrade");
    let _ = make_upgrade_backup(path);
    log::debug!("Reading file data");
    let bytes = std::fs::read(path).map_err(|error| {
        anyhow::anyhow!("Couldn't load save file from {}: {error}", path.display())
    })?;
    log::debug!("Parsing JSON");
    let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
        anyhow::anyhow!("Couldn't load save file from {}: {error}", path.display())
    })?;
    let object = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Save file root is not an object"))?;
    log::debug!("Version");
    let version = object
        .get("Version")
        .and_then(Value::as_i64)
        .unwrap_or_default() as i32;
    let backward_compatibility = object.get("BackwardCompatibility").cloned();
    let camera = object.get("Camera").cloned();
    log::debug!("parsing camera");
    let time_of_day = object.get("Time of day").cloned().or_else(|| {
        log::warn!("This save file had no 'Time of day' data");
        None
    });
    Ok(ParsedGameData {
        version,
        backward_compatibility,
        camera,
        time_of_day,
    })
}

/// upgrade 前存档备份（O：parse_saved_game_data 内 "Making save file backup for
/// upgrade"）。把源文件复制到 `history-recovered-<时间戳>.json`；失败仅告警不中止解析。
fn make_upgrade_backup(path: &Path) -> anyhow::Result<()> {
    let backup_folder = path
        .parent()
        .map(|parent| parent.join("..").join("backup-saves"))
        .unwrap_or_else(|| std::env::temp_dir());
    let mut file = std::fs::File::open(path)?;
    let modified = file
        .metadata()
        .and_then(|metadata| metadata.modified())
        .unwrap_or(std::time::UNIX_EPOCH);
    let stamp = modified
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let target = backup_folder.join(format!("history-recovered-{stamp}.json"));
    std::fs::create_dir_all(&target.parent().expect("backup target has a parent"))?;
    use std::io::Write;
    let mut output = std::fs::File::create(&target)?;
    std::io::copy(&mut file, &mut output)?;
    output.flush()?;
    Ok(())
}

/// 版本 → 全局开关（O：0x14146dc82..0x14146dc95，有符号比较）。复用
/// `resources::save_file::LoadVersionGates`。
pub fn version_gates(version: i32) -> LoadVersionGates {
    LoadVersionGates::from_version(version)
}

/// 旧版窗口颜色升级（O：位 48，"Detected old version of window color IDs..."）。
/// 子类型 == 0x3a 的记录颜色改 999。
pub fn upgrade_window_color(subtype: u8, color: &mut u32) {
    upgrade_window_record_color(subtype, color);
}

/// 旧版箭孔颜色升级（O：位 40，"Detected old version of arrow slits color IDs..."）。
/// 子类型 ∈ 0x38..=0x3a 的记录颜色按映射表改写。
pub fn upgrade_arrow_slit_color(subtype: u8, color: &mut u32) {
    if arrow_slit_record_applies(subtype) {
        *color = upgrade_arrow_slit_color_id(*color);
    }
}

/// `extract_max_wall_id_from_history`（内联，行 899）：遍历 MultiEdit 取最大 wall id。
/// 原版写计数器资源 +0xc0；本实现返回最大值，由调用方写入其计数器资源。
pub fn extract_max_wall_id_from_history(
    edits: impl IntoIterator<Item = WallCompoundHistoryEdit>,
) -> Option<u64> {
    let mut max = None;
    for edit in edits {
        for group in &edit.groups {
            for entry in group {
                if let Some(id) = wall_id_of(entry) {
                    max = Some(max.map_or(id, |m: u64| m.max(id)));
                }
            }
        }
    }
    max
}

/// 提取单条墙编辑的 wall id；非墙变体返回 None。
fn wall_id_of(edit: &WallHistoryEdit) -> Option<u64> {
    match edit {
        WallHistoryEdit::Create { wall, .. }
        | WallHistoryEdit::Delete { wall, .. }
        | WallHistoryEdit::EnclosureChange { wall, .. }
        | WallHistoryEdit::StateChange { wall, .. } => Some(wall.0),
    }
}

/// `load_game` 主系统的错误面：存档解析失败。
#[derive(Debug)]
pub enum LoadGameError {
    /// 存档解析失败。
    Io(anyhow::Error),
}
impl From<anyhow::Error> for LoadGameError {
    fn from(error: anyhow::Error) -> Self {
        Self::Io(error)
    }
}

/// on_load 两遍回放输出命令。
#[derive(Clone, Debug)]
pub enum ReplayedOnLoadCommand {
    /// 第一遍变体 0：写 manager 状态 + 24B 事件载荷。
    WriteManager { payload: u64 },
    /// 第一遍变体 1：MetaCommand（0x38B = 7 qword）。应用由 meta_commander.rs
    /// 的 commander_update 消费；此处登记命令本身，不伪造应用。
    MetaCommand(MetaCommand),
    /// 第一遍变体 2：fence 释放（unblock + Arc drop；RE-SESSION-SWITCH 定案，
    /// fence 标志 +0x10）。
    FenceRelease { fence_payload: u64 },
    /// 第二遍变体 0：PostGameLoadCommand::SwitchAppMode（原版忽略并 log）。
    SwitchAppMode,
    /// 第二遍变体 1/2+：PostGameLoadCommand 载荷 {tag, 载荷×2}。
    Apply { tag: u64, payload_a: u64, payload_b: u64 },
}

/// 第一遍回放：MetaCommand（0x14146d030 行 ~390 区）。**终止哨兵 =
/// 0x8000000000000003（tag）**（RE-SESSION-SWITCH 定案；fence 是变体 0 载荷，非终止）。
/// 变体解码 `tag ^ 0x8000000000000000`，`min(·,2)`：0=manager 状态、1=MetaCommand、
/// 2=fence 释放（unblock + Arc drop）。批量条目（tag=3）展开为 MetaCommand。
fn replay_meta_commands(
    manager: &mut LoadGameManager,
    on_load: &[OnLoadEntry],
    out: &mut Vec<ReplayedOnLoadCommand>,
) {
    for entry in on_load {
        match entry {
            OnLoadEntry::Command { tag, payload } => {
                if *tag == ON_LOAD_FENCE {
                    break;
                }
                match (tag ^ ON_LOAD_VARIANT_MASK).min(2) {
                    0 => {
                        manager.progress_counter = manager.progress_counter.wrapping_add(1);
                        out.push(ReplayedOnLoadCommand::WriteManager { payload: *payload });
                    }
                    1 => {
                        out.push(ReplayedOnLoadCommand::MetaCommand(MetaCommand {
                            tag: *tag,
                            payloads: [*payload, 0, 0, 0, 0, 0],
                        }));
                    }
                    2 => {
                        out.push(ReplayedOnLoadCommand::FenceRelease {
                            fence_payload: *payload,
                        });
                    }
                    _ => {}
                }
            }
            OnLoadEntry::Batch { commands } => {
                for command in commands {
                    out.push(ReplayedOnLoadCommand::MetaCommand(*command));
                }
            }
        }
    }
}

/// 第二遍回放：PostGameLoadCommand。变体 0（SwitchAppMode）忽略并 log；变体 1 push
/// 命令；变体 2+ push `{0x8000000000000000, elem[+8], ...}`；**终止哨兵 =
/// 0x8000000000000003（tag）**。批量条目展开为 Apply。
fn replay_post_game_load_commands(on_load: &[OnLoadEntry], out: &mut Vec<ReplayedOnLoadCommand>) {
    for entry in on_load {
        match entry {
            OnLoadEntry::Command { tag, payload } => {
                if *tag == ON_LOAD_FENCE {
                    break;
                }
                match tag ^ ON_LOAD_VARIANT_MASK {
                    0 => {
                        log::info!("Ignoring PostGameLoadCommand::SwitchAppMode");
                        out.push(ReplayedOnLoadCommand::SwitchAppMode);
                    }
                    1 => {
                        out.push(ReplayedOnLoadCommand::Apply {
                            tag: *tag,
                            payload_a: *payload,
                            payload_b: 0,
                        });
                    }
                    2 | _ => {
                        out.push(ReplayedOnLoadCommand::Apply {
                            tag: ON_LOAD_VARIANT_MASK,
                            payload_a: *payload,
                            payload_b: 0,
                        });
                    }
                }
            }
            OnLoadEntry::Batch { commands } => {
                for command in commands {
                    out.push(ReplayedOnLoadCommand::Apply {
                        tag: command.tag,
                        payload_a: command.payloads[0],
                        payload_b: command.payloads[1],
                    });
                }
            }
        }
    }
}

/// `load_game` 主系统纯核心：状态推进 + kind 解码 + on_load 两遍回放。
///
/// - 门控 `app_mode == LoadGameData` 由调用方执行（Bevy 系统参数未齐）。
/// - kind 解码：`decoded = kind ^ 0x8000000000000000; sel = min(decoded, 2)`。
///   sel==1 → parse_saved_game_data（含 upgrade 备份）；sel==2 → on_load 两遍回放
///   （MetaCommand → PostGameLoadCommand，fence 终止）。
pub fn advance_load_game(
    manager: &mut LoadGameManager,
    next: &NextSession,
    save_path: Option<&Path>,
) -> Result<(Option<ParsedGameData>, Vec<ReplayedOnLoadCommand>), LoadGameError> {
    match manager.state {
        LoadGameState::Loading => {
            manager.progress_counter = manager.progress_counter.wrapping_add(1);
            manager.state = LoadGameState::Loaded;
            Ok((None, Vec::new()))
        }
        LoadGameState::Loaded => {
            let selected = next.kind.decoded().min(2);
            match selected {
                0 => {
                    // 无请求（load_latest 默认 kind=0x8000000000000002 → sel==2）。
                    Ok((None, Vec::new()))
                }
                1 => {
                    let path = save_path
                        .ok_or_else(|| anyhow::anyhow!("parse_saved_game_data needs a path"))?;
                    let parsed = parse_saved_game_data(path)?;
                    manager.progress_counter = manager.progress_counter.wrapping_add(1);
                    Ok((Some(parsed), Vec::new()))
                }
                2 => {
                    let mut commands = Vec::new();
                    replay_meta_commands(manager, &next.on_load, &mut commands);
                    replay_post_game_load_commands(&next.on_load, &mut commands);
                    Ok((None, commands))
                }
                _ => unreachable!(),
            }
        }
    }
}

/// `load_game_session`：完整会话切换纯核心（0x14146d030 主流程的可接部分）。
///
/// 状态机推进 → kind 解码 → 两遍 on_load 回放（fence 终止）→ 版本开关 → 颜色 ID 升级
/// → max wall id 恢复。`MetaCommand` 应用（commander_update case 0）与
/// `init_game_state_from_snapshot` 依赖游戏资源，由宿主/域恢复后接入；不伪造加载成功。
#[allow(clippy::too_many_arguments)]
pub fn load_game_session(
    manager: &mut LoadGameManager,
    next: &NextSession,
    save_path: Option<&Path>,
    edits: &[WallCompoundHistoryEdit],
    upgrade_flags: crate::resources::save_file::LoadUpgradeFlags,
) -> Result<
    (
        Option<ParsedGameData>,
        Vec<ReplayedOnLoadCommand>,
        LoadVersionGates,
        Option<u64>,
    ),
    LoadGameError,
> {
    // 1. 状态推进 + kind 解码 + 两遍回放。
    let (parsed, commands) = advance_load_game(manager, next, save_path)?;
    // 2. 版本 → 全局开关（O：0x14146dc82..0x14146dc95）。
    let version = parsed.as_ref().map_or(0, |data| data.version);
    let gates = version_gates(version);
    // 3. 颜色 ID 升级（位 48 window / 位 40 arrow slits）——作用于 MultiEdit 记录；
    //    纯函数入口见 `upgrade_window_color`/`upgrade_arrow_slit_color`（按记录遍历由
    //    MultiEdit 恢复后接入）。此处仅登记标志消费。
    let _ = upgrade_flags;
    // 4. max wall id 恢复（O：extract_max_wall_id_from_history）。
    let max_wall_id = extract_max_wall_id_from_history(edits.iter().cloned());
    Ok((parsed, commands, gates, max_wall_id))
}

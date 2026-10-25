//! B `systems/save/load_game.rs` — 读档主系统。
//!
//! `load_game` 0x14146d030（13+ 段 SystemParam）：状态机（`LoadGameState::LoadGameData`）
//! → 写 `loading.tmp` → NextSessionKind 解码（1=parse_saved_game_data（解析后 load_game
//! 做 upgrade 备份）/ 2=on_load 命令回放+内存快照）→ PostGameLoadCommand 回放
//! （SwitchAppMode 忽略）→ 版本全局开关（centered_path = v>=0x1a；stairs checksum =
//! v>0x1a）→ 旧版颜色 ID 升级 → `extract_max_wall_id_from_history` →
//! `init_game_state_from_snapshot`。
//!
//! 2026-10-15 会话切换链（RE-SESSION-SWITCH）：commander_update case 0 经
//! `insert_resource<NextSession>` 触发 load_game；on_load 列表元素 0x20B {tag, payload}；
//! 两遍回放终止哨兵 = tag 0x8000000000000003（fence 是变体 0 载荷，非终止）；fence 释放
//! 走变体 2（unblock +0x10 + Arc drop）。MetaCommand 应用（commander_update case 0）由
//! `init_game_state_from_snapshot` 之外的历史链所有者处理。
//!
//! 快照读档（Round9 G1）：`try_load_game_system` 在解析存档后按最近记录打开
//! `{snapshot_id}.snapshot`；打开成功 → `snapshot::load_snapshot_file`（motomoto
//! chunks → Snapshot JSON + 命名栅格/Terrain chunk）→ 基础会话状态应用 →
//! `snapshot::init_game_state_from_snapshot`（World 级域写回）。打不开的记录按原版
//! 告警后尝试下一条；无可用快照 → 原版 @14146dfa6 的「历史链 cursor-1」分支。
use crate::meta_commander::{MetaCommand, MetaCommander};
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::history::wall_edit::WallHistoryEdit;
use crate::resources::save_file::LoadVersionGates;
use crate::resources::save_file::upgrade_arrow_slit_record_color;
use crate::resources::save_file::upgrade_window_color_id;
use crate::resources::save_file::window_record_applies;
use crate::resources::session::NextSession;
use crate::resources::session::ON_LOAD_FENCE;
use crate::resources::session::ON_LOAD_VARIANT_MASK;
use crate::resources::session::OnLoadEntry;
use crate::resources::session::SessionId;
use crate::systems::mode_manager::AppMode;
use anyhow::Context as _;
use bevy_ecs::prelude::Resource;
use serde_json::Value;
use std::path::Path;

#[path = "load_game_migration.rs"]
mod migration;

/// B state byte at +0x48. The old `+9` interpretation confused a
/// qword index with a byte offset. Constructor @140944bc6 stores 2.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum LoadGameState {
    Loading = 0,
    /// After real snapshot/domain application: emit the signed counter delta.
    ApplyCounter = 1,
    /// Idle/post-command phase. Kept named Loaded for existing pure API callers.
    #[default]
    Loaded = 2,
    /// 失败终态（I：原生 state 字节只证 0/1/2；这里用 3 明确“本次请求已失败、
    /// 不得重试/不得当完成”）。进入后 `try_load_game_system` 只返回 Ok 不再重复上报。
    Failed = 3,
}

/// B source discriminant at +0x18, taken by load_game @14146d030.
/// This is distinct from NextSessionKind, consumed by startup_session earlier.
#[derive(Clone, Debug, Default)]
pub enum LoadGameSource {
    #[default]
    None,
    /// Native niche 0x8000000000000001; owns the selected history.json path.
    Path(std::path::PathBuf),
    /// The original other active variant owns pre-parsed game data.
    Parsed {
        data: Box<ParsedGameData>,
        /// Native active parsed-source also owns the actual snapshot base.
        snapshot_root: std::path::PathBuf,
    },
}

/// Active request flags are initialized by startup_session, not by the idle
/// constructor. Option preserves that validity boundary in safe Rust.
#[derive(Clone, Copy, Debug)]
pub struct ActiveLoadOptions {
    pub ignore_snapshot: bool,
    pub request_reveal: bool,
    pub new_session: bool,
}

#[derive(Resource, Clone, Debug)]
pub struct LoadGameManager {
    pub state: LoadGameState,
    pub progress_counter: i32,
    pub wrote_loading_marker: bool,
    pub source: LoadGameSource,
    pub options: Option<ActiveLoadOptions>,
    pub post_load_commands: Vec<PostGameLoadCommand>,
    /// 已成功执行的命令数；失败重试时不重复执行前面的副作用。
    post_load_cursor: usize,
    /// Original active initializer writes +0x50=0. No later meaning is claimed.
    pub selection_word_50: Option<u64>,
}
impl Default for LoadGameManager {
    fn default() -> Self {
        Self::new()
    }
}
impl LoadGameManager {
    /// Original resource constructor @140944b6c..bfc: one owned post-load
    /// SwitchAppMode(Editing(None)), no source, state2. No save-thread ownership.
    pub fn new() -> Self {
        Self {
            state: LoadGameState::Loaded,
            progress_counter: 0,
            wrote_loading_marker: false,
            source: LoadGameSource::None,
            options: None,
            post_load_commands: vec![PostGameLoadCommand::SwitchAppMode(AppMode::default())],
            post_load_cursor: 0,
            selection_word_50: None,
        }
    }

    /// Actual startup producer replaces all request owners before state0.
    pub fn begin(
        &mut self,
        source: LoadGameSource,
        options: ActiveLoadOptions,
        post_load_commands: Vec<PostGameLoadCommand>,
    ) {
        *self = Self {
            state: LoadGameState::Loading,
            progress_counter: 0,
            wrote_loading_marker: false,
            source,
            options: Some(options),
            post_load_commands,
            post_load_cursor: 0,
            selection_word_50: Some(0),
        };
    }

    /// 世界状态实际应用成功后调用。失败保留当前命令，重试从失败处继续。
    /// 域命令由宿主执行；这里只处理已知的命令队列、模式和安全信号。
    pub fn finish_after_world_applied(
        &mut self,
        commander: &mut MetaCommander,
        app_mode: &mut AppMode,
        mut apply_domain: impl FnMut(&PostGameLoadCommand) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(self.options.is_some(), "没有正在加载的会话");
        while let Some(command) = self.post_load_commands.get(self.post_load_cursor) {
            match command {
                PostGameLoadCommand::SwitchAppMode(_) => {}
                PostGameLoadCommand::SessionEntry(OnLoadEntry::Batch { commands }) => {
                    commander.extend_script(commands.iter().copied());
                }
                PostGameLoadCommand::SessionEntry(OnLoadEntry::Fence(_)) => {}
                PostGameLoadCommand::SessionEntry(OnLoadEntry::Command { tag, .. })
                    if *tag == ON_LOAD_FENCE => {}
                other => apply_domain(other)?,
            }
            self.post_load_cursor += 1;
        }
        // 所有可能失败的动作都已完成，才切换模式和解除等待，避免提前宣告完成。
        for command in &self.post_load_commands {
            match command {
                PostGameLoadCommand::SwitchAppMode(mode) => *app_mode = *mode,
                PostGameLoadCommand::SessionEntry(OnLoadEntry::Fence(fence)) => fence.unblock(),
                _ => {}
            }
        }
        self.post_load_commands.clear();
        self.post_load_cursor = 0;
        self.state = LoadGameState::Loaded;
        self.source = LoadGameSource::None;
        self.options = None;
        Ok(())
    }
}

/// 解析后的存档数据（PDB：`load_game::ParsedGameData`）。
///
/// fresh 反编译 0x141470d00 的节解析面（2026-10-17；键串经 `read_memory`
/// @0x142c644a0..0x142c64a10 核对）：`Version`（typed i32；解析失败默认 0x19）→
/// `Camera`（`CameraRigParametersCache`，失败 context **"parsing camera"** 致命）→
/// The native successful return owns a 240B body plus these four flags.
/// Its other four-byte field is the target chain_index, already stored above.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParsedLoadMetadata {
    /// Native wrapper+c: parsed source version!=27 OR an actual migration ran.
    pub requires_upgrade_backup: bool,
    /// Native wrapper+d: WINDOW snapshot colors, then invalidate snapshot refs.
    pub upgrade_window_colors: bool,
    /// Native wrapper+e: ARROW SLIT snapshot colors. Not the previous swapped bit.
    pub upgrade_arrow_slit_colors: bool,
    /// Native wrapper+f: source version<19 requests SpawnPreplacedTrees.
    /// The historical field name is retained for owning-request callers;
    /// it is not DisplayCorruptedSaveFileWarning. Err uses niche value2.
    pub legacy_version_notice: bool,
}
impl ParsedLoadMetadata {
    /// Proven native v27 branch @141471bfe..1c23 / @141472e76..eb3.
    pub const VERSION_27: Self = Self {
        requires_upgrade_backup: false,
        upgrade_window_colors: false,
        upgrade_arrow_slit_colors: false,
        legacy_version_notice: false,
    };

    pub fn snapshot_color_upgrade_flags(self) -> crate::resources::save_file::LoadUpgradeFlags {
        crate::resources::save_file::LoadUpgradeFlags(
            ((self.upgrade_window_colors as u64) << 40)
                | ((self.upgrade_arrow_slit_colors as u64) << 48),
        )
    }
}

/// `Time of day`（typed 3×f32；失败回退 legacy f32；再失败告警
/// **"This save file had no `Time of day` data"** 并用默认）→ `Sheep`
/// （`SheepControlSnapshot`；失败日志 **"Error parrsing sheep snapshot: {}"** [原文拼写]）
/// → `Colors`（`GladeColorCustomization`；成功 "Loading existing color customization."，
/// 失败 "Initializing new color customization." + default）→ `History`（4 字段：
/// chain_index/history_start/snapshots/edit_chain；context **"parsing history"**）→
/// `Identity`（`HistoryIdentity`；失败 "Couldn't read save identity: {}." →
/// `derive_fallback_save_file_identity` + "Derived save identity from file meta: {}"）
/// → `BackwardCompatibility`。
#[derive(Clone, Debug, Default)]
pub struct ParsedGameData {
    /// All four original wrapper flags, carried through the owning request.
    /// A missing value is only possible in hand-built inputs; the parser
    /// always produces all four flags, including compatibility migrations.
    pub migration_metadata: Option<ParsedLoadMetadata>,
    pub version: i32,
    pub backward_compatibility: Option<Value>,
    pub camera: Option<crate::systems::camera_rig::CameraRigParametersCache>,
    pub time_of_day: TimeOfDaySection,
    pub sheep: Option<crate::resources::critters::sheep::control::SheepControlSnapshot>,
    pub colors: crate::resources::glade_color_customization::GladeColorCustomization,
    pub identity: Option<crate::resources::history::HistoryIdentity>,
    pub chain_index: i32,
    /// Native +6c undo boundary; serde visit_object @1415bd861 reads i32,
    /// and the omitted-field branch @1415bd97d supplies -1.
    pub history_start: i32,
    /// Saved MultiEdits decoded into the same owners used by live replay.
    pub edits: Vec<
        crate::resources::history::transactions::HistoryEntry<crate::resources::history::EditType>,
    >,
    /// `History.snapshots`（`HistoryChainSnapshot` JSON 镜像）。
    pub snapshots: Vec<Value>,
}

/// `Time of day` 节 typed 镜像（O：`to_json_string<TimeOfDay>` 0x140b389b0 只写三个
/// f32：`time_of_day` / `sun_xz_rotation` / `latitude_degrees`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeOfDaySection {
    #[serde(default)]
    pub time_of_day: f32,
    #[serde(default)]
    pub sun_xz_rotation: f32,
    #[serde(default)]
    pub latitude_degrees: f32,
}

/// `History` 节 4 字段（O：save 包络 `"History":{"chain_index"…,"history_start"…,
/// "snapshots"…,"edit_chain":[…]}`；deserialize_struct 字段数 4）。
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct HistorySectionJson {
    pub chain_index: i32,
    #[serde(default = "default_history_start")]
    pub history_start: i32,
    #[serde(default)]
    pub snapshots: Vec<Value>,
    pub edit_chain: Vec<crate::resources::history::serialized::SavedMultiEdit>,
}

/// Original ResolvedHistoryChainSnapshot serde fields (1415ce1c0): both are
/// required. The unresolved snapshot payload itself is never a unit value.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
struct SavedSnapshotReference {
    chain_index: i32,
    snapshot_id: u64,
}

/// 原版快照选择循环：按 `nearest_snapshot_index` 从最近的记录开始，打开文件；打不开的
/// 记录从**本地**候选表移除后继续找，打开成功的记录交给真实文件加载器
/// （[`crate::systems::save::snapshot::load_snapshot_file`]：motomoto chunks →
/// Snapshot JSON + 命名栅格/ Terrain chunk）。
///
/// `ignore_snapshot` 保留原版 ignore 分支：不读文件，历史引用（`parsed.snapshots`）
/// 原样保留。
///
/// 已保存引用**不**从 `parsed.snapshots` 删除：原版保留存档引用列表（快照 GC/重存
/// 依赖），只有本地候选表随打不开的文件收缩。
fn load_closest_valid_snapshot(
    references: &[Value],
    target_chain_index: i32,
    snapshot_root: &Path,
    ignore_snapshot: bool,
) -> anyhow::Result<Option<(i32, crate::systems::save::snapshot::LoadedSnapshotData)>> {
    let mut snapshots: Vec<
        crate::resources::history::transactions::ResolvedHistorySnapshot<SavedSnapshotReference>,
    > = references
        .iter()
        .cloned()
        .map(|value| {
            serde_json::from_value::<SavedSnapshotReference>(value).map(|record| {
                crate::resources::history::transactions::ResolvedHistorySnapshot {
                    chain_index: record.chain_index,
                    value: record,
                }
            })
        })
        .collect::<Result<_, _>>()
        .context("parsing snapshot references")?;
    if ignore_snapshot {
        return Ok(None);
    }
    while let Some(index) =
        crate::resources::history::snapshots::nearest_snapshot_index(&snapshots, target_chain_index)
    {
        let record = snapshots[index].value;
        let path = snapshot_root.join(format!("{}.snapshot", record.snapshot_id));
        match std::fs::File::open(&path) {
            Ok(_) => {
                // 打开成功 = 权威数据源；解析失败保留原版失败语义（不静默回退）。
                let loaded = crate::systems::save::snapshot::load_snapshot_file(&path)
                    .with_context(|| format!("loading history snapshot {}", path.display()))?;
                log::info!(
                    "Loading history snapshot {} (chain_index {})",
                    path.display(),
                    record.chain_index
                );
                return Ok(Some((record.chain_index, loaded)));
            }
            Err(error) => {
                log::warn!(
                    "Failed to load history snapshot {}: {error}",
                    path.display()
                );
                snapshots.retain(|item| item.value.snapshot_id != record.snapshot_id);
            }
        }
    }
    Ok(None)
}

fn default_history_start() -> i32 {
    -1
}

/// 存档版本兼容区间（O：fresh 反编译 `parse_saved_game_data` 结束段，
/// `version < 3` → "This save file is version {}. Minimum compatible version is {}"；
/// `version < 0x1c` 才继续，否则 "This save file version {} is above this build's
/// compatable version {}" [原文拼写]；latest = 0x1b）。
pub const MIN_COMPATIBLE_SAVE_VERSION: i32 = 3;
pub const LATEST_SAVE_VERSION: i32 = 0x1b;
/// Version 节解析失败时的默认值（O：`+0x11c = 0x19`）。
pub const DEFAULT_SAVE_VERSION: i32 = 0x19;

/// 读档后命令（PDB：`load_game::PostGameLoadCommand`）。
/// 变体 0 = SwitchAppMode（load_game 回放时忽略并 log）。第二遍回放已输出
/// `ReplayedOnLoadCommand::Apply`/`SwitchAppMode`；此枚举登记原 PDB 合同。
#[allow(dead_code)] // Recovered contract type; replayed commands surface as ReplayedOnLoadCommand.
#[derive(Clone, Debug)]
pub enum PostGameLoadCommand {
    /// 请求中的拥有型加载后动作，按原顺序移动，不复制裸指针。
    SessionEntry(OnLoadEntry),
    SwitchAppMode(AppMode),
    /// 变体 1：{tag, 载荷×2}；域载荷由所属模块提供。
    Apply {
        tag: u64,
        payload_a: Value,
        payload_b: Value,
    },
    /// 变体 2+：Arc 负载 drop（I）。
    DropArc(Value),
}

/// `LoadingGameComplete` 事件（PDB：`load_game::LoadingGameComplete`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadingGameComplete {
    /// B state2 PostGameLoadCommand::SwitchAppMode carries the 16 B AppMode.
    pub restore_to: AppMode,
}

/// `StartupRevealStarted` / `StartupRevealFinished` 事件（PDB 同名）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StartupRevealStarted;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StartupRevealFinished;

/// `parse_saved_game_data` 0x141470d00：读档并解析节序
/// `Version → BackwardCompatibility → Camera → Time of day`。
/// **upgrade 备份不在本函数**（fresh 反编译 0x141470d00 无 copy_file_to_backup 调用；
/// 备份在调用方 load_game 0x14146d030 行 681，见 [`make_upgrade_backup`]）。
/// 失败返回 "Couldn't load save file from " 语义错误。
pub fn parse_saved_game_data(
    path: &Path,
    settings: &crate::resources::glade_settings::GladeSettings,
) -> anyhow::Result<ParsedGameData> {
    use crate::resources::critters::sheep::control::SheepControlSnapshot;
    use crate::resources::glade_color_customization::GladeColorCustomization;
    use crate::resources::history::HistoryIdentity;
    use crate::systems::camera_rig::CameraRigParametersCache;

    log::debug!("Reading file data");
    // O：`std::fs::read_to_string`（0x141470d00 行 195）→ serde_json `from_str<Value>`。
    let text = std::fs::read_to_string(path).map_err(|error| {
        anyhow::anyhow!("Couldn't load save file from {}: {error}", path.display())
    })?;
    log::debug!("Parsing JSON");
    let value: Value = serde_json::from_str(&text).map_err(|error| {
        anyhow::anyhow!("Couldn't load save file from {}: {error}", path.display())
    })?;
    let object = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Save file root is not an object"))?;

    // Version：typed i32；解析失败保留默认 0x19（O：`+0x11c = 0x19` 分支）。
    let version = match object.get("Version") {
        Some(value) => serde_json::from_value::<i32>(value.clone()).unwrap_or_else(|error| {
            log::warn!("Couldn't parse save file version: {error}");
            DEFAULT_SAVE_VERSION
        }),
        None => DEFAULT_SAVE_VERSION,
    };
    // 兼容区间（O：fresh 结束段两条 format 错误）。
    if version < MIN_COMPATIBLE_SAVE_VERSION {
        anyhow::bail!(
            "This save file is version {version}. Minimum compatible version is {MIN_COMPATIBLE_SAVE_VERSION}"
        );
    }
    if version >= LATEST_SAVE_VERSION + 1 {
        anyhow::bail!(
            "This save file version {version} is above this build's compatable version {LATEST_SAVE_VERSION}"
        );
    }

    // Camera：typed，失败 fatal（context "parsing camera"）。
    let camera_value = object.get("Camera").cloned().unwrap_or(Value::Null);
    let camera: CameraRigParametersCache =
        serde_json::from_value(camera_value).context("parsing camera")?;

    // Time of day：typed 3×f32 → legacy f32 → 默认 + 告警。
    let time_of_day = match object.get("Time of day") {
        Some(value) => match serde_json::from_value::<TimeOfDaySection>(value.clone()) {
            Ok(section) => section,
            Err(_) => match serde_json::from_value::<f32>(value.clone()) {
                Ok(time_of_day) => TimeOfDaySection {
                    time_of_day,
                    ..TimeOfDaySection::default()
                },
                Err(_) => {
                    log::warn!("This save file had no `Time of day` data");
                    TimeOfDaySection::default()
                }
            },
        },
        None => {
            log::warn!("This save file had no `Time of day` data");
            TimeOfDaySection::default()
        }
    };

    // Sheep：失败日志 "Error parrsing sheep snapshot: {}"（原文拼写）→ None。
    let sheep: Option<SheepControlSnapshot> = object.get("Sheep").and_then(|value| {
        serde_json::from_value::<SheepControlSnapshot>(value.clone())
            .map_err(|error| log::warn!("Error parrsing sheep snapshot: {error}"))
            .ok()
    });

    let initial_colors = || GladeColorCustomization {
        freeform_wall: crate::resources::color_id::ColorId(settings.default_freeform_wall_color_id),
        building: crate::resources::color_id::ColorId(settings.default_building_color_id),
        roof: crate::resources::color_id::ColorId(settings.default_roof_color_id),
        stairs: crate::resources::glade_color_customization::StairCustomization::default(),
        lantern_brightness: 1.0,
        building_wall_style: crate::resources::walls::inner_wall_state::WallStyle::STONE_WITH_SKIRTS,
        ..GladeColorCustomization::default()
    };
    // Colors：失败 → default + "Initializing new color customization."。
    let colors: GladeColorCustomization = match object.get("Colors") {
        Some(value) => match serde_json::from_value(value.clone()) {
            Ok(colors) => {
                log::debug!("Loading existing color customization.");
                colors
            }
            Err(error) => {
                log::warn!("Initializing new color customization.: {error}");
                initial_colors()
            }
        },
        None => {
            log::warn!("Initializing new color customization.");
            initial_colors()
        }
    };

    // History：typed 4 字段；失败 fatal（context "parsing history"）。
    let mut history_value = object.get("History").cloned().unwrap_or(Value::Null);
    let legacy_enclosure_changed = if version != LATEST_SAVE_VERSION {
        migration::migrate_legacy_enclosure(&mut history_value)?
    } else {
        false
    };
    let mut history: HistorySectionJson =
        serde_json::from_value(history_value).context("parsing history")?;
    let history_start = history.history_start;

    // Identity：失败 → log + `derive_fallback_save_file_identity`。
    let identity = match object.get("Identity") {
        Some(value) => match serde_json::from_value::<HistoryIdentity>(value.clone()) {
            Ok(identity) => Some(identity),
            Err(error) => {
                log::warn!("Couldn't read save identity: {error}.");
                None
            }
        },
        None => {
            log::warn!("Couldn't read save identity: Field not found.");
            None
        }
    };
    let mut identity = match identity {
        Some(identity) => Some(identity),
        None => {
            let derived =
                crate::systems::save::save_file_identity::derive_fallback_save_file_identity(path);
            log::info!("Derived save identity from file meta: {}", derived.0);
            Some(derived)
        }
    };

    let backward_compatibility = object.get("BackwardCompatibility").cloned();

    let (metadata, history_mutated) =
        migration::migrate_history(&mut history, version, legacy_enclosure_changed);
    if history_mutated {
        // History::finish_potential_mutation 140a86980 uses the same native
        // Identity owner after invalidating snapshots; no second resource.
        identity = Some(HistoryIdentity::generate());
        log::info!("Deprecated operations have been found. Upgrading the save file");
    }

    Ok(ParsedGameData {
        migration_metadata: Some(metadata),
        version,
        backward_compatibility,
        camera: Some(camera),
        time_of_day,
        sheep,
        colors,
        identity,
        chain_index: history.chain_index,
        history_start,
        edits: history
            .edit_chain
            .into_iter()
            .map(|entry| entry.0)
            .collect(),
        snapshots: history.snapshots,
    })
}

/// upgrade 前存档备份（**在 load_game 0x14146d030 调用，非 parse_saved_game_data**）。
///
/// **目标名验证（2026-10-17 fresh 反编译）**：load_game 0x14146d030 行 681 调用
/// `::utils::file::copy_file_to_backup`（0x140c96a80）；该函数 = `generate_backup_file_path`
/// （0x140c968b0）= `{源文件名}-{YYYY-MM-DD HH-MM-SS}.json`（同目录，`with_file_name`
/// 替换）+ `std::sys::pal::windows::fs::copy` + `with_context`。`date_time_string_for_filename`
/// （0x140c96690）= `OffsetDateTime::now_utc().to_offset(LOCAL_OFFSET)` + time crate 格式
/// 描述串（"{:04}-{:02}-{:02} {:02}-{:02}-{:02}"，与 utils/src/file.rs 实现一致）。
/// 旧 `backup-saves/history-recovered-{ts}.json` 目标名已弃。
///
/// Native gate: wrapper+c && !new_session, Path source only. Copy failure
/// propagates to the existing failed-load route, never silent success.
fn make_upgrade_backup(path: &Path) -> anyhow::Result<()> {
    utils::file::copy_file_to_backup(path).map_err(Into::into)
}

/// 版本 → 全局开关（O：0x14146dc82..0x14146dc95，有符号比较）。复用
/// `resources::save_file::LoadVersionGates`。
pub fn version_gates(version: i32) -> LoadVersionGates {
    LoadVersionGates::from_version(version)
}

/// Window bit40 applies the original nine-way map to all three window types.
pub fn upgrade_window_color(subtype: u8, color: &mut u32) {
    if window_record_applies(subtype) {
        *color = upgrade_window_color_id(*color);
    }
}

/// ArrowSlit bit48 replaces only type0x3a with999.
pub fn upgrade_arrow_slit_color(subtype: u8, color: &mut u32) {
    upgrade_arrow_slit_record_color(subtype, color);
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
    edit.wall_id().map(|wall| wall.0)
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
    /// 第一遍变体 0：写 manager 状态 + 24B 事件载荷（O：0x14146d030 首遍 uVar23==0 段，
    /// push `{&str, SettingsMenuState}` + 进度计数）。
    WriteManager {
        payload_a: u64,
        payload_b: u64,
        counter: i32,
    },
    /// 第一遍变体 1：MetaCommand（0x38B = 7 qword；载荷取条目 3 qword）。应用由
    /// meta_commander.rs 的 commander_update 消费。
    MetaCommand(MetaCommand),
    /// 第二遍变体 1/2+：PostGameLoadCommand 载荷 {tag, payload_a, payload_b, payload_c}。
    Apply {
        tag: u64,
        payload_a: u64,
        payload_b: u64,
        payload_c: u64,
    },
}

/// 第一遍回放：MetaCommand（0x14146d030 行 ~390 区）。**终止哨兵 =
/// 0x8000000000000003（tag）**（RE-SESSION-SWITCH 定案）。变体解码
/// `decoded = tag ^ 0x8000000000000000`，**`decoded > 2 → 1`**（原生 `if (2<decoded)
/// decoded=1`，非 `.min(2)`；≥3 分派不同）。0=manager 状态、1=MetaCommand、
/// 2=**fence 释放（内联 unblock + Arc drop，O：0x14146dc.. 首遍 uVar23==2 段）**。
/// 批量条目（tag=3）展开为 MetaCommand。
fn replay_meta_commands(
    manager: &mut LoadGameManager,
    on_load: &[OnLoadEntry],
    out: &mut Vec<ReplayedOnLoadCommand>,
) {
    for entry in on_load {
        match entry {
            OnLoadEntry::Command {
                tag,
                payload_a,
                payload_b,
                payload_c,
            } => {
                if *tag == ON_LOAD_FENCE {
                    break;
                }
                let decoded = *tag ^ ON_LOAD_VARIANT_MASK;
                let decoded = if decoded > 2 { 1 } else { decoded };
                match decoded {
                    0 => {
                        manager.progress_counter = manager.progress_counter.wrapping_add(1);
                        out.push(ReplayedOnLoadCommand::WriteManager {
                            payload_a: *payload_a,
                            payload_b: *payload_b,
                            counter: manager.progress_counter,
                        });
                    }
                    1 => {
                        out.push(ReplayedOnLoadCommand::MetaCommand(MetaCommand {
                            tag: *tag,
                            payloads: [*payload_a, *payload_b, *payload_c, 0, 0, 0],
                        }));
                    }
                    2 => {
                        // 原始整数不是 Arc 所有权凭据，禁止从任意地址构造 Arc。
                        // 新请求统一使用 OnLoadEntry::Fence。
                        if *payload_a != 0 {
                            log::error!("拒绝旧式裸地址 fence；请使用拥有型加载信号");
                        }
                    }
                    _ => {}
                }
            }
            OnLoadEntry::Fence(fence) => fence.unblock(),
            OnLoadEntry::Batch { commands } => {
                for command in commands {
                    out.push(ReplayedOnLoadCommand::MetaCommand(*command));
                }
            }
        }
    }
}

/// 第二遍回放：PostGameLoadCommand（0x14146d030 行 ~800 区）。变体 0（SwitchAppMode）
/// **仅 log 不 push**（原生只 log "Ignoring PostGameLoadCommand::SwitchAppMode"）；
/// 变体 1 push `{tag, payload_a, payload_b, payload_c}`；变体 2+ push
/// `{0x8000000000000000, ...}`；**终止哨兵 = 0x8000000000000003（tag）**。
/// 批量条目展开为 Apply。
fn replay_post_game_load_commands(on_load: &[OnLoadEntry], out: &mut Vec<ReplayedOnLoadCommand>) {
    for entry in on_load {
        match entry {
            OnLoadEntry::Command {
                tag,
                payload_a,
                payload_b,
                payload_c,
            } => {
                if *tag == ON_LOAD_FENCE {
                    break;
                }
                let decoded = *tag ^ ON_LOAD_VARIANT_MASK;
                let decoded = if decoded > 2 { 1 } else { decoded };
                match decoded {
                    0 => {
                        log::info!("Ignoring PostGameLoadCommand::SwitchAppMode");
                    }
                    1 => {
                        out.push(ReplayedOnLoadCommand::Apply {
                            tag: *tag,
                            payload_a: *payload_a,
                            payload_b: *payload_b,
                            payload_c: *payload_c,
                        });
                    }
                    _ => {
                        out.push(ReplayedOnLoadCommand::Apply {
                            tag: ON_LOAD_VARIANT_MASK,
                            payload_a: *payload_a,
                            payload_b: *payload_b,
                            payload_c: *payload_c,
                        });
                    }
                }
            }
            OnLoadEntry::Fence(_) => {}
            OnLoadEntry::Batch { commands } => {
                for command in commands {
                    out.push(ReplayedOnLoadCommand::Apply {
                        tag: command.tag,
                        payload_a: command.payloads[0],
                        payload_b: command.payloads[1],
                        payload_c: command.payloads[2],
                    });
                }
            }
        }
    }
}

/// NextSession kind 解码 clamp（O：load_game 0x14146d030 两处条目解码
/// `decoded = tag ^ 0x8000000000000000`；**`if (decoded > 2) decoded = 1`**，
/// 不是 `.min(2)`——≥3 与 2 的分派不同：2 是 on_load 回放，≥3 落到变体 1）。
#[inline]
pub fn clamp_next_session_kind(decoded: u64) -> u64 {
    if decoded > 2 { 1 } else { decoded }
}

/// `load_game` 主系统纯核心：状态推进 + kind 解码 + on_load 两遍回放。
///
/// - 门控 `app_mode == LoadGameData` 由调用方执行（Bevy 系统参数未齐）。
/// - kind 解码：`decoded = kind ^ 0x8000000000000000`；`decoded > 2 → 1`
///   （O：fresh `if (2 < decoded) decoded = 1`）。sel==1 → parse_saved_game_data
///   （解析成功后 load_game 做 upgrade 备份）；sel==2 → on_load 两遍回放
///   （MetaCommand → PostGameLoadCommand，fence 终止）。
pub fn advance_load_game(
    manager: &mut LoadGameManager,
    next: &mut NextSession,
    save_path: Option<&Path>,
    settings: &crate::resources::glade_settings::GladeSettings,
) -> Result<(Option<ParsedGameData>, Vec<ReplayedOnLoadCommand>), LoadGameError> {
    match manager.state {
        LoadGameState::Loading => {
            manager.progress_counter = manager.progress_counter.wrapping_add(1);
            manager.state = LoadGameState::Loaded;
            Ok((None, Vec::new()))
        }
        LoadGameState::ApplyCounter => Err(anyhow::anyhow!(
            "B load_game state1 requires original counter event producer"
        )
        .into()),
        LoadGameState::Failed => {
            Err(anyhow::anyhow!("B load_game request failed; state is terminal (no retry)").into())
        }
        LoadGameState::Loaded => {
            if next.kind == crate::resources::session::NextSessionKind::LoadParsedGame {
                let parsed = *next.take_parsed_data()?;
                manager.progress_counter = manager.progress_counter.wrapping_add(1);
                return Ok((Some(parsed), Vec::new()));
            }
            let selected = clamp_next_session_kind(next.kind.decoded());
            match selected {
                0 => {
                    // 无请求（load_latest 默认 kind=0x8000000000000002 → sel==2）。
                    Ok((None, Vec::new()))
                }
                1 => {
                    let path = save_path
                        .ok_or_else(|| anyhow::anyhow!("parse_saved_game_data needs a path"))?;
                    let parsed = parse_saved_game_data(path, settings)?;
                    manager.progress_counter = manager.progress_counter.wrapping_add(1);
                    Ok((Some(parsed), Vec::new()))
                }
                2 => {
                    if next.on_load.iter().any(|entry| {
                        matches!(entry,
                        OnLoadEntry::Command { tag, payload_a, .. }
                        if (*tag ^ ON_LOAD_VARIANT_MASK) == 2 && *payload_a != 0)
                    }) {
                        return Err(anyhow::anyhow!(
                            "旧式裸地址 fence 无法安全回放，请改用拥有型信号"
                        )
                        .into());
                    }
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
/// → max wall id 恢复。本纯核心不持有 World；快照域写回走
/// `try_load_game_system` → `snapshot::init_game_state_from_snapshot`。
#[allow(clippy::too_many_arguments)]
pub fn load_game_session(
    manager: &mut LoadGameManager,
    next: &mut NextSession,
    save_path: Option<&Path>,
    edits: &[WallCompoundHistoryEdit],
    upgrade_flags: crate::resources::save_file::LoadUpgradeFlags,
    settings: &crate::resources::glade_settings::GladeSettings,
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
    let (parsed, commands) = advance_load_game(manager, next, save_path, settings)?;
    // 1b. upgrade 备份（O：load_game 0x14146d030 行 681 在 parse 成功后调用
    //     copy_file_to_backup；备份只出现在 kind==1 解析路径 = parsed.is_some()）。
    if let (Some(parsed), Some(path)) = (&parsed, save_path.filter(|_| next.kind != crate::resources::session::NextSessionKind::LoadParsedGame)) {
        if parsed
            .migration_metadata
            .context("B parsed migration metadata missing")?
            .requires_upgrade_backup
        {
            make_upgrade_backup(path).context("backing up save before upgrade")?;
        }
    }
    // 2. 版本 → 全局开关（O：0x14146dc82..0x14146dc95）。
    let version = parsed.as_ref().map_or(0, |data| data.version);
    let gates = version_gates(version);
    if parsed.is_some() {
        // 成功读取存档后先设置采样模式；加载准备阶段和命令回放不覆盖当前会话策略。
        gates.apply_centered_path(&crate::utils::float_grid::CENTERED_PATH);
    }
    // Entry-owned history colors are migrated by the parser. The flags here
    // apply to a loaded snapshot, owned only by the World-level consumer.
    let _ = upgrade_flags;
    // 4. max wall id 恢复（O：extract_max_wall_id_from_history）。
    let max_wall_id = extract_max_wall_id_from_history(edits.iter().cloned());
    Ok((parsed, commands, gates, max_wall_id))
}

/// 写 `save_folder/loading.tmp`（O：load_game 0x14146d030 后置写标记；@0x142c6416e）。
/// 文件内容为读档进度标记；失败仅告警（宿主可据此判定正在读档）。
pub fn write_loading_tmp(session: &SessionId) {
    let path = session.save_folder_path().join("loading.tmp");
    if let Err(error) = std::fs::write(&path, b"loading") {
        log::warn!("Failed writing loading.tmp for {}: {error}", session.0);
    }
}

/// `load_game` 0x14146d030 的 ECS 系统（消费 `NextSession`，门 AppMode::LoadGame）。
///
/// - 门：`*app_mode == 0xe`（AppMode::LoadGame）才运行；
/// - 状态字节 +9 推进（`advance_load_game`：0→进度记录+置 2；2→主流程）；
/// - kind 解码：sel==1 → parse_saved_game_data（含 upgrade 备份）；sel==2 → on_load
///   两遍回放（MetaCommand → PostGameLoadCommand，fence 终止）；
/// - 回放的 MetaCommand 经 `MetaCommander::extend_script` 入队（commander_update case 0
///   会话加载链消费）；
/// - 事件：LoadingGameComplete / StartupRevealStarted（StartupRevealFinished 由宿主
///   应用完加载后发）。
///
/// PostGameLoadCommand::Apply 的应用（FUN_1412e0b90）由域消费者处理；
/// 快照域写回由调用方在基础状态之后经
/// `snapshot::init_game_state_from_snapshot` 接入。
/// B no-valid-snapshot branch @14146dfa6. The authoritative starting state is
/// the real saved edit chain at cursor-1, not an invented empty snapshot.
fn apply_history_without_snapshot(
    world: &mut bevy_ecs::world::World,
    mut parsed: ParsedGameData,
    options: ActiveLoadOptions,
    _source_file: Option<&Path>,
) -> anyhow::Result<super::save_game::AsyncSaveCmd> {
    use super::save_game::{
        AsyncSaveCmd, AutoSaveState, SerializedJson, assemble_game_save_sections,
    };
    use crate::resources::{
        history::{History, transactions::ObservedHistoryLedger},
        session::ActiveSession,
        time_of_day::TimeOfDay,
        walls::InnerWalls,
    };
    use crate::systems::camera_rig::{CameraRigParametersCache, main_camera::MainCamera};
    // Validate/prepare fallible owners before publishing a half-applied World.
    let session = world
        .get_resource::<ActiveSession>()
        .context("B active session producer missing")?
        .id;
    let config = world
        .get_resource::<crate::game_config::GameConfig>()
        .context("B GameConfig missing")?;
    let demo = config.dev.demo;
    let target_fov = config.game.camera_fov;
    let mut camera = world
        .get_resource::<MainCamera>()
        .context("B MainCamera missing")?
        .clone();
    camera.load_parameters(
        parsed.camera.as_ref().context("B parsed Camera missing")?,
        options.ignore_snapshot,
    );
    camera.finish_loaded_normal_interpolation(demo)?;
    // Original new-session flag replaces the incoming version with0x1b.
    if options.new_session {
        parsed.version = LATEST_SAVE_VERSION;
    }
    let gates = version_gates(parsed.version);
    let mut time = world
        .get_resource::<TimeOfDay>()
        .context("B TimeOfDay producer missing")?
        .clone();
    time.time_of_day = parsed.time_of_day.time_of_day;
    time.sun_xz_rotation = parsed.time_of_day.sun_xz_rotation;
    time.latitude_degrees = parsed.time_of_day.latitude_degrees;
    let max_id = parsed
        .edits
        .iter()
        .flat_map(|entry| &entry.edits)
        .filter_map(|edit| {
            if let crate::resources::history::EditType::Wall(wall) = edit {
                Some((**wall).clone())
            } else {
                None
            }
        });
    let max_id = extract_max_wall_id_from_history(max_id).unwrap_or(0);
    anyhow::ensure!(
        world.contains_resource::<InnerWalls>(),
        "B InnerWalls constructor missing"
    );
    anyhow::ensure!(
        world.contains_resource::<AutoSaveState>(),
        "B Async save owner missing"
    );
    let saved_pose = camera.store_parameters();
    let saved_camera = CameraRigParametersCache {
        yaw: saved_pose.yaw,
        pitch: saved_pose.pitch,
        position: saved_pose.position,
        camera_rig_arm_length: saved_pose.camera_rig_arm_length,
        vertical_fov_degrees: saved_pose.vertical_fov_degrees,
        pitch_offset: saved_pose.pitch_offset,
        yaw_offset: saved_pose.yaw_offset,
    };
    // Native +ec is moved unchanged into History+6c. In particular -1 is
    // the saved/default no-anchor value, not an instruction to clamp to0.
    let undo_boundary = parsed.history_start;
    let sections = assemble_game_save_sections(
        parsed.version,
        parsed
            .identity
            .context("B parsed history identity missing")?,
        &saved_camera,
        &parsed.colors,
        SerializedJson::new(&time)?,
        SerializedJson::new(&parsed.sheep)?,
        SerializedJson::new(&parsed.chain_index)?,
        SerializedJson::new(&undo_boundary)?,
        SerializedJson::new(&parsed.snapshots)?,
    );
    let edits = parsed
        .edits
        .iter()
        .cloned()
        .map(crate::resources::history::serialized::SavedMultiEdit)
        .map(|entry| SerializedJson::new(&entry))
        .collect::<Result<Vec<_>, _>>()?;
    let backups = super::save_backup_list::SaveBackupList::new_from_session(session);
    gates.apply_centered_path(&crate::utils::float_grid::CENTERED_PATH);
    // The second version global is likewise the original shared owner.
    crate::resources::stairs::stairs_state::VERIFY_STAIR_CHECKSUMS
        .store(gates.stairs_checksum, std::sync::atomic::Ordering::Relaxed);
    let count = parsed.edits.len();
    let snapshot_references = parsed
        .snapshots
        .iter()
        .cloned()
        .map(serde_json::from_value::<crate::resources::history::ResolvedHistoryChainSnapshot>)
        .collect::<Result<Vec<_>, _>>()
        .context("canonical History snapshot references")?;
    world.insert_resource(History::from_observed(ObservedHistoryLedger {
        entries: parsed.edits,
        cursor: -1,
        undo_anchor: undo_boundary,
        resolved_snapshots: snapshot_references.into_iter().map(Into::into).collect(),
        observed_count_58: count,
        // Parser writes parsed+0x48; load14146dd98..de95 moves it into
        // canonical History+0x60 after the serde object's temporary zero.
        mutation_nonce: parsed
            .identity
            .context("B parsed history identity missing")?
            .0,
    }));
    camera.write_f32(0x194, target_fov);
    world.insert_resource(camera);
    world.insert_resource(time);
    world.insert_resource(parsed.colors);
    // Existing canonical SheepControl is the deferred snapshot owner. This
    // does not spawn a sheep or fake the downstream critter query's contents.
    world.insert_resource(crate::systems::debug::sheep_controls::SheepControl {
        snapshot: parsed.sheep,
    });
    {
        let mut walls = world.resource_mut::<InnerWalls>();
        let restored_max = walls.max_id().max(max_id);
        walls.set_max_id(restored_max);
    }
    {
        let mut auto = world.resource_mut::<AutoSaveState>();
        auto.session = session;
        auto.metronome =
            super::save_backup_metronome::SaveBackupMetronome::new_from_backup_list(&backups);
    }
    let mut manager = world.resource_mut::<LoadGameManager>();
    manager.progress_counter = parsed.chain_index.wrapping_sub(-1);
    manager.source = LoadGameSource::None;
    // `state` 由调用方在全部域写回成功后推进（M2：快照 apply 失败不得进入
    // ApplyCounter → Loaded 的完成路径）。
    // Native OpenFile is sent after snapshot application and compatibility
    // invalidation. Keep this owned command local until that boundary.
    Ok(AsyncSaveCmd::OpenFile {
        session,
        sections,
        edits,
    })
}

pub fn load_game_system(world: &mut bevy_ecs::world::World) -> bevy_ecs::error::Result {
    try_load_game_system(world).map_err(|error| error.to_string().into())
}

pub fn try_load_game_system(world: &mut bevy_ecs::world::World) -> anyhow::Result<()> {
    if world.resource::<AppMode>().native_tag() != 14 {
        return Ok(());
    }
    let manager = world.resource::<LoadGameManager>();
    match manager.state {
        LoadGameState::Loading => {
            // M2/M3：只有全部域写回成功才推进到 ApplyCounter；任何 `?` 失败都转
            // Failed 终态（下一帧不再重试同一坏文件，也不进入 Loaded 完成路径）。
            let result = (|| -> anyhow::Result<()> {
                let manager = world.resource::<LoadGameManager>();
                let options = manager
                    .options
                    .context("B active load has no startup options")?;
                let source = std::mem::take(&mut world.resource_mut::<LoadGameManager>().source);
                let (parsed, snapshot_root, source_file) = match source {
                    LoadGameSource::Path(path) => (
                        parse_saved_game_data(
                            &path,
                            world
                                .get_resource::<crate::resources::glade_settings::GladeSettings>()
                                .context("B GladeSettings producer missing")?,
                        )
                        .with_context(|| format!("B load_game source {}", path.display()))?,
                        path.parent()
                            .context("B source history path has no parent")?
                            .join("snapshots"),
                        Some(path.clone()),
                    ),
                    LoadGameSource::Parsed {
                        data,
                        snapshot_root,
                    } => (*data, snapshot_root, None),
                    LoadGameSource::None => {
                        anyhow::bail!("B LoadGameState::LoadGameData has no source")
                    }
                };
                let metadata = parsed
                    .migration_metadata
                    .context("B parsed migration metadata missing")?;
                if let Some(path) = &source_file {
                    if metadata.requires_upgrade_backup && !options.new_session {
                        make_upgrade_backup(path).context("backing up save before upgrade")?;
                    }
                }
                let snapshot_references = parsed.snapshots.clone();
                let target_chain_index = parsed.chain_index;
                // 快照 load_state 会把 InnerWalls.max_id 置为快照记录最大值；历史链里
                // 可能有更大的 id（原版 extract_max_wall_id_from_history 写同一计数器），
                // 先取值再应用。
                let history_max_wall_id = extract_max_wall_id_from_history(
                    parsed
                        .edits
                        .iter()
                        .flat_map(|entry| &entry.edits)
                        .filter_map(|edit| match edit {
                            crate::resources::history::EditType::Wall(wall) => {
                                Some((**wall).clone())
                            }
                            _ => None,
                        }),
                );
                // 2. 基础状态（相机/时间/颜色/历史/保存 owner）。
                let mut save_open =
                    apply_history_without_snapshot(world, parsed, options, source_file.as_deref())?;
                // Native bit56 writes the SpawnPreplacedTrees unit event,
                // not a warning event: tuple writer at in_stack68[9].
                if metadata.legacy_version_notice {
                    world
                        .resource_mut::<bevy_ecs::event::Events<
                            crate::resources::history::tree_edit::SpawnPreplacedTrees,
                        >>()
                        .send(crate::resources::history::tree_edit::SpawnPreplacedTrees);
                }
                let snapshot = load_closest_valid_snapshot(
                    &snapshot_references,
                    target_chain_index,
                    &snapshot_root,
                    options.ignore_snapshot,
                )?;
                // 3. 有快照 → 走 init_game_state_from_snapshot 的域写回；无快照 → 原版
                //    @14146dfa6 分支（历史链 cursor-1 起始状态，已在上面应用）。
                if let Some((snapshot_chain_index, mut loaded)) = snapshot {
                    migration::migrate_snapshot(&mut loaded, metadata)?;
                    let resources = crate::systems::save::snapshot::SnapshotLoadResources {
                        loaded: Some(loaded),
                    };
                    let report = crate::systems::save::snapshot::init_game_state_from_snapshot(
                        world, &resources,
                    )?;
                    // Original applies the selected snapshot's chain cursor,
                    // then replays only the signed delta to the target.
                    {
                        let mut history =
                            world.resource_mut::<crate::resources::history::History>();
                        history.ledger_mut().cursor = snapshot_chain_index;
                        if metadata.upgrade_window_colors {
                            history.ledger_mut().resolved_snapshots.clear();
                        }
                    }
                    if metadata.upgrade_window_colors {
                        if let super::save_game::AsyncSaveCmd::OpenFile { sections, .. } =
                            &mut save_open
                        {
                            sections.values[crate::resources::save_file::SaveFileSection::Snapshots
                                as usize] =
                                super::save_game::SerializedJson::new(&Vec::<Value>::new())?;
                        }
                    }
                    world.resource_mut::<LoadGameManager>().progress_counter =
                        target_chain_index.wrapping_sub(snapshot_chain_index);
                    if let Some(history_max) = history_max_wall_id {
                        let mut walls = world.resource_mut::<crate::resources::walls::InnerWalls>();
                        if walls.max_id() < history_max {
                            walls.set_max_id(history_max);
                        }
                    }
                    if report.gaps.is_empty() {
                        log::info!(
                            "Applied history snapshot: walls={} roofs={} decorators={} trees={} strokes={} raster={}",
                            report.walls,
                            report.roofs,
                            report.decorators,
                            report.trees,
                            report.terrain_strokes,
                            report.raster_slots
                        );
                    } else {
                        // 部分域未闭合：明确记录，不宣告完整加载。
                        log::warn!(
                            "History snapshot applied with {} gap(s): {}",
                            report.gaps.len(),
                            report.gaps.join("; ")
                        );
                    }
                } else {
                    log::debug!("No history snapshot applied; starting from the saved edit chain");
                }
                world
                    .resource_mut::<super::save_game::AutoSaveState>()
                    .send_command_expect(save_open);
                Ok(())
            })();
            match result {
                Ok(()) => {
                    world.resource_mut::<LoadGameManager>().state = LoadGameState::ApplyCounter;
                    Ok(())
                }
                Err(error) => {
                    {
                        let mut manager = world.resource_mut::<LoadGameManager>();
                        manager.state = LoadGameState::Failed;
                        manager.source = LoadGameSource::None;
                        manager.options = None;
                        manager.post_load_commands.clear();
                        manager.post_load_cursor = 0;
                    }
                    log::error!("B load_game failed (terminal): {error}");
                    Err(error)
                }
            }
        }
        LoadGameState::Failed => {
            // 终态：首次失败已上报，后续帧不重复上报、不重试同一输入。
            Ok(())
        }
        LoadGameState::ApplyCounter => {
            world.resource_scope(|world, mut manager: bevy_ecs::prelude::Mut<LoadGameManager>| {
                emit_load_history_commands(&mut manager, &mut world.resource_mut::<bevy_ecs::event::Events<crate::systems::mode_manager::HistoryCommand>>());
            });
            Ok(())
        }
        LoadGameState::Loaded => {
            // Only a real active request may consume post-load commands. The
            // idle constructor also holds a default SwitchAppMode command.
            if world.resource::<LoadGameManager>().options.is_none() {
                return Ok(());
            }
            // 先拒绝当前没有消费者的旧载荷，避免先通知完成或释放信号后才报错。
            anyhow::ensure!(
                world
                    .resource::<LoadGameManager>()
                    .post_load_commands
                    .iter()
                    .all(|command| {
                        matches!(
                            command,
                            PostGameLoadCommand::SwitchAppMode(_)
                                | PostGameLoadCommand::SessionEntry(
                                    OnLoadEntry::Batch { .. } | OnLoadEntry::Fence(_)
                                )
                                | PostGameLoadCommand::SessionEntry(OnLoadEntry::Command {
                                    tag: ON_LOAD_FENCE,
                                    ..
                                })
                        )
                    }),
                "加载后动作包含尚未接入的域命令"
            );
            // Original state2 consumes commands in order. SwitchAppMode is a
            // sixteen-byte LoadingGameComplete payload, not a direct mode
            // mutation here. In particular it is not delayed until replay has
            // drained, nor moved past following command/fence side effects.
            loop {
                let manager = world.resource::<LoadGameManager>();
                let Some(command) = manager
                    .post_load_commands
                    .get(manager.post_load_cursor)
                    .cloned()
                else {
                    break;
                };
                match command {
                    PostGameLoadCommand::SwitchAppMode(restore_to) => {
                        world
                            .resource_mut::<bevy_ecs::event::Events<LoadingGameComplete>>()
                            .send(LoadingGameComplete { restore_to });
                    }
                    PostGameLoadCommand::SessionEntry(OnLoadEntry::Batch { commands }) => world
                        .resource_mut::<MetaCommander>()
                        .extend_script(commands),
                    PostGameLoadCommand::SessionEntry(OnLoadEntry::Fence(fence)) => fence.unblock(),
                    PostGameLoadCommand::SessionEntry(OnLoadEntry::Command {
                        tag: ON_LOAD_FENCE,
                        ..
                    }) => break,
                    other => anyhow::bail!(
                        "B load_game post-load domain command requires its original consumer: {other:?}"
                    ),
                }
                world.resource_mut::<LoadGameManager>().post_load_cursor += 1;
            }
            let mut manager = world.resource_mut::<LoadGameManager>();
            manager.post_load_commands.clear();
            manager.post_load_cursor = 0;
            manager.source = LoadGameSource::None;
            manager.options = None;
            Ok(())
        }
    }
}

/// Original B state1 @14146d8a2 /14146d952. Preserve signed NEG and the
/// signed loop bound (including its empty loop for i32::MIN). The ready event
/// is a later state2 post-command; consuming a delta never fabricates it.
pub fn emit_load_history_commands(
    manager: &mut LoadGameManager,
    commands: &mut bevy_ecs::event::Events<crate::systems::mode_manager::HistoryCommand>,
) {
    assert_eq!(manager.state, LoadGameState::ApplyCounter);
    let delta = manager.progress_counter;
    let (count, payload) = if delta < 0 {
        (delta.wrapping_neg(), 0x0301_0001_u32)
    } else {
        (delta, 0x0301_0101_u32)
    };
    for _ in 0..count {
        commands.send(crate::systems::mode_manager::HistoryCommand {
            bytes: payload.to_le_bytes(),
        });
    }
    manager.state = LoadGameState::Loaded;
}

#[cfg(test)]
mod state_machine_tests {
    use super::*;
    use bevy_ecs::world::World;

    fn loading_manager(source: LoadGameSource) -> LoadGameManager {
        let mut manager = LoadGameManager::new();
        manager.begin(
            source,
            ActiveLoadOptions {
                ignore_snapshot: false,
                request_reveal: false,
                new_session: false,
            },
            Vec::new(),
        );
        manager
    }

    #[test]
    fn load_failure_is_terminal_and_reported_once() {
        // M3：首次解析失败 → Err + Failed 终态，清空请求；后续帧不再重试同一输入。
        let mut world = World::new();
        world.insert_resource(AppMode::LoadGame);
        world.insert_resource(loading_manager(LoadGameSource::Path(
            std::path::PathBuf::from("missing/history.json"),
        )));
        assert!(try_load_game_system(&mut world).is_err());
        {
            let manager = world.resource::<LoadGameManager>();
            assert_eq!(manager.state, LoadGameState::Failed);
            assert!(manager.options.is_none());
            assert!(matches!(manager.source, LoadGameSource::None));
        }
        // 终态帧：不重复上报、不重试。
        assert!(try_load_game_system(&mut world).is_ok());
        assert_eq!(
            world.resource::<LoadGameManager>().state,
            LoadGameState::Failed
        );
    }

    #[test]
    fn apply_counter_is_only_reached_after_a_successful_load() {
        // M2：构造阶段（Loading）不再提前写 ApplyCounter；失败终态保持 Failed。
        let mut world = World::new();
        world.insert_resource(AppMode::LoadGame);
        world.insert_resource(loading_manager(LoadGameSource::Path(
            std::path::PathBuf::from("missing/history.json"),
        )));
        let _ = try_load_game_system(&mut world);
        assert_ne!(
            world.resource::<LoadGameManager>().state,
            LoadGameState::ApplyCounter
        );
        // 未加载时 state2 不消费命令（options 已清空）。
        assert!(try_load_game_system(&mut world).is_ok());
    }
}

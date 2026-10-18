//! B 0x141750c90 + 0x141751c90: JSON envelope and section/edit offsets.
//! Domain values are serialized by their owning modules; this layer does not
//! substitute fabricated wall/roof/snapshot payloads for those values.
//!
//! 本层还承载存档线程命令与自动存档状态（PDB：`save_game::AsyncSaveCmd`、
//! `AutoSaveState`、`AutoSaveSectionState`、`DeferredJsonSerialize`、`GameSaveData`、
//! `SaveGameCmd`、`DeleteSessionCmd`、`SavingGameComplete`）。
//! Identity/Camera/Colors 三节已按 re-analyst 域类型证据包组装；其余节（TimeOfDay/
//! Sheep/chain_index/history_start/snapshots）由所属域提供序列化载荷，缺失即 panic
//! （原版每节必写），不伪造。
pub use crate::resources::save_file::SaveFileSection;
use crate::resources::history::snapshots::current_snapshot;
use crate::resources::history::snapshots::set_current_snapshot;
use crate::resources::history::transactions::ResolvedHistorySnapshot;
use crate::resources::glade_color_customization::GladeColorCustomization;
use crate::resources::history::HistoryIdentity;
use crate::resources::roofs::roof_animation::RoofMarkedForDeletion;
use crate::resources::roofs::roof_shape::Roof;
use crate::resources::session::SessionId;
use crate::resources::session::SessionMeta;
use crate::resources::time::GameTime;
use crate::systems::camera_rig::CameraRigParametersCache;
use crate::systems::debug::sheep_controls::SheepControl;
use crate::systems::save::save_backup_list::BackupSaveFileKind;
use crate::systems::save::save_backup_metronome::SaveBackupMetronome;
use crate::systems::save::snapshot::SnapshotDomainData;
use crate::systems::save::snapshot::SnapshotSaveData;
use crate::systems::save::snapshot::capture_game_snapshot_save_data;
use bevy_ecs::prelude::{Query, Res, ResMut, Without};
use hashbrown::HashMap;
use serde::{Serialize, Serializer};
use std::{
    io::Write,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone, Debug, Default)]
pub struct SaveFileStructure {
    pub section_ranges: HashMap<SaveFileSection, Range<u64>>,
    pub history_edit_offsets: Vec<u64>,
}

/// Compact serialized JSON, validated at construction; consumed unchanged by
/// section replacement and history append, matching their original &str inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializedJson(String);
impl SerializedJson {
    pub fn new<T: Serialize + ?Sized>(value: &T) -> serde_json::Result<Self> {
        serde_json::to_string(value).map(Self)
    }
    /// 由**已经是 JSON 文本**的字符串直接构造（域序列化器输出；如
    /// `sheep_section_json`）。不二次转义。
    pub fn from_raw_string(value: String) -> Self {
        Self(value)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// Each field is required. There are seven root sections and three History
/// sections; no absent-domain value is silently replaced with null or zero.
#[derive(Clone, Debug)]
pub struct GameSaveSections {
    pub values: [SerializedJson; 10],
}

pub fn write_json_sections(
    output: &mut Vec<u8>,
    sections: impl IntoIterator<Item = (SaveFileSection, SerializedJson)>,
    structure: &mut SaveFileStructure,
) {
    for (section, value) in sections {
        output.push(b'"');
        output.extend_from_slice(section.json_key().as_bytes());
        output.extend_from_slice(b"\":");
        let start = output.len() as u64;
        output.extend_from_slice(value.as_bytes());
        output.push(b',');
        output.resize(output.len() + section.padding(), b' ');
        output.push(b'\n');
        structure
            .section_ranges
            .insert(section, start..output.len() as u64);
    }
}

/// `SerializedJson` 作为 **JSON 值**参与 serde 序列化（不二次转义）。
///
/// `save_game_to_file` 0x140241ca0/0x141750c90 的原生数据流 = 入参
/// `Iter<MultiEdit>` → 逐条 `serialize<serde_json::value::ser::Serializer>`
/// （MultiEdit → `Value`）→ `to_string` 写入 `edit_chain`。本 impl 让已预序列化的
/// 载荷仍可走同一路径；真正的域类型（`MultiEdit` 等价物）直接实现 `Serialize`
/// 即可，不需要调用方预先序列化。
impl Serialize for SerializedJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value: serde_json::Value =
            serde_json::from_str(&self.0).map_err(serde::ser::Error::custom)?;
        value.serialize(serializer)
    }
}

/// `save_game_to_file` 0x140241ca0 / 0x141750c90：生成完整存档 JSON 并写出。
///
/// **原生入参 = `Iter<MultiEdit>` + 函数内 serde**（fresh callees：
/// `collect_seq<serde_json::value::ser::Serializer, ref<Vec<MultiEdit>>>`、
/// `serialize<serde_json::value::ser::Serializer>`、`serialize<...CompactFormatter>`）。
/// 因此本函数以 `Iterator<Item = E: Serialize>` 承载编辑；每条编辑在函数内转成
/// `serde_json::Value` 再序列化，不在调用方预序列化。
pub fn save_game_to_memory<E: Serialize>(
    sections: GameSaveSections,
    edits: impl IntoIterator<Item = E>,
) -> (Vec<u8>, SaveFileStructure) {
    let mut output = vec![b'{'];
    let mut structure = SaveFileStructure::default();
    let mut sections = SaveFileSection::ALL.into_iter().zip(sections.values);
    write_json_sections(&mut output, sections.by_ref().take(7), &mut structure);
    output.extend_from_slice(b"\"History\":{\n");
    write_json_sections(&mut output, sections, &mut structure);
    output.extend_from_slice(b"\"edit_chain\":[\n");
    for (index, edit) in edits.into_iter().enumerate() {
        structure.history_edit_offsets.push(output.len() as u64);
        if index != 0 {
            output.extend_from_slice(b",\n");
        }
        // 原生：MultiEdit → serde_json::Value → CompactFormatter bytes；失败 panic
        // （unwrap_failed，save_game_to_file 无错误返回分支）。
        let value: serde_json::Value =
            serde_json::to_value(&edit).expect("MultiEdit rejects serde_json::Value");
        let text = serde_json::to_string(&value).expect("Value rejects CompactFormatter");
        output.extend_from_slice(text.as_bytes());
    }
    output.extend_from_slice(b"\n]}}");
    (output, structure)
}

/// `save_game_to_file<File>` 0x140241ca0 / `<Cursor<Vec<u8>>>` 0x141750c90：
/// 生成完整存档 JSON 并写出。字节布局与 `save_game_to_memory` 一致；编辑在函数内
/// 逐条 serde（见 [`save_game_to_memory`]）。
pub fn save_game_to_writer<E: Serialize>(
    writer: &mut impl Write,
    sections: GameSaveSections,
    edits: impl IntoIterator<Item = E>,
) -> std::io::Result<()> {
    let (bytes, _) = save_game_to_memory(sections, edits);
    writer.write_all(&bytes)
}

// ---------------------------------------------------------------------------
// 存档线程命令通道（PDB：`save_game::AsyncSaveCmd`，经 mpsc 传输）
// ---------------------------------------------------------------------------

/// 游戏线程 → 存档线程的命令。原版变体标签 **3..=11**（① r6-async-savecmd 契约 §1，
/// ALGORITHMS §2.1 save_thread 0x141720680 / 分发 0x141721aa0 权威；0/1/2 未在 switch
/// 出现 = 死/历史变体）。判别值显式对齐：3 OpenFile / 4 CloseFile / 9 CaptureSnapshot /
/// 10 Flush / 0xb(11) Shutdown；5/6/7/8 = ChangeSection/ReplaceEdits/BackupSave/ManualBackup。
///
/// **字段名约定**：各变体的载荷字段名是**语义字段**（由分发行为/存档 codec 合同推导），
/// **非**原字节布局字段名。原版载荷仅 tag@+0（qword 判别）与分段偏移已被观测
/// （见 save-SPEC §1.1/§1.4），各分段载荷字段名 **U/待取证**，不编造字节布局名。
#[derive(Clone, Debug)]
#[repr(u64)]
pub enum AsyncSaveCmd {
    /// 3 OpenFile：全量重写 + 建 ShadowFile("history.json")。
    /// 载荷分段（O）：path（经 save_folder_path join）+ 全量存档数据（save_game_to_file
    /// <Cursor> 重写）。`session`/`sections`/`edits` 为语义字段，字节布局名待取证。
    OpenFile {
        session: SessionId,
        sections: GameSaveSections,
        edits: Vec<SerializedJson>,
    } = 3,
    /// 4 CloseFile。载荷含 session id（语义；分段字段 U）。
    CloseFile { session: SessionId } = 4,
    /// 5 ChangeSection：改节 + mark_dirty。
    /// 载荷分段（O）：param_1[1..2]（+8/+0x10）、[3..4]（+0x18/+0x20）。
    /// `section`/`value` 为语义字段（change_section 行为合同），字节布局名待取证。
    ChangeSection {
        section: SaveFileSection,
        value: SerializedJson,
    } = 5,
    /// 6 ReplaceEdits：截断 + 追加编辑。
    /// 载荷分段（O）：{byte 标志@+3, edit 列表 ptr/len}。`truncate`/`edits` 为语义字段。
    ReplaceEdits {
        truncate: usize,
        edits: Vec<SerializedJson>,
    } = 6,
    /// 7 BackupSave：`history-auto` 前缀备份。
    /// `path`/`kind` 为语义字段；`remove_counter` = 节拍器返回的 retention 删除候选
    /// （O：BackupDecision.remove_counter → "Backup limit reached; removing " 删除旧备份）。
    BackupSave {
        path: PathBuf,
        kind: BackupSaveFileKind,
        remove_counter: Option<u32>,
    } = 7,
    /// 8 ManualBackup（execute_save_cmd 无专属 case，落 default）。
    /// 语义字段；原版 default 区含文件身份字节（+0x53/+0x299）比对与 "snapshot"
    /// 常量（0x746f687370616e73）校验（见 save_thread::perform_manual_backup）。
    /// `reply`（O：create_backup_of 0x141e59860 同步请求-应答）：可选应答通道，
    /// 存档线程执行完把 `Result<(),anyhow::Error>` 回发；无应答（游戏线程直发）为 None。
    ManualBackup {
        path: PathBuf,
        kind: BackupSaveFileKind,
        reply: Option<crossbeam_channel::Sender<Result<(), anyhow::Error>>>,
    } = 8,
    /// 9 CaptureSnapshot：快照 JSON 落盘。
    CaptureSnapshot { snapshot: Arc<SnapshotSaveData> } = 9,
    /// 10 Flush。
    Flush = 10,
    /// 11 Shutdown：需要时 flush_deferred_and_sync + 确认 + 退出线程。
    Shutdown { flush: bool } = 11,
}

impl AsyncSaveCmd {
    /// 原 `AsyncSaveCmd::should_trigger_debug_log`：带 debug 串的命令才需要
    /// 调试级日志门控（"AsyncSaveCmd::OpenFile(" / "AsyncSaveCmd::CloseFile("）。
    pub const fn should_trigger_debug_log(&self) -> bool {
        matches!(self, Self::OpenFile { .. } | Self::CloseFile { .. })
    }
}

/// 快照捕获请求（X4：save_game 0x140a3f470 触发时宿主构建 SnapshotSaveData）。
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct CaptureSnapshotRequested;

/// 同步保存载荷（宿主/域采集后注入；O：save_game 0x140a3f470 组装
/// `GameSaveData` 的域资源面）。
///
/// 原生 save_game 在函数内从 ~9 域资源组装 `GameSaveData`（History edits
/// `Cloned<Skip<Iter<MultiEdit>>>`、Sheep Query、Roof Query、Camera/Colors/ToD/…），
/// 再 `send<AsyncSaveCmd>`（OpenFile/ChangeSection/Flush）+ `SessionMeta::write_to_disk`
/// + `sync_channel` 备份请求。域 Query/资源尚未全部在本层注册，`sections`/`edits`
/// 由宿主采集后放入本资源；未注册时系统跳过同步保存面（并记录一次），不伪造节载荷。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct SaveGameSyncData {
    /// 完整 10 节载荷（缺省 = 宿主未接入）。
    pub sections: Option<GameSaveSections>,
    /// History 编辑（原生 `history.edits.iter().skip(1).cloned()` 的等价输入）。
    pub edits: Vec<SerializedJson>,
}

/// `save_game` 0x140a3f470 的 ECS 系统（本体组装，可接部分）。
///
/// fresh callees（2026-10-17）确立了本系统的真实调用链：
/// `add_active_game_time` → `consider_backing_up`（threshold `+0x148 < 60.0` 门、
/// clamp[0,8]、逐秒档位）→ **内联快照采集**（`current_snapshot` 0x140a84650 为空 →
/// log "Capturing a game snapshot" → `capture_game_snapshot_save_data` 0x1417a0430 →
/// Arc → `send<AsyncSaveCmd>`（tag 9 CaptureSnapshot）→ `set_current_snapshot`
/// 0x140a84760）→ 各节 `ChangeSection(5)`（值 != 缓存才发）→ **内联同步保存**
/// （History edits → `send<AsyncSaveCmd>`）→ `SessionMeta::write_to_disk` 0x140b463c0 →
/// `sync_channel<Result<(),anyhow::Error>>` 手动备份请求-应答 → 错误状态
/// `try_recv<AutosaveErrorState>` 吸干。
///
/// 域 Query（Sheep/Roof/History/TimeOfDay…）在本 crate 能取得的面直接采集；其余
/// 节载荷由 [`SaveGameSyncData`] 注入（不伪造）。
pub fn save_game_system(
    mut auto: ResMut<AutoSaveState>,
    time: Res<GameTime>,
    active: Res<crate::resources::session::ActiveSession>,
    colors: Res<GladeColorCustomization>,
    sheep: Res<SheepControl>,
    mut snapshot_state: ResMut<SnapshotCaptureState>,
    roofs: Query<&Roof, Without<RoofMarkedForDeletion>>,
    walls: Option<Res<crate::resources::walls::InnerWalls>>,
    terrain: Option<Res<crate::resources::terrain_editing::TerrainEditingData>>,
    stairs: Option<Res<crate::resources::stairs::stairs_state::StairsState>>,
    sync_data: Option<Res<SaveGameSyncData>>,
    save_thread: Option<ResMut<super::save_thread::SaveThreadHandle>>,
) {
    // ① 节拍器：阈值门（+0x148 < 60.0）在 metronome 累计块内。
    auto.metronome.add_active_game_time(time.delta);
    // ② 触发备份 → 内联快照采集 + 备份生产。
    if let Some(decision) = auto.metronome.consider_backing_up() {
        // O：`History::current_snapshot` 为空才捕获；否则仅 drop 既有 Arc。
        if should_capture_snapshot(&snapshot_state.snapshots, snapshot_state.cursor) {
            log::info!("Capturing a game snapshot");
            let domain = collect_snapshot_domain(
                walls.as_deref(),
                roofs.iter(),
                terrain.as_deref(),
                stairs.as_deref(),
            );
            let snapshot = capture_game_snapshot_save_data(
                active.id,
                snapshot_state.cursor as u64, // U：snapshot_id 源待①取证
                snapshot_state.cursor,
                domain.sorted(),
            );
            // 原生 `send<AsyncSaveCmd>` 后 unwrap_failed：发送失败 panic。
            auto.send_command_expect(AsyncSaveCmd::CaptureSnapshot {
                snapshot: snapshot.clone(),
            });
            let cursor = snapshot_state.cursor;
            record_snapshot(&mut snapshot_state.snapshots, cursor, snapshot);
        }
        // 备份生产（O：SPEC §3.1）：counter → `history-auto-{counter}.json`；
        // remove_counter → retention 删除候选（save_thread 执行删除）。
        let backup_path = active
            .id
            .backup_folder_path()
            .join(format!("history-auto-{}.json", decision.counter));
        auto.send_command(AsyncSaveCmd::BackupSave {
            path: backup_path,
            kind: BackupSaveFileKind::Auto,
            remove_counter: decision.remove_counter,
        });
    }
    // ③ 各节 cooldown 值变更 → ChangeSection(5)（可采集节：Colors/Sheep）。
    //    Camera/Version/TimeOfDay/chain_index/history_start/snapshots 的载荷由宿主
    //    （`SaveGameSyncData.sections` 的对应节）采集；本系统不伪造。
    emit_section_change_if_changed(
        &mut auto,
        SaveFileSection::Colors,
        SerializedJson::new(&*colors).expect("GladeColorCustomization serializes"),
    );
    emit_section_change_if_changed(
        &mut auto,
        SaveFileSection::Sheep,
        sheep_section_serialized(sheep.snapshot.as_ref()),
    );
    // ④ 内联同步保存（O：History edits → `send<AsyncSaveCmd>`）。域资源未接入
    //    （sections 缺省）时跳过并登记一次，不编造节载荷。
    if let Some(sync_data) = sync_data.as_deref() {
        if let Some(sections) = sync_data.sections.clone() {
            auto.send_command_expect(AsyncSaveCmd::OpenFile {
                session: active.id,
                sections,
                edits: sync_data.edits.clone(),
            });
        }
    }
    // ⑤ SessionMeta 落盘 + Flush(0xa)。
    if let Err(error) = active.meta.write_to_disk(active.id) {
        log::error!("Error writing session meta to disk: {error}");
    }
    auto.send_command(AsyncSaveCmd::Flush);
    // ⑥ 吸干存档线程回传的错误状态（O：try_recv<AutosaveErrorState> ×3）。
    if let Some(mut handle) = save_thread {
        handle.error_state.drain_pending();
    }
}

/// 快照域采集（save_game 0x140a3f470 的 Sheep/Roof/域资源面 + capture_snapshot
/// 系统共用）。域序列化器未就绪的节（Decorators/WallColors/GateColors/Raster/…）
/// 保持空并由宿主经 `section_values` 注入。
fn collect_snapshot_domain<'a>(
    walls: Option<&crate::resources::walls::InnerWalls>,
    roofs: impl Iterator<Item = &'a Roof>,
    terrain: Option<&crate::resources::terrain_editing::TerrainEditingData>,
    stairs: Option<&crate::resources::stairs::stairs_state::StairsState>,
) -> SnapshotDomainData {
    // 墙采集：inner_walls::all() → collect_walls（O：closure_env$1）。
    let walls = walls
        .map(|inner| {
            crate::systems::save::snapshot::collect_walls(
                inner.all().map(|(id, state)| (id, state.clone())),
            )
        })
        .unwrap_or_default();
    // 地形采集：TerrainEditingData（strokes + next_index；flate2 只用于栅格尾）。
    let terrain = terrain.cloned().unwrap_or_else(|| {
        crate::resources::terrain_editing::TerrainEditingData(
            tg_terrain::stroke::TerrainEditingData::default(),
        )
    });
    // 楼梯采集：backup_accurate 断言前置（原生 assert）；未 backup 的 StairsState
    // 不触发 panic（宿主备份完成后自动纳入）。
    let stairs = stairs
        .filter(|state| state.is_backed_up())
        .map(crate::systems::save::snapshot::take_stairs_snapshot);
    // 9 域收集器在 [`SnapshotDomainData::collect`] 内组装（O：0x1417a0430）。
    SnapshotDomainData::collect(
        std::iter::empty::<crate::systems::save::snapshot::SnapshotTreesEntry>(), // Trees
        walls,                     // ② inner_walls::all() → 0x141308b40
        std::iter::empty::<u64>(), // ③ HashSet<u64> → 0x141309840
        stairs,                    // ④ take_snapshot → 0x14157d200/0x140901120
        std::iter::empty::<(
            crate::resources::walls::decorator_storage::DecoratorStorageAddr,
            crate::resources::walls::decorator_storage::DecoratorAddrSnapshot,
        )>(), // ⑤ DecoratorStorage::take_snapshot
        (
            std::iter::empty::<(usize, crate::resources::color_id::ColorId)>(), // ⑥ WallColors
            std::iter::empty::<(usize, crate::resources::color_id::ColorId)>(), //    GateColors
            std::iter::empty::<(usize, crate::resources::color_id::ColorId)>(), //    Halftimber
        ),
        terrain,                                       // ⑦ Terrain strokes + heights
        std::iter::empty::<(&'static str, Vec<u8>)>(), // ⑧ Raster 6 组 fold
        roofs.cloned(),                                // ⑨ Roofs Query
    )
}

/// 单节 cooldown 值变更判定（O：save_game 节 cooldown 段 `if (section值!=缓存)`）。
/// 值变化才发 `ChangeSection(5)` 并更新缓存；cooldown 秒表值待取证，当前以变更即发
/// （不伪造表值）。发送失败仅记录（send_command 已 log）。
fn emit_section_change_if_changed(
    auto: &mut AutoSaveState,
    section: SaveFileSection,
    value: SerializedJson,
) {
    if auto.cached_sections[section as usize].as_ref() != Some(&value) {
        if auto.send_command(AsyncSaveCmd::ChangeSection {
            section,
            value: value.clone(),
        }) {
            auto.cached_sections[section as usize] = Some(value);
        }
    }
}

/// 注册 save_game 系统事件/资源。
pub fn register_save_game_events(world: &mut bevy_ecs::world::World) {
    world.init_resource::<bevy_ecs::event::Events<CaptureSnapshotRequested>>();
    world.init_resource::<SnapshotCaptureState>();
    world.init_resource::<SaveGameSyncData>();
}

// ---------------------------------------------------------------------------
// 快照捕获域内部分（O：save_game 0x140a3f470 触发后内联快照采集）
// ---------------------------------------------------------------------------

/// 快照捕获决策（O：`History::current_snapshot` 0x140a84650）——已解析快照链中
/// 与游标等链索引的快照存在则不捕获。调用方（宿主/系统）从 History 资源取
/// `resolved_snapshots` 与 `cursor` 喂入。
pub fn should_capture_snapshot<S>(snapshots: &[ResolvedHistorySnapshot<S>], cursor: i32) -> bool {
    current_snapshot(snapshots, cursor).is_none()
}

/// `History::set_current_snapshot` 0x140a84760：游标已有快照时 panic，否则有序插入。
pub fn record_snapshot<S>(snapshots: &mut Vec<ResolvedHistorySnapshot<S>>, cursor: i32, value: S) {
    set_current_snapshot(snapshots, cursor, value);
}

/// 快照链状态（save 域自持；O：`History::current_snapshot`/`set_current_snapshot`
/// 的链索引/游标语义）。宿主从 History 资源把 `resolved_snapshots`/`cursor` 同步到
/// 本资源（History 类型过大且未注册为 Resource，save 域不直接持 History，登记）。
/// `snapshot_id` 源 U（原生自快照链计数；当前取游标值，待①取证）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct SnapshotCaptureState {
    /// 已解析快照链（值 = 捕获的 [`SnapshotSaveData`]）。
    pub snapshots: Vec<ResolvedHistorySnapshot<Arc<SnapshotSaveData>>>,
    /// 当前游标（History ledger cursor）。
    pub cursor: i32,
}

/// 快照捕获系统（域内部分；O：save_game 0x140a3f470 触发后内联快照采集）。
///
/// 消费 `CaptureSnapshotRequested`（save_game 节拍器触发）→ `should_capture_snapshot`
/// 决策 → 采集域数据（`Query<&Roof, Without<RoofMarkedForDeletion>>` + Sheep +
/// GladeColorCustomization + **可选域资源：InnerWalls/TerrainEditingData/StairsState**）
/// → `capture_game_snapshot_save_data` → Arc → `AsyncSaveCmd::CaptureSnapshot(9)` 发送 →
/// `record_snapshot`（set_current_snapshot）。
///
/// 域内可采集节（roofs/sheep/colors + walls/terrain/stairs 域资源在场时）已组装进
/// [`SnapshotDomainData`]；Decorators（take_snapshot 空桶前置）/Trees/WallColors/
/// GateColors/Raster 的域序列化器未就绪，宿主经域资源采集后把 `section_values` 注入
/// （不伪造节载荷）。History::set_current_snapshot 由 `record_snapshot` 在
/// `SnapshotCaptureState` 上落地（宿主同步自 History）。可选资源以 `Option<Res>` 承载
/// ——宿主注册对应域资源后自动纳入，未注册不阻断快照。
pub fn capture_snapshot_system(
    mut auto: ResMut<AutoSaveState>,
    mut state: ResMut<SnapshotCaptureState>,
    active: Res<crate::resources::session::ActiveSession>,
    roofs: Query<&Roof, Without<RoofMarkedForDeletion>>,
    walls: Option<Res<crate::resources::walls::InnerWalls>>,
    terrain: Option<Res<crate::resources::terrain_editing::TerrainEditingData>>,
    stairs: Option<Res<crate::resources::stairs::stairs_state::StairsState>>,
    mut requested: bevy_ecs::event::EventReader<CaptureSnapshotRequested>,
) {
    for _ in requested.read() {
        // 决策（O：History::current_snapshot 为空才捕获；否则 drop 既有）。
        if !should_capture_snapshot(&state.snapshots, state.cursor) {
            continue;
        }
        // 域内可采集节（roofs + walls/terrain/stairs）组装；
        // Decorators/Trees/三色/Raster 的域序列化器未就绪，宿主经域资源采集后
        // 注入 section_values。
        let domain = collect_snapshot_domain(
            walls.as_deref(),
            roofs.iter(),
            terrain.as_deref(),
            stairs.as_deref(),
        );
        let snapshot = capture_game_snapshot_save_data(
            active.id,
            state.cursor as u64, // U：snapshot_id 源待①取证
            state.cursor,
            domain.sorted(),
        );
        if auto.send_command(AsyncSaveCmd::CaptureSnapshot {
            snapshot: snapshot.clone(),
        }) {
            let cursor = state.cursor;
            record_snapshot(&mut state.snapshots, cursor, snapshot);
        }
    }
}

// ---------------------------------------------------------------------------
// 事件 / 请求（PDB：`SaveGameCmd`、`DeleteSessionCmd`、`SavingGameComplete`）
// ---------------------------------------------------------------------------

/// 游戏线程同步保存请求（PDB：`save_game::SaveGameCmd`）。域资源组装见
/// `GameSaveData`；命令载荷在域恢复后接入。
#[derive(Clone, Debug)]
pub struct SaveGameCmd {
    pub data: GameSaveData,
}

/// 删除会话命令（PDB：`save_game::DeleteSessionCmd`）。`SessionFlowPlugin::build`
/// 0x140b47840 注册为事件（`register_event<DeleteSessionCmd>`）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct DeleteSessionCmd {
    pub session: SessionId,
}

/// 保存完成事件（PDB：`save_game::SavingGameComplete`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SavingGameComplete;

/// 同步存档路径的组装数据（PDB：`save_game::GameSaveData`）。原版由
/// save_game 0x140a3f470 从 ~9 资源拷贝组装（+0x28..+0x100 密集字段，**240B 完整字段名
/// U/待取证**——不编造）。`sections`/`edits` 为**自定语义名**（存档 codec 合同），
/// 非字节布局字段名；域资源未齐时组装系统留待对应域恢复后接入。
#[derive(Clone, Debug)]
pub struct GameSaveData {
    pub sections: GameSaveSections,
    pub edits: Vec<SerializedJson>,
}

// ---------------------------------------------------------------------------
// 自动存档状态（PDB：`AutoSaveState` / `AutoSaveSectionState` / `DeferredJsonSerialize`）
// ---------------------------------------------------------------------------

/// 每节存档状态（PDB：`save_game::AutoSaveSectionState`）。原版 10 节各带
/// cooldown；`save_cooldown_seconds` 具体表值待取证，这里只保留节级时间账。
#[derive(Clone, Copy, Debug, Default)]
pub struct AutoSaveSectionState {
    /// 该节上次落盘的游戏时间（秒）。
    pub last_saved_game_time: f32,
}

/// 延迟 JSON 序列化（PDB：`save_game::DeferredJsonSerialize`）。
/// 原版用于分帧序列化大节载荷；载荷本身由所属域提供。
#[derive(Clone, Debug)]
pub struct DeferredJsonSerialize {
    pub section: SaveFileSection,
    pub json: SerializedJson,
}

/// 自动存档状态资源（PDB：`save_game::AutoSaveState`）。
///
/// - `close_file` 0x140a43840：**先 `Sender::send<AsyncSaveCmd>`（载荷 CloseFile tag=4）**，
///   成功后清节状态/错误/延迟序列化（O：+0x120=0、drop +0x28 Option<HistoryChainSnapshots>、
///   +0x140=0、+0x144=0）；send 失败 **panic**（`core::result::unwrap_failed`）。
/// - `create_backup_of<PathBuf>` 0x141e59860：**同步请求-应答**——`sync_channel<Result<(),anyhow::Error>>`
///   → 发 AsyncSaveCmd（含应答 Sender）→ `recv` 结果 → Err 构 anyhow::Error，**返回 `Result`**。
/// - `last_error_state` 0x140a439a0：返回 +0x10 错误状态指针（语义 = 最近错误访问器）。
/// - 节拍器字段（counter@+0x140/累计@+0x144/阈值@+0x148）由 `SaveBackupMetronome` 持有；
///   阈值门在 `add_active_game_time` 累计块内（reviewer 复核）。原生 **无 `sections[10]`**
///   （节 cooldown 在系统侧）；本实现保留节级时间账供系统 cooldown 判定，语义为 Rust 建模。
#[derive(bevy_ecs::resource::Resource, Debug)]
pub struct AutoSaveState {
    pub metronome: SaveBackupMetronome,
    pub sections: [AutoSaveSectionState; 10],
    /// 当前活动会话（close_file 发送 CloseFile 载荷用；O：AutoSaveState 内持有）。
    pub session: SessionId,
    /// 各节上次发送的序列化值缓存（O：save_game 0x140a3f470 节 cooldown
    /// `if (section值!=缓存) { alloc→send ChangeSection }` 的缓存面；Rust 建模）。
    pub cached_sections: [Option<SerializedJson>; 10],
    /// Original add_save_load_events_and_resources shares this exact Arc with
    /// Async save and startup_session::is_session_ok_to_load.
    pub unsaved_files: Arc<super::save_thread::UnsavedFileList>,
    last_error: Option<anyhow::Error>,
    command_sender: Option<crossbeam_channel::Sender<AsyncSaveCmd>>,
}

impl Default for AutoSaveState {
    fn default() -> Self {
        Self {
            metronome: SaveBackupMetronome::default(),
            sections: [AutoSaveSectionState::default(); 10],
            // Inactive Rust representation: no live file identity exists yet.
            session: SessionId(uuid::Uuid::nil()),
            cached_sections: std::array::from_fn(|_| None),
            unsaved_files: Arc::new(super::save_thread::UnsavedFileList::default()),
            last_error: None,
            command_sender: None,
        }
    }
}

impl AutoSaveState {
    /// 0x140a43840：**先发送 CloseFile(4) 命令**（载荷 session = 本状态持会话），
    /// send 失败 **panic**（原生 `core::result::unwrap_failed`）；成功后清节状态、
    /// 最近错误与延迟序列化账（Rust 建模 +0x120/+0x28/+0x140/+0x144 清空语义）。
    ///
    /// **fresh disasm（2026-10-17）**：`local_a8 == 0xb` 为 send 成功判别，否则
    /// `unwrap_failed`；随后 `+0x120 = 0`、`+0x28` `Option<HistoryChainSnapshots>`
    /// drop 并写哨兵 `0x8000000000000000`、`+0x140 = 0`（counter）、
    /// `+0x144 = 0x501502f900000000`（累计 0.0 + 阈值 1e10）。节拍器重置见
    /// [`SaveBackupMetronome::reset_after_close`]。
    pub fn close_file(&mut self) {
        let sender = self
            .command_sender
            .as_ref()
            .expect("save command sender must be attached to close the active file");
        sender
            .send(AsyncSaveCmd::CloseFile {
                session: self.session,
            })
            .expect("sending AsyncSaveCmd::CloseFile");
        self.sections = [AutoSaveSectionState::default(); 10];
        self.cached_sections = std::array::from_fn(|_| None);
        self.last_error = None;
        // +0x140 / +0x144 / +0x148 重置（O：close_file 尾部常量写入）。
        self.metronome.reset_after_close();
    }

    /// 0x141e59860：**同步请求-应答**。建 `sync_channel<Result<(),anyhow::Error>>` →
    /// 发 ManualBackup 命令（带应答 Sender）→ 阻塞 `recv` → 返回 `Result`。
    /// send 失败或执行失败 → Err(anyhow::Error)（O：Err 经 `format_err` 构 ContextError）。
    pub fn create_backup_of(&mut self, path: PathBuf) -> Result<(), anyhow::Error> {
        let (reply_sender, reply_receiver) = crossbeam_channel::bounded(1);
        let command = AsyncSaveCmd::ManualBackup {
            path,
            kind: BackupSaveFileKind::Manual,
            reply: Some(reply_sender),
        };
        match self.command_sender.as_ref() {
            Some(sender) => {
                sender.send(command).map_err(|error| {
                    anyhow::anyhow!("Could not send async save command: {error}")
                })?;
            }
            None => {
                return Err(anyhow::anyhow!("save command sender not attached"));
            }
        }
        match reply_receiver.recv() {
            Ok(result) => result,
            Err(error) => Err(anyhow::anyhow!(
                "save thread disconnected while creating backup: {error}"
            )),
        }
    }

    /// 0x140a439a0：**返回 `+0x10` 错误状态字段的引用**（fresh disasm
    /// `LEA RAX,[RCX+0x10]; RET`，4 字节访问器）。本实现以引用返回错误状态
    /// 字段（Rust 无原生字节偏移；访问器语义 = 暴露最近错误状态存储）。
    pub fn last_error_state(&self) -> &Option<anyhow::Error> {
        &self.last_error
    }

    /// 绑定存档线程命令发送端（由存档初始化者装配）。
    pub fn attach_command_sender(&mut self, sender: crossbeam_channel::Sender<AsyncSaveCmd>) {
        self.command_sender = Some(sender);
    }

    /// 是否已装配命令发送端（init_save_thread 幂等门）。
    pub fn is_command_attached(&self) -> bool {
        self.command_sender.is_some()
    }

    /// 发送一个异步存档命令；发送失败（线程已退出）记录错误并返回 false。
    pub fn send_command(&mut self, command: AsyncSaveCmd) -> bool {
        match self.command_sender.as_ref() {
            Some(sender) => match sender.send(command) {
                Ok(()) => true,
                Err(error) => {
                    log::error!("Could not send async save command: {error}");
                    self.last_error = Some(anyhow::anyhow!(error));
                    false
                }
            },
            None => {
                self.last_error = Some(anyhow::anyhow!("save command sender not attached"));
                false
            }
        }
    }

    /// 发送命令并在失败时 **panic**（O：save_game 0x140a3f470 的
    /// `Sender::send` 后 `core::result::unwrap_failed`；close_file 同）。
    pub fn send_command_expect(&mut self, command: AsyncSaveCmd) {
        let sender = self
            .command_sender
            .as_ref()
            .expect("save command sender must be attached");
        sender
            .send(command)
            .expect("sending AsyncSaveCmd from save_game");
    }
}

/// 会话 meta 落盘（B 0x1417522e0 `save_session_meta_to_file<PathBuf>`）：
/// 确保父目录 → 写 `*.tmp` → sync → rename 到目标。与 `SessionMeta::write_to_path`
/// 同一 tmp+rename 协议；此入口由保存系统直接调用。
pub fn save_session_meta_to_file(path: &Path, meta: &SessionMeta) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("tmp");
    let data = serde_json::to_string(meta)?;
    let mut file = std::fs::File::create(&temporary)?;
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    drop(file);
    std::fs::rename(temporary, path)?;
    Ok(())
}

/// Identity 节载荷 = 裸 u64 十进制（O：`to_json_string<HistoryIdentity>` 0x140b385e0）。
pub fn identity_to_json_string(identity: u64) -> SerializedJson {
    SerializedJson::new(&identity).expect("u64 is always serializable")
}

/// Sheep 节载荷（O `to_json_string<Option<SheepControlSnapshot>>` @0x140a51d40；
/// B 组接口 `sheep::control::sheep_section_json`，None → null）。真实 producer 接线：
/// save_game 同步保存路径的域采集器（X4 Query）经 `capture_sheep_snapshot` 取行后
/// 调本入口；`None` = 无羊 → 节载荷 `null`。
pub fn sheep_section_serialized(
    snapshot: Option<&crate::resources::critters::sheep::control::SheepControlSnapshot>,
) -> SerializedJson {
    SerializedJson::from_raw_string(
        crate::resources::critters::sheep::control::sheep_section_json(snapshot),
    )
}

/// Sheep 节生产资源（⑤轮 B 块，① b-sheep-write-path 契约 §1/§4）：`sheep_controls`
/// 捕获活羊 → 本资源承载序列化的 Sheep 节载荷（`SaveFileSection::Sheep` = 5）。
/// 宿主 `assemble_game_save_sections` 从本资源取 Sheep 节真实值；默认 `None` → `null`。
/// X4 活羊 Query 精确面待①取证（b 契约 §3），现以 debug 域 `sheep_controls` 捕获为
/// crate 内真实生产调用点。
#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct SheepSaveSectionValue(pub SerializedJson);
impl Default for SheepSaveSectionValue {
    fn default() -> Self {
        Self(sheep_section_serialized(None))
    }
}

/// Sheep 节生产系统：`SheepControl`（`sheep_controls` 捕获）→ `sheep_section_serialized`
/// → [`SheepSaveSectionValue`]。真实生产调用点（⑤ B 块；与 `sheep_controls` 同批进
/// 生产 Schedule）。
pub fn sheep_save_section_system(
    control: Res<SheepControl>,
    mut value: ResMut<SheepSaveSectionValue>,
) {
    value.0 = sheep_section_serialized(control.snapshot.as_ref());
}

/// `save_game` 0x140a3f470 的 GameSaveData 组装（O：Identity/Camera/Colors 三节由
/// `HistoryIdentity`/`CameraRigParametersCache`/`GladeColorCustomization` 直接串化，
/// re-analyst 域类型证据包）。其余节载荷由所属域提供；原版每节必写，缺失即 panic，
/// 不伪造。
pub fn assemble_game_save_sections(
    version: i32,
    identity: HistoryIdentity,
    camera: &CameraRigParametersCache,
    colors: &GladeColorCustomization,
    time_of_day: SerializedJson,
    sheep: SerializedJson,
    chain_index: SerializedJson,
    history_start: SerializedJson,
    snapshots: SerializedJson,
) -> GameSaveSections {
    GameSaveSections {
        values: [
            // Version（0）：存档版本（load_game 版本门控消费）。
            SerializedJson::new(&version).expect("i32 serializes"),
            // Identity（1）：HistoryIdentity 裸 u64 十进制。
            SerializedJson::new(&identity).expect("HistoryIdentity serializes"),
            // BackwardCompatibility（2）：0。
            SerializedJson::new(&0u8).expect("u8 serializes"),
            // Camera（3）：CameraRigParametersCache 6 字段。
            SerializedJson::new(camera).expect("CameraRigParametersCache serializes"),
            time_of_day,
            sheep,
            // Colors（6）：GladeColorCustomization 15 字段。
            SerializedJson::new(colors).expect("GladeColorCustomization serializes"),
            chain_index,
            history_start,
            snapshots,
        ],
    }
}

/// 为需要自定义 `Serialize` 的域载荷提供统一入口；参数即序列化结果，不做二次猜测。
pub fn serialize_value<T: Serialize + ?Sized>(value: &T) -> SerializedJson {
    SerializedJson::new(value).expect("value must serialize")
}

/// serde 辅助：把 `SerializedJson` 当字符串写出（供域载荷组合 JSON 时复用）。
pub fn serialize_serialized_json<S: Serializer>(
    value: &SerializedJson,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.as_str())
}

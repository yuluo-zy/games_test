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
use crate::resources::critters::{
    sheep::{control::capture_sheep_snapshot, Sheep},
    CritterGoal,
};
use crate::resources::glade_color_customization::GladeColorCustomization;
use crate::resources::history::snapshots::current_snapshot;
use crate::resources::history::snapshots::set_current_snapshot;
use crate::resources::history::transactions::ResolvedHistorySnapshot;
use crate::resources::history::HistoryIdentity;
use crate::resources::roofs::roof_animation::RoofMarkedForDeletion;
use crate::resources::roofs::roof_shape::Roof;
pub use crate::resources::save_file::SaveFileSection;
use crate::resources::session::SessionId;
use crate::resources::session::SessionMeta;
use crate::resources::time::GameTime;
use crate::systems::camera_rig::CameraRigParametersCache;
use crate::systems::save::save_backup_list::BackupSaveFileKind;
use crate::systems::save::save_backup_metronome::SaveBackupMetronome;
use crate::systems::save::snapshot::capture_game_snapshot_save_data;
use crate::systems::save::snapshot::SnapshotDomainData;
use crate::systems::save::snapshot::SnapshotSaveData;
use crate::utils::components::transform::Transform;
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

/// 游戏线程 → 存档线程的命令。原版已观测业务变体标签 **3..=10**（① r6-async-savecmd 契约 §1，
/// ALGORITHMS §2.1 save_thread 0x141720680 / 分发 0x141721aa0 权威；0/1/2 未在 switch
/// 出现 = 死/历史变体）。判别值显式对齐：3 OpenFile / 4 CloseFile / 9 CaptureSnapshot /
/// 10 Flush (0xb belongs to outer TryRecvError, not AsyncSaveCmd)；5/6/7/8 = ReplaceEdits/ChangeSection/BackupSave/ManualBackup。
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
    /// 6 ChangeSection：改节 + mark_dirty。
    /// 载荷分段（O）：param_1[1..2]（+8/+0x10）、[3..4]（+0x18/+0x20）。
    /// `section`/`value` 为语义字段（change_section 行为合同），字节布局名待取证。
    ChangeSection {
        section: SaveFileSection,
        value: DeferredJsonValue,
    } = 6,
    /// 5 ReplaceEdits：截断 + 追加编辑。
    /// 载荷分段（O）：{byte 标志@+3, edit 列表 ptr/len}。`truncate`/`edits` 为语义字段。
    ReplaceEdits {
        truncate: usize,
        edits: Vec<SerializedJson>,
    } = 5,
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
    CaptureSnapshot {
        snapshot: Arc<SnapshotSaveData>,
        pending: crate::resources::history::MaybePendingSnapshotId,
    } = 9,
    /// Original tag10 carries a one-result channel. The worker applies all
    /// deferred sections, syncs the actual shadow file, then returns success/error.
    Flush {
        reply: crossbeam_channel::Sender<Result<(), anyhow::Error>>,
    } = 10,
}

impl AsyncSaveCmd {
    /// 原 `AsyncSaveCmd::should_trigger_debug_log`：带 debug 串的命令才需要
    /// 调试级日志门控（"AsyncSaveCmd::OpenFile(" / "AsyncSaveCmd::CloseFile("）。
    pub const fn should_trigger_debug_log(&self) -> bool {
        matches!(self, Self::OpenFile { .. } | Self::CloseFile { .. })
    }
}

/// B `SaveGameReplayModTracker`: previous pop-attempt counter, distinct from
/// queue length. It is updated even on frames where queued replay prevents capture.
#[derive(Default)]
pub struct SaveGameReplayModTracker {
    previous_pop_counter: u64,
}

/// Immutable projection of the six existing CPU raster owners. It adds no
/// second raster authority and borrows the same composite selected by `read`.
#[derive(bevy_ecs::system::SystemParam)]
pub struct SaveRasterRead<'w> {
    path: Res<'w, crate::resources::world_raster_defs::PathRaster>,
    cobblestone1: Res<'w, crate::resources::world_raster_defs::Cobblestone1Raster>,
    cobblestone2: Res<'w, crate::resources::world_raster_defs::Cobblestone2Raster>,
    cobblestone3: Res<'w, crate::resources::world_raster_defs::Cobblestone3Raster>,
    water: Res<'w, crate::resources::world_raster_defs::WaterRaster>,
    garden: Res<'w, crate::resources::world_raster_defs::GardenRaster>,
}
impl SaveRasterRead<'_> {
    fn capture(&self) -> Vec<(&'static str, Vec<u8>)> {
        crate::resources::world_raster_defs::AllWorldRasterRead {
            path: &self.path,
            cobblestone1: &self.cobblestone1,
            cobblestone2: &self.cobblestone2,
            cobblestone3: &self.cobblestone3,
            water: &self.water,
            garden: &self.garden,
        }
        .read()
        .map(|(name, raster)| (name, raster.data.to_vec()))
        .into_iter()
        .collect()
    }
}

/// Same live owners as the original 16-param first tuple. Snapshot collection
/// is inline on the save decision; there is no separately synchronized History
/// cursor, optional-domain fallback or injected JSON resource.
#[derive(bevy_ecs::system::SystemParam)]
pub struct SaveSnapshotWorld<'w, 's> {
    walls: Res<'w, crate::resources::walls::InnerWalls>,
    roofs: Query<'w, 's, &'static Roof, Without<RoofMarkedForDeletion>>,
    terrain: Res<'w, crate::resources::terrain_editing::TerrainEditingData>,
    heights: Res<
        'w,
        crate::resources::terrain_height::TerrainHeightsData<
            crate::resources::terrain_height::TerrainHeightSsbo,
        >,
    >,
    decorators: Res<'w, crate::resources::walls::decorator_storage::DecoratorStorage>,
    wall_colors: Res<'w, crate::resources::color_id::WallColorIds>,
    gate_colors: Res<'w, crate::resources::color_id::GateColorIds>,
    halftimber_colors: Res<'w, crate::resources::color_id::HalftimberColorIds>,
    trees: Query<
        'w,
        's,
        (
            &'static crate::systems::prefab::PrefabInstanceData,
            &'static crate::resources::prefab::PrefabInstance,
            &'static crate::components::trees::TreeComponent,
        ),
    >,
    clutter: Res<'w, crate::systems::ui_clutter::DeletedAutoClutter>,
    raster: SaveRasterRead<'w>,
    stairs: Res<'w, crate::resources::stairs::stairs_state::StairsState>,
    removed_supports:
        Res<'w, crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,
}
impl SaveSnapshotWorld<'_, '_> {
    fn capture(&self) -> SnapshotDomainData {
        use super::snapshot::SnapshotTreesEntry;
        use crate::resources::color_id::ColorId;
        use crate::resources::history::tree_edit::TreeEditData;
        use crate::resources::tree_storage::TreeStorageIndex;
        let trees: Vec<SnapshotTreesEntry> = self
            .trees
            .iter()
            .map(|(data, instance, tree)| {
                let placement = &instance.0;
                SnapshotTreesEntry {
                    index: TreeStorageIndex(tree.id),
                    tree: TreeEditData {
                        storage_index: Some(TreeStorageIndex(tree.id)),
                        position: placement.position,
                        scale: placement.scale,
                        rotation_degrees: placement.rotation_angle,
                        id: placement.id,
                        prefab_name: data.prefab_name.as_str().to_owned(),
                        is_preplaced: tree.is_preplaced,
                        // This runtime-only field is not part of TreeData's six
                        // serialized entries; a captured current tree has no move.
                        previous_position: placement.position,
                    },
                }
            })
            .collect();
        SnapshotDomainData::collect(
            trees,
            super::snapshot::collect_walls(self.walls.all().map(|(id, wall)| (id, wall.clone()))),
            self.clutter.0.iter().copied(),
            Some(super::snapshot::take_stairs_snapshot(&self.stairs)),
            self.decorators.take_snapshot().states,
            (
                self.wall_colors
                    .iter_nondefault()
                    .map(|(i, value)| (i, ColorId(value))),
                self.gate_colors
                    .iter_nondefault()
                    .map(|(i, value)| (i, ColorId(value))),
                self.halftimber_colors
                    .iter_nondefault()
                    .map(|(i, value)| (i, ColorId(value))),
            ),
            (*self.terrain).clone(),
            self.raster.capture(),
            self.roofs.iter().cloned(),
        )
        .with_terrain_and_supports(
            // The current canonical CPU owner packs two u16 cells in each
            // native u32 word. Preserve its exact 0x80000B allocation bytes;
            // no height conversion, zero fill or stroke-derived substitute.
            bytemuck::cast_slice(self.heights.heights()).to_vec(),
            self.heights.min_height()..self.heights.max_height(),
            (*self.removed_supports).clone(),
        )
        .sorted()
    }
}

#[derive(bevy_ecs::system::SystemParam)]
pub struct SaveFrameWorld<'w, 's> {
    history: ResMut<'w, crate::resources::history::History>,
    auto: ResMut<'w, AutoSaveState>,
    active: ResMut<'w, crate::resources::session::ActiveSession>,
    camera: Res<'w, crate::systems::camera_rig::main_camera::MainCamera>,
    tod: Res<'w, crate::resources::time_of_day::TimeOfDay>,
    colors: Res<'w, GladeColorCustomization>,
    time: Res<'w, GameTime>,
    replay: Res<'w, crate::resources::history::ReplayBuffer>,
    bulldoze:
        Res<'w, crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,
    image: Res<'w, crate::resources::image_formation::ImageFormationConfigs>,
    sheep: Query<'w, 's, (&'static Transform, &'static Sheep, &'static CritterGoal)>,
    save_thread: ResMut<'w, super::save_thread::SaveThreadHandle>,
}

/// B `save_game` @140a3f470, 17,300 bytes. Registered in numeric set43,
/// conditioned only on the original Option<Res<ActiveSession>> presence gate.
/// Save/close requests are consumed regardless of whether capture is stable;
/// the worker's synchronous flush result decides whether completion is emitted.
pub fn save_game_system(
    mut frame: SaveFrameWorld,
    snapshot_world: SaveSnapshotWorld,
    mut save_requests: bevy_ecs::event::EventReader<SaveGameCmd>,
    mut shutdown_requests: bevy_ecs::event::EventReader<crate::resources::events::ShutdownGameCmd>,
    mut complete: bevy_ecs::event::EventWriter<SavingGameComplete>,
    mut replay_tracker: bevy_ecs::system::Local<SaveGameReplayModTracker>,
) {
    frame.save_thread.error_state.drain_pending();
    publish_history_edits(&mut frame.history, &mut frame.auto);
    let stored = frame.camera.store_parameters();
    let camera = CameraRigParametersCache {
        yaw: stored.yaw,
        pitch: stored.pitch,
        position: stored.position,
        camera_rig_arm_length: stored.camera_rig_arm_length,
        vertical_fov_degrees: stored.vertical_fov_degrees,
        pitch_offset: stored.pitch_offset,
        yaw_offset: stored.yaw_offset,
    };
    let sheep = Some(capture_sheep_snapshot(frame.sheep.iter()));
    let pop_counter = frame.replay.pop_counter();
    let stable = frame.replay.is_empty()
        && replay_tracker.previous_pop_counter == pop_counter
        && frame.bulldoze.0.is_empty()
        && !frame.history.is_recording();
    replay_tracker.previous_pop_counter = pop_counter;
    let backup = if frame.auto.metronome.active_game_threshold() < 60.0 {
        frame.auto.metronome.add_active_game_time(frame.time.delta);
        if stable {
            frame.auto.metronome.consider_backing_up()
        } else {
            None
        }
    } else {
        None
    };
    // Do not use short-circuit || between readers: both original cursors move
    // to the end even when the other queue already contains a request.
    let explicit_save = save_requests.read().count() != 0;
    let explicit_shutdown = shutdown_requests.read().count() != 0;
    let requested = explicit_save || explicit_shutdown;
    if stable
        && (backup.is_some() || requested)
        && current_snapshot(
            &frame.history.ledger().resolved_snapshots,
            frame.history.ledger().cursor,
        )
        .is_none()
    {
        log::info!("Capturing a game snapshot");
        let cursor = frame.history.ledger().cursor;
        let pending = crate::resources::history::MaybePendingSnapshotId::new();
        let snapshot =
            capture_game_snapshot_save_data(frame.active.id, 0, cursor, snapshot_world.capture());
        frame
            .auto
            .send_command_expect(AsyncSaveCmd::CaptureSnapshot {
                snapshot,
                pending: pending.clone(),
            });
        set_current_snapshot(
            &mut frame.history.ledger_mut().resolved_snapshots,
            cursor,
            pending,
        );
    }
    publish_save_sections(
        &frame.history,
        &mut frame.auto,
        camera,
        &frame.tod,
        sheep,
        *frame.colors,
    );
    if let Some(decision) = backup {
        let path = frame
            .active
            .id
            .backup_folder_path()
            .join(format!("history-auto-{}.json", decision.counter));
        frame.auto.send_command_expect(AsyncSaveCmd::BackupSave {
            path,
            kind: BackupSaveFileKind::Auto,
            remove_counter: decision.remove_counter,
        });
    }
    if requested {
        let current = frame.image.current();
        frame.active.meta.image = if image_settings_equal(current, &frame.image.configs[0]) {
            None
        } else {
            Some(current.clone())
        };
        if let Err(error) = frame.active.meta.write_to_disk(frame.active.id) {
            log::error!("Error writing session meta to disk: {error}");
        }
        match frame.auto.flush_synchronously() {
            Ok(()) => {
                complete.write(SavingGameComplete {
                    path: frame.active.id.save_folder_path(),
                });
            }
            Err(error) => {
                log::error!("Error while performing a synchronous save: {error:#}");
            }
        }
    }
}

/// Native save set's event identities, maintained by the App's real Bevy
/// registry. Idempotency never initializes an unregistered Events container.
pub fn register_save_game_events(world: &mut bevy_ecs::world::World) {
    fn register<T: bevy_ecs::event::Event>(world: &mut bevy_ecs::world::World) {
        if !world.contains_resource::<bevy_ecs::event::Events<T>>() {
            bevy_ecs::event::EventRegistry::register_event::<T>(world);
        }
    }
    register::<SaveGameCmd>(world);
    register::<SavingGameComplete>(world);
}

/// Canonical ledger helper retained for existing domain tests; no mirrored
/// SnapshotCaptureState resource is registered or updated.
pub fn should_capture_snapshot<S>(snapshots: &[ResolvedHistorySnapshot<S>], cursor: i32) -> bool {
    current_snapshot(snapshots, cursor).is_none()
}
pub fn record_snapshot<S>(snapshots: &mut Vec<ResolvedHistorySnapshot<S>>, cursor: i32, value: S) {
    set_current_snapshot(snapshots, cursor, value);
}

// ---------------------------------------------------------------------------
// 事件 / 请求（PDB：`SaveGameCmd`、`DeleteSessionCmd`、`SavingGameComplete`）
// ---------------------------------------------------------------------------

/// 游戏线程同步保存请求（PDB：`save_game::SaveGameCmd`）。域资源组装见
/// `GameSaveData`；命令载荷在域恢复后接入。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SaveGameCmd;

/// 删除会话命令（PDB：`save_game::DeleteSessionCmd`）。`SessionFlowPlugin::build`
/// 0x140b47840 注册为事件（`register_event<DeleteSessionCmd>`）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct DeleteSessionCmd {
    pub session: SessionId,
}

/// 保存完成事件（PDB：`save_game::SavingGameComplete`）。
#[derive(bevy_ecs::event::Event, Clone, Debug, PartialEq, Eq)]
pub struct SavingGameComplete {
    pub path: PathBuf,
}

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

/// B trait object payload of AsyncSaveCmd::ChangeSection. Serialization occurs
/// on the save worker, after all preceding commands (especially tag9) finish.
pub trait DeferredJsonSerialize: Send + Sync + core::fmt::Debug {
    fn to_json_string(&self) -> SerializedJson;
}
impl<T> DeferredJsonSerialize for T
where
    T: Serialize + Send + Sync + core::fmt::Debug,
{
    fn to_json_string(&self) -> SerializedJson {
        SerializedJson::new(self).expect("serializing an owned save section")
    }
}
pub type DeferredJsonValue = Arc<dyn DeferredJsonSerialize>;
pub fn defer_json<T>(value: T) -> DeferredJsonValue
where
    T: Serialize + Send + Sync + core::fmt::Debug + 'static,
{
    Arc::new(value)
}

/// Owns clones of the same atomic IDs as the canonical History ledger. The
/// worker reads them only when handling this queued section after capture.
#[derive(Clone, Debug)]
pub struct SnapshotReferencesToJson(pub Vec<crate::resources::history::HistoryChainSnapshot>);
impl Serialize for SnapshotReferencesToJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::resources::history::snapshots::resolved_snapshot_references(&self.0)
            .serialize(serializer)
    }
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
    last_saved_edit_count: usize,
    frame_cache: Option<AutoSaveFrameCache>,
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
            last_saved_edit_count: 0,
            frame_cache: None,
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
        self.last_saved_edit_count = 0;
        self.frame_cache = None;
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

    /// B save_game@140a419b5..41a51: bound1 reply channel, tag10,
    /// wait for the actual worker result. Send/receive disconnects panic as in
    /// the original; a file flush error is returned for the caller to log.
    pub fn flush_synchronously(&mut self) -> Result<(), anyhow::Error> {
        let (reply, result) = crossbeam_channel::bounded(1);
        self.send_command_expect(AsyncSaveCmd::Flush { reply });
        result
            .recv()
            .expect("receiving synchronous save completion")
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
/// 调本入口。实时 Query 捕获即使零行也为 Some(empty) → {"sheep":[]}；
/// None 是 wire 中明确的缺省快照 → null，不能拿 debug freeze 状态替代活羊。
pub fn sheep_section_serialized(
    snapshot: Option<&crate::resources::critters::sheep::control::SheepControlSnapshot>,
) -> SerializedJson {
    SerializedJson::from_raw_string(
        crate::resources::critters::sheep::control::sheep_section_json(snapshot),
    )
}

/// Sheep section staged by its live-query producer. The initial None is an
/// unproduced value; the producer runs before save-section assembly and writes
/// Some(empty) for a world with no matching live sheep.
#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct SheepSaveSectionValue(pub SerializedJson);
impl Default for SheepSaveSectionValue {
    fn default() -> Self {
        Self(sheep_section_serialized(None))
    }
}

/// Snapshot current live sheep independently of the debug freeze owner.
pub fn sheep_save_section_system(
    sheep: Query<(&Transform, &Sheep, &CritterGoal)>,
    mut value: ResMut<SheepSaveSectionValue>,
) {
    let snapshot = capture_sheep_snapshot(sheep.iter());
    value.0 = sheep_section_serialized(Some(&snapshot));
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

/// Native AutoSaveState typed comparison cache, kept in the existing owner.
/// This contains shared snapshot ID owners only; captured world data is never
/// copied into a second History resource.
#[derive(Clone, Debug)]
struct AutoSaveFrameCache {
    identity: u64,
    camera: CameraRigParametersCache,
    tod: crate::resources::time_of_day::TimeOfDay,
    sheep: Option<crate::resources::critters::sheep::control::SheepControlSnapshot>,
    colors: GladeColorCustomization,
    snapshots: Vec<crate::resources::history::HistoryChainSnapshot>,
    chain_index: i32,
    history_start: i32,
}

/// B @140a3f9fb..faf2: write the changed suffix from the actual committed
/// ledger, then advance both saved-count observations only after send succeeds.
fn publish_history_edits(
    history: &mut crate::resources::history::History,
    auto: &mut AutoSaveState,
) {
    let ledger = history.ledger();
    let count = ledger.entries.len();
    let truncate = ledger.observed_count_58;
    if truncate == count && auto.last_saved_edit_count == count {
        return;
    }
    assert!(
        truncate <= count,
        "saved history prefix exceeds committed entries"
    );
    let edits = ledger.entries[truncate..]
        .iter()
        .cloned()
        .map(crate::resources::history::serialized::SavedMultiEdit)
        .map(|entry| SerializedJson::new(&entry).expect("serializing actual committed MultiEdit"))
        .collect();
    auto.send_command_expect(AsyncSaveCmd::ReplaceEdits { truncate, edits });
    auto.metronome.set_active_game_threshold(0.0);
    history.ledger_mut().observed_count_58 = count;
    auto.last_saved_edit_count = count;
}

/// B @140a41305..373: pending or invalid IDs force a fresh section command.
/// Resolved IDs compare by value and chain index; Arc pointer equality is only
/// an original optimization and is not needed for this same-owner projection.
fn snapshot_owners_equal(
    left: &[crate::resources::history::HistoryChainSnapshot],
    right: &[crate::resources::history::HistoryChainSnapshot],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
            let a = left.value.state();
            let b = right.value.state();
            left.chain_index == right.chain_index && a >= 2 && b >= 2 && a == b
        })
}

/// Original change order: Identity, Camera, TOD, Sheep, Colors, Snapshots,
/// chain_index, history_start. Initial invocation installs a baseline cache;
/// the already-open save owns the initial sections. No per-frame OpenFile.
fn publish_save_sections(
    history: &crate::resources::history::History,
    auto: &mut AutoSaveState,
    camera: CameraRigParametersCache,
    tod: &crate::resources::time_of_day::TimeOfDay,
    sheep: Option<crate::resources::critters::sheep::control::SheepControlSnapshot>,
    colors: GladeColorCustomization,
) {
    let ledger = history.ledger();
    let current = AutoSaveFrameCache {
        identity: ledger.mutation_nonce,
        camera,
        tod: tod.clone(),
        sheep,
        colors,
        snapshots: ledger.resolved_snapshots.clone(),
        chain_index: ledger.cursor,
        history_start: ledger.undo_anchor,
    };
    if let Some(previous) = auto.frame_cache.take() {
        if previous.identity != current.identity {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::Identity,
                value: defer_json(HistoryIdentity(current.identity)),
            });
            auto.metronome.set_active_game_threshold(0.0);
        }
        if previous.camera != current.camera {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::Camera,
                value: defer_json(current.camera),
            });
        }
        if previous.tod != current.tod {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::TimeOfDay,
                value: defer_json(current.tod.clone()),
            });
        }
        if previous.sheep != current.sheep {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::Sheep,
                value: defer_json(current.sheep.clone()),
            });
        }
        if previous.colors != current.colors {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::Colors,
                value: defer_json(current.colors),
            });
        }
        if !snapshot_owners_equal(&previous.snapshots, &current.snapshots) {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::Snapshots,
                value: defer_json(SnapshotReferencesToJson(current.snapshots.clone())),
            });
        }
        if previous.chain_index != current.chain_index {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::ChainIndex,
                value: defer_json(current.chain_index),
            });
            auto.metronome.set_active_game_threshold(0.0);
        }
        if previous.history_start != current.history_start {
            auto.send_command_expect(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::HistoryStart,
                value: defer_json(current.history_start),
            });
            auto.metronome.set_active_game_threshold(0.0);
        }
    }
    auto.frame_cache = Some(current);
}

/// Native SessionMeta image comparison ignores alignment padding and compares
/// word_38 as f32 (not integer bits). Thus NaN differs and signed zero equals.
fn image_settings_equal(
    left: &crate::resources::image_formation::ImageFormationConfig,
    right: &crate::resources::image_formation::ImageFormationConfig,
) -> bool {
    left.name == right.name
        && left.flag_28 == right.flag_28
        && left.params_18 == right.params_18
        && left.params_30 == right.params_30
        && f32::from_bits(left.word_38) == f32::from_bits(right.word_38)
        && left.flag_3c == right.flag_3c
        && left.flag_3d == right.flag_3d
}

#[cfg(test)]
mod original_incremental_tests {
    use super::*;
    #[test]
    fn current_history_edits_append_to_the_real_file_without_reopening() {
        use super::super::save_thread::{execute_save_cmd, CurrentSaveFile, UnsavedFileList};
        use crate::resources::history::{
            serialized::SavedMultiEdit, transactions::ObservedHistoryLedger, History,
        };
        let original: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-intro-history-b.json"
        ))
        .unwrap();
        let entries: Vec<SavedMultiEdit> =
            serde_json::from_value(original["History"]["edit_chain"].clone()).unwrap();
        let sections = GameSaveSections {
            values: SaveFileSection::ALL.map(|section| {
                let owner = if section as u8 >= SaveFileSection::ChainIndex as u8 {
                    &original["History"]
                } else {
                    &original
                };
                SerializedJson::new(&owner[section.json_key()]).unwrap()
            }),
        };
        let serialized = entries
            .iter()
            .map(|entry| SerializedJson::new(entry).unwrap())
            .collect::<Vec<_>>();
        let root = tempfile::tempdir().unwrap();
        let mut current = Some(CurrentSaveFile::create(root.path(), sections, &serialized));
        let mut history: History = History::from_observed(ObservedHistoryLedger {
            entries: entries.into_iter().map(|entry| entry.0).collect(),
            cursor: 12,
            undo_anchor: -1,
            resolved_snapshots: Vec::new(),
            observed_count_58: 13,
            mutation_nonce: 17,
        });
        let actual_next = history.ledger().entries.last().unwrap().clone();
        history.ledger_mut().entries.push(actual_next);
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut auto = AutoSaveState::default();
        auto.attach_command_sender(sender);
        publish_history_edits(&mut history, &mut auto);
        let command = receiver.try_recv().unwrap();
        assert!(
            matches!(&command, AsyncSaveCmd::ReplaceEdits {truncate:13,edits} if edits.len()==1)
        );
        execute_save_cmd(&mut current, command, &UnsavedFileList::default());
        assert_eq!(history.ledger().observed_count_58, 14);
        publish_history_edits(&mut history, &mut auto);
        assert!(receiver.try_recv().is_err());
        assert!(current.as_mut().unwrap().flush_deferred_and_sync());
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(root.path().join("history.json")).unwrap())
                .unwrap();
        assert_eq!(saved["History"]["edit_chain"].as_array().unwrap().len(), 14);
        assert_eq!(
            saved["History"]["edit_chain"][13],
            saved["History"]["edit_chain"][12]
        );
    }

    #[test]
    fn pending_snapshot_cache_requires_late_worker_serialization() {
        use crate::resources::history::{
            transactions::ResolvedHistorySnapshot, HistorySnapshotId, MaybePendingSnapshotId,
        };
        let owner = MaybePendingSnapshotId::new();
        let left = vec![ResolvedHistorySnapshot {
            chain_index: 2,
            value: owner.clone(),
        }];
        let right = left.clone();
        assert!(!snapshot_owners_equal(&left, &right));
        owner.assign_invalid();
        assert!(!snapshot_owners_equal(&left, &right));
        owner.assign(HistorySnapshotId(73));
        assert!(snapshot_owners_equal(&left, &right));
        let other = vec![ResolvedHistorySnapshot {
            chain_index: 2,
            value: MaybePendingSnapshotId::from_snapshot_id(HistorySnapshotId(73)),
        }];
        assert!(snapshot_owners_equal(&left, &other));
    }

    #[test]
    fn session_image_equality_ignores_padding_and_uses_float_comparisons() {
        use crate::resources::image_formation::ImageFormationConfig;
        let a = ImageFormationConfig::default_plain();
        let mut b = a.clone();
        b._pad_29 = [9; 7];
        b._pad_3e = [8; 2];
        b.word_38 = (-0.0f32).to_bits();
        assert!(image_settings_equal(&a, &b));
        b.word_38 = f32::NAN.to_bits();
        assert!(!image_settings_equal(&b, &b));
    }
}

#[cfg(test)]
mod original_world_worker_integration {
    use super::super::save_thread::{execute_save_cmd, CurrentSaveFile, UnsavedFileList};
    use super::*;
    use crate::resources::{
        color_id::{GateColorIds, HalftimberColorIds, WallColorIds},
        history::{History, MaybePendingSnapshotId},
        stairs::{stairs_removed_supports::StairsRemovedSupports, stairs_state::StairsState},
        terrain_editing::TerrainEditingData,
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        walls::{decorator_storage::DecoratorStorage, InnerWalls, WallState},
        world_raster::DirtyTilePublisher,
        world_raster_defs::{
            Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, GardenRaster, PathRaster,
            WaterRaster,
        },
    };
    use bevy_ecs::{prelude::World, system::RunSystemOnce};
    #[test]
    fn real_nonempty_world_capture_resolves_the_same_history_owner_on_worker_flush() {
        // Original intro wall data; all other below values are explicit edits
        // to actual CPU owners, not replacement JSON snapshot sections.
        let original: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-intro-wall-states-b.json"
        ))
        .unwrap();
        let state: WallState = serde_json::from_value(original[0]["state"].clone()).unwrap();
        let mut walls = InnerWalls::new();
        let wall_id = walls.new_wall(state);
        let mut terrain = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
        terrain.new_stroke([1.0, 2.0, 3.0]).unwrap().unwrap();
        let mut heights = TerrainHeightsData::<TerrainHeightSsbo>::new();
        heights.heights_mut()[1] = 0x1234;
        heights.heights_mut()[512] = 0xabcd;
        let expected_heights = bytemuck::cast_slice::<u16, u8>(heights.heights()).to_vec();
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        path.composite_mut()[17] = 255;
        let expected_path = path.current_buffer().to_vec();
        let mut stairs = StairsState::new();
        stairs.make_backup();
        let mut world = World::new();
        world.insert_resource(walls);
        world.insert_resource(terrain);
        world.insert_resource(heights);
        world.insert_resource(path);
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(stairs);
        world.insert_resource(StairsRemovedSupports::default());
        world.insert_resource(DecoratorStorage::default());
        world.insert_resource(WallColorIds::new());
        world.insert_resource(GateColorIds::new());
        world.insert_resource(HalftimberColorIds::new());
        world.init_resource::<crate::systems::ui_clutter::DeletedAutoClutter>();
        let domain = world
            .run_system_once(|snapshot: SaveSnapshotWorld| snapshot.capture())
            .unwrap();
        assert_eq!(domain.walls.len(), 1);
        assert_eq!(domain.walls[0].0, wall_id);
        assert_eq!(domain.terrain.strokes.len(), 1);
        assert_eq!(
            domain.terrain_data.as_deref(),
            Some(expected_heights.as_slice())
        );
        assert_eq!(domain.raster[0], ("PathRaster", expected_path.clone()));

        let mut history: History = History::empty_glade();
        let pending = MaybePendingSnapshotId::new();
        set_current_snapshot(
            &mut history.ledger_mut().resolved_snapshots,
            -1,
            pending.clone(),
        );
        assert_eq!(pending.state(), 0);
        // Clone the actual same atomic owners before the worker assigns a hash.
        let references = SnapshotReferencesToJson(history.ledger().resolved_snapshots.clone());
        let snapshot = capture_game_snapshot_save_data(SessionId::new(), 0, -1, domain);
        let original_save: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-intro-history-b.json"
        ))
        .unwrap();
        let sections = GameSaveSections {
            values: SaveFileSection::ALL.map(|section| {
                let owner = if section as u8 >= SaveFileSection::ChainIndex as u8 {
                    &original_save["History"]
                } else {
                    &original_save
                };
                SerializedJson::new(&owner[section.json_key()]).unwrap()
            }),
        };
        let directory = tempfile::tempdir().unwrap();
        let current = CurrentSaveFile::create(directory.path(), sections, &[]);
        let (commands, receiver) = crossbeam_channel::unbounded();
        let (start, begin) = crossbeam_channel::bounded::<()>(1);
        // Execute the actual production worker dispatcher on its own thread.
        // The fixture supplies only its initial real temp-directory file; it
        // does not redirect global save paths or replace any worker command.
        let worker = std::thread::spawn(move || {
            begin.recv().unwrap();
            let mut current = Some(current);
            let unsaved = UnsavedFileList::default();
            while let Ok(command) = receiver.recv() {
                execute_save_cmd(&mut current, command, &unsaved);
            }
            assert_eq!(unsaved.len(), 0);
        });
        commands
            .send(AsyncSaveCmd::CaptureSnapshot {
                snapshot,
                pending: pending.clone(),
            })
            .unwrap();
        commands
            .send(AsyncSaveCmd::ChangeSection {
                section: SaveFileSection::Snapshots,
                value: defer_json(references),
            })
            .unwrap();
        assert_eq!(pending.state(), 0, "queued capture has not executed");
        let mut auto = AutoSaveState::default();
        auto.attach_command_sender(commands);
        start.send(()).unwrap();
        // Real tag10 reply follows snapshot assignment, late reference serde,
        // section writes and actual ShadowFile.sync_all.
        auto.flush_synchronously().unwrap();
        let id = pending
            .snapshot_id()
            .expect("actual snapshot hash assigned");
        let shared = current_snapshot(&history.ledger().resolved_snapshots, -1).unwrap();
        assert_eq!(shared.value.snapshot_id(), Some(id));
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(directory.path().join("history.json")).unwrap())
                .unwrap();
        assert_eq!(saved["History"]["snapshots"][0]["snapshot_id"], id.0);
        let bytes = std::fs::read(
            directory
                .path()
                .join("snapshots")
                .join(format!("{}.snapshot", id.0)),
        )
        .unwrap();
        let chunks = motomoto::Reader::new(&bytes)
            .iter()
            .collect::<std::io::Result<Vec<_>>>()
            .unwrap();
        let metadata: serde_json::Value = serde_json::from_slice(chunks[0].data).unwrap();
        assert_eq!(metadata.as_object().unwrap().len(), 14);
        assert_eq!(metadata["Walls"][0][0], wall_id.0);
        assert_eq!(metadata["Terrain"]["strokes"].as_object().unwrap().len(), 1);
        let path_chunk = chunks
            .iter()
            .find(|chunk| chunk.tag == motomoto::tag_from_str_trunc("PathRaster"))
            .unwrap();
        assert_eq!(
            super::super::snapshot::inflate_terrain(&path_chunk.data[8..]).unwrap(),
            expected_path
        );
        let terrain_chunk = chunks
            .iter()
            .find(|chunk| chunk.tag == motomoto::tag_from_str("Terrain"))
            .unwrap();
        assert_eq!(
            super::super::snapshot::inflate_terrain(terrain_chunk.data).unwrap(),
            expected_heights
        );
        drop(auto);
        worker.join().unwrap();
    }
}

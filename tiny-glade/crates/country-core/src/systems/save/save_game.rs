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
use crate::{
    resources::{
        glade_color_customization::GladeColorCustomization,
        history::HistoryIdentity,
        session::{SessionId, SessionMeta},
        time::GameTime,
    },
    systems::{
        camera_rig::CameraRigParametersCache,
        save::{
            save_backup_list::BackupSaveFileKind,
            save_backup_metronome::SaveBackupMetronome,
            snapshot::SnapshotSaveData,
        },
    },
};
use bevy_ecs::prelude::{EventWriter, Res, ResMut};
use hashbrown::HashMap;
use serde::{Serialize, Serializer};
use std::{
    io::Write,
    ops::Range,
    path::{Path, PathBuf},
    sync::{mpsc, Arc},
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

pub fn save_game_to_memory(
    sections: GameSaveSections,
    edits: &[SerializedJson],
) -> (Vec<u8>, SaveFileStructure) {
    let mut output = vec![b'{'];
    let mut structure = SaveFileStructure::default();
    let mut sections = SaveFileSection::ALL.into_iter().zip(sections.values);
    write_json_sections(&mut output, sections.by_ref().take(7), &mut structure);
    output.extend_from_slice(b"\"History\":{\n");
    write_json_sections(&mut output, sections, &mut structure);
    output.extend_from_slice(b"\"edit_chain\":[\n");
    for (index, edit) in edits.iter().enumerate() {
        structure.history_edit_offsets.push(output.len() as u64);
        if index != 0 {
            output.extend_from_slice(b",\n");
        }
        output.extend_from_slice(edit.as_bytes());
    }
    output.extend_from_slice(b"\n]}}");
    (output, structure)
}

/// `save_game_to_file<File>` 0x140241ca0 / `<Cursor<Vec<u8>>>` 0x141750c90：
/// 生成完整存档 JSON 并写出。字节布局与 `save_game_to_memory` 一致。
pub fn save_game_to_writer(
    writer: &mut impl Write,
    sections: GameSaveSections,
    edits: &[SerializedJson],
) -> std::io::Result<()> {
    let (bytes, _) = save_game_to_memory(sections, edits);
    writer.write_all(&bytes)
}

// ---------------------------------------------------------------------------
// 存档线程命令通道（PDB：`save_game::AsyncSaveCmd`，经 mpsc 传输）
// ---------------------------------------------------------------------------

/// 游戏线程 → 存档线程的命令。原版变体标签 3..=11（execute_save_cmd 0x141721aa0
/// 的 switch 面）；0/1/2 未在 switch 出现（I：死/历史变体）。
#[derive(Clone, Debug)]
pub enum AsyncSaveCmd {
    /// 3 OpenFile：全量重写 + 建 ShadowFile("history.json")。载荷带完整节/编辑。
    OpenFile {
        session: SessionId,
        sections: GameSaveSections,
        edits: Vec<SerializedJson>,
    },
    /// 4 CloseFile。
    CloseFile {
        session: SessionId,
    },
    /// 5 ChangeSection：改节 + mark_dirty。
    ChangeSection {
        section: SaveFileSection,
        value: SerializedJson,
    },
    /// 6 ReplaceEdits：截断 + 追加编辑。
    ReplaceEdits {
        truncate: usize,
        edits: Vec<SerializedJson>,
    },
    /// 7 BackupSave：`history-auto` 前缀备份。
    BackupSave {
        path: PathBuf,
        kind: BackupSaveFileKind,
    },
    /// 8 ManualBackup（execute_save_cmd 无专属 case，落 default）。
    ManualBackup {
        path: PathBuf,
        kind: BackupSaveFileKind,
    },
    /// 9 CaptureSnapshot：快照 JSON 落盘。
    CaptureSnapshot {
        snapshot: Arc<SnapshotSaveData>,
    },
    /// 10 Flush。
    Flush,
    /// 11 Shutdown：需要时 flush_deferred_and_sync + 确认 + 退出线程。
    Shutdown { flush: bool },
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

/// `save_game` 0x140a3f470 的 ECS 系统（X4 本体组装，可接部分）。
///
/// ① 备份节拍器推进（`AutoSaveState.metronome`，+0x140/0x144/0x148 阈值门）；
/// ② 触发备份 → log "Capturing a game snapshot" + 发 `CaptureSnapshotRequested` 事件
///    （宿主构建快照 Arc 后经 `AsyncSaveCmd::CaptureSnapshot` 发送）；
/// ③ 同步保存（assemble_game_save_sections → `AsyncSaveCmd::OpenFile`）由宿主在事件响应中
///    完成（域资源 InnerWalls/Roof/TimeOfDay/Terrain… 由宿主收集，不伪造节载荷）。
pub fn save_game_system<S: Send + Sync + 'static>(
    mut auto: ResMut<AutoSaveState>,
    time: Res<GameTime>,
    mut capture: EventWriter<CaptureSnapshotRequested>,
) {
    // ① 节拍器：阈值门（+0x148 < 60.0）在 metronome 累计块内。
    auto.metronome.add_active_game_time(time.delta);
    // ② 触发备份。
    if auto.metronome.consider_backing_up().is_some() {
        log::info!("Capturing a game snapshot");
        capture.write(CaptureSnapshotRequested);
    }
    // ③ 同步保存的 AsyncSaveCmd 发送由宿主在事件响应中经 `auto.send_command` 完成。
}

/// 注册 save_game 系统事件。
pub fn register_save_game_events(world: &mut bevy_ecs::world::World) {
    world.init_resource::<bevy_ecs::event::Events<CaptureSnapshotRequested>>();
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

/// 删除会话命令（PDB：`save_game::DeleteSessionCmd`）。
#[derive(Clone, Debug)]
pub struct DeleteSessionCmd {
    pub session: SessionId,
}

/// 保存完成事件（PDB：`save_game::SavingGameComplete`）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SavingGameComplete;

/// 同步存档路径的组装数据（PDB：`save_game::GameSaveData`）。原版由
/// save_game 0x140a3f470 从 ~9 资源拷贝组装；本 crate 的域资源尚未齐全，
/// 组装系统留待对应域恢复后接入。
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
/// - `close_file` 0x140a43840：清节状态与错误列表。
/// - `create_backup_of<PathBuf>` 0x141e59860：创建备份目录。
/// - `last_error_state` 0x140a439a0：错误状态访问。
/// - 节拍器字段（counter@+0x140/累计@+0x144/阈值@+0x148）由 `SaveBackupMetronome` 持有；
///   阈值门在 `add_active_game_time` 累计块内（reviewer 复核）。
#[derive(bevy_ecs::resource::Resource, Debug)]
pub struct AutoSaveState {
    pub metronome: SaveBackupMetronome,
    pub sections: [AutoSaveSectionState; 10],
    last_error: Option<anyhow::Error>,
    command_sender: Option<mpsc::Sender<AsyncSaveCmd>>,
}

impl Default for AutoSaveState {
    fn default() -> Self {
        Self {
            metronome: SaveBackupMetronome::default(),
            sections: [AutoSaveSectionState::default(); 10],
            last_error: None,
            command_sender: None,
        }
    }
}

impl AutoSaveState {
    /// 0x140a43840：关闭当前档，清节状态、错误与延迟序列化。
    pub fn close_file(&mut self) {
        self.sections = [AutoSaveSectionState::default(); 10];
        self.last_error = None;
    }

    /// 0x141e59860：创建备份目录；失败记录错误并返回。
    pub fn create_backup_of(&mut self, path: PathBuf) -> bool {
        match std::fs::create_dir_all(&path) {
            Ok(()) => true,
            Err(error) => {
                log::error!("Error creating backup save folder: {error}");
                self.last_error = Some(error.into());
                false
            }
        }
    }

    /// 0x140a439a0：最近错误状态。
    pub fn last_error_state(&self) -> Option<&anyhow::Error> {
        self.last_error.as_ref()
    }

    /// 绑定存档线程命令发送端（由存档初始化者装配）。
    pub fn attach_command_sender(&mut self, sender: mpsc::Sender<AsyncSaveCmd>) {
        self.command_sender = Some(sender);
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

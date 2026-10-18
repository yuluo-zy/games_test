//! Mutable file portion of CurrentSaveFile; native offsets +0x10 shadow,
//! +0x90 history offsets, +0xa8 section ranges.
//!
//! B save_thread 0x141720680 主循环（O，全文见 session-save/ALGORITHMS.md §2.1）：
//! 每 ≥1s tick 逐文件 flush → condvar 等待 ≤1s → 错误列表 diff/日志/回传游戏线程 →
//! drop 当前存档 → recv 命令 → 3=OpenFile 前置 unsaved 检查 → execute_save_cmd。
//!
//! `execute_save_cmd` 0x141721aa0（O，§2.2）：switch {3,4,5,6,7,9,10}+default
//! （8=ManualBackup 落 default）；公共 truncate 断言；state==3 = "无活动档"的
//! No-active panic/日志门控。on_load 回放依赖 meta_commander.rs，按主会话裁决留最后。
use super::{
    save_game::{
        AsyncSaveCmd, GameSaveSections, SaveFileSection, SaveFileStructure, SerializedJson,
    },
    shadow_file::ShadowFile,
    snapshot::SnapshotSaveData,
};
use crate::resources::session::SessionId;
use bevy_ecs::prelude::{Commands, ResMut};
use crossbeam_channel::Receiver;
use hashbrown::HashSet;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    thread::JoinHandle,
    time::Duration,
};

/// 手动备份身份字节偏移（O：0x141721aa0 default 区 `param_2 + 0x53`）。
pub const MANUAL_BACKUP_IDENTITY_OFFSET: usize = 0x53;
/// "snapshot" 常量（O：u64 小端 0x746f687370616e73）。
pub const MANUAL_BACKUP_SNAPSHOT_TAG: u64 = 0x746f_6873_7061_6e73;

/// 每节延迟变更（PDB：`save_thread::DeferredSection`；原版 `[Option<..>; 10]` 槽位）。
#[derive(Clone, Debug)]
pub struct DeferredSection {
    pub section: SaveFileSection,
    pub value: SerializedJson,
}

/// 未落盘文件列表条目（O：`UnsavedFileList::add` 0x14171ff80 fresh disasm：
/// `len << 7` 计算尾部、`grow_one` 后**拷贝 16 qword = 0x80B**；`contains_file`
/// 0x1417201a0 每步 `len*0x80 - 0x80` 遍历，条目内路径按 `Path::components()`
/// 反向 `eq_by` 比较）。路径位于条目 **+0**（`OsStr/PathBuf` 24B）；其余 `0x68`
/// 字段身份 U（`contains_file` 不读），保持为不解析的不透明字节，不编造字段名。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsavedFileEntry {
    /// +0x00：路径（`PathBuf`，24B）。
    pub path: PathBuf,
    /// +0x18..0x80：未被 `contains_file` 读取的原生尾部（U，原样保留）。
    pub opaque: [u8; 0x68],
}
impl UnsavedFileEntry {
    /// 生产路径构造（未被消费的 0x68 尾部置零；不是对原生字段值的断言）。
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            path,
            opaque: [0u8; 0x68],
        }
    }
}
const _: () = assert!(std::mem::size_of::<UnsavedFileEntry>() == 0x80);

/// 未落盘文件列表（PDB：`save_thread::UnsavedFileList`；`parking_lot::Mutex`
/// 保护，O：add 0x14171ff80 / contains_file 0x1417201a0 的 `RawMutex` fast/slow
/// 路径与 `Vec` 三元组 {ptr, cap, len}）。
#[derive(Debug, Default)]
pub struct UnsavedFileList {
    files: parking_lot::Mutex<Vec<UnsavedFileEntry>>,
}
impl UnsavedFileList {
    /// B 0x14171ff80：登记一个未落盘文件（0x80B 整条目拷贝）。
    pub fn add(&self, entry: UnsavedFileEntry) {
        self.files.lock().push(entry);
    }
    /// 生产便利入口：以路径构造 0x80B 条目。
    pub fn add_path(&self, path: PathBuf) {
        self.add(UnsavedFileEntry::from_path(path));
    }
    /// B 0x1417201a0：列表查找（"Was asked to load an unsaved file" 前置检查）。
    /// 原生按 `Path::components()` 反向逐组件比较；对等价的绝对/相对路径，
    /// `PathBuf ==` 等价（CRATE 侧路径均为 join 生成的同形路径）。
    pub fn contains_file(&self, path: &Path) -> bool {
        self.files
            .lock()
            .iter()
            .any(|candidate| candidate.path == path)
    }
    /// 条目数（测试/诊断）。
    pub fn len(&self) -> usize {
        self.files.lock().len()
    }
}

/// 单文件自动保存失败（PDB：`save_thread::AutoSaveError`）。原错误载荷变体待取证；
/// 这里保留触发失败的 OS 消息，供错误列表 diff 与日志。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutoSaveError {
    /// 落盘/同步失败，携带底层错误文本。
    Io(String),
}

/// 自动保存错误状态（PDB：`save_thread::AutosaveErrorState`；`is_ok` = 列表空）。
/// 游戏线程侧持有，接收存档线程回传的错误列表。
#[derive(Debug)]
pub struct AutosaveErrorState {
    receiver: Receiver<Vec<(PathBuf, AutoSaveError)>>,
    errors: Vec<(PathBuf, AutoSaveError)>,
}
impl AutosaveErrorState {
    /// 建立错误回传通道对：`Sender` 交给存档线程，`AutosaveErrorState` 留在游戏线程。
    pub fn channel() -> (
        crossbeam_channel::Sender<Vec<(PathBuf, AutoSaveError)>>,
        Self,
    ) {
        let (sender, receiver) = crossbeam_channel::unbounded();
        (sender, Self::new(receiver))
    }
    /// 直接绑定接收端。
    pub fn new(receiver: Receiver<Vec<(PathBuf, AutoSaveError)>>) -> Self {
        Self {
            receiver,
            errors: Vec::new(),
        }
    }
    /// **VA 错锚纠正（O：fresh 反编译）**：0x140b92210 实为
    /// `tg_text::shape_stream::ShapeStream::is_empty`（9B 空函数），**非**
    /// `AutosaveErrorState::is_ok`。本访问器语义 = 错误列表空（原生 `[rcx+0x10]==0`）。
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
    pub fn errors(&self) -> &[(PathBuf, AutoSaveError)] {
        &self.errors
    }
    /// 吸干通道里的错误列表（与存档线程的 diff 回传配套）。
    pub fn drain_pending(&mut self) {
        while let Ok(list) = self.receiver.try_recv() {
            self.errors = list;
        }
    }
}

#[derive(Debug)]
pub struct CurrentSaveFile {
    pub shadow_file: ShadowFile,
    pub save_file_structure: SaveFileStructure,
    deferred_sections: [Option<DeferredSection>; 10],
    locked_backups: HashSet<PathBuf>,
    snapshot_gc_releases: Option<Receiver<PathBuf>>,
}

impl CurrentSaveFile {
    pub fn create(
        directory: impl AsRef<Path>,
        sections: GameSaveSections,
        edits: &[SerializedJson],
    ) -> Self {
        let (bytes, structure) = super::save_game::save_game_to_memory(sections, edits);
        Self {
            shadow_file: ShadowFile::create(bytes, directory, "history.json"),
            save_file_structure: structure,
            deferred_sections: std::array::from_fn(|_| None),
            locked_backups: HashSet::new(),
            snapshot_gc_releases: None,
        }
    }

    /// B 0x14171ece0. Range includes value, comma, padding and newline.
    pub fn change_section(&mut self, section: SaveFileSection, value: &SerializedJson) {
        let old = self
            .save_file_structure
            .section_ranges
            .get(&section)
            .expect("Save section not found")
            .clone();
        let old_length = old.end - old.start;
        let required = value.as_bytes().len() as u64 + 2;
        let (new_length, tail) = if required > old_length {
            let tail = self.shadow_file.data()[old.end as usize..].to_vec();
            let length = (old_length * 2).max(required);
            let growth = length - old_length;
            for range in self.save_file_structure.section_ranges.values_mut() {
                if range.start > old.start {
                    range.start += growth;
                }
                if range.end > old.start {
                    range.end += growth;
                }
            }
            for offset in &mut self.save_file_structure.history_edit_offsets {
                if *offset > old.start {
                    *offset += growth;
                }
            }
            (length, tail)
        } else {
            (old_length, Vec::new())
        };
        let mut replacement = Vec::with_capacity(new_length as usize);
        replacement.extend_from_slice(value.as_bytes());
        replacement.push(b',');
        replacement.resize(new_length as usize - 1, b' ');
        replacement.push(b'\n');
        assert_eq!(replacement.len() as u64, new_length);
        self.shadow_file.write_at(old.start, &replacement);
        if !tail.is_empty() {
            self.shadow_file.write_at(old.start + new_length, &tail);
        }
    }

    /// 延迟节变更入槽（10 槽位；`flush_deferred_and_sync` 统一落盘）。
    pub fn queue_section_change(&mut self, section: SaveFileSection, value: SerializedJson) {
        self.deferred_sections[section as usize] = Some(DeferredSection { section, value });
    }

    /// B ReplaceEdits case: keep the requested prefix, truncate at the first
    /// removed edit's pre-separator offset, append replacements, restore footer.
    pub fn replace_edits(&mut self, truncate: usize, edits: &[SerializedJson]) -> bool {
        let offsets = &mut self.save_file_structure.history_edit_offsets;
        assert!(
            truncate <= offsets.len(),
            "truncate <= self.save_file_structure.history_edit_offsets.len()"
        );
        let position = offsets
            .get(truncate)
            .copied()
            .unwrap_or(self.shadow_file.data().len() as u64 - 4);
        offsets.truncate(truncate);
        self.shadow_file.buffer.append_offset = position;
        for edit in edits {
            let position = self.shadow_file.buffer.append_offset;
            if !offsets.is_empty() {
                self.shadow_file.write_at(position, b",\n");
            }
            offsets.push(position);
            self.shadow_file
                .write_at(self.shadow_file.buffer.append_offset, edit.as_bytes());
        }
        self.shadow_file
            .write_at(self.shadow_file.buffer.append_offset, b"\n]}}");
        self.shadow_file
            .truncate(self.shadow_file.buffer.append_offset);
        self.shadow_file.flush()
    }

    /// `add_history_edit_to_save_file` / `ensure_current_snapshot` 0x140a439b0：
    /// 追加一条编辑到存档（",\n" 分隔 + 序列化 + mark_dirty）。复用
    /// `ShadowBuffer::append_edit` 的节偏移协议。序列化失败时按原版日志
    /// "Error appending edit to save file: " 记录。
    pub fn add_history_edit_to_save_file<E>(
        &mut self,
        serialize: impl FnOnce(&mut Vec<u8>) -> Result<(), E>,
    ) -> Result<(), E> {
        match self.shadow_file.buffer.append_edit(serialize) {
            Ok(()) => Ok(()),
            Err(error) => {
                log::error!("Error appending edit to save file");
                Err(error)
            }
        }
    }

    pub fn sync_all(&mut self) -> bool {
        self.shadow_file.sync_all()
    }

    /// Complete B1934B @14171f7f0. Its tail@14171fc9f calls
    /// ShadowFile::sync_all after applying each deferred section. Earlier
    /// decompilation omitted that tail and incorrectly returned unconditional true.
    pub fn flush_deferred_and_sync(&mut self) -> bool {
        let pending = std::mem::take(&mut self.deferred_sections);
        for slot in pending.into_iter().flatten() {
            self.change_section(slot.section, &slot.value);
        }
        self.sync_all()
    }

    /// B execute_save_cmd case 7 BackupSave / default 8 ManualBackup：
    /// "Copying inactive save file to backup" / "Writing backup from shadow copy" /
    /// "Error flushing history file for backup"。先把内存影像 flush 到影子文件，再把
    /// 数据复制到目标备份路径。
    pub fn backup_save(
        &mut self,
        target: &Path,
        _kind: super::save_backup_list::BackupSaveFileKind,
    ) -> bool {
        if !self.shadow_file.flush() {
            log::error!("Error flushing history file for backup");
            return false;
        }
        if let Some(parent) = target.parent() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                log::error!("Error making an auto-save backup: {error}");
                return false;
            }
        }
        match std::fs::write(target, self.shadow_file.data()) {
            Ok(()) => true,
            Err(error) => {
                log::error!("Error making an auto-save backup: {error}");
                false
            }
        }
    }

    /// 手动备份（execute_save_cmd 无专属 case，落 default）。
    ///
    /// **身份比对（O：0x141721aa0 default 区）**：载荷比对文件身份字节
    /// `+0x53`/`+0x299..0x2a7` 与 "snapshot" 常量 `0x746f687370616e73`（="snapshot"
    /// u64 小端）。身份命中 snapshot 标记 → 拒绝备份。`+0x299` 起 16 字节旧态身份比对
    /// 载荷结构 U（待取证，不编造）；此处实现可观测的 +0x53 身份字节校验。
    pub fn perform_manual_backup(
        &mut self,
        target: &Path,
        kind: super::save_backup_list::BackupSaveFileKind,
    ) -> bool {
        let data = self.shadow_file.data();
        if data.len() > MANUAL_BACKUP_IDENTITY_OFFSET + 8 {
            let identity = u64::from_le_bytes(
                data[MANUAL_BACKUP_IDENTITY_OFFSET..MANUAL_BACKUP_IDENTITY_OFFSET + 8]
                    .try_into()
                    .expect("manual backup identity read"),
            );
            if identity == MANUAL_BACKUP_SNAPSHOT_TAG {
                log::warn!(
                    "Manual backup rejected: save file identity matches the snapshot marker"
                );
                return false;
            }
        }
        self.backup_save(target, kind)
    }

    /// B execute_save_cmd case 7 的 retention 删除：`remove_counter` 命中 →
    /// 删除旧 `history-auto-{counter}.json` 备份（日志 "Backup limit reached; removing "
    /// / "Error deleting stale backup "）。返回是否成功删除。
    pub fn remove_stale_backup(&mut self, folder: &Path, counter: u32) -> bool {
        let path = folder.join(format!("history-auto-{counter}.json"));
        match std::fs::remove_file(&path) {
            Ok(()) => {
                log::info!("Backup limit reached; removing {}", path.display());
                true
            }
            Err(error) => {
                log::error!("Error deleting stale backup {}: {error}", path.display());
                false
            }
        }
    }

    /// B execute_save_cmd case 9 CaptureSnapshot：创建 `snapshots` 目录 → 快照 JSON
    /// 落盘为 `<snapshot_id>.snapshot`（GC 按 stem 解析 u64 引用判定）。
    pub fn capture_snapshot(&mut self, snapshot: &Arc<SnapshotSaveData>) -> bool {
        let folder = self.shadow_file.directory().join("snapshots");
        if let Err(error) = std::fs::create_dir_all(&folder) {
            log::error!("creating snapshot folder: {error}");
            return false;
        }
        let target = folder.join(format!("{}.snapshot", snapshot.snapshot_id));
        let json = snapshot.to_json().to_string();
        match std::fs::write(&target, json.as_bytes()) {
            Ok(()) => true,
            Err(error) => {
                log::error!("Error writing snapshot {}: {error}", target.display());
                false
            }
        }
    }

    /// `nuke_unreferenced_shapshots`（PDB 名拼写即如此）：清理未引用快照，由
    /// `snapshot_gc_thread` 0x14172b280 实现；本方法启动 GC 线程并接管释放接收端。
    pub fn nuke_unreferenced_shapshots(
        &mut self,
        backups: Vec<PathBuf>,
        snapshots: Vec<PathBuf>,
        referenced: HashSet<u64>,
    ) -> JoinHandle<()> {
        self.start_snapshot_gc(backups, snapshots, referenced)
    }

    /// Install the new file's release receiver and lock the exact backup list
    /// passed to the worker. Replacing the receiver cancels the previous job.
    /// The caller supplies the current history's real referenced IDs; none are
    /// inferred from filenames or fabricated from an empty history.
    pub fn start_snapshot_gc(
        &mut self,
        backups: Vec<PathBuf>,
        snapshots: Vec<PathBuf>,
        referenced: HashSet<u64>,
    ) -> JoinHandle<()> {
        let (sender, receiver) = crossbeam_channel::unbounded();
        self.snapshot_gc_releases = Some(receiver);
        self.locked_backups = backups.iter().cloned().collect();
        std::thread::spawn(move || {
            super::snapshot_gc::snapshot_gc_thread(backups, snapshots, referenced, sender)
        })
    }

    /// B save_thread 0x141720680 consumes one ready release per loop and
    /// asserts that it belongs to the current file's locked backup set.
    pub fn consume_snapshot_gc_release(&mut self) -> Option<PathBuf> {
        let path = self.snapshot_gc_releases.as_ref()?.try_recv().ok()?;
        assert!(
            self.locked_backups.remove(&path),
            "current_save_file.locked_backups.remove(&path_buf)"
        );
        Some(path)
    }

    pub fn is_backup_locked(&self, path: &Path) -> bool {
        self.locked_backups.contains(path)
    }
}

/// `execute_save_cmd` 0x141721aa0：命令分发。
///
/// 公共断言 `truncate <= history_edit_offsets.len()` 已由 `replace_edits` 内置；
/// `ChangeSection`/`ReplaceEdits` 在无活动档时 panic（state==3），
/// `CaptureSnapshot`/`Flush` 在无活动档时仅日志。
///
/// **case3(OpenFile)/case4(CloseFile)（O：fresh 反编译）**：均对前一档
/// `flush_deferred_and_sync`（非旧实现的 `sync_all`），失败时 **`UnsavedFileList::add`**
/// （原生生产路径调用 add，unsaved 机制由 save_thread OpenFile 前置检查消费）。
pub fn execute_save_cmd(
    current: &mut Option<CurrentSaveFile>,
    cmd: AsyncSaveCmd,
    unsaved: &UnsavedFileList,
) {
    match cmd {
        AsyncSaveCmd::OpenFile {
            session,
            sections,
            edits,
        } => {
            execute_open_file_cmd(current, session, sections, edits, unsaved);
        }
        AsyncSaveCmd::CloseFile { .. } => {
            // case4：flush 前一档 + 失败 UnsavedFileList::add + drop（原生 = flush+add+drop）。
            if let Some(mut previous) = current.take() {
                if !previous.flush_deferred_and_sync() {
                    let path = previous.shadow_file.path().to_path_buf();
                    unsaved.add_path(path);
                }
            }
        }
        AsyncSaveCmd::ChangeSection { section, value } => {
            let file = current
                .as_mut()
                .expect("No active save file, but AsyncSaveCmd::ChangeSection has been issued");
            file.change_section(section, &value);
        }
        AsyncSaveCmd::ReplaceEdits { truncate, edits } => {
            let file = current
                .as_mut()
                .expect("No active save file, but AsyncSaveCmd::ReplaceEdits has been issued");
            if file.replace_edits(truncate, &edits) {
                log::debug!("Edit appended to save file");
            } else {
                log::error!(
                    "Error appending edit to save file: {:?}",
                    file.shadow_file.last_error()
                );
            }
        }
        AsyncSaveCmd::BackupSave {
            path,
            kind,
            remove_counter,
        } => {
            if let Some(file) = current.as_mut() {
                file.backup_save(&path, kind);
                // retention 删除（O：BackupDecision.remove_counter → 删除旧备份）。
                if let Some(counter) = remove_counter {
                    if let Some(folder) = path.parent() {
                        file.remove_stale_backup(folder, counter);
                    }
                }
            } else {
                log::warn!("No active save file, but AsyncSaveCmd::BackupSave has been issued");
            }
        }
        AsyncSaveCmd::ManualBackup { path, kind, reply } => {
            if let Some(file) = current.as_mut() {
                let result = file.perform_manual_backup(&path, kind);
                if let Some(reply) = reply {
                    let _ = reply.send(if result {
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Manual backup failed: {}", path.display()))
                    });
                }
            } else {
                log::warn!("No active save file, but AsyncSaveCmd::ManualBackup has been issued");
                // 同步请求-应答（create_backup_of 0x141e59860）：无活动档也必须回发，
                // 否则请求方阻塞 recv 死锁。
                if let Some(reply) = reply {
                    let _ = reply.send(Err(anyhow::anyhow!(
                        "No active save file, but AsyncSaveCmd::ManualBackup has been issued"
                    )));
                }
            }
        }
        AsyncSaveCmd::CaptureSnapshot { snapshot } => {
            if let Some(file) = current.as_mut() {
                file.capture_snapshot(&snapshot);
            } else {
                log::warn!(
                    "No active save file, but AsyncSaveCmd::CaptureSnapshot has been issued"
                );
            }
        }
        AsyncSaveCmd::Flush { reply } => {
            // Original execute_save_cmd tag10 panics without an active file.
            let file = current
                .as_mut()
                .expect("No active save file, but a synchronous flush has been issued");
            let result = if file.flush_deferred_and_sync() {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Error flushing changes to disk"))
            };
            // Dropping the waiting game thread does not turn a disk failure
            // into success. Native ignores a failed reply send.
            let _ = reply.send(result);
        }
        AsyncSaveCmd::Shutdown { .. } => unreachable!("Shutdown is handled by save_thread"),
    }
}

/// B execute_save_cmd case 3 OpenFile（内联 `execute_open_file_cmd`）：
/// 前一档 `flush_deferred_and_sync`（失败 `UnsavedFileList::add`，log
/// "Flush error while closing previous save file (upon open request)"）→
/// 全量重写（save_game_to_file<Cursor>，失败 log "Rewriting history file"）→
/// ShadowFile::create("history.json") → 装入新 CurrentSaveFile。
pub fn execute_open_file_cmd(
    current: &mut Option<CurrentSaveFile>,
    session: SessionId,
    sections: GameSaveSections,
    edits: Vec<SerializedJson>,
    unsaved: &UnsavedFileList,
) {
    if let Some(previous) = current.as_mut() {
        if !previous.flush_deferred_and_sync() {
            log::warn!("Flush error while closing previous save file (upon open request)");
            let path = previous.shadow_file.path().to_path_buf();
            unsaved.add_path(path);
        }
    }
    let new_file = CurrentSaveFile::create(session.save_folder_path(), sections, &edits);
    if !new_file.shadow_file.is_open() {
        log::error!("Rewriting history file");
    }
    *current = Some(new_file);
}

/// `save_thread` 0x141720680 主循环（O，ALGORITHMS.md §2.1）。
///
/// - 每 ≥1s tick：有活动档且脏块非空 → 首个脏条目 `ShadowFile::sync_all`，失败入错误列表；
/// - condvar 等待 ≤1s（本实现用 `recv_timeout` 等价等待命令）；
/// - 错误列表 diff：新列表空 → log "Auto-save I/O error resolved"；非空 →
///   log "Save thread I/O error; affected file count: " → 经 `error_sender` 回传游戏线程；
/// - drop 当前存档（消费一个 GC 释放，断言属于当前档）；
/// - 3=OpenFile 前置：`unsaved.contains_file(save_folder/history.json)` → panic
///   "Was asked to load an unsaved file"；
/// - 11=Shutdown：**flush=true** → `flush_deferred_and_sync`（失败 warn
///   "Error flushing save file on game thread disconnect."）→ drop 当前档 → **返回退出**；
///   **flush=false** → 应用 deferred + 睡 10ms + **继续循环（不 break，O 0x141721104
///   jumptable 应用 deferred change_section→sleep(0x989680=10ms)→continue）**。
///
/// 原生**无 shutdown_ack 通道**（退出路径 drop error_sender + commands receiver 后
/// return）；本实现已删自造 shutdown_ack。
pub fn save_thread(
    commands: Receiver<AsyncSaveCmd>,
    error_sender: crossbeam_channel::Sender<Vec<(PathBuf, AutoSaveError)>>,
    unsaved: Arc<UnsavedFileList>,
) {
    let mut current: Option<CurrentSaveFile> = None;
    let mut previous_errors: Vec<(PathBuf, AutoSaveError)> = Vec::new();
    let mut last_tick = std::time::Instant::now();

    loop {
        // 1. ≥1s tick：flush 第一个脏影子文件。
        if last_tick.elapsed() >= Duration::from_secs(1) {
            let mut error_list: Vec<(PathBuf, AutoSaveError)> = Vec::new();
            if let Some(file) = current.as_mut() {
                if file.shadow_file.dirty_blocks().next().is_some() {
                    if !file.shadow_file.sync_all() {
                        let path = file.shadow_file.path().to_path_buf();
                        let message = file
                            .shadow_file
                            .last_error()
                            .map(|error| format!("{error:#}"))
                            .unwrap_or_else(|| "sync failed".to_string());
                        error_list.push((path, AutoSaveError::Io(message)));
                    }
                }
            }
            last_tick = std::time::Instant::now();

            // 3. 错误列表 diff + 日志 + 回传游戏线程。
            if error_list != previous_errors {
                if error_list.is_empty() {
                    log::info!("Auto-save I/O error resolved");
                } else {
                    log::warn!(
                        "Save thread I/O error; affected file count: {}",
                        error_list.len()
                    );
                }
                let _ = error_sender.send(error_list.clone());
                previous_errors = error_list;
            }
        }

        // 2/6. 等待命令（≤1s）。
        match commands.recv_timeout(Duration::from_secs(1)) {
            Ok(AsyncSaveCmd::Shutdown { flush }) => {
                if flush {
                    // 11=Shutdown{flush:true}：flush_deferred_and_sync + drop + 退出线程。
                    if let Some(file) = current.as_mut() {
                        if !file.flush_deferred_and_sync() {
                            log::warn!("Error flushing save file on game thread disconnect.");
                        }
                    }
                    // current drop + error_sender/commands receiver drop + return 退出。
                    return;
                }
                // 11=Shutdown{flush:false}：应用 deferred（有活动档时）+ 睡 10ms + 继续循环。
                if let Some(file) = current.as_mut() {
                    file.flush_deferred_and_sync();
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(cmd) => {
                // 5. 每次处理命令前消费一个 GC 释放并断言属于当前档。
                if let Some(file) = current.as_mut() {
                    file.consume_snapshot_gc_release();
                }
                // 8. OpenFile 前置：unsaved 列表命中 history.json → panic。
                if let AsyncSaveCmd::OpenFile { session, .. } = &cmd {
                    let history = session.save_folder_path().join("history.json");
                    if unsaved.contains_file(&history) {
                        panic!("Was asked to load an unsaved file");
                    }
                }
                execute_save_cmd(&mut current, cmd, &unsaved);
            }
            Err(_) => {
                // 超时：无命令 → 有活动档时应用 deferred（原生每轮无命令即 apply），
                // 进入下一轮（diff 已在每轮开头执行）。
                if let Some(file) = current.as_mut() {
                    file.flush_deferred_and_sync();
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 生产 spawner（O：run_game_thread 装配 save_thread；经 add_save_load_systems_impl
// 注册 `init_save_thread`，由中央仲裁接入 startup/run_game.rs）
// ---------------------------------------------------------------------------

/// 存档线程生产句柄（游戏线程侧资源）。`join` 供收尾等待；`command_sender` 经
/// `AutoSaveState::attach_command_sender` 装配；`error_state` 供游戏线程吸干错误列表。
/// 原生无 shutdown_ack（Shutdown{flush:true} 直接退出线程），已删。
#[derive(bevy_ecs::resource::Resource)]
pub struct SaveThreadHandle {
    pub join: JoinHandle<()>,
    pub command_sender: crossbeam_channel::Sender<AsyncSaveCmd>,
    pub error_state: AutosaveErrorState,
}

impl SaveThreadHandle {
    /// 生产存档线程（O：主循环 `save_thread` + 通道对 + unsaved 共享列表）。
    pub fn spawn() -> Self {
        Self::spawn_with_unsaved(Arc::new(UnsavedFileList::default()))
    }

    pub fn spawn_with_unsaved(unsaved: Arc<UnsavedFileList>) -> Self {
        let (command_sender, command_receiver) = crossbeam_channel::unbounded();
        let (error_sender, error_state) = AutosaveErrorState::channel();
        let join = std::thread::Builder::new()
            .name("Async save".into())
            .spawn(move || save_thread(command_receiver, error_sender, unsaved))
            .expect("spawning Async save");
        Self {
            join,
            command_sender,
            error_state,
        }
    }
}

/// `init_save_thread`（O：run_game_thread 装配点）：生产存档线程并把命令发送端
/// 绑定到 `AutoSaveState`。已在装配（command_sender 已附）则跳过（幂等）。
pub fn init_save_thread(mut auto: ResMut<super::save_game::AutoSaveState>, mut commands: Commands) {
    if auto.is_command_attached() {
        return;
    }
    let handle = SaveThreadHandle::spawn_with_unsaved(auto.unsaved_files.clone());
    auto.attach_command_sender(handle.command_sender.clone());
    commands.insert_resource(handle);
}

#[cfg(test)]
mod synchronous_flush_recovery_tests {
    use super::*;
    use crate::systems::save::save_game::{GameSaveSections, SerializedJson};
    #[test]
    fn native_sync_reply_follows_deferred_sections_and_actual_disk_sync() {
        let dir = tempfile::tempdir().unwrap();
        let values = [
            serde_json::json!(27),
            serde_json::json!(17u64),
            serde_json::json!(0),
            serde_json::json!({"before":true}),
            serde_json::json!({}),
            serde_json::json!([]),
            serde_json::json!({}),
            serde_json::json!(-1),
            serde_json::json!(-1),
            serde_json::json!([]),
        ];
        let sections = GameSaveSections {
            values: values.map(|value| SerializedJson::new(&value).unwrap()),
        };
        let mut current = Some(CurrentSaveFile::create(dir.path(), sections, &[]));
        let captured = serde_json::json!({"captured_from_live_camera":"x".repeat(300)});
        current.as_mut().unwrap().queue_section_change(
            SaveFileSection::Camera,
            SerializedJson::new(&captured).unwrap(),
        );
        let (reply, completed) = crossbeam_channel::bounded(1);
        execute_save_cmd(
            &mut current,
            AsyncSaveCmd::Flush { reply },
            &UnsavedFileList::default(),
        );
        completed.recv().unwrap().unwrap();
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(dir.path().join("history.json")).unwrap())
                .unwrap();
        assert_eq!(saved["Camera"], captured);
    }
    #[test]
    #[should_panic(expected = "No active save file")]
    fn native_synchronous_flush_without_active_file_is_rejected() {
        let (reply, _completed) = crossbeam_channel::bounded(1);
        execute_save_cmd(
            &mut None,
            AsyncSaveCmd::Flush { reply },
            &UnsavedFileList::default(),
        );
    }
}

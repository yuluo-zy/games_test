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
use bevy_ecs::prelude::{Commands, ResMut, World};
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
    pub expires_at: std::time::Instant,
}

/// 保留现有路径标记API；opaque不是可恢复的ShadowFile数据，不能用它承载失败写入。
/// 原B128字节条目的真实类型已由retain_mut<ShadowFile>和闭包14130f900确认。
/// 真正写入失败的文件经UnsavedFileList::add_file转移所有权；仅路径标记用于既有加载保护。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsavedFileEntry {
    pub path: String,
    pub opaque: [u8; 0x68],
}
impl UnsavedFileEntry {
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            path: path.to_string_lossy().into_owned(),
            opaque: [0u8; 0x68],
        }
    }
}
const _: () = assert!(std::mem::size_of::<UnsavedFileEntry>() == 0x80);

#[derive(Debug)]
enum UnsavedFile {
    /// 当前加载保护接口的路径标记，不伪造不存在的文件内存或成功重试。
    Marked(UnsavedFileEntry),
    /// 原UnsavedFileList真正保留的ShadowFile：后续tick重试，成功后释放。
    Pending(ShadowFile),
}
impl UnsavedFile {
    fn path(&self) -> &Path {
        match self {
            Self::Marked(entry) => Path::new(&entry.path),
            Self::Pending(file) => file.path(),
        }
    }
}

/// 未落盘文件列表：保留当前共享mutex owner，原add/contains_file的路径保护仍可用。
/// 不复制原生私有ABI；实际失败文件必须保留完整ShadowFile及内存数据。
#[derive(Debug, Default)]
pub struct UnsavedFileList {
    files: parking_lot::Mutex<Vec<UnsavedFile>>,
}
impl UnsavedFileList {
    pub fn add(&self, entry: UnsavedFileEntry) {
        self.files.lock().push(UnsavedFile::Marked(entry));
    }
    pub fn add_path(&self, path: PathBuf) {
        self.add(UnsavedFileEntry::from_path(path));
    }
    /// 原14171ff80把ShadowFile整体移入列表，而不是仅拷贝路径后丢弃脏数据。
    pub fn add_file(&self, file: ShadowFile) {
        self.files.lock().push(UnsavedFile::Pending(file));
    }
    pub fn contains_file(&self, path: &Path) -> bool {
        self.files
            .lock()
            .iter()
            .any(|candidate| candidate.path() == path)
    }
    pub fn len(&self) -> usize {
        self.files.lock().len()
    }
    /// 原14130f900：sync失败保留并按顺序追加错误；成功移除，释放真实文件owner。
    fn retry_pending(&self, errors: &mut Vec<(PathBuf, AutoSaveError)>) {
        self.files.lock().retain_mut(|entry| {
            let UnsavedFile::Pending(file) = entry else {
                return true;
            };
            if file.sync_all() {
                log::info!("I/O error resolved. Save file successfully written.");
                false
            } else {
                if let Some(error) = file.last_error() {
                    errors.push((file.path().to_owned(), AutoSaveError::from_error(error)));
                }
                true
            }
        });
    }
}

/// 单文件自动保存失败（原B `AutoSaveError::from` 0x141720490）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutoSaveError {
    /// 原tag0：真实I/O StorageFull，载荷是该I/O错误的文字，不含外层anyhow上下文。
    StorageFull(String),
    /// 原tag1：完整anyhow显示链。
    Io(String),
}
impl AutoSaveError {
    /// 原转换先downcast到I/O错误，再按ErrorKind裁决，不能按消息文字猜测。
    fn from_error(error: &anyhow::Error) -> Self {
        if let Some(io) = error
            .downcast_ref::<std::io::Error>()
            .filter(|io| io.kind() == std::io::ErrorKind::StorageFull)
        {
            Self::StorageFull(format!("{io:#}"))
        } else {
            Self::Io(format!("{error:#}"))
        }
    }
    pub fn message(&self) -> &str {
        match self {
            Self::StorageFull(message) | Self::Io(message) => message,
        }
    }
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
        let slot = &mut self.deferred_sections[section as usize];
        // Replacements retain the first deadline instead of extending it forever.
        let expires_at = slot
            .as_ref()
            .map(|pending| pending.expires_at)
            .unwrap_or_else(|| std::time::Instant::now() + Duration::from_secs(120));
        *slot = Some(DeferredSection {
            section,
            value,
            expires_at,
        });
    }

    /// Original idle path @1417210e0..11a2 applies only expired slots.
    /// Forced Flush/Close uses flush_deferred_and_sync and ignores deadlines.
    pub fn apply_due_sections(&mut self, now: std::time::Instant) {
        for index in 0..self.deferred_sections.len() {
            if self.deferred_sections[index]
                .as_ref()
                .is_some_and(|pending| pending.expires_at <= now)
            {
                let pending = self.deferred_sections[index].take().unwrap();
                self.change_section(pending.section, &pending.value);
            }
        }
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

    /// B @141724033..726d78: complete Motomoto blob -> deterministic
    /// SipHash13(slice length + bytes) -> existing-file shortcut or atomic
    /// NamedTempFile write/persist -> publish the same ledger ID owner.
    pub fn capture_snapshot(
        &mut self,
        snapshot: &Arc<SnapshotSaveData>,
        pending: &crate::resources::history::MaybePendingSnapshotId,
    ) -> bool {
        // The domain owner supplies the single Result encoder. Never turn an
        // encoding failure into a partially populated JSON snapshot.
        let result = snapshot
            .encode()
            .and_then(|blob| self.write_snapshot_blob(&blob));
        match result {
            Ok(id) => {
                pending.assign(id);
                true
            }
            Err(error) => {
                pending.assign_invalid();
                log::error!("Error while capturing a game snapshot: {error:#}");
                false
            }
        }
    }

    fn write_snapshot_blob(
        &mut self,
        blob: &[u8],
    ) -> anyhow::Result<crate::resources::history::HistorySnapshotId> {
        use std::io::Write;
        // Native utils::calculate_hash<&[u8]> @14145b1a0 writes usize length
        // then the bytes, using SipHash13 with both keys zero. The canonical
        // helper lives in the shared `utils` crate.
        let id = crate::resources::history::HistorySnapshotId(::utils::calculate_hash(blob));
        let folder = self.shadow_file.directory().join("snapshots");
        let target = folder.join(format!("{}.snapshot", id.0));
        if !target.is_file() {
            // Native uses DirBuilder with recursive=true, then new_in,
            // write_all and persist. It does not add a snapshot fsync call.
            std::fs::create_dir_all(&folder)?;
            let mut temporary = tempfile::NamedTempFile::new_in(&folder)?;
            temporary.write_all(blob)?;
            temporary
                .persist(&target)
                .map_err(|error| anyhow::anyhow!(error.error))?;
        }
        Ok(id)
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
            // case4：失败时把真实ShadowFile转移给共享重试列表，不能只留下路径。
            if let Some(mut previous) = current.take() {
                if !previous.flush_deferred_and_sync() {
                    unsaved.add_file(previous.shadow_file);
                }
            }
        }
        AsyncSaveCmd::ChangeSection { section, value } => {
            let file = current
                .as_mut()
                .expect("No active save file, but AsyncSaveCmd::ChangeSection has been issued");
            // Original vtable+18 invocation @14172312f happens here, not on
            // the gameplay thread. It sees IDs assigned by earlier tag9 work.
            let value = value.to_json_string();
            if matches!(
                section,
                SaveFileSection::Camera | SaveFileSection::TimeOfDay | SaveFileSection::Sheep
            ) {
                file.queue_section_change(section, value);
            } else {
                file.change_section(section, &value);
                file.shadow_file.flush();
            }
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
        AsyncSaveCmd::CaptureSnapshot { snapshot, pending } => {
            let file = current
                .as_mut()
                .expect("No active save file, but AsyncSaveCmd::CaptureSnapshot has been issued");
            file.capture_snapshot(&snapshot, &pending);
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
    if let Some(mut previous) = current.take() {
        if !previous.flush_deferred_and_sync() {
            log::warn!("Flush error while closing previous save file (upon open request)");
            unsaved.add_file(previous.shadow_file);
        }
    }
    let new_file = CurrentSaveFile::create(session.save_folder_path(), sections, &edits);
    if !new_file.shadow_file.is_open() {
        log::error!("Rewriting history file");
    }
    *current = Some(new_file);
}

/// B save_thread @141720680, full5139B. Keep the existing crossbeam channel
/// architecture while restoring its actual TryRecvError branches: Empty applies
/// due sections and sleeps10ms; Disconnected forces all deferred data to disk
/// and returns. The native outer Result niche11 is not an AsyncSaveCmd variant.
/// A one-second tick syncs dirty shadow data and reports the actual error diff.
pub fn save_thread(
    commands: Receiver<AsyncSaveCmd>,
    error_sender: crossbeam_channel::Sender<Vec<(PathBuf, AutoSaveError)>>,
    unsaved: Arc<UnsavedFileList>,
) {
    save_thread_with_current(commands, error_sender, unsaved, None);
}

// One worker loop shared by production and the isolated-file CPU regression.
// Production always enters with None; tests supply an actual temporary file
// rather than mutating the process-global account paths.
fn save_thread_with_current(
    commands: Receiver<AsyncSaveCmd>,
    error_sender: crossbeam_channel::Sender<Vec<(PathBuf, AutoSaveError)>>,
    unsaved: Arc<UnsavedFileList>,
    mut current: Option<CurrentSaveFile>,
) {
    let mut previous_errors: Vec<(PathBuf, AutoSaveError)> = Vec::new();
    let mut last_tick = std::time::Instant::now();

    loop {
        // 1. ≥1s tick：flush 第一个脏影子文件。
        if last_tick.elapsed() >= Duration::from_secs(1) {
            let mut error_list: Vec<(PathBuf, AutoSaveError)> = Vec::new();
            if let Some(file) = current.as_mut() {
                // 原141720823/83d按last_error保留重试；OS sync失败时dirty可能已清空。
                if file.shadow_file.last_error().is_some()
                    || file.shadow_file.dirty_blocks().next().is_some()
                {
                    if !file.shadow_file.sync_all() {
                        let path = file.shadow_file.path().to_path_buf();
                        let error = file
                            .shadow_file
                            .last_error()
                            .map(AutoSaveError::from_error)
                            .unwrap_or_else(|| AutoSaveError::Io("sync failed".to_string()));
                        error_list.push((path, error));
                    }
                }
            }
            // 原mutex内retain_mut的错误接在当前文件之后，成功文件从共享列表卸载。
            unsaved.retry_pending(&mut error_list);
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

        // B @141720f8c tests the outer try_recv Result niche0xb. Its byte
        // payload is TryRecvError::{Empty,Disconnected}, not a Shutdown command.
        match commands.try_recv() {
            Ok(cmd) => {
                if let Some(file) = current.as_mut() {
                    file.consume_snapshot_gc_release();
                }
                if let AsyncSaveCmd::OpenFile { session, .. } = &cmd {
                    let history = session.save_folder_path().join("history.json");
                    assert!(
                        !unsaved.contains_file(&history),
                        "Was asked to load an unsaved file"
                    );
                }
                execute_save_cmd(&mut current, cmd, &unsaved);
            }
            Err(crossbeam_channel::TryRecvError::Empty) => {
                if let Some(file) = current.as_mut() {
                    file.apply_due_sections(std::time::Instant::now());
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                if let Some(file) = current.as_mut() {
                    if !file.flush_deferred_and_sync() {
                        log::warn!("Error flushing save file on game thread disconnect.");
                    }
                }
                return;
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
/// The stopped host must call shutdown_save_thread to disconnect both actual
/// senders and join this worker; dropping this handle alone detaches it.
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

/// Current host disposal adapter, called only after this World's schedules have
/// stopped permanently. This joins the actual worker and reports known errors;
/// it does not prove durability: the original disconnect flush is warn-only.
pub fn shutdown_save_thread(world: &mut World) -> anyhow::Result<()> {
    let (auto_present, attached, mut failure, unsaved) = match world
        .get_resource_mut::<super::save_game::AutoSaveState>()
    {
        Some(mut auto) => {
            let attached = auto.is_command_attached();
            let unsaved = auto.unsaved_files.clone();
            (true, attached, auto.detach_for_shutdown(), Some(unsaved))
        }
        None => (false, false, None, None),
    };
    if let Some(handle) = world.remove_resource::<SaveThreadHandle>() {
        let SaveThreadHandle { join, command_sender, mut error_state } = handle;
        // AutoSaveState's private sender has already been dropped above.
        drop(command_sender);
        if let Err(payload) = join.join() {
            let message = if let Some(message) = payload.downcast_ref::<String>() {
                message.as_str()
            } else if let Some(message) = payload.downcast_ref::<&'static str>() {
                *message
            } else {
                "non-string panic payload"
            };
            retain_first_shutdown_error(&mut failure,
                anyhow::anyhow!("Async save worker panicked while stopping: {message}"));
        }
        // Read the real worker channel only after its final send/termination.
        error_state.drain_pending();
        if !error_state.is_ok() {
            retain_first_shutdown_error(&mut failure,
                anyhow::anyhow!("Async save worker stopped with reported I/O errors: {:?}", error_state.errors()));
        }
        if !auto_present {
            retain_first_shutdown_error(&mut failure,
                anyhow::anyhow!("Async save worker joined without its canonical AutoSaveState; shared unsaved status is unknown"));
        }
    } else if attached {
        retain_first_shutdown_error(&mut failure,
            anyhow::anyhow!("AutoSaveState had an attached sender but no SaveThreadHandle; worker termination could not be joined"));
    }
    if let Some(unsaved) = unsaved {
        let count = unsaved.len();
        if count != 0 {
            retain_first_shutdown_error(&mut failure,
                anyhow::anyhow!("Async save stopped with {count} known unsaved file(s)"));
        }
    }
    match failure {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn retain_first_shutdown_error(first: &mut Option<anyhow::Error>, error: anyhow::Error) {
    if first.is_some() {
        log::error!("Additional save shutdown failure: {error:#}");
    } else {
        *first = Some(error);
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

#[cfg(test)]
mod native_deferred_tests {
    use super::*;

    #[test]
    fn native_section_deadline_is_retained_and_forced_flush_overrides_it() {
        let root = tempfile::tempdir().unwrap();
        let sections = GameSaveSections {
            values: std::array::from_fn(|_| SerializedJson::new(&0).unwrap()),
        };
        let mut file = CurrentSaveFile::create(root.path(), sections, &[]);
        file.queue_section_change(SaveFileSection::Camera, SerializedJson::new(&1).unwrap());
        let deadline = file.deferred_sections[SaveFileSection::Camera as usize]
            .as_ref()
            .unwrap()
            .expires_at;
        file.queue_section_change(SaveFileSection::Camera, SerializedJson::new(&2).unwrap());
        assert_eq!(
            file.deferred_sections[SaveFileSection::Camera as usize]
                .as_ref()
                .unwrap()
                .expires_at,
            deadline
        );
        file.apply_due_sections(deadline - Duration::from_millis(1));
        assert!(file.deferred_sections[SaveFileSection::Camera as usize].is_some());
        assert!(file.flush_deferred_and_sync());
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(root.path().join("history.json")).unwrap())
                .unwrap();
        assert_eq!(saved["Camera"], 2);
    }

    #[test]
    fn native_snapshot_reference_serialization_runs_on_the_worker_after_id_assignment() {
        use super::super::save_game::{defer_json, SnapshotReferencesToJson};
        use crate::resources::history::{
            transactions::ResolvedHistorySnapshot, HistorySnapshotId, MaybePendingSnapshotId,
        };
        let root = tempfile::tempdir().unwrap();
        let sections = GameSaveSections {
            values: std::array::from_fn(|_| SerializedJson::new(&0).unwrap()),
        };
        let mut current = Some(CurrentSaveFile::create(root.path(), sections, &[]));
        let pending = MaybePendingSnapshotId::new();
        let command = AsyncSaveCmd::ChangeSection {
            section: SaveFileSection::Snapshots,
            value: defer_json(SnapshotReferencesToJson(vec![ResolvedHistorySnapshot {
                chain_index: 4,
                value: pending.clone(),
            }])),
        };
        // Queue ownership is established while still pending; real tag9 assigns
        // this exact atomic before the subsequent tag6 is handled.
        pending.assign(HistorySnapshotId(73));
        execute_save_cmd(&mut current, command, &UnsavedFileList::default());
        let file = current.as_mut().unwrap();
        assert!(file.flush_deferred_and_sync());
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(root.path().join("history.json")).unwrap())
                .unwrap();
        assert_eq!(
            saved["History"]["snapshots"],
            serde_json::json!([{"chain_index":4,"snapshot_id":73}])
        );
    }
}

#[cfg(test)]
mod original_snapshot_file_tests {
    use super::*;
    #[test]
    fn snapshot_blob_identity_and_existing_file_shortcut_use_the_same_content_hash() {
        let root = tempfile::tempdir().unwrap();
        let sections = GameSaveSections {
            values: SaveFileSection::ALL.map(|_| SerializedJson::new(&0).unwrap()),
        };
        let mut file = CurrentSaveFile::create(root.path(), sections, &[]);
        // This exercises the writer boundary after the domain encoder: bytes
        // are opaque here, not a replacement valid-game snapshot fixture.
        let blob = b"encoded-domain-blob";
        let id = file.write_snapshot_blob(blob).unwrap();
        assert!(id.0 >= 2);
        let target = root
            .path()
            .join("snapshots")
            .join(format!("{}.snapshot", id.0));
        assert_eq!(std::fs::read(&target).unwrap(), blob);
        std::fs::write(&target, b"existing").unwrap();
        assert_eq!(file.write_snapshot_blob(blob).unwrap(), id);
        assert_eq!(std::fs::read(&target).unwrap(), b"existing");
        let other = file.write_snapshot_blob(b"encoded-other-blob").unwrap();
        assert_ne!(id, other);
    }
}

#[cfg(test)]
mod shutdown_save_host_tests {
    use super::*;
    use crate::systems::save::save_game::{AutoSaveState, defer_json};

    fn insert_actual_inactive_worker(world: &mut World, mut auto: AutoSaveState) {
        let handle = SaveThreadHandle::spawn_with_unsaved(auto.unsaved_files.clone());
        auto.attach_command_sender(handle.command_sender.clone());
        world.insert_resource(auto);
        world.insert_resource(handle);
    }

    #[test]
    fn shutdown_without_active_file_joins_and_keeps_the_same_unsaved_owner() {
        let mut world = World::new();
        shutdown_save_thread(&mut world).unwrap();
        let auto = AutoSaveState::default();
        let unsaved = auto.unsaved_files.clone();
        insert_actual_inactive_worker(&mut world, auto);
        shutdown_save_thread(&mut world).unwrap();
        let auto = world.resource::<AutoSaveState>();
        assert!(!auto.is_command_attached());
        assert!(Arc::ptr_eq(&unsaved, &auto.unsaved_files));
        assert!(!world.contains_resource::<SaveThreadHandle>());
        shutdown_save_thread(&mut world).unwrap();
    }

    #[test]
    fn disconnect_runs_the_real_worker_deferred_flush_into_an_actual_file() {
        let directory = tempfile::tempdir().unwrap();
        let values = [serde_json::json!(27), serde_json::json!(17u64),
            serde_json::json!(0), serde_json::json!({"before":true}),
            serde_json::json!({}), serde_json::json!([]), serde_json::json!({}),
            serde_json::json!(-1), serde_json::json!(-1), serde_json::json!([])];
        let sections = GameSaveSections {
            values: values.map(|value| SerializedJson::new(&value).unwrap()),
        };
        let current = CurrentSaveFile::create(directory.path(), sections, &[]);
        let mut auto = AutoSaveState::default();
        let unsaved = auto.unsaved_files.clone();
        let worker_unsaved = unsaved.clone();
        let (command_sender, receiver) = crossbeam_channel::unbounded();
        let (error_sender, error_state) = AutosaveErrorState::channel();
        let join = std::thread::spawn(move || {
            // Exact production loop; only the initial real file is supplied so
            // this fixture does not redirect process-global account paths.
            save_thread_with_current(receiver, error_sender, worker_unsaved, Some(current));
        });
        auto.attach_command_sender(command_sender.clone());
        let expected = serde_json::json!({"camera_at_disconnect":"x".repeat(300)});
        auto.send_command_expect(AsyncSaveCmd::ChangeSection {
            section: SaveFileSection::Camera, value: defer_json(expected.clone()),
        });
        let mut world = World::new();
        world.insert_resource(auto);
        world.insert_resource(SaveThreadHandle { join, command_sender, error_state });
        shutdown_save_thread(&mut world).unwrap();
        let saved: serde_json::Value = serde_json::from_slice(
            &std::fs::read(directory.path().join("history.json")).unwrap()).unwrap();
        assert_eq!(saved[SaveFileSection::Camera.json_key()], expected);
        assert!(Arc::ptr_eq(&unsaved, &world.resource::<AutoSaveState>().unsaved_files));
    }

    #[test]
    fn attached_without_handle_reports_unknown_join_and_disconnects_its_sender() {
        let mut world = World::new();
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut auto = AutoSaveState::default();
        auto.attach_command_sender(sender);
        world.insert_resource(auto);
        let error = shutdown_save_thread(&mut world).unwrap_err();
        assert!(error.to_string().contains("no SaveThreadHandle"));
        assert!(matches!(receiver.try_recv(), Err(crossbeam_channel::TryRecvError::Disconnected)));
    }

    #[test]
    fn previous_auto_error_does_not_skip_join_or_hide_it_behind_later_failures() {
        let mut world = World::new();
        let mut auto = AutoSaveState::default();
        assert!(!auto.send_command(AsyncSaveCmd::ChangeSection {
            section: SaveFileSection::Camera, value: defer_json(serde_json::json!({})),
        }));
        auto.unsaved_files.add_path(PathBuf::from("previously-unsaved.json"));
        let (command_sender, receiver) = crossbeam_channel::unbounded();
        let (error_sender, error_state) = AutosaveErrorState::channel();
        let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let worker_stopped = stopped.clone();
        let join = std::thread::spawn(move || {
            assert!(matches!(receiver.recv_timeout(Duration::from_secs(2)),
                Err(crossbeam_channel::RecvTimeoutError::Disconnected)));
            error_sender.send(vec![(PathBuf::from("worker.json"), AutoSaveError::Io("disk failure".into()))]).unwrap();
            worker_stopped.store(true, std::sync::atomic::Ordering::Release);
            panic!("worker panic retained as secondary");
        });
        auto.attach_command_sender(command_sender.clone());
        world.insert_resource(auto);
        world.insert_resource(SaveThreadHandle { join, command_sender, error_state });
        let error = shutdown_save_thread(&mut world).unwrap_err();
        assert_eq!(error.to_string(), "save command sender not attached");
        assert!(stopped.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(world.resource::<AutoSaveState>().unsaved_files.len(), 1);
        assert!(!world.contains_resource::<SaveThreadHandle>());
    }

    #[test]
    fn worker_error_diff_and_unsaved_list_are_observable_failures_after_join() {
        for reported_error in [false, true] {
            let mut auto = AutoSaveState::default();
            if !reported_error {
                auto.unsaved_files.add_path(PathBuf::from("known-unsaved.json"));
            }
            let (command_sender, receiver) = crossbeam_channel::unbounded();
            let (error_sender, error_state) = AutosaveErrorState::channel();
            let join = std::thread::spawn(move || {
                assert!(matches!(receiver.recv_timeout(Duration::from_secs(2)),
                    Err(crossbeam_channel::RecvTimeoutError::Disconnected)));
                if reported_error {
                    error_sender.send(vec![(PathBuf::from("disk.json"), AutoSaveError::Io("real reported error".into()))]).unwrap();
                }
            });
            auto.attach_command_sender(command_sender.clone());
            let mut world = World::new();
            world.insert_resource(auto);
            world.insert_resource(SaveThreadHandle { join, command_sender, error_state });
            let error = shutdown_save_thread(&mut world).unwrap_err();
            assert!(error.to_string().contains(if reported_error {"reported I/O errors"} else {"known unsaved"}));
        }
    }
}


#[cfg(test)]
mod autosave_error_contract_tests {
    use super::*;

    #[test]
    fn error_classification_uses_nested_io_kind_and_preserves_context() {
        let error = anyhow::Error::new(std::io::Error::from(std::io::ErrorKind::StorageFull))
            .context("Sync data")
            .context("Saving history.json");
        let classified = AutoSaveError::from_error(&error);
        assert!(matches!(classified, AutoSaveError::StorageFull(_)));
        assert!(
            !classified.message().contains("Saving history.json"),
            "原StorageFull只显示下层I/O载荷"
        );
        assert_eq!(
            classified.message(),
            std::io::Error::from(std::io::ErrorKind::StorageFull).to_string()
        );
        let generic =
            anyhow::Error::new(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
                .context("Sync data")
                .context("Saving history.json");
        assert!(
            AutoSaveError::from_error(&generic)
                .message()
                .contains("Saving history.json: Sync data:")
        );
        for error in [
            anyhow::anyhow!("disk full"),
            anyhow::Error::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "disk full",
            )),
            anyhow::Error::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                std::io::Error::new(std::io::ErrorKind::StorageFull, "disk full"),
            )),
        ] {
            let classified = AutoSaveError::from_error(&error);
            assert!(
                matches!(classified, AutoSaveError::Io(_)),
                "文本不能替代真实I/O分类"
            );
            assert!(classified.message().contains("disk full"));
        }
    }

    #[test]
    fn queued_error_snapshots_replace_in_order_and_disconnection_keeps_last_state() {
        let (sender, mut state) = AutosaveErrorState::channel();
        assert!(state.is_ok());
        sender
            .send(vec![(
                PathBuf::from("first"),
                AutoSaveError::Io("first".into()),
            )])
            .unwrap();
        sender.send(Vec::new()).unwrap();
        sender
            .send(vec![(
                PathBuf::from("last"),
                AutoSaveError::StorageFull("last".into()),
            )])
            .unwrap();
        state.drain_pending();
        assert_eq!(
            state.errors(),
            &[(
                PathBuf::from("last"),
                AutoSaveError::StorageFull("last".into())
            )]
        );
        drop(sender);
        state.drain_pending();
        assert!(!state.is_ok(), "断链不应伪造I/O恢复");
    }

    #[test]
    fn close_failure_retains_deferred_file_bytes_until_real_retry_succeeds() {
        let root = tempfile::tempdir().unwrap();
        let missing = root.path().join("old-session");
        let values = [
            serde_json::json!(27),
            serde_json::json!(17u64),
            serde_json::json!(0),
            serde_json::json!({}),
            serde_json::json!({}),
            serde_json::json!([]),
            serde_json::json!({}),
            serde_json::json!(-1),
            serde_json::json!(-1),
            serde_json::json!([]),
        ];
        let mut file = CurrentSaveFile::create(
            &missing,
            GameSaveSections {
                values: values.map(|value| SerializedJson::new(&value).unwrap()),
            },
            &[],
        );
        let expected = serde_json::json!({"camera_at_close": "仍在内存的原始编辑".repeat(32)});
        file.queue_section_change(
            SaveFileSection::Camera,
            SerializedJson::new(&expected).unwrap(),
        );
        let unsaved = Arc::new(UnsavedFileList::default());
        let pending = unsaved.clone();
        let (commands, command_receiver) = crossbeam_channel::unbounded();
        let (errors, receiver) = crossbeam_channel::unbounded();
        let worker = std::thread::spawn(move || {
            save_thread_with_current(command_receiver, errors, pending, Some(file))
        });
        commands
            .send(AsyncSaveCmd::CloseFile {
                session: SessionId::new(),
            })
            .unwrap();
        let failure = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(failure.len(), 1);
        assert_eq!(failure[0].0, missing.join("history.json"));
        assert_eq!(unsaved.len(), 1);
        assert!(unsaved.contains_file(&missing.join("history.json")));
        std::fs::create_dir(&missing).unwrap();
        assert!(
            receiver
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
                .is_empty()
        );
        assert_eq!(unsaved.len(), 0, "成功落盘才卸载并解除加载保护");
        assert!(!unsaved.contains_file(&missing.join("history.json")));
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(missing.join("history.json")).unwrap()).unwrap();
        assert_eq!(
            saved[SaveFileSection::Camera.json_key()],
            expected,
            "CloseFile必须保留已应用的deferred内容，不能只存路径"
        );
        drop(commands);
        worker.join().unwrap();
    }

    #[test]
    fn multiple_pending_files_keep_order_and_do_not_report_early_recovery() {
        let root = tempfile::tempdir().unwrap();
        let first = root.path().join("first");
        let second = root.path().join("second");
        let unsaved = Arc::new(UnsavedFileList::default());
        unsaved.add_file(ShadowFile::create(
            b"first bytes".to_vec(),
            &first,
            "data.json",
        ));
        unsaved.add_file(ShadowFile::create(
            b"second bytes".to_vec(),
            &second,
            "data.json",
        ));
        let pending = unsaved.clone();
        let (commands, command_receiver) = crossbeam_channel::unbounded();
        let (errors, receiver) = crossbeam_channel::unbounded();
        let worker = std::thread::spawn(move || {
            save_thread_with_current(command_receiver, errors, pending, None)
        });
        let initial = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(
            initial.iter().map(|(path, _)| path).collect::<Vec<_>>(),
            vec![&first.join("data.json"), &second.join("data.json")]
        );
        std::fs::create_dir(&first).unwrap();
        let partial = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(partial.len(), 1, "一个文件恢复不能清空另一个真实错误");
        assert_eq!(partial[0].0, second.join("data.json"));
        assert_eq!(unsaved.len(), 1);
        assert_eq!(
            std::fs::read(first.join("data.json")).unwrap(),
            b"first bytes"
        );
        std::fs::create_dir(&second).unwrap();
        assert!(
            receiver
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
                .is_empty()
        );
        assert_eq!(unsaved.len(), 0);
        assert_eq!(
            std::fs::read(second.join("data.json")).unwrap(),
            b"second bytes"
        );
        drop(commands);
        worker.join().unwrap();
    }

    #[test]
    fn worker_retries_reported_error_even_without_dirty_blocks() {
        let directory = tempfile::tempdir().unwrap();
        let missing = directory.path().join("empty-file-parent");
        // 空文件创建失败会保留真实错误而没有任何脏块，独立验证last_error重试门控。
        let current = CurrentSaveFile {
            shadow_file: ShadowFile::create(Vec::new(), &missing, "empty.json"),
            save_file_structure: SaveFileStructure {
                section_ranges: Default::default(),
                history_edit_offsets: Vec::new(),
            },
            deferred_sections: std::array::from_fn(|_| None),
            locked_backups: HashSet::new(),
            snapshot_gc_releases: None,
        };
        assert!(current.shadow_file.last_error().is_some());
        assert!(current.shadow_file.dirty_blocks().next().is_none());
        let (commands, command_receiver) = crossbeam_channel::unbounded();
        let (errors, receiver) = crossbeam_channel::unbounded();
        let worker = std::thread::spawn(move || {
            save_thread_with_current(
                command_receiver,
                errors,
                Arc::new(UnsavedFileList::default()),
                Some(current),
            )
        });
        let failure = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(failure.len(), 1);
        std::fs::create_dir(&missing).unwrap();
        assert!(
            receiver
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
                .is_empty()
        );
        assert_eq!(std::fs::read(missing.join("empty.json")).unwrap(), b"");
        drop(commands);
        worker.join().unwrap();
    }

    #[test]
    fn real_worker_reports_failed_disk_write_and_later_successful_retry() {
        let directory = tempfile::tempdir().unwrap();
        let missing = directory.path().join("initially-missing");
        let values = [
            serde_json::json!(27),
            serde_json::json!(17u64),
            serde_json::json!(0),
            serde_json::json!({}),
            serde_json::json!({}),
            serde_json::json!([]),
            serde_json::json!({}),
            serde_json::json!(-1),
            serde_json::json!(-1),
            serde_json::json!([]),
        ];
        let current = CurrentSaveFile::create(
            &missing,
            GameSaveSections {
                values: values.map(|value| SerializedJson::new(&value).unwrap()),
            },
            &[],
        );
        assert!(current.shadow_file.last_error().is_some());
        let (commands, command_receiver) = crossbeam_channel::unbounded();
        let (errors, receiver) = crossbeam_channel::unbounded();
        let worker = std::thread::spawn(move || {
            save_thread_with_current(
                command_receiver,
                errors,
                Arc::new(UnsavedFileList::default()),
                Some(current),
            )
        });
        let failure = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(failure.len(), 1);
        assert_eq!(failure[0].0, missing.join("history.json"));
        assert!(matches!(failure[0].1, AutoSaveError::Io(_)));
        std::fs::create_dir(&missing).unwrap();
        let recovery = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert!(recovery.is_empty(), "真实重试落盘成功后发布空错误快照");
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(missing.join("history.json")).unwrap()).unwrap();
        assert!(saved.is_object());
        drop(commands);
        worker.join().unwrap();
    }
}

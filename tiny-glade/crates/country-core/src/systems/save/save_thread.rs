//! Mutable file portion of CurrentSaveFile; native offsets +0x10 shadow,
//! +0x90 history offsets, +0xa8 section ranges. Async thread orchestration,
//! snapshot capture and the complete asynchronous command queue remain outside
//! this recovered slice. The snapshot-GC lock/release consumer is included.
use super::{
    save_game::{
        save_game_to_memory, GameSaveSections, SaveFileSection, SaveFileStructure, SerializedJson,
    },
    shadow_file::ShadowFile,
};
use crossbeam_channel::Receiver;
use hashbrown::HashSet;
use std::{
    path::{Path, PathBuf},
    thread::JoinHandle,
};

#[derive(Debug)]
pub struct CurrentSaveFile {
    pub shadow_file: ShadowFile,
    pub save_file_structure: SaveFileStructure,
    locked_backups: HashSet<PathBuf>,
    snapshot_gc_releases: Option<Receiver<PathBuf>>,
}

impl CurrentSaveFile {
    pub fn create(
        directory: impl AsRef<Path>,
        sections: GameSaveSections,
        edits: &[SerializedJson],
    ) -> Self {
        let (bytes, structure) = save_game_to_memory(sections, edits);
        Self {
            shadow_file: ShadowFile::create(bytes, directory, "history.json"),
            save_file_structure: structure,
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

    pub fn sync_all(&mut self) -> bool {
        self.shadow_file.sync_all()
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

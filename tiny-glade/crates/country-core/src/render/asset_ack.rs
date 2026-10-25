//! 资产操作的分阶段确认（PARALLEL_REFACTOR.md 任务 5）。
//!
//! 资产库（SSBO/网格/VAO/纹理等）在游戏侧发布单调的版本号（发布序号）；渲染
//! 线程按版本增量应用到自己的副本。源日志版本用于去重，实际交接发布单调操作
//! 序号，资源移除/重插也发布新序号；同一未变更快照不重复应用。本资源把进度
//! 分阶段确认回游戏：
//!
//! * `applied`：渲染线程已把该版本应用到渲染副本（CPU 应用）；
//! * `submitted`：引用该版本的帧已成功提交 GPU（上传随帧提交）；
//! * `gpu_complete`：该帧的 GPU 工作已完成。
//!
//! 编译/提交失败的帧只确认到 `applied`，不推进后两级；渲染副本不因失败而
//! 回退已应用的版本，因此重试不会重放已接受的上传或创建。旧 App 的确认由
//! 来源 WorldId 过滤，不会写入新 World。
use std::any::TypeId;
use std::collections::HashMap;

use bevy_ecs::resource::Resource;

/// Library-local identities include lookup aliases and descriptor publication,
/// so removal of an alias never reports removal of its underlying asset.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AssetEntryKey {
    Id(u64),
    Name(String),
    Path(std::path::PathBuf),
    TextureName(u64),
    Preset(u8),
    Descriptor(u64),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssetOperationKind { Upsert, Remove }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetOperation {
    pub library: TypeId,
    pub entry: AssetEntryKey,
    /// Snapshot publication sequence: writes coalesced before extraction use
    /// the same checkpoint, and are applied once with their final owner/value.
    pub revision: u64,
    pub kind: AssetOperationKind,
}
#[derive(Clone, Copy, Debug)]
pub struct AssetChangeCursor(pub(crate) crate::render::extract::ChangeCursor);
impl AssetChangeCursor {
    pub fn revision(self) -> u64 { self.0.revision }
}
pub trait AssetSnapshot: Resource + Clone {
    fn asset_cursor(&self) -> AssetChangeCursor;
    fn asset_changes(&self, cursor: Option<AssetChangeCursor>) -> Option<(AssetChangeCursor, Vec<AssetEntryKey>)>;
    fn asset_keys(&self) -> Vec<AssetEntryKey>;
    fn asset_contains(&self, key: &AssetEntryKey) -> bool;
    fn update_asset_snapshot(&self, destination: &mut Self);
    fn asset_revision(&self) -> u64 { self.asset_cursor().revision() }
}
/// Reserve a publication before capturing the source checkpoint, including
/// resource removal/reinsertion which need not mutate a producer's journal.
pub fn next_asset_publication() -> u64 { crate::render::extract::next_publication() }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetEntryAck {
    pub stages: AssetStageAck,
    pub kind: AssetOperationKind,
}

/// Only dirty, not-yet-submitted entries remain here. A failed frame retains
/// them; a successful frame moves them into feedback and receipt retirement.
#[derive(Resource, Default)]
pub struct AppliedAssetOperations(HashMap<(TypeId, AssetEntryKey), AssetOperation>);
impl AppliedAssetOperations {
    pub fn record(&mut self, operation: AssetOperation) {
        let key = (operation.library, operation.entry.clone());
        if self.0.get(&key).is_none_or(|old| old.revision <= operation.revision) {
            self.0.insert(key, operation);
        }
    }
    pub fn feedback(&mut self, submitted: bool) -> Vec<AssetOperation> {
        if submitted { self.0.drain().map(|(_, operation)| operation).collect() }
        else { self.0.values().cloned().collect() }
    }
    pub fn remove_library<T: 'static>(&mut self) { self.0.retain(|(ty, _), _| *ty != TypeId::of::<T>()); }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AssetStageAck {
    pub applied: u64,
    pub submitted: u64,
    pub gpu_complete: u64,
}

#[derive(Resource, Default, Debug)]
pub struct RenderAssetAcks {
    stages: HashMap<TypeId, AssetStageAck>,
    /// 已提交帧 → 该帧引用的 (类型, 版本)。GPU 完成后转为 gpu_complete。
    in_flight: Vec<(u64, Vec<(TypeId, u64)>)>,
    gpu_completed_frame: u64,
    entries: HashMap<(TypeId, AssetEntryKey), AssetEntryAck>,
    entry_flights: Vec<(u64, Vec<AssetOperation>)>,
}

impl RenderAssetAcks {
    pub fn entry<T: 'static>(&self, key: &AssetEntryKey) -> Option<AssetEntryAck> {
        self.entries.get(&(TypeId::of::<T>(), key.clone())).copied()
    }
    /// Completed removals retire their per-entry bookkeeping. The library's
    /// completed checkpoint still proves consumption of those older operations.
    pub fn entry_gpu_complete<T: 'static>(&self, key: &AssetEntryKey, revision: u64) -> bool {
        self.entry::<T>(key).is_some_and(|ack| ack.stages.gpu_complete >= revision)
            || self.stage::<T>().gpu_complete >= revision
    }
    pub fn acknowledge_entries(&mut self, operations: Vec<AssetOperation>, submitted_frame: Option<u64>) {
        let mut pending = Vec::new();
        let mut retired = Vec::new();
        for operation in operations {
            let completed = self.stages.get(&operation.library).map_or(0, |stage| stage.gpu_complete);
            let ack = match self.entries.entry((operation.library, operation.entry.clone())) {
                std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
                std::collections::hash_map::Entry::Vacant(_) if completed >= operation.revision => continue,
                std::collections::hash_map::Entry::Vacant(entry) => entry.insert(AssetEntryAck {
                    stages: AssetStageAck::default(), kind: operation.kind,
                }),
            };
            if operation.revision >= ack.stages.applied { ack.kind = operation.kind; }
            ack.stages.applied = ack.stages.applied.max(operation.revision);
            if let Some(frame) = submitted_frame {
                let advanced = operation.revision > ack.stages.submitted;
                ack.stages.submitted = ack.stages.submitted.max(operation.revision);
                if frame <= self.gpu_completed_frame {
                    ack.stages.gpu_complete = ack.stages.gpu_complete.max(operation.revision);
                    if ack.kind == AssetOperationKind::Remove && ack.stages.gpu_complete >= ack.stages.applied {
                        retired.push((operation.library, operation.entry.clone()));
                    }
                } else if advanced { pending.push(operation); }
            }
        }
        for key in retired {
            if self.entries.get(&key).is_some_and(|ack| ack.kind == AssetOperationKind::Remove
                && ack.stages.gpu_complete >= ack.stages.applied) { self.entries.remove(&key); }
        }
        if let Some(frame) = submitted_frame && !pending.is_empty() { self.entry_flights.push((frame, pending)); }
    }
    pub fn stage<T: 'static>(&self) -> AssetStageAck {
        self.stages.get(&TypeId::of::<T>()).copied().unwrap_or_default()
    }

    /// 某版本是否已被 GPU 消费完成（可以安全回收只为它保留的数据）。
    pub fn gpu_complete<T: 'static>(&self, revision: u64) -> bool {
        self.stage::<T>().gpu_complete >= revision
    }

    pub fn gpu_completed_frame(&self) -> u64 {
        self.gpu_completed_frame
    }

    /// 渲染侧本帧应用的版本。`submitted_frame` 为 Some 时同时确认提交。
    pub fn acknowledge(&mut self, applied: &[(TypeId, u64)], submitted_frame: Option<u64>) {
        let mut newly_submitted = Vec::new();
        for &(ty, revision) in applied {
            let stage = self.stages.entry(ty).or_default();
            stage.applied = stage.applied.max(revision);
            if submitted_frame.is_some() {
                if submitted_frame.is_some_and(|frame| frame <= self.gpu_completed_frame) {
                    stage.gpu_complete = stage.gpu_complete.max(revision);
                } else if revision > stage.submitted { newly_submitted.push((ty, revision)); }
                stage.submitted = stage.submitted.max(revision);
            }
        }
        if let Some(frame) = submitted_frame && !newly_submitted.is_empty() {
            self.in_flight.push((frame, newly_submitted));
        }
    }

    /// 帧 `frame` 及更早帧的 GPU 工作已完成。
    pub fn gpu_completed(&mut self, frame: u64) {
        if frame <= self.gpu_completed_frame { return; }
        self.gpu_completed_frame = frame;
        let mut retired = Vec::new();
        self.entry_flights.retain(|(at, operations)| {
            if *at > frame { return true; }
            for operation in operations {
                if let Some(ack) = self.entries.get_mut(&(operation.library, operation.entry.clone())) {
                    ack.stages.gpu_complete = ack.stages.gpu_complete.max(operation.revision);
                    if ack.kind == AssetOperationKind::Remove && ack.stages.gpu_complete >= ack.stages.applied {
                        retired.push((operation.library, operation.entry.clone()));
                    }
                }
            }
            false
        });
        for key in retired {
            if self.entries.get(&key).is_some_and(|ack| ack.kind == AssetOperationKind::Remove
                && ack.stages.gpu_complete >= ack.stages.applied) { self.entries.remove(&key); }
        }
        let (done, pending): (Vec<_>, Vec<_>) = std::mem::take(&mut self.in_flight)
            .into_iter().partition(|(at, _)| *at <= frame);
        self.in_flight = pending;
        for (_, revisions) in done {
            for (ty, revision) in revisions {
                let stage = self.stages.entry(ty).or_default();
                stage.gpu_complete = stage.gpu_complete.max(revision);
            }
        }
    }
}

/// 渲染 World 内当前已应用的版本。保留到替换/删除，失败帧不会消费掉确认资格。
#[derive(Resource, Default, Debug)]
pub struct AppliedAssetRevisions(pub Vec<(TypeId, u64)>);

impl AppliedAssetRevisions {
    pub fn record<T: 'static>(&mut self, revision: u64) {
        let ty = TypeId::of::<T>();
        if let Some(entry) = self.0.iter_mut().find(|entry| entry.0 == ty) {
            entry.1 = revision;
        } else { self.0.push((ty, revision)); }
    }
    pub fn remove<T: 'static>(&mut self) { self.0.retain(|entry| entry.0 != TypeId::of::<T>()); }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Lib;

    #[test]
    fn stages_advance_independently_and_failed_frames_stop_at_applied() {
        let ty = TypeId::of::<Lib>();
        let mut acks = RenderAssetAcks::default();
        acks.acknowledge(&[(ty, 3)], None);
        assert_eq!(acks.stage::<Lib>(), AssetStageAck { applied: 3, submitted: 0, gpu_complete: 0 });
        acks.acknowledge(&[(ty, 4)], Some(10));
        acks.acknowledge(&[(ty, 5)], Some(11));
        assert_eq!(acks.stage::<Lib>().submitted, 5);
        acks.gpu_completed(10);
        assert_eq!(acks.stage::<Lib>().gpu_complete, 4);
        assert!(acks.gpu_complete::<Lib>(4) && !acks.gpu_complete::<Lib>(5));
        // Stale or repeated completion is idempotent.
        acks.gpu_completed(9);
        acks.gpu_completed(11);
        acks.gpu_completed(11);
        assert_eq!(acks.stage::<Lib>(), AssetStageAck { applied: 5, submitted: 5, gpu_complete: 5 });
    }

    #[test]
    fn entry_retry_duplicate_reordering_and_removal_retirement_keep_the_latest_operation() {
        let ty = TypeId::of::<Lib>();
        let key = AssetEntryKey::Id(9);
        let op = |revision, kind| AssetOperation { library: ty, entry: key.clone(), revision, kind };
        let mut pending = AppliedAssetOperations::default();
        pending.record(op(3, AssetOperationKind::Upsert));
        pending.record(op(2, AssetOperationKind::Remove));
        let mut acks = RenderAssetAcks::default();
        acks.acknowledge(&[(ty, 3)], None);
        acks.acknowledge_entries(pending.feedback(false), None);
        assert_eq!(acks.entry::<Lib>(&key).unwrap().stages,
            AssetStageAck { applied: 3, submitted: 0, gpu_complete: 0 });
        acks.acknowledge(&[(ty, 3)], Some(10));
        acks.acknowledge_entries(pending.feedback(true), Some(10));
        assert!(pending.feedback(false).is_empty());
        acks.acknowledge_entries(vec![op(3, AssetOperationKind::Upsert), op(2, AssetOperationKind::Remove)], Some(11));
        acks.gpu_completed(10);
        assert!(acks.entry_gpu_complete::<Lib>(&key, 3));
        assert_eq!(acks.entry::<Lib>(&key).unwrap().kind, AssetOperationKind::Upsert);
        acks.acknowledge(&[(ty, 4)], Some(12));
        acks.acknowledge_entries(vec![op(4, AssetOperationKind::Remove)], Some(12));
        assert!(!acks.entry_gpu_complete::<Lib>(&key, 4));
        acks.gpu_completed(12);
        assert!(acks.entry::<Lib>(&key).is_none(), "completed tombstones must not grow with transient assets");
        assert!(acks.entry_gpu_complete::<Lib>(&key, 4));
        acks.acknowledge_entries(vec![op(3, AssetOperationKind::Upsert)], Some(10));
        assert!(acks.entry::<Lib>(&key).is_none(), "old ACK must not recreate retired entry state");
        acks.acknowledge(&[(ty, 5)], Some(13));
        acks.acknowledge_entries(vec![op(5, AssetOperationKind::Upsert)], Some(13));
        acks.gpu_completed(12);
        assert!(!acks.entry_gpu_complete::<Lib>(&key, 5));
        acks.gpu_completed(13);
        assert!(acks.entry_gpu_complete::<Lib>(&key, 5));
    }
}

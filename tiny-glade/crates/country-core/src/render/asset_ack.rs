//! 资产操作的分阶段确认（PARALLEL_REFACTOR.md 任务 5）。
//!
//! 资产库（SSBO/网格/VAO/纹理等）在游戏侧发布单调的版本号（发布序号）；渲染
//! 线程按版本增量应用到自己的副本。版本号就是操作序号：同一来源 World 中只增
//! 不减，渲染侧对已应用的版本不重复应用（幂等）。本资源把每个库版本的进度
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
}

impl RenderAssetAcks {
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
        for &(ty, revision) in applied {
            let stage = self.stages.entry(ty).or_default();
            stage.applied = stage.applied.max(revision);
            if submitted_frame.is_some() {
                stage.submitted = stage.submitted.max(revision);
            }
        }
        if let Some(frame) = submitted_frame && !applied.is_empty() {
            self.in_flight.push((frame, applied.to_vec()));
        }
    }

    /// 帧 `frame` 及更早帧的 GPU 工作已完成。
    pub fn gpu_completed(&mut self, frame: u64) {
        if frame <= self.gpu_completed_frame { return; }
        self.gpu_completed_frame = frame;
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

/// 渲染 World 内：本次交接中被应用的资产版本（每帧由提取重建）。
#[derive(Resource, Default, Debug)]
pub struct AppliedAssetRevisions(pub Vec<(TypeId, u64)>);

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
}

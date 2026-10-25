//! 输入录制流和拥有完整状态的历史同步点。
//!
//! 队列复用标准 VecDeque。快照不保存裸地址，帧索引始终表示下一帧的位置。
use crate::{
    resources::toolbar::Toolbar,
    systems::{
        camera_rig::main_camera::{CameraRigParameters, MainCamera},
        mode_manager::AppMode,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const FRAME_PRUNE_THRESHOLD: usize = 9_999;
pub const MAX_SYNC_POINTS: usize = 9;
/// 均匀分段，避免触及上限时把全部近期输入一次丢空。
pub const SYNC_POINT_INTERVAL_FRAMES: usize = FRAME_PRUNE_THRESHOLD / MAX_SYNC_POINTS;

/// 最近输入用于自动复现；完整录制不自动裁掉用户选择的帧。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordingRetention {
    Recent,
    All,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputSnapshot {
    pub toolbar: Toolbar,
    pub app_mode: AppMode,
    pub camera: CameraRigParameters,
}
impl InputSnapshot {
    pub fn capture(toolbar: &Toolbar, app_mode: AppMode, camera: &MainCamera) -> Self {
        Self {
            toolbar: toolbar.clone(),
            app_mode,
            camera: camera.store_parameters(),
        }
    }
    /// 校验成功后整体恢复实际业务资源，失败不会留下部分恢复的状态。
    pub fn restore(
        &self,
        toolbar: &mut Toolbar,
        app_mode: &mut AppMode,
        camera: &mut MainCamera,
    ) -> anyhow::Result<()> {
        self.validate()?;
        toolbar.clone_from(&self.toolbar);
        *app_mode = self.app_mode;
        camera.restore_input_parameters(&self.camera);
        Ok(())
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            (self.toolbar.current as usize) < self.toolbar.categories.len(),
            "工具栏选中类别越界"
        );
        for category in &self.toolbar.categories {
            anyhow::ensure!(
                category.current < category.modes.len()
                    || (category.modes.is_empty() && category.current == 0),
                "工具栏类别的模式索引越界"
            );
        }
        let camera = &self.camera;
        anyhow::ensure!(
            camera.position.is_finite()
                && [
                    camera.yaw,
                    camera.pitch,
                    camera.vertical_fov_degrees,
                    camera.camera_rig_arm_length,
                    camera.pitch_offset,
                    camera.yaw_offset
                ]
                .iter()
                .all(|value| value.is_finite()),
            "相机快照包含非有限值"
        );
        anyhow::ensure!(
            camera.vertical_fov_degrees > 0.0 && camera.vertical_fov_degrees < 180.0,
            "相机视场角必须位于 0 到 180 度之间"
        );
        // store_parameters 尚不提供这些偏移；不接受加载后会被静默丢弃的值。
        anyhow::ensure!(
            camera.camera_rig_arm_length == 0.0
                && camera.pitch_offset == 0.0
                && camera.yaw_offset == 0.0,
            "相机输入快照不支持额外轨道偏移"
        );
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputStreamSyncPoint {
    pub state: InputSnapshot,
    pub frame_index: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputStream<F> {
    sync_points: VecDeque<InputStreamSyncPoint>,
    frames: VecDeque<F>,
}
impl<F> Default for InputStream<F> {
    fn default() -> Self {
        Self {
            sync_points: VecDeque::new(),
            frames: VecDeque::new(),
        }
    }
}
impl<F> InputStream<F> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
    pub fn sync_point_count(&self) -> usize {
        self.sync_points.len()
    }
    pub fn frames(&self) -> impl Iterator<Item = &F> {
        self.frames.iter()
    }
    pub fn sync_points(&self) -> &VecDeque<InputStreamSyncPoint> {
        &self.sync_points
    }
    pub fn get_frame(&self, index: usize) -> Option<&F> {
        self.frames.get(index)
    }
    /// 已检查索引的内部调用可使用此接口；外部回放入口使用 get_frame。
    pub fn frame(&self, index: usize) -> &F {
        &self.frames[index]
    }
    pub fn last_frame_mut(&mut self) -> Option<&mut F> {
        self.frames.back_mut()
    }
    pub fn clear(&mut self) {
        self.frames.clear();
        self.sync_points.clear();
    }
    pub fn reset(&mut self, snapshot: InputSnapshot) {
        self.clear();
        self.push_sync_point(snapshot);
    }
    /// 同一帧多次打点只保留最新状态，避免无效的零长度分段。
    pub fn push_sync_point(&mut self, snapshot: InputSnapshot) {
        let frame_index = self.frames.len();
        if self
            .sync_points
            .back()
            .is_some_and(|point| point.frame_index == frame_index)
        {
            self.sync_points.back_mut().unwrap().state = snapshot;
        } else {
            self.sync_points.push_back(InputStreamSyncPoint {
                state: snapshot,
                frame_index,
            });
        }
    }
    /// 快照只在首帧和分段边界创建；普通帧没有工具栏克隆和动作 Vec 分配。
    pub fn record_frame(
        &mut self,
        frame: F,
        snapshot: impl FnOnce() -> InputSnapshot,
        retention: RecordingRetention,
    ) {
        let needs_checkpoint = self.sync_points.is_empty()
            || (retention == RecordingRetention::Recent
                && (self.frames.len() >= FRAME_PRUNE_THRESHOLD
                    || self.sync_points.back().is_some_and(|point| {
                        self.frames.len().saturating_sub(point.frame_index)
                            >= SYNC_POINT_INTERVAL_FRAMES
                    })));
        if needs_checkpoint {
            self.push_sync_point(snapshot());
        }
        if retention == RecordingRetention::Recent {
            while self.sync_points.len() > MAX_SYNC_POINTS
                || (self.frames.len() >= FRAME_PRUNE_THRESHOLD && self.sync_points.len() >= 2)
            {
                self.drop_oldest_sync_point();
            }
        }
        self.frames.push_back(frame);
    }
    /// 丢掉旧分段，新头的快照与第零帧重新对齐。
    pub fn drop_oldest_sync_point(&mut self) {
        self.sync_points.pop_front();
        let discarded_frames = self
            .sync_points
            .front()
            .map_or(self.frames.len(), |point| point.frame_index)
            .min(self.frames.len());
        self.frames.drain(..discarded_frames);
        for point in &mut self.sync_points {
            point.frame_index = point.frame_index.saturating_sub(discarded_frames);
        }
    }
    pub fn validate_structure(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.frames.is_empty(), "录制没有输入帧");
        anyhow::ensure!(
            self.sync_points
                .front()
                .is_some_and(|point| point.frame_index == 0),
            "录制必须从第零帧的同步点开始"
        );
        let mut previous_index = 0;
        for point in &self.sync_points {
            anyhow::ensure!(
                point.frame_index >= previous_index && point.frame_index <= self.frames.len(),
                "同步点索引无序或越界"
            );
            point.state.validate()?;
            previous_index = point.frame_index;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn snapshot() -> InputSnapshot {
        InputSnapshot::capture(
            &Toolbar::default(),
            AppMode::default(),
            &MainCamera::new(1.0),
        )
    }
    #[test]
    fn recent_recording_remains_bounded_and_aligned_without_history_events() {
        let mut stream = InputStream::new();
        for frame in 0..FRAME_PRUNE_THRESHOLD * 3 + 3 {
            stream.record_frame(frame, snapshot, RecordingRetention::Recent);
        }
        assert!(stream.frame_count() <= FRAME_PRUNE_THRESHOLD);
        assert!(stream.frame_count() >= FRAME_PRUNE_THRESHOLD - SYNC_POINT_INTERVAL_FRAMES);
        assert_eq!(
            stream.get_frame(0),
            Some(&(FRAME_PRUNE_THRESHOLD * 3 + 3 - stream.frame_count()))
        );
        assert_eq!(stream.sync_points()[0].frame_index, 0);
        stream.validate_structure().unwrap();
    }
    #[test]
    fn all_recording_keeps_frames_and_creates_only_one_initial_snapshot() {
        let mut captures = 0;
        let mut stream = InputStream::new();
        for frame in 0..FRAME_PRUNE_THRESHOLD + 5 {
            stream.record_frame(
                frame,
                || {
                    captures += 1;
                    snapshot()
                },
                RecordingRetention::All,
            );
        }
        assert_eq!(captures, 1);
        assert_eq!(stream.frame_count(), FRAME_PRUNE_THRESHOLD + 5);
    }
    #[test]
    fn dropping_rebases_to_the_surviving_checkpoint() {
        let mut stream = InputStream::new();
        for frame in 0..10 {
            stream.record_frame(frame, snapshot, RecordingRetention::All);
        }
        stream.push_sync_point(snapshot());
        for frame in 10..15 {
            stream.record_frame(frame, snapshot, RecordingRetention::All);
        }
        stream.drop_oldest_sync_point();
        assert_eq!(stream.get_frame(0), Some(&10));
        assert_eq!(stream.sync_points()[0].frame_index, 0);
    }
}

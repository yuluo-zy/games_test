//! hint_camera_progress（O：CameraHintProgress 36B = 进度 pan/rotate/zoom +
//! 上一帧快照 + 阈值 pan1.4/rotate1.2/zoom120（启动 0.7/0.7/60，reset 恢复 2×）。
//!
//! `hint_camera_progress_update` 0x14227bd30（782）：先快照；读相机事件（tag 2→rotate
//! += dt/阈值，tag 1→zoom += 事件值/阈值，其它→pan += dt/阈值）；跨 1.0 边沿 →
//! 写 TutorialHintComplete（tag 0xa/0xb/0xc）。
//! `hint_camera_progress_reset` 0x14227c070（436）：当前提示 id 变化时归零并恢复阈值。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

use crate::tutorial_completion::TutorialHintComplete;

/// 相机进度（O：CameraHintProgress，9×f32：pan/rotate/zoom 进度@+0/+4/+8，
/// 上一帧快照@+0xc/+0x10/+0x14，阈值@+0x18/+0x1c/+0x20）。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct CameraHintProgress {
    pub pan: f32,
    pub rotate: f32,
    pub zoom: f32,
    pub prev_pan: f32,
    pub prev_rotate: f32,
    pub prev_zoom: f32,
    pub pan_threshold: f32,
    pub rotate_threshold: f32,
    pub zoom_threshold: f32,
}

impl Default for CameraHintProgress {
    fn default() -> Self {
        Self {
            pan: 0.0,
            rotate: 0.0,
            zoom: 0.0,
            prev_pan: 0.0,
            prev_rotate: 0.0,
            prev_zoom: 0.0,
            pan_threshold: 0.7,
            rotate_threshold: 0.7,
            zoom_threshold: 60.0,
        }
    }
}

impl CameraHintProgress {
    /// 启动阈值（O hints_startup：0.7/0.7/60 = 0x3f333333/0x3f333333/0x42700000）。
    pub fn with_startup_thresholds(pan: bool, rotate: bool, zoom: bool) -> Self {
        let mut s = Self::default();
        s.pan = if pan { 1.0 } else { 0.0 };
        s.rotate = if rotate { 1.0 } else { 0.0 };
        s.zoom = if zoom { 1.0 } else { 0.0 };
        s.prev_pan = s.pan;
        s.prev_rotate = s.rotate;
        s.prev_zoom = s.zoom;
        s
    }
    pub fn progress_pan(&self) -> f32 {
        self.pan
    }
    pub fn progress_rotate(&self) -> f32 {
        self.rotate
    }
    pub fn progress_zoom(&self) -> f32 {
        self.zoom
    }
}

/// 上一帧提示 id 记录（Local<DisplayedHintDelta>）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DisplayedHintDelta(pub u8);

/// hint_camera_progress_reset（O 0x14227c070）：读当前提示 id（cVar5，无实体记 19）；
/// 与上一帧（DisplayedHintDelta）不同 → 归零并恢复阈值：
/// 0xa(pan)→pan=0,pan_threshold=1.4(0x3fb33333)；0xb(rotate)→rotate=0,
/// rotate_threshold=1.2(0x3f99999a)；0xc(zoom)→zoom=0,zoom_threshold=120(0x42f00000)。
/// 最后写回 DisplayedHintDelta。
pub fn hint_camera_progress_reset(
    mut progress: ResMut<CameraHintProgress>,
    query: Query<&crate::hint_orchestrator::HintType>,
    mut prev: Local<DisplayedHintDelta>,
) {
    // O：稀疏集游标取当前提示实体 HintType 值；无实体 → 19。
    let cvar5: u8 = query.iter().next().map_or(0x13, |t| *t as u8);
    if prev.0 != cvar5 {
        match cvar5 {
            0xc => {
                progress.zoom = 0.0;
                progress.zoom_threshold = 120.0;
            }
            0xb => {
                progress.rotate = 0.0;
                progress.rotate_threshold = 1.2;
            }
            0xa => {
                progress.pan = 0.0;
                progress.pan_threshold = 1.4;
            }
            _ => {}
        }
    }
    prev.0 = cvar5;
}
/// hint_camera_progress_update（O 0x14227bd30）：快照 → 事件累积（tag 2→rotate
/// += dt/threshold、tag 1→zoom += value/threshold、其它→pan += dt/threshold）→
/// 跨 1.0 边沿写 TutorialHintComplete（tag 0xa/0xb/0xc）。
/// **判等（2026-10-17）**：事件读面已由 seam 切换为 `country_core::
/// resources::onboarding::NotifyHintSystemCameraMoved`（{tag u32, value u32}）。
pub fn hint_camera_progress_update(
    time: Res<GameTime>,
    mut progress: ResMut<CameraHintProgress>,
    mut camera_events: EventReader<
        country_core::resources::onboarding::NotifyHintSystemCameraMoved,
    >,
    mut completed: EventWriter<TutorialHintComplete>,
) {
    let dt = time.delta;
    // 快照上一帧。
    progress.prev_pan = progress.pan;
    progress.prev_rotate = progress.rotate;
    progress.prev_zoom = progress.zoom;
    for event in camera_events.read() {
        match event.tag {
            2 => progress.rotate += dt / progress.rotate_threshold,
            1 => progress.zoom += event.value as f32 / progress.zoom_threshold,
            _ => progress.pan += dt / progress.pan_threshold,
        }
    }
    // 跨 1.0 边沿（上一帧 <1 且本帧 >=1）→ 完成事件。
    if progress.prev_pan <= 1.0 && progress.prev_pan != 1.0 && 1.0 <= progress.pan {
        completed.write(TutorialHintComplete { id: 0xa });
    }
    if progress.prev_rotate <= 1.0 && progress.prev_rotate != 1.0 && 1.0 <= progress.rotate {
        completed.write(TutorialHintComplete { id: 0xb });
    }
    if progress.prev_zoom <= 1.0 && progress.prev_zoom != 1.0 && 1.0 <= progress.zoom {
        completed.write(TutorialHintComplete { id: 0xc });
    }
}

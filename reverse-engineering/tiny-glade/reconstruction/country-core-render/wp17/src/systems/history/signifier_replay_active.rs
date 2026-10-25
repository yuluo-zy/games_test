//! `systems::history::signifier_replay_active` — 回放激活指示（0x140b1b860，
//! WP17）。
//!
//! O（反编译 0x140b1b860 + enriched 接口 a3c33749…）：
//!   - 系统签名：`(Res<ReplayBuffer>, ResMut<ReplayModeElapsedTime>,
//!     Query<(&mut Transform, &SignifierReplayIndex)>,
//!     Res<MainCamera>, Res<GameTime>, Res<CursorTerrainRaycast>,
//!     Res<ReplayBuffer>)`。
//!   - `ReplayModeElapsedTime`（enum2$ 资源；判别==1 活跃；累加 GameTime）。
//!   - `ease_out_elastic_jiggle`：`0.5*exp2(-10t)*sin(7.853982t)`。
//!   - `register_required_components` 0x140b1d680：注册所需组件。
//!   - 游标处 signifier 弹性抖动；消耗 `ReplayBuffer::current_replay_cmd`。

use crate::resources::history::replay_cmd_buffer::ReplayBuffer;
use crate::typed_gap::Gap;

/// 回放模式已过时间（O：enum2$ 资源；判别==1 时活跃）。
#[derive(Clone, Debug, Default)]
pub struct ReplayModeElapsedTime {
    /// O：判别（1=active）。
    pub active: bool,
    /// O：已过时间（累加 GameTime delta；int 存储观察）。
    pub elapsed: f32,
}

impl ReplayModeElapsedTime {
    /// O：判别==1 检查（signifier_replay_active 首行）。
    pub fn is_active(&self) -> bool {
        self.active
    }
    /// O：帧累加（`+GameTime.delta`）。
    pub fn tick(&mut self, dt: f32) -> f32 {
        self.elapsed += dt;
        self.elapsed
    }
}

/// 回放激活 signifier 索引（O：组件；挂到 signifier entity）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SignifierReplayIndex(pub u32);

/// 光标位置边界（O：`CursorTerrainRaycast`；查 signifier 目标位置）。
#[derive(Clone, Debug, Default)]
pub struct CursorTerrainRaycastBoundary(pub Gap);

/// 相机边界（O：`MainCamera`；`CameraMatrices::eye_position`）。
#[derive(Clone, Debug, Default)]
pub struct MainCameraBoundary(pub Gap);

/// Transform 边界（O：`utils::components::transform::Transform` 40B：
/// {t@+0x0, r@+0xc, s@+0x1c}；look_at）。
#[derive(Clone, Debug, Default)]
pub struct TransformBoundary(pub Gap);

impl TransformBoundary {
    pub fn look_at(&mut self, _target: (f32, f32, f32)) {}
}

/// O：`ease_out_elastic_jiggle`（源码行 75）：`0.5*exp2(-10t)*sin(7.853982t)`。
pub fn ease_out_elastic_jiggle(t: f32) -> f32 {
    if t < 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 0.0;
    }
    0.5 * (-10.0 * t).exp2() * (7.853982 * t).sin()
}

/// O：`signifier_replay_active` 0x140b1b860。骨架实现：活跃时累加计时，对每个
/// `SignifierReplayIndex` entity 把 Transform 向光标目标做弹性抖动插值。
pub fn signifier_replay_active(
    replay_elapsed: &mut ReplayModeElapsedTime,
    buffer: &ReplayBuffer,
    dt: f32,
    _signifiers: &mut [(SignifierReplayIndex, TransformBoundary)],
    _cursor: &CursorTerrainRaycastBoundary,
    _camera: &MainCameraBoundary,
) {
    if !replay_elapsed.is_active() {
        return;
    }
    let _ = replay_elapsed.tick(dt);
    // O：进度 `frac(elapsed / clamp(0.9/dt, 0.5, ..))`。
    let scale = (0.9 / dt.max(1e-6)).clamp(0.5, f32::MAX);
    let t = replay_elapsed.elapsed / scale;
    let _t_frac = t - t.trunc();
    // O：ReplayBuffer 状态驱动（current_replay_cmd / is_empty）。
    let _active = !buffer.is_empty();
    for (_idx, _transform) in _signifiers.iter_mut() {
        // O：从光标射线表取目标位置，`Transform::look_at` + 弹性偏移（U 数学）。
        // ease_out_elastic_jiggle 用于抖动幅度。
        let _jiggle = ease_out_elastic_jiggle(_t_frac);
    }
}

/// O：`impl$0::register_required_components` 0x140b1d680。
pub fn register_required_components() {}

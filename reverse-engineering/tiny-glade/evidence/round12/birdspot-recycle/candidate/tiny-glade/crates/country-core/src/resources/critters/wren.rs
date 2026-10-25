//! Known original Wren state belongs to its canonical core component identity.
//! Audio remains a separate host component. This is a semantic projection,
//! not a claim that the native 80-byte record has this Rust layout.
use bevy_ecs::component::Component;
use glam::Vec3;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Wren {
    pub pos: Vec3,
    pub prev_pos: Vec3,
    pub phase: f32,
    pub hop_timer: f32,
    pub spot: WrenSpotState,
}
impl Wren {
    /// All three original sim_wren_boids spawn tuples write state 2 at +0x30.
    pub fn new_airborne(pos: Vec3) -> Self {
        Self { pos, prev_pos: pos, phase: 0.0, hop_timer: 0.0,
            spot: WrenSpotState { state: 2, ..Default::default() } }
    }
}
impl Default for Wren {
    fn default() -> Self { Self::new_airborne(Vec3::ZERO) }
}

/// 鹪鹩 per-spot 状态机记录（二进制 spot 0x50 记录的语义投影；字段偏移见各注释）。
/// 状态跳表 DAT_14304b638（RE-ANALYSIS §1.1 解码，index=`*(uint*)(spot+0x30)`）：
/// 0=计时/等待（timer 倒计时；到点或距目标<1.0 → state1）、1=音频过渡（→2）、
/// 2/4=扫描续（无 handler）、3=smoothstep 移动（→重置后高度更新）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WrenSpotState {
    /// +0x30：状态字节（0..4）。
    pub state: u32,
    /// +0x34：计时（state0 倒计时；state3 复用作 dir.x 方向分量）。
    pub timer: f32,
    /// +0x38：state3 方向分量 y。
    pub dir_y: f32,
    /// +0x3c：state3 方向分量 z。
    pub dir_z: f32,
    /// +0x40：smoothstep 进度计时（state3 `+= dt`，>6.0 → 重置）。
    pub progress: f32,
    /// +0x44：动画流经值（state0/1 经 XMM15 原样回写；state3 = 新 progress）。
    pub anim_val: f32,
    /// +0x48：高度（`clamp01(height + dt*(state==0 ? -50 : 5))`，表 [0x14304b3b0]=(-50,5)）。
    pub height: f32,
}


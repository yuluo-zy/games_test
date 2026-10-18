use bevy_ecs::component::Component;

/// `Butterfly`（stride 0x18；state@+0、handler 参数/计时@+4、目标计时@+8（goals `timer@+2` 段）、
/// dwell@+0xc、phase@+0x10、phase_ref@+0x14；phase_ref 是 GPU `anim_t`）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Butterfly {
    pub state: u32,
    /// +4：状态跳表 handler 的 `t`（state0/2 作倒计时；state3 写 height 或
    /// `uniform*5+5`，见 `update_butterfly_states` fresh 反汇编）。原字段名 U。
    pub state_t: f32,
    /// +8：目标计时（update_butterfly_goals 读写）。
    pub goal_timer: f32,
    /// +0xc：状态机 dwell 计时。
    pub dwell: f32,
    /// +0x10：相位。
    pub phase: f32,
    /// +0x14：相位参考（= GPU anim_t）。
    pub phase_ref: f32,
}


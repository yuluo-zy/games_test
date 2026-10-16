//! B `crates/systems/decorator/src/window_lights.rs` — 窗灯系统（D4 restorer tg_d4）。
//!
//! 证据（fresh RE-MCP，decomp-1407b5980.txt，O 结构）：
//! - `window_lights` 0x1407b5980：遍历屋顶/墙（PublicWalls 双模式：+0xf8 id
//!   数组 + 模式字节 +0x108；roof 0x48 段 / wall 0xa0 段）收集窗实体 → 维护
//!   `GlobalWindowLightsControl` 的实体表（0x18B 条目：{key@-0x18, f32@-0xc,
//!   type u8@-8, flag u8@-7}）→ 帧末按 `Time` delta 推进每条目计时
//!   （`f32 -= delta`，归零拷贝 flag 字节 = 灯开关）→ 旧条目剔除
//!   （FUN_141268bd0 = 表裁剪，O）。
//! - 开灯判定（O 常量）：`t ∈ [0.44 | 0.5097, 0.7199)` —— 昼夜循环值
//!   （`*param_3 + 8` 的 f32，0x142a83cb8=0.44 / 0x142a83cbc=0.5097 按
//!   `ActiveSession::is_olden`（0x140a9fa70）选界，上限 0x142a80d44=0.7199）；
//!   结果写入控制字节 +0x30（`lights_on`）。
//! - 逐条目插值 = `time_continuous_lerp`（0x140c904e0，O 结构：
//!   `lerp(a, b, 1 - F((bits ^ 0x8000_0000) as f32 * rate))`；F =
//!   0x142923ec0 std 导入 thunk，身份 U——RESTORE.md 登记）。
//!
//! core 缺口：`country_core::resources::window_lights::GlobalWindowLightsControl`
//! 仅胶水（章节 window_lights.rs.md）；本模块本地承载类型（RESTORE.md 登记）。

use bevy_ecs::prelude::*;
use bevy_ecs::resource::Resource;

use country_core::resources::session::ActiveSession;
use country_core::resources::time::GameTime;

/// 单窗灯条目（B 0x18B 记录：{key@-0x18, brightness f32@-0x10, timer f32@-0xc,
/// actual u8@-8, target u8@-7}）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowLightState {
    /// 亮度插值（f32@-0x10；time_continuous_lerp 位模式驱动）。
    pub brightness: f32,
    /// 亮灯计时（f32@-0xc；每帧 -= delta，归零把 actual ← target）。
    pub timer: f32,
    /// 实际开关字节（u8@-8；计时归零时拷入 target）。
    pub actual_on: u8,
    /// 目标开关字节（u8@-7；I：灯型语义待取证）。
    pub target_on: u8,
}

impl Default for WindowLightState {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            timer: 1.0,
            actual_on: 0,
            target_on: 0,
        }
    }
}

/// B `country_core::resources::window_lights::GlobalWindowLightsControl`
/// （core 未恢复类型 → 本地承载，登记 RESTORE.md）：窗灯全局控制。
#[derive(Resource, Clone, Debug, Default)]
pub struct GlobalWindowLightsControl {
    /// 昼夜循环值 t（B +8 区 f32；开灯判定区间）。
    pub cycle_t: f32,
    /// 灯表（entity → 状态；B 0x18B 条目）。
    pub lights: hashbrown::HashMap<u64, WindowLightState>,
    /// 开灯标志（B +0x30 字节；`t∈[lower, 0.7199)` 时置 1）。
    pub lights_on: bool,
}

/// 开灯区间下界：非 olden 0.5097（0x142a83cbc），olden 0.44（0x142a83cb8）。
const LOWER_BOUND_NORMAL: f32 = 0.5097;
const LOWER_BOUND_OLDEN: f32 = 0.44;
/// 开灯区间上界（0x142a80d44 = 0.7199）。
const UPPER_BOUND: f32 = 0.7199;

/// B utils crate `time_continuous_lerp`（0x140c904e0，O 结构）：
/// `lerp(a, b, 1 - F((bits ^ 0x8000_0000) as f32 * rate))` —— 位模式驱动的
/// 连续动画插值（窗灯亮度/烟柱厚度共用的 utils 缺口函数，登记 RESTORE.md）。
pub fn time_continuous_lerp(a: f32, b: f32, bits: u32, rate: f32) -> f32 {
    let v = ((bits ^ 0x8000_0000) as f32) * rate;
    // F = 0x142923ec0（std 导入 thunk，身份 U；以 sin 承载振荡语义并登记）。
    let f = v.sin();
    (b - a) * (1.0 - f) + a
}

/// `window_lights`（0x1407b5980，O 结构）：窗灯维护系统。
/// 1. 收集屋顶/墙窗实体；2. 表缺失条目补入；3. 帧末推进计时/开关；
/// 4. 剔除失效条目；5. 写全局 lights_on。
pub fn window_lights(
    mut control: ResMut<GlobalWindowLightsControl>,
    session: Res<ActiveSession>,
    time: Res<GameTime>,
    // 原参数面另含屋顶/墙遍历资源（PublicWalls 双模式；O 结构）+ 昼夜循环值
    // 资源（t @ +8）+ 实体收集资源（ResMut tick@+0x1c）—— 身份待核，
    // RESTORE.md 挂账登记。
) {
    let olden = session.is_olden();
    let lower = if olden {
        LOWER_BOUND_OLDEN
    } else {
        LOWER_BOUND_NORMAL
    };
    // 开灯判定（O：`lower <= t && t < 0.7199`；写控制字节 +0x30）。
    let t = control.cycle_t;
    control.lights_on = lower <= t && t < UPPER_BOUND;

    let delta = time.delta;
    // 帧末推进（O：timer -= delta，`<=0` 钳 0，归零时 `actual ← target` 拷贝；
    // brightness 由 time_continuous_lerp 位模式驱动刷新）。
    for state in control.lights.values_mut() {
        if state.timer > 0.0 {
            state.timer = (state.timer - delta).max(0.0);
            if state.timer == 0.0 {
                state.actual_on = state.target_on;
            }
        }
        // 亮度插值（O：time_continuous_lerp；I：位来源 = timer 位模式，
        // 登记 RESTORE.md）。
        let bits = state.timer.to_bits();
        state.brightness = time_continuous_lerp(0.0, 1.0, bits, 0.05);
    }
    // 剔除失效条目（O：FUN_141268bd0 表裁剪；I：失效判定 =
    // actual_on 归零且计时耗尽，由调用方注册的裁剪集承载）。
    control
        .lights
        .retain(|_entity, state| state.timer > 0.0 || state.actual_on != 0);
}

//! B `crates/systems/decorator/src/window_lights.rs` — 窗灯系统。
//!
//! 证据（2026-10-17 fresh Ghidra shared 反编译 0x1407b5980（32KB，
//! `reports/state/decorator/review-evidence/fresh-decomp/0x1407b5980_*.c`）+
//! read_memory 常量核验）：
//! - 参数面（O）：in_RCX = ResMut<GlobalWindowLightsControl>（tick 回写
//!   +0x1c→+0x10；控制 +0x30 写 lights_on）、in_RDX = ResMut<PublicWalls 复合>
//!   （+0xf8 墙 id 数组 / +0x100 数组末 / +0x108 模式字节 / +0x110 索引、
//!   +0x100/+0x1a8 双桶表）、in_R8 = Res<昼夜循环资源>（t 于 `*+8`）、in_R9 =
//!   Res<GameTime>（delta `*+0x28`）、栈 5 参 = ResMut<瓦缓冲复合>。
//! - 开灯判定（O）：`SessionLoadReadiness::deny_load()` 假→下界 0.51f
//!   （0x3f028f5c，read_memory 0x142a83cbc 核验；**旧 0.5097 是错 lane**）、
//!   真→0.44f（0x3ee147ae，0x142a83cb8）；上界 **0.72f**（0x3f3851ec，
//!   0x142a80d44 核验；**旧 0.7199 是错 lane**）。`lights_on = lower <= t &&
//!   t < 0.72`；写控制字节 +0x30。
//! - 灯表条目（O，0x18B）：`{key u64 @0, brightness f32 @8, timer f32 @0xc,
//!   actual u8 @0x10, target u8 @0x11}`（key 由 HashMap 承载）。
//! - 灯态变化（O，逐条目）：`entry.target != lights_on` →
//!   `timer = 2.0*fastrand::f32()` + `target = lights_on`（随机抖动）。
//! - 窗实体收集（O 结构）：墙（模式字节 +0x108==0）走 `+0xf8` id 数组 +
//!   0xa0 段桶，屋顶走 0x48 段桶；经 `+0x110` 索引反查实体记录（0x10 步长
//!   子记录），逐实体核对灯表：缺失 → 插 `{key, brightness=0,
//!   actual=lights_on, target=lights_on}`；命中 + 形状变化条件
//!   （`(A 新瓦||B 新瓦) && local_f4==plVar30[1]`）→ `brightness=1.0, target=1`。
//! - 帧末（O）：逐条目 `timer>0 → timer -= delta（钳 0），==0 且非 NaN →
//!   actual=target`；`brightness = utils::time_continuous_lerp(...)`（0x140c904e0）。
//! - 剔除（O）：`HashMap::retain<closure_env_1>` 裁剪失效条目（闭包 U）。
//!
//! U 登记：墙/屋顶遍历资源身份（PublicWalls 双模式密集表，core 未暴露——
//! `PublicWalls` 现为 HashMap 抽象，stride-0xa0/0x48 窗桶表内嵌
//! InnerWalls/RoofShape，walls 组在途域）+ t 资源（昼夜循环资源 `*+8` f32，
//! 本地以 [`GlobalWindowLightsControl::cycle_t`] 承载为近似）+ 收集资源
//! （ResMut tick@+0x1c）+ 栈 5 参瓦缓冲复合身份；形状变化条件
//! `local_f4 == plVar30[1]` 字段语义；retain 闭包谓词；
//! time_continuous_lerp 的 F thunk 身份与实参。

use bevy_ecs::prelude::*;
use bevy_ecs::resource::Resource;
use fastrand::Rng;

use country_core::resources::session::ActiveSession;
use country_core::resources::time::GameTime;

/// 单窗灯条目（B 0x18B 记录余下 16B：`{brightness f32 @8, timer f32 @0xc,
/// actual u8 @0x10, target u8 @0x11}`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowLightState {
    /// +0x08：亮度插值（time_continuous_lerp 输出）。
    pub brightness: f32,
    /// +0x0c：亮灯计时（灯态变化时 = 2*rng；帧末 -= delta，归零 → actual=target）。
    pub timer: f32,
    /// +0x10：实际开关字节（计时归零时拷入 target）。
    pub actual_on: u8,
    /// +0x11：目标开关字节（= lights_on）。
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
    /// 昼夜循环值 t（B +8 区 f32；开灯判定区间；外部系统写入）。
    pub cycle_t: f32,
    /// 灯表（entity → 状态；B 0x18B 条目）。
    pub lights: hashbrown::HashMap<u64, WindowLightState>,
    /// +0x30：开灯标志（`t∈[lower, 0.72)` 时置 1）。
    pub lights_on: bool,
}

/// 开灯区间下界：非 olden 0.51f（0x3f028f5c，read_memory 0x142a83cbc 核验；
/// **旧 0.5097 是错 lane**），olden 0.44f（0x3ee147ae，0x142a83cb8）。
/// O：`SessionLoadReadiness::deny_load()`（0x140a9fa70）与
/// `GladeType/ActiveSession::is_olden` 同址折叠 —— 以 `is_olden()` 承载等价。
const LOWER_BOUND_NORMAL: f32 = 0.51;
const LOWER_BOUND_OLDEN: f32 = 0.44;
/// 开灯区间上界（0x142a80d44 = 0.72f，read_memory 核验；**旧 0.7199 是错 lane**）。
const UPPER_BOUND: f32 = 0.72;

/// B utils crate `time_continuous_lerp`（0x140c904e0，O 结构）：
/// `lerp(a, b, 1 - F((bits ^ 0x8000_0000) as f32 * rate))` —— 位模式驱动的
/// 连续动画插值（窗灯亮度共用 utils 缺口函数，登记 RESTORE.md；
/// F = 0x142923ec0 std 导入 thunk 身份 U）。
pub fn time_continuous_lerp(a: f32, b: f32, bits: u32, rate: f32) -> f32 {
    let v = ((bits ^ 0x8000_0000) as f32) * rate;
    // F = 0x142923ec0（std 导入 thunk，身份 U；以 sin 承载振荡语义并登记）。
    let f = v.sin();
    (b - a) * (1.0 - f) + a
}

/// `window_lights`（0x1407b5980，O 结构 + 部分判等）。
///
/// 1. 开灯判定（deny_load 门控下界 0.51/0.44，上界 0.72）——判等；
/// 2. 灯态变化随机重计时（逐条目 target != lights_on → timer=2*rng）——判等；
/// 3. 窗实体收集核对（墙/屋顶双模式桶）——**未实现**（收集资源身份 U，见
///    文件头；当前以条目全量维护承载）；
/// 4. 帧末推进计时/开关 + 亮度插值——判等（time_continuous_lerp 实参 U）；
/// 5. 剔除失效（retain 谓词 U）。
pub fn window_lights(
    mut control: ResMut<GlobalWindowLightsControl>,
    session: Res<ActiveSession>,
    time: Res<GameTime>,
) {
    let olden = session.is_olden();
    let lower = if olden {
        LOWER_BOUND_OLDEN
    } else {
        LOWER_BOUND_NORMAL
    };
    // 开灯判定（O：`lower <= t && t < 0.72`；写控制字节 +0x30）。
    let t = control.cycle_t;
    let lights_on = lower <= t && t < UPPER_BOUND;
    control.lights_on = lights_on;

    // 灯态变化（O，逐条目）：target != lights_on → 随机重计时 + 更新 target。
    let rng = Rng::with_seed(rng_seed());
    for state in control.lights.values_mut() {
        if state.target_on != lights_on as u8 {
            state.timer = 2.0 * rng.f32();
            state.target_on = lights_on as u8;
        }
    }

    let delta = time.delta;
    // 剔除失效条目（O 结构，fresh LAB_1407b62ed：`HashMap::retain<closure_env_1>`
    // 在**帧末推进之前**执行——B 顺序 = 灯态变化 → 收集 → retain → 帧末；原
    // Rust 把 retain 放帧末后，与二进制次序分歧，2026-10-17 已调整。谓词 U，
    // 以 timer>0 或已亮保留承载，登记 RESTORE.md）。
    control
        .lights
        .retain(|_entity, state| state.timer > 0.0 || state.actual_on != 0);
    // 帧末推进（O：timer -= delta，`<=0` 钳 0，归零且非 NaN 时 `actual ← target`；
    // brightness = time_continuous_lerp，位来源 = timer 位模式，登记）。
    for state in control.lights.values_mut() {
        if state.timer > 0.0 {
            state.timer = (state.timer - delta).max(0.0);
            if state.timer == 0.0 {
                state.actual_on = state.target_on;
            }
        }
        let bits = state.timer.to_bits();
        state.brightness = time_continuous_lerp(0.0, 1.0, bits, 0.05);
    }
}

/// rng 种子（U：原 fastrand::init_rng 种子来源；以 t 位模式承载，登记）。
fn rng_seed() -> u64 {
    0
}

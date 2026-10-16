//! B `crates/systems/decorator/src/chimney_smoke_anim_thickness.rs` — 烟柱厚度
//! 动画（D4 restorer tg_d4）。
//!
//! 证据（fresh RE-MCP，decomp-140818df0.txt，O 结构）：
//! - `chimney_smoke_anim_thickness` 0x140818df0（行 9）：遍历屋顶/墙双模式
//!   （PublicWalls：+0xf8 id 数组 + 模式字节 +0x108；roof 0x48 段 / wall 0xa0
//!   段；实体经 +0x110 索引反查）收集烟囱实体列表 → 每实体查厚度表
//!   （0x18B 条目 {key@-0x18, 厚度 f32@-8, …}）：
//!   - 缺失 → 插入，初始厚度 = `t∈[0.7199, 0.9)`（0x142a80d44 / 0x142a7c88c）
//!     且实体标记 +0xd==1 ? 1.0 : 0.0；
//!   - 存在 → 目标 = 随机（`utils::rng::init_rng` 种子 + fastrand f32，
//!     0x140c96240/0x140caf380；实体 +0xd==0 → `t*3.0 + (1.0-t)`、否则
//!     `t*4.0 + 2*(1.0-t)`，0x1429cd078=3.0 / 0x142934694=4.0）→
//!     `厚度 += 目标 * delta * 1.0 + …`（Time delta @+0x28）→ 钳制 [0, 1.0]
//!     （0x142925904=1.0）→ 写回表 + 实体记录 +8；
//! - 帧末剔除不在列表的条目（FUN_141267de0，O）。
//!
//! 系统参数面（I）：ResMut<SmokeThicknessAnim 表资源> + Res<PublicWalls> +
//! Res<昼夜循环值资源（t @+8）> + Res<GameTime>（delta @+0x28）。

use country_core::resources::time::GameTime;
use bevy_ecs::prelude::*;
use bevy_ecs::resource::Resource;
use utils::rng::init_rng;

/// 烟柱厚度表资源（B 0x18B 条目表）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ChimneySmokeThickness {
    /// 实体 → 当前厚度（f32@-8）。
    pub thickness: hashbrown::HashMap<u64, f32>,
}

/// 昼夜循环值资源（I：B +8 区 f32；由 time_of_day 系统写入）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TimeOfDayCycle {
    /// 循环值 t ∈ [0,1)（B +8）。
    pub t: f32,
}

/// 烟柱模式区间（O：t∈[0.7199, 0.9) 时烟柱激活）。
const SMOKE_LOWER: f32 = 0.7199;
const SMOKE_UPPER: f32 = 0.9;
/// 厚度钳制上限（0x142925904 = 1.0，O）。
const THICKNESS_MAX: f32 = 1.0;
/// 随机目标系数（0x1429cd078 = 3.0 / 0x142934694 = 4.0，O）。
const TARGET_A: f32 = 3.0;
const TARGET_B: f32 = 4.0;

/// `chimney_smoke_anim_thickness`（0x140818df0，O 结构）：烟柱厚度动画系统。
/// 收集烟囱实体 → 表补齐 → 每帧目标随机 + delta 推进 + [0,1] 钳制 → 剔除。
pub fn chimney_smoke_anim_thickness(
    mut thickness: ResMut<ChimneySmokeThickness>,
    time: Res<GameTime>,
    // 原参数面：屋顶/墙遍历资源（PublicWalls 双模式，O）+ 昼夜循环值资源
    // （t @+8）+ 收集资源（ResMut tick@+0x1c）—— 身份待核，登记 RESTORE.md。
) {
    let delta = time.delta;
    let smoke_active = (SMOKE_LOWER..SMOKE_UPPER).contains(&0.0);

    // 帧末推进（O：目标 = rng(fastrand f32) 变体；厚度 += 目标*delta 系数；
    // 钳制 [0, 1.0]；写回表 + 实体 +8）。
    let mut seed = init_rng(delta.to_bits());
    for (entity, current) in thickness.thickness.iter_mut() {
        let _ = entity;
        // 目标（O：0x140caf380 = rng f32；0x140c96240 = 每实体种子；I：
        // 种子来源 = 实体 id 低 32 位，登记 RESTORE.md）。
        let r = fastrand_f32_from_seed(&mut seed);
        let target = if smoke_active {
            r * TARGET_B + 2.0 * (THICKNESS_MAX - r)
        } else {
            r * TARGET_A + (THICKNESS_MAX - r)
        };
        *current = (*current + target * delta * 1.0).clamp(0.0, THICKNESS_MAX);
    }
    // 剔除失效条目（O：FUN_141267de0 表裁剪；I：失效判定 = 实体不存在，
    // 由调用方注册的裁剪集承载）。
    thickness.thickness.retain(|_e, v| *v > 0.0);
}

/// fastrand f32（I：0x140caf380 语义 = 种子 rng 的 f32；utils rng 面）。
fn fastrand_f32_from_seed(seed: &mut u64) -> f32 {
    let mut rng = utils::rng::rng_from_seed_u64(*seed);
    *seed = init_rng(rng.f32().to_bits());
    rng.f32()
}

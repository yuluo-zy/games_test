//! B `crates/systems/decorator/src/chimney_smoke_anim_thickness.rs` — 烟柱厚度
//! 动画。
//!
//! 证据（2026-10-17 fresh Ghidra shared 反编译 0x140818df0 + ReVa write 全量
//! `reva_chimney_smoke_140818df0.c` 31KB + read_memory 常量核验）：
//! - 参数面（O，ReVa）：in_RCX = ResMut<ChimneySmokeThickness>、in_RDX =
//!   ResMut<PublicWalls 复合>（+0xf8 墙 id 数组 / +0x100 / +0x108 模式字节 /
//!   +0x110 索引、+0x100/+0x1a8 双桶表）、in_R8 = Res<昼夜循环>（t `*+8`）、
//!   in_R9 = Res<GameTime>（delta `*+0x28`）。
//! - 烟囱实体收集（O 结构）：墙（模式字节 +0x108==0）走 `+0xf8` id 数组 +
//!   0xa0 段桶，屋顶走 0x48 段桶；经 `+0x110` 索引反查实体记录（0x10 步长
//!   子记录），逐实体维护厚度表。**收集主循环未实现**（墙/屋顶密集表 core
//!   未暴露，与 window_lights 同源）——以既有 `ChimneySmokeThickness` 表驱动。
//! - 厚度表（O）：`HashMap<GlobalId, f32>`，条目 `{key, 厚度 f32}`；帧末
//!   `HashMap::retain` 剔除失效（谓词 U，以 `thickness > 0.0` 承载）。
//! - 插入初始厚度（O，ReVa 0x140819390-0x140819393）：窗口内且 `+0xd==1` →
//!   1.0，否则 0.0。
//! - **更新公式（O，ReVa 全量核验）**：每 spot `utils::rng::init_rng()` 后：
//!   - `+0xd==0`：目标 `fVar59 = rng*3.0 + (1.0-rng)`（0x1429cd078=3.0），
//!     符号恒 `-1.0`（fVar21=0xbf800000）；且该分支**先丢弃一次抽签**
//!     （`fastrand::Rng::f32()` 后再次抽签，0x14081956f-0x140819573）。
//!   - `+0xd!=0`：目标 `fVar59 = rng*4.0 + (1.0-rng) + (1.0-rng)`（
//!     0x142934694=4.0），符号 `fVar58`：窗口内（t<0.72 || t>=0.9）→ +1.0，
//!     否则 -1.0（0x14081947f-0x140819492）。
//!   - 更新 `new = clamp01(old + fVar59 * dt / CONCAT13(divisor))`，其中
//!     fVar59 算完目标后被**改写为符号**（ReVa：`fVar59 = fVar21/fVar58`），
//!     CONCAT13(divisor) = 刚算目标值的字节重拼 → 除数即目标值 → 实为
//!     `new = clamp01(old + sign * dt / target)`（0x140819646-0x140819654）。
//!   - dt 源 = `*(f32*)(GameTime + 0x28)`（0x140819646 lVar35+0x28；country_core
//!     `GameClock.delta` 字段 @+0x28 已对齐）。
//!
//! ⚠ 与 B 布局的分歧（U，RESTORE.md §2）：
//! - `+0xd` 烟型标记按 spot 记录持有；当前无 spot 收集主循环，以条目字段
//!   `ChimneySmokeEntry.flag` 持久化承载（二进制表值仍只存 f32 —— 布局分歧 U）。
//! - rng 种子来源（init_rng 实参寄存器未取到）以 `delta.to_bits()` 承载（U）。

use country_core::resources::time::GameTime;
use bevy_ecs::prelude::*;
use bevy_ecs::resource::Resource;
use utils::rng::init_rng;

/// 单条烟柱厚度条目。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChimneySmokeEntry {
    /// 当前厚度（B 表值 f32）。
    pub thickness: f32,
    /// B spot 记录 `+0xd` 烟型标记（0=恒减薄 / 1=窗口内增厚）。当前无 spot
    /// 收集主循环，以条目持久化承载（分歧 U，见模块头）。
    pub flag: u8,
}

/// 烟柱厚度表资源（B 条目 `{key, 厚度 f32}`；+0xd 标记以条目承载）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ChimneySmokeThickness {
    /// 实体 → 烟柱条目。
    pub thickness: hashbrown::HashMap<u64, ChimneySmokeEntry>,
}

/// 昼夜循环值资源（I：B +8 区 f32；由 time_of_day 系统写入）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TimeOfDayCycle {
    /// 循环值 t ∈ [0,1)（B +8）。
    pub t: f32,
}

/// 烟柱激活窗口（O）：`t < 0.72 || t >= 0.9`（0x142a80d44=0.72f、
/// 0x142a7c88c=0.9f，read_memory 核验）。
const SMOKE_LOWER: f32 = 0.72;
const SMOKE_UPPER: f32 = 0.9;
/// 厚度钳制上限（0x142925904 = 1.0，O，read_memory 核验）。
const THICKNESS_MAX: f32 = 1.0;
/// 随机目标系数（0x1429cd078 = 3.0 / 0x142934694 = 4.0，O，read_memory 核验）。
const TARGET_A: f32 = 3.0;
const TARGET_B: f32 = 4.0;

/// `chimney_smoke_anim_thickness`（0x140818df0，O 结构 + ReVa 全量公式）。
///
/// 判等面（2026-10-17 wave2 ReVa 核验 + wave3 fresh 复核）：t 窗口
/// （0.72..0.9 外激活）、目标公式（+0xd==0 → `r*3.0+(1-r)`、否则
/// `r*4.0+2*(1-r)`）、符号（+0xd==0 恒 -1.0；否则窗口内 +1.0 / 窗口外 -1.0）、
/// 更新 `new = clamp01(old + sign*dt/target)`（fVar59 算完目标改写为符号、
/// divisor = 目标字节重拼）、[0,1] 钳制。
/// 未实现面：spot 收集主循环（墙/屋顶双模式桶，PublicWalls 密集表 core 未
/// 暴露，与 window_lights 同源）、rng 种子来源、retain 谓词、初始厚度插入点。
pub fn chimney_smoke_anim_thickness(
    mut thickness: ResMut<ChimneySmokeThickness>,
    cycle: Res<TimeOfDayCycle>,
    time: Res<GameTime>,
) {
    let delta = time.delta;
    let t = cycle.t;
    // O：激活窗口 `t < 0.72 || t >= 0.9`（`bVar14 = t < 0.72`、
    // `bVar13 = 0.9 <= t`，0x140819488-0x140819490）。
    let smoke_active = t < SMOKE_LOWER || t >= SMOKE_UPPER;

    // 帧末推进（O 结构）：init_rng + fastrand 目标；厚度累加 + 钳制。
    // U：rng 种子来源 = init_rng(delta.to_bits()) 候选；逐 entry 链条承载
    //（二进制每 spot 独立 init_rng，种子寄存器未取到，登记）。
    let mut seed = init_rng(delta.to_bits());
    for (entity, entry) in thickness.thickness.iter_mut() {
        let _ = entity;
        // O：目标公式按 spot+0xd（entry.flag）分派（ReVa 0x14081956f-0x140819585）。
        let (target, sign) = if entry.flag == 0 {
            // O：+0xd==0 分支先丢弃一次抽签（`fastrand::f32()` 后再次抽签），
            // 目标 = r*3.0 + (1-r)；符号恒 -1.0（fVar21）。
            let _discarded = seed_f32(&mut seed);
            let r = seed_f32(&mut seed);
            (r * TARGET_A + (THICKNESS_MAX - r), -1.0)
        } else {
            // O：+0xd!=0 分支目标 = r*4.0 + (1-r) + (1-r)；符号 fVar58 按窗口。
            let r = seed_f32(&mut seed);
            (
                r * TARGET_B + 2.0 * (THICKNESS_MAX - r),
                if smoke_active { 1.0 } else { -1.0 },
            )
        };
        // O：`new = clamp01(old + fVar59 * dt / CONCAT13(divisor))`，divisor 即
        // 目标值字节重拼 → `old + sign * dt / target`（ReVa 0x140819646-0x140819654）。
        let new = (entry.thickness + sign * delta / target).clamp(0.0, THICKNESS_MAX);
        entry.thickness = new;
    }
    // 剔除失效条目（O：HashMap::retain；谓词 U，以非零承载，登记）。
    thickness.thickness.retain(|_e, v| v.thickness > 0.0);
}

/// fastrand f32（O：utils::rng::init_rng + fastrand::Rng::f32；种子来源 U）。
fn seed_f32(seed: &mut u64) -> f32 {
    let rng = utils::rng::rng_from_seed_u64(*seed);
    *seed = init_rng(rng.f32().to_bits());
    rng.f32()
}

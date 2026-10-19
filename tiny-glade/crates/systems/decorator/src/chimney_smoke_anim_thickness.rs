//! B `crates/systems/decorator/src/chimney_smoke_anim_thickness.rs` — 烟柱厚度
//! 动画（`chimney_smoke_anim_thickness` 0x140818df0）。
//!
//! # 证据与参数面
//!
//! 反编译：`evidence/restorer-state-decorator/decomp/0x140818df0_chimney_smoke_anim_thickness.c`
//! （31 KB 全文，2026-10-17 ReVa write 落盘；本文件逐段复核）。
//!
//! PDB 候选系统签名（`skeleton/B/crates/systems/decorator/src/chimney_smoke_anim_thickness.rs`，
//! 「文件级接口候选，具体函数归属 unknown」）：
//!
//! ```text
//! void (*)(ResMut<GlobalSmokeControl>,
//!          Query<&mut Smoke>,
//!          Res<country_core::resources::time_of_day::TimeOfDay>,
//!          Res<country_core::resources::time::GameTime>)
//! ```
//!
//! 注：候选把第二参写作 `Query<&mut Smoke>`，但反编译 `in_RDX` 的读面
//! （`+0xf8/+0x100/+0x108/+0x110` 与 0xa0/0x48 段表）与其它装饰系统的
//! PublicWalls/墙状态复合布局一致；本文件按机器码读面建模 spot 收集源。
//!
//! 与反编译寄存器面的对应（O）：
//! - `in_R8`：`fVar54 = *(f32 *)(*in_R8 + 8)` → `TimeOfDay.time_of_day`（core 布局
//!   `+0x08`，见 `country-core/src/resources/time_of_day.rs`）。
//! - `in_R9`：`*(f32 *)(lVar30 + 0x28)` → `GameTime.delta`（core 注释 `delta @+0x28`）。
//! - `in_RCX`：厚度表资源（`plVar21 = *in_RCX`；PDB retain 单态
//!   `HashMap::retain<usize, f32, RandomState, Global, closure_env$0>`；表项
//!   16 B `{key usize @+0, thickness f32 @+8}`；厚度初值写 0x3f800000/0）。
//! - `in_RDX`：烟柱 spot 收集源（`lVar7 = *in_RDX` 墙 `+0xf8` id 数组 +
//!   `0xa0` 段表、`in_RDX[1]` 屋顶 `0x48` 段表，`+0x108` 模式字节、`+0x110`
//!   索引；spot 记录 0x10 B，首字段为 GlobalId/句柄，`+0xd` 为烟型字节）。
//!   **收集主循环未落 Rust**：core 未暴露对应墙/屋顶密集表；本模块以
//!   [`ChimneySmokeThickness::spots`] 作为收集结果的输入面（U，见 §U）。
//!
//! # 状态机（逐条 O，行号为反编译文件行）
//!
//! 1. **帧初插入**（L383）：对每个 spot，
//!    `thickness.entry(key).or_insert(if (t<0.72 || 0.9<=t) && spot.+0xd==1 { 1.0 } else { 0.0 })`。
//! 2. **帧末 retain**（L474：`HashMap::retain<usize,f32,closure_env$0>`）：闭包
//!    使用 `slice::contains`（PDB closure$0 依赖 `core::slice::impl$0::contains`）
//!    → 只保留本帧 spot 列表中的 key。**不是** `thickness > 0.0`。
//! 3. **逐 spot 更新**（L553-646）：
//!    - 每个 spot 独立 `utils::rng::init_rng()`（O 调用点；实参 = U）；
//!    - 先抽一次 `fastrand::Rng::f32()`；
//!    - `+0xd == 0`：**丢弃首抽**，再抽 `r`，目标 `r*3.0 + (1-r)`，符号恒 `-1.0`；
//!    - `+0xd != 0`：目标 `r*4.0 + (1-r) + (1-r)`，符号窗口内 `+1.0`、窗口外 `-1.0`
//!      （窗口 = `t<0.72 || 0.9<=t`，L477-479）；
//!    - `new = clamp01(old + sign * dt / target)`（L633-644；除数即目标值字节重拼）；
//!    - 写回厚度表（`entry value f32`）与 spot 记录 `+8` 输出（L646 `*(float *)(plVar37+1)`）；
//!    - 查表失败走 `core::option::unwrap_failed`（L636 分支），本实现保留 `expect` panic。
//!
//! # U（证据不足，禁止默认值冒充）
//!
//! - spot 收集主循环（`in_RDX` 墙/屋顶双模式桶）未实现：core 的 PublicWalls/屋顶
//!   密集表未暴露，`spots` 由外部收集器填充（帧内输入，处理后原地保留输出厚度）。
//! - `init_rng` 实参未取到；本实现以 spot key 作候选种子并登记 U（不影响公式）。
//! - spot 记录 0x10 B 的次字段（`+8` 之前/之后）语义未取到；本实现只承载
//!   `key` / `+0xd` 烟型 / `+8` 厚度输出三个已验证面。
//! - `TimeOfDayCycle` 是 crate 内镜像资源（lantern 消费，见 `decorator_visual/lantern.rs`）；
//!   系统优先读真实 `TimeOfDay.time_of_day`，缺失时才回退镜像（U）。

use bevy_ecs::prelude::*;
use bevy_ecs::resource::Resource;
use country_core::resources::time::GameTime;
use country_core::resources::time_of_day::TimeOfDay;
use utils::rng::init_rng;

/// 烟柱激活窗口下界（0x142a80d44 = 0.72f，O：read_memory 复核）。
const SMOKE_LOWER: f32 = 0.72;
/// 烟柱激活窗口上界（0x142a7c88c = 0.9f，O）。
const SMOKE_UPPER: f32 = 0.9;
/// 厚度上限（0x142925904 = 1.0）。
const THICKNESS_MAX: f32 = 1.0;
/// 随机目标系数（0x1429cd078 = 3.0 / 0x142934694 = 4.0）。
const TARGET_A: f32 = 3.0;
const TARGET_B: f32 = 4.0;

/// 单个烟柱 spot（O：0x10 B 记录中已验证的三个面）。
///
/// - `key`：0x10 B 记录首字段（GlobalId/句柄），作为厚度表 `usize` 键；
/// - `flag`：记录 `+0xd` 烟型字节（0 = 恒减薄 / 1 = 窗口内增厚）；
/// - `thickness`：记录 `+8` 的厚度输出副本（原版每帧写回，L646）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SmokeSpot {
    /// 厚度表键（O：`HashMap<usize, f32>`）。
    pub key: usize,
    /// spot 记录 `+0xd` 烟型标记（O）。
    pub flag: u8,
    /// spot 记录 `+8` 厚度输出（O：L646 写回）。
    pub thickness: f32,
}

/// 烟柱厚度表资源（O：`HashMap<usize, f32>` + spot 收集输入）。
///
/// 原版 `GlobalSmokeControl` 的厚度表；`spots` 是未实现收集主循环（U）的输入面：
/// 帧内收集器写入本帧 spot 列表，系统处理后原地更新 `SmokeSpot::thickness`。
#[derive(Resource, Clone, Debug, Default)]
pub struct ChimneySmokeThickness {
    /// spot key → 厚度（O：PDB `HashMap<usize, f32>` 单态）。
    pub thickness: hashbrown::HashMap<usize, f32>,
    /// 本帧 spot 列表（U：收集主循环未实现；顺序即原版桶迭代顺序）。
    pub spots: Vec<SmokeSpot>,
}

/// 昼夜循环镜像资源（I/U）：`TimeOfDayCycle.t` = `TimeOfDay.time_of_day`。
///
/// `decorator_visual/lantern.rs` 直接消费该镜像；系统优先读真实 `TimeOfDay`。
#[derive(Resource, Clone, Debug, Default)]
pub struct TimeOfDayCycle {
    /// 循环值 t ∈ [0,1)（对应 core `TimeOfDay +0x08`）。
    pub t: f32,
}

/// 烟柱激活窗口（O：`t < 0.72 || t >= 0.9`）。
#[inline]
pub fn smoke_is_active(t: f32) -> bool {
    t < SMOKE_LOWER || t >= SMOKE_UPPER
}

/// 帧初插入的初始厚度（O：L383）。
///
/// 窗口内且 spot `+0xd == 1` → `1.0`，否则 `0.0`。
#[inline]
pub fn initial_thickness(t: f32, flag: u8) -> f32 {
    if smoke_is_active(t) && flag == 1 {
        THICKNESS_MAX
    } else {
        0.0
    }
}

/// 随机目标值与符号（O：L558-571）。
///
/// `flag == 0` → `(r*3 + (1-r), -1.0)`；否则 `(r*4 + 2*(1-r), active ? +1 : -1)`。
#[inline]
pub fn target_and_sign(flag: u8, active: bool, r: f32) -> (f32, f32) {
    if flag == 0 {
        (r * TARGET_A + (THICKNESS_MAX - r), -1.0)
    } else {
        (
            r * TARGET_B + (THICKNESS_MAX - r) + (THICKNESS_MAX - r),
            if active { 1.0 } else { -1.0 },
        )
    }
}

/// 单步推进：`clamp01(old + sign * dt / target)`（O：L633-644）。
///
/// 除数 = 目标值（`CONCAT13(divisor)` 即目标 f32 字节重拼）；`clamp01` 是
/// L637-643 的 `0.0 <= v` / `v <= 1.0` 两段选择。
#[inline]
pub fn step_thickness(old: f32, sign: f32, target: f32, delta: f32) -> f32 {
    (old + sign * delta / target).clamp(0.0, THICKNESS_MAX)
}

/// 每 spot RNG（O 调用点；种子实参 U）。
///
/// 原版每个 spot 独立调用 `utils::rng::init_rng()` 后使用全局 `fastrand` 抽签；
/// 实参寄存器未取到，本实现以 spot key 作候选种子（U，不影响公式）。
#[inline]
fn smoke_rng(spot_key: usize) -> fastrand::Rng {
    fastrand::Rng::with_seed(init_rng(spot_key as u32))
}

/// 厚度状态机推进（O 全流程；spot 收集 U）。
///
/// 对应 0x140818df0 的 L383 → L474 → L553-646 三段：插入初值、按 live spot
/// retain、逐 spot 随机演化并写回。`table.spots` 既是输入也是输出（`thickness`
/// 字段被更新）。
pub fn advance_thickness(table: &mut ChimneySmokeThickness, t: f32, delta: f32) {
    let active = smoke_is_active(t);

    // 1. 帧初插入（O L383）：缺失项写 1.0/0.0。
    for spot in &table.spots {
        table
            .thickness
            .entry(spot.key)
            .or_insert_with(|| initial_thickness(t, spot.flag));
    }

    // 2. 帧末 retain（O L474，closure = slice::contains）：只保留本帧 spot key。
    let live_keys: Vec<usize> = table.spots.iter().map(|spot| spot.key).collect();
    table
        .thickness
        .retain(|key, _value| live_keys.contains(key));

    // 3. 逐 spot 更新（O L553-646）。
    for spot in &mut table.spots {
        // O：每 spot 一次 init_rng + 一次无条件 f32 抽签。
        let rng = smoke_rng(spot.key);
        let first_draw = rng.f32();
        let (target, sign) = if spot.flag == 0 {
            // O L558：先丢弃一次抽签，再抽目标随机数。
            let _discarded = first_draw;
            let r = rng.f32();
            target_and_sign(0, active, r)
        } else {
            // O L566：首抽即目标随机数。
            target_and_sign(spot.flag, active, first_draw)
        };
        // O L636：查表失败为 unwrap_failed（保留 panic）。
        let old = *table
            .thickness
            .get(&spot.key)
            .expect("smoke spot thickness missing");
        let new = step_thickness(old, sign, target, delta);
        table.thickness.insert(spot.key, new);
        // O L646：写回 spot 记录 +8。
        spot.thickness = new;
    }
}

/// `chimney_smoke_anim_thickness`（0x140818df0，O 结构 + 全量公式）。
///
/// 优先读取真实 `TimeOfDay.time_of_day`（O `+0x08`）；资源缺失时回退 crate 内
/// `TimeOfDayCycle` 镜像（U）。`delta` = `GameTime.delta`（O `+0x28`）。
pub fn chimney_smoke_anim_thickness(
    mut thickness: ResMut<ChimneySmokeThickness>,
    cycle: Res<TimeOfDayCycle>,
    time: Res<GameTime>,
    time_of_day: Option<Res<TimeOfDay>>,
) {
    let t = time_of_day
        .as_deref()
        .map(|tod| tod.time_of_day)
        .unwrap_or(cycle.t);
    advance_thickness(&mut thickness, t, time.delta);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spot(key: usize, flag: u8) -> SmokeSpot {
        SmokeSpot {
            key,
            flag,
            thickness: 0.0,
        }
    }

    #[test]
    fn activation_window_matches_native_constants() {
        assert!(smoke_is_active(0.0));
        assert!(smoke_is_active(0.719_999));
        assert!(!smoke_is_active(0.72)); // 下界严格 <
        assert!(!smoke_is_active(0.8));
        assert!(!smoke_is_active(0.899_999));
        assert!(smoke_is_active(0.9)); // 上界闭区间
        assert!(smoke_is_active(1.0));
    }

    #[test]
    fn initial_thickness_requires_window_and_flag_one() {
        assert_eq!(initial_thickness(0.1, 1), 1.0);
        assert_eq!(initial_thickness(0.95, 1), 1.0);
        assert_eq!(initial_thickness(0.1, 0), 0.0);
        assert_eq!(initial_thickness(0.8, 1), 0.0);
        assert_eq!(initial_thickness(0.8, 2), 0.0);
    }

    #[test]
    fn target_formulas_match_native() {
        // flag 0: r*3 + (1-r) ∈ [1, 3]，符号恒 -1。
        let (t0, s0) = target_and_sign(0, true, 0.0);
        assert_eq!((t0, s0), (1.0, -1.0));
        let (t1, s1) = target_and_sign(0, false, 0.5);
        assert_eq!((t1, s1), (2.0, -1.0));
        let (t2, _) = target_and_sign(0, true, 1.0);
        assert_eq!(t2, 3.0);
        // flag != 0: r*4 + 2*(1-r) ∈ [2, 4]，符号随窗口。
        let (t3, s3) = target_and_sign(1, true, 0.0);
        assert_eq!((t3, s3), (2.0, 1.0));
        let (t4, s4) = target_and_sign(1, false, 0.5);
        assert_eq!((t4, s4), (3.0, -1.0));
        let (t5, _) = target_and_sign(1, true, 1.0);
        assert_eq!(t5, 4.0);
    }

    #[test]
    fn step_uses_division_by_target_and_clamps01() {
        // new = old + sign*dt/target
        assert!((step_thickness(0.5, 1.0, 2.0, 0.25) - 0.625).abs() < 1e-6);
        assert!((step_thickness(0.5, -1.0, 2.0, 0.25) - 0.375).abs() < 1e-6);
        // 下钳制。
        assert_eq!(step_thickness(0.1, -1.0, 1.0, 5.0), 0.0);
        // 上钳制。
        assert_eq!(step_thickness(0.9, 1.0, 1.0, 5.0), 1.0);
    }

    #[test]
    fn retain_drops_keys_not_in_live_spots() {
        let mut table = ChimneySmokeThickness::default();
        table.thickness.insert(1, 0.5);
        table.thickness.insert(2, 0.5);
        table.thickness.insert(3, 0.5);
        table.spots = vec![spot(1, 0), spot(3, 0)];
        advance_thickness(&mut table, 0.5, 0.0);
        assert!(table.thickness.contains_key(&1));
        assert!(!table.thickness.contains_key(&2)); // 非 live 即使厚度 > 0 也删除
        assert!(table.thickness.contains_key(&3));
    }

    #[test]
    fn missing_spot_initialized_before_update() {
        let mut table = ChimneySmokeThickness::default();
        // 窗口外、flag=1 → 初值 0.0；符号 -1，delta=0 → 仍 0。
        table.spots = vec![spot(7, 1)];
        advance_thickness(&mut table, 0.8, 0.0);
        assert_eq!(table.thickness[&7], 0.0);
        assert_eq!(table.spots[0].thickness, 0.0);
        // 窗口内、flag=1 → 初值 1.0。
        let mut table = ChimneySmokeThickness::default();
        table.spots = vec![spot(7, 1)];
        advance_thickness(&mut table, 0.1, 0.0);
        assert_eq!(table.thickness[&7], 1.0);
    }

    #[test]
    fn flag_zero_always_thins_and_writes_back_spot() {
        let mut table = ChimneySmokeThickness::default();
        table.thickness.insert(5, 1.0);
        table.spots = vec![spot(5, 0)];
        advance_thickness(&mut table, 0.5, 0.1);
        // 符号 -1、target ∈ [1,3] → 厚度严格下降；输出副本同步。
        assert!(table.thickness[&5] < 1.0);
        assert_eq!(table.spots[0].thickness, table.thickness[&5]);
    }

    #[test]
    fn flag_one_thickens_inside_window() {
        let mut table = ChimneySmokeThickness::default();
        table.thickness.insert(9, 0.2);
        table.spots = vec![spot(9, 1)];
        advance_thickness(&mut table, 0.5, 0.1);
        // 窗口内（0.5 ∈ [0.72 外？0.5<0.72 → active）符号 +1 → 上升。
        assert!(table.thickness[&9] > 0.2);
    }
}

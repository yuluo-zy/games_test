//! 已判等函数 / 原生常量的数值断言（tester-numeric-assertions，2026-10-17）。
//!
//! 证据：`roof_animation` 0x1421a2da0 —— `reports/state/roof/REVIEW.md` verdict **判等**：
//! - 删除标记分支 `primary = dt·-4.0`、`secondary = dt·-10.0`；
//! - 正常分支因子 `is_flat ? 2.0 : 1.0`；`primary = dt·2.857143·factor`；
//!   `secondary = (value[0] > 0.99 ? dt·3.3333333 : 0)·factor`；
//! - 逐通道 `delta != 0` 才写 tick 并 `clamp(value+delta, 0, 1)`。
//! 常量立即数：0x4036db6e=2.857143、0x40555555=3.3333333、0x3f7d70a4=0.99；
//! `is_flat` 门控来自 `roof[0x30]`（`ty_observed` 阈值 0x3dcccccd=0.1）。
//!
//! 实现位于 country-core `RoofAnimation::advance`，由 system-roof `roof_animation`
//! 委托消费；本测试从 roof crate 直接调用已判等内核（不跑 ECS 迭代）。

use country_core::resources::roofs::components::RoofAnimation as InnerRoofAnimation;
use country_core::resources::roofs::RoofAnimation;

const PRIMARY_RATE: f32 = f32::from_bits(0x4036db6e); // 2.857143
const SECONDARY_RATE: f32 = f32::from_bits(0x40555555); // 3.3333333
const SECONDARY_THRESHOLD: f32 = f32::from_bits(0x3f7d70a4); // 0.99

fn animation(values: [f32; 2]) -> RoofAnimation {
    let mut bytes = [0u8; 8];
    bytes[..4].copy_from_slice(&values[0].to_le_bytes());
    bytes[4..].copy_from_slice(&values[1].to_le_bytes());
    RoofAnimation::from_inner(InnerRoofAnimation::from_observed(bytes))
}

/// `roof[0x30]` = 高度字节；`ty_observed` 为 `0.1 < value`（无序/NaN 也真）。
fn roof_bytes(value_at_0x30: f32) -> [u8; 88] {
    let mut bytes = [0u8; 88];
    bytes[0x30..0x34].copy_from_slice(&value_at_0x30.to_le_bytes());
    bytes
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 1e-6,
        "actual={actual} expected={expected}"
    );
}

/// 删除标记：`dt·-4.0` / `dt·-10.0`，从 1.0 降并 clamp 到 0；两通道均写 tick。
#[test]
fn marked_branch_decays_and_clamps() {
    let mut anim = animation([1.0, 1.0]);
    let mut tick = 0;
    anim.advance_observed(&roof_bytes(0.0), true, 0.25, &mut tick, 42);
    assert_eq!(anim.values(), [0.0, 0.0]);
    assert_eq!(tick, 42);
}

/// 正常平顶（`roof[0x30] = 0 < 0.1` → factor 2.0）：
/// primary = 0.1·2.857143·2 = 0.5714286；secondary 条件 0>0.99 假 → 0，不写。
#[test]
fn flat_roof_uses_factor_two() {
    let mut anim = animation([0.0, 0.0]);
    let mut tick = 0;
    anim.advance_observed(&roof_bytes(0.0), false, 0.1, &mut tick, 7);
    assert_close(anim.values()[0], 0.1 * PRIMARY_RATE * 2.0);
    assert_close(anim.values()[0], 0.571_428_6);
    assert_eq!(anim.values()[1], 0.0);
    assert_eq!(tick, 7);
}

/// 正常坡顶（`roof[0x30] = 1.0 > 0.1` → factor 1.0）：primary 到顶 clamp 1.0；
/// 初值 0.95 不满足 >0.99，secondary 通道不写。
#[test]
fn pitched_roof_primary_clamps_to_one() {
    let mut anim = animation([0.95, 0.0]);
    let mut tick = 0;
    anim.advance_observed(&roof_bytes(1.0), false, 0.1, &mut tick, 7);
    assert_eq!(anim.values(), [1.0, 0.0]);
    assert_eq!(tick, 7);
}

/// secondary 门控边界：初值必须严格 > 0.99。0.99 本身不触发；0.995 触发
/// `dt·3.3333333` 并 clamp。
#[test]
fn secondary_threshold_is_strictly_greater_than_099() {
    // 0.99 边界（原生 immediate 0x3f7d70a4）：secondary 保持。
    let mut at_boundary = animation([0.99, 0.4]);
    let mut tick = 0;
    at_boundary.advance_observed(&roof_bytes(1.0), false, 0.1, &mut tick, 1);
    assert_eq!(at_boundary.values()[1], 0.4);

    // 0.995 > 0.99：secondary += 0.1·3.3333333 = 0.3333333。
    let mut above = animation([0.995, 0.5]);
    let mut tick = 0;
    above.advance_observed(&roof_bytes(1.0), false, 0.1, &mut tick, 1);
    assert_eq!(above.values()[0], 1.0);
    assert_close(above.values()[1], 0.5 + 0.1 * SECONDARY_RATE);
    assert_close(above.values()[1], 0.833_333_3);
}

/// `ty_observed` 阈值边界 0.1（0x3dcccccd）：恰为 0.1 → 平顶 factor 2.0；
/// 下一个 f32 才走坡顶 factor 1.0。
#[test]
fn flat_threshold_boundary_at_point_one() {
    let mut flat = animation([0.0, 0.0]);
    let mut tick = 0;
    flat.advance_observed(
        &roof_bytes(f32::from_bits(0x3dcccccd)),
        false,
        0.1,
        &mut tick,
        1,
    );
    assert_close(flat.values()[0], 0.1 * PRIMARY_RATE * 2.0);

    let mut pitched = animation([0.0, 0.0]);
    pitched.advance_observed(
        &roof_bytes(f32::from_bits(0x3dcccccd + 1)),
        false,
        0.1,
        &mut tick,
        1,
    );
    assert_close(pitched.values()[0], 0.1 * PRIMARY_RATE);
    // 两分支相差 2 倍。
    assert_close(flat.values()[0], 2.0 * pitched.values()[0]);
}

/// `delta == 0` 的通道不写值、不写 tick；dt=0 时两通道均无变化。
#[test]
fn zero_delta_leaves_value_and_tick_untouched() {
    let mut anim = animation([0.3, 0.4]);
    let mut tick = 0;
    anim.advance_observed(&roof_bytes(1.0), false, 0.0, &mut tick, 9);
    assert_eq!(anim.values(), [0.3, 0.4]);
    assert_eq!(tick, 0);
}

/// NaN 语义：dt=NaN → primary 通道为 NaN，仍算 `delta != 0` → 写 tick；
/// clamp 的 `0>NaN`/`1<NaN` 均为假，通道保留 NaN。secondary 由初值 0.3 未过
/// 0.99 门控而为 0，不写通道。
#[test]
fn nan_delta_writes_nan_primary_and_tick() {
    let mut anim = animation([0.3, 0.4]);
    let mut tick = 0;
    anim.advance_observed(&roof_bytes(1.0), false, f32::NAN, &mut tick, 5);
    assert!(anim.values()[0].is_nan());
    assert_eq!(anim.values()[1], 0.4);
    assert_eq!(tick, 5);
}

/// 常量立即数位值（fresh 反编译）：0x4036db6e / 0x40555555 / 0x3f7d70a4。
#[test]
fn native_rate_immediates() {
    assert_eq!(PRIMARY_RATE.to_bits(), 0x4036_db6e);
    assert_eq!(SECONDARY_RATE.to_bits(), 0x4055_5555);
    assert_eq!(SECONDARY_THRESHOLD.to_bits(), 0x3f7d_70a4);
    assert!((PRIMARY_RATE - 2.857_143).abs() < 1e-6);
    assert!((SECONDARY_RATE - 3.333_333_3).abs() < 1e-6);
    assert!((SECONDARY_THRESHOLD - 0.99).abs() < 1e-8);
}

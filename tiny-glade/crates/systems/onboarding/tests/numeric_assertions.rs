//! 已判等函数 / 原生常量的数值断言（tester-numeric-assertions，2026-10-17）。
//!
//! 证据：
//! - `progress_bar_update_system` 0x141c530f0（`reports/interaction/onboarding/REVIEW.md`
//!   §1.7 **判等**）：量化 `p==0→0`；`0<p≤0.4→1`；`0.4<p≤0.99→2`；`p<0 或 p>0.99→3`；
//!   **NaN→1**（逗号表达式副作用）。`quantize` 为该系统抽出的同序判定。
//! - `DELETION_COOLDOWN_TABLE`：`hint_display_system.rs` 记录 0x143038ae8 区直读
//!   （按提示 id 的 0.3s 删除冷却）。

use system_onboarding::hint_display_system::{DELETION_COOLDOWN_TABLE, quantize};

/// 量化阈值边界：0.4 属档 1、0.99 属档 2；越界与 NaN。
#[test]
fn quantize_threshold_boundaries() {
    // p == 0（含 -0.0）。
    assert_eq!(quantize(0.0), 0);
    assert_eq!(quantize(-0.0), 0);
    // (0, 0.4] → 1。
    assert_eq!(quantize(0.2), 1);
    assert_eq!(quantize(0.4), 1);
    assert_eq!(quantize(f32::from_bits(0.4f32.to_bits() - 1)), 1);
    // (0.4, 0.99] → 2。
    assert_eq!(quantize(0.4_f32.next_up()), 2);
    assert_eq!(quantize(0.5), 2);
    assert_eq!(quantize(0.99), 2);
    // <0 或 >0.99 → 3。
    assert_eq!(quantize(-1e-6), 3);
    assert_eq!(quantize(0.99_f32.next_up()), 3);
    assert_eq!(quantize(1.2), 3);
    assert_eq!(quantize(f32::INFINITY), 3);
    assert_eq!(quantize(f32::NEG_INFINITY), 3);
    // NaN → 1（全比较为假落 else 分支）。
    assert_eq!(quantize(f32::NAN), 1);
}

/// 删除冷却表 0x143038ae8（native 20 项 id 0..19；本表取 19 项覆盖 id 18 过山车）：
/// 提示 1/4/5/6/7/10/11/12 为 0.3s，其余 0。
#[test]
fn deletion_cooldown_table_native_values() {
    assert_eq!(DELETION_COOLDOWN_TABLE.len(), 19);
    let expected = [
        0.0, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.3, 0.0, 0.0, 0.3, 0.3, 0.3, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
    ];
    assert_eq!(DELETION_COOLDOWN_TABLE, expected);
    for (index, value) in DELETION_COOLDOWN_TABLE.iter().enumerate() {
        assert_eq!(*value, if expected[index] == 0.0 { 0.0 } else { 0.3 });
    }
}

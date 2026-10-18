//! 已判等纯函数 / 原生常量的数值断言（tester-numeric-assertions，2026-10-17）。
//!
//! 证据（逐函数 fresh 判等）：
//! - `random_tree_prefab` 0x14226a170：`reports/eco/tree-tool/REVIEW.md` verdict **判等**
//!   （门控 + 7 串表逐分支吻合）；字符串表见 RESTORE §。
//! - `despawn_trees` 0x1422686d0：REVIEW **判等**；α 阈值 `0.01`（0x3c23d70a）。
//! - `recalc_tree_position_on_terrain_height_change` 0x142268dc0：REVIEW **判等**；
//!   世界边界 65.0（0x42820000，含边界）。
//! - `tree_move_bend` 0x1422689c0：REVIEW「bend 数学本身判等」；rate=5.0（0x142925988）、
//!   dist 阈值 0.1（0x142925984）。`time_continuous_lerp` 公式逐语句判等。
//! - `ui_place_tree` 主体未判等（REVIEW），但其中水下重映射行由字节直读
//!   （`DIVSS XMM0,-2.5` / `MULSS XMM0,-1.2`，RESTORE 记录）；本测试只锁该原生常量与
//!   公开纯 helper，不声称宿主函数整体判等。

use system_tree::process_tree_cmds::DESPAWN_ALPHA_THRESHOLD;
use system_tree::recalc_tree_position_on_terrain_height_change::WORLD_BOUND;
use system_tree::tree_move_bend::{
    TREE_BEND_DIST_THRESHOLD, TREE_BEND_RATE, TREE_BEND_STEP_MAX, time_continuous_lerp,
};
use system_tree::ui_place_tree::{
    TREE_PREFAB_AUTUMN, TREE_PREFAB_BRANCHY, TREE_PREFAB_NAKED, TREE_PREFAB_NO_MUSHROOMS,
    TREE_ROTATION_MAX, TREE_SCALE_MAX, TREE_SCALE_MIN, WATER_REMAP_DIVISOR, WATER_REMAP_TARGET,
    random_tree_prefab, remap_pos_y_if_in_water,
};

/// `random_tree_prefab` 的 7 项路径表 + 三门控（corrupted / deny_load / halloween / drag）
/// 与 bit=`selector&1` 分派；期望字符串逐字来自 0x1430351d0 区直读。
#[test]
fn random_tree_prefab_branch_literals() {
    // 默认 branchy：bit 0 → *2 表首、bit 1 → *1 表次。
    assert_eq!(
        random_tree_prefab(0, false, false, false, false),
        "clearing/tree/branchy2"
    );
    assert_eq!(
        random_tree_prefab(1, false, false, false, false),
        "clearing/tree/branchy1"
    );
    // drag_started → autumn 表同一 bit 分派。
    assert_eq!(
        random_tree_prefab(0, false, false, false, true),
        "clearing/tree/autumn/branchy2"
    );
    assert_eq!(
        random_tree_prefab(1, false, false, false, true),
        "clearing/tree/autumn/branchy1"
    );
    // corrupted 且 bit==0 → naked1_no_mushrooms（bit==1 不命中，落默认表）。
    assert_eq!(
        random_tree_prefab(0, false, false, true, false),
        TREE_PREFAB_NO_MUSHROOMS
    );
    assert_eq!(
        random_tree_prefab(1, false, false, true, false),
        "clearing/tree/branchy1"
    );
    // deny_load / halloween 且 bit==0 → naked1。
    assert_eq!(random_tree_prefab(0, true, false, false, false), TREE_PREFAB_NAKED[0]);
    assert_eq!(random_tree_prefab(0, false, true, false, false), TREE_PREFAB_NAKED[0]);
    // bit==1 绕过上述两门控（drag 仍生效）。
    assert_eq!(
        random_tree_prefab(1, true, true, false, true),
        TREE_PREFAB_AUTUMN[1]
    );
    // 表本身（0x1430351d0 区直读，长度 22/29/26/39）。
    assert_eq!(TREE_PREFAB_BRANCHY[0].len(), 22);
    assert_eq!(TREE_PREFAB_BRANCHY[1].len(), 22);
    assert_eq!(TREE_PREFAB_AUTUMN[0].len(), 29);
    assert_eq!(TREE_PREFAB_AUTUMN[1].len(), 29);
    assert_eq!(TREE_PREFAB_NAKED[0].len(), 26);
    assert_eq!(TREE_PREFAB_NAKED[1].len(), 26);
    assert_eq!(TREE_PREFAB_NO_MUSHROOMS.len(), 39);
}

/// `time_continuous_lerp` 逐语句公式：`a + (b-a)*(1-expf(-delta*rate))`。
/// 期望值为手算：`1-e^{-1} = 0.63212056`（delta=0.2, rate=5）。
#[test]
fn time_continuous_lerp_native_formula() {
    assert_eq!(time_continuous_lerp(3.0, 3.0, 0.5, TREE_BEND_RATE), 3.0);
    assert_eq!(time_continuous_lerp(2.0, 9.0, 0.0, TREE_BEND_RATE), 2.0);
    let one_tau = time_continuous_lerp(0.0, 1.0, 0.2, 5.0);
    assert!(
        (one_tau - 0.632_120_56).abs() < 1e-6,
        "1-exp(-1)={one_tau}"
    );
    // 长时间后收敛到 b 的邻域。
    let settled = time_continuous_lerp(0.0, 1.0, 10.0, 5.0);
    assert!((settled - 1.0).abs() < 1e-6);
    // 反向（delta<0）：-delta*rate=+1 → 1-exp(1) = -1.7182818。
    let backward = time_continuous_lerp(0.0, 1.0, -0.2, 5.0);
    assert!(
        (backward - (1.0 - std::f32::consts::E)).abs() < 1e-5,
        "{backward}"
    );
    // rate 与位移上限同源 5.0。
    assert_eq!(TREE_BEND_RATE, TREE_BEND_STEP_MAX);
    assert_eq!(TREE_BEND_RATE, 5.0);
    assert_eq!(TREE_BEND_DIST_THRESHOLD, 0.1);
}

/// 水下重映射（字节直读 `y / -2.5 * -1.2`）：y<0 分支与 y>=0 原样。
#[test]
fn water_remap_native_constants() {
    assert_eq!(WATER_REMAP_DIVISOR, -2.5);
    assert_eq!(WATER_REMAP_TARGET, -1.2);
    assert_eq!(remap_pos_y_if_in_water(2.5), 2.5);
    assert_eq!(remap_pos_y_if_in_water(0.0), 0.0);
    assert_eq!(remap_pos_y_if_in_water(-2.5), -1.2);
    assert_eq!(remap_pos_y_if_in_water(-5.0), -2.4);
    let y = -1.0_f32;
    let expected = (y / -2.5) * -1.2 + (1.0 - (y / -2.5)) * 0.0;
    assert_eq!(remap_pos_y_if_in_water(y), expected);
}

/// 树放置 scale/rotation 原生区间（place_tree 0x142269970 判等分支）：
/// `scale = r*0.9 + (1-r)*0.5 ∈ [0.5,0.9]`，`rotation = r*360 + (1-r)*0 ∈ [0,360)`。
#[test]
fn place_tree_scale_rotation_endpoints() {
    assert_eq!(TREE_SCALE_MIN, 0.5);
    assert_eq!(TREE_SCALE_MAX, 0.9);
    assert_eq!(TREE_ROTATION_MAX, 360.0);
    for r in [0.0_f32, 0.25, 0.5, 0.75, 1.0] {
        let scale = r * TREE_SCALE_MAX + (1.0 - r) * TREE_SCALE_MIN;
        assert!((TREE_SCALE_MIN..=TREE_SCALE_MAX).contains(&scale));
        let rotation = r * TREE_ROTATION_MAX + (1.0 - r) * 0.0;
        assert!((0.0..=TREE_ROTATION_MAX).contains(&rotation));
    }
    assert_eq!(0.0_f32 * TREE_SCALE_MAX + 1.0 * TREE_SCALE_MIN, 0.5);
    assert_eq!(1.0_f32 * TREE_SCALE_MAX + 0.0 * TREE_SCALE_MIN, 0.9);
}

/// despawn / recalc 的原生边界常量（判等 verdict 的阈值面）。
#[test]
fn despawn_and_boundary_constants() {
    assert_eq!(DESPAWN_ALPHA_THRESHOLD, 0.01);
    assert_eq!(WORLD_BOUND, 65.0);
    // 二进制 `<=0.01 && !=0.01` ≡ 严格 `<0.01`：边界 0.01 与 NaN 均不 despawn。
    let alpha_at_boundary = 0.01_f32;
    assert!(!(alpha_at_boundary < DESPAWN_ALPHA_THRESHOLD));
    assert!(!(f32::NAN < DESPAWN_ALPHA_THRESHOLD));
    // recalc 门控 `ABS(x) <= 65 && ABS(z) <= 65` 含边界、NaN 为假。
    assert!(65.0_f32.abs() <= WORLD_BOUND);
    assert!(!(65.0_f32.next_up().abs() <= WORLD_BOUND));
    assert!(!(f32::NAN.abs() <= WORLD_BOUND));
}

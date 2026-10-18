//! 已判等函数 / 原生常量的数值断言（tester-numeric-assertions，2026-10-17）。
//!
//! 证据：
//! - 矩形 `construct_timberframe_rectangle` 0x141a41ba0：`reports/state/timberframe/REVIEW.md`
//!   「结构/常量/相位/计数判等」——等距除数 1.51（0x142ebc300）、截面 0.2/0.18、
//!   屋脊周长阈值 0.5、`orientation_mat3`（0x140c99570）列序判等。
//! - 圆形 `construct_timberframe_circle` 0x1421b67a0：REVIEW「`max(6,⌊perimeter/1.5⌋)`、
//!   `u=i/(count-1)`、截面 0.2 判等」——1.5（0x1429258d0）、下限 6。
//! - 音频 `timberframe_beam_count_change_audio` 0x1421b6610：REVIEW「判等（主体）」——
//!   `(frame-last_frame==1) && (last_count!=count)`、kind=6、位置 x0y(center)、
//!   last_frame/last_count 回写。此处以 World 调度验证触发门与载荷；帧读面沿用机器码
//!   裸读 +0x2c（与实现同一读面；GameClock 布局分歧见 RESTORE FIX-3）。
//!
//! 构造系统的逐梁写路径未判等（WoodenPlankSsbo/RenderInstance 等 GAP），本测试不触碰。

use bevy_ecs::prelude::*;
use glam::{Mat3, Vec2, Vec3};

use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::time::GameTime;
use country_core::systems::audio::PlayTimberframeSound;
use timberframe::timberframe_beam_count_change_audio::{
    TimberframeBeamCount, timberframe_beam_count_change_audio,
};
use timberframe::timberframe_circle::{
    BEAM_SECTION as CIRCLE_BEAM_SECTION, CIRCLE_COUNT_DIVISOR, MIN_CIRCLE_SEGMENTS,
};
use timberframe::timberframe_rectangle::{
    BEAM_SECTION, BEAM_SECTION_MID, EDGE_DIST_DIVISOR, RIDGE_MIN_PERIMETER, orientation_mat3,
};

/// 圆形梁数原生常量：除数 1.5、下限 6、截面 0.2。
/// 边界值按实现公式 `max(6, (perimeter/1.5) as usize)` 手算。
#[test]
fn circle_count_native_constants_and_boundaries() {
    assert_eq!(CIRCLE_COUNT_DIVISOR, 1.5);
    assert_eq!(MIN_CIRCLE_SEGMENTS, 6);
    assert_eq!(CIRCLE_BEAM_SECTION, 0.2);

    let count = |perimeter: f32| ((perimeter / CIRCLE_COUNT_DIVISOR) as usize).max(MIN_CIRCLE_SEGMENTS);
    // 30/1.5 = 20（高于下限）；12/1.5 = 8；9/1.5 = 6（恰为下限）。
    assert_eq!(count(30.0), 20);
    assert_eq!(count(12.0), 8);
    assert_eq!(count(9.0), 6);
    // 7.5/1.5 = 5 → 提高到下限 6；3.0/1.5 = 2 → 6。
    assert_eq!(count(7.5), 6);
    assert_eq!(count(3.0), 6);
    // 20.25/1.5 = 13.5 → trunc 13。
    assert_eq!(count(20.25), 13);
}

/// 矩形等距梁：`n = ⌊d/1.51⌋`，仅 `1 < n` 时插值 `n-1` 根（t=i/n）。
/// 截面：垂直/裙/屋脊 0.2，中段插值 0.18；屋脊补梁阈值 perimeter>0.5。
#[test]
fn rectangle_spacing_and_section_constants() {
    assert_eq!(EDGE_DIST_DIVISOR, 1.51);
    assert_eq!(BEAM_SECTION, 0.2);
    // 原生立即数 0x3e3851eb；不得再放宽为相邻的 Rust 0.18 字面量。
    let native_mid = f32::from_bits(0x3e3851eb);
    assert_eq!(BEAM_SECTION_MID.to_bits(), native_mid.to_bits());
    assert_eq!(RIDGE_MIN_PERIMETER, 0.5);

    let n = |d: f32| (d / EDGE_DIST_DIVISOR) as i64;
    // 内点值（避开 1.51 的二进制表示边界）：4.6→3、3.03→2、1.60→1、1.40→0。
    assert_eq!(n(4.6), 3);
    assert_eq!(n(3.03), 2);
    assert_eq!(n(1.60), 1);
    assert_eq!(n(1.40), 0);
    // 插入根数 = n-1（n>1 才有）。
    assert_eq!((1..4_i64).count(), 3); // n=4 → i∈{1,2,3}
    // 屋脊补梁门控：严格 `perimeter > 0.5`。
    assert!(0.500_000_1_f32 > RIDGE_MIN_PERIMETER);
    assert!(!(0.5_f32 > RIDGE_MIN_PERIMETER));
    assert!(!(f32::NAN > RIDGE_MIN_PERIMETER));
}

/// `orientation_mat3`（0x140c99570）列序判等：
/// `Mat3::from_cols((axis.x,0,axis.y), Y, (-axis.y,0,axis.x))`。
#[test]
fn orientation_mat3_column_order() {
    assert_eq!(orientation_mat3(Vec2::X), Mat3::IDENTITY);
    assert_eq!(
        orientation_mat3(Vec2::Y),
        Mat3::from_cols(Vec3::new(0.0, 0.0, 1.0), Vec3::Y, Vec3::new(-1.0, 0.0, 0.0))
    );
    assert_eq!(
        orientation_mat3(Vec2::new(0.6, 0.8)),
        Mat3::from_cols(
            Vec3::new(0.6, 0.0, 0.8),
            Vec3::Y,
            Vec3::new(-0.8, 0.0, 0.6)
        )
    );
}

// ---------------------------------------------------------------------------
// 音频触发（World 调度；不 mock 纯函数，直接跑已判等系统）
// ---------------------------------------------------------------------------

fn game_time_with_frame(frame: i32) -> GameTime {
    let mut time = GameTime::default();
    // 机器码读面 `*(int*)(GameTime+0x2c)`（与实现同一读面；见 RESTORE FIX-3）。
    unsafe {
        let base = core::ptr::addr_of_mut!(time).cast::<u8>();
        core::ptr::write_unaligned(base.add(0x2c).cast::<i32>(), frame);
    }
    time
}

fn run_audio(
    beams: TimberframeBeamCount,
    frame: i32,
    event: bool,
) -> (TimberframeBeamCount, usize, Vec<PlayTimberframeSound>) {
    let mut world = World::new();
    world.init_resource::<Events<ConstructTimberframeCmd>>();
    world.init_resource::<Events<PlayTimberframeSound>>();
    world.insert_resource(beams);
    world.insert_resource(game_time_with_frame(frame));
    if event {
        world
            .resource_mut::<Events<ConstructTimberframeCmd>>()
            .send(ConstructTimberframeCmd::default());
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(timberframe_beam_count_change_audio);
    schedule.run(&mut world);

    let events: Vec<_> = world
        .resource::<Events<PlayTimberframeSound>>()
        .iter_current_update_events()
        .copied()
        .collect();
    let output = *world.resource::<TimberframeBeamCount>();
    (output, events.len(), events)
}

/// 触发门逐项：无事件不触发且不回写；Δframe==1 且 last_count!=count 才触发；
/// kind=6；位置 = x0y(center)（空形状 → 原点）；回写 last_frame=frame、last_count=count。
#[test]
fn audio_trigger_gate_and_payload() {
    // 无事件：早退，last_frame 保持。
    let (out, len, _) = run_audio(
        TimberframeBeamCount { last_count: 3, count: 5, last_frame: 9 },
        10,
        false,
    );
    assert_eq!(len, 0);
    assert_eq!(out.last_frame, 9);
    assert_eq!(out.last_count, 3);

    // Δframe=1 且计数变化 → 触发一次，kind=6，位置 x0y(center)=原点。
    let (out, len, events) = run_audio(
        TimberframeBeamCount { last_count: 3, count: 5, last_frame: 9 },
        10,
        true,
    );
    assert_eq!(len, 1);
    assert_eq!(events[0].kind, 6);
    assert_eq!(events[0].position, Vec3::ZERO);
    assert_eq!(out.last_frame, 10);
    assert_eq!(out.last_count, 5);

    // Δframe=2 → 不触发，但仍回写。
    let (out, len, _) = run_audio(
        TimberframeBeamCount { last_count: 3, count: 5, last_frame: 8 },
        10,
        true,
    );
    assert_eq!(len, 0);
    assert_eq!(out.last_frame, 10);
    assert_eq!(out.last_count, 5);

    // last_count == count → 不触发。
    let (_, len, _) = run_audio(
        TimberframeBeamCount { last_count: 5, count: 5, last_frame: 9 },
        10,
        true,
    );
    assert_eq!(len, 0);

    // 帧计数回绕：last_frame=u32::MAX as i32，frame=0 → wrapping_sub == 1 → 触发。
    let (_, len, events) = run_audio(
        TimberframeBeamCount { last_count: 0, count: 7, last_frame: -1 },
        0,
        true,
    );
    assert_eq!(len, 1);
    assert_eq!(events[0].kind, 6);
}

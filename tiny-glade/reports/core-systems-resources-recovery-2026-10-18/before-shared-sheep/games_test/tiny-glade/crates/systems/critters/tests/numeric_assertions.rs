//! 已判等系统 / 原生阈值的数值断言（tester-numeric-assertions，2026-10-17）。
//!
//! 证据：
//! - `sheep_walk_animation` 0x14231a630：`reports/wave2/critters/RESTORE.md` +
//!   `reports/eco/critters/REVIEW.md` verdict **判等（handler 逐语句）**：
//!   - 速度状态机阈值 **0.2**（0x3e4ccccd）：`speed<=0.2` → state1 转 2、progress=0；
//!     `speed>0.2` → 非 1 转 1、progress=speed；
//!   - walk handler 帧索引 `(idx + n*20 + 3) mod 30`，`accum=fmodf(accum+dt,1.25)`、
//!     `idx=clamp_f32_to_i64(accum*24.0)`（24.0=0x142934600，1.25=0x142a8d600）；
//!   - 过渡 handler `progress += dt/0.1`，`progress>=1.0` → state=3。
//! - `Sheep::being_petted` 0x14089d4c0：`amplitude > 0.01`（0x3c23d70a）。
//!
//! 本测试直接调度已判等系统（World/Schedule），不 mock 内部分支。

use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::resources::time::GameTime;
use system_critters::components::{Sheep, Transform};
use system_critters::sheep::sheep_walk_animation::{
    SheepWalkRecord, sheep_walk_animation,
};

fn sheep(pet_target: u64, amplitude: f32) -> Sheep {
    Sheep {
        behavior_state: 0,
        goal_timer: 0.0,
        pet_phase: 0,
        pet_progress: 0.0,
        pet_target,
        flag_30: 0,
        shared_timers: [0.0; 2],
        amplitude,
        control_flag: 0,
    }
}

fn run_sheep(
    mut record: SheepWalkRecord,
    last_pos: Vec3,
    dt: f32,
    pet_target: u64,
    amplitude: f32,
) -> SheepWalkRecord {
    record.last_pos = last_pos;
    let mut world = World::new();
    world.insert_resource(GameTime {
        delta: dt,
        ..Default::default()
    });
    let entity = world
        .spawn((
            Transform::default(), // translation = (0,0,0)
            sheep(pet_target, amplitude),
            record,
        ))
        .id();
    let mut schedule = Schedule::default();
    schedule.add_systems(sheep_walk_animation);
    schedule.run(&mut world);
    *world.get::<SheepWalkRecord>(entity).unwrap()
}

fn record(state: u32, progress: f32, accum: f32) -> SheepWalkRecord {
    SheepWalkRecord {
        state,
        progress,
        frame_index: 0,
        speed: 0.0,
        last_pos: Vec3::ZERO,
        accum,
    }
}

/// 速度阈值 0.2（含等号）与状态机四分支；being_petted 跳过动画 handler 以隔离状态机。
#[test]
fn speed_threshold_point_two_state_machine() {
    let petted = 0.02; // > 0.01 → handler 跳过。

    // speed = 0.2（含边界）：state 1 → 2，progress 归零。
    let out = run_sheep(record(1, 0.7, 0.0), Vec3::new(0.2, 0.0, 0.0), 1.0, 0, petted);
    assert_eq!(out.speed, 0.2);
    assert_eq!(out.state, 2);
    assert_eq!(out.progress, 0.0);

    // speed = 0.2001（>0.2）：state 0 → 1，progress = speed。
    let out = run_sheep(
        record(0, 0.3, 0.0),
        Vec3::new(0.2001, 0.0, 0.0),
        1.0,
        0,
        petted,
    );
    assert!(out.speed > 0.2);
    assert_eq!(out.state, 1);
    assert_eq!(out.progress, out.speed);

    // speed > 0.2 且 state 已为 1：保持 1，progress 不变。
    let out = run_sheep(record(1, 0.7, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.1, 0, petted);
    assert_eq!(out.state, 1);
    assert_eq!(out.progress, 0.7);

    // speed <= 0.2 且 state 非 1：状态与 progress 均保持。
    let out = run_sheep(record(2, 0.4, 0.0), Vec3::ZERO, 0.1, 0, petted);
    assert_eq!(out.state, 2);
    assert_eq!(out.progress, 0.4);
}

/// walk handler：`(idx + n*20 + 3) mod 30`，`idx=(fmod(accum+dt,1.25)*24) as i64`。
#[test]
fn walk_handler_frame_index_mod_30() {
    // state=1 且 speed>0.2 → handler 执行。
    // accum 0.1、dt 0.1 → 0.2；idx=(0.2*24)=4；frame=(4+0+3)%30=7。
    let out = run_sheep(record(1, 0.0, 0.1), Vec3::new(1.0, 0.0, 0.0), 0.1, 0, 0.0);
    assert_eq!(out.accum, 0.2_f32 % 1.25);
    assert_eq!(out.frame_index, 7);

    // 同 accum，n=2 → (4 + 40 + 3) % 30 = 17。
    let out = run_sheep(record(1, 0.0, 0.1), Vec3::new(1.0, 0.0, 0.0), 0.1, 2, 0.0);
    assert_eq!(out.frame_index, 17);

    // n=5 → (4 + 100 + 3) % 30 = 107 % 30 = 17。
    let out = run_sheep(record(1, 0.0, 0.1), Vec3::new(1.0, 0.0, 0.0), 0.1, 5, 0.0);
    assert_eq!(out.frame_index, 17);

    // fmod 回绕：accum 1.2 + dt 0.1 = 1.3 → 1.3 % 1.25 = 0.05；idx=1 → frame=4。
    let out = run_sheep(record(1, 0.0, 1.2), Vec3::new(1.0, 0.0, 0.0), 0.1, 0, 0.0);
    assert!((out.accum - 0.05).abs() < 1e-6, "accum={}", out.accum);
    assert_eq!(out.frame_index, 4);

    // accum 到 1.25 边界：1.2 + 0.05 = 1.25 → fmod 0 → idx 0 → frame 3。
    let out = run_sheep(record(1, 0.0, 1.2), Vec3::new(1.0, 0.0, 0.0), 0.05, 0, 0.0);
    assert_eq!(out.accum, 0.0);
    assert_eq!(out.frame_index, 3);
}

/// 过渡 handler（state 2，speed<=0.2）：`progress += dt/0.1`；
/// `progress>=1.0` → state=3（不写 progress），否则 state=2 + 写 progress。
#[test]
fn transition_handler_progress_threshold() {
    let out = run_sheep(record(2, 0.0, 0.0), Vec3::ZERO, 0.05, 0, 0.0);
    assert_eq!(out.state, 2);
    assert!((out.progress - 0.5).abs() < 1e-6, "progress={}", out.progress);
    // 同帧帧索引：accum 0.05 → idx=(0.05*24)=1 → frame=4。
    assert_eq!(out.frame_index, 4);

    let out = run_sheep(record(2, 0.0, 0.0), Vec3::ZERO, 0.1, 0, 0.0);
    assert_eq!(out.state, 3);
    assert_eq!(out.progress, 0.0); // 到 1.0 时不写 progress。
}

/// `being_petted` 阈值 `> 0.01`（0x3c23d70a）：恰为 0.01 不跳过，下一 f32 跳过。
#[test]
fn being_petted_threshold_point_zero_one() {
    let threshold = f32::from_bits(0x3c23d70a);
    assert_eq!(threshold, 0.01);
    // amplitude == 0.01 → 不视为抚摸 → handler 执行（frame 7）。
    let out = run_sheep(
        record(1, 0.0, 0.1),
        Vec3::new(1.0, 0.0, 0.0),
        0.1,
        0,
        threshold,
    );
    assert_eq!(out.frame_index, 7);
    // amplitude = 0.01 的下一 f32 → 抚摸 → handler 跳过（frame/accum 保持）。
    let out = run_sheep(
        record(1, 0.0, 0.1),
        Vec3::new(1.0, 0.0, 0.0),
        0.1,
        0,
        threshold.next_up(),
    );
    assert_eq!(out.accum, 0.1);
    assert_eq!(out.frame_index, 0);
}

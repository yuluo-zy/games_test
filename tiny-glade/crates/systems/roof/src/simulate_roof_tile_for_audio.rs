//! 屋顶瓦片音频模拟（simulate_roof_tile_for_audio.rs）。
//!
//! `simulate_roof_tile_for_audio`（0x1421a7090，1654B/357）。参数面（mangled 名 O）：
//! `void(Query<(&mut RoofTileSimulation, &Roof)>, EventWriter<RoofSound>,
//!  Res<GameTime>, Res<PublicWalls>)`。
//!
//! 机器码（fresh decomp-0x1421a7090）：
//! - `dt==0`（含无序比较，仅精确 0 且非 NaN）直接返回；
//! - 逐 roof（wall→roof 哈希表两段查询）读 `RoofTileSimulation` 单记录
//!   （stride 0x18：`[0]=当前位置`、`[2]=上帧高度`、`[3]=角度`、`[4]=累加器`、
//!   `[5]=过阈标志`），`Roof::is_flat` 门控（平顶跳过）；
//! - 每记录：`angle' = dt·−45.126003 + angle`；`mixed = dt·angle' + mix`（mix 掩码 SELECT：
//!   `mix = smaller ? prev_h : (cur>=0 ? prev_h : (over ? prev_h : target))`）；
//!   `if mixed <= target`：`15.0 < angle'` → `clamp::do_panic`；`mixed'=NaN?target:max(target,mixed)`；
//!   `angle = min(max(angle', (cur/dt)·0.166), 15.0)`；
//!   `decay = expf(dt·−13.300004)`；`new_h = (1−decay)·target + decay·mixed'`；
//! - `|new_h−target|` 与 `0.1`/`0.01` 阈值比较：首跨 `0.1`（`[5]==0`）发
//!   RoofSound kind 5；回落 `≤0.1`（`[5]!=0`）且累加器 `>0.01` 发 kind 4
//!   （`(accum−0.01)/0.19` clamp[0,1]）；`angle'<0`（最终 angle）时累加器 `+= dt`；
//!   `[5]` 更新、每记录写 changed tick。
//!
//! ## wave3 修正（2026-10-17，restorer-wave3-timber-roof）
//! 旧实现以**旧角度** `angle` 判 `15.0 < angle` panic、做角度钳制、判 `angle<0` 累加器——
//! 与解压不符。解压统一以 `angle'`（= dt·−45.126003 + 旧角）判 panic、钳制，并以钳制后的
//! **最终角**判 `angle<0` 累加器门控。本实现按解压修正。
//!
//! I 级边界：core `RoofTileSimulation` 为单 24B 记录（原为 stride-0x18 记录数组）；`RoofSound`
//! 载荷字段属 core 重建面；`GameClock.delta` 在 core 为 0x24（原读 +0x28，系统面用符号读取不受影响）。
use country_core::resources::{
    roofs::{Roof, RoofTileSimulation, commands::RoofSound},
    time::GameTime,
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;

/// B 0x1421a7090。
pub fn simulate_roof_tile_for_audio(
    mut tiles: Query<(ComponentWrite<RoofTileSimulation>, &Roof)>,
    mut sounds: EventWriter<RoofSound>,
    time: Res<GameTime>,
    walls: Res<PublicWalls>,
) {
    let dt = time.delta;
    if dt == 0.0 {
        return;
    }
    for (mut simulation, roof) in &mut tiles {
        if roof.is_flat() {
            continue;
        }
        let wall = roof.wall_id();
        let Some(entity) = walls.roof_entity(wall) else {
            continue;
        };
        let pivot = roof.pivot_3d().to_array();
        let target = roof.floor_y();
        let mut bytes = *simulation.observed();
        let cur = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let height = f32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let angle = f32::from_le_bytes(bytes[12..16].try_into().unwrap());

        // 角度推进 + 高度混合（SSE 分支语义，逐项还原 decomp-0x1421a7090）：
        // uVar22 = cur/dt <= -120；uVar23 = target-cur < height。
        let over = cur / dt <= -120.0;
        let smaller = target - cur < height;
        // `mix = smaller ? prev_h : (cur>=0 ? prev_h : (over ? prev_h : target))`。
        let mix = if smaller || cur >= 0.0 || over {
            height
        } else {
            target
        };
        // O：`angle' = dt·−45.126003 + angle`；`mixed = dt·angle' + mix`。
        let angle_prime = dt * -45.126003 + angle;
        let mut mixed = dt * angle_prime + mix;
        let mut final_angle = angle_prime;

        if mixed <= target {
            // O：角度上界 15.0：`15.0 < angle'` → `clamp::do_panic::runtime`（用**新角**，非旧角）。
            if 15.0 < angle_prime {
                panic!("f32 clamp: 15.0 < angle");
            }
            // new_height 上界：`max(target, mixed)`（NaN → target）。
            mixed = if mixed.is_nan() { target } else { mixed.max(target) };
            // O：`angle = min(max(angle', (cur/dt)·0.166), 15.0)`（用**新角**，非旧角）。
            let clamped_low = (cur / dt) * 0.166;
            let angled = if angle_prime > clamped_low {
                angle_prime
            } else {
                clamped_low
            };
            final_angle = if angled <= 15.0 { angled } else { 15.0 };
        }

        let decay = (dt * -13.300004).exp();
        let new_height = (1.0 - decay) * target + decay * mixed;
        bytes[8..12].copy_from_slice(&new_height.to_le_bytes());
        bytes[12..16].copy_from_slice(&final_angle.to_le_bytes());
        let diff = (new_height - target).abs();
        let flag = bytes[20];

        let new_flag;
        if flag == 0 {
            if diff < 0.1 {
                bytes[16..20].copy_from_slice(&0.0f32.to_le_bytes());
                new_flag = 0;
            } else {
                // 首跨 0.1 → 滑落音效 kind 5。
                sounds.write(RoofSound {
                    entity,
                    wall,
                    pivot,
                    kind: 5,
                    flat_surface: None,
                });
                new_flag = 1;
            }
        } else if diff <= 0.1 {
            let accum = f32::from_le_bytes(bytes[16..20].try_into().unwrap());
            if 0.01 < accum {
                // 回落触地音效 kind 4：(accum−0.01)/0.19 clamp[0,1]。
                let mut decay_01 = (accum - 0.01) / 0.19;
                decay_01 = if decay_01 < 0.0 { 0.0 } else { decay_01 };
                decay_01 = if decay_01 <= 1.0 { decay_01 } else { 1.0 };
                let _ = decay_01; // 载荷值（core RoofSound 重建面外，I 级登记）
                sounds.write(RoofSound {
                    entity,
                    wall,
                    pivot,
                    kind: 4,
                    flat_surface: None,
                });
            }
            new_flag = 0;
        } else {
            new_flag = 1;
            // O：`if (final_angle < 0.0) { accum += dt; }`（**最终角**，非旧角）。
            if final_angle < 0.0 {
                let accum = f32::from_le_bytes(bytes[16..20].try_into().unwrap());
                bytes[16..20].copy_from_slice(&(dt + accum).to_le_bytes());
            }
        }
        bytes[20] = new_flag;
        simulation.replace_observed(bytes);
    }
}

use country_core::render::extract::ComponentWrite;

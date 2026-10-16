//! B `crates/systems/decorator/src/audio/op_preview_audio.rs` — 放置预览音。
//!
//! 证据（fresh RE-MCP，decomp-140822e00.txt，O 结构）：
//! - `op_preview_audio` 0x140822e00：遍历存储记录（双 0x90B 段；skip 记录
//!   tag==2 / type '?'）→ 按 DecoratorType（+0x81 区域 type 字节）分派 kind
//!   （跳表 0x142a91168/0x142a91198）：0x38/0x39/0x3a/0x3b → kind 7（flag =
//!   subtype==2/3 变体）、0x3c → 0xc、0x3d → subtype 细分 {0x1c,0x1d}、
//!   0x3e → subtype==9 ? 0xc : 0x24、0x3f+ → 0x16/0x24 …；
//! - 屋顶木地板特殊路径（O）：type 0x42（'?' 判 `bVar26=b-0x38; 7<→6` 含
//!   0x42）且 dst 非楼梯 → `PublicWalls::roof_component`（0x140a923a0）→
//!   `Roof::is_flat`（0x1408e3380）→ `FlatRoofStyle::is_wooden`（0x1408e31a0）
//!   → 事件 kind 6（DecoratorMoveWoodenFloorTEMP 循环，章节 decorator_audio）；
//! - 事件经 `FUN_14125e4f0`（= Vec<0x28B> push，O 全解）入 `DecoratorAudioEvents`
//!   帧队列；帧状态 {5,0}/{6,0}/{7,0} 写回（预览去重键 I）。
//!
//! `move_sound`（行 183 内联，I）：事件位置回填（记录 +0x2c/+0x34 打包位置 →
//! 事件 position 字段）。

use crate::audio::{DecoratorAudioEvent, DecoratorAudioEvents};

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::{FlatRoofStyle, Roof};
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use glam::Vec3;

/// `op_preview_audio`（0x140822e00，O 结构）：放置预览音收集。
/// 遍历存储记录；按类型分派 kind；屋顶木地板特殊事件；全部入帧队列。
pub fn op_preview_audio(
    mut audio_events: ResMut<DecoratorAudioEvents>,
    storage: Res<DecoratorStorage>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    // 原第 5/6 参数（kind 表资源 / 附加资源 ResMut tick@+0x1c）：身份待核，
    // RESTORE.md 挂账登记。kind 表（type → kind 常量数组）按 0x140822e00 跳表
    // 固化于 [`kind_for_type`]。
) {
    audio_events.new_frame();

    for (wall, decorators) in storage.iter_walls() {
        for (id, info) in decorators.decorators.iter() {
            let mut event = preview_event_for(info, wall, &walls, &roofs);
            event.decorator_id = country_core::resources::walls::decorator::DecoratorId::new(*id);
            event.position = position_from_info(info);
            audio_events.push_event(event);
        }
    }
    for decorators in storage.iter_stair_decorators() {
        for (id, info) in decorators.decorators.iter() {
            let mut event = preview_event_for(info, WallId(u64::MAX), &walls, &roofs);
            event.decorator_id = country_core::resources::walls::decorator::DecoratorId::new(*id);
            event.position = position_from_info(info);
            audio_events.push_event(event);
        }
    }
}

/// 单记录 → 预览事件（O：skip 规则 + type 分派 + 木地板特殊路径）。
fn preview_event_for(
    info: &DecoratorInfo,
    wall: WallId,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
) -> DecoratorAudioEvent {
    let decorator_type = info.decorator_type();
    let subtype = info.u45;
    let mut event = DecoratorAudioEvent::new(0, 0, 0, subtype);
    match info.dst_tag() {
        // 楼梯附着：kind 8（0x19 若 type 0x3d；I）。
        2 => {
            event.tag = 0x8;
            event.kind = if decorator_type
                == country_core::resources::walls::decorator::DecoratorType(0x3d)
            {
                0x19
            } else {
                8
            };
        }
        // 屋顶附着（tag 1）：木地板特殊路径（O；B `PublicWalls::roof_component`
        // 0x140a923a0 = roof_entity + Roof 组件取回，core 未恢复访问器 → 本地
        // 组合，RESTORE.md 登记）。
        1 => {
            let roof = walls.roof_entity(wall).and_then(|e| roofs.get(e).ok());
            if let Some(roof) = roof {
                if roof.is_flat() && roof_flat_style_is_wooden(roof) {
                    event.tag = 6;
                    event.kind = 6; // DecoratorMoveWoodenFloorTEMP
                    return event;
                }
            }
            event = kind_for_type(decorator_type, subtype);
        }
        // 墙附着（tag 0）：type 分派。
        _ => {
            event = kind_for_type(decorator_type, subtype);
        }
    }
    event
}

/// type → kind 分派（O：0x140822e00 跳表 0x142a91168/0x142a91198 槽位）。
fn kind_for_type(decorator_type: country_core::resources::walls::decorator::DecoratorType, subtype: u8) -> DecoratorAudioEvent {
    let mut event = DecoratorAudioEvent::new(0, 0, 0, subtype);
    match decorator_type.0.wrapping_sub(0x38) {
        0..=3 => {
            // 窗族 + 0x3b：kind 7（flag 变体按 subtype）。
            event.tag = 7;
            event.kind = 7;
            event.flag = if subtype == 2 { 2 } else if subtype == 3 { 3 } else { 0 };
        }
        4 => {
            event.tag = 0xc;
            event.kind = 0xc;
        }
        5 => {
            // 楼梯顶（0x3d）：subtype 细分。
            event.tag = 0x1c;
            event.kind = match subtype.wrapping_sub(2) {
                1 => 0x1d,
                _ => 0x1c,
            };
        }
        6 => {
            // 烟囱（0x3e）：subtype==9 → 0xc，否则 0x24。
            if subtype == 9 {
                event.tag = 0xc;
                event.kind = 0xc;
            } else {
                event.tag = 0x24;
                event.kind = 0x24;
            }
        }
        _ => {
            event.tag = 0x16;
            event.kind = 0x16;
        }
    }
    event
}


/// B `FlatRoofStyle::is_wooden`（0x1408e31a0，O）：平面屋顶风格在 Roof 字节
/// +0x40 区（12B）。`other` 参数 = 原 +0x54 覆盖字节（I：以 false 承载）。
fn roof_flat_style_is_wooden(roof: &Roof) -> bool {
    let bytes = roof.observed();
    let style_bytes: [u8; 12] = bytes
        .get(0x40..0x4c)
        .expect("roof flat style range")
        .try_into()
        .expect("flat style 12 bytes");
    FlatRoofStyle::from_observed(style_bytes).is_wooden(false)
}

/// `move_sound`（行 183 内联，I）：从记录打包位置（+0x2c/+0x34）回填事件位置。
fn position_from_info(info: &DecoratorInfo) -> Vec3 {
    let x = f32::from_bits(info.u24);
    let z = f32::from_bits(info.u28);
    Vec3::new(x, 0.0, z)
}

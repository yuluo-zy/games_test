//! B `crates/systems/decorator/src/audio/op_preview_audio.rs` — 放置预览音。
//!
//! 证据（`evidence/toolchain/decor-trunk/decompiled/140822e00_op_preview_audio.c`，
//! O 结构）：
//!
//! - `op_preview_audio` 0x140822e00 消费 `Events<DecoratorOpEventKind>`（0x90B
//!   记录）：帧首 `new_frame` → 逐事件过滤 type==0x3f / 状态==2 → 屋顶木地板
//!   特殊路径 → 其余按 `DecoratorType`（记录 `+0x81`）分派 kind
//!   （跳表 `0x142a91168` / `0x142a91198`）：
//!   - 0x38/0x39/0x3a → kind 7，flag=(subtype@+0x3a==2)；
//!   - 0x3b → kind 0x16，flag=(subtype==2)；
//!   - 0x3c → kind 0xc，flag=(subtype==3)；
//!   - 0x3d → subtype-2 ∈ {0,2} → 0x1c/flag 1；==1 → 0x1d；其余 0x1c/flag 0；
//!   - slot 6 (0x3e and every out-of-range type) tests the original TYPE
//!     byte against 9, then uses subtype for the flag; it does not test subtype==9.
//! - 屋顶木地板（O）：记录 `+0x42 == 2`（目的是屋顶）→
//!   `PublicWalls::roof_component`（0x140a923a0）→ `Roof::is_flat`
//!   （0x1408e3380）→ `FlatRoofStyle::is_wooden`（0x1408e31a0）→
//!   kind 6（`DecoratorMoveWoodenFloorTEMP` 循环）。
//! - 记录 `+0x60 != 0` 分支（O：`code_r0x140823106`）→ 预览放置 kind 3，
//!   由帧状态去重（`*state != 6` 同型一次）。
//! - 事件经 `EventWriter::write<PlayDecoratorSoundCmd>()` 写出（反编译
//!   4 个 write 站点：木地板两处、常规分派一处、kind 3 一处）。
//!
//! U（登记）：反编译开头另有一个事件流探针（0x90B 记录），在非空时提前
//! 返回/清游标；其事件类型（候选 `AudioCursorMoved`）在预览路径内未出现
//! 直接读写，本实现不引入该资源，保持为显式缺口。`Roof` 的
//! `roof_flat_style_is_wooden` 在原码中是 `FlatRoofStyle::is_wooden` 的
//! `other` 参数（`+0x54` 覆盖字节），本实现按 false 承载（与既有章节一致）。

use crate::audio::{AudioEventKey, DecoratorAudioEvents, DecoratorOpEventView};

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::{FlatRoofStyle, Roof};
use country_core::resources::walls::decorator::{DecoratorOpEventKind, DecoratorType};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::audio::PlayDecoratorSoundCmd;

/// type → (kind, flag)，O：跳表 `0x142a91168/0x142a91198`。
pub fn preview_kind(decorator_type: DecoratorType, subtype: u8) -> (u8, u8) {
    let slot = decorator_type.0.wrapping_sub(0x38);
    let slot = if slot < 8 { slot } else { 6 };
    match slot {
        // O：窗族 0x38..0x3a。
        0..=2 => (7, u8::from(subtype == 2)),
        // O：0x3b 保持缺省 kind。
        3 => (0x16, u8::from(subtype == 2)),
        // O：0x3c。
        4 => (0xc, u8::from(subtype == 3)),
        // O：0x3d。
        5 => match subtype.wrapping_sub(2) {
            0 | 2 => (0x1c, 1),
            1 => (0x1d, 0),
            _ => (0x1c, 0),
        },
        // B1408234b2 tests DL (the original decorator type), not CL (subtype).
        // Out-of-range tags deliberately select this same slot at 1408233be.
        6 => {
            if decorator_type.0 == 9 {
                (0xc, u8::from(subtype == 3))
            } else {
                (0x24, u8::from(subtype != 3))
            }
        }
        // Slot 7 only; tags outside 0x38..=0x3f already selected slot 6.
        _ => (0x16, 0),
    }
}

/// `op_preview_audio`（0x140822e00，O 结构）：放置预览音收集。
pub fn op_preview_audio(
    mut audio_events: ResMut<DecoratorAudioEvents>,
    mut op_events: EventReader<DecoratorOpEventKind>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut sounds: EventWriter<PlayDecoratorSoundCmd>,
) {
    // O：preview 的 PDB 面没有 `new_frame`（仅 confirm 内联该实例）；去重表
    // 为本次运行的局部 `HashMap<AudioEventKey, _>`。
    let mut seen: std::collections::HashMap<AudioEventKey, ()> = std::collections::HashMap::new();

    for event in op_events.read() {
        let view = DecoratorOpEventView::new(event);
        if !view.is_audio_relevant() {
            continue;
        }
        let decorator_type = view.decorator_type();
        let position = view.position();

        // O：木地板特殊路径（记录 +0x42 == 2 → 目的屋顶）。
        if view.roof_tag() == 2 {
            let wall = WallId(u64::from_le_bytes(view.raw[0x08..0x10].try_into().unwrap()));
            if let Some(roof) = walls.roof_component(wall, |entity| roofs.get(entity).ok()) {
                if roof.is_flat() && roof_flat_style_is_wooden(roof) {
                    // O：kind 6（DecoratorMoveWoodenFloorTEMP 循环）。
                    let key = AudioEventKey {
                        decorator_type,
                        flag: 6,
                    };
                    let cmd = PlayDecoratorSoundCmd::at(6, position);
                    if seen.insert(key, ()).is_none() {
                        sounds.write(cmd);
                    }
                    audio_events.set_state(6, 0);
                    continue;
                }
            }
        }

        // O：+0x60 != 0 → 预览放置 kind 3（帧状态去重）。
        if view.byte_60() != 0 {
            if audio_events.state()[0] != 3 {
                let key = AudioEventKey {
                    decorator_type,
                    flag: 3,
                };
                let cmd = PlayDecoratorSoundCmd::at(3, position);
                if seen.insert(key, ()).is_none() {
                    sounds.write(cmd);
                }
                audio_events.set_state(3, 0);
            }
            continue;
        }

        let (kind, flag) = preview_kind(decorator_type, view.subtype_at_3a());
        let key = AudioEventKey {
            decorator_type,
            flag,
        };
        let cmd = PlayDecoratorSoundCmd::at(kind, position);
        if seen.insert(key, ()).is_none() {
            sounds.write(cmd);
        }
    }
}

/// B `FlatRoofStyle::is_wooden`（0x1408e31a0，O）：平面屋顶风格字节
/// `+0x40..+0x4c`（12B）。`other` 参数 = 原 `+0x54` 覆盖字节（I：false）。
pub fn roof_flat_style_is_wooden(roof: &Roof) -> bool {
    let bytes = roof.observed();
    let style_bytes: [u8; 12] = bytes[0x40..0x4c].try_into().unwrap();
    FlatRoofStyle::from_observed(style_bytes).is_wooden(false)
}

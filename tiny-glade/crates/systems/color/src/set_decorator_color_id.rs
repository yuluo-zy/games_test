//! set_decorator_color_id.rs — 装饰物颜色写入（B 0x1422af0c0，2879B；
//! restorer-state-color 2026-10-17 fresh 反编译重建）。
//!
//! 真实控制流（5 层嵌套事件源，stride 0x18/0x18/0x18/0x10/0x18）：
//! 1. `SetDecoratorColorIdCmd`（外环）：`lookup_decorator_info_mut`（unwrap）→
//!    写 `DecoratorInfo+0x28=色`；`sub=type-0x38`（`>7→6`）；`sub==7`（'?' 楼梯）
//!    → `StairsState::get_all_connected_nodes` 逐节点写 +0x28；custom → 推记录
//!    （tag 由 `0xcf>>sub&1` 判定：默认 10、sub==4→0xd、否则 8）。
//! 2. `SetDecoratorBrightnessCmd`：type=='<'（灯笼）→ `as_lantern` 写 +0x0；
//!    custom → 推记录 tag=**0xe**。
//! 3. `SetLadderColorIdCmd`：type=='?' → 逐连接节点写 `as_stair` 载荷 +0x8。
//! 4. `SetStairRailingCmd`（stride 0x10）：type=='?' → 逐连接节点写 +0x10。
//! 5. `SetMagicColorIdCmd`（最内环）：type=='?' → 逐连接节点写 +0xc。
//! 记录 value 载荷在反编译多为未解出的栈值（U），本实现取事件色/值。

use bevy_ecs::prelude::*;

use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayWallUiSound;

use crate::{
    SetDecoratorBrightnessCmd, SetDecoratorColorIdCmd, SetLadderColorIdCmd, SetMagicColorIdCmd,
    SetStairRailingCmd,
};

/// `set_decorator_color_id` @0x1422af0c0（2879B，重建）。
#[allow(clippy::too_many_arguments)]
pub fn set_decorator_color_id(
    mut storage: ResMut<DecoratorStorage>,
    mut colors: EventReader<SetDecoratorColorIdCmd>,
    mut brightness: EventReader<SetDecoratorBrightnessCmd>,
    mut ladders: EventReader<SetLadderColorIdCmd>,
    mut railings: EventReader<SetStairRailingCmd>,
    mut magic: EventReader<SetMagicColorIdCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    stairs: Res<StairsState>,
    transforms: Res<CachedDecoratorTransforms>,
) {
    // O：层级 1 —— SetDecoratorColorIdCmd（反编译外环，lookup unwrap）。
    for cmd in colors.read() {
        let info = storage
            .lookup_decorator_info_mut(DecoratorId(cmd.id))
            .unwrap();
        info.u28 = cmd.color;
        let mut sub = info.ty.wrapping_sub(0x38);
        if sub > 7 {
            sub = 6;
        }
        // O：sub==7（type '?' 楼梯）→ 逐连接节点写 +0x28。
        if sub == 7 {
            for node in stairs.get_all_connected_nodes(cmd.id) {
                let node_info = storage
                    .lookup_decorator_info_mut(DecoratorId(node))
                    .unwrap();
                node_info.u28 = cmd.color;
            }
        }
        // O：custom != 0（反编译 `event+0x10 != '\0'`，L1 用 !=0）→ tag 判定。
        if cmd.custom != 0 {
            let mut tag = 10u8;
            if (0xcfu8 >> (sub & 0x1f)) & 1 == 0 {
                tag = if sub == 4 { 0xd } else { 8 };
            }
            let position = transforms.position(cmd.id);
            sounds.write(PlayWallUiSound {
                kind: tag,
                selector: 0,
                value: f32::from_bits(cmd.color),
                position,
            });
        }
    }

    // O：层级 2 —— SetDecoratorBrightnessCmd（type '<' 灯笼，写 +0x0）。
    for cmd in brightness.read() {
        if let Some(info) = storage.lookup_decorator_info_mut(DecoratorId(cmd.id)) {
            if info.ty == b'<' {
                let mut extra = info.extra();
                let lantern = extra.as_lantern_mut();
                // O：原生整 4 字节写事件 +0xc 原始位模式到 LanternExtra(f32)；
                // 位模式等价于 `from_bits`（保持逐字节一致）。
                lantern.0 = f32::from_bits(cmd.value);
                info.set_extra(extra);
                if cmd.custom == 1 {
                    let position = transforms.position(cmd.id);
                    sounds.write(PlayWallUiSound {
                        kind: 0xe,
                        selector: 0,
                        value: f32::from_bits(cmd.value),
                        position,
                    });
                }
            }
        }
    }

    // O：层级 3 —— SetLadderColorIdCmd（type '?'，逐连接节点写 stair+0x8）。
    for cmd in ladders.read() {
        if let Some(info) = storage.lookup_decorator_info_mut(DecoratorId(cmd.id)) {
            if info.ty == b'?' {
                for node in stairs.get_all_connected_nodes(cmd.id) {
                    let node_info = storage
                        .lookup_decorator_info_mut(DecoratorId(node))
                        .unwrap();
                    let mut extra = node_info.extra();
                    let stair = extra.as_stair_mut();
                    stair.0[8..12].copy_from_slice(&cmd.value.to_le_bytes());
                    node_info.set_extra(extra);
                }
                if cmd.custom == 1 {
                    let position = transforms.position(cmd.id);
                    sounds.write(PlayWallUiSound {
                        kind: 10,
                        selector: 0,
                        value: f32::from_bits(cmd.value),
                        position,
                    });
                }
            }
        }
    }

    // O：层级 4 —— SetStairRailingCmd（stride 0x10；写 stair+0x10 u8）。
    for cmd in railings.read() {
        if let Some(info) = storage.lookup_decorator_info_mut(DecoratorId(cmd.id)) {
            if info.ty == b'?' {
                for node in stairs.get_all_connected_nodes(cmd.id) {
                    let node_info = storage
                        .lookup_decorator_info_mut(DecoratorId(node))
                        .unwrap();
                    let mut extra = node_info.extra();
                    let stair = extra.as_stair_mut();
                    stair.0[0x10] = cmd.value;
                    node_info.set_extra(extra);
                }
                if cmd.custom == 1 {
                    let position = transforms.position(cmd.id);
                    sounds.write(PlayWallUiSound {
                        kind: 10,
                        selector: 0,
                        value: f32::from_bits(u32::from(cmd.value)),
                        position,
                    });
                }
            }
        }
    }

    // O：层级 5 —— SetMagicColorIdCmd（最内环；写 stair+0xc）。
    for cmd in magic.read() {
        if let Some(info) = storage.lookup_decorator_info_mut(DecoratorId(cmd.id)) {
            if info.ty == b'?' {
                for node in stairs.get_all_connected_nodes(cmd.id) {
                    let node_info = storage
                        .lookup_decorator_info_mut(DecoratorId(node))
                        .unwrap();
                    let mut extra = node_info.extra();
                    let stair = extra.as_stair_mut();
                    stair.0[12..16].copy_from_slice(&cmd.value.to_le_bytes());
                    node_info.set_extra(extra);
                }
                if cmd.custom == 1 {
                    let position = transforms.position(cmd.id);
                    sounds.write(PlayWallUiSound {
                        kind: 10,
                        selector: 0,
                        value: f32::from_bits(cmd.value),
                        position,
                    });
                }
            }
        }
    }
}

//! B `crates/systems/decorator/src/audio/op_confirm_audio.rs` — 装饰操作确认音。
//!
//! 证据（`evidence/toolchain/decor-trunk/decompiled/140822530_op_confirm_audio.c`
//! + `140822cf0_place_sound.c`，O 结构）：
//!
//! - `op_confirm_audio` 0x140822530 消费 `Events<DecoratorOpEventKind>`（0x90B
//!   记录步长）：帧首 `new_frame` → 逐事件按 `+0x81` type / `+0x7c` 状态过滤
//!   （type==0x3f、状态==2 跳过）→ 按事件判别 `+0x00` 分派 → 生成
//!   `PlayDecoratorSoundCmd` 并去重插入 `HashMap<AudioEventKey, _>`。
//! - `place_sound` 0x140822cf0（O）：type 分派 `(type-0x38)` 8 槽跳表
//!   `0x142a910c0`：
//!   - 0x38..0x3a：tag=1、kind=1（0x3a 为 2）、flag 字节=(flag==2)；
//!   - 0x3b：tag=0x11，子类型经 `DerivedDecoratorInfo` 校验 ∈{0x13..0x15}
//!     （否则 panic "internal error: entered unreachable code"），
//!     变体字节 = `0x100 >> ((subtype-0x13 & 3) << 3)`；
//!   - 0x3c：tag=0xb、kind=0x10、flag=(flag==3)；
//!   - 0x3d：子类型跳表（kind 0x1c/0x1d）；
//!   - 0x3e：tag=0x23、kind=0x20、变体字节=type。
//!
//! 记录级分派（O，同一函数 `0x140822530` 内的跳表 `0x142a90ff0`）：
//! - 非特殊判别（dics ∉ {3,4}）：`kind = 4` 缺省；type 槽 3→0x17、4→0x10、
//!   5→0x1e、6→0x20；
//! - disc==3（Change）：`+0x10 != 3` → `place_sound`；否则楼梯簇
//!   `+0x62!=0` → kind 8（type 0x3d 时 0x19）；`+0x65` 不为 0x40 →
//!   kind 8（`+0x65-0x38` 为 6 或 >7 时 0x23）；
//! - disc==4（Remove）：`+0x44==0` → kind 0；否则 `place_sound`。
//!
//! U（登记，不伪造）：`place_sound` case 5（type 0x3d）的间接跳表
//! `0x142a910c0` 未被反编译器展开；本实现按 `op_preview_audio` 同址跳表的
//! 可读分支（subtype-2 ∈ {0,2} → 0x1c/1；==1 → 0x1d）承载，并在
//! `place_sound` 注释中标记。

use crate::audio::{AudioEventKey, DecoratorAudioEvents, DecoratorOpEventView};

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::{
    DecoratorOpEventKind, DecoratorType,
};
use country_core::systems::audio::PlayDecoratorSoundCmd;
use glam::Vec3;

/// `place_sound` 的输出（0x28B 记录中的 `{tag, subtype, flag, kind}` 簇）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaceSound {
    /// 记录 `+0x00`：事件 tag。
    pub tag: u8,
    /// 记录 `+0x01`：子类型/变体字节。
    pub subtype: u8,
    /// 记录 `+0x02`：flag 字节。
    pub flag: u8,
    /// `PlayDecoratorSoundCmd.kind`。
    pub kind: u8,
}

/// `place_sound`（0x140822cf0，O）：按装饰类型/子类型写一条音频事件记录。
///
/// 参数顺序按机器码寄存器面：`(decorator_type, subtype, flag)`。
pub fn place_sound(decorator_type: DecoratorType, subtype: u8, flag: u8) -> PlaceSound {
    let slot = decorator_type.0.wrapping_sub(0x38);
    // O：`uVar1 = 6; if byte(type-0x38) < 8 { uVar1 = type-0x38 }`。
    let slot = if slot < 8 { slot } else { 6 };
    let mut out = PlaceSound {
        tag: 0,
        subtype: 0,
        flag: 0,
        kind: 0,
    };
    match slot {
        // 0x38..0x3a：窗族。
        0..=2 => {
            out.tag = 1;
            out.kind = if slot == 2 { 2 } else { 1 };
            out.subtype = subtype;
            out.flag = u8::from(flag == 2);
        }
        // 0x3b：子类型经 DerivedDecoratorInfo 校验 ∈ {0x13,0x14,0x15}。
        3 => {
            if subtype.wrapping_sub(0x13) > 2 {
                // O：0x142a90fae panic；fat pointer 0x142a90fd8。
                panic!("internal error: entered unreachable code");
            }
            out.tag = 0x11;
            out.subtype = (0x100u32 >> ((subtype.wrapping_sub(0x13) & 3) << 3)) as u8;
            out.kind = 0;
        }
        // 0x3c：tag=0xb、kind=0x10、flag=(flag==3)。
        4 => {
            out.tag = 0xb;
            out.kind = 0x10;
            out.flag = u8::from(flag == 3);
        }
        // 0x3d：子类型跳表（U：表未展开；按 preview 同址可读分支承载）。
        5 => {
            out.tag = match subtype.wrapping_sub(2) {
                1 => 0x1d,
                _ => 0x1c,
            };
            out.kind = out.tag;
            out.flag = match subtype.wrapping_sub(2) {
                0 | 2 => 1,
                _ => 0,
            };
        }
        // 0x3e：tag=0x23、kind=0x20、变体字节=type（O：`in_RCX[1] = in_DL`）。
        6 => {
            out.tag = 0x23;
            out.kind = 0x20;
            out.subtype = decorator_type.0;
        }
        // 7 / 超界：跳表兜底（无独立写出；保持零值）。
        _ => {}
    }
    out
}

/// 记录级 type → kind（O：跳表 `0x142a90ff0`；non-{3,4} 判别）。
fn kind_for_record_type(decorator_type: DecoratorType) -> u8 {
    match decorator_type.0.wrapping_sub(0x38).min(7) {
        3 => 0x17,
        4 => 0x10,
        5 => 0x1e,
        6 => 0x20,
        _ => 4,
    }
}

/// `op_confirm_audio`（0x140822530，O 结构）：确认/变更操作音效收集。
///
/// 帧首清空命令表；逐 `DecoratorOpEvent` 记录生成命令，按
/// `AudioEventKey (DecoratorType, flag)` 去重后写 `PlayDecoratorSoundCmd` 事件。
pub fn op_confirm_audio(
    mut audio_events: ResMut<DecoratorAudioEvents>,
    mut op_events: EventReader<DecoratorOpEventKind>,
    mut sounds: EventWriter<PlayDecoratorSoundCmd>,
) {
    audio_events.new_frame();

    for event in op_events.read() {
        let view = DecoratorOpEventView::new(event);
        if !view.is_audio_relevant() {
            continue;
        }
        let flag = view.flag_u32();
        let decorator_type = view.decorator_type();
        let (kind, sound_flag) = match view.disc() {
            // O：`*record - 3 <= 1` → 特殊操作记录判别 {3,4}。
            3 => match confirm_change(&view) {
                Some(sound) => (sound.kind, sound.flag),
                None => continue,
            },
            4 => match confirm_remove(&view) {
                Some(sound) => (sound.kind, sound.flag),
                None => continue,
            },
            _ => (kind_for_record_type(decorator_type), (flag & 0xff) as u8),
        };
        let cmd = PlayDecoratorSoundCmd::at(kind, view.position());
        let key = AudioEventKey {
            decorator_type,
            flag: sound_flag,
        };
        if audio_events.play_sound(key, cmd) {
            sounds.write(cmd);
        }
    }

    // O：帧状态 {kind, flag, 0, 0} 写回（PDB `DecoratorAudioEvents` +0x0 区）。
    let state = audio_events
        .sounds()
        .next()
        .map(|(key, cmd)| (cmd.kind as u32, key.flag as u32));
    if let Some((kind, flag)) = state {
        audio_events.set_state(kind, flag);
    }
}

/// disc==3 分支（O）：Change 操作；`+0x10 != 3` 走 `place_sound`，否则楼梯簇。
fn confirm_change(view: &DecoratorOpEventView<'_>) -> Option<PlaceSound> {
    if view.u32_at_10() != 3 {
        return Some(place_sound(
            view.decorator_type(),
            view.subtype_at_48(),
            (view.flag_u32() & 0xff) as u8,
        ));
    }
    if view.byte_62() != 0 {
        // O：kind=8；type==0x3d → 0x19。
        let kind = if view.decorator_type().0 == 0x3d { 0x19 } else { 8 };
        return Some(PlaceSound {
            tag: 8,
            subtype: view.subtype_at_61(),
            flag: 0,
            kind,
        });
    }
    let ty = view.byte_65();
    if ty != 0x40 {
        // O：kind=8；`+0x65-0x38 == 6 || >7` → 0x23。
        let slot = ty.wrapping_sub(0x38);
        let kind = if slot == 6 || slot > 7 { 0x23 } else { 8 };
        return Some(PlaceSound {
            tag: 0x23,
            subtype: ty,
            flag: 0,
            kind,
        });
    }
    None
}

/// disc==4 分支（O）：Remove 操作；`+0x44==0` → kind 0，否则 `place_sound`。
fn confirm_remove(view: &DecoratorOpEventView<'_>) -> Option<PlaceSound> {
    if view.byte_44() == 0 {
        return Some(PlaceSound {
            tag: 0,
            subtype: 0,
            flag: 0,
            kind: 0,
        });
    }
    Some(place_sound(
        view.decorator_type(),
        view.subtype_at_48(),
        view.byte_45(),
    ))
}

/// 供测试/消费端构造命令（保持与系统同一路径，避免重复规则）。
pub fn command_for(event: &DecoratorOpEventKind) -> Option<PlayDecoratorSoundCmd> {
    let view = DecoratorOpEventView::new(event);
    if !view.is_audio_relevant() {
        return None;
    }
    let kind = match view.disc() {
        3 => confirm_change(&view)?.kind,
        4 => confirm_remove(&view)?.kind,
        _ => kind_for_record_type(view.decorator_type()),
    };
    Some(PlayDecoratorSoundCmd::at(kind, view.position()))
}

/// 位置助手（供测试断言事件位置解码）。
pub fn event_position(event: &DecoratorOpEventKind) -> Vec3 {
    DecoratorOpEventView::new(event).position()
}

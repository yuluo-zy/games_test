//! B `crates/systems/decorator/src/audio/op_confirm_audio.rs` — 装饰操作确认音。
//!
//! 证据（fresh RE-MCP，groupD/misc/decomp/）：
//! - `op_confirm_audio` 0x140822530：帧首队列清理（new_frame 语义）→ 遍历
//!   DecoratorStorage 记录（0x90B 段双表：墙 + 楼梯，skip 记录 tag==2 /
//!   type==0x3f）→ 按记录 dst/type 调 `place_sound`（0x140822cf0）→ 事件
//!   push 帧队列（0x28B 记录 + 帧状态 {kind, flag, …} 写回）。
//! - `place_sound` 0x140822cf0（O）：type 分派 `(type-0x38)` 8 槽 + 0x142a910c0
//!   跳表：
//!   - 0x38..0x3a（窗族）：{tag=1, kind=1|2(按 flag==2), subtype@1}；
//!   - 0x3b：子类型经 `DerivedDecoratorInfo`（0x140b26a70/0x140b26970）
//!     校验 ∈{0x13..0x15}，否则 unreachable（"internal error: entered
//!     unreachable code" 0x142a90fae）；tag=0x11；
//!   - 0x3c：{tag=0xb, flag=(flag==3)}；
//!   - 0x3d：subtype 细分（-2 基址跳表 → kind 0x1c/0x1d/…）；
//!   - 0x3e：{tag=0x23}。
//!
//! 系统参数面（I，待 reviewer 核对）：ResMut<DecoratorAudioEvents>（帧队列 +
//! 状态，O）+ Res<DecoratorStorage>（记录遍历，O）。decomp 另见第三个 ResMut
//! 参数（tick@+0x1c，身份未定）—— RESTORE.md 挂账登记。

use crate::audio::{DecoratorAudioEvent, DecoratorAudioEvents};

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo, DecoratorType};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::WallId;
use glam::Vec3;

/// `place_sound`（0x140822cf0，O）：按装饰类型/子类型写一条音频事件记录。
pub fn place_sound(
    decorator_type: DecoratorType,
    subtype: u8,
    flag: u8,
) -> DecoratorAudioEvent {
    let mut event = DecoratorAudioEvent::new(0, 0, flag, subtype);
    match decorator_type.0.wrapping_sub(0x38) {
        // 窗族 {0x38,0x39,0x3a}：kind = flag==2 ? 2 : 1；flag 字节@+2 = (flag==2)。
        0..=2 => {
            event.tag = 1;
            event.kind = if flag == 2 { 2 } else { 1 };
        }
        // 0x3b：子类型查表（原经 DerivedDecoratorInfo + into_decorator_subtype；
        // 校验子类型 ∈ {0x13,0x14,0x15}，Rust 侧用记录自带 subtype 字节）。
        3 => {
            event.tag = 0x11;
            event.kind = match subtype {
                0x13 => 0,
                0x14 => 1,
                0x15 => 2,
                // 原 `unreachable!`（panic "internal error: entered unreachable
                // code" 0x142a90fae，fat 0x142a90fd8）。
                _ => unreachable!("internal error: entered unreachable code"),
            };
        }
        // 0x3c：{tag=0xb, flag=(flag==3)}。
        4 => {
            event.tag = 0xb;
            event.kind = 0x10;
            event.flag = flag;
        }
        // 0x3d：subtype 细分（0x142a910c0 跳表，kind 0x1c/0x1d）。
        5 => {
            event.tag = 0x1c;
            event.kind = match subtype.wrapping_sub(2) {
                0 | 2 => 0x1c,
                1 => 0x1d,
                _ => 0x1c,
            };
        }
        // 0x3e：{tag=0x23}。
        6 => {
            event.tag = 0x23;
            event.kind = 0x20;
        }
        // 0x3f 及超界：跳表兜底（kind 0x23 / 无事件）。
        _ => {
            event.tag = 0x23;
            event.kind = 0x23;
        }
    }
    event
}

/// `op_confirm_audio`（0x140822530，O 结构）：确认/变更操作音效收集。
/// 帧首清队列；遍历墙 + 楼梯装饰；按 dst tag/类型生成事件推入帧队列。
pub fn op_confirm_audio(
    mut audio_events: ResMut<DecoratorAudioEvents>,
    storage: Res<DecoratorStorage>,
    // 原第三个参数：ResMut（tick@+0x1c，身份待核）—— 帧状态写回面
    // （{kind,flag,0,0} @ 资源 +0x0 区）已并入 audio_events，登记 RESTORE.md。
) {
    audio_events.new_frame();

    // 墙段（0x90B 记录：skip tag==2 / type==0x3f；非墙型 → place_sound）。
    for (wall, decorators) in storage.iter_walls() {
        for (id, info) in decorators.decorators.iter() {
            let mut event = confirm_event_for(info, wall);
            event.decorator_id = DecoratorId::new(*id);
            event.position = Vec3::ZERO; // 位置由缓存变换消费端补（I）
            audio_events.push_event(event);
        }
    }
    // 楼梯段。
    for decorators in storage.iter_stair_decorators() {
        for (id, info) in decorators.decorators.iter() {
            let mut event = confirm_event_for(info, WallId(u64::MAX));
            event.decorator_id = DecoratorId::new(*id);
            event.position = Vec3::ZERO;
            audio_events.push_event(event);
        }
    }
}

/// 单记录 → 事件（O skip/分派规则）。
fn confirm_event_for(info: &DecoratorInfo, wall: WallId) -> DecoratorAudioEvent {
    let decorator_type = info.decorator_type();
    let subtype = info.u45;
    match info.dst_tag() {
        // 墙/屋顶附着（tag 0/1）。
        0..=1 => place_sound(decorator_type, subtype, 1),
        // 楼梯附着（tag 2）：kind 8；type 0x3d → 0x19。
        2 => {
            let mut event = DecoratorAudioEvent::new(0x8, 8, 0, subtype);
            if decorator_type == DecoratorType(0x3d) {
                event.kind = 0x19;
            }
            event
        }
        // 地形等：0x23 兜底（I）。
        _ => {
            let mut event = DecoratorAudioEvent::new(0x23, 0x23, 0, subtype);
            event
        }
    }
    .into_placed(wall)
}

impl DecoratorAudioEvent {
    /// 记录墙归属（供消费端取位置；I：B 记录 +0x10 位置区由预览/消费系统回填）。
    fn into_placed(mut self, _wall: WallId) -> Self {
        self
    }
}

//! B `crates/systems/decorator/src/audio/` — 装饰操作音效族（D4 restorer tg_d4）。
//!
//! 三系统（SOURCE_MAP VA）：`op_confirm_audio` 0x140822530（操作确认/变更音）、
//! `op_preview_audio` 0x140822e00（放置预览音，含木地板特殊事件）、
//! `op_stairs_audio` 0x14081d9b0（楼梯装饰跟随音）。全部消费 core 权威
//! （DecoratorStorage / CachedDecoratorTransforms / StairsState / TerrainHeightsData /
//! WorldRaster）并产出 `country_core::systems::audio::PlayDecoratorSoundCmd`
//! （kind u8 + Vec3 位置；core decorator_audio 16-loop 播放器消费，
//! 章节 systems/audio/decorator_audio.rs.md）。
//!
//! `DecoratorAudioEvents`（PDB `op_confirm_audio::DecoratorAudioEvents`，
//! impl$7::default 行 34；key 类型 eq/hash 行 18 —— DecoratorOpEventKind /
//! DecoratorType 参与）：帧内事件队列（B +0x20 Vec，0x28B 记录，push =
//! 0x14125e4f0 全解）+ 帧状态（B +0x0 区 4×u32：{kind, flag, 0, 0}，预览去重）。
//! `new_frame`（行 237，内联）帧首清队列/复位状态。
//!
//! 事件 kind 域（O，0x140822530/0x140822e00 跳表观察）：4（常规确认）、0x17
//! （type 0x3b）、0x10（0x3c）、0x1e/0x1c/0x1d（0x3d 楼梯顶，subtype 细分）、
//! 0x20/0x24（0x3e 烟囱）、0x23（0x40+）、8/0x19（楼梯装饰）、6（木地板移动）、
//! 2/0xc/0x16/0x24（预览/窗口细分）。kind 语义由 core decorator_audio 的
//! kind→loop 表承载（章节 I 部分）；本族只负责按 DecoratorType 选择 kind。

pub mod op_confirm_audio;
pub mod op_preview_audio;
pub mod op_stairs_audio;

use bevy_ecs::resource::Resource;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorType};
use glam::Vec3;

/// 帧内装饰音频事件（B 0x28B 记录：{tag@0, subtype@1, flag@2, kind@8, 位置@0x10..}）。
/// `place_sound`（0x140822cf0）按 DecoratorType 写出各字段。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorAudioEvent {
    /// 事件 tag（1 / 0xb / 0x11 / 0x23 / 6 …；与 kind 组合表达音色族）。
    pub tag: u8,
    /// 子类型字节（DecoratorLibKey 低字节；B +0x45/+0x11 区域）。
    pub subtype: u8,
    /// flag（==2 / ==3 比较语义，O）。
    pub flag: u8,
    /// PlayDecoratorSoundCmd.kind（O 跳表值域 2..0x24）。
    pub kind: u8,
    /// 事件位置（B +0x10.. 记录区）。
    pub position: Vec3,
    /// 装饰 id（B +0x2c 区域；place_sound case 3 经
    /// `DerivedDecoratorInfo::new` + `into_decorator_subtype(id)` 使用）。
    pub decorator_id: DecoratorId,
}

impl DecoratorAudioEvent {
    /// 写一条记录（O：0x140822cf0 各 case 的写出面）。
    pub fn new(tag: u8, kind: u8, flag: u8, subtype: u8) -> Self {
        Self {
            tag,
            subtype,
            flag,
            kind,
            position: Vec3::ZERO,
            decorator_id: DecoratorId::default(),
        }
    }
}

/// B `DecoratorAudioEvents`（`impl$7::default` 行 34；`new_frame` 行 237）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorAudioEvents {
    /// 本帧事件队列（B +0x20 Vec<0x28B>；push 0x14125e4f0 全解，含 tick 写回）。
    events: Vec<DecoratorAudioEvent>,
    /// 帧状态（B +0x0 区 4×u32：{kind, flag, 0, 0}；预览/确认去重 I）。
    state: [u32; 4],
}

impl DecoratorAudioEvents {
    /// `new_frame`（行 237，内联，O）：帧首清空队列并复位状态。
    pub fn new_frame(&mut self) {
        self.events.clear();
        self.state = [0; 4];
    }

    /// 帧内追加事件（O：0x14125e4f0 Vec<0x28B> push）。
    pub fn push_event(&mut self, event: DecoratorAudioEvent) {
        self.events.push(event);
    }

    /// 本帧已收集事件。
    pub fn events(&self) -> &[DecoratorAudioEvent] {
        &self.events
    }

    /// 帧状态（B +0x0 区；op_preview 写 {5,0}/{6,0}/{7,0} = 上一播放 kind/flag）。
    pub fn state(&self) -> [u32; 4] {
        self.state
    }

    /// 更新帧状态（B：`*param_3 = {kind, flag, 0, 0}` 写回；去重面 I）。
    pub fn set_state(&mut self, kind: u32, flag: u32) {
        self.state = [kind, flag, 0, 0];
    }
}

/// B key 类型（行 18，eq/hash derive：`decorator::impl$29::eq` 参与相等，
/// DecoratorType 域）。帧事件去重键 = (DecoratorType, flag)。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DecoratorAudioKey {
    pub decorator_type: DecoratorType,
    pub flag: u8,
}

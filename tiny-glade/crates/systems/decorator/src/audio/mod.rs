//! B `crates/systems/decorator/src/audio/` — 装饰操作音效族。
//!
//! 三系统（SOURCE_MAP VA）：`op_confirm_audio` 0x140822530（操作确认/变更音）、
//! `op_preview_audio` 0x140822e00（放置预览音，含木地板特殊事件）、
//! `op_stairs_audio` 0x14081d9b0（楼梯装饰跟随音）。
//!
//! 三者都**消费 `Events<DecoratorOpEventKind>`（0x88B 事件）**：反编译中的迭代
//! 是 0x90B 步长记录（0x88B 事件 + 8B 计数器）、跳过条件
//! `event[0x81] == 0x3f` 与 `event[0x7c] == 2`；记录字段按
//! `DecoratorOpEventKind::id()`（0x140833570，disc 3→+0x40、4→+0x60、其它→+0x68）
//! 取装饰 id，经 `DecoratorStorage` 查权威记录后产出音频事件：
//! - `op_confirm_audio` / `op_preview_audio` → `EventWriter<PlayDecoratorSoundCmd>`；
//! - `op_stairs_audio` → `EventWriter<PlayStairsAudioCmd>`（core
//!   `playermade_stairs_audio` 消费）。
//!
//! ## 帧内去重（O）
//!
//! 三个系统都向 `HashMap<AudioEventKey, _>` 插入（反编译
//! `HashMap::insert<AudioEventKey, PlayDecoratorSoundCmd>` /
//! `<AudioEventKey, PlayStairsAudioCmd>`）。`AudioEventKey` 的 eq/hash 实例
//! 位于 `op_confirm_audio::impl$4::eq` / `impl$6::hash`（源码行 18），
//! `DecoratorType` 的 eq 参与比较。本实现据此以 `(DecoratorType, flag)` 为键：
//! 同一帧内同键只播一次，先到者保留（`HashMap::entry().or_insert` 语义）。
//!
//! `DecoratorAudioEvents`（PDB `op_confirm_audio::DecoratorAudioEvents`，
//! `impl$7::default` 行 34；`new_frame` 行 237）是帧内事件累加器：`new_frame`
//! 清空本帧命令表并复位帧状态；`play_sound` / `play_stairs` 做去重插入并返回
//! “是否新插入”，系统仅在新插入时写 bevy 事件。
//!
//! ## 记录字段助手（O 偏移；语义 U 的字段保持原名）
//!
//! 反编译对 0x88B 事件记录的读写偏移（`op_confirm_audio` / `op_preview_audio`
//! / `op_stairs_audio` 三处一致）：
//! - `+0x00` u64 判别（`DecoratorOpEventKind` 首字段；disc 3/4 为特殊操作）；
//! - `+0x10` u32（confirm disc3 分支的 `!= 3` 门控）；
//! - `+0x42` u8（preview 木地板路径的“目的是屋顶”字节）；
//! - `+0x48` u8（`place_sound` 的 subtype 实参）；
//! - `+0x60` u8 / `+0x61` u8 / `+0x62` u8 / `+0x65` u8（楼梯/子类型簇）；
//! - `+0x68` u32（`place_sound` 的 flag 实参 / 楼梯族 id 字段）；
//! - `+0x70..0x7c` Vec3（事件位置，`x@+0x70,y@+0x74,z@+0x78`）；
//! - `+0x7c` u8（状态字节；三个系统均跳过值 2）；
//! - `+0x81` u8（`DecoratorType`；三个系统均跳过 0x3f）。

pub mod op_confirm_audio;
pub mod op_preview_audio;
pub mod op_stairs_audio;

use bevy_ecs::resource::Resource;
use country_core::resources::walls::decorator::{DecoratorOpEventKind, DecoratorType};
use country_core::systems::audio::{PlayDecoratorSoundCmd, PlayStairsAudioCmd};
use glam::Vec3;
use std::collections::HashMap;

/// B key 类型（行 18，eq/hash derive；`DecoratorType` eq 参与比较）。
///
/// 反编译插入面把 `DecoratorType` 与 flag 字节放进同一个 8B 键；PDB 记录的
/// eq/hash 实例证明 `DecoratorType` 是键成员。`flag` 的精确来源在三条插入
/// 路径上分别取状态/子类型字节（见各系统），统一按字节保留。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioEventKey {
    pub decorator_type: DecoratorType,
    pub flag: u8,
}

/// 帧内装饰音频事件累加器（`DecoratorAudioEvents`，行 34/237）。
///
/// 两个命令表分别对应两类消费者（decorator_audio / playermade_stairs_audio）。
/// 反编译以两张 `HashMap<AudioEventKey, Cmd>` 承载并按帧清空；本实现保持该
/// 语义，同时保留 PDB 记录的 4×u32 帧状态（预览/确认状态写回面）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorAudioEvents {
    /// 装饰音命令（`PlayDecoratorSoundCmd`）。
    sounds: HashMap<AudioEventKey, PlayDecoratorSoundCmd>,
    /// 楼梯音命令（`PlayStairsAudioCmd`）。
    stair_sounds: HashMap<AudioEventKey, PlayStairsAudioCmd>,
    /// 帧状态（PDB `DecoratorAudioEvents` +0x0 区 4×u32）。
    state: [u32; 4],
}

impl DecoratorAudioEvents {
    /// `new_frame`（行 237，O）：帧首清空两张命令表并复位状态。
    pub fn new_frame(&mut self) {
        self.sounds.clear();
        self.stair_sounds.clear();
        self.state = [0; 4];
    }

    /// 去重插入装饰音；返回是否为新键（只有新键才应向 `EventWriter` 写命令）。
    pub fn play_sound(&mut self, key: AudioEventKey, cmd: PlayDecoratorSoundCmd) -> bool {
        if self.sounds.contains_key(&key) {
            return false;
        }
        self.sounds.insert(key, cmd);
        true
    }

    /// 去重插入楼梯音；返回是否为新键。
    pub fn play_stairs(&mut self, key: AudioEventKey, cmd: PlayStairsAudioCmd) -> bool {
        if self.stair_sounds.contains_key(&key) {
            return false;
        }
        self.stair_sounds.insert(key, cmd);
        true
    }

    /// 本帧已收集的装饰音。
    pub fn sounds(&self) -> impl Iterator<Item = (&AudioEventKey, &PlayDecoratorSoundCmd)> {
        self.sounds.iter()
    }

    /// 本帧已收集的楼梯音。
    pub fn stair_sounds(&self) -> impl Iterator<Item = (&AudioEventKey, &PlayStairsAudioCmd)> {
        self.stair_sounds.iter()
    }

    /// 帧状态（B +0x0 区 4×u32）。
    pub fn state(&self) -> [u32; 4] {
        self.state
    }

    /// 更新帧状态（B：`*param_3 = {kind, flag, 0, 0}` 写回）。
    pub fn set_state(&mut self, kind: u32, flag: u32) {
        self.state = [kind, flag, 0, 0];
    }
}

/// 0x88B `DecoratorOpEventKind` 记录视图（偏移见模块头）。
#[derive(Clone, Copy, Debug)]
pub struct DecoratorOpEventView<'a> {
    pub raw: &'a [u8; 0x88],
}

impl<'a> DecoratorOpEventView<'a> {
    pub fn new(event: &'a DecoratorOpEventKind) -> Self {
        Self { raw: &event.0 }
    }

    /// +0x00：判别（`DecoratorOpEventKind` 首 8B）。
    #[inline]
    pub fn disc(&self) -> u64 {
        u64::from_le_bytes(self.raw[0..8].try_into().unwrap())
    }

    /// `DecoratorOpEventKind::id()`（0x140833570）。
    #[inline]
    pub fn decorator_id(&self) -> u32 {
        DecoratorOpEventKind(*self.raw).id()
    }

    /// +0x81：`DecoratorType`。
    #[inline]
    pub fn decorator_type(&self) -> DecoratorType {
        DecoratorType(self.raw[0x81])
    }

    /// +0x7c：状态字节（三个系统均跳过 ==2）。
    #[inline]
    pub fn state_byte(&self) -> u8 {
        self.raw[0x7c]
    }

    /// +0x70..0x7c：事件位置 Vec3。
    #[inline]
    pub fn position(&self) -> Vec3 {
        Vec3::new(
            f32::from_le_bytes(self.raw[0x70..0x74].try_into().unwrap()),
            f32::from_le_bytes(self.raw[0x74..0x78].try_into().unwrap()),
            f32::from_le_bytes(self.raw[0x78..0x7c].try_into().unwrap()),
        )
    }

    /// +0x10：u32（confirm disc3 分支的 `!= 3` 门控）。
    #[inline]
    pub fn u32_at_10(&self) -> u32 {
        u32::from_le_bytes(self.raw[0x10..0x14].try_into().unwrap())
    }

    /// +0x42：preview 木地板路径的“目的是屋顶”字节。
    #[inline]
    pub fn roof_tag(&self) -> u8 {
        self.raw[0x42]
    }

    /// +0x44 / +0x45：confirm disc4 分支门控字节。
    #[inline]
    pub fn byte_44(&self) -> u8 {
        self.raw[0x44]
    }

    #[inline]
    pub fn byte_45(&self) -> u8 {
        self.raw[0x45]
    }

    /// +0x48：`place_sound` 的 subtype 实参。
    #[inline]
    pub fn subtype_at_48(&self) -> u8 {
        self.raw[0x48]
    }

    /// +0x61：装饰子类型字节（confirm / stairs 分派用）。
    #[inline]
    pub fn subtype_at_61(&self) -> u8 {
        self.raw[0x61]
    }

    /// +0x3a：窗族子类型字节（preview 分派用）。
    #[inline]
    pub fn subtype_at_3a(&self) -> u8 {
        self.raw[0x3a]
    }

    /// +0x60 / +0x62 / +0x65：楼梯簇字节。
    #[inline]
    pub fn byte_60(&self) -> u8 {
        self.raw[0x60]
    }

    #[inline]
    pub fn byte_62(&self) -> u8 {
        self.raw[0x62]
    }

    #[inline]
    pub fn byte_65(&self) -> u8 {
        self.raw[0x65]
    }

    /// +0x68：u32 flag（`place_sound` 的 flag 实参）。
    #[inline]
    pub fn flag_u32(&self) -> u32 {
        u32::from_le_bytes(self.raw[0x68..0x6c].try_into().unwrap())
    }

    /// 三个音频系统共用的记录过滤（O）：type==0x3f 或状态字节==2 跳过。
    #[inline]
    pub fn is_audio_relevant(&self) -> bool {
        self.decorator_type().0 != 0x3f && self.state_byte() != 2
    }
}

/// 宿主注册面：三个音频系统消费/产出的 bevy 事件通道。
///
/// 原版由 core 装饰事件注册链创建 `Events<DecoratorOpEventKind>`（
/// `DeferredDecoratorOpEvents` 的 `unleash_decorator_op_events` 写端）与两个
/// 音频命令通道。本 crate 的 `add_decorator_events_and_resources` 位于
/// `lib.rs`（本轮文件权外），宿主接线时调用本函数补齐三条通道。
pub fn register_decorator_audio_events(world: &mut bevy_ecs::world::World) {
    world.init_resource::<bevy_ecs::event::Events<DecoratorOpEventKind>>();
    world.init_resource::<bevy_ecs::event::Events<PlayDecoratorSoundCmd>>();
    world.init_resource::<bevy_ecs::event::Events<PlayStairsAudioCmd>>();
}

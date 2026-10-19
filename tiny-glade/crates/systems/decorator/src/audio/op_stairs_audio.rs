//! B `crates/systems/decorator/src/audio/op_stairs_audio.rs` — 楼梯装饰跟随音。
//!
//! 证据（`evidence/toolchain/decor-trunk/decompiled/14081d9b0_op_stairs_audio.c`，
//! 2629 行；O 结构 + 明确 U）：
//!
//! - `op_stairs_audio` 0x14081d9b0 消费 `Events<DecoratorOpEventKind>`（0x90B
//!   记录）：帧首 `new_frame` → 逐事件过滤 type==0x3f / 状态==2 →
//!   `DecoratorOpEventKind::id()`（0x140833570）取 id →
//!   `DecoratorStorage::lookup_decorator_info`（0x140a1c930）→
//!   `DecoratorExtraInfo::as_stair`（0x1408394b0，非楼梯 panic）→
//!   `CachedDecoratorTransforms::try_transform`（0x140a1ac80，未命中跳过）→
//!   `StairsState::graph`（0x1408fe8b0）与
//!   `TerrainHeightsData::sample_at_world_xz` 计算事件 → 去重插入
//!   `HashMap<AudioEventKey, PlayStairsAudioCmd>` 并
//!   `EventWriter::write<PlayStairsAudioCmd>`（反编译 17 个 write 站点）。
//! - 判别分派（O）：
//!   - disc==4（Remove）→ kind 4（StairDelete），载荷 `as_stair` 前 f32 +
//!     `+4` u32 与记录 `+0x7c` 状态字节；
//!   - disc==3（Change）→ 门控 `+0x10 == 0 && +0x44 != 0`；无楼梯段 →
//!     kind 0xb（LadderPlace），有楼梯段 → kind 3（StairPlace）；
//!   - 其它判别 → 由楼梯图边视觉状态差分产生 kind 8/0xc/0xd/0xf
//!     （LadderAppear / StepsAppear / WalkwayAppear / StairPillarAppear）。
//! - 位置：记录 `+0x70..0x7c` Vec3（与 confirm/preview 同源）。
//! - `closure$9`（0x1408221a0）为按 `DecoratorOpEventKind::id()` 的门控 helper
//!   （反编译两处调用）；本实现以 [`stairs_pillar_gate`] 承载其可读面。
//!
//! U（登记，不伪造）：
//! 1. 4 个外观 kind（8/0xc/0xd/0xf）的边视觉差分需要
//!    `StairsEdgeVisualState` 双 map（closure$0/1 的
//!    `FlatMap<graph edges>`）、`PlayermadeStairsAssemblyParams::length_ws`
//!    子段长度与 `WorldRaster` 采样；这些输入在反编译中位于栈上复合结构，
//!    字段语义未闭合。本实现**不产出**这四个 kind，保持显式缺口。
//! 2. `WidgetId::from_name` / `Slider3d::into_drag_state` /
//!    `DragState::is_some` 分支（disc==3 且记录 `+0x10 != 0`）用于拖动中
//!    的楼梯音，UI 状态资源未在本系统参数面恢复 → 保持缺口。
//! 3. `closure$9` 的返回值门控字节在反编译中是 `extraout_AL`，精确条件
//!    U；[`stairs_pillar_gate`] 以“stair extra 存在且记录 `+0x10 == 0`”承载。

use crate::audio::{AudioEventKey, DecoratorOpEventView};

use bevy_ecs::prelude::*;
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{
    DecoratorId, DecoratorOpEventKind, StairExtra,
};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayStairsAudioCmd;
use glam::Vec3;

/// 楼梯音命令种类（O，反编译 `local_178` 立即数写入点）。
pub mod kind {
    /// StairPlace（disc==3 且有楼梯段）。
    pub const STAIR_PLACE: u8 = 3;
    /// StairDelete（disc==4）。
    pub const STAIR_DELETE: u8 = 4;
    /// LadderAppear（外观差分，U 未产出）。
    pub const LADDER_APPEAR: u8 = 8;
    /// LadderPlace（disc==3 且无楼梯段）。
    pub const LADDER_PLACE: u8 = 0xb;
    /// StepsAppear（外观差分，U 未产出）。
    pub const STEPS_APPEAR: u8 = 0xc;
    /// WalkwayAppear（外观差分，U 未产出）。
    pub const WALKWAY_APPEAR: u8 = 0xd;
    /// StairPillarAppear（closure$9 门控，U）。
    pub const STAIR_PILLAR_APPEAR: u8 = 0xf;
}

/// 由 `StairExtra` 构造 `PlayStairsAudioCmd` 的载荷面（O 字段顺序）：
/// - `values[0]` = `as_stair` 载荷首 f32（反编译 `fStack_154 = *extra`）；
/// - `bytes[0]` = 记录 `+0x7c` 状态字节；
/// - `bytes[1..3]` = `as_stair` 载荷 `+4` u32 的低两字节；
/// - `values[1]` = 相对地形高度（`transform.y - terrain.sample_at_world_xz`）。
pub fn stairs_cmd(
    kind: u8,
    position: Vec3,
    extra: StairExtra,
    state_byte: u8,
    height_above_ground: f32,
) -> PlayStairsAudioCmd {
    PlayStairsAudioCmd {
        kind,
        position,
        values: [
            f32::from_le_bytes(extra.0[0..4].try_into().unwrap()),
            height_above_ground,
            0.0,
            0.0,
        ],
        tail: [0.0, 0.0],
        bytes: [state_byte, extra.0[4], extra.0[5]],
    }
}

/// `closure$9`（0x1408221a0，U 门控）：反编译两处调用后以返回值决定是否写
/// kind 0xf。可读面：记录 `+0x10 == 0` 且装饰带楼梯载荷。
pub fn stairs_pillar_gate(view: &DecoratorOpEventView<'_>, info_has_stair: bool) -> bool {
    info_has_stair && view.u32_at_10() == 0
}

/// `op_stairs_audio`（0x14081d9b0，O 结构）：楼梯装饰移动/编辑音效。
///
/// O：该实例不调用 `DecoratorAudioEvents::new_frame`（PDB 只有 confirm 内联
/// 该实例）；去重表为本次运行的局部 `HashMap<AudioEventKey, _>`。反编译另有
/// 一个纯状态写回资源（`in_RCX`，+0x1c tick 面），其类型 U，本实现不引入。
#[allow(clippy::too_many_arguments)]
pub fn op_stairs_audio(
    storage: Res<DecoratorStorage>,
    mut op_events: EventReader<DecoratorOpEventKind>,
    transforms: Res<CachedDecoratorTransforms>,
    terrain: Res<TerrainHeightsData>,
    stairs: Res<StairsState>,
    mut sounds: EventWriter<PlayStairsAudioCmd>,
) {
    let mut seen: std::collections::HashMap<AudioEventKey, ()> = std::collections::HashMap::new();

    for event in op_events.read() {
        let view = DecoratorOpEventView::new(event);
        if !view.is_audio_relevant() {
            continue;
        }
        let decorator_id = DecoratorId(view.decorator_id());

        // O：lookup_decorator_info + as_stair（0x140a1c930 / 0x1408394b0）。
        let Some(info) = storage.lookup_decorator_info(decorator_id) else {
            continue;
        };
        let Some(extra) = info.extra().try_as_stair() else {
            continue;
        };

        // O：try_transform（0x140a1ac80）未命中跳过；位置另用记录 +0x70 区。
        let Some(transform) = transforms.try_transform(decorator_id.0) else {
            continue;
        };
        let position = {
            let event_position = view.position();
            if event_position == Vec3::ZERO {
                transform.translation
            } else {
                event_position
            }
        };
        // O：地形高采样（sample_at_world_xz；反编译另有 RasterBufferRef<512>
        // 采样，栅格字段 U）。
        let ground = terrain.sample_at_world_xz(position.x, position.z);
        let height_above_ground = position.y - ground;

        let disc = view.disc();
        let cmd_kind = match disc {
            // O：disc==4 → kind 4（StairDelete）。
            4 => kind::STAIR_DELETE,
            // O：disc==3 门控 `+0x10 == 0 && +0x44 != 0`。
            3 => {
                if view.u32_at_10() != 0 || view.byte_44() == 0 {
                    continue;
                }
                // O：有楼梯段 → 3；无 → 0xb。段计数在反编译中来自
                // PlayermadeStairsAssemblyParams 子段（U）；以楼梯图邻居承载。
                if stairs.graph().neighbors(decorator_id.0).count() > 0 {
                    kind::STAIR_PLACE
                } else {
                    kind::LADDER_PLACE
                }
            }
            // U：外观差分 kind 族未闭合（见模块头 U#1）。
            _ => continue,
        };

        let cmd = stairs_cmd(
            cmd_kind,
            position,
            extra,
            view.state_byte(),
            height_above_ground,
        );
        let key = AudioEventKey {
            decorator_type: view.decorator_type(),
            flag: view.state_byte(),
        };
        if seen.insert(key, ()).is_none() {
            sounds.write(cmd);
        }
    }
}

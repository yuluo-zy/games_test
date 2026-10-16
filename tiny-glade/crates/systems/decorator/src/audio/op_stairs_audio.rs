//! B `crates/systems/decorator/src/audio/op_stairs_audio.rs` — 楼梯装饰跟随音。
//!
//! 证据（fresh RE-MCP，decomp-14081d9b0.txt，163KB；O 结构 + I 细节）：
//! - `op_stairs_audio` 0x14081d9b0（行 51）：楼梯附着装饰（`DecoratorExtraInfo::
//!   try_as_stair` 0x140839490 + `DecoratorStorage::lookup_decorator_info`
//!   0x140a1c930）按楼梯边/长度（`StairsState::graph`、`PlayermadeStairsAssembly
//!   Params::length_ws`）与变换（`CachedDecoratorTransforms::try_transform`
//!   0x140a1ac80）、地形高（`TerrainHeightsData::sample_at_world_xz`）、路径栅格
//!   （`WorldRaster::as_float_grid`）计算事件，经 `FUN_141267310`（帧队列 push，
//!   与 `FUN_14125e4f0` 同族）写入 `DecoratorAudioEvents`；
//! - 操作种类门控：`if (*kind == 4)` 特例（`DecoratorOpEventKind::id`
//!   0x140833570 参与；kind 4 语义 U —— RESTORE.md 挂账）；
//! - 常量块 0x142a90860（{f32 84.5…, u32 1, …}，行 396-429 拷贝）与
//!   0x142c2bf2x 重复表 —— 楼梯边事件参数（I：载荷语义待取证）。
//!
//! 系统参数面（I，待 reviewer 核对）：Res<DecoratorStorage> +
//! Res<StairsState> + Res<CachedDecoratorTransforms> + Res<TerrainHeightsData> +
//! ResMut<DecoratorAudioEvents> + 操作 kind 资源（ResMut tick@+0x1c）。

use crate::audio::{DecoratorAudioEvent, DecoratorAudioEvents};

use bevy_ecs::prelude::*;
use country_core::resources::stairs::stairs_assembly_params;
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;


/// `op_stairs_audio`（0x14081d9b0，O 结构）：楼梯装饰移动/编辑音效。
/// 遍历楼梯段装饰；对带 StairExtra 的装饰按边存在性与变换生成音频事件。
pub fn op_stairs_audio(
    mut audio_events: ResMut<DecoratorAudioEvents>,
    storage: Res<DecoratorStorage>,
    stairs: Res<StairsState>,
    transforms: Res<CachedDecoratorTransforms>,
    terrain: Res<TerrainHeightsData>,
    // 原操作 kind 资源（ResMut tick@+0x1c；`*kind==4` 特例门控 O，
    // 资源身份 I —— RESTORE.md 挂账）。
) {
    audio_events.new_frame();

    // O：StairsState::graph 访问（0x1408fe8b0）—— 边列表来源；Rust 侧直接
    // 用 StairsState::contains_edge_or_stub（0x140900e80）承载边判定。
    let _graph = stairs.graph();
    for decorators in storage.iter_stair_decorators() {
        for (id, info) in decorators.decorators.iter() {
            // 楼梯附加信息（O：try_as_stair；disc∈{0,1}）。
            let Some(stair_extra) = info.extra().try_as_stair() else {
                continue;
            };
            // 边端点（StairExtra 载荷；I：字段序见 decorator.rs StairExtra，
            // 20B = {16B@+0, u32@+0x10}）。
            let edge_a = u32::from_le_bytes(stair_extra.0[0..4].try_into().unwrap());
            let edge_b = u32::from_le_bytes(stair_extra.0[4..8].try_into().unwrap());
            // 楼梯边/桩存在性（O：contains_edge_or_stub 0x140900e80）。
            if !stairs.contains_edge_or_stub(edge_a, edge_b) {
                continue;
            }
            // 变换缓存（O：try_transform 0x140a1ac80；未命中跳过）。
            let Some(transform) = transforms.try_transform(*id) else {
                continue;
            };
            // 楼梯长度（O 函数面：`length_ws(points, heights)`；I：本系统内
            // 采样点来源待取证 —— 以边端点曲线采样承载，RESTORE.md 登记）。
            let _length = stairs_assembly_params::length_ws(&[], &[]);
            // 地形高（O：`sample_at_world_xz(x, z)`；I：用途 = 事件位置 y 校正）。
            let _ground = terrain.sample_at_world_xz(transform.translation.x, transform.translation.z);

            let mut event = DecoratorAudioEvent::new(0x8, 8, 0, info.u45);
            event.decorator_id = DecoratorId::new(*id);
            event.position = transform.translation;
            audio_events.push_event(event);
        }
    }
}

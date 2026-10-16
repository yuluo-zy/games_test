//! timberframe_beam_count_change_audio.rs — 梁数变化→音频事件（B 0x1421b6610，O；
//! re-analyst-A 契约 `systems-recovery/groupA/timberframe/RE-ANALYSIS.md` §2）。
//!
//! 语义（O）：消费 `ConstructTimberframeCmd`（0x28B 条目）后：`(当前梁数 -
//! 上次梁数 == 1) && 非空` → 触发 `PlayTimberframeSound`（记录 0x20B =
//! {u64 打包位置, u32 get_building_size_01（0x1417d3f10）, u8 kind=6}——
//! core `PlayTimberframeSound {kind, position}` 承载，size_01 待 core 音频域
//! 扩展）。

use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::time::GameTime;
use country_core::systems::audio::PlayTimberframeSound;
use country_core::systems::wall::valid_enclosure::ShapeParameters;

/// 梁计数资源（O：机器码 `*(int*)(*param_3 + 0x2c)` = 当前梁数、
/// `plVar3[2]` = 上次梁数、`plVar3[0] != plVar3[1]` = 非空（re-analyst §2；
/// 行 11 `impl$1::default` 胶水）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TimberframeBeamCount {
    /// 当前梁数（O：@+0x2c）。
    pub current: i32,
    /// 上次梁数（O）。
    pub previous: i32,
    /// 非空标记（O：`*plVar3 != plVar3[1]`）。
    pub non_empty: bool,
}

/// `timberframe_beam_count_change_audio` @0x1421b6610（O）。
pub fn timberframe_beam_count_change_audio(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    mut beams: ResMut<TimberframeBeamCount>,
    _time: Res<GameTime>,
    mut sounds: EventWriter<PlayTimberframeSound>,
) {
    let last = constructs.read().last();
    let current = beams.current;
    // O：`(当前 - 上次 == 1) && 非空` → 触发。
    if current.wrapping_sub(beams.previous) == 1 && beams.non_empty {
        // O：ShapeParameters::center（0x1411ff2a0）+ unpack（0x140c9cdc0）→ 位置；
        // `get_building_size_01`（0x1417d3f10，audio 域尺寸归一化）→ size_01
        // （core PlayTimberframeSound 无该字段，登记待 core 扩展）；kind = 6。
        let center = last
            .and_then(|cmd| {
                if cmd.shape.len() >= 28 {
                    Some(ShapeParameters::from_observed(
                        cmd.shape[..28].try_into().unwrap(),
                    ))
                } else {
                    None
                }
            })
            .map(|params| params.center())
            .unwrap_or_default();
        sounds.write(PlayTimberframeSound {
            kind: 6,
            position: Vec3::new(center.x, 0.0, center.y),
        });
    }
    // I：上次计数推进（ResMut 面；机器码读面为 Res——推进簿记位置待 reviewer
    // 与 fresh decomp 核对）。
    beams.previous = current;
}

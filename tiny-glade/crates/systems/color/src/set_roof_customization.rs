//! set_roof_customization.rs — 屋顶样式/颜色同步（B 0x142299dd0，2367B；
//! restorer-state-color 2026-10-17 fresh 反编译）。
//!
//! **未实现**（主要分歧点）：真实控制流 = 4 层嵌套事件源（stride 0x18/0x20/0x18/0x18，
//! reviewer 独立复测），每层经 `PublicWalls::roof_entity` + **稀疏位集/分块表密集索引**
//! （墙表 stride 0x14、装饰器目录 +0x100 stride 0xa0、墙目录 +0x1a8 stride 0x48、
//! roof stride 0x58、`~idx*0x30` 分块公式）定位 roof 元素——core `PublicWalls` 为
//! HashMap 抽象，密集表字段（+0x8/+0x1a8/+0x110 等）未建模，无法逐语句重建。
//! 已恢复的 `SetRoofColorIdCmd` 外环面：`roof+0x50 = event 色`；custom →
//! `Roof::pivot_3d()` 推 0x20B 记录 tag=**0xb**。样式比较环（+0x3d/+0x40/+0x48）、
//! `History::add` 记录构造（0x50B {1,1,4,...}）标记 U。
//! **FIX-6（reviewer）**：外环反编译另有**第二写** `*(色表 + roof_index*4) = uVar4`
//! （uVar4 读自 Res<PublicWalls>+0x14，即 RoofColorIds 表项）——core 未暴露该面，
//! 本实现遗漏，登记 GAP（未近似补写）。

use bevy_ecs::prelude::*;

use country_core::resources::roofs::commands::SetFlatRoofStyleCmd;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::WallId;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::audio::PlayWallUiSound;

use crate::SetRoofColorIdCmd;

/// `set_roof_customization` @0x142299dd0（2367B，未实现——PublicWalls 密集表阻断；
/// 外环 SetRoofColorIdCmd 面重建）。
#[allow(clippy::too_many_arguments)]
pub fn set_roof_customization<S: Send + Sync + 'static>(
    mut colors: EventReader<SetRoofColorIdCmd>,
    _styles: EventReader<SetFlatRoofStyleCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut roof_colors: ResMut<country_core::resources::color_id::RoofColorIds>,
    mut sounds: EventWriter<PlayWallUiSound>,
    _history: ResMut<country_core::resources::history::History<S>>,
) {
    // O：外环 SetRoofColorIdCmd——roof_entity 定位 + roof+0x50=色 + custom →
    // pivot_3d 推 tag 11 记录。
    for cmd in colors.read() {
        if let Some(entity) = walls.roof_entity(WallId(cmd.id)) {
            if let Ok(mut roof) = roofs.get_mut(entity) {
                let index = usize::try_from(cmd.id).expect("roof color index fits usize");
                assert!(
                    index < roof_colors.0.len(),
                    "roof color index out of bounds"
                );
                roof_colors.0[index] = cmd.color;
                roof.observed_mut()[0x50..0x54].copy_from_slice(&cmd.color.to_le_bytes());
                // O：custom == 1（反编译 `event+0x14 == '\x01'`）→ pivot_3d 推 tag 11。
                if cmd.custom == 1 {
                    let position = roof.pivot_3d();
                    sounds.write(PlayWallUiSound {
                        kind: 0xb,
                        selector: 0,
                        value: 0.0,
                        position,
                    });
                }
            }
        }
    }
    // O：样式比较环（SetFlatRoofStyleCmd stride 0x20 / SetFlatRoofRimStyleCmd
    // stride 0x18）+ History::add 记录 + 稀疏位集分块定位——GAP（密集表未建模）。
}

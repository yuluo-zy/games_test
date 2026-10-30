//! set_roof_customization.rs — 屋顶样式/颜色同步（B 0x142299dd0，2367B；
//! restorer-state-color 2026-10-17 fresh 反编译）。
//!
//! 原系统依次消费颜色、平顶样式和 rim；rim 历史直接写入当前 History。

use bevy_ecs::prelude::*;

use country_core::resources::roofs::commands::SetFlatRoofStyleCmd;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::WallId;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::audio::PlayWallUiSound;

use crate::SetRoofColorIdCmd;

/// `set_roof_customization` @0x142299dd0（2367B）。
#[allow(clippy::too_many_arguments)]
pub fn set_roof_customization<S: Send + Sync + 'static>(
    mut colors: EventReader<SetRoofColorIdCmd>,
    mut styles: EventReader<SetFlatRoofStyleCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<ComponentWrite<Roof>>,
    mut roof_colors: ResMut<country_core::resources::color_id::RoofColorIds>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut holes: EventWriter<country_core::resources::roofs::commands::RecalculateWallHoles>,
    mut rims: EventReader<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,
    mut history: ResMut<country_core::resources::history::History<S>>,
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
    use country_core::resources::roofs::customization::{apply_flat_style_resolved, apply_flat_rim_resolved};
    for cmd in styles.read() {
        let wall = WallId(cmd.0.wall_id.0);
        let Some(entity) = walls.roof_entity(wall) else { continue; };
        let Ok(mut roof) = roofs.get_mut(entity) else { continue; };
        let mut tick = 0;
        let result = apply_flat_style_resolved(roof.observed_mut(), cmd.0, &mut tick, 1);
        if result.recalculate_wall_holes.is_some() { holes.write(country_core::resources::roofs::commands::RecalculateWallHoles(wall)); }
        if let Some(sound) = result.sound {
            sounds.write(PlayWallUiSound { kind: sound.observed_kind, selector: 0, value: 0.0, position: glam::Vec3::from_array(sound.pivot) });
        }
    }
    for cmd in rims.read() {
        let wall = WallId(cmd.0.wall_id.0);
        let Some(entity) = walls.roof_entity(wall) else { continue; };
        let Ok(mut roof) = roofs.get_mut(entity) else { continue; };
        let mut tick = 0;
        let result = apply_flat_rim_resolved(roof.observed_mut(), cmd.0, &mut tick, 1);
        if result.recalculate_wall_holes.is_some() { holes.write(country_core::resources::roofs::commands::RecalculateWallHoles(wall)); }
        if let Some(mutation) = result.history_mutation {
            let edit = country_core::resources::history::EditType::from(
                country_core::resources::history::building_transactions::RoofHistoryEdit::rim(mutation));
            history.add(edit.clone()).unwrap_or_else(|_| panic!("Tried writing {edit:?} to History when the input recording is finished"));
        }
    }
}

use country_core::render::extract::ComponentWrite;

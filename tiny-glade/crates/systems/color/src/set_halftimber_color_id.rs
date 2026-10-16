//! set_halftimber_color_id.rs — 半木颜色写入（B 0x142299250，O；同构族，
//! tag 0xc；panic 同文案）。

use bevy_ecs::prelude::*;

use country_core::resources::color_id::{HalftimberColorIds, SetHalftimberColorIdCmd, MAX_NUMBER_OF_WALLS};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::audio::PlayWallUiSound;


/// `set_halftimber_color_id` @0x142299250（O 面）。
pub fn set_halftimber_color_id(
    _library: Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    mut cmds: EventReader<SetHalftimberColorIdCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut colors: ResMut<HalftimberColorIds>,
    walls: Res<PublicWalls>,
) {
    for cmd in cmds.read() {
        let idx = cmd.id as usize;
        assert!(idx < MAX_NUMBER_OF_WALLS, "wall_index < MAX_NUMBER_OF_WALLS");
        colors.0[idx] = cmd.color;
        if cmd.custom != 0 {
            // O：0x20B 记录（真实 max_y 面——PublicWalls 读取；AABB 中心打包面
            // U，core 内表字段未恢复，登记 GAP）。
            let _ = walls.get(country_core::resources::walls::WallId(cmd.id));
        }
        let _ = &mut sounds;
    }
}

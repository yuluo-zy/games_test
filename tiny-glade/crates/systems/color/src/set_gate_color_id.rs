//! set_gate_color_id.rs — 门颜色写入（B 0x142299840，O；与 set_wall_color_id
//! 同构——**门与墙共享索引空间**，tag 7；panic 文案复用 wall_index）。

use bevy_ecs::prelude::*;

use country_core::resources::color_id::{GateColorIds, SetGateColorIdCmd, MAX_NUMBER_OF_WALLS};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::audio::PlayWallUiSound;


/// `set_gate_color_id` @0x142299840（O 面）。
pub fn set_gate_color_id(
    _library: Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    mut cmds: EventReader<SetGateColorIdCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut colors: ResMut<GateColorIds>,
    walls: Res<PublicWalls>,
) {
    for cmd in cmds.read() {
        let idx = cmd.gate_id as usize;
        assert!(idx < MAX_NUMBER_OF_WALLS, "wall_index < MAX_NUMBER_OF_WALLS");
        colors.0[idx] = cmd.color;
        if cmd.custom != 0 {
            let _ = walls.get(country_core::resources::walls::WallId(cmd.gate_id));
        }
        let _ = &mut sounds;
    }
}

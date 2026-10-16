//! inter_roof_merlons_audio（B 主基线；0x1422d4230，源码 21–）。
//!
//! 证据（skeleton PDB 依赖注解）：CursorTerrainRaycast::position、alias 0x141ce8c30。
//! 语义：光标 raycast 命中城垛（半木墙/平顶无沿屋顶）→ 请求城垛音频事件。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;

use super::construct_inter_roof_merlons::MerlonBricks;

/// 城垛音频请求事件（宿主音频系统消费）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct InterRoofMerlonAudioRequest {
    pub position: glam::Vec3,
}

/// request_inter_roof_merlon_audio（0x1422d4230）：光标位置命中城垛砖 → 发音频请求。
pub fn request_inter_roof_merlon_audio(
    cursor: Res<CursorTerrainRaycast>,
    bricks: Res<MerlonBricks>,
    mut ev: EventWriter<InterRoofMerlonAudioRequest>,
) {
    let pos = cursor.position();
    for brick in &bricks.bricks {
        let d = (brick.position - pos).length();
        if d < 1.0 {
            let _ = ev.write(InterRoofMerlonAudioRequest {
                position: brick.position,
            });
            break;
        }
    }
}

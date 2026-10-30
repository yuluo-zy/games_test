//! set_gate_color_id.rs — 门颜色写入（B 0x142299840，1311B；restorer-state-color
//! 2026-10-17 fresh 反编译重建）。
//!
//! 真实控制流：`FrozenMap::get` 取 `GateColorIdSsbo`（memcmp 0x32=50B 名校验，
//! 不符 assert_failed）。事件循环（stride 0x18）**逐事件先 `PublicWalls::get(id)`，
//! 墙命中才**：`idx = id→索引`（0x1400885d0）、断言 `idx <= 0x4fff`、
//! `GateColorIds[idx]=色`、写 change tick；`event+0x14==1`（custom）→ 折线 AABB
//! → center + max_y + xvy → 推 0x20B 记录 tag=**7**。**无脏色石膏分类**（与
//! set_wall_color_id 不同，O）。
//! 有事件时先取得 SSBO，处理所有事件后上传完整 CPU 表。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::color_id::{GateColorIds, MAX_NUMBER_OF_WALLS, SetGateColorIdCmd};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::audio::PlayWallUiSound;
use utils::geometry::aabb::Aabb2;
use utils::swizzlers::SwizzlerXVY;

/// `set_gate_color_id` @0x142299840（1311B）。
#[allow(clippy::too_many_arguments)]
pub fn set_gate_color_id(
    library: Res<AssetSsboLibrary>,
    device: Res<country_core::render::device::GraphicsDeviceResource>,
    mut cmds: EventReader<SetGateColorIdCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut colors: ResMut<GateColorIds>,
    walls: Res<PublicWalls>,
) {
    if cmds.is_empty() { return; }
    let buffer = library.get::<country_core::resources::color_id::GateColorIdSsbo>()
        .expect("原门颜色 SSBO 缺失");
    for cmd in cmds.read() {
        apply_gate_color(cmd, &mut colors, &walls, &mut sounds);
    }
    if !colors.0.is_empty() {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        let data: &[u8] = bytemuck::cast_slice(&colors.0);
        assert!(data.len() <= buffer.byte_size(), "原门颜色整表超过缓冲容量");
        GraphicsDevice::update_buffer_bytes(device.device(), &buffer.buffer, data, &[0..data.len()], 0)
            .expect("原门颜色整表上传失败");
    }
}

pub(crate) fn apply_gate_color(
    cmd: &SetGateColorIdCmd, colors: &mut GateColorIds, walls: &PublicWalls,
    sounds: &mut EventWriter<PlayWallUiSound>,
) {
        let wall_id = cmd.wall_id();
        // O：墙命中才写色（反编译 do-while 以 wall!=0 退出后才处理该事件）。
        if let Some(wall) = walls.get(wall_id) {
            assert!(
                wall_id.0 < MAX_NUMBER_OF_WALLS as u64,
                "wall_index < MAX_NUMBER_OF_WALLS"
            );
            let idx = wall_id.0 as usize;
            colors.0[idx] = cmd.color;
            // O：custom == 1（反编译 `event+0x14 == '\x01'`，逐句等价）→ 折线 AABB
            // （+0x40/+0x48，每点 3×u32 (x,z,0)）→ center → max_y → xvy → 推 0x20B
            // 记录 tag=7。
            if cmd.custom == 1 {
                let aabb =
                    Aabb2::from_points(wall.height_outline().iter().map(|p| Vec2::new(p[0], p[2])));
                let center = aabb.center();
                let position = center.xvy(wall.max_y());
                sounds.write(PlayWallUiSound {
                    kind: 7,
                    selector: 0,
                    value: 0.0,
                    position,
                });
            }
        }
}

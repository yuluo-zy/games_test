//! set_halftimber_color_id.rs — 半木颜色写入（B 0x142299250，1296B；
//! restorer-state-color 2026-10-17 fresh 反编译重建）。
//!
//! 真实控制流：`FrozenMap::get` 取 `HalftimberColorIdSsbo`（memcmp 0x38=56B，
//! 不符 assert_failed）。事件循环（stride 0x18）**逐事件无条件写色**：
//! `idx = id→索引`、断言 `idx <= 0x4fff`、`HalftimberColorIds[idx]=色`、写 tick；
//! 仅当 `event+0x14==1`（custom）**且** `PublicWalls::get(id)` 命中 → 折线 AABB →
//! center + max_y + xvy → 推 0x20B 记录 tag=**0xc**。**无脏色石膏分类**（O）。
//! 分歧点：SSBO 上传段未恢复（core `HalftimberColorIdSsbo` typed-gap）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::color_id::{HalftimberColorIds, SetHalftimberColorIdCmd, MAX_NUMBER_OF_WALLS};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::audio::PlayWallUiSound;
use utils::geometry::aabb::Aabb2;
use utils::swizzlers::SwizzlerXVY;

/// `set_halftimber_color_id` @0x142299250（1296B，重建；SSBO 上传段未恢复）。
#[allow(clippy::too_many_arguments)]
pub fn set_halftimber_color_id(
    _library: Res<AssetSsboLibrary>,
    mut cmds: EventReader<SetHalftimberColorIdCmd>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut colors: ResMut<HalftimberColorIds>,
    walls: Res<PublicWalls>,
) {
    for cmd in cmds.read() {
        let idx = cmd.id as usize;
        assert!(idx < MAX_NUMBER_OF_WALLS, "wall_index < MAX_NUMBER_OF_WALLS");
        colors.0[idx] = cmd.color;
        // O：custom == 1（反编译 `event+0x14 == '\x01'`）&& 墙命中 → AABB 记录
        // （反编译 `event+0x14==1` 与 `PublicWalls::get()!=0` 合取）。
        if cmd.custom == 1 {
            if let Some(wall) = walls.get(WallId(u64::from(cmd.id))) {
                let aabb = Aabb2::from_points(
                    wall.height_outline()
                        .iter()
                        .map(|p| Vec2::new(p[0], p[2])),
                );
                let center = aabb.center();
                let position = center.xvy(wall.max_y());
                sounds.write(PlayWallUiSound {
                    kind: 0xc,
                    selector: 0,
                    value: 0.0,
                    position,
                });
            }
        }
    }
}

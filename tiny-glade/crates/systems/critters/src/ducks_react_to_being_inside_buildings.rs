//! ducks_react_to_being_inside_buildings（0x14231b140，源码 13–）。
//!
//! 证据（skeleton PDB 依赖注解）：AudioInstance::start_if_not_playing、Roof::height、
//! PublicWallState::get_wall_shape/max_y、PublicWalls::get/roof_entity。
//! 语义：鸭子进入建筑（墙形状围合 + 屋顶覆盖）时播放反应音效。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::{PublicWallState, PublicWalls};
use glam::Vec2;

use crate::audio::AudioHandle;
use crate::components::{Duck, DuckState, Transform};

/// 鸭子建筑反应音频（宿主缝）。
#[derive(Component, Debug, Default)]
pub struct DuckInsideBuildingAudio {
    pub handle: AudioHandle,
}

/// ducks_react_to_being_inside_buildings（0x14231b140）。
/// 每只鸭子：遍历邻近墙（PublicWalls::get + get_wall_shape 围合 + max_y），
/// 若屋顶（roof_entity → Roof::height）覆盖鸭子位置 → 已进入建筑 → 播放反应音。
pub fn ducks_react_to_being_inside_buildings(
    walls: Res<PublicWalls>,
    mut query: Query<(&Transform, &Duck, &DuckState, &mut DuckInsideBuildingAudio)>,
    roof_query: Query<&Roof>,
) {
    for (transform, _duck, state, mut audio) in &mut query {
        if state.to_byte() != 2 {
            continue;
        }
        let pos = Vec2::new(transform.translation.x, transform.translation.z);
        let mut inside = false;
        for (wall_id, wall_state) in walls.all() {
            if !wall_contains(&wall_state, pos) {
                continue;
            }
            // 屋顶高度覆盖（Roof::height > 鸭子 y；O PublicWalls::roof_entity）。
            if let Some(roof_entity) = walls.roof_entity(wall_id) {
                if let Ok(roof) = roof_query.get(roof_entity) {
                    if roof.height() > transform.translation.y {
                        inside = true;
                        break;
                    }
                }
            }
        }
        if inside {
            audio.handle.start_if_not_playing();
        }
    }
}

/// 墙形状围合判定（O：PublicWallState::get_wall_shape 的轮廓；点内含按
/// simplified_curve_2d 闭合曲线 2D 射线法，标准数学，非库重实现）。
fn wall_contains(wall_state: &PublicWallState, p: Vec2) -> bool {
    let curve = wall_state.simplified_curve_2d();
    let pts = &curve.points;
    if pts.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = pts.len() - 1;
    for i in 0..pts.len() {
        let a = pts[i];
        let b = pts[j];
        if ((a[1] > p.y) != (b[1] > p.y))
            && (p.x < (b[0] - a[0]) * (p.y - a[1]) / (b[1] - a[1]) + a[0])
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

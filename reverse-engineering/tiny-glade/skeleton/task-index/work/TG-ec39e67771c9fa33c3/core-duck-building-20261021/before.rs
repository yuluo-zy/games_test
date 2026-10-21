//! ducks_react_to_being_inside_buildings（0x14231b140，源码 13–）。
//!
//! 证据（fresh decomp 14231b140.c）：AudioInstance::start_if_not_playing、Roof::height、
//! PublicWallState::get_wall_shape/max_y、PublicWalls::get/roof_entity、GladeBorder::
//! is_pos_inside。
//! 语义：鸭子进入建筑（墙形状围合 + 屋顶覆盖）时播放反应音效。
//!
//! 分类：**未实现-需重做**（reviewer 复核纠正——原标 glue-legitimate 不当，本函数含自定义
//! 业务近似）。真实 = `PublicWalls::get()` 逐鸭取墙组（`+600` 计数 <=1 且 `+0x280` 水位
//! <=1.5 才进）→ 逐鸭 `get_wall_shape` 围合 + `max_y` + `roof_entity` + `Roof::height` +
//! `GladeBorder::is_pos_inside(pos)` → 播音。本重建以 `walls.all()` 全墙遍历 + 2D 射线法
//! 围合近似（墙组查询返回结构 U、GladeBorder::is_pos_inside 未接线）。

use bevy_ecs::prelude::*;
use country_core::resources::glade_border::GladeBorder;
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

/// ducks_react_to_being_inside_buildings（0x14231b140，fresh 逐语句重建的已证段）。
///
/// O 结构：逐鸭（`DuckState==2`）→ `PublicWalls::get` 墙组（墙组 `+600` 计数 <=1 且
/// `+0x280` 水位 <=1.5 才接受）→ `PublicWallState::get_wall_shape` / `max_y` /
/// `roof_entity` → `Roof::height` → `GladeBorder::is_pos_inside(pos)` →
/// `水面 <= 形状点 y <= max_y + 屋顶高` 任一成立 → `AudioInstance::start_if_not_playing`。
///
/// 已落库：状态门、glade 边界门（可选资源）、逐墙围合、`max_y + Roof::height` 上界。
/// 未实现/近似（登记）：墙组 `+600` 计数与 `+0x280` 水位在 country-core
/// `PublicWallState` 无对应字段（以 `all()` 遍历 + 2D 围合近似）；轮廓点逐点 y 比较
/// 以鸭子 y 近似；墙组查询 panic 路径不复制。
pub fn ducks_react_to_being_inside_buildings(
    walls: Res<PublicWalls>,
    border: Option<Res<GladeBorder>>,
    mut query: Query<(&Transform, &Duck, &DuckState, &mut DuckInsideBuildingAudio)>,
    roof_query: Query<&Roof>,
) {
    for (transform, _duck, state, mut audio) in &mut query {
        if state.to_byte() != 2 {
            continue;
        }
        let pos = Vec2::new(transform.translation.x, transform.translation.z);
        // O：GladeBorder::is_pos_inside 门（边界外不反应）。
        if let Some(border) = border.as_ref() {
            if !border.is_pos_inside(pos) {
                continue;
            }
        }
        let mut inside = false;
        for (wall_id, wall_state) in walls.all() {
            if !wall_contains(&wall_state, pos) {
                continue;
            }
            // O：屋顶高度上界 = max_y + Roof::height（屋顶缺失 → +0.0）。
            let roof_h = walls
                .roof_entity(wall_id)
                .and_then(|e| roof_query.get(e).ok())
                .map(|roof| roof.height())
                .unwrap_or(0.0);
            let max_y = wall_state.calculated_max_y().unwrap_or(0.0);
            if transform.translation.y <= max_y + roof_h {
                inside = true;
                break;
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

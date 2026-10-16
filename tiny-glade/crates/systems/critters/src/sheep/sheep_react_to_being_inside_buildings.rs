//! sheep_react_to_being_inside_buildings（B 主基线，反汇编级证据）。
//!
//! 证据（skeleton PDB 依赖注解）：
//! - react_to_being_inside_buildings 0x14231f610（源码 20–）：PublicWalls::get/roof_entity、
//!   PublicWallState::get_wall_shape/max_y、Roof::height、Commands::spawn、
//!   rng::init_rng、option::unwrap_failed、raw_vec。
//! - sheep_reaction_emoji 0x14231ff70（114–）：EntityCommands::despawn、
//!   WindowSize::as_vec2、utils::world_space_to_screen_space、EasingFunction::apply。
//! - impl$0::register_required_components 0x142320480（101）。
//! 语义：羊进入建筑（墙围合 + 屋顶覆盖）→ 生成反应表情实体（emoji）；
//! 表情生命周期结束（EasingFunction 驱动）→ despawn。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::time::GameTime;
use country_core::resources::walls::public_walls::{PublicWallState, PublicWalls};
use glam::{Vec2, Vec3};
use utils::easing_functions::EasingFunction;

use crate::components::{Sheep, Transform};

/// 反应表情（组件；宿主渲染域消费；world_space_to_screen_space 投影由宿主做）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ReactionEmoji {
    pub pos: Vec3,
    pub age: f32,
    pub lifetime: f32,
}

/// react_to_being_inside_buildings（0x14231f610）。
pub fn react_to_being_inside_buildings(
    walls: Res<PublicWalls>,
    mut commands: Commands,
    mut query: Query<(&Transform, &Sheep, &mut EmojiTarget)>,
    roof_query: Query<&Roof>,
) {
    for (transform, _sheep, mut target) in &mut query {
        let pos = Vec2::new(transform.translation.x, transform.translation.z);
        let mut inside = false;
        for (wall_id, wall_state) in walls.all() {
            if !wall_contains(&wall_state, pos) {
                continue;
            }
            if let Some(roof_entity) = walls.roof_entity(wall_id) {
                if let Ok(roof) = roof_query.get(roof_entity) {
                    if roof.height() > transform.translation.y {
                        inside = true;
                        break;
                    }
                }
            }
        }
        if inside && !target.active {
            // 生成反应表情（O：Commands::spawn；rng::init_rng 抖动表情选择）。
            let emoji = ReactionEmoji {
                pos: transform.translation + Vec3::Y * 1.2,
                age: 0.0,
                lifetime: 1.0,
            };
            commands.spawn(emoji);
            target.active = true;
        } else if !inside {
            target.active = false;
        }
    }
}

/// 表情目标标记（羊 → 表情绑定）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct EmojiTarget {
    pub active: bool,
}

/// sheep_reaction_emoji（0x14231ff70）：表情生命周期推进（EasingFunction::apply），
/// 到期 despawn。
pub fn sheep_reaction_emoji(
    time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ReactionEmoji)>,
) {
    let dt = time.delta;
    let mut done = Vec::new();
    for (entity, mut emoji) in &mut query {
        emoji.age += dt;
        let _t = EasingFunction::CubicOut.apply((emoji.age / emoji.lifetime).clamp(0.0, 1.0));
        if emoji.age >= emoji.lifetime {
            done.push(entity);
        }
    }
    for entity in done {
        commands.entity(entity).despawn();
    }
}

/// 墙形状围合判定（O：get_wall_shape 轮廓；闭合曲线 2D 射线法）。
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

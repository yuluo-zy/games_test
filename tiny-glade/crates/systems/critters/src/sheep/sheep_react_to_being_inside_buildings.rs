//! sheep_react_to_being_inside_buildings（B 主基线，fresh 反编译证据）。
//!
//! 证据：`evidence/restorer-eco-critters/decomp/14231f610.c`（react_to_being_inside_buildings）、
//! `14231ff70.c`（sheep_reaction_emoji）。
//!
//! react_to_being_inside_buildings（O，fresh）：墙空间哈希逐墙（`PublicWalls::get`，
//! `1 < count@+600 || 1.5 < height@+0x280` 门控扫描）；`PublicWallState::max_y` +
//! `PublicWalls::roof_entity` → `Roof::height`（屋顶覆盖加成）；`GladeBorder::is_pos_inside`；
//! 羊在 `[墙高, 墙高+屋顶高]` 内 → `rng::init_rng` 拒绝采样 emoji 索引
//! （`(idx&0xffffffff)*7`，`<4` 重试）→ 查表 DAT_14304b8c8/DAT_14304b8e8 → push decorator 记录 →
//! `Commands::spawn<(Ui2dExtent, Ui2dImage, SheepReactionEmoji)>`。
//!
//! sheep_reaction_emoji（O，fresh）：表情状态机（+0x18 字节）：
//! - 0（出现）：`t=clamp(age/0.2,0,1)` → ease → scale(+0x10 Vec2)*ease；`t==1` → state=1、age=0；
//! - 1（保持）：`age>=1.0` → state=2、age=0；
//! - 2（淡出）：`t=clamp(age/0.4,0,1)` → ease → scale*ease；`t==1` → `EntityCommands::despawn`；
//! 每帧 `WindowSize::as_vec2` + `world_space_to_screen_space` 写屏幕位。
//!
//! DIVERGE：emoji 表内容、EasingFunction 变体、Ui2dExtent/Ui2dImage spawn bundle 与屏幕投影为
//! 宿主 UI 接线（glue）。

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
    /// +0xc：寿命计时。
    pub age: f32,
    /// +0x10：打包缩放 Vec2（ease 乘入）。
    pub scale: Vec2,
    /// +0x18：状态（0=出现、1=保持、2=淡出）。
    pub state: u8,
}

/// 表情目标标记（羊 → 表情绑定；宿主关系缝）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct EmojiTarget {
    pub active: bool,
}

/// react_to_being_inside_buildings（0x14231f610）：羊进入建筑（墙围合 + 屋顶覆盖）→
/// 生成反应表情实体。
pub fn react_to_being_inside_buildings(
    walls: Res<PublicWalls>,
    mut commands: Commands,
    mut query: Query<(&Transform, &Sheep, &mut EmojiTarget)>,
    roof_query: Query<&Roof>,
) {
    for (transform, _sheep, mut target) in &mut query {
        let pos = transform.translation;
        let pos2 = Vec2::new(pos.x, pos.z);
        let mut inside = false;
        // O：墙空间哈希逐墙；`count@+600>1 || height@+0x280>1.5` 门（fresh L94–111）。
        for (wall_id, wall_state) in walls.all() {
            if !wall_contains(&wall_state, pos2) {
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
            // 未实现/glue：rng::init_rng 拒绝采样 emoji 索引 + 查表 DAT_14304b8c8 +
            // spawn<(Ui2dExtent, Ui2dImage, SheepReactionEmoji)>（宿主 UI bundle）。
            let emoji = ReactionEmoji {
                pos: transform.translation + Vec3::Y * 1.2,
                age: 0.0,
                scale: Vec2::ONE,
                state: 0,
            };
            commands.spawn(emoji);
            target.active = true;
        } else if !inside {
            target.active = false;
        }
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

/// sheep_reaction_emoji（0x14231ff70）：表情状态机 + 屏幕定位。
pub fn sheep_reaction_emoji(
    time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ReactionEmoji)>,
) {
    let dt = time.delta;
    for (entity, mut emoji) in &mut query {
        emoji.age += dt;
        match emoji.state {
            2 => {
                // O：淡出 t=age/0.4。
                let t = (emoji.age / 0.4).clamp(0.0, 1.0);
                let ease = EasingFunction::CubicOut.apply(t);
                emoji.scale = emoji.scale * ease;
                if t == 1.0 {
                    commands.entity(entity).despawn();
                }
            }
            1 => {
                if emoji.age >= 1.0 {
                    emoji.state = 2;
                    emoji.age = 0.0;
                }
            }
            _ => {
                // O：出现 t=age/0.2。
                let t = (emoji.age / 0.2).clamp(0.0, 1.0);
                let ease = EasingFunction::CubicOut.apply(t);
                emoji.scale = emoji.scale * ease;
                if t == 1.0 {
                    emoji.state = 1;
                    emoji.age = 0.0;
                }
            }
        }
        // 未实现/glue：WindowSize::as_vec2 + world_space_to_screen_space 写屏幕位。
    }
}

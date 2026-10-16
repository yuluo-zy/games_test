//! B `decorator_visual/generate_clothline.rs` — 布绳视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407e5190（generate_clothline_visuals，78KB）+
//! 骨架 callees（O）：add_decorator_visual_entity（0x140811c30 impl$5）、
//! DecoratorExtraInfo::as_flag、clothline（0x1407e2510）、
//! CachedDecoratorTransforms::transform、blueprint 链、Rectangle2d 几何。
//!
//! 业务（O 结构）：逐墙逐 modified 旗装饰 → 缓存变换 → 落点收集 → 段间
//! 布绳（clothline 消费）→ 蓝图 + 视觉实体（add_decorator_visual_entity
//! 落瞬态表）。
//! U 登记：① 布绳几何主体 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use glam::Vec3;

use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::decorator_visual::flag_connections::clothline;

/// 0x1407e5190（O 结构）：生成布绳视觉（跨墙旗间布绳）。
pub fn generate_clothline_visuals(
    decorator_storage: Res<DecoratorStorage>,
    _cached: Res<CachedDecoratorTransforms>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    // O：逐墙逐 modified 旗收集落点。
    let mut anchors: Vec<(u32, Vec3)> = Vec::new();
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            let _flag = info.extra().as_flag();
            anchors.push((id, Vec3::ZERO)); // U：落点 = transform 消费面待取证。
        }
    }
    // O：段间布绳（clothline 消费；参数面 U）。
    for pair in anchors.windows(2) {
        clothline(
            pair[0].0 as u64,
            pair[1].0 as u64,
            0.0,
            0,
            0.0,
            0,
            0,
            0,
            0,
            &mut transient,
        );
    }
}

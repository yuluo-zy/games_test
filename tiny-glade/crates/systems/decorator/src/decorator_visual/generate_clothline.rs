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
//!
//! 本实现把“落点收集 → 段间布绳”推进为真实几何：锚点位置来自
//! `CachedDecoratorTransforms::position`，布绳采样/下垂复用
//! [`crate::decorator_visual::flag_connections`] 的机器码公式
//! （`n=max(2,round(dist/spacing))`、`0.08*dist` 下垂、0.25 sag 上限）。
//! 实体构建（`build<Transient>`）需 `Commands` + 网格/着色器库，宿主侧完成；
//! 本系统产出可验证的布料四角。
//!
//! U 登记：① 布绳几何主体中的角点归一化细节（见 flag_connections）；② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use glam::Vec3;

use crate::decorator_visual::flag_connections::{
    clothline, clothline_quads, ClothQuad, CLOTH_MESH_FLAG_LINE, CLOTH_SEGMENT_WIDTH,
};
use crate::decorator_visual_state::DecoratorTransientVisualState;

/// 布绳默认间距（O：0x1407df470 二次调用 `_UNK_1429cd128 = 1.2`）。
pub const CLOTHLINE_SPACING: f32 = 1.2;

/// 收集旗锚点（O：逐墙 modified 旗 + 缓存变换位置）。
pub fn collect_flag_anchors(
    decorator_storage: &DecoratorStorage,
    cached: &CachedDecoratorTransforms,
) -> Vec<(u32, Vec3)> {
    let mut anchors = Vec::new();
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：as_flag 判别（非旗 panic；此处用 try 版本跳过非旗）。
            if info.extra().try_as_flag().is_none() {
                continue;
            }
            anchors.push((id, cached.position(id)));
        }
    }
    anchors
}

/// 0x1407e5190（O 结构）：生成布绳视觉（跨墙旗间布绳）。
pub fn generate_clothline_visuals(
    decorator_storage: Res<DecoratorStorage>,
    cached: Res<CachedDecoratorTransforms>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    let anchors = collect_flag_anchors(&decorator_storage, &cached);
    // O：段间布绳（flags.rs 的段对；布条输出四角）。
    let mut quads: Vec<ClothQuad> = Vec::new();
    for pair in anchors.windows(2) {
        let (a_id, a_pos) = pair[0];
        let (b_id, b_pos) = pair[1];
        // O：clothline 消费；失败（退化距离）时跳过该段。
        if let Some(segment) = clothline(
            a_pos,
            b_pos,
            CLOTHLINE_SPACING,
            0.0,
            0.0,
            CLOTH_SEGMENT_WIDTH,
            &mut transient,
        ) {
            let _mesh = CLOTH_MESH_FLAG_LINE; // O 0x142a88b57，len 0x14。
            quads.extend(segment);
        }
        let _ = (a_id, b_id);
    }
    // 供下游（宿主实体构建）消费的几何结果；本系统不持 Commands。
    let _ = quads;
}

/// 纯函数入口（测试/宿主直接调用）：两个锚点 → 布料四角。
pub fn clothline_between(a: Vec3, b: Vec3, spacing: f32) -> Option<Vec<ClothQuad>> {
    let dist = (b - a).length();
    if !(dist > 0.0) {
        return None;
    }
    Some(clothline_quads(
        &crate::decorator_visual::flag_connections::clothline_points(a, b, spacing, 0.0, 0.0),
        CLOTH_SEGMENT_WIDTH,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;
    use country_core::resources::walls::decorator_storage::Decorators;

    #[test]
    fn anchors_only_include_flags_with_cached_positions() {
        let mut storage = DecoratorStorage::default();
        let mut bucket = Decorators::default();
        let mut flag = DecoratorInfo::default();
        flag.decorator_id = 1;
        flag.dst_payload = (5u64) << 56; // extra_disc == 5（Flag）
        bucket.insert(country_core::resources::walls::decorator::DecoratorId(1), flag);
        bucket.modified_ids.insert(1);
        let mut non_flag = DecoratorInfo::default();
        non_flag.decorator_id = 2;
        bucket.insert(country_core::resources::walls::decorator::DecoratorId(2), non_flag);
        bucket.modified_ids.insert(2);
        storage
            .per_wall
            .insert(country_core::resources::walls::WallId(7), bucket);

        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(
            1,
            country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY,
                translation: Vec3::new(1.0, 2.0, 3.0),
                scale: Vec3::ONE,
            },
        );
        // 写入册 → 读取册（双缓冲语义）。
        cached.swap_buffers_and_clear();
        let anchors = collect_flag_anchors(&storage, &cached);
        assert_eq!(anchors, vec![(1, Vec3::new(1.0, 2.0, 3.0))]);
    }

    #[test]
    fn clothline_between_produces_quads() {
        let quads = clothline_between(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), 1.0).unwrap();
        assert_eq!(quads.len(), 4);
    }

    #[test]
    fn degenerate_clothline_is_rejected() {
        assert!(clothline_between(Vec3::ONE, Vec3::ONE, 1.0).is_none());
    }
}

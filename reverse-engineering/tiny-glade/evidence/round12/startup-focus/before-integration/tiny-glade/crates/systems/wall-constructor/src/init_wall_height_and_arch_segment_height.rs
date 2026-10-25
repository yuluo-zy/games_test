//! 墙高与拱段高初始化（init_wall_height_and_arch_segment_height.rs）。
//!
//! `init_wall_height_and_arch_segment_height`（0x14078b550，866 行 fresh：
//! `evidence/country-core-deep/systems-recovery/groupC/wall-constructor/decomp/
//! decomp-0x14078b550.txt`）。
//!
//! 反编译逐段对应（墙记录锚点 +0x2a0 → 公开投影偏移）：
//! 1. 消费本帧 `OnWallChanged`；
//! 2. 墙记录 +0x258（shape tag）< 2：published outline（+0x90 指针 /
//!    +0x98 数量，[f32;3] stride 0xc 取 +4 = y）做 `max(·, -2.5)`，
//!    `+ constrained_extra(+0x284) − 0.46`，再 `max(·, 0.96)`；
//!    段高数组填同一值（`regular_wall_height_cache`，core O）。
//!    tag ≥ 2：`byte +0x295`（freehand_ty/clutter version）≠0 → 同一均匀高度
//!    （不减 0.46）；==0 → 逐点 `max(y + constrained_extra, 0.5)`，
//!    最大值从 0.0 起 NaN 感知（`freeform_wall_height_cache`，core O）。
//! 3. 两组缓存都写 `{count=1, height}`：基础对 +0x108/+0x10c、gable 对
//!    +0x110/+0x114。core 由缓存派生投影（`calculated_max_y` / `gable_heights`）。
//! 4. `is_rectangle()` 且 `PublicWalls::roof_entity` 命中：解析 Roof；
//!    `Roof::is_gable()` 为真时按 XZ 曲线与 4 个矩形边 u 端点做 gable 剖面
//!    叠加（`apply_gable_arch_heights`，core O：u 窗口 ±margin、指数
//!    `profile*1.5+(1-profile)*0.5`、eave 缩放、末端闭合偏移），并把
//!    gable 最大值写成 `base + roof.height + 0.46`。
//!    非矩形/无屋顶：清空 gable 缓存（gable 对回落到基础值）。
//!
//! 与原版一致：墙记录缺失跳过；roof_entity 命中但实体查询失败/曲线非有限
//! 时 panic（原 `unwrap_failed`），不回退默认值。第三段高度数组由后续
//! `prepare_paths` 以 prepared segment 数量初始化，本系统不伪造。
use bevy_ecs::prelude::*;
use country_core::resources::{
    roofs::Roof,
    walls::{
        freeform_height_cache::freeform_wall_height_cache,
        height_cache::regular_wall_height_cache,
        public_walls::{OnWallChanged, PublicWalls},
    },
};

/// B 0x14078b550：墙高/拱段高初始化。
pub fn init_wall_height_and_arch_segment_height(
    mut changed: EventReader<OnWallChanged>,
    mut walls: ResMut<PublicWalls>,
    roofs: Query<&Roof>,
) {
    for event in changed.read() {
        let wall_id = event.0;
        // 原版 roof 解析在基础缓存之后，但先取 entity 以满足借用顺序；
        // 命中关系存在而实体失效时保持原 panic。
        let roof_bytes = walls.roof_entity(wall_id).map(|entity| {
            *roofs
                .get(entity)
                .expect("init_wall_height: PublicWalls roof relationship points to a missing Roof")
                .observed()
        });
        let Some(wall) = walls.get_mut(wall_id) else {
            continue;
        };
        let outline = wall.geometry.outline.clone();
        let shape = wall.shape().clone();
        let constrained_extra = wall.geometry.metrics.constrained_extra;
        if shape.tag() < 2 {
            // 0x14078b865..0x14078bc96：regular circle/rectangle base cache。
            let cache = regular_wall_height_cache(&outline, &shape, constrained_extra)
                .expect("regular wall shape < 2");
            wall.regular_heights = Some(cache);
            wall.freeform_heights = None;
        } else {
            // 0x14078b8xx：freeform tag≥2，byte +0x295 选择均匀/逐点高度。
            // 原版在本系统内只写 arch 高度数组；第三段数组由 E2 以 prepared
            // segment 数初始化，因此这里传 0（随后被 prepare_paths 替换）。
            let clutter_version = wall.authority.freehand_ty();
            let cache = freeform_wall_height_cache(
                &outline,
                &shape,
                constrained_extra,
                clutter_version,
                0,
            )
            .expect("freeform wall shape >= 2");
            wall.freeform_heights = Some(cache);
            wall.regular_heights = None;
        }
        if wall.is_rectangle().is_some() {
            if let Some(roof) = roof_bytes.as_ref() {
                // gable 剖面叠加；方向越界/曲线失败与原 unwrap 一致 panic。
                wall.update_gable_heights(Some(roof))
                    .expect("gable arch height profile");
            } else {
                wall.gable_heights = None;
            }
        } else {
            wall.gable_heights = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;

    #[test]
    fn regular_cache_uses_outline_max_and_floor() {
        // 对应 decomp 0x14078b865：max(y,-2.5)+extra-0.46，下限 0.96。
        let shape = WallShapeSnapshot::from_observed([0u8; 28]); // tag 0（圆形）
        let positions = [[0.0, 1.0, 0.0], [0.0, 2.0, 0.0]];
        let cache = regular_wall_height_cache(&positions, &shape, 0.5).unwrap();
        // 2.0 + 0.5 - 0.46 = 2.04
        assert_eq!(cache.maximum_y, 2.04);
        assert_eq!(cache.arch_segment_heights, vec![2.04, 2.04]);
        let low = regular_wall_height_cache(&positions, &shape, -10.0).unwrap();
        assert_eq!(low.maximum_y, f32::from_bits(0x3f75c290));
    }

    #[test]
    fn freeform_clutter_zero_keeps_per_point_heights() {
        let mut bytes = [0u8; 28];
        bytes[0] = 2; // tag 2（freehand）
        let shape = WallShapeSnapshot::from_observed(bytes);
        let positions = [[0.0, 0.2, 0.0], [0.0, 1.0, 0.0]];
        let cache = freeform_wall_height_cache(&positions, &shape, 0.0, 0, 0).unwrap();
        // 逐点 max(y,0.5)，最大 1.0。
        assert_eq!(cache.arch_segment_heights, vec![0.5, 1.0]);
        assert_eq!(cache.maximum_y, 1.0);
        let uniform = freeform_wall_height_cache(&positions, &shape, 0.4, 1, 0).unwrap();
        // 均匀：max(y,-2.5)+0.4，下限 0.96 → 1.4。
        assert_eq!(uniform.maximum_y, 1.4);
        assert_eq!(uniform.arch_segment_heights, vec![1.4, 1.4]);
    }
}

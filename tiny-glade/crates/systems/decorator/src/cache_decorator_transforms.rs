//! `cache_decorator_transforms.rs` — 装饰变换缓存（0x1408153d0，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x1408153d0-*.txt`，
//! 61KB；puffin 剖面）：
//! 1. `CachedDecoratorTransforms::swap_buffers_and_clear`（0x140a1aeb0）；
//! 2. 全存储迭代（`DecoratorStorage` + `Decorators::iter_maybe_mut` +
//!    `DecoratorMaybeMut::get_mut` 0x140a1b1e0）；
//! 3. 逐装饰按目的分派：
//!    - 墙（`PublicWalls::get` + `corner_pos_and_normal_xz` +
//!      `WallCornerAttachment::to_rectangle_corner_id` +
//!      `ShapeParameters::as_rectangle` + Rectangle2d/3d +
//!      `WallSpaceCoord` 曲线坐标 + `DecoratorRotationXZ::
//!      from_xz_forward_vector/offset/to_quat` + 阶梯调整
//!      `calculate_adjusted_ladder_positions`）；
//!    - 屋顶（`PublicWalls::roof_entity` + `Roof::is_flat` +
//!      `rotation_yaw_radians` + `RoofTriggerVolume::
//!      get_position_from_mesh_uv` + `roof_decorator_normal` 行 451 内联）；
//!    - 地形（`TerrainHeightsData::sample_at_world_xz`）；
//! 4. `CachedDecoratorTransforms::insert`（0x140a1ada0，有限性断言）。
//!
//! 精确位置/旋转公式尾段 U（证据：decomp 全文本）；本实现按机器码调用面
//! 结构落地（墙曲线 + 屋顶触发体 + 地形采样）。

use bevy_ecs::prelude::*;

use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::cached_decorator_transforms::{
    CachedDecoratorTransform, CachedDecoratorTransforms,
};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use glam::{Quat, Vec3};

/// `cache_decorator_transforms` @0x1408153d0（O 结构；公式尾段 U）。
pub fn cache_decorator_transforms(
    mut cache: ResMut<CachedDecoratorTransforms>,
    storage: Res<DecoratorStorage>,
    walls: Res<PublicWalls>,
    roof_trigger_volumes: Query<&RoofTriggerVolume>,
    terrain: Res<TerrainHeightsData>,
) {
    cache.swap_buffers_and_clear();
    for (wall, bucket) in storage.iter_walls() {
        let wall_state = walls.get(wall);
        for (_id, info) in bucket.iter_maybe_mut() {
            let transform = compute_decorator_transform(
                info,
                Some(wall),
                wall_state,
                &walls,
                &roof_trigger_volumes,
                &terrain,
            );
            let Some(transform) = transform else { continue };
            cache.insert(info.decorator_id, transform);
        }
    }
    // O：原生迭代 `DecoratorStorage` 全存储（墙 + 地形 + 楼梯）；旧实现只
    // 遍历墙桶，地形/楼梯装饰永远不会进入缓存（已修）。
    for bucket in storage.iter_terrain_decorators() {
        for info in bucket.iter_maybe_mut().map(|(_, info)| info) {
            let Some(transform) = compute_decorator_transform(
                info,
                None,
                None,
                &walls,
                &roof_trigger_volumes,
                &terrain,
            ) else {
                continue;
            };
            cache.insert(info.decorator_id, transform);
        }
    }
    for bucket in storage.iter_stair_decorators() {
        for info in bucket.iter_maybe_mut().map(|(_, info)| info) {
            let Some(transform) = compute_decorator_transform(
                info,
                None,
                None,
                &walls,
                &roof_trigger_volumes,
                &terrain,
            ) else {
                continue;
            };
            cache.insert(info.decorator_id, transform);
        }
    }
}

/// 逐装饰世界变换（O 调用面；公式尾段 U——精确公式见 decomp 全文本）。
#[allow(clippy::too_many_arguments)]
fn compute_decorator_transform(
    info: &country_core::resources::walls::decorator::DecoratorInfo,
    wall: Option<WallId>,
    wall_state: Option<&PublicWallState>,
    walls: &PublicWalls,
    roof_trigger_volumes: &Query<&RoofTriggerVolume>,
    terrain: &TerrainHeightsData,
) -> Option<CachedDecoratorTransform> {
    match info.dst_tag() {
        0 | 1 => {
            // 墙目的（O）：墙曲线坐标 + XZ 旋转（from_xz_forward_vector/
            // offset/to_quat 调用面）。**公式尾段仍是 U**（decomp 0x1408153d0
            // 61KB 内联；u/v→曲线坐标与附着高未逐字节闭合）。显式返回 None，
            // 不再以 `Vec3::ZERO`/恒等旋转冒充变换（W6）。
            let _ = (wall, wall_state, walls);
            None
        }
        2 => {
            // 屋顶目的（O）：roof_entity + Roof + RoofTriggerVolume::
            // get_position_from_mesh_uv + roof_decorator_normal。uv 计算 U；
            // `Vec2::ZERO` 是伪造值 → 显式未恢复（W7）。
            let _ = (wall, roof_trigger_volumes);
            None
        }
        3 => {
            // 地形目的（O）：`DecoratorInfo+0x00` 前 8B = Vec2(x,z)（与
            // ui_change_stair_height 地形处理器 0x1407ae060 同源读面），
            // `TerrainHeightsData::sample_at_world_xz` 得 y。
            let (x, z) = decorator_terrain_xz(info);
            let y = terrain.sample_at_world_xz(x, z);
            Some(CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::new(x, y, z),
                scale: Vec3::ONE,
            })
        }
        // 楼梯目的（tag 4）：O 跳表有专用处理器；其位置公式（楼梯拓扑
        // 采样）尚未闭合，显式返回 None（U），不以零变换冒充。
        _ => None,
    }
}

/// 地形装饰位置（O：`DecoratorInfo+0x00` 前 8B = Vec2(x,z)）。
pub fn decorator_terrain_xz(
    info: &country_core::resources::walls::decorator::DecoratorInfo,
) -> (f32, f32) {
    (
        f32::from_bits(info.dst_wall as u32),
        f32::from_bits((info.dst_wall >> 32) as u32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;

    #[test]
    fn terrain_xz_reads_packed_dst_wall() {
        let info = DecoratorInfo {
            dst_wall: u64::from(1.5f32.to_bits()) | (u64::from((-2.25f32).to_bits()) << 32),
            ..DecoratorInfo::default()
        };
        assert_eq!(decorator_terrain_xz(&info), (1.5, -2.25));
    }
}

/// 屋顶法线辅助（行 451 内联；U 公式——decomp 内联面）。
#[allow(dead_code)]
fn roof_decorator_normal(_position: Vec3, _roof: &Roof) -> Vec3 {
    Vec3::Y
}

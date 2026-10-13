//! draw_path.rs — 路径绘制资源（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`systems/paths/draw_path.rs`；PDB 命名实体：
//! `DrawState` / `PathRastersResources`。
//! ECS 签名（O）中 `PathRastersResources` 作为**裸系统参数**出现
//! （非 `Res<...>`）——即自定义 `SystemParam`（`nani_update_preplaced_flower` 读路径光栅）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use bevy_ecs::system::SystemParam;

use crate::resources::world_raster::RasterBufferRef;
use crate::typed_gap::Gap;

/// 内部 typed-gap 载体：路径光栅数据（原始 `PathRastersResources` 内部形状 U；
/// 此处以 `Res<PathRasterData>` 承载采样入口）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PathRasterData(pub Gap);

/// 路径光栅系统参数（O：裸 `SystemParam` 出现；内部字段 U）。
/// `bilinear_sample_at_world_xz<RasterBufferRef<512>>`（O）用于花/路径共处判定。
#[derive(SystemParam)]
pub struct PathRastersResources<'w> {
    /// typed gap：路径光栅数据（原参形状 U，此为推断载体）。
    pub data: bevy_ecs::change_detection::Res<'w, PathRasterData>,
}

/// 路径绘制状态（O：命名实体；未消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DrawState(pub Gap);

/// 光栅引用（O：与 `resources::world_raster::RasterBufferRef` 同族）。
pub type PathRasterRef = RasterBufferRef<512>;

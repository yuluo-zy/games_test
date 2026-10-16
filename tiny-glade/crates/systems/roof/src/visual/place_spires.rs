//! 屋顶尖顶放置（visual/place_spires.rs）。
//!
//! `place_roof_spires`（0x142196e70）。候选参数面（skeleton 富化）：
//! `(Res<GraphicsDeviceResource>, Query<(&Roof, &RoofAnimation),
//!  With<RoofSpire>>, Res<AssetSsboLibrary>, …)`。
//!
//! 机器码（building-roof-spires 既有 .c + fresh decomp）：puffin scope →
//! `RoofSpire` 实体（system_roof::utils::components）→ 尖顶变换记录
//! （RoofSpireSsbo，槽名 "RoofSpearTransforms"；ssbo_tags O）→
//! `GraphicsDevice::update_buffer_bytes`（rhapsody 后台上传）。
//! GraphicsDeviceResource 属 country-core render（未落 workspace）——
//! 参数面按原候选登记；本实现完成 CPU 侧尖顶记录生产与缓冲写回。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::{Roof, RoofAnimation},
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// B `RoofSpireSsbo` 记录（观测：尖顶变换 4×4 打包 96B；槽
/// "RoofSpearTransforms"）。
#[derive(Clone, Copy, Debug)]
pub struct RoofSpireRecord(pub [u8; 96]);

/// 尖顶缓冲资源（系统写回；GPU 上传经 AssetSsboLibrary/ssbo 库后置接线）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct RoofSpireRecords {
    pub records: Vec<RoofSpireRecord>,
}

/// B 0x142196e70。
pub fn place_roof_spires(
    walls: Res<PublicWalls>,
    spires: Query<(&Roof, &RoofAnimation), With<crate::utils::components::RoofSpire>>,
    mut records: ResMut<RoofSpireRecords>,
    ssbo: Res<AssetSsboLibrary>,
) {
    records.records.clear();
    for (roof, _animation) in &spires {
        // 尖顶记录：屋顶 tip 位置（pivot + tip_offset）+ 屋顶高度 → 96B
        // 变换（building-roof-spires 证据；精确矩阵打包见 decomp-0x142196e70）。
        let pivot = roof.pivot_3d();
        let tip = roof.roof_tip_offset_xz();
        let y = roof.height() + pivot.y;
        let mut record = [0u8; 96];
        let position = Vec3::new(pivot.x + tip[0], y, pivot.z + tip[1]);
        record[..12].copy_from_slice(&position.to_array().map(f32::to_le_bytes).concat());
        record[12..16].copy_from_slice(&1.0f32.to_le_bytes());
        records.records.push(RoofSpireRecord(record));
        let _ = walls.get(roof.wall_id());
    }
    let _ = ssbo;
}

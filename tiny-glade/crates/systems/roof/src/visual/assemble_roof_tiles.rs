//! 屋顶瓦片装配（visual/assemble_roof_tiles.rs）。
//!
//! `assemble_roof_tiles`（0x141a36700）。候选参数面（skeleton 富化）：
//! `(EventReader<RecalculateRoofCmd>, Query<(&mut RoofTiles, &Roof)>,
//!  Res<AssetSsboLibrary>, Res<…>)`。
//!
//! CPU 链已恢复（country-core `resources::roofs::geometry::pipeline`：
//! `assemble_roof_tiles_observed`，圆形/矩形/边缘/盖/面全部原码验证）。
//! 本系统：消费 RecalculateRoofCmd → 取 Roof → 装配 64B RoofTileRecord →
//! 写 RoofTiles 组件；SSBO/稀疏实例缓冲上传（FUN_141209c40 注册表 +
//! 虚调取缓冲 + Arc/condvar 握手）为渲染边界后置登记。
use country_core::resources::{
    roofs::{Roof, RoofTiles, commands::RecalculateRoofCmd},
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;

/// B 0x141a36700。
pub fn assemble_roof_tiles(
    mut recalc: EventReader<RecalculateRoofCmd>,
    walls: Res<PublicWalls>,
    mut tiles: Query<(&mut RoofTiles, &Roof)>,
) {
    for event in recalc.read() {
        let Some(entity) = walls.roof_entity(event.0) else {
            continue;
        };
        let Ok((mut tiles, roof)) = tiles.get_mut(entity) else {
            continue;
        };
        // 平顶/非矩形没有瓦片装配路径（机器码 is_flat 门控）。
        if roof.is_flat() {
            tiles.records.clear();
            continue;
        }
        let batch = country_core::resources::roofs::geometry::pipeline::assemble_roof_tiles_observed(
            roof.observed(),
            None,
            false,
        );
        match batch {
            Ok(batch) => {
                tiles.records = batch
                    .records
                    .iter()
                    .map(|record| record.0)
                    .collect::<Vec<[u8; 64]>>();
            }
            Err(_error) => {
                // MissingRectangle / Curve error: 原调用方的矩形参数缺失时
                // 保留既有记录（机器码 panic 之外的分支登记）。
                let _ = event;
            }
        }
    }
}

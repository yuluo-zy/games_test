//! 矩形屋顶装配（utils/assemble_rectangular_roof.rs）。
//!
//! `assemble_rectangular_roof<assemble_roof_tiles::closure_env$0>`
//! （0x1421aeef0）：矩形瓦片装配的泛型实例（closure = 记录消费者）。
//! CPU 链已恢复 = `pipeline::assemble_roof_tiles_observed` 的矩形分支
//! （rectangular_edges/caps/faces，4514+ 案例验证）；本文件提供原名调用面。
use country_core::resources::roofs::geometry::{pipeline::RoofBuildError, tiles::RoofTileRecord};

/// 矩形屋顶瓦片装配（0x1421aeef0 的 CPU 链 + closure 消费）。
pub fn assemble_rectangular_roof(
    roof: &[u8; 88],
    rectangle: &[u8; 24],
    special_mode: bool,
    mut consume: impl FnMut(&RoofTileRecord),
) -> Result<usize, RoofBuildError> {
    let batch =
        country_core::resources::roofs::geometry::pipeline::assemble_roof_tiles_observed(
            roof,
            Some(rectangle),
            special_mode,
        )?;
    let count = batch.records.len();
    for record in &batch.records {
        consume(record);
    }
    Ok(count)
}

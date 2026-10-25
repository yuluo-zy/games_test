//! 圆形屋顶装配（utils/assemble_circular_roof.rs）。
//!
//! `assemble_circular_roof<assemble_roof_tiles::closure_env$0>`
//! （0x1421b3120）：圆形瓦片装配的泛型实例（closure = 记录消费者）。
//! CPU 链已恢复 = `tiles::assemble_circular_from_observed`
//! （11,467 片圆形瓦 0 差异，evidence/roof-tiles）；本文件提供原名的
//! 调用面（closure 泛型实例化为记录追加）。
use country_core::resources::roofs::geometry::{surface::CurveError, tiles::{self, RoofTileRecord}};

/// 圆形屋顶瓦片装配（0x1421b3120 的 CPU 链 + closure 消费）。
pub fn assemble_circular_roof(
    roof: &[u8; 88],
    special_mode: bool,
    mut consume: impl FnMut(&RoofTileRecord),
) -> Result<usize, CurveError> {
    let records = tiles::assemble_circular_from_observed(roof, special_mode)?;
    let count = records.len();
    for record in &records {
        consume(record);
    }
    Ok(count)
}

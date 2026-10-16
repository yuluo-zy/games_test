//! 屋顶视觉系统（visual/mod.rs）。
pub mod assemble_roof_tiles;
pub mod generate_roof_stone_floor_and_roof_bottom;
pub mod place_spires;
pub mod roof_animation;
pub mod switch_roof_type;

/// `visual/place_support_beams.rs`：源图 0 业务（内联/胶水），以
/// construct_lintels 的内联实例为准，无独立实现。
pub mod place_support_beams {}

/// `visual/roof_snow.rs`：源图 0 业务（胶水），无独立实现。
pub mod roof_snow {}

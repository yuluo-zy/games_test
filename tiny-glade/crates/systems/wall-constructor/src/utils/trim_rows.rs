//! 行修剪（utils/trim_rows.rs）。
//!
//! `trim_rows`（0x14123eef0）/`above_to_below`（0x14123fab0）/
//! `below_to_above`（0x14123fb70）：split_heightmap_by_threshold
//! （0x141fe0750）阈值穿越/横向量化/窄条删除/扩宽；32B 行输出
//! （G04 冻结 200 例全部 32 字节原输出通过）。
//! CPU 链已恢复 = country-core recovered_building::wall_constructor::trim_rows。
pub use country_core::recovered_building::wall_constructor::trim_rows;

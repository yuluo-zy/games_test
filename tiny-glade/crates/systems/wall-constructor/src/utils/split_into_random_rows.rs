//! 随机行拆分（utils/split_into_random_rows.rs）。
//!
//! `split_into_random_rows`（0x14123e110，G04 冻结 160 原指令输入通过）：
//! 0.495 上限、0.21/span jitter、0.046/height 去近边；fastrand 1.8.0。
//! CPU 链已恢复 = country-core systems::wall::construction::bricks::
//! split_into_random_rows。
pub use country_core::systems::wall::construction::bricks::split_into_random_rows;

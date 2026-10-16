//! 原 `rhapsody::rg::resource_info` 的访问掩码 → 资源用途转换（0x140bd4b00 / 0x140bd49d0）。
//! 实现唯一一份在 `raw::resource_usage`；此处只保留 `rg::resource_usage` 公开路径。
pub use crate::raw::resource_usage::*;

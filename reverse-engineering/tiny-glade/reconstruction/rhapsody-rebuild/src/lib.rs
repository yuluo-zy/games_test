//! 整工程候选入口，保留 PDB 已观察到的原模块树；实现状态及未对齐行为见 MODULE_STATUS.md。
pub mod app;
pub mod raw;
pub mod rg;
pub use rg::executor::{ExecutionBackend, ExecutionChain};

pub mod leek;

// 原着色器资产格式及编译请求的恢复模块；验证边界见 shader-toolchain 报告。
pub mod shader_pipeline;
pub mod shader_compiler;

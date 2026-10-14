//! B 版已恢复的渲染图、资源与 Vulkan 调用链；原作等价性范围见 README。
pub mod raw;
pub mod rg;
pub use rg::executor::{ExecutionBackend, ExecutionChain};
pub use shader_compiler;
pub use shader_pipeline;

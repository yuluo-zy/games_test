//! B 版已恢复的渲染图、资源与 Vulkan 调用链；原作等价性范围见 README。
pub mod command;
pub mod bindless;
pub mod device;
pub mod error;
pub mod leek;
pub mod log_ring;
pub mod pipeline;
pub mod raw;
pub mod resource;
pub mod rg;
pub mod sync;
pub mod transfer;
pub use error::{Error, ErrorKind, Result};
pub use log_ring::{dump_log, dump_log_vec, init_logging, print_log_dump};
pub use rg::executor::{ExecutionBackend, ExecutionChain};
pub use shader_compiler;
pub use shader_pipeline;

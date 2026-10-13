//! country-core pass 声明层（render/pass_builder/*）。
//!
//! 依据 B PDB 骨架与机器码恢复；对齐原骨架模块树（skeleton/B/crates/country-core/src/render/pass_builder/）。
//! 本层声明 compute/raster pass 的资源访问与绘制/dispatch，实际构图/录制交给
//! 复用依赖 `rhapsody::rg`（API 边界见 reconstruction/rhapsody-rebuild）。
//!
//! 模块所有权（跨包契约）：
//! - WP1（本层）拥有：`ComputePass`、`DrawBuilder`、`PassResources`、`UniformBinder`、
//!   `RasterPass`/`DrawPacket`/`DrawStream`、`ShaderUri`、pass 资源访问绑定类型。
//! - 游戏 SSBO 结构体（`*Ssbo`）由 WP2 缓冲 / WP3 nani / WP4 持有，本层仅作泛型参数引用。

pub mod bind_resources;
pub mod compute_pass_builder;
pub mod draw_builder;
pub mod draw_list;
pub mod draw_stream;
pub mod framebuffer_binding;
pub mod pass_resource_access;
pub mod pass_resources;
pub mod shader_uri;
pub mod uniform_binder;

pub use bind_resources::BindGameResources;
pub use compute_pass_builder::{ComputeDispatchApi, ComputeDispatchIndirectApi, ComputePass};
pub use draw_builder::{DrawBuilder, NoDropOnlyTake};
pub use draw_stream::{DrawKind, DrawPacket, DrawPassRenderCheck, RasterPass, TextureBindingKey};
pub use pass_resources::PassResources;
pub use shader_uri::{AsComputeShaderSource, ShaderUri};
pub use uniform_binder::UniformBinder;

/// 绘制/dispatch 模式判别（原机器码观测值，Rust 枚举名未恢复 —— typed gap）。
/// 语义见 contracts/draw_builder.md §4 与 compute_pass_builder.md §5。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawMode(pub u32);

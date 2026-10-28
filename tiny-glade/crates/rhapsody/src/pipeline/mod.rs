//! Bounded background pipeline creation, persistent driver data and explicit
//! publication at a frame boundary. The compiler is owned by the renderer,
//! never by Device, so cached pipelines cannot form a Device ownership cycle.

internal_mod!(shader);
pub(crate) mod shader_interface;
pub use shader::{
    BindlessShaderAbi, ComputePipelineDesc, DynamicRenderingFormats, PipelineCache, PipelineCacheData, PipelineLayout,
    RasterPipelineDesc, RasterPipelineOpts, ShaderLibrary, ShaderModule, VulkanPipeline,
    get_vk_blend_mode, reflect_spirv_entries, vertex_input_from_reflection,
};
mod request;
mod compiler;
mod hot;
mod disk;
pub use request::{CompilerConfig, CompilePriority, RequestOptions, PipelineTicket, PipelinePoll, CompilerStats};
pub use compiler::AsyncPipelineCompiler;
pub use hot::HotPipeline;
pub use disk::{DiskCacheStatus, PersistentPipelineCache};

#[cfg(test)]
mod tests;

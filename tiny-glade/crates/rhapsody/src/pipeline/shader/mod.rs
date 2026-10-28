//! Shader assets and reflection, pipeline layouts, pipelines and the pipeline
//! cache. SPIR-V parsing uses rspirv; reflection comes from the stored shader
//! container catalog.
use crate::accounting::{
    AllocatorStats, CpuMemoryReservation, cpu_vec_bytes, reserve_shared_cpu_metadata,
};
use crate::collections::temp_list::TempList;
use crate::device::Device;
use crate::Result;
use ash::vk;
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub(crate) use interface::{canonical_type, reflected_stage};
use layout::invalid_shader;
use crate::device::observe_vulkan;
mod pipeline_cache;
mod pipeline_key;
mod reflection;
mod bindless_abi;
pub use bindless_abi::BindlessShaderAbi;
pub(crate) use pipeline_cache::{MAX_PIPELINE_CACHE_BYTES, valid_pipeline_cache_data};
pub use pipeline_cache::{PipelineCache, PipelineCacheData};
#[cfg(test)]
use pipeline_key::visit_dynamic_rendering_key;
pub(crate) use pipeline_key::{
    raster_pipeline_key, raster_pipeline_key_len, raster_pipeline_key_matches,
};
pub(crate) use reflection::reflect_spirv_entries_used;
pub use reflection::{
    InterfaceError, ReflectedDescriptor, ReflectedEntry, ReflectedLocation, ReflectedPushConstant,
    compiled_pipeline_entries, original_panics, reflect_spirv_entries, validate_compute_reflection,
};
mod layout;
mod raster;
mod pipeline;
mod interface;
mod library;
pub use layout::{ShaderModule, PipelineLayout};
pub use raster::{DynamicRenderingFormats, RasterTargetSignature, RasterPipelineDesc, RasterPipelineOpts, get_vk_blend_mode, map_vertex_attribute_format, vertex_input_from_reflection};
pub use pipeline::{ComputePipelineDesc, VulkanPipeline};
pub use interface::{ShaderInterfacePlan, ReflectedShader, stored_pipeline_entry};
pub use library::{ShaderLibrary, RasterProgram};


#[cfg(test)]
mod raster_opts_tests;

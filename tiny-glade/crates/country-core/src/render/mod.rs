//! 第 0 层文件的挂载点（相对路径指向真实源文件，不拷贝）。
pub mod declare_probe;
pub mod asset_ack;
pub mod extract;
pub mod gpu_readback;
pub mod device;
pub mod dirty_index_range;
pub mod frame_constants;
pub mod mesh;
pub mod shadowed_buffer;
pub mod ssbo;
pub mod utils;
pub mod vao;
pub mod view_constants;
pub mod pass_builder {
    pub mod bind_resources;
    pub mod compute_pass_builder;
    pub mod draw_builder;
    pub mod draw_list;
    pub mod draw_stream;
    pub mod framebuffer_binding;
    pub mod pass_resource_access;
    pub mod pass_resources;
    pub mod rg_resources;
    pub mod shader_uri;
    pub mod uniform_binder;
}
pub mod atmosphere;
pub mod adaptive_render_state;
pub mod directional_light;
pub mod execution_host;
pub mod load_texture;
pub mod mesh_atlas;
pub mod nani;
pub mod normals_debug;
pub mod render_config;
pub mod rg_helpers;
pub mod shader;
pub mod sparse_instance_buffer;

pub mod indirect;
pub mod raster_single;

/// 共享测试占位设备（CPU 测试用，替代 `rhapsody_contract::GraphicsDevice` 的旧 tuple 占位）。
#[cfg(any(test, feature = "test-support"))]
pub mod test_device;

pub mod render_temporal_state;
pub use render_temporal_state::{FrameCameraMatrices, RenderTemporalState};

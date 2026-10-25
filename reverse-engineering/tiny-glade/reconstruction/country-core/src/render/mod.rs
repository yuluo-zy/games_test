//! Render-facing core types recovered from the original Tiny Glade PDB skeleton,
//! assembled from the five work packages (B version).
//!
//!   - `pass_builder/`  (wp1) — ComputePass / DrawBuilder / RasterPass / PassResources /
//!                             UniformBinder / ShaderUri / pass resource access
//!   - `nani/`          (wp3) — GPU-driven instance world + instance data + mesh atlas
//!   - flat files       (wp2) — ssbo / shadowed_buffer / sparse_instance_buffer / vao /
//!                             raster_single / dirty_index_range / uniform / indirect
//!   - flat files       (wp4) — mesh / shader / render_config / view_constants /
//!                             frame_constants / directional_light / atmosphere / fsr /
//!                             debug_info_buffers / adaptive_render_state / load_texture
//!   - `mesh_atlas.rs`  (new) — `render::mesh_atlas::AtlassedMeshName` (referenced by wp3)
//!   - `utils.rs`       (new) — `render::utils::calculate_hash` (referenced by wp1)
//!
//! Marking legend: O = observed (machine code / PDB), I = inferred, U = unknown (typed gap).

pub mod pass_builder;
pub mod nani;

// WP2 flat modules.
pub mod dirty_index_range;
pub mod indirect;
pub mod raster_single;
pub mod shadowed_buffer;
pub mod sparse_instance_buffer;
pub mod ssbo;
pub mod uniform;
pub mod vao;

// WP4 flat modules.
pub mod adaptive_render_state;
pub mod atmosphere;
pub mod debug_info_buffers;
pub mod directional_light;
pub mod frame_constants;
pub mod fsr_plugin;
pub mod load_texture;
pub mod mesh;
pub mod render_config;
pub mod shader;
pub mod rg_helpers;
pub mod view_constants;

// Assembly-added gap modules.
pub mod mesh_atlas;
pub mod utils;

/// Bevy resource wrapping the raw graphics device.
///
/// O（0x140987ca0）：`Deref::deref` 是 `[self] + 0x10`，即 `ArcInner` 的数据偏移 —— 资源里放的是
/// `Arc<GraphicsDevice>`（旧版直接存设备本体是错的）。设备本身是复用库 `rhapsody::raw::device::GraphicsDevice`。
#[derive(bevy_ecs::resource::Resource)]
pub struct GraphicsDeviceResource(pub std::sync::Arc<crate::rhapsody_contract::GraphicsDevice>);

impl core::ops::Deref for GraphicsDeviceResource {
    type Target = crate::rhapsody_contract::GraphicsDevice;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A physical graphics adapter as enumerated at startup.
///
/// 160 bytes / element (O: `AvailableGraphicsDevices::from_preference_order`
/// swaps 0xa0-byte elements). Exact field layout is a typed gap.
pub struct AvailableGraphicsDevice {
    /// O: opaque 160-byte record; fields unknown.
    _raw: [u64; 20],
}

/// Ordered list of available graphics devices.
pub struct AvailableGraphicsDevices(pub Vec<AvailableGraphicsDevice>);

impl AvailableGraphicsDevices {
    /// `from_preference_order(devices)`（O 0x140987cb0）：就地 `reverse()` —— 成对交换 0xa0 字节的元素，
    /// 共 `len/2` 轮；输入的排列方向与用途（I：调用方按“最不优先在前”给出，列表内部最优在前）未在该函数内体现。
    pub fn from_preference_order(mut devices: Vec<AvailableGraphicsDevice>) -> Self {
        devices.reverse();
        Self(devices)
    }
}

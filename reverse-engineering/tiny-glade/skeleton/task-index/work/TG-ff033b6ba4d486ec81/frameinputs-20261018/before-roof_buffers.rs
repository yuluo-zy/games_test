//! `roof_buffers.rs` — RoofBuffers 资源 + WoodenPlankSsbo 稀疏梁实例缓冲元素（B
//! `resources/roofs/roof_buffers.rs`）。
//!
//! 证据：`construct_timberframe_rectangle` 0x141a41ba0（2026-10-17 coregap-1 fresh 反编译）：
//! `SparseInstanceBufferWriter::push<WoodenPlankSsbo>` 写梁实例（0x20B 记录族）；槽空时
//! `AssetSsboLibrary` 查找 WoodenPlankSsbo 创建持久 `SparseInstanceBufferAllocation`。
//! `SparseSsbo` 的 `CHUNK_COUNT` 按 sparse_instance_buffer 文档（"Brick/Plank 0x800 起"）
//! 取 0x800（与 `BUFFER_LEN == CHUNK_COUNT * 32` 一致）。
use bevy_ecs::resource::Resource;
use glam::Vec3;

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::sparse_instance_buffer::{SparseInstanceBuffer, SparseSsbo, SLOTS_PER_CHUNK};
use crate::render::ssbo::{
    StorageElement, rhapsody_boundary::GraphicsDevice,
};

/// 屋顶 GPU 缓冲句柄容器（B `RoofBuffers`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RoofBuffers {
    /// 字段语义 U：渲染域 SSBO 句柄容器待取证。
    pub(crate) _opaque: (),
}

/// 木框架梁实例记录（O：`resources::roofs::roof_buffers::WoodenPlankSsbo` 类型名串
/// 0x142ff8cf0；0x20B 记录族——`construct_timberframe_rectangle`/`circle` 逐梁
/// `SparseInstanceBufferWriter::push` 写面。字段 = 位置 + 截面尺寸 0.2/0.18 +
/// `orientation_mat3`/`rotation_from_normal` 朝向；**字段序 I（渲染实例合同待证，
/// 以 `{position, size, rotation}` 承载，不冒充字节序）**）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WoodenPlankSsbo {
    /// 梁中心位置（世界/墙空间）。
    pub position: Vec3,
    /// 截面尺寸（O：垂直梁 0.2、等距梁 0.18、裙层 0.2）。
    pub size: f32,
    /// 朝向四元数（O：`orientation_mat3` 派生；分量序 I）。
    pub rotation: glam::Quat,
}

// SAFETY: 全零是合法实例（sparse 影子数组 memset 零初始化；`SparseSsbo` 契约）。
unsafe impl bytemuck::Zeroable for WoodenPlankSsbo {}
unsafe impl bytemuck::Pod for WoodenPlankSsbo {}

impl StorageElement for WoodenPlankSsbo {
    const NAME: &'static str = "WoodenPlankSsbo";
    const BUFFER_LEN: usize = 0x800 * SLOTS_PER_CHUNK;
}

// SAFETY: 全零合法（同 `SparseSsbo` 契约）。
unsafe impl SparseSsbo for WoodenPlankSsbo {
    /// O：Plank 类 0x800 起（sparse_instance_buffer 文档）。
    const CHUNK_COUNT: usize = 0x800;
}

impl SsboType for WoodenPlankSsbo {
    type Storage = SparseInstanceBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        SparseInstanceBuffer::<Self>::new_storage_buffer(device, initial)
    }
}

impl WoodenPlankSsbo {
    /// 便捷构造（I：渲染实例合同待证，字段序不冒充）。
    pub fn new(position: Vec3, size: f32, rotation: glam::Quat) -> Self {
        Self {
            position,
            size,
            rotation,
        }
    }
}

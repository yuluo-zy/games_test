//! `render/device.rs` — `GraphicsDeviceResource`（B `country_core::render::GraphicsDeviceResource`）。
//!
//! 证据：`gpu_flush_buffers` 0x140de50a0（roof RESTORE 2026-10-17 登记：`AssetSsboLibrary::
//! get_mut<RoofTileSsbo>` → `GraphicsDeviceResource::deref` → `SparseInstanceBuffer::
//! gpu_flush<RoofTileSsbo>` + tick）；`compute_pass_builder.rs` 记 `+0x188` 为
//! `HashMap<RenderPassDesc, RenderPass>` 管线缓存（语义 U，登记 GAP）。
//!
//! 本资源持 rhapsody 设备并 `Deref` 到 `render::ssbo` 的设备 trait（`gpu_flush` 的
//! `&dyn GraphicsDevice` 实参面）；管线缓存槽 +0x188 未落库（不冒充）。

use bevy_ecs::resource::Resource;
use core::ops::Deref;

/// GPU 设备资源（B `GraphicsDeviceResource`；`gpu_flush_buffers`/`ComputePass::new`
/// 的 `Res<GraphicsDeviceResource>` 面）。
#[derive(Resource, Clone)]
pub struct GraphicsDeviceResource {
    /// rhapsody 设备（实现 `render::ssbo::rhapsody_boundary::GraphicsDevice` trait）。
    device: crate::rhapsody_contract::GraphicsDevice,
    /// @+0x188：`HashMap<RenderPassDesc, RenderPass>` 管线缓存（O 偏移；语义 U，
    /// compute_pass_builder.rs 登记 GAP——不造字段）。
    _pass_cache_gap: core::marker::PhantomData<()>,
}

impl GraphicsDeviceResource {
    /// 从真实 rhapsody 设备构造（WIRING 接入面）。
    pub fn new(device: crate::rhapsody_contract::GraphicsDevice) -> Self {
        Self {
            device,
            _pass_cache_gap: core::marker::PhantomData,
        }
    }

    /// 底层 rhapsody 设备（`SparseInstanceBuffer::gpu_flush` 等调用面）。
    pub fn device(&self) -> &crate::rhapsody_contract::GraphicsDevice {
        &self.device
    }
}

impl Deref for GraphicsDeviceResource {
    /// O：`GraphicsDeviceResource::deref`（0x140de50a0 调用面）返回设备。
    type Target = crate::rhapsody_contract::GraphicsDevice;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

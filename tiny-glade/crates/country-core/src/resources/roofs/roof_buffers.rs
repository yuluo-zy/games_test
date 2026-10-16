//! `roof_buffers.rs` — RoofBuffers 资源（B `resources/roofs/roof_buffers.rs`，0 函数）。
//!
//! inventory 记录 0 原函数、无 use 依赖；本文件仅类型声明（SSBO 缓冲句柄容器）。
//! 字段语义 U（渲染域消费端未挖）；登记待取证，不编造字段。
use bevy_ecs::resource::Resource;

/// 屋顶 GPU 缓冲句柄容器（B `RoofBuffers`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RoofBuffers {
    /// 字段语义 U：渲染域 SSBO 句柄容器待取证。
    pub(crate) _opaque: (),
}

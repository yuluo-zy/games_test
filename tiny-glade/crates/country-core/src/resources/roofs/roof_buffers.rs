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

/// 木框架 plank 实例缓冲（O：`resources::roofs::roof_buffers::WoodenPlankSsbo`
/// 类型名串 0x142ff8cf0；`timberframe` crate 两构造系统写入、render 域
/// `draw_callbacks_a` 以预设 9/10 消费（0x141631c90 引用面）。布局 U
/// （re-analyst-A 契约 §2：梁实例记录字段明细 U，typed gap）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WoodenPlankSsbo {
    /// 梁实例记录（0x20B 条目族；字段 U）。
    pub(crate) _opaque: (),
}

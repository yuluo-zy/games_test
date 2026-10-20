//! rhapsody 执行链宿主（D 组接入，WIRING §2.7 / §5.2 步骤 4）。
//!
//! 真实组合：构图走 `rg::graph::RenderGraph`（`create_raw_resource`/`add_pass(..).render_with_context(..)`/
//! `export`/`batch_per_pass` → `build()` → `into_parts()`），后端 `rg::vulkan::VulkanGraphBackend`，
//! 执行走 `rg::vulkan::VulkanGraphSession`，复用提交事务与失败帧回收。
//!
//! GPU 真机执行后置（协调简报 §D 规则 4）；本模块验收 = 真实调用接线 + `cargo check` + 原侧入口核对
//! （`render_frame.rs::render` 0x1404e8010 的调用点映射见 WIRING §2.7 表）。
//!
//! # 状态（2026-10-19，G2）：保留，但**不是生产路径**
//!
//! workspace 内没有任何调用者。生产唯一执行面是 tiny-glade `CoreRenderBridge`
//! （`render_bridge.rs`）：它持有同一种 `VulkanGraphSession`，并额外绑定宿主
//! `BufferHost`/`TextureHost`/`ShaderHost`/`RasterShared` 与 App 资产库身份。
//! 本模块的 `RhapsodyExecutionHost` 若直接使用会得到一套没有这些宿主的会话，
//! 因此不作为构图/提交入口。保留它是因为它把 `BuildGraph`→`SessionFrameReceipt`
//! 的调用形状与 `pass_api_from_context` 的裸句柄转换记录在案，可作为 rhapsody
//! 边界核对材料；新增接线一律走 `CoreRenderBridge`，不要再创建第二条会话。

use std::sync::Arc;

use ash::vk::Handle as _;

use crate::rhapsody_contract::GraphicsDevice;
use crate::rhapsody_types::{
    AccessTable, AshCommandBackend, PassContext, RhapsodyVulkanGraphBackend,
};
use rhapsody::rg::{
    graph::BuiltGraph,
    vulkan::{SessionFrameReceipt, VulkanGraphSession},
};

/// 游戏宿主复用生产图会话；部分提交、真实终态及延迟回收只有一条实现路径。
pub struct RhapsodyExecutionHost {
    session: VulkanGraphSession,
}
impl RhapsodyExecutionHost {
    pub fn new(device: &GraphicsDevice) -> rhapsody::Result<Self> {
        let backend =
            RhapsodyVulkanGraphBackend::new(device.inner().clone(), AccessTable::original())?;
        Ok(Self {
            session: VulkanGraphSession::new(backend),
        })
    }
    pub fn begin_frame(&mut self) -> rhapsody::Result<rhapsody::rg::graph::RenderGraph> {
        self.session.begin_frame()
    }
    /// BuiltGraph 同时移交回调和退役动作；失败批次不会丢失其在飞所有权。
    pub fn execute_graph(&mut self, graph: BuiltGraph) -> rhapsody::Result<SessionFrameReceipt> {
        self.session.execute_frame(graph)
    }
    pub fn complete_primary(&mut self) -> rhapsody::Result<()> {
        self.session.collect_completed_frames().map(|_| ())
    }
    pub fn wait_idle_and_recycle_resources(&mut self) -> rhapsody::Result<()> {
        self.session.wait_submits_done(u64::MAX)?;
        self.session
            .backend
            .device
            .wait_idle_and_recycle_resources()
    }
}

/// 构图入口（WIRING §2.7）：`rg::graph::RenderGraph::new()`。调用方随后
/// `create_raw_resource`/`add_pass(name).add_resource_access(..).render_with_context(..)`/`export(..)`/
/// `batch_per_pass(..)` → `build()` → `into_parts()` 得到 `(GraphDeclaration, callbacks, retire)`。
pub fn new_render_graph() -> rhapsody::rg::graph::RenderGraph {
    rhapsody::rg::graph::RenderGraph::new()
}

/// 在 pass 回调里构造真实 `RenderPassApi`（WIRING §3.1 #5）。
/// 命令缓冲经 `PassContext::command.handle()` 的裸句柄构造 `vk::CommandBuffer`
/// （executor.rs:540 `vk::CommandBuffer::from_raw(command)`）。
/// 返回后即可调用真实 `RenderPassApi::dispatch_groups_3d(x, y, z)`（pass_api.rs:595）等命令面。
pub fn pass_api_from_context<'b>(
    backend: &'b mut AshCommandBackend<'b>,
    context: &PassContext<'_>,
) -> rhapsody::rg::pass_api::RenderPassApi<'b, AshCommandBackend<'b>> {
    rhapsody::rg::pass_api::RenderPassApi::new(
        backend,
        ash::vk::CommandBuffer::from_raw(context.command.handle()),
        0,
        false,
    )
}

/// 取图内逻辑句柄的原始索引（WIRING §1：`ResourceHandle::raw()` = 图内逻辑索引）。
pub fn resource_raw(handle: rhapsody::rg::resource::ResourceHandle) -> u32 {
    handle.raw()
}

/// 执行链的 PGO 统计投影（WIRING §2.7 `last_gpu_frame_ns`：`GraphExecution::stats`）。
pub fn last_frame_ns(execution: &SessionFrameReceipt) -> u64 {
    execution.stats.submit_ns.max(execution.stats.record_ns)
}

/// 该模块可用的真实设备 Arc（供宿主外部接线用）。
pub fn device_of(device: &GraphicsDevice) -> Arc<rhapsody::device::Device> {
    device.inner().clone()
}

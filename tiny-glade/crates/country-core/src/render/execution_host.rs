//! rhapsody 执行链宿主（D 组接入，WIRING §2.7 / §5.2 步骤 4）。
//!
//! 真实组合：构图走 `rg::graph::RenderGraph`（`create_raw_resource`/`add_pass(..).render_with_context(..)`/
//! `export`/`batch_per_pass` → `build()` → `into_parts()`），后端 `rg::vulkan::VulkanGraphBackend`，
//! 执行走 `rg::executor::ExecutionChain::execute_graph_with_context`。
//!
//! GPU 真机执行后置（协调简报 §D 规则 4）；本模块验收 = 真实调用接线 + `cargo check` + 原侧入口核对
//! （`render_frame.rs::render` 0x1404e8010 的调用点映射见 WIRING §2.7 表）。

use std::sync::Arc;

use ash::vk::Handle as _;

use crate::rhapsody_contract::GraphicsDevice;
use crate::rhapsody_types::{
    AccessTable, AshCommandBackend, ExecutionChain, GraphDeclaration, PassContext, RgPool,
    RhapsodyVulkanGraphBackend,
};

/// 真实 rhapsody 执行链宿主：持有设备、执行链、Vulkan 图后端与资源池。
pub struct RhapsodyExecutionHost {
    device: Arc<rhapsody::raw::device::GraphicsDevice>,
    chain: ExecutionChain,
    backend: RhapsodyVulkanGraphBackend,
    pool: RgPool,
}

impl RhapsodyExecutionHost {
    /// 从游戏侧设备包装构造；执行链只保存真实队列句柄，时间线与提交状态由设备统一管理。
    pub fn new(device: &GraphicsDevice) -> rhapsody::Result<Self> {
        let vulkan = device.inner().inner.clone();
        let backend = RhapsodyVulkanGraphBackend::new(vulkan.clone(), AccessTable::original())?;
        // family 编号可以稀疏，池边界取最大编号 + 1，不能使用不同族的数量。
        let family_count = vulkan.additional_queues.iter().map(|queue| queue.family)
            .chain(std::iter::once(vulkan.queue_family)).max().unwrap_or(0) + 1;
        let chain = ExecutionChain::new(vulkan.queue,
            vulkan.additional_queues.iter().map(|queue| queue.handle).collect(), family_count);
        Ok(Self {
            device: device.inner().clone(),
            chain,
            backend,
            pool: RgPool::default(),
        })
    }

    /// 执行一张图声明（WIRING §2.7：`execute_graph_with_context`，executor.rs:277）。
    /// `callbacks.len()` 必须等于 `graph.passes.len()`（否则 `Error::CallbackCount`，executor.rs:300）。
    /// `PassContext` 提供真实 `CommandState`（`command_state()`）与托管注册表（`managed`）。
    pub fn execute_graph(
        &mut self,
        graph: GraphDeclaration,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
    ) -> Result<crate::rhapsody_types::GraphExecution, rhapsody::rg::executor::Error> {
        let mut access = AccessTable::original();
        self.chain.execute_graph_with_context(
            graph,
            &mut self.pool,
            &mut access,
            &mut self.backend,
            callbacks,
        )
    }

    /// 结束/推进完成回收（WIRING §2.7 `end_frame`：`complete(queue, timeline)` 推进完成侧）。
    /// 主队列完成值由 `timeline.completed()` 读取（vulkan.rs 后端的 `TimelineSemaphore`）。
    pub fn complete_primary(&mut self) -> Result<(), rhapsody::rg::executor::Error> {
        let value = self
            .backend
            .timeline
            .completed()
            .map_err(|_| rhapsody::rg::executor::Error::EndFailed)?;
        self.chain.complete(
            rhapsody::raw::managed::lifecycle::Queue::Primary,
            rhapsody::raw::managed::lifecycle::Timeline(value),
        )
    }

    /// 设备空闲 + 回收资源（WIRING §2.7 `wait_idle_and_recycle_resources`：device.rs:699）。
    pub fn wait_idle_and_recycle_resources(&self) -> Result<(), ash::vk::Result> {
        self.device.wait_idle_and_recycle_resources()
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
pub fn last_frame_ns(execution: &crate::rhapsody_types::GraphExecution) -> u64 {
    execution.stats.submit_ns.max(execution.stats.record_ns)
}

/// 该模块可用的真实设备 Arc（供宿主外部接线用）。
pub fn device_of(device: &GraphicsDevice) -> Arc<rhapsody::raw::device::GraphicsDevice> {
    device.inner().clone()
}

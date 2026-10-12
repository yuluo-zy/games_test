# rhapsody 整工程重建候选

本目录依据原PDB、类型、调用关系与机器码恢复可编译Rust候选，并用原入口差分与真实Vulkan读回修正行为。进度按功能验收记录，原源码路径清单仅供结构核对。

当前已实际运行的链路包括：

- 渲染图资源声明、编译、屏障、录制、wait/signal提交和完成回收。
- 原shader容器/反射catalog生成布局、descriptor状态、compute/raster管线与缓存；原填充shader输出及raster像素读回。
- 附件视图/renderpass/framebuffer缓存与真实渲染；跨帧temporal双缓冲、resize与资源持有。
- 同一session连续3帧复用资源池、命令缓冲和时间线，失败停止及完成后释放。
- Win32隐藏窗口交换链acquire/submit/present、resize与vsync重建。
- bufferDeviceAddress、动态常量、时间戳、GPU诊断marker/debug packet与分配统计。

各链路仅对验收场景负责，原完整游戏画面、并行调度、资产/pass关联、依赖私有分叉和部分高级路径尚未证明一致。详情与难度见[功能进度表](MODULE_STATUS.md)，当前源码与历史证据输入是否一致也记录于表中。

## 2026-10-13 缺失函数补全

对照 `skeleton-enriched/B/crates/rhapsody` 骨架函数清单，按用户授权按清单顺序补全 rg/raw/managed 缺失函数约 200 个（新增候选接口已接入真实类型，不另开 candidate crate）。覆盖：

- **rg**：RenderPassApi draw/dispatch 9 命令 + 调用计数；PGO 全套（PassPgoNameId/Duration/Stats）；ExportedRenderGraphResources/ExecutedRenderGraph；Handle/ExportableResource/create_resource_handle；ResourceInfo extract_* 拆出与访问器；Parity/Temporal；RenderGraphRecorder/PassResourceRef/ensure_active_cb；VulkanGraphSession wait_submits_done/帧时长/顶层时间戳；leek wait_submits_done。
- **raw**：GraphicsDevice 候选包装（资源创建、屏障下发、扩展支持、内置采样器）+ create_raw_device/Queue；RasterPipelineOpts 29 个 setter/getter(+checked)；Buffer 持久映射 mapped_slice_mut；SubmitInfo/SubmissionEngine；instance 黑名单系列；texture/buffer/barrier/descriptor/command_buffer 访问器族。
- **managed**：GpuOwned 全族；ManagedHandle/ManagedRef/ManagedResourceChecksum；owning_device/throw_out_managed_resource/get_maybe_tracked；SwapchainInner 包装；纹理视图辅助。

交付证据见 [missing-functions-completion-2026-10-13.json](evidence/missing-functions-completion-2026-10-13.json)（含 42 个改动文件 sha256）。验证：`cargo test --locked` 69 通过 0 失败（含此前因资产路径不可编译的 device_policy 5 个）；`cargo clippy --locked --all-targets -- -D warnings` 通过；GPU 探针未重跑。

**已知剩余缺口**（按项目规则，业务逻辑无法用 LLM 重写时才去逆向取证）：setup 命令缓冲体系（get_setup_cb_submit_info/with_setup_cb）、SwapchainInner::set_fullscreen_exclusive、纹理暂存上传（init_texture_data/update_texture）；以及机制层偏差（BufferDesc 16B 布局、raw 释放延迟入桶、同步来源内部化）与 C 版 rhapsody2 重构结构，见 [MODULE_STATUS.md](MODULE_STATUS.md) 与机制差异文档。

```powershell
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked --example shader_production_gpu
cargo run --locked --example queue_graph_gpu
```

命令行主程序读取逐行JSON构图场景并输出CPU状态与API捕获，examples/two-pass.json为输入示例。实际GPU路径使用rg::vulkan::VulkanGraphBackend及VulkanGraphSession。原vk-sync-fork0.4.0本地分叉来源未确认，访问码映射仍通过AccessTable显式输入。原gpu-allocator0.26.0通用池由AllocationBackend边界供给，候选DirectMemoryBackend只使用已有Vulkan专用分配，不能视为恢复原公共分叉。

依赖版本和来源遵循原Cargo.lock；原PounceLight/ron固定commit、ustr及rspirv直接复用，不重写通用依赖。解释性注释使用中文。

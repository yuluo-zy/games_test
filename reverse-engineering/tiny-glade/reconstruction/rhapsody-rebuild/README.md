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

```powershell
cargo test --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo run --offline --locked --example shader_production_gpu
cargo run --offline --locked --example queue_graph_gpu
```

命令行主程序读取逐行JSON构图场景并输出CPU状态与API捕获，examples/two-pass.json为输入示例。实际GPU路径使用rg::vulkan::VulkanGraphBackend及VulkanGraphSession。原vk-sync-fork0.4.0本地分叉来源未确认，访问码映射仍通过AccessTable显式输入。原gpu-allocator0.26.0通用池由AllocationBackend边界供给，候选DirectMemoryBackend只使用已有Vulkan专用分配，不能视为恢复原公共分叉。

依赖版本和来源遵循原Cargo.lock；原PounceLight/ron固定commit、ustr及rspirv直接复用，不重写通用依赖。解释性注释使用中文。

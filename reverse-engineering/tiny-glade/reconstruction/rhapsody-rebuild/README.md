# rhapsody 整工程重建候选

本目录依据原 PDB、类型、调用关系与机器码恢复可编译的 Rust 候选，并用原入口差分与真实 Vulkan 读回修正行为。进度按功能验收记录；"候选"指没有原作逐字节证据、由我们自行设计的部分。

## 文档索引

| 文档 | 内容 |
|---|---|
| [MODULE_STATUS.md](MODULE_STATUS.md) | 逐功能的验收状态与证据链接（权威状态表） |
| [ARCHITECTURE.md](ARCHITECTURE.md) | 分层、并行录制三段式、锁清单、未做的重构及理由 |
| [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md) | 优化提案 A–G 的评估与已落地项、已知限制 |
| [DEPENDENCY_AUDIT.md](DEPENDENCY_AUDIT.md) | 原作 36 个依赖与本工程的对照、并发协议、`RingBuffer`/位域补查、首条真实 pass |
| [perf/README.md](perf/README.md) | 性能基准与门禁（`cargo bench --bench perf`） |
| [AGENTS.md](AGENTS.md) | 本目录的工作约定 |

原作取证（本目录之外）：[并行录制](../../evidence/render-recovery/parallel-recording/FINDINGS.md)、[setup 命令缓冲/纹理上传](../../evidence/render-recovery/p4-sync-setup-cb/FINDINGS.md)、[多队列与跨队列屏障](../../evidence/render-recovery/multi-queue/FINDINGS.md)、[机制差异表](../../docs/reference/MECHANISM_DIFF.md)。

## 当前已实际运行的链路

- 渲染图资源声明、编译、屏障、录制、wait/signal 提交和完成回收；**按上一帧耗时分块的并行录制**（还原原作 `record_batch`）。
- 原 shader 容器/反射 catalog 生成布局、descriptor 状态、compute/raster 管线与缓存；**原作 blur shader 的 SPIR-V 经 rg 图执行并与 HLSL 参考比对**（`tests/real_pass_blur.rs`）。
- 附件视图/renderpass/framebuffer 缓存与真实渲染；跨帧 temporal 双缓冲、resize 与资源持有。
- 同一 session 连续 3 帧复用资源池、命令缓冲和时间线，失败停止及完成后释放。
- Win32 隐藏窗口交换链 acquire/submit/present、resize 与 vsync 重建。
- bufferDeviceAddress、动态常量、时间戳、GPU 诊断 marker/debug packet 与分配统计。
- setup 命令缓冲与纹理上传（同步候选）、延迟销毁场、显存子分配（公开 `gpu-allocator 0.27.0`，**默认关闭**，`RHAPSODY_ALLOCATION=sub` 开启）、屏障计数。

## 验证状态（2026-10-13）

- `cargo test --locked`：**132 通过**（专用分配、子分配、无 Vulkan 三种环境一致）；`cargo clippy --all-targets` 无警告。
- 本机用 **lavapipe（软件 Vulkan）** 跑 GPU 测试与示例；`device_policy_gpu`/`shader_production_gpu` 需原 `D:/` 资产，`swapchain_gpu` 需 Win32 窗口，本机跳过。lavapipe 不是真显卡，**绝对耗时不能外推**。
- `evidence/*-validation.json` 等是 2026-10-12 的验收快照，记录的源文件哈希对应当时的源码；此后源码已改动（并行录制、子分配、依赖对齐等），这些快照需要在原环境用 `scripts/` 重跑才会刷新，**不代表当前源码的验收**。

## 与原作的已知差异

- 原作并行录制已恢复；命令池来源、其余并行点（如 `load_textures`）未恢复；第一帧与"重 pass 排在最后"两种情形来自反汇编推断，未动态验证。
- 原作渲染图录制写死主队列，屏障无所有权转移、图像 EXCLUSIVE（见多队列取证）；我们同样不做多队列调度。
- `ADAPTIVE`、`Balanced`、延迟销毁场、显存子分配是候选扩展，不是原作机制。
- 原 `gpu-allocator 0.26.0` 是本地分叉，公开 0.27.0 只是最接近且兼容的替代。
- `RasterPipelineOpts` 原作是 4 字节位域（默认 `0x1FFC00D0`），我们是 `Option` 字段结构，字段集合不同；位偏移未恢复。
- 未做：swapchain `retire`、`set_fullscreen_exclusive`、`queue_submit` 余下约 70% 取证、缓存淘汰（未调查）、真显卡与 validation layer 下的验证。

## 常用命令

```powershell
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo bench --bench perf -- --check
cargo run --locked --example shader_production_gpu
cargo run --locked --example queue_graph_gpu
```

命令行主程序读取逐行 JSON 构图场景并输出 CPU 状态与 API 捕获，`examples/two-pass.json` 为输入示例。实际 GPU 路径使用 `rg::vulkan::VulkanGraphBackend` 及 `VulkanGraphSession`。原 `vk-sync-fork 0.4.0` 本地分叉来源未确认，访问码映射通过 `AccessTable` 显式输入。

依赖版本和来源遵循原 `Cargo.lock`；`PounceLight/ron` 固定 commit、`ustr`、`rspirv`、`bevy_tasks`、`parking_lot`、`log`、`arc-swap`、`append-only-vec` 直接复用，不重写通用依赖。解释性注释使用中文。

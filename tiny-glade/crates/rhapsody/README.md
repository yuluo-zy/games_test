# rhapsody

自有渲染框架：把渲染 pass 和资源访问声明编译为录制、屏障、提交及 GPU 完成回收链。目录对应 B 版 `rhapsody/src/{rg,raw}`；shader 资产协议与构建器分别位于嵌套的 `shader-pipeline`、`shader-compiler` 包。

## 架构重构

当前按 [ARCHITECTURE.md](ARCHITECTURE.md) 与 [REFACTOR_PLAN.md](REFACTOR_PLAN.md) 改造，以正确性和性能为标准。实现进度与验证见 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)。下文来源记录保留用于追溯，不约束新接口的布局或行为。

设备提交统一走每队列时间线与锁；提交成功才确认完成点，延迟销毁使用 `Retirement`。`create_headless_device()` 默认单队列；需要计算/传输辅助队列时使用 `create_headless_device_configured(&QueueConfig { require_present: false, ..QueueConfig::prefer_all() })`。`VulkanGraphBackend::new_for_class` 按配置选择类别，别名类别共享同一条设备时间线。

图会话默认串行录制，一个普通批次使用一个命令缓冲。资源创建错误保留设备错误类别；录制失败取消未提交部分，已提交部分等待真实完成后清理，其 GPU 副作用不会撤销。只有设备丢失使会话永久不可用。

## 已恢复与功能布局

| 模块 | 当前实现 |
| --- | --- |
| `rg/graph`、`pass_builder`、`recorder`、`resource_info`、`compiled` | 图声明、访问账本、usage 标志、生命周期、批次编译、导入/导出、回调消费 |
| `rg/resource_registry`、`pass_bindings`、`attachments`、`pass_api` | typed resource Ref、managed 所有权、descriptor 绑定、附件缓存与 draw/dispatch 命令 |
| `rg/executor`、`parallel`、`state`、`temporal` | 录制与序号、原 PGO 双缓冲及贪心分块、持久状态、跨帧资源配对/导出 |
| `raw/device`、`init`、`physical_device`、`instance` | 已知原设备/驱动/白名单规则、Vulkan 实例/设备/能力与队列调用 |
| `raw/buffer`、`texture`、`sampler`、`shader`、`descriptor_set`、`render_pass`、`framebuffer` | 真实资源/视图、映射读回、shader 包消费、descriptor 与 compute/raster 管线缓存 |
| `raw/managed`、`submit`、`disposal`、`semaphore`、`dynamic_constants` | generation/checksum/持有、CPU 退役与 GPU 完成分离、时间线提交与常量资源 |
| `raw/format::observed`、`resource_usage`、`crash` | 原格式入口、访问掩码换算、GPU assertion/debug packet 解析 |
| `rg/vulkan`、`raw/setup`、`texture_upload`、`swapchain` | 已恢复规则的可调用 Vulkan 适配；具体候选边界如下 |

既有 `rg/compiled_graph` 叶 API 保留，以免丢失上轮已导入的恢复实现；`rg/resource_usage` 改为 re-export `raw/resource_usage`，不再维护两份同体拷贝。`raw/format` 的公共 Khronos 注册表与 `observed` 原私有入口分开；公共表不是新的游戏算法。原格式观察表已合入 `format_table` 的 `OBSERVED_*`，删除了上轮未接入模块树的重复 `format_data` 文件。

## 来源与逆向记录

源代码来自 [`reconstruction/rhapsody-rebuild`](../../../reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/README.md)。重要提交：`92b4ca26`（逆向取证）、`344b73f7`（缺失 typed 接口）、`52a14460`（原并行分块、依赖对齐与真实 blur pass）、`e4e0fbe2`（实际 shader 构建）、`db1ecad7`（原 compute 宏与接口修复）。每个迁入文件的路径、提交和快照哈希见 [migration-sources.json](migration-sources.json)。

另逐个核查旧 `render/work-packages` 的 21 个目录，归宿、后续原码纠正与 oracle/harness 排除原因见 [WORK_PACKAGE_AUDIT.md](WORK_PACKAGE_AUDIT.md)。其中原 sparse descriptor/tag/数组/dynamic offset 规则本次补入 `raw/descriptor_binding.rs`，没有以旧重建工程“看起来完整”代替闭包核查。

原始报告保留在 [`rhapsody-rebuild/evidence`](../../../reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/evidence) 与 [`evidence/render-recovery`](../../../reverse-engineering/tiny-glade/evidence/render-recovery)。其中 `MODULE_STATUS.md`、`DEPENDENCY_AUDIT.md`、`evidence/functional-project-validation.json`、`evidence/missing-functions-completion-2026-10-13.json` 分别描述局部合同和历史验证。历史输入哈希不自动覆盖本工程的 feature 隔离、模块迁移及未来修改。

## 候选集成与剩余功能

有原指令证据的图编译、生命周期、PGO、格式和绑定规则已放入对应模块；整帧尚未验收不再成为排除这些实现的理由。可运行适配也不被冒称为原完整对象 ABI：`VulkanDevice`、主图 provider 桥、串行 session、同步 setup 上传、WSI 重建和诊断后端仍有登记的候选策略。`RasterPipelineOpts` 已按 0x1416a3720 还原为 4 字节位域（同时是 `RasterPipelineKey.selector`）；`[device+0xad9]` 已更正为 wideLines 门控并接入设备创建；子通道号恒 0 仍是候选。

`suballocation` 默认开启，启用公共 `gpu-allocator 0.27`：原版是子分配，B 版 Cargo.lock 记 0.26.0 本地分叉，但 PDB 中 `AllocatorReport` 只在 0.27 出现，故按 0.27 复用（见 Cargo.toml 注释）；该分叉源码仍未找到，`tools/verify_inventory.py` 对此依赖身份仍报差异。`candidate-optimizations` 默认关闭；session 默认串行，调用方可显式设置 `ParallelRecording`。旧序号回收与逐命令复用仍在迁移到帧环池的过程中。没有重写 Bevy、ash 或分配库算法。`raw/descriptor_binding` 覆盖已取证的资源 tag 0–6、8–9；tag 7 等未知路径保留显式不支持边界，不把错误返回宣称为原完整分支恢复。

尚缺原完整设备排序/驱动编码、原 managed 槽位与销毁桶布局、首个真实 shader callback 自动绑定、原多队列 ownership transfer/调度、原 temporal 淘汰与 ABI、原窗口焦点/全屏策略、厂商故障转储、完整游戏 producer/consumer。原版没有硬件光追路径（`rt()` 恒为 None）；`should_reset_temporal` 属 country-core consumer。temporal 已可经 `VulkanGraphSession::execute_temporal_frame` 走导出与 `validate_compiled_exports`，但 `tiny-glade/src/raster_host.rs` 的三个 temporal 入口仍未接（core 键 `ustr_contract::intern` 未恢复）。`get_setup_cb_submit_info` 已按 0x140bf84e0 实现为 `SetupCommandSlot::take_for_submit`；未恢复的 `set_fullscreen_exclusive`、`require_vendor_crash_dump` 占位方法没有迁入。

## 验证范围

迁入 `tests/` 覆盖图 callback、退役、managed key、绑定、附件、命令、PGO 和分配私有包装合同。少量管线测试需要 Vulkan，缺设备时会明确提示跳过，不能当真实 GPU 验收。原工程的格式 816 案例、GPU 读回、temporal/swapchain 等报告仅证明报告明确覆盖的子链和硬件；不证明整个游戏渲染/所有设备/三版本一致。正式工程的本次编译与测试记录见 [migration-validation.json](migration-validation.json) 及根工程审计报告。`tools/` 是来源库存刷新/复核辅助工具，不作为新的游戏逆向功能计数。

workspace 声明 Rust 1.88 下界以容纳原恢复代码已有的 let-chains，但没有在 Rust 1.88 编译器上实测。2026-10-16 缺陷修复轮在 rustc 1.96.0 + lavapipe 上：三个渲染包 `--all-features` 342 项、默认 feature 339 项测试通过，两种 feature 组合下 `clippy --all-targets -D warnings` 均通过；shader-compiler 原合同测试缺游戏资产时显式跳过（`TG_B_GAME_ROOT` 可指定）。早期测试计数保留为历史断点。

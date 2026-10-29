# rhapsody

rhapsody 是 tiny-glade 工作区中的 Vulkan 渲染框架 crate（基于 `ash`）。它把渲染 pass 与资源访问声明编译为录制、屏障、提交和 GPU 完成回收链。着色器资产协议与构建器分别位于嵌套的 `crates/shader-pipeline` 与 `crates/shader-compiler` 包。

源码由逆向重建迁入。迁入文件的路径、提交与快照哈希见 [migration-sources.json](migration-sources.json)，迁移侧的编译与测试记录见 [migration-validation.json](migration-validation.json)。

> 本文只描述当前代码。测试结果、验收门槛与真机覆盖以 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md) 为准。本文不复述验证数字；本次编写时没有重新运行测试，文中凡未见到结果的内容都标为未验证。

## 文档入口

| 文档 | 用途 |
| --- | --- |
| [ARCHITECTURE.md](ARCHITECTURE.md) | 当前模块划分、接口职责、同步与所有权不变量 |
| [CROSS_BACKEND_API.md](CROSS_BACKEND_API.md) | 跨后端(Vulkan/Metal)公共 API 草案:复用/抽象/重写分类、接口签名、泄漏点与迁移步骤 |
| [METAL_INTEGRATION_AUDIT.md](METAL_INTEGRATION_AUDIT.md) | Metal 集成调查:测试影响、重写清单、宿主层泄漏审计(附 [METAL_AUDIT_leaks.tsv](METAL_AUDIT_leaks.tsv))、移动端铺垫项 |
| [METAL_DESIGN.md](METAL_DESIGN.md) | Metal 集成与移动/桌面双端渲染设计(第 3 版,经两轮独立审查):27 个模块的多方案比较与选择、依赖拆分、性能门槛、分阶段计划、决策实验与审查记录。与另两份文档冲突时以此为准 |
| [DELIVERY_PLAN.md](DELIVERY_PLAN.md) | 交付范围、阶段顺序与验收门槛 |
| [REFACTOR_PLAN.md](REFACTOR_PLAN.md) | 各阶段开发工作包、依赖与下游迁移 |
| [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md) | 当前状态与实测证据，区分实现、接线与验收 |
| [OPTIMIZATIONS.md](OPTIMIZATIONS.md) | 已实现的性能机制、位置、开关，以及尚未测量的部分 |
| [src/present/swapchain/README.md](src/present/swapchain/README.md) | 交换链与呈现完成证明；交换链细节以此为准 |
| [examples/window_lab/README.md](examples/window_lab/README.md) | Window Lab：真实窗口的素材播放与验收入口 |
| [WORK_PACKAGE_AUDIT.md](WORK_PACKAGE_AUDIT.md) | 原工作包的归宿核查 |
| [FILE_AUDIT.md](FILE_AUDIT.md)、[IMPROVEMENT_TASKS.md](IMPROVEMENT_TASKS.md) | 逐文件走查台账与改善任务清单（本次未复核其中的路径与结论） |

## 模块布局

`src/lib.rs` 公开下列模块：`accounting`、`bindless`、`collections`、`command`、`device`、`format`、`native`、`pipeline`、`profile`、`present`、`resource`、`rg`、`sync`、`tracking`、`transfer`。`log_ring`、`retire`、`trace` 为内部模块。`Error`、`ErrorKind`、`Result`、`SharedError` 在 crate 根重新导出。

| 模块 | 内容 | 主要位置 |
| --- | --- | --- |
| `device` | `Device`、`DeviceConfig`、队列规划、内存与预算、退役（lifetime）、提交 | `device/init/mod.rs`、`device/submission/mod.rs`、`device/lifetime.rs` |
| `native` | 原生 Vulkan 封装：实例、物理设备、surface、访问表 | `native/instance/`、`native/surface.rs` |
| `command` | 命令缓冲与池、屏障发射、描述符集、待获取的队列族所有权 | `command/barrier/`、`command/descriptor_set/`、`command/pending_acquire/` |
| `resource` | 缓冲、纹理、采样器、托管句柄、动态常量 | `resource/buffer/`、`resource/texture/`、`resource/managed/` |
| `tracking` | 资源身份、范围状态（range state）与使用访问 | `tracking/range_state/` |
| `sync` | 每队列时间线、设备代际、跨队列依赖阶段 | `sync/timeline/`、`sync/mod.rs` |
| `pipeline` | 着色器模块与管线布局、异步编译、磁盘缓存、热替换 | `pipeline/compiler.rs`、`pipeline/disk.rs`、`pipeline/hot.rs` |
| `bindless` | update-after-bind 描述符堆、参数页与 shader ABI | `bindless/mod.rs`、`bindless/parameters/` |
| `transfer` | 异步缓冲与纹理上传、读回、setup 命令 | `transfer/mod.rs`、`transfer/flush.rs` |
| `rg` | 渲染图声明、计划与校验、执行、并行录制、Vulkan 会话、帧循环 | 见下 |
| `present` | 交换链、呈现策略、全屏状态 | `present/swapchain/` |
| `accounting` | 预算、统计、可驱逐缓存的注册表（`Evictable`） | `accounting/budget/`、`accounting/stats/` |
| `format`、`collections`、`profile` | 格式表、内部数据结构与池、性能剖析辅助 | — |

`rg` 内部：`graph/`（RenderGraph 声明与构建）、`plan/`（`GraphDefinition`、`CompiledPlan`）、`plan_check/`（独立计划校验器）、`executor/`（执行器与 `PassContext`）、`parallel/`（分块录制）、`vulkan/`（`VulkanGraphBackend`、`VulkanGraphSession`）、`frame_loop/`（`FrameLoop`）、`transient_resources/`（瞬态别名）、`sync_plan/`（同步计划）、`result_cache/`、`temporal/`。

## 构建与测试

以下命令在 tiny-glade 工作区根目录（`tiny-glade/`，不是本 crate 目录）运行：

```bash
cd tiny-glade
cargo test -p rhapsody --all-features --no-fail-fast
cargo test -p rhapsody --no-default-features --lib
cargo clippy -p rhapsody --no-deps --all-targets --all-features -- -D warnings
cargo doc -p rhapsody --no-deps
cargo test -p shader-compiler
cargo test -p shader-pipeline
```

- workspace 声明 `edition = "2024"`、`rust-version = "1.88"`。本次没有在 1.88 编译器上复核。
- 多数 GPU 用例在没有可用 Vulkan 设备时显式跳过。设置 `RHAPSODY_REQUIRE_VULKAN=1` 后，设备初始化失败会变为测试失败（`tests/gpu_test_support/mod.rs`）。
- 软件 Vulkan（例如 Mesa lavapipe）可通过 Vulkan loader 的 `VK_ICD_FILENAMES` 选择。
- `tiny-glade/.cargo/config.toml` 为 macOS Homebrew 的 loader 设置 `RHAPSODY_VULKAN_LIBRARY` 默认值（`force = false`，调用方覆盖优先）。`native/instance/mod.rs` 初始化时优先读取该变量。
- 直接运行二进制时需显式传入该变量，不能只依赖 Cargo 配置。
- Window Lab 的运行命令与参数见 [examples/window_lab/README.md](examples/window_lab/README.md)。

### Cargo 特性

| 特性 | 默认 | 作用 |
| --- | --- | --- |
| `suballocation` | 开 | 启用公共 `gpu-allocator` 0.27 子分配（`device/suballoc/`）。关闭后 `AllocationStrategy::Suballocated` 会返回 `Unsupported` |
| `validation` | 关 | release 构建在计划缓存未命中时也运行独立计划校验器；debug 构建始终运行 |
| `cpu-probe` | 关 | 本地原生命令计数等测量探针，不进入默认构建 |
| `test-support` | 关 | 把内部模块以 `#[doc(hidden)]` 暴露给本 crate 的集成测试与基准，不属于公共 API |

## 主要公开入口

```rust
use rhapsody::device::{Device, DeviceConfig};

let device = Device::new_headless(&DeviceConfig::headless())?;
```

| 入口 | 说明 |
| --- | --- |
| `device::{Device, DeviceConfig, QueueConfig, QueueClass}` | 设备创建（`Device::new_headless`、`Device::new(&instance, Some(&surface), &config)`）、队列配置与能力查询 |
| `device::{create_headless_device, create_headless_device_configured}` | 兼容的无头设备入口，后者接受 `QueueConfig` |
| `native::surface::Surface` | `from_raw`、`from_win32_raw` 均为 `unsafe`；库不推断窗口平台 |
| `rg::VulkanGraphSession` | 每帧 `begin_frame` → 声明 → `execute_frame`；命名持久资源 `named_texture`、`named_buffer`、`temporal_texture`；图作用域导入 `import_frame_buffer`、`import_frame_texture` |
| `rg::RenderGraph`、`rg::GraphDefinition` | 图声明；`PassBuilder::on_queue(QueueClass)` 指定队列类别，`with_side_effect()` 标记无导出的外部效果 |
| `rg::FrameLoop` | 窗口帧循环：`new(session, swapchain)`，随后按 `begin_frame`、`execute_graph`、`acquire`、`present`、`collect`、`shutdown` 推进 |
| `present::swapchain::{Swapchain, SwapchainPolicy, PresentPolicy, WindowMode}` | 交换链策略、失败帧策略与窗口模式 |
| `transfer::TransferManager` | `enqueue_upload`、`flush_uploads`、`upload_textures`、`flush_acquires` |
| `pipeline::{AsyncPipelineCompiler, PipelineTicket}` | 后台管线编译；帧内 `PipelineTicket::try_get` 对未就绪返回超时，不阻塞 |
| `bindless::BindlessHeap` | 共享描述符堆；租约不可变，槽位在 GPU 使用完成后才复用 |
| `Device::completed()`、`Device::poll()` | `completed()` 返回缓存的完成值、不访问驱动；`poll()` 查询驱动并刷新 |

## 支持边界与已知限制

- 阶段与范围：V0–V9 的划分见 [DELIVERY_PLAN.md](DELIVERY_PLAN.md)。完成度与缺口以 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md) 为准。
- 真机覆盖：据 REFACTOR_PROGRESS.md，多队列族传输、跨族获取、非一致性内存与独占全屏尚未在真硬件上运行；测试环境以单队列、一致性内存的软件 Vulkan 为主。
- 生产图形路径默认要求的能力：`DeviceConfig::default()` 的 `required_features` 包含 bindless 所需的 descriptor indexing 特性、timeline semaphore、dynamic rendering、Synchronization2 与 `shaderDemoteToHelperInvocation`（`device/init/mod.rs`）。最低硬件矩阵尚未在目标环境确定。
- 交换链：`SwapchainPolicy::Strict` 在没有 `VK_EXT_swapchain_maintenance1` 时以 `QueueIdle` 证明呈现完成，这比 fence 弱；其超时分支没有被触发过（见 [交换链说明](src/present/swapchain/README.md)）。
- 交换链：在 Mesa llvmpipe 上不使用 `vkReleaseSwapchainImagesEXT`（`can_release_images`）。
- 交换链：`PresentPolicy::PreviousFrame` 不受支持，设置时返回 `Unsupported`。
- Windows 独占全屏：驱动层的获取、释放与显示器切换仍未在原生环境验证。`FrameLoop::set_window_mode` 是 `unsafe`，库不设置窗口样式、分辨率或选择显示器。
- 与原版的对应仍有缺口，包括原设备排序与驱动编码、托管槽位布局、多队列所有权调度、原 temporal 淘汰。这些条目以进度表为准，本文未复核。
- 本 crate 的测试不构成游戏整帧或全部设备的一致性验收。
- 着色器构建：`shader-compiler` 依赖外部 DXC。原始离线工具的版本与配置尚未完全恢复，见 [shader-compiler 说明](crates/shader-compiler/README.md)。

## 性能

性能机制、位置、开关与测量记录见 [OPTIMIZATIONS.md](OPTIMIZATIONS.md)。

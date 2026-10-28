# rhapsody 重构状态快照

本文件只记录当前源码的实现范围、验证状态、开放事项与下一步。范围与验收门槛见 [DELIVERY_PLAN.md](DELIVERY_PLAN.md)，剩余任务见 [REFACTOR_PLAN.md](REFACTOR_PLAN.md)，目标结构与不变量见 [ARCHITECTURE.md](ARCHITECTURE.md)，交换链细节见 [swapchain README](src/present/swapchain/README.md)。

本文件不保存历史轮次、旧日志或已删除的报告。`reports/` 目录已删除，本文不引用它。

## 1. 当前实现范围

### 1.1 设备、队列与错误

- 唯一 `Device`（`src/device/logical.rs`）由 `extensions`、`lifetime`、`memory`、`queues`、`core` 组成；`core` 必须是最后一个字段，负责持有 `VkDevice` 与 instance。
- 内部缓存只持有 `Weak<Device>`，`Device` 总是构造在 `Arc` 中，避免设备到缓存再回到设备的强引用环。
- `DeviceConfig` / `DevicePreference`（`src/device/init/mod.rs`）决定实际启用的扩展与队列。
- `QueuePreference::{Off, Prefer, Require}`：没有独立队列时 `Prefer` 别名到 graphics，`Require` 返回 `Unsupported`（`src/device/queue_config/mod.rs`）。
- 每个队列有独立时间线（`src/device/queues.rs`）。
- 实例在驱动提供时自动启用 portability 枚举（`src/native/instance/mod.rs`）。
- 容量耗尽使用 `Error::Exhausted`（槽位、描述符池、arena、缓存），与 `Timeout`、`OutOfDeviceMemory`、`DeviceLost` 分开（`src/error/mod.rs`）。

### 1.2 提交、完成与回收

- 完成点用 `SyncSet` 聚合，回收用 `Retirement<T>`（`src/retire/mod.rs`）。
- 帧事务为 `FrameTransaction`（`src/rg/transaction.rs`）。
- 上传的所有权获取由 `TransferManager` 处理，无图提交时调用 `flush_acquires()`。
- 屏障统一为 `RangeBarrier` 与 `Device::issue_barriers`；访问描述统一为 `AccessSpec`，所有权判断统一为 `RangeOwner::admits`。
- 并行录制通过 `ChunkLease`（`src/rg/executor/backend_api.rs`）分配录制块。

### 1.3 呈现与交换链

- `SwapchainPolicy` 默认 `Strict`。有 `VK_EXT_swapchain_maintenance1` 时用 present fence 证明完成；没有时用 `PresentProof::QueueIdle`（呈现队列上的空批次、有界等待）。`Strict` 不要求 maintenance1。
- `AcquireOnly` 只做获取，拒绝 `present`。
- `PresentPolicy`：`SkipPresent`（默认）、`ClearTo([f32; 4])`、`PreviousFrame`（返回 `Unsupported`，交换链不保存已呈现内容）。
- `release-images` 只在 `can_release_images()` 为真时使用：启用 maintenance1，且驱动不是 Mesa llvmpipe。
- 常量：获取超时 `ACQUIRE_TIMEOUT_NS` 为 3 秒；未完成证明的旧代上限 `MAX_RETIRED_GENERATIONS` 为 4。
- 全屏：`WindowMode::{Windowed, Borderless, Exclusive { monitor }}` 与 `FullscreenFallback::{Unsupported, AcquisitionFailed, CreationFailed, ExclusiveLost}`（`src/present/swapchain/fullscreen/`）。独占全屏代码已接入，Windows 上的实际行为未验证。
- `FrameLoop`（`src/rg/frame_loop/mod.rs`）是生产帧入口，提供 `execute`、`begin_frame`、`acquire`、`present`、`wait_render`、`collect`、`shutdown`、`resize`、`replace_surface`、`set_focused`、`set_on_failure`；`set_window_mode` 为 `unsafe`。
- `FrameStatus::{Presented, Skipped, Recreated}` 区分正常呈现、无图帧与实际重建。

### 1.4 渲染图

- 分层：`GraphDefinition`（`src/rg/plan/definition.rs`）→ `CompiledPlan`（`src/rg/plan/compiled.rs`，不含 Vulkan 对象或帧状态）→ `FrameInstance`（`src/rg/graph/snapshot.rs`，绑定本帧导入资源）。
- 图构建与执行入口在 `src/rg/graph/`、`src/rg/executor/`、`src/rg/vulkan/`。
- 交换链图像作为导入资源进入图（`import_swapchain_image`、`PresentTarget`），首次使用处可切批（`gate_first_use`）。
- 资源池与瞬态别名缓存实现 `Evictable`，压力下只释放已完成的空闲项；`VulkanGraphSession::cache_stats()` 返回 `GraphCacheStats`。
- 规划校验器在 debug 构建、`validation` 特性或 `rg::set_plan_validation(true)` 时运行（`src/rg/mod.rs`、`Cargo.toml`）。
- 生产源码中没有 `VkRenderPass` / `VkFramebuffer` 的创建（`rg` 搜索为空），动态渲染是唯一光栅路径。

### 1.5 内存、预算与上传

- 默认特性 `suballocation` 启用依赖分配器（`gpu-allocator 0.27` 本地分叉，见 `Cargo.toml` 注释）。
- 瞬态内存：`src/rg/transient_memory/`、`src/rg/transient_memory_check/`。
- 记账与预算：`src/accounting/`（budget、stats、cpu_storage）。
- 上传与读回：`src/transfer/`（enqueue、flush、tickets、staging）。

### 1.6 bindless、管线与热重载

- bindless 堆：`src/bindless/`（descriptors、slots、pages、lease、residency）。
- 管线：`src/pipeline/`（request、compiler、hot、disk），磁盘缓存文件的读写与校验在 `disk.rs`。
- `pipeline/hot.rs` 的 `publish_at_frame_boundary` 负责帧边界发布新版本。

### 1.7 Window Lab

- 入口 `examples/window_lab/`，平台宿主 `win32.rs`（Windows）与 `macos.rs`（AppKit / CAMetalLayer）。
- 场景（`--scene`）：`showcase`（默认）、`fixture`、`raster`、`temporal`、`transfer`、`transient`、`result`、`multi-queue`。
- 常用参数：`--playlist`、`--frames`、`--fps`、`--script`、`--wsi-stress`、`--repeat-script`、`--report`、`--capture`、`--require-validation`、`--vsync`、`--queues`、`--sharing`、`--rendering`、`--performance`、`--warmup`、`--samples`。完整列表见 `examples/window_lab/app.rs` 的解析函数。

## 2. 当前验证状态

### 2.1 本次核对范围

- 本文件在一次纯文档整理中重写：只做了源码与文档的对照（`rg` / 阅读），**没有运行** `cargo test`、clippy、GPU 或窗口检查。
- 旧报告与日志已删除，其中的测试计数、截图、像素数据都不作为当前证据。
- `src/present/swapchain/README.md` 记录了一次 Linux lavapipe 运行（lib 494 通过、2 个忽略；临时关闭 maintenance1 后 `raw_fixes_swapchain` 16 项通过）。该结果未在本次复跑，仅作为 README 的记录引用。

### 2.2 里程碑状态

状态取值：PLANNED / IN_PROGRESS / PASS / FAIL / BLOCKED。**当前没有任何 M0–M9 里程碑为 PASS。**

| 里程碑 | 状态 | 当前可引用的证据 | 主要缺口 |
| --- | --- | --- | --- |
| M0 窗口基线 | IN_PROGRESS | 无有效保留报告；代码路径（Win32/macOS 共用帧体）已接入 | 屏幕像素与键盘人工检查未完成；Windows 未运行 |
| M1 设备与完成 | IN_PROGRESS | 代码已接入 | 独立硬件队列族上的隔离与稳态计数未验证；完整目标矩阵未复跑 |
| M2 提交与呈现 | IN_PROGRESS | `Strict` 无 maintenance1 路径：lavapipe 记录通过（见 2.1） | Intel UHD 630 无 maintenance1 真机未运行；`QueueIdle` 超时分支未触发；底层阶段注入矩阵未闭合；Windows 独占全屏 BLOCKED |
| M3 图分层与契约 | IN_PROGRESS | `GraphDefinition` / `CompiledPlan` / `FrameInstance` 与检查器代码已接入 | 最终矩阵与 S03/S04 窗口结果未复跑 |
| M4 帧池、传输与预算 | IN_PROGRESS | 帧 × worker × family 命令池、上传与预算代码已接入 | 框架热路径分配为 0 的最终测量未复跑；S07 长跑未执行 |
| M5 整图/子图/结果缓存 | IN_PROGRESS | 缓存代码与统计接口已接入 | S06 最终检查与冻结源码矩阵未复跑 |
| M6 瞬态内存着色 | IN_PROGRESS | 别名与 backing 所有权代码已接入 | 实际 committed 削减的最终测量与性能 A/B 未执行 |
| M7 动态渲染 / bindless / 管线 | IN_PROGRESS | 动态渲染唯一路径、bindless、异步管线与磁盘缓存代码已接入 | 热重载与错误 shader 的窗口验收、性能门未执行；DXC 专用构建协议未完成（`shader-compiler` 检查中记录为 BLOCKED） |
| M8 多队列 | IN_PROGRESS（独立硬件验收 BLOCKED） | `QueueClass` 分批、ownership 与时间线依赖代码已接入 | 需要真实独立 graphics / compute / transfer 族；`MinimizeMemory` 策略在源码中不存在 |
| M9 收敛与发布 | IN_PROGRESS | — | 删除审计、下游总审计、30 分钟长跑（至少 10000 帧，三类负载）、300 预热 + 3000 采样 × 3 的性能门全部未执行 |

### 2.3 硬件与平台边界

- **Windows**：当前没有可引用的运行结果。Win32 窗口、独占全屏、显示器切换与失焦驱动行为为 BLOCKED。
- **macOS / MoltenVK（Apple M3）**：可用于 GPU 与窗口检查。屏幕像素与键盘人工检查受 Computer Use 权限限制，未完成。此前记录的设备只暴露别名队列族，因此独立族验证不能在该设备完成。
- **Linux / lavapipe**：单族、一致性内存的软件设备。可以覆盖 CPU 逻辑与部分 GPU 路径，不能代表多族、非一致性内存或真实呈现。
- **Intel UHD 630**：无 maintenance1 路径的目标设备，尚未运行。

## 3. 开放事项与阻塞

1. **人工窗口检查**（M0）：屏幕像素、键盘交互需要权限或人工操作；不能用离屏读回替代。
2. **Windows 验收**（M0、M2、M8）：Win32 窗口、独占全屏、失焦行为全部 BLOCKED，等待 Windows 环境。
3. **无 maintenance1 的真机验证**（M2）：`QueueIdle` 证明弱于 fence（规范未把 `vkQueueWaitIdle` 定义为呈现完成证明）。超时分支目前没有触发手段。
4. **呈现的单元测试缺口**：`GraphPresentation`（`arm → submitted → present / abandon`）没有直接单元测试，只有经 `rg` 的间接覆盖（`src/present/swapchain/tests.rs` 中无引用）。
5. **呈现的小型热路径问题**：`can_release_images` 每次查询设备属性，`release_image` 每次创建 `swapchain_maintenance1` loader；全屏配置查询在循环内创建 surface loader。只在失败路径或配置时执行，但每次都重复。
6. **示例测试目标编译失败**：`examples/window_lab/scene.rs` 的测试（约第 1363 行）引用了不存在的 `rhapsody::rg::Outcome`（`rg` 只再导出 `FrameOutcome`）。二进制可编译，但 `cargo test --example window_lab` 不能通过编译。
7. **规划器与运行期状态机**：`PendingOwner` 在源码中仍有 18 处引用，尚未核对是否已与运行期状态机合并。
8. **文档与代码的 Device 组件描述不一致**：旧文档写作 core / queues / memory / lifetime / caches 五组件，但当前 `Device` 结构体没有 `caches` 字段，缓存分散在各模块中。本文以现有结构为准。
9. **下游构建**：`tiny-glade`、`tg-text`、`country-core` 的当前构建与测试状态未在本次核对中验证。
10. **长跑与性能门**（M9）：30 分钟长跑与 300/3000 × 3 性能对照尚未执行；当前没有任何可引用的性能数字。
11. **`MinimizeMemory`**：`DELIVERY_PLAN.md` 中的策略名在源码中不存在，属于未实现的计划项。

## 4. 下一步

1. 修正 `examples/window_lab/scene.rs` 测试中的 `rhapsody::rg::Outcome` 引用，使 `cargo test -p rhapsody --example window_lab` 能编译。
2. 为 `GraphPresentation` 增加直接单元测试，并把 `can_release_images` 与全屏配置的 loader 改为复用，消除重复查询。
3. 按 [REFACTOR_PLAN.md](REFACTOR_PLAN.md) 的验证矩阵，在冻结的源码上重新运行默认与全特性的库、集成与 clippy 检查，并把结果作为新的记录写入本文件。
4. 在无 maintenance1 的设备（Intel UHD 630 或等价设备）上运行 `Strict` 窗口路径，覆盖启动、resize、关闭。
5. Windows 环境可用后，执行 Window Lab 的 Win32 基线（360 帧、素材遍历、缩放、最小化恢复、正常关闭）。
6. 继续 V3–V9 的剩余任务，顺序以 [REFACTOR_PLAN.md](REFACTOR_PLAN.md) 第 3 节为准。

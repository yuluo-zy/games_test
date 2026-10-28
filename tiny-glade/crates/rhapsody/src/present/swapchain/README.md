# 交换链模块（`present/swapchain`）

本文描述当前代码的设计、实现和注意点。

## 设计

### 职责

- 创建、重建、替换 WSI 交换链（`VK_KHR_swapchain`），管理图像获取、呈现、取消与关闭。
- 每个图像槽持有二值 acquire 信号量和 acquire 完成栅栏；每张图像持有 present 信号量和 present fence。
- 证明每一代交换链何时可以安全销毁或回收，即"呈现完成证明"。
- Windows 独占全屏的请求、降级与状态上报（`fullscreen/`）。

渲染图侧入口在 `rg/frame_loop/`（`FrameLoop`）和 `rg/vulkan/presentation/`（图执行时的呈现会话）。

### 策略

**`SwapchainPolicy`**

| 值 | 含义 | 呈现 |
| --- | --- | --- |
| `Strict`（默认） | 生产呈现。关闭与代际回收必须有呈现完成证明 | 允许 |
| `AcquireOnly` | 只做获取（测试、离屏消费者） | 拒绝 `present` |

`Strict` 不要求 `VK_EXT_swapchain_maintenance1`。证明方式由设备能力决定：

| 设备能力 | 证明方式（`PresentProof`） | 证明内容 |
| --- | --- | --- |
| 有 maintenance1 | `Fences` | 每次呈现带 present fence，关闭时等待 fence |
| 无 maintenance1 | `QueueIdle` | 向呈现队列提交空批次，带超时等待其时间线值 |

`PresentProof` 的选择在 `SwapchainPolicy::present_proof`，`Swapchain::present_proof` 结合设备能力。

**`PresentPolicy`**（失败帧如何处理）

| 值 | 行为 |
| --- | --- |
| `SkipPresent`（默认） | 不呈现。能 release 时直接释放图像；否则提交一次真实清屏，再呈现不透明黑帧。绝不呈现未定义内容 |
| `ClearTo(rgba)` | 清成指定颜色后呈现。需要 `TRANSFER_DST`，否则退化为 `SkipPresent` |
| `PreviousFrame` | 不支持。设置时返回 `Unsupported`，因为交换链不保存已呈现内容 |

release 只在 `can_release_images()` 为真时使用：启用 maintenance1，且驱动不是 Mesa llvmpipe（其 `vkReleaseSwapchainImagesEXT` 会崩溃）。

### 证明何时成立

| 事件 | 证明的内容 |
| --- | --- |
| 同一图像被再次获取，且其 acquire 信号已完成 | 该图像上一次呈现已完成（`PresentProgress::acquired`） |
| 同一代的 present fence 已 signal | 对应呈现已完成（`Fences`） |
| 新代的第一次呈现完成 | 旧代的呈现已完成，旧代可回收（`collect_presentations`） |
| `QueueIdle` 的空批次时间线值在超时内到达 | 之前提交到该队列的工作（包括呈现）已完成，本代及所有旧代记为完成（`confirm_generation_complete`） |

渲染完成（`SyncPoint`）与呈现完成是两件事。渲染完成只允许回收渲染资源，不证明呈现资源可以释放。

## 实现

### 文件

| 文件 | 内容 |
| --- | --- |
| `mod.rs` | `Swapchain`、`SwapchainDesc`、内部状态 `SwapchainInner`、锁顺序、超时与保留上限常量 |
| `policy.rs` | `SwapchainPolicy`、`PresentProof`、`PresentPolicy` |
| `progress.rs` | `ImagePresentation`：每张图像的 present 信号量、fence、序号与完成进度 |
| `frame_sync.rs` | `FrameSemaphores`：每个图像槽的 acquire 信号量、栅栏与提交栅栏 |
| `lease.rs` | `SwapchainImageLease`（原始租约）、`AcquiredImageLease`（生产租约）、`PresentTicket` |
| `acquire.rs` | 获取（`acquire_frame`、`acquire_next_image`、`validate_swapchain_and_acquire_image`）、呈现（`present_unnoted`）、`can_release_images` |
| `recreate.rs` | 创建（`new`、`new_with_policy`）、重建、`resize`、`set_vsync`、`replace_surface`、代际预算 |
| `retire.rs` | `collect_presentations`、`shutdown`、`prove_presents_by_queue_idle`、保留代际计数 |
| `wsi.rs` | WSI 错误分类、fence 操作抽象（便于测试注入）、呈现结果分类 |
| `graph_presentation.rs` | 渲染图使用的 `GraphPresentation`：arm → submitted → present / abandon |
| `fullscreen/` | `WindowMode`、`FullscreenStatus`、降级原因与独占获取状态机 |
| `tests.rs` | 单元测试，使用注入的 fence 与 Vulkan 操作替身，不需要设备 |

### 每帧流程

1. **获取**：先等该槽的 acquire 栅栏并重置；`vkAcquireNextImageKHR` 写入该槽的 acquire 信号量。
2. **渲染提交**：消费 acquire 信号量，并 signal 图像的 render 信号量（`rendered`）。
3. **呈现**：先对该图像上一次呈现的 fence 做有界等待（不持任何状态锁），再在 WSI 锁与队列锁下调用 `vkQueuePresentKHR`。
4. **记账**：呈现已入队则分配序号，有 fence 时记为 `certain`（可等待）或 `uncertain`（只轮询）。呈现被拒绝且未入队则不分配序号，只做清理（`finish_rejected_present`）。

### 锁顺序

`wsi` → `inner` → 图像槽 / 图像级 mutex → 设备队列锁。

- `wsi` 串行化所有与交换链句柄有外部同步关系的原生调用：获取、呈现、release、全屏、带 `oldSwapchain` 的创建，以及关闭时的 `QueueIdle` 批次提交。
- `inner` 只保存记账信息，不跨阻塞等待或原生 WSI 调用持有。因此 `desc`、`resize`、焦点查询不会等待获取。
- 栅栏等待（`prepare`、`finish_acquire`）在 `wsi` 之外进行。

### 生命周期

**正常呈现**：获取（`validate_swapchain_and_acquire_image` 或 `FrameLoop`）→ 渲染 → 呈现。`SUBOPTIMAL` 只在 surface 的尺寸或变换确实变化时才标记过期。

**失败**

- 获取前失败：不消费 acquire，经 Drop 路径取消。
- 呈现前失败：按 `PresentPolicy` 返回图像（release 或清屏呈现）。
- acquire 已消费后失败：不再第二次等待同一个二值信号量，由真实提交完成后续步骤。
- `DeviceLost`：停止补救提交，之后的操作直接返回 `DeviceLost`。
- 原生呈现失败且未入队：先等待渲染完成，再清理。清理失败则保留所有权。

**重建与代际预算**

- 重建时新代以旧代为 `oldSwapchain`，旧代进入 `retired_generations`。
- 未完成证明的旧代最多 `MAX_RETIRED_GENERATIONS = 4`。
- 预算满且为 `QueueIdle`：先执行一次 `prove_presents_by_queue_idle`（使用 3 秒超时），再检查。
- 预算满且为 `Fences`：返回 `Exhausted`，调用方重试，`collect_presentations` 逐步回收。

**关闭** `shutdown(timeout)`

1. 设置 `closing`，释放独占全屏。
2. 有界等待所有槽的 acquire 完成与 GPU 使用完成。
3. `QueueIdle` 时执行 `prove_presents_by_queue_idle(timeout)`。
4. 逐图像 `finish_shutdown`：消费剩余 present fence；在 `QueueIdle` 下由第 3 步证明。
5. 递归关闭旧代。

任一步失败都返回错误并保留所有权。`Swapchain::drop` 关闭失败时泄漏 device 与 surface，计入 `RETAINED_GENERATIONS`。这是可观察的安全兜底，不是成功的清理。`FrameLoop::shutdown` 同样只在真实证明齐全后才算完成。

### 常量

| 常量 | 值 | 用途 |
| --- | --- | --- |
| `ACQUIRE_TIMEOUT_NS` | 3 s | 内部获取、栅栏与 `QueueIdle` 等待的上限 |
| `MAX_RETIRED_GENERATIONS` | 4 | 未完成证明的旧代上限 |

## 注意点

1. **`QueueIdle` 保证弱于 fence。** 它依赖"呈现队列上之前提交的工作（包括呈现）按序完成"。规范没有把这一点定义为呈现完成的证明，这正是 maintenance1 要解决的问题。它只用于无 maintenance1 的设备，不能对外宣称与 `Fences` 等价。
2. **`QueueIdle` 超时后的行为。** 超时返回 `Timeout`，不标记完成，保留所有权。调用方可以重试。超时分支目前没有被触发过的测试，因为需要人为造成 GPU 挂起。
3. **`present_unnoted` 名称不直观。** 它是不记录 SUBOPTIMAL 状态的原始呈现，`present` 包装它并补上记录。改名（如 `present_raw`）不改变行为。
4. **`can_release_images` 与 release 的重复开销。** `can_release_images` 每次调用都查询 `PhysicalDeviceProperties2`，`release_image` 每次新建 `swapchain_maintenance1` loader。两者只在失败路径调用，但每次都重复执行，可以缓存。
5. **全屏配置查询每次新建 loader。** `fullscreen/mod.rs` 的 surface 查询在循环内新建 `get_surface_capabilities2` loader，只在配置独占全屏时执行。
6. **`GraphPresentation` 没有直接单元测试。** `arm → submitted → present / abandon` 这条线性协议只有经 `rg/vulkan/presentation` 的间接覆盖。
7. **`image()` / `image_view()` 访问器不检查索引。** 所有租约都在构造前校验过索引（`acquire_next_image`、`acquire_frame`），所以正常路径安全；访问器本身不做检查。
8. **`AcquireOnly` 的 acquire 结果不能呈现。** 它的取消路径会 `mark_stale` 并放弃该代。调用方不应假设 AcquireOnly 链能参与呈现。

## 验证状态

2026-10-28，Linux 开发机。

| 命令 | 结果 |
| --- | --- |
| `cargo test -p rhapsody --lib` | 494 通过，2 个显式忽略 |
| `cargo test -p rhapsody --lib swapchain` | 34 通过 |
| `cargo build -p rhapsody --all-targets` | 通过，无警告 |
| `cargo doc -p rhapsody --no-deps` | 本模块无警告 |
| 临时关闭 maintenance1 后运行 `cargo test -p rhapsody --test raw_fixes_swapchain` | 16 通过（覆盖 `QueueIdle` 路径）。临时改动已还原，不在正式代码中 |

未验证：

- 默认配置下的 `tests/raw_fixes_swapchain.rs`。`strict_frame_loop` 在缺少 maintenance1 时会直接跳过，因此无 maintenance1 的覆盖只来自上面的临时运行。
- Intel UHD 630 上的窗口运行（启动、resize、关闭）。这是目标设备，尚未运行。
- `VK_LAYER_KHRONOS_validation` 下的运行。
- Windows 原生独占全屏（BLOCKED）。
- macOS / MoltenVK 上的历史窗口实测只覆盖 maintenance1 路径，不能替代无 maintenance1 路径的验证。

## 参考

- 设计背景：`ARCHITECTURE.md` 的「帧循环与交换链」一节。
- 当前状态与开放事项：`REFACTOR_PROGRESS.md`。
- Vulkan 规范：[交换链重建示例](https://docs.vulkan.org/samples/latest/samples/api/swapchain_recreation/README.html)、[呈现 fence](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentFenceInfoKHR.html)、[queue present](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html)、[呈现信号量复用](https://docs.vulkan.org/guide/latest/swapchain_semaphore_reuse.html)。

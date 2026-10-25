# rhapsody

## 2026-10-18 core / rhapsody 调用接线

新增宿主侧 `tiny_glade::render_bridge::CoreRenderBridge`，共用一个 `VulkanGraphSession`，连接 core 计算/光栅 pass、权威 shader 句柄和真实资源导入，并提供构图失败取消。接口调查、调用方式、验证范围及剩余阻塞见 [接线报告](../../reports/runtime/2026-10-18-core-rhapsody-bridge/README.md)。这不代表完整 `WorldAccess`、`FramePasses`、游戏主循环或 GPU 帧已经验收。


自有渲染框架：把渲染 pass 和资源访问声明编译为录制、屏障、提交及 GPU 完成回收链。目录对应 B 版 `rhapsody/src/{rg,raw}`；shader 资产协议与构建器分别位于嵌套的 `shader-pipeline`、`shader-compiler` 包。

## 文档入口与完整重构范围

本轮以正确性和性能为标准，按 **V0–V9 完成整个模块重构**；允许破坏性 API 变更并同步迁移下游。

| 文档 | 用途 |
| --- | --- |
| [ARCHITECTURE.md](ARCHITECTURE.md) | 目标模块、接口职责、同步/所有权等不变量 |
| [DELIVERY_PLAN.md](DELIVERY_PLAN.md) | 完整范围、交付顺序、S01–S08 场景及 M0–M9 验收门槛 |
| [REFACTOR_PLAN.md](REFACTOR_PLAN.md) | 各版开发工作包、依赖、下游迁移与旧路径删除 |
| [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md) | 唯一的当前状态与实测证据入口，区分实现/接线/验收 |
| [OPTIMIZATIONS.md](OPTIMIZATIONS.md) | 当前已落地的性能优化清单：机制、位置、开关、实测数据与未验证项 |
| [FILE_AUDIT.md](FILE_AUDIT.md) | 145 个源文件的逐文件走查台账（机器可读表格 + 复核记录），每条发现带 `F-xxx` 编号与 `path:line` |
| [IMPROVEMENT_TASKS.md](IMPROVEMENT_TASKS.md) | 66 条改善任务（机器可读索引 + 动作/验收），按 `IMP-xx` 编号并与 `F-xxx` 互链 |

V0 窗口基线 → V1 设备与回收 → V2 提交/呈现闭环 → V3 图与资源 → V4 池/传输/预算 → V5 缓存 → V6 内存着色 → V7 管线/描述符 → V8 多队列 → V9 全链路发布验收。P0–P7 仅作为技术领域索引。GPU 驱动、mesh shader、descriptor buffer 等后续实验不作为本轮完成条件。

## 当前可用入口与限制

- `device::Device::new_headless(&DeviceConfig::headless())` 和 `Device::new(instance, Some(&surface), &config)` 是当前工作树中的配置入口；支持能力与实际启用分开查询。最终最低硬件矩阵仍待目标环境验证。
- `raw::init::create_headless_device()` 默认单队列；兼容入口 `create_headless_device_configured(&QueueConfig { require_present: false, ..QueueConfig::prefer_all() })` 可请求辅助队列。类别别名共享真实队列的锁与时间线；pass 的 `on_queue(QueueClass)` 经实际队列域生成批次与 timeline 依赖，Exclusive 使用配对 release/acquire，Concurrent 校验真实 family 集合。默认 Off 退化为同一队列。
- 图内资源只通过访问声明进入录制；`PassContext::keep_alive`、原始命令的旧 `keep_alive*` 与 `GpuUse::collect_uses` 已删除。管线等无内存域对象使用可失败的 `retain_object`；图回调不能用它夹带 buffer/image。`GpuUse::visit_uses` 直接访问有界、预算化的收集器，错误使本次命令不可提交。原始未知命令使用显式 `retain_opaque` 合同，仍进入同一范围状态表与真实提交完成域。
- `RangeProgram` 是不含物理对象和时间线值的静态范围计划，`BoundSynchronization` 绑定本帧实际状态与队列依赖。Buffer 字节区间及 image aspect/mip/layer 只在实际资源的 `SyncCell` 范围表中登记；旧整资源屏障投影已删除。成功提交才发布范围变更，部分成功保留真实 prefix；独立检查器可校验实际绑定结果。
- `VulkanGraphSession::import_frame_buffer` / `import_frame_texture` 为上传页、索引和外部 shader 资源提供图作用域导入，复用同图物理身份；声明取消、帧失败和正常退休均释放租约。实际 GPU 资源仍由已提交命令持有，后台 token 不逐帧常驻。常量参数页由内部只读范围协议登记，不能冒充图声明资源。
- 设备提交成功后才发布完成点。图执行器使用 SyncPoint/SyncSet；命令自动收集声明资源并登记 last_use，最后一个 CPU 引用释放后按资源依赖进入 Retirement。时间线信号量也按实际提交登记 last_use；呈现资源使用独立完成协议。
- `DeviceConfig::submission_api` 默认 `SubmissionApi::Auto`，支持时启用 Synchronization2；`Legacy` 显式选择 `vkQueueSubmit` 并保留相同 timeline/资源回收协议。支持集与启用集仍分别查询；Legacy 与必需 Synchronization2 的配置冲突在创建设备前拒绝。
- `DeviceConfig::reclamation` 为 normal/pressure 维护 tick 分别配置扫描和销毁配额。`try_collect_garbage` 有界，`try_collect_all_completed` 显式排空已完成对象；关闭入口使用完整排空。已提交对象 Drop 只入队，未提交对象立即销毁。
- 图会话默认 Serial，一个普通批次使用一个命令缓冲；实际命令池按帧槽、worker 和队列族隔离，全部使用完成后整池 reset。CPU arena 在 CPU 工作结束后独立归还。pass 使用全局 bindless heap，旧每帧描述符 arena 已删除；lease 由实际绑定命令持有，完成前不能重用其槽。
- `rg::plan::{GraphDefinition, CompiledPlan}` 保存纯声明与符号化计划，`rg::graph::FrameInstance` 每帧绑定实际资源和导入初态；类型化图 ID、写版本与访问范围在提交前校验。纯 pass 以导出为根裁剪，外部效果须显式 `with_side_effect()`。
- ExecutionChain 具有有界整图/子图计划缓存；完整键确认命中，导入 token、初态和回调逐帧绑定。独立检查器验证内容来源、依赖、屏障和执行表。会话结果缓存要求完整内容依赖声明，输出持久化且携带生产者完成点；部分失败不发布未完成内容。
- `transfer::TransferManager` 提供有界异步 Buffer 上传和 readback ticket，槽满返回 Timeout，读回票仍存活时不复用其槽。`enqueue_upload` / `flush_uploads` 支持同键脏区合并、优先级和 epoch 取消；只有真实提交成功才发布 SyncPoint。Buffer 便捷读写与 GPU 提交共用资源预留门和唯一范围表；显式未知原生命令仍保守等待整对象。裸映射切片为 unsafe 接口。
- `DeviceConfig::memory_budget_bytes` 默认 1 GiB，实际资源分配按 live/in-flight/cache/retiring 计账；已知 CPU arena、帧表/回执/录制 owner 池、托管元数据、shader 请求/接口和 bindless 表也参与预算。只读回执的最后一个 CPU owner 控制其存储复用，GPU 资源由实际完成点回收；外部回执钉满池时在提交前返回 Timeout。分配器整块保留与资源区间分别报告，driver 内部不透明内存不可查询。完整预算及分配目标以进度表验收为准。
- `device::Device` 是唯一设备本体；设备托管存储只保存带类型和代际的资源 Weak，设备缓存不强持有资源。旧 `from_device` 转换已删除。同步 setup 入口 `with_setup_resources` 显式接收资源依赖；生产窗口入口为 `rg::frame_loop::FrameLoop`，主图使用同一设备时间线连接展示提交；`RenderingPolicy::Auto` 选择已启用的动态渲染；`bindless::BindlessHeap` 以不可变 lease 和真实完成点管理 descriptor 索引。`pipeline::AsyncPipelineCompiler` 提供有界优先队列、去重与取消，帧内 `PipelineTicket::try_get` 对未就绪返回 Timeout；预热才显式 wait。
- 生产图形路径最低要求 timeline semaphore、dynamic rendering 和完整 descriptor indexing 能力：runtime array、partially bound、sampled/storage image 与 storage buffer 的 update-after-bind、nonuniform/dynamic indexing，以及 update-unused-while-pending。创建共享 heap 时还向 Vulkan 查询实际 layout 支持并检查容量；缺能力返回 Unsupported。descriptor buffer/GPL/shader object 只作能力查询，未登记为实现策略。
- 命名持久与历史资源由 `VulkanGraphSession::named_texture` / `named_buffer` / `temporal_texture` 管理，正常 `execute_frame` 自动校验、导出和提交；回调通过 `PassContext::history_valid` / `buffer_history_valid` 查询本帧历史。失败帧不发布新内容；旧公开 physical token/temporal owner 入口已删除，tiny-glade 的 temporal 宿主已接入。
- `bindless::BindlessShaderPackage` 将现有 shader 资产一次转换为共享 heap ABI，原 uniform 和 push 数据进入有界参数页，实际 push 仅携带参数块索引。`BindlessParameters` 校验声明范围并登记真实资源和不可变 lease；`HotPipeline` 在帧边界核验逻辑 ABI、接口、compute 工作组大小和目标后发布，失败继续保留旧管线。持久管线缓存校验设备/版本/完整性，损坏时重建；输入与驱动提取数据携带 CPU 预算，保存不再复制整份 payload。
- 瞬态别名默认开启（`backend.set_transient_aliasing(false)` 关闭），只利用图中已有顺序、不增依赖；别名成员首用由同步规划器建模为前一成员的合成前序访问并发出屏障/跨队列等待；物理别名集按计划与资源规格缓存、整组借给单帧，帧退役后复用。实际 requirements/dedicated/共享方式决定兼容分组，所有别名对象共用唯一 backing owner；报告区分区间与实际 native allocation，不保证每种小负载都减少整块保留显存。
- `backend.configure_gpu_timing(Some(config))` 开启有界 query ring；`config.pass_spans` 可额外测逐 pass 跨度，提交的真实完成点到达后才可读取和复用。命令与 pass span 分开统计，不能重复相加或冒充多队列帧延迟。Window Lab 用 `--gpu-pass-timing on` 保留有界的带名称样本；历史性能 A/B 保持双方相同观测配置。
- 未完成旧交换链代际最多保留 4 个。默认生产呈现要求 maintenance1，`FrameLoop::shutdown` 只接受真实 acquire/render/present 完成证明；显式兼容策略无法证明最终关闭时保留所有权并返回错误，详见进度记录。

## 焦点与全屏呈现

宿主设置 OS 窗口样式后，通过当前 `FrameLoop::set_window_mode` 请求 `Windowed`、`Borderless` 或 `Exclusive { monitor }`；焦点事件传给 `FrameLoop::set_focused(bool)`。这两个入口也直接提供在 `Swapchain` 上。`set_window_mode` 是 unsafe：独占请求的 Win32 `HMONITOR` 必须对应有效窗口及显示器。库不设置窗口样式、分辨率或选择显示器。外部创建的 Win32 surface 使用 `Surface::from_win32_raw`；普通 `from_raw` 不推断表面平台。

失焦或请求尺寸为零时暂停新 acquire；已有租约允许完成呈现，全部归还后由 `collect_presentations` 或下一次 acquire 释放独占权。恢复焦点会重新尝试独占。Windows 的扩展和实例依赖齐备时自动启用 `VK_EXT_full_screen_exclusive`，按相同 monitor/policy 查询 capabilities、formats、present modes 并创建交换链。模式变更走现有代际重建与呈现完成协议，释放独占不充当资源回收证明。

`swapchain.fullscreen_status()` 区分宿主请求、焦点、实际独占获取和 fallback。扩展/表面不支持、独占创建或获取被拒、原生独占丢失都会降级为普通呈现代际；`Windowed`/`Borderless` 在扩展启用时明确使用 DISALLOWED。降级原因保留到显式模式重试或真实焦点恢复，避免每帧重建。OOM、SurfaceLost 和 DeviceLost 保留错误类别。请求和实际状态可能在下一次 acquire 重建前不同。

Windows 独占获取/释放及显示器切换的驱动实测仍为 BLOCKED；跨平台状态机和故障测试不等同 Windows 原生验证。

## 渲染框架性能运行模式

`tests/p0_perf_baseline.rs` 现在包含同一 raster/copy/readback 场景的三条正式路径。三者默认都带 `#[ignore]`，避免混入普通测试：

```bash
# 1) 预生成固定动画时间轴（frame % 8）的参考像素/PPM；数据、参考、写盘分别计时
cargo test -p rhapsody --release --test p0_perf_baseline -- --ignored --nocapture render_perf_prepare

# 2) 严格验收：逐帧 wait、GPU 读回、与预生成参考逐像素比较；写 acceptance.json
cargo test -p rhapsody --release --test p0_perf_baseline -- --ignored --nocapture render_perf_strict_acceptance

# 3) 正式吞吐：默认 300 预热、3000 采样、3 次重复、3 帧在飞；
#    每 60 帧才异步抽样读回，不在每帧等待 GPU
cargo test -p rhapsody --release --test p0_perf_baseline -- --ignored --nocapture render_perf_throughput
```

`render_perf_throughput` 只接受同 `scene_fingerprint`、同设备/驱动、Release 构建且 validation layer 为空的 `acceptance.json`；缺失时直接失败。输出在 `target/render-perf/`：`prepared/manifest.json`、`prepared/reference-*.bin/.ppm`、`acceptance.json`、`bench-<timestamp>.json`。

可用环境变量覆盖采样参数：`RHAPSODY_PERF_WARMUP`、`RHAPSODY_PERF_SAMPLES`、`RHAPSODY_PERF_REPEATS`、`RHAPSODY_PERF_SAMPLE_EVERY`、`RHAPSODY_PERF_FRAMES_IN_FLIGHT`（1..=3）、`RHAPSODY_PERF_CACHE=on|off`（plan/subgraph/result 缓存一起开关）。正式模式拒绝低于 300/3000/3 的参数；冒烟用 `RHAPSODY_PERF_ALLOW_SHORT=1`。`RHAPSODY_PERF_ALLOW_DEVICE_CHANGE=1` 才允许复用其他设备上的验收。GPU 缺席默认跳过；CI 严格运行设 `RHAPSODY_REQUIRE_VULKAN=1`。

macOS/Homebrew 的 Vulkan loader/MoltenVK 已安装。workspace 的 `.cargo/config.toml` 默认设置 `RHAPSODY_VULKAN_LIBRARY=/opt/homebrew/lib/libvulkan.dylib`，初始化入口优先尝试该路径，再保留系统 fallback；用户环境覆盖优先。直接运行二进制需显式传该变量，不能依赖 Cargo 配置；仅配置 dyld 搜索变量可能被 Cargo 测试环境重写。普通沙箱若禁止 Metal/IOKit，仍需使用允许 GPU 访问的宿主执行。2026-10-18 的实际 Apple M3 严格链验证为 2 passed、0 SKIP，详见 [环境修复报告](../../reports/foundation/2026-10-18-vulkan-loader-config/README.md)。

报告分开记录 CPU begin/build/compile/record/submit、等待帧槽、GPU command-span 与逐 pass span、p50/p95/p99 帧间隔、CPU/GPU 每帧分配次数和分配峰值。离屏吞吐入口的 `present_mode` 明确为 `offscreen`；真实 WSI 的 acquire/present 与呈现模式仍由 Window Lab `--performance --gpu-pass-timing on` 覆盖，两者不能把离屏吞吐当作窗口 FPS。

统一窗口验收入口为 [Window Lab](examples/window_lab/README.md)，支持 Windows/Win32 和 macOS/AppKit/CAMetalLayer；其他平台明确退出。两种宿主共用 FrameLoop、素材、逐帧读回与报告。实例创建自动协商 portability 枚举扩展及 flag，设备启用广告的 portability subset，可运行 MoltenVK。示例构建、GPU 读回和最终窗口显示分别验收；实际覆盖见进度记录。

下列内容保留历史来源与迁入布局。**历史“原版尚缺”不构成本轮开发待办，也不约束新 API**；当前差距统一由开发计划和进度表维护。

## 历史迁入布局

| 模块 | 迁入时的实现范围 |
| --- | --- |
| `rg/graph`、`pass_builder`、`recorder`、`resource_info`、`compiled` | 图声明、访问账本、usage 标志、生命周期、批次编译、导入/导出、回调消费 |
| `rg/resource_registry`、`pass_bindings`、`attachments`、`pass_api` | typed resource Ref、managed 所有权、descriptor 绑定、附件缓存与 draw/dispatch 命令 |
| `rg/executor`、`parallel`、原 `state`、`temporal` | 迁入时的录制与序号、原 PGO 双缓冲及贪心分块、持久状态、跨帧资源配对/导出；原 `state` 已删除 |
| `raw/device`、`init`、`physical_device`、`instance` | 已知原设备/驱动/白名单规则、Vulkan 实例/设备/能力与队列调用 |
| `raw/buffer`、`texture`、`sampler`、`shader`、`descriptor_set` | 真实资源/视图、映射读回、shader 包消费、descriptor 与 compute/raster 管线缓存；光栅使用 dynamic rendering |
| `raw/managed`、`submit`、`semaphore`、`dynamic_constants` | 类型化代际弱索引与 Arc 持有、SyncCell 精确退役、时间线提交与有界常量页 |
| `raw/format::observed`、`resource_usage`、`crash` | 原格式入口、访问掩码换算、GPU assertion/debug packet 解析 |
| `rg/vulkan`、`raw/setup`、`texture_upload`、`swapchain` | 已恢复规则的可调用 Vulkan 适配；具体候选边界如下 |

重复的 `rg/compiled_graph` 与旧编译 ABI 投影已删除；`rg/resource_usage` 改为 re-export `raw/resource_usage`，不再维护两份同体拷贝。`raw/format` 的公共 Khronos 注册表与 `observed` 原私有入口分开；公共表不是新的游戏算法。原格式观察表已合入 `format_table` 的 `OBSERVED_*`，删除了上轮未接入模块树的重复 `format_data` 文件。

## 来源与逆向记录

源代码来自 [`reconstruction/rhapsody-rebuild`](../../../reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/README.md)。重要提交：`92b4ca26`（逆向取证）、`344b73f7`（缺失 typed 接口）、`52a14460`（原并行分块、依赖对齐与真实 blur pass）、`e4e0fbe2`（实际 shader 构建）、`db1ecad7`（原 compute 宏与接口修复）。每个迁入文件的路径、提交和快照哈希见 [migration-sources.json](migration-sources.json)。

另逐个核查旧 `render/work-packages` 的 21 个目录，归宿、后续原码纠正与 oracle/harness 排除原因见 [WORK_PACKAGE_AUDIT.md](WORK_PACKAGE_AUDIT.md)。其中原 sparse descriptor/tag/数组/dynamic offset 规则本次补入 `raw/descriptor_binding.rs`，没有以旧重建工程“看起来完整”代替闭包核查。

原始报告保留在 [`rhapsody-rebuild/evidence`](../../../reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/evidence) 与 [`evidence/render-recovery`](../../../reverse-engineering/tiny-glade/evidence/render-recovery)。其中 `MODULE_STATUS.md`、`DEPENDENCY_AUDIT.md`、`evidence/functional-project-validation.json`、`evidence/missing-functions-completion-2026-10-13.json` 分别描述局部合同和历史验证。历史输入哈希不自动覆盖本工程的 feature 隔离、模块迁移及未来修改。

## 历史恢复边界（非本轮待办）

有原指令证据的图编译、生命周期、PGO、格式和绑定规则已放入对应模块；整帧尚未验收不再成为排除这些实现的理由。可运行适配也不被冒称为原完整对象 ABI：`Device` 的 Vulkan 适配、主图 provider 桥、串行 session、同步 setup 上传、WSI 重建和诊断后端仍有登记的候选策略。`RasterPipelineOpts` 已按 0x1416a3720 还原为 4 字节位域（同时是 `RasterPipelineKey.selector`）；`[device+0xad9]` 已更正为 wideLines 门控并接入设备创建；子通道号恒 0 仍是候选。

`suballocation` 默认开启，启用公共 `gpu-allocator 0.27`：原版是子分配，B 版 Cargo.lock 记 0.26.0 本地分叉，但 PDB 中 `AllocatorReport` 只在 0.27 出现，故按 0.27 复用（见 Cargo.toml 注释）；该分叉源码仍未找到，`tools/verify_inventory.py` 对此依赖身份仍报差异。`validation` 特性让 release 构建在计划缓存未命中时也运行独立计划校验器（debug 构建始终运行，`rg::set_plan_validation` 可在运行期开启）；设备 `RecordingPolicy::Parallel` 时 session 使用自适应并行录制 `ParallelRecording::ADAPTIVE`（无统计或轻帧退回串行），调用方可显式设置 `ParallelRecording`。旧序号回收已由每队列时间线替代，整帧池优化按 V4 推进。没有重写 Bevy、ash 或分配库算法。`raw/descriptor_binding` 覆盖已取证的资源 tag 0–6、8–9；tag 7 等未知路径保留显式不支持边界，不把错误返回宣称为原完整分支恢复。

尚缺原完整设备排序/驱动编码、原 managed 槽位与销毁桶布局、首个真实 shader callback 自动绑定、原多队列 ownership transfer/调度、原 temporal 淘汰与 ABI、原窗口焦点/全屏策略、厂商故障转储、完整游戏 producer/consumer。原版没有硬件光追路径（`rt()` 恒为 None）；`should_reset_temporal` 属 country-core consumer。此历史断点曾缺 temporal 宿主接线；当前由 Session-owned 命名资源和普通 `execute_frame` 完成，三个宿主入口已迁移，真实运行证据见进度表。`get_setup_cb_submit_info` 已按 0x140bf84e0 实现为 `SetupCommandSlot::take_for_submit`；未恢复的 `set_fullscreen_exclusive`、`require_vendor_crash_dump` 占位方法没有迁入。

## 历史验证范围（不覆盖当前工作树）

迁入 `tests/` 覆盖图 callback、退役、managed key、绑定、附件、命令、PGO 和分配私有包装合同。少量管线测试需要 Vulkan，缺设备时会明确提示跳过，不能当真实 GPU 验收。原工程的格式 816 案例、GPU 读回、temporal/swapchain 等报告仅证明报告明确覆盖的子链和硬件；不证明整个游戏渲染/所有设备/三版本一致。正式工程的本次编译与测试记录见 [migration-validation.json](migration-validation.json) 及根工程审计报告。`tools/` 是来源库存刷新/复核辅助工具，不作为新的游戏逆向功能计数。

workspace 声明 Rust 1.88 下界以容纳原恢复代码已有的 let-chains，但没有在 Rust 1.88 编译器上实测。2026-10-16 缺陷修复轮在 rustc 1.96.0 + lavapipe 上：三个渲染包 `--all-features` 342 项、默认 feature 339 项测试通过，两种 feature 组合下 `clippy --all-targets -D warnings` 均通过；shader-compiler 原合同测试缺游戏资产时显式跳过（`TG_B_GAME_ROOT` 可指定）。早期测试计数保留为历史断点。

# rhapsody 重构剩余开发计划

本计划按 [DELIVERY_PLAN.md](DELIVERY_PLAN.md) 的 V0–V9 执行，覆盖其全部必做范围。目标接口与不变量见 [ARCHITECTURE.md](ARCHITECTURE.md)；当前实现与验证状态见 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)。

本文只列出仍需完成或需要复验的工作。已经存在于源码中的能力只在 REFACTOR_PROGRESS.md 登记，不在此重复。

## 1. 开发原则与完成定义

- 正确性与性能优先。允许破坏性 API 变更，不保留原版地址、布局、panic 文案或 oracle 兼容要求。依赖 ash、Bevy、gpu-allocator 等成熟库，不重写通用库。
- 一个工作包必须覆盖：声明或创建 → 真实调用者 → 提交或失败 → 回收。单独新增类型、re-export、CPU 模型或 demo 专用后端都不算完整交付。
- 每次改变公开接口，同步迁移实际下游。V9 做总审计。
- 旧路径只用于迁移与对照。替代链验收后删除；不在末期保留两套设备、完成状态、图编译器或呈现引擎。
- 缺 Vulkan、验证层、窗口或独立队列时，记录具体的 BLOCKED 或 UNSUPPORTED 项。可以推进不依赖硬件的工作，但不能跳过验收把版本记为 PASS。
- 完成条件以 DELIVERY_PLAN.md 第 1 节为准。GPU 驱动、mesh shader、程序化建模迁 compute、descriptor buffer、GPL、shader object 不作为前置。

## 2. 技术领域与交付版本映射

P0–P7 保留作技术索引，不能按 P 编号替代 V 的交付顺序。

| 技术领域 | 主要交付版本 | 边界 |
| --- | --- | --- |
| P0 基线与旧 oracle 清理 | V0，V9 对照 | 旧离屏基线不等于 Window Lab 的 M0 |
| P1 统一错误 | 各版持续，V9 扫尾 | 误用、OOM、DeviceLost 等类别贯穿调用链 |
| P2 设备与队列 | V1 | 真正合并设备所有权；别名队列不是独立队列 |
| P3 完成、回收、池 | V1、V2、V4 | V1 精确 last_use；V2 事务安全；V4 整池复位与局部池 |
| P4 资源、图、缓存、内存、管线 | V3–V7 | 分别验收，不压成一个任务 |
| P5 帧与呈现 | V2 | V4 的优化不能成为推迟呈现正确性的理由 |
| P6 图内多队列 | V8 | 配置辅助队列不等于多队列图执行 |
| P7 清理、性能、发布 | 各版与 V9 | 对照、删除、尾延迟与下游总验收 |

主依赖：V0 → V1 → V2 → V3 → V4 → V5 → V6 → V7 → V8 → V9。纯模型、夹具与报告格式可以提前准备，生产接线与版本验收按依赖推进。硬件阻塞只挂在受影响的验证项上。

## 3. 分版剩余任务

### V0：窗口基线

1. **Windows 基线**：在 Windows Vulkan 环境运行 Window Lab，覆盖默认素材遍历（不少于 300 帧，默认 360 帧）、窗口截图、缩放、最小化恢复、正常关闭与验证层结果。
2. **人工检查**：屏幕像素与键盘交互。需要具备相应桌面权限，或由人工执行。离屏读回不能替代。
3. **报告重建**：旧报告已删除。以当前源码重新生成 `report.json`、读回图与检查记录。

### V1：唯一设备、精确使用与统一回收

1. **文档与结构对齐**：ARCHITECTURE.md 中的 `Device` 组件描述需与 `src/device/logical.rs` 对齐。当前结构由 `extensions`、`lifetime`、`memory`、`queues`、`core` 组成，没有 `caches` 字段。
2. **独立硬件族验证**：在真实独立 graphics / compute / transfer 族上验证队列隔离、时间线独立推进与 1000 帧稳态计数。
3. **下游复验**：`tiny-glade`、`tg-text`、`country-core` 的构建与迁移复验。
4. **测试目标修复**：`examples/window_lab/scene.rs` 测试引用不存在的 `rhapsody::rg::Outcome`，需改为正确类型，使 example 测试目标可编译。

### V2：呈现闭环

1. **无 maintenance1 真机运行**：在 Intel UHD 630 或等价设备上运行 `Strict` 窗口路径，覆盖启动、resize、关闭，并记录 `QueueIdle` 证明的实际行为。
2. **超时分支验证**：`QueueIdle` 的超时分支目前无触发手段。需要设计一个不伪造完成的注入方式（例如测试 adapter），否则保持未验证。
3. **阶段故障矩阵**：在 acquire 后、录制中、首批与后续提交、present、回收各阶段注入失败，覆盖下一帧恢复与 temporal 历史失效。检查现有 CPU 测试是否全部经过生产协议。
4. **直接单元测试**：为 `GraphPresentation`（arm → submitted → present / abandon）补充直接测试。
5. **loader 复用**：`can_release_images` 与 `release_image` 的属性查询和 loader 改为复用；全屏配置查询的 surface loader 移出循环。
6. **Windows 呈现**：Win32 独占全屏的获取与释放、显示器切换、失焦驱动行为，需 Windows 环境，保留 BLOCKED。
7. **resize 与最小化压力**：按当前源码重新运行 100 次 resize 与 20 次最小化 / 恢复，确认无验证层 error 与资源归零。

### V3：图分层与独立检查

1. **分层复核**：确认 `CompiledPlan` 不持有 Vulkan 对象、回调、时间线值或设备查询；`FrameInstance` 校验导入描述、usage、初态与设备代际。
2. **规划器与状态机**：`PendingOwner`（源码中 18 处引用）与运行期状态机是否合并，需要核对。若保留两者，需在 ARCHITECTURE.md 中写明分工。
3. **窗口与 GPU 参考**：S03 光栅与 S04 temporal reset 在当前源码上重新运行，记录像素与参考结果。
4. **检查器覆盖**：随机小图与保守执行的对照，覆盖部分区间读写与读改写。

### V4：帧池、传输与预算

1. **分配复测**：固定图预热后，测量框架热路径的临时分配（目标 0 次/帧），并与素材解码、报告分配分开记录。
2. **S07 压力与长跑**：容量耗尽与恢复、非一致性路径、长跑中的预算回落。记录 p95 / p99。
3. **非一致性内存**：当前可用设备只覆盖一致性内存，non-coherent 对齐与可见性需在有此类内存的设备上验证。
4. **帧槽完成前的 reset**：确认拒绝路径在生产帧上有实际覆盖。

### V5：缓存

1. **S06 检查**：按实际执行与跳过次数、像素核对，而不只看命中计数。
2. **失效矩阵**：哈希碰撞、内容改变、结构改变、shader 改变、设备代际改变，分别验证失效。
3. **A/B 测量**：缓存开关的 A/B，键构造与边界协调成本计入总 CPU。
4. **依赖声明**：确认无法完整声明依赖的 pass 拒绝结果复用，不会因默认开启而误复用。

### V6：瞬态内存着色

1. **S05 对照**：开关前后像素一致；已知可复用案例的实际 committed 字节下降。
2. **依赖边检查**：默认 `PreserveParallelism` 下，额外同步不得增加依赖边。
3. **排除规则**：导入、temporal、持久导出、结果缓存、dedicated 资源的排除规则需要有测试覆盖。
4. **产物**：生命周期与偏移转储，峰值、碎片、分配次数与 p95 / p99。

### V7：动态渲染、bindless 与异步管线

1. **能力合同**：列明动态渲染与 bindless 的最低必需 feature 与 limits，区分基础模式与 bindless 模式，并写入 README。
2. **动态渲染**：生产源码已无 `VkRenderPass` / `VkFramebuffer`。S03 与参考输出需在当前源码上重新对照。
3. **bindless 延迟回收**：槽位在全部使用完成后归还，在飞 descriptor 不被提前复用。
4. **热重载**：合法与错误 shader 的连续切换；错误保留旧管线；坏缓存重建。
5. **DXC 协议**：`shader-compiler` 的 DXC 专用构建协议未完成。shader 工具可用不等于协议已完成，需要单独验收。
6. **性能门**：渲染线程同步管线创建次数为 0；记录首次与稳定帧的 p99 与后台队列峰值。

### V8：多队列

1. **真实独立族**：S08 在 graphics / compute / transfer 三个独立族上运行，覆盖 Off / Prefer / Require 与 Exclusive / Concurrent，检查像素、时间线与同步验证。
2. **偏序别名**：内存着色按 happens-before 偏序扩展；跨队列同时使用的资源不得别名。
3. **乱序完成**：随机不同完成顺序不提前回收。
4. **调度策略**：若引入 `MinimizeMemory` 类策略，先校验无环，并报告关键路径代价。当前源码中没有该策略。
5. **硬件门**：没有独立族的设备上，M8 保持 BLOCKED。

### V9：删除、下游与发布

1. **删除审计**：按 ARCHITECTURE.md 第 6 节逐项确认替代后的旧设备、生命周期、编译、屏障与呈现路径已删除；兼容桥、原版布局单测与地址注释已清理。仅保留一份屏障计算。
2. **provenance 审计**：rhapsody 的 provenance 审计调整为不再要求新设计迁就历史恢复。
3. **下游总审计**：`tiny-glade` 的 render host 与 raster host、`tg-text` 的 gpu 路径、实际 country-core 宿主、shader-pipeline 与 shader-compiler、相关测试，统一使用正式接口。不把未恢复的业务无限扩入本模块。
4. **裸句柄入口**：`unsafe` 契约逐项确认（例如 `FrameLoop::set_window_mode`、裸 Vulkan 命令与信号量入口），不能以安全函数签名掩盖生命周期责任。
5. **正确性长跑**：S01–S08，每种负载至少 30 分钟且不少于 10000 帧（稳定负载、效果切换、缩放压力）。
6. **性能对照**：同机、同驱动、同负载，预热 300 帧，采样不少于 3000 帧，重复 3 次；逐优化 A/B。p95 / p99 默认回退门为 5%。
7. **最终报告**：功能矩阵、支持硬件矩阵、下游全绿、旧机制删除清单、优化数据与关闭方式。

## 4. 设计问题的当前处理

| 问题 | 当前处理 | 仍需决定或验证的内容 |
| --- | --- | --- |
| 设备与缓存的 Arc 环 | 设备总是构造在 `Arc` 中，内部缓存只持 `Weak`（`src/device/logical.rs`） | V9 删除审计时确认没有遗留的强引用环 |
| SyncCell 的实现选型 | 仍使用 `SyncCell`（约 40 个源文件引用） | 按争用基准决定实现，目前没有基准结论，不凭实现名推断性能 |
| 帧槽必须管理真实池 | 帧 × worker × family 命令池已接入（`FrameCommandPool`） | 稳态分配与整池 reset 的最终测量（V4） |
| Serial 与 PerBatch 的默认值 | 旧文档描述的录制策略类型在当前源码中已不存在，该问题不再适用 | 若重新引入录制策略，需另行设计并写入 ARCHITECTURE.md |
| 最低 GPU 能力与 fallback | 能力检查与 `Unsupported` 返回已接入 | V7 冻结 bindless 与动态渲染的精确 feature / limits 与硬件矩阵；descriptor buffer、GPL、shader object 仍为可选 |
| SyncPoint 归属与裸 Vulkan 入口 | 设备、store 与图作用域校验已接入 | V9 逐项复核 unsafe 契约 |
| 下游 temporal 业务键 | 未在本次核对中验证 | 在 V3 与 V9 的下游审计中确认 |

## 5. 验证矩阵与报告

每个工作包记录：任务 ID、源码快照（提交与 dirty 状态）、生产调用链、CPU / GPU / 窗口 / 性能结果、已删除的旧机制、未覆盖的边界。验收产物沿用 Window Lab 的 `report.json`、读回图、窗口截图与检查记录。

在 `tiny-glade/` 工作区运行，默认与全特性分别检查：

```bash
cargo test -p rhapsody -p shader-compiler -p shader-pipeline --lib --tests --no-fail-fast
cargo test -p rhapsody -p shader-compiler -p shader-pipeline --lib --tests --all-features --no-fail-fast
cargo clippy -p rhapsody -p shader-compiler -p shader-pipeline --no-deps --all-targets -- -D warnings
cargo clippy -p rhapsody -p shader-compiler -p shader-pipeline --no-deps --all-targets --all-features -- -D warnings
cargo build -p tiny-glade -p tg-text --features tg-text/gpu --all-targets
cargo test -p rhapsody --example window_lab
```

涉及分配策略时补充 `--no-default-features`。Windows 专属代码必须在 Windows 目标上检查，其他平台跳过的模块不计为覆盖。Linux lavapipe 只记录软件设备的覆盖。真实窗口、多族与 GPU 性能需在对应硬件上另测。缺依赖或临时隔离 manifest 的结果，不能替代完整 workspace 检查。

`tools/refactor_checks.py` 与 `tools/window_lab_checks.py` 用于记录二进制身份、源码指纹与逐项结果，缺环境、验证层错误、源码变化与跳过都会单独记录。两个脚本的注释仍提到 `reports/` 目录，该目录已删除，使用前需确认输出路径。

## 6. 下一步（按顺序）

1. 修正 `examples/window_lab/scene.rs` 测试中的 `rhapsody::rg::Outcome` 引用，并确认 `cargo test -p rhapsody --example window_lab` 可编译。
2. 补充 `GraphPresentation` 的直接单元测试，并完成 loader 复用（V2 第 4、5 项）。
3. 冻结源码，按第 5 节的矩阵复跑默认与全特性的库、集成与 clippy 检查，记录结果到 REFACTOR_PROGRESS.md。
4. 在无 maintenance1 的设备上运行 `Strict` 窗口路径（V2 第 1 项）。
5. Windows 环境可用后，执行 V0 的 Win32 基线。
6. 在 V3–V9 的剩余任务中，按依赖顺序推进，每版完成后更新 REFACTOR_PROGRESS.md 的里程碑状态。

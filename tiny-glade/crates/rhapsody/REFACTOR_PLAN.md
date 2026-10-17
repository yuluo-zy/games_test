# rhapsody 完整重构开发计划

更新：2026-10-17。本计划按 [DELIVERY_PLAN.md](DELIVERY_PLAN.md) 的 V0–V9 执行，覆盖其全部必做范围。目标接口和不变量见 [ARCHITECTURE.md](ARCHITECTURE.md)；实际完成与验证只登记在 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)。本文件的任务均为开发要求，不因列在这里就表示已实现。

## 1. 开发原则与完成定义

- 正确性与性能为标准，允许破坏性 API 变更；不再保留原版地址、布局、panic 文案或 oracle 兼容要求。复用 ash、Bevy、gpu-allocator 等依赖，不重写通用库。
- 一个工作包必须覆盖声明/创建 → 真实调用者 → 提交/失败 → 回收；单独新增类型、re-export、CPU 模型或 demo 专用后端不算完整交付。
- 每版按接口与不变量、CPU 验证、生产接线、Window Lab、GPU/窗口验收、性能对照与报告推进。每次改变公开接口，同步迁移实际下游，V9 再做总审计。
- 旧路径只用于迁移和对照。替代链验收后删除，不在末期保留两套设备、完成状态、图编译器或展示引擎。
- 实现、接线和验收分开记录。缺 Vulkan、验证层、窗口或独立队列时记录具体 BLOCKED/UNSUPPORTED 项；可以推进不依赖它的工作，不能跳过验收并把版本记为 PASS。
- 完成条件是 M0–M9 满足交付计划门槛、S01–S08 有有效报告、下游迁移与旧路径删除完成。GPU 驱动、mesh shader、程序化建模迁 compute、descriptor buffer、GPL、shader object 不作为本轮前置。

## 2. 技术领域与交付版本映射

P0–P7 保留作技术索引，不能按 P 编号先后替代 V 交付顺序。

| 技术领域 | 主要交付版本 | 边界 |
| --- | --- | --- |
| P0 基线与旧 oracle 清理 | V0，V9 对照 | 旧离屏基线不等于 Window Lab M0 |
| P1 统一错误 | V1 起，各版持续，V9 扫尾 | 误用、OOM、DeviceLost 等类别贯穿调用链 |
| P2 设备与队列 | V1 | 真正合并设备所有权；别名不是合并 |
| P3 完成、回收、池 | V1、V2、V4 | V1 精确 last_use；V2 事务安全；V4 整池复位和局部池 |
| P4 资源、图、缓存、内存、管线 | V3–V7 | 分别验收，不压成一个“类型化资源”任务 |
| P5 帧与呈现 | V2 提前交付 | V4 的优化不能成为推迟呈现正确性的理由 |
| P6 图内多队列 | V8 | 配置辅助队列不等于多队列图执行 |
| P7 清理、性能、发布 | 各版与 V9 | 对照、删除、尾延迟及下游总验收 |

主依赖：V0 → V1 → V2 → V3 → V4 → V5 → V6 → V7 → V8 → V9。纯模型、夹具和报告格式可提前准备；生产接线与版本验收按依赖推进。硬件阻塞只挂在受影响的验证项上，不能以扩大单测数量替代实测。

## 3. 分版开发任务

### V0：冻结可复现窗口基线

前置：现有 device / graph / swapchain 可运行；暂时允许保守等待和离屏、展示分段提交。

1. **V0.1 素材与操作**：审计 `examples/window_lab/` 的 S01/S02、PPM 播放列表、固定步长、暂停/单步/切换/退出和有限帧协议；补齐脚本事件的真实发生记录。
2. **V0.2 报告**：记录代码快照及 dirty、fixture 哈希、设备/驱动/能力、验证层、离屏比对和窗口检查；缺指标用 null/unsupported。保存可定位到源代码版本的报告与读回图。
3. **V0.3 实测基线**：在 Windows Vulkan 环境运行至少 300 帧并遍历全部默认素材；现成命令采用 360 帧。补窗口截图、缩放/最小化/恢复、正常关闭和验证层结果。

出口：M0 报告与图像齐备，实际窗口检查完成；现有离屏 P0 日志只能作历史参考。入口与命令见 [Window Lab README](examples/window_lab/README.md)。

### V1：唯一设备、精确使用与统一回收

前置：V0 提供对照负载；复用现有 `sync/` 和队列规划。

1. **V1.1 配置与能力**：完成 `device/{init,caps,policy,generation,queue_config}` 与实际创建设备的一致性。区分 supported/enabled/实际实现策略；校验设备选择、surface、必需能力、1–3 在飞帧、分配/录制策略。核对新旧创建入口及 submit/submit2 路径，保留原始错误类别。
2. **V1.2 设备合并**：梳理 `raw/device.rs` 的两层设备、managed store 与各缓存的所有权；避免 Device → 缓存资源 → Arc<Device> 循环。把真实设备、队列、分配、缓存、回收合入 `Device`，迁移 producer/consumer，删除 `VulkanDevice`/`GraphicsDevice` 双层及旧选择白名单路径。
3. **V1.3 精确 last_use**：让 buffer/image/view/sampler/pipeline/descriptor/命令所引用对象进入资源持有集合；成功提交后无失败地登记 SyncPoint，drop 只按本对象全部实际使用退役。取消未提交引用不会提前释放已提交使用。审查 SyncCell 的 reset/并发约束，不把原子计数当成资源持有协议。
4. **V1.4 清掉旧完成账本**：用 SyncSet + Retirement 替代 SequenceRuntime、兼容 SubmissionEngine/DeviceState、CommandBufferSeqTracker、disposal 桶和 ManagedResources dropped 记录。此版可按命令分配退役，V4 再优化为整池 reset；不能保留第二套完成真相源等待 V4。
5. **V1.5 错误与下游**：迁移 raw/managed 的公共错误和调用方可触发 panic；覆盖过期、跨设备、旧代际、未提交等待和 DeviceLost。同步修改 `tiny-glade`、`tg-text` 及实际调用 rhapsody 的 `country-core` 宿主接口。

出口：M1 的失败注入、独立队列锁和 timeline 验证；Window Lab 1000 帧热身后计数稳定；关闭释放与旧机制删除清单。Device 别名和全队列 submitted 快照回收均不满足出口。

### V2：事务提交与真实呈现闭环

前置：V1 唯一设备与精确回收。使用最小帧槽管理，后续由 V4 优化。

1. **V2.1 FrameTransaction**：提交前预留资源引用、回执和退役容量；逐批成功后立即登记真实完成点及访问/ownership 终态。部分成功不能回滚；失败清理不无限等待，保留主错误，清理与回调各执行一次。
2. **V2.2 Acquire 与 present 协议**：实现 `AcquiredImageLease`、交换链代际和按图像槽的二值 present 信号量；分别记录 acquire 是否消费、布局、提交和呈现状态。补救转换、release-images、二值信号量收尾分别处理。
3. **V2.3 PresentTicket**：实现启用 maintenance1 的 fence 路径和无扩展的代际回退；渲染 timeline 完成不能单独释放呈现资源。DeviceLost 停止补救提交。
4. **V2.4 FrameLoop 接线**：主图和展示纳入同一事务/执行协议，Window Lab 改用正式 FrameLoop。零尺寸 Skipped、实际重建后 Recreated；移除 demo 自管提交的替代路径，迁移窗口宿主。
5. **V2.5 故障矩阵**：在测试 adapter 按阶段/调用序号注入 acquire 后、录制、首批/后续提交、present、回收失败；覆盖下一帧恢复与 temporal 历史失效。

出口：M2 完整失败矩阵；100 次 resize、20 次最小化/恢复，无重复消费、在飞 reset 或验证层 error；关闭资源归零。正常帧协议先闭合，再开展图和性能改造。

### V3：图分层、类型化资源和独立检查

前置：V2 有可靠帧与失败协议；此版 S03 可使用既有光栅路径，V7 再迁动态渲染。

1. **V3.1 资源身份**：接通 `resource/{handle,tracked,sharing}` 与真实 Buffer/Texture/View/Sampler；补设备/store/图作用域身份，拒绝同型跨 store、过期和跨图句柄；Session-owned 命名 persistent/temporal 取代公开 u64 token，首帧/resize/reset 管理 history_valid，名字随会话所有权释放。
2. **V3.2 声明与版本**：GraphDefinition 显式声明 TextureId/BufferId、每次写版本、aspect/mip/layer 与 offset/size、导出和副作用；拒绝未知来源读取和非法范围；保留原地写反依赖。
3. **V3.3 纯计划**：拆开 `rg/compiled.rs` / `rg/vulkan.rs` 中规划、资源分配与真实绑定。CompiledPlan 不含 Vulkan 对象、回调、时间线值或可变设备查询；FrameInstance 校验导入描述、usage、初态、代际并持有本帧资源。
4. **V3.4 裁剪与检查器**：导出/副作用作为根，裁剪无用 pass；提供可读计划转储。独立规则验证读取来源、冲突依赖和回收覆盖，随机小图与保守执行作对照。
5. **V3.5 场景与迁移**：S03 光栅及后处理、S04 temporal 乒乓/reset；迁移下游图 API，删除 u64 resolver、旧 temporal token 与替代的 managed 句柄/checksum 路径。

出口：M3 契约/随机图测试和 GPU 参考结果；生产帧使用新分层，计划可跨帧复用但不携带帧状态。

### V4：帧局部池、异步传输和预算

前置：V1/V2 使用登记完整，V3 资源绑定明确。

1. **V4.1 完整帧槽**：在实际图后端闭合多批次 SyncSet 累积、未提交取消、部分成功和动态 1–3 槽配置。每帧 × 实际执行 worker × family 独占 Vulkan 命令池，全部相关队列完成后整池 reset；删除没有实际消费者的独立 CPU FrameRing。
2. **V4.2 局部临时内存**：帧描述符过渡池、动态常量环与 CPU arena；CPU 任务完成与 GPU 完成分别控制复用。局部池有字节上限、空闲回流与代际，录制策略按实际 worker 取池。
3. **V4.3 Upload/Readback**：实现区间预留、完成 ticket、非阻塞 try_read、有界环及背压；host 写与并发提交的区间登记互斥，正确 flush/invalidate 和 non-coherent 对齐。
4. **V4.4 预算闭环**：将 MemoryBudgetController 接实际分配、缓存、在飞和退役；定义分类转换避免重复记账。实现高低水位、只淘汰已完成空闲对象、低优先任务延后、OOM 收尾和有界后台队列。
5. **V4.5 上传缺陷与指标**：修复 cube 全层上传与合法 BC 格式块尺寸覆盖；S07 执行耗尽/恢复、非 coherent 和长跑。单独计框架临时分配、素材和报告分配。

出口：M4；在飞池 reset 被拒绝，正常帧无 device/queue idle；固定图预热后框架临时分配目标 0 次/帧，报告 p95/p99、预算峰值与回落。

### V5：整图、子图和结果缓存

前置：V3 纯计划/版本与 V4 预算/退役。三个子版均为本轮必做，是否默认启用由收益决定。

1. **V5a 整图**：规范化完整键覆盖所有烘焙输入、编译器版本、能力和队列；哈希定位后比较完整键。绑定时重算动态边界；条目数/字节有上限，记录失效原因。
2. **V5b 子图**：实现 SubgraphInterface 与稳定局部计划；全局统一跨子图同步、队列和内存放置，局部缓存不得固化全局偏移/完成值。
3. **V5c 结果**：内容版本、参数、shader 实现、相机/光源/历史等完整依赖驱动失效；命中保留产出完成依赖并登记新使用；输出转为持久、受预算控制且排除普通瞬态着色。
4. **V5.4 下游与 S06**：接入版本化几何/阴影/文字夹具和下游脏标记；测实际执行/跳过次数和像素。无法完整声明依赖的 pass 拒绝结果复用。

出口：M5；强制哈希碰撞、内容/结构/shader/代际改变分别正确失效；稳定图预热后结构编译 0 次。A/B 包含键构造与协调成本，无收益时默认整图策略，但子图/结果功能不能据此省略。

### V6：单队列瞬态内存着色

前置：V3 生命周期/纯计划、V4 预算和帧槽、V5 持久结果边界。

1. **V6.1 内存事实**：查询真实 requirements/dedicated；以不可变事实和设备兼容签名供放置器使用，不能用估算纹理大小替代。
2. **V6.2 放置**：实现兼容分组、size/alignment/memoryTypeBits/granularity 约束和贪心偏移；排除导入、temporal、持久导出、结果缓存和 dedicated。不同在飞帧槽不得无依赖复用区间。
3. **V6.3 唯一 backing owner**：多对象共用一个释放责任，汇总全部使用；别名切换生成执行/内存/布局依赖，首次未初始化读被拒绝。
4. **V6.4 独立检查与 S05**：默认 PreserveParallelism 只复用已有依赖可证明的区间，不新增边；转储生命周期与偏移，校验并存资源不重叠、backing 释放一次。

出口：M6；着色开关像素一致，已知可复用案例实际 committed 字节下降；报告峰值、碎片、分配次数、额外同步与尾延迟，不预设统一节省比例。

### V7：动态渲染、bindless 和异步管线

前置：V1 能力模型、V3 资源/图、V4 有界任务与 V5 失效协议。

1. **V7.1 能力合同**：列明最低必需 feature 位与 limits；基础模式及 bindless 模式分别检查，不因硬件支持某扩展就启用未实现路径。不支持必需路径明确 Unsupported，并保留硬件验收缺口。
2. **V7.2 动态渲染**：光栅与管线改用 rendering attachments / PipelineRenderingCreateInfo；S03 与参考输出对照后删除 raw render_pass/framebuffer 及缓存。
3. **V7.3 Bindless**：实现全局堆、索引、视图和 shader 参数协议，覆盖 sampled/storage image、buffer、sampler 及实际 limits；槽位按全部使用完成点延迟归还。迁移 shader 包、宏/编译器和下游消费者，删除逐 dispatch 临时描述符路径。
4. **V7.4 异步管线**：规范化请求同键合并、有界优先级队列、显式 fallback/跳过策略；磁盘 cache 校验设备/版本、原子保存、损坏重建。
5. **V7.5 热重载**：帧边界发布新版本，失败保留旧管线；旧对象安全退役，shader 变化传播结果缓存失效；S03 连续变体与合法/错误 shader 覆盖。

出口：M7；渲染线程同步创建管线 0 次，在飞 descriptor 不提前复用，同键只编译一次，坏缓存不崩溃，记录首次/稳定帧 p99 与后台队列峰值。

### V8：多队列图调度与偏序别名

前置：V2 多批事务、V3 访问/检查器、V6 放置与别名协议。

1. **V8.1 分批**：QueueClass 经实际 QueuePlan 生成批次与跨队列时间线等待；别名队列退化为单队列，不新增无用同步。
2. **V8.2 Ownership**：Exclusive 配对 release/acquire，Concurrent 用实际 family 集合；迁移当前跨族拒绝分支，覆盖首次导入和最终导出。
3. **V8.3 全局偏序**：按全部访问的 happens-before 扩展着色与检查器；不同完成顺序不能提前回收。MinimizeMemory 若新增依赖，先校验无环并报告关键路径成本，默认保持并行性。
4. **V8.4 S08 真机**：分别覆盖 Off/Prefer/Require、Exclusive/Concurrent；真实独立 graphics/compute/transfer 的依赖链、像素、时间线及同步验证。自动关键路径迁移按测量决定，不把启用辅助队列当作吞吐收益。

出口：M8；只有别名队列测试时仍 BLOCKED（缺硬件），不能宣称真实多队列验收通过。

### V9：删除、下游与发布总验收

前置：各版必做功能与有效验收记录齐备。

1. **V9.1 删除审计**：按架构 §6 清除替代后的旧设备/生命周期/编译/屏障/呈现路径、兼容桥、原版布局单测与地址注释；仅保留一份屏障计算。调整 rhapsody 的 provenance 审计，不让历史恢复要求重新约束新设计。
2. **V9.2 下游总审计**：`tiny-glade/{render_host,raster_host}`、`tg-text/gpu`、实际 country-core consumer、shader-pipeline/compiler 和相关测试统一正式接口；不把历史未恢复业务无限扩入本模块范围。
3. **V9.3 正确性长跑**：S01–S08，至少 30 分钟且 10000 帧；稳定负载、效果切换、resize 压力分别测；无验证层 error、像素回归、未解释挂起或无界增长。
4. **V9.4 性能与发布**：同机同驱动同负载预热 300 帧、采样至少 3000 帧、重复 3 次；分离框架 CPU/GPU 与 vsync，逐优化 A/B。p95/p99 默认回退门为 5%，超过则修复或关闭，噪声过大扩大采样。

出口：M9 最终功能/硬件矩阵、下游全绿、旧机制删除清单、优化数据及关闭方式。任何必做未验证路径或开放正确性缺陷都保留未完成。

## 4. 开工时必须收敛的设计问题

| 问题 | 收敛节点与处理 |
| --- | --- |
| 两层设备与缓存的 Arc 环 | V1.2 先画所有权表，确定各缓存/存储的持有和销毁顺序，再合并设备 |
| SyncCell 原子实现与架构 Mutex 起步不同 | V1.3 审查并发/生命周期，不凭原子实现名称推断更快；按争用基准选型 |
| 帧槽必须管理真实池 | V2 明确多批次和取消合同，V4 在图后端按 frame/worker/family 管理真实池，不能用独立 CPU 状态模型冒充完整帧事务 |
| Serial 与目标 PerBatch 默认不一致 | 保留当前 Serial 的实测事实；V4 明确 PerBatch 语义并测单批单命令，避免在文档中提前宣布已切换 |
| 最低 GPU 能力/扩展与 fallback | V1 明确基础设备合同，V7 冻结 bindless/dynamic rendering 精确 feature/limits 与硬件矩阵；descriptor buffer/GPL/shader object 仍可选 |
| SyncPoint/句柄归属与裸 Vulkan 入口 | V1/V3 明确设备、store、图作用域校验；裸句柄提交必须有明确 unsafe 契约或受控包装，不能以安全函数签名掩盖生命周期责任 |
| 下游 temporal 的业务键尚有历史依赖 | V3 用命名资源/显式版本建立正式适配；若阻塞真实 consumer，单列依赖和替代边界，不能由独立夹具宣称游戏接线已完成 |

## 5. 验证、报告与当前起步顺序

每个工作包记录：任务 ID、代码快照、生产调用链、CPU/GPU/窗口/性能结果、已删旧机制、未覆盖边界。验收产物沿用 Window Lab `report.json`、读回图、窗口截图和检查记录，不创建另一套替代验收协议。

在 `tiny-glade/` 工作区运行，默认特性与全特性分别检查：

```bash
cargo test -p rhapsody -p shader-compiler -p shader-pipeline --lib --tests --no-fail-fast
cargo test -p rhapsody -p shader-compiler -p shader-pipeline --lib --tests --all-features --no-fail-fast
cargo clippy -p rhapsody -p shader-compiler -p shader-pipeline --no-deps --all-targets -- -D warnings
cargo clippy -p rhapsody -p shader-compiler -p shader-pipeline --no-deps --all-targets --all-features -- -D warnings
cargo build -p tiny-glade -p tg-text --features tg-text/gpu --all-targets
cargo test -p rhapsody --example window_lab
```

涉及分配策略时补 `--no-default-features`；Windows 专属代码必须在 Windows 目标检查，其他平台跳过模块不算覆盖。Linux lavapipe 只记录软件设备覆盖，真实窗口、多族和 GPU 性能另测。缺依赖或临时隔离 manifest 的结果不能替代完整 workspace 检查。

执行优先级按用户指示：先闭合 V1 剩余源码与消费者，再接续 V2 的生产帧协议和故障矩阵，随后推进 V3–V9。Windows 运行验收后置；本机可用的 MoltenVK 环境用于验证当前源码，硬件或桌面权限缺口不作为其它源码停工条件。已有代码保留并按进度记录接续，不再新开孤立类型模块；CPU 注入必须调用生产协议，不能用另写状态模型代替真实接线。

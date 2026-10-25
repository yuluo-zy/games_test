# rhapsody 渲染性能架构分析:从 2 FPS 到 120 FPS

2026-10-21。只读分析,未构建、未运行。证据来自三条线:原版二进制静态取证(`reverse-engineering/tiny-glade/evidence/render-recovery/producer-closure/SYNTHESIS.md` 等)、游戏侧每帧调用链(`crates/tiny-glade/src/production_renderer.rs`、`country-core/.../world_graph.rs`)、rhapsody 内部热路径(`crates/rhapsody/src/rg/*`、`resource/range_state.rs`、`raw/command_buffer.rs`)。标注 **[C]** 为代码确认,**[I]** 为推断,**[U]** 为未知。

## 0. 先说三件必须先接受的事实

1. **真机上没有任何分阶段计时数据。** `TG_FRAME_TIMING=1` 的 `render_stages` 日志已接好(`production_renderer.rs:1251`),但 reports 里没有一条真机输出。唯一的数字是 debug + lavapipe + validation layer 在 480×270 的基线(`main-loop-render-recovery-2026-10-19/MULTIFRAME-BASELINE-2026-10-20.md`):App update 24.6 ms、声明 227 ms、编译 349→11 ms(计划缓存修复后)、录制 195 ms。这些不能换算成 release 真机数字,只能看相对比例。
2. **"2 FPS" 的测试机是 Intel UHD 630。** 原版 Tiny Glade 的最低配置是 GTX 1060 / RX 580 级别。UHD 630 大约是 GTX 1060 的 1/4~1/5 算力,而这个场景每帧 160 个 pass,包括软件 BVH 的 ray-traced GI、SSGI、ReSTIR 直接光、体积光、TAA、4×MSAA 最终目标。**即使 CPU 侧完全归零,UHD 630 在完整画质下也极大概率达不到 120 FPS。** 120 FPS 这条线是 CPU 架构目标(每帧 CPU ≤ 3 ms,帧率只由 GPU 决定),GPU 侧能不能到 8.3 ms 要在验收标准指定的硬件上测,或者由画质分级决定。
3. **"GPU 未跑满"不能证明是纯 CPU 瓶颈。** 当前实际只有一帧在飞(见 §2.1),CPU 和 GPU 串行相加,GPU 占用率天然读数低。

## 1. 原版设计 vs 当前设计

| 维度 | 原版(静态取证) | 当前 rhapsody + 宿主 | 差异对性能的影响 |
|---|---|---|---|
| 图生命周期 | 每帧重声明、重编译整张图(kajiya 风格),无编译缓存 [I] | 每帧 `RenderGraph::new()` 两次(主图 + 展示图),编译有 128 项 LRU 计划缓存 [C] | 原版靠极低的常数因子撑住;当前声明阶段常数因子大(§2.3) |
| 每帧图数量 | 主图执行后 acquire,再跑展示图 [C] | 相同 [C] | 相同结构,但当前把两张图各算一帧占用在飞槽 |
| 帧在飞 | `PendingSubmission` deque,begin_frame 时 len>1 才等,约 2 帧 [I] | `frames_in_flight=2`,但**每张图**都算一帧 [C] | **实际 1 帧在飞,CPU+GPU 串行** |
| 屏障 | 录制时按前次访问链即时计算,只在不可并发处插 [C] | 规划期 `sync_plan` 生成 RangeProgram → 每帧 `bind` 到物理 SyncCell → 提交时再 replay 一遍 patch 校验 [C] | 同一份状态算三遍;3270 次转换/帧 |
| 描述符 | 一个持久 bindless 纹理集 + 每 dispatch 瞬时 set(池 100,free-list 复用)[C] | 全 bindless,每个 draw 构建一份参数快照(Vec + image_view 查表 + prepare)[C] | Shadows×3 录制 92 ms(debug)就是这个 |
| 绘制提交 | HZB cull + `multi_draw_indexed_indirect`,GPU 驱动,跑两轮 [C] | 有 Nani cull & bucket ×10,但光栅 pass 仍逐 draw 录制并逐 draw 准备参数 [C] | CPU 成本 O(draw 数) 而非 O(列表数) |
| 上传 | CpuToGpuLowPrio 暂存 + 设备级 setup 命令缓冲,提交前排在图命令之前,**不等待** [C] | `BufferHost::update_buffer_bytes` 对 GpuOnly 缓冲走 `upload_buffer_ranges_wait(u64::MAX)`,每 1 MiB 批次等一次 fence;W11 每帧全量上传建筑 [C] | 声明阶段内嵌 GPU 往返 |
| 并行录制 | PGO 分块,`par_chunk_map_mut`,2~16 块 [C] | 实现了,但 PGO 统计按图交换,主图读到的是展示图的统计,主图大概率一直串行 [I] | 并行录制形同虚设 |
| 队列 | 单图形队列,无 async compute、无 transfer 队列 [C] | 可配置多队列,默认别名 | 不是当前瓶颈 |
| 管线 | 首次使用时创建,靠 VkPipelineCache [C] | 缓存命中无分配;但 `prewarm` 每帧两次扫全部 ticket [C] | 稳态浪费,新变体同步编译卡顿 |
| 验证 | 无 | release 下 plan_check 关、validation layer 关 [C] | 不是问题 |

结论:原版的"每帧重建"之所以能跑,是因为它的每帧工作量是 O(pass + access) 且常数极小,所有重活(剔除、draw 列表)在 GPU 上。当前实现保留了"每帧重建"的外形,却在每一层加了 O(U×S)、O(N×T)、O(cells×patches) 的扫描和二次校验,再叠加一帧在飞和同步上传。**问题不在"重建图"这个范式,而在常数因子与串行化。**

## 2. 2 FPS 的成因链(按确定度排序)

### 2.1 实际只有一帧在飞 [C]
- `VulkanGraphSession::begin_frame` 在 `pending >= frames_in_flight` 时阻塞(`rg/vulkan.rs:2433-2436`),每次 `execute_prepared` 都 `end_frame` 计一帧(`:3058`)。
- 一帧显示 = 主图 + 展示图 = 2 个 session 帧,默认 `frames_in_flight=2`(`device/init.rs:71`)。
- 结果:声明第 n 帧主图前要等第 n-1 帧主图 GPU 完成;声明展示图前要等第 n-1 帧展示图完成。CPU 声明/录制时间与 GPU 时间**相加**而不是重叠。
- `acquire_swapchain_image` 还会在 `pending >= frames_in_flight` 时等 `PresentTicket`(`production_renderer.rs:1647`)。

### 2.2 声明阶段内嵌同步 GPU 往返 [C]
- `BufferHost::update_buffer_bytes`(`render_host.rs:173-221`)对所有 GpuOnly 缓冲(typed SSBO `memory=1`)调用 `upload_buffer_ranges_wait(..., u64::MAX)`,`transfer.rs:510-607` 每 1 MiB 批次等一个 fence。
- `world_terrain.rs:72-78`(W11)每帧重新上传全部 grounded buildings,还分配 HashMap + Vec。
- 每次等待都是一次 CPU→GPU→CPU 完整往返,在一帧在飞的前提下相当于串行再插一段 GPU 延迟。

### 2.3 声明阶段的重复工作 [C]
- VAO 快照全量重建:`build_draw_lists` 一次 + 7~8 次 `SceneRasterHost::new`(W23/25/30/34/43/44/46/53)+ W5 一次,每次扫 21 个列表 + ECS 查询 + 两个 HashMap(`render-cpu-hotpath-audit.md`)。
- `ShaderHost::prewarm` 每帧两次克隆全部 compute/raster 条目并逐个 `wait`(`render_host.rs:1595-1626`)。
- `raster_has_previous_contents` 对每个 LOAD 附件线性扫前面所有 pass/access(`raster_host.rs:2353`)。
- `frame_sampled_table` 每个光栅 pass 克隆全部视图并逐个 import(`render_host.rs:1417`、`raster_host.rs:1332-1388`)。
- `validate_graph` 每帧跑两遍(`vulkan.rs:2932` 与 `:1525`)。
- 命名/temporal 资源每次调用 `format!` 构造 String 键(`vulkan.rs:2678, 2739, 2774-2776`);别名键是 `BTreeMap<String,_>`(`transient_resources.rs:486-618`)。

### 2.4 rhapsody 内部每帧的超线性扫描 [C]
- `sync_plan::bind::bind_use` 对每个 use 遍历**所有资源的所有 shadow cell**(`bind.rs:413-417, 431-436`):O(U×S)。160 pass、3270 转换的图上这是百万级比较。
- `recorder.rs:34-41` 每个 pass 扫整条 bound transition 列表:O(N×T)。
- `vulkan_queue.rs:358-424` 每个 batch 过滤全部 patch/transition/wait 列表:O(B×P)。
- 提交时 `prepare_submission_states`(`raw/command_buffer.rs:640-711`)对每个 cell 按 `Arc::ptr_eq` 过滤每条命令的全部 patch,再把 patch 重新 apply 到 `RangeStateTable`(`range_state.rs:479-574`)并排序合并——规划器已经算过一遍的状态在提交时**全量重放**。
- 动态常量页每次 host 写入取 3 把锁并 prepare/publish 一个 range;页内 cell 随写入增长,K 次写入约 O(K²)(`raw/dynamic_constants.rs:239-243`)[I]。
- `refresh_memory_budget` 每帧调 `vkGetPhysicalDeviceMemoryProperties2`(`device/lifetime.rs:65`)。
- PGO 每 pass 两次 `Instant::now()` + 两次 SipHash 字符串 map 插入,且无开关(`executor.rs:1339-1405`)。

### 2.5 录制阶段逐 draw 构建 bindless 参数 [C]
- `raster_host.rs:1685, 2144-2253`:每个 draw 分配 Vec、查 image_view、prepare 参数块。
- debug 基线里 Shadows×3 92.7 ms、G-buffer 87.8 ms、G-buffer OC/2 45.4 ms,三者占录制总量 195 ms 的大头。

### 2.6 其他
- Bevy 单线程:没有 crate 启用 `bevy_ecs/multi_threaded`,`ExecutorKind::SingleThreaded`;debug 下 App.update 25 ms [C]。
- 160 个 pass、109 个不同名字(Shadows×3、Nani cull & bucket×10、Clear buffer×6)。pass 数本身放大了 §2.4 的所有 O(N·) 项。

**量化估计(非测量)**:把 debug/lavapipe 的 CPU 806 ms 按 release 常见 5~10× 折算,CPU 侧约 80~160 ms;加上 UHD 630 上 160 pass 的 GPU 时间(未知,可能 100~300 ms)和同步上传往返,串行相加得到 300~500 ms/帧,与 2 FPS 吻合。但这是推断,**第一步必须真机跑 `TG_FRAME_TIMING=1` 拿到 `declare/prewarm/compile/record_submit/present_ticket_wait/acquire/resolve` 的分项**。

## 3. 重构方案

目标拆成两条独立的线:**CPU 线**(架构问题,我们能控制)目标每帧 CPU ≤ 3 ms;**GPU 线**(硬件 + 画质问题)在目标硬件上单独剖析。

### 第 0 步:测量(半天)
- 真机 Release、960×540、完整存档,`TG_FRAME_TIMING=1`,记录 30 帧 `render_stages`。
- 恢复顶层 GPU timestamp span(`last_gpu_frame_ns` 当前返回 `None`,`production_renderer.rs:1739`),rhapsody 已有 per-pass 时间戳接口(`rg/profiling.rs`),默认关(`vulkan.rs:662`)。打开它拿每 pass GPU 时间。
- 这两组数字决定 A/B/C 三层的优先级。没有它们,下面全是按代码推断的排序。

### 第 A 层:不改架构的止血(1~3 天,预期 2 → 10~30 FPS)[I]
| # | 改动 | 位置 | 依据 |
|---|---|---|---|
| A1 | 主图 + 展示图合并成一张图,或 session 按"显示帧"而不是"图"计数;`frames_in_flight` 提到 3 | `rg/vulkan.rs:2433, 3058, 3146`;`production_renderer.rs:1305, 1647` | §2.1。ARCHITECTURE §3.8 本来就要求"主图与展示图合并为一张图,交换链图像作为 Import" |
| A2 | `update_buffer_bytes` 改为异步 ticket(rhapsody 已有 `UploadManager` ticket 设计),帧内绝不 `wait`;W11 建筑上传改脏区/变化时上传 | `render_host.rs:173-221`、`world_terrain.rs:72-78` | §2.2。原版 setup 命令缓冲就是不等待的 |
| A3 | `prewarm` 改为"待决 ticket 集合",只等本帧新注册的;稳态零扫描 | `render_host.rs:1595-1626` | §2.3 |
| A4 | VAO 快照从 9 次降到 1 次:W23 前刷新一次,带版本号的 scoped 快照贯穿 W53;`refresh_vao_handles` 直接调用使快照失效 | `scene_raster.rs:67`、`raster_host.rs:349` | `render-cpu-hotpath-audit.md` 已论证安全区间 |
| A5 | 启用 `bevy_ecs/multi_threaded` | Cargo features | §2.6,App.update 是独立的一段 |
| A6 | PGO 统计按图分槽,让主图真正走并行录制;录制线程数按核数 | `executor.rs:960, 2082` | §1 并行录制行 |
| A7 | `refresh_memory_budget` 改每 60 帧或后台;PGO `Instant`/SipHash 加开关 | `device/lifetime.rs:65`、`executor.rs:1339` | 小项 |

### 第 B 层:rhapsody 内部把每帧复杂度降到 O(N+A)(1~2 周,目标框架开销 < 2 ms)[I]
| # | 改动 | 核心思路 |
|---|---|---|
| B1 | **sync bind 增量化** | 计划缓存命中时,`BoundTransition` 列表随计划缓存;每帧只对"导入资源初态变化"的资源重新 bind。shadow cell 按资源建索引(每资源一个小 Vec),`bind_use` 只看本资源,消灭 O(U×S) |
| B2 | **提交时不再 replay** | 规划器直接产出最终 `StatePatch`(已经有 `prepare`/`publish` 分离),提交只做 `publish`。图内瞬态/别名资源根本不需要 `RangeStateTable` 校验——它们的全部访问都在计划里;只有 `Import` 资源保留运行期状态机。修订号校验保留在 debug/validation |
| B3 | recorder/batch 用预分组索引 | transition 按 pass 分桶、patch 按 batch 分桶,一次构建随计划缓存;`recorder.rs:34`、`vulkan_queue.rs:358` 变成 O(1) 切片 |
| B4 | 去字符串 | 资源名用 `ustr` interned(ARCHITECTURE §3.5 已设计 `ResourceName(Ustr)` 但未落地);`format!` 键、`BTreeMap<String>` 别名键、`specifications` 名字复制全部换成整数 id |
| B5 | 计划缓存查找 | 64 位哈希先行 + `Arc::ptr_eq` 快速路径,主图/展示图各一个固定槽;避免 128 项逐字段比较 |
| B6 | 动态常量页 | 一页一个 range cell,页封存时一次 publish,去掉逐写入三锁 |
| B7 | 双重 `validate_graph`、`finish_frame` 的 BTreeSet、`collect_named_retired` 的 O(retired×pending×M) | 合并/索引化 |

### 第 C 层:架构级,奔着 120 FPS 的 CPU 预算(2~4 周)[I]
这一层是"不被当前架构困住"的部分。两种路线,建议 **C1 + C2 都做**,C3/C4 视测量。

- **C1 保留图(retained graph)替代每帧重声明。** 原版和当前都每帧从零声明。但世界图的拓扑在相机不动、无编辑时逐帧完全相同——这正是计划缓存 26/28 命中的原因。更进一步:让宿主持有一张常驻 `GraphDefinition`,每帧只提交 `FrameInstance`(相机常量、资源版本号、脏标志、可选 pass 的开关位),声明成本从 O(160 pass + 全部 access) 降到 O(变化量)。rhapsody 已经有 `GraphDefinition → CompiledPlan → FrameInstance` 分层设计(ARCHITECTURE §4.4),缺的是宿主侧"不重建"的接口和 pass 回调的参数化(回调拿 `FrameInstance` 而不是闭包捕获本帧对象)。编辑时(墙/树变化)走显式 invalidate。
- **C2 draw 录制 GPU 驱动化。** 原版光栅 pass 是 HZB cull + `multi_draw_indexed_indirect`。当前逐 draw 构建 bindless 参数块是 Shadows/G-buffer 录制 90 ms(debug)的来源。改为:每个 draw list 一个 SSBO 存全部 draw 的参数(材质索引、变换、纹理索引),shader 按 `gl_DrawID`/instance 索引读取;CPU 每 pass 只做一次 `vkCmdDrawIndexedIndirectCount`。参数 SSBO 只在列表变化时(编辑)重写。这是对齐原版,不是新发明。
- **C3 pass 与屏障合并。** 160 pass/3270 转换太多。Nani cull & bucket×10、Clear buffer×6、Shadows×3 这类同形 pass 合并为单 pass 多 dispatch;屏障粒度从 mip/layer 范围合并到每 pass 一次 `vkCmdPipelineBarrier2`(ARCHITECTURE §3.6 第 3 点的目标就是这个);范围级精确性只在 debug 校验器里保留。
- **C4 上传走独立路径。** setup 命令缓冲(原版做法)或 transfer 队列,在图之前无等待提交;宿主所有 `*_wait` 变体从生产路径删除。
- **C5 GPU 线(产品决策,非架构)。** 拿到 per-pass GPU 时间戳后,如果 UHD 630 上 GPU 本身 > 8.3 ms,120 FPS 只能靠画质分级:软件 BVH GI / SSGI / 体积光 / 4×MSAA 做可配置档位。这需要用户决定验收硬件。

### 顺序与验收门
1. 第 0 步真机计时 → 确定 A 层各项实际占比。
2. A 层全部做完,真机复测。预期进入两位数 FPS;若仍 < 10 FPS 且 `record_submit_ms` 占大头,提前做 C2。
3. B 层以 rhapsody 自己的 `p0_perf_baseline`(160 pass 规模的合成图)做门:框架 CPU/帧 < 2 ms、稳态分配 ≈ 0。
4. C1/C2 以 `render_stages` 的 `declare_ms + record_submit_ms` < 3 ms 为门。
5. 全程保留 `PERFORMANCE-STANDARD.md` 的口径:持续平均、1% low、p95/p99,Release,960×540,完整画质。

## 4. 不建议做的事
- 不要先做 async compute / transfer 队列调度(ARCHITECTURE §4.6):UHD 630 和多数集显是别名队列,收益为零;原版也没用。
- 不要为了帧率删 pass、跳 Logo、降低 MSAA 来"达标";画质分级是 C5 的显式产品决策。
- 不要继续在没有真机 `render_stages` 数据的情况下按代码推断排优先级——本文的排序本身就是这种推断,必须用第 0 步验证。

## 5. 线程架构:原版 vs 当前(2026-10-21 补充)

| 线程 | 原版 | 当前 | 备注 |
|---|---|---|---|
| ECS 调度执行器 | `MultiThreadedExecutor::run` 在 PDB 公开符号中存在(`evidence/render-architecture/public-symbols.json:240565`)[C] | `bevy_ecs` 未启用 `multi_threaded` feature,默认 `SingleThreadedExecutor` [C];`bevy_tasks` 有 `multi_threaded` 但 ECS 没用上 | debug 下 App.update 25 ms 全在单线程 |
| 游戏主循环线程 | `run_game_thread`(0x1405a3440)+ 通用助手 `compute_on_dedicated_thread`(0x14032d170,`sync_channel(1)` + 具名线程 + 分离)[C 存在;主循环是否跑在该线程上 I] | `run_game_thread` 直接在主线程调用(`run_game.rs:195`),帧在 winit `RedrawRequested` 回调里跑(`event_loop.rs:136`)[C] | 当前 OS 事件泵与渲染同线程,一帧 500 ms 内输入/关闭全部阻塞 |
| 并行录制 | `bevy_tasks::ComputeTaskPool` PGO 分块 [C] | 已实现,但 PGO 统计按图交换,主图读到展示图的统计,大概率一直串行 [I] | 见 A6 |
| 管线编译 | 未取证 [U] | `rhapsody-pipeline-{i}` worker,默认 `workers: 1`(`pipeline.rs:38`)[C] | 新变体串行排队 |
| RT BVH 构建 | `bvh_builder_worker` 独立线程状态机 [C] | 已实现(`rt/bvh_builder.rs:93`)[C] | 一致 |
| 存档 | `init_save_thread` [C] | `save_thread` 已实现 [C] | 一致 |
| 两个后台线程 0x1404d8100 / 0x1404d8250 | 存在,角色未定(候选 save / 渲染线程)[U] | 无对应 | 若其中一个是渲染线程,原版就是"模拟/渲染分线程"流水 |
| 资产热更新 | texture_watch / configwatch 线程 [C] | 已实现 [C] | 一致 |
| 音频 | FMOD 自带线程 | FMOD bank 加载线程 [C] | 一致 |

**线程层面的改法(按收益排序)**:
1. **`bevy_ecs` 开 `multi_threaded`**:原版就是多线程执行器;一行 feature,系统并行度由 Schedule 的数据依赖决定。
2. **主循环搬到专用线程,主线程只泵 OS 事件**:对齐原版 `run_game_thread` 的形态;输入/关闭不再被渲染阻塞;winit 的 `request_redraw` 节拍不再和帧率绑死。
3. **模拟/渲染双线程流水**:第 N 帧的声明+录制与第 N+1 帧的 `App.update` 重叠。这比原版更进一步(原版是否如此 [U]),需要 World 快照或 draw list 双缓冲;与 retained graph(C1)配合后声明很薄,收益主要来自把 25 ms 级的 update 藏到渲染后面。
4. **管线 worker 数提到 `cores/2`**:只影响新变体卡顿,不影响稳态。
5. **上传独立线程 / transfer 队列**:A2 的异步 ticket 落地后,staging 拷贝和 flush 可以离开主循环。
6. **draw list 构建与排序并行化**:`build_draw_lists` 每帧对 21 个列表排序,可 `par_sort` 或按脏列表增量排序。

## 6. 其他优化点(补充,未在 §3 中列出)

**提交/呈现侧**
- 呈现模式:`vsync=true` 是 FIFO,60 Hz 屏上帧率封顶 60;验收 120 FPS 要么 MAILBOX,要么 120 Hz 显示器。测量时先 `--vsync off`。
- `present_wait`/`present_id`:FIFO 下 acquire 可能阻塞整个主循环;用 present wait 决定何时开始下一帧。
- Timeline `poll` 每帧多次 `vkGetSemaphoreCounterValue`,合并为帧首一次。

**rhapsody 常数因子(B 层之外的小项)**
- 每个 transition 一次 `Arc` clone + `BTreeMap` 插入进 command owners(`vulkan.rs:1794`):按资源去重后一帧只持一次。
- `vkUpdateDescriptorSets` 每个 binding 一次 + SeqCst 原子(`raw/binding.rs:57-93`):批量成一帧一次。
- 附件视图缓存线性查找(`rg/attachments.rs:393`,256 项):换哈希。
- 管线表全局锁 + 线性键扫描(`pipeline_cache.rs:83`):换读写锁 + 哈希。
- per-frame `HashMap`/`BTreeSet` 用 `RandomState`:换 `FxHash`/`ahash`,或改用帧 arena 上的 `Vec` + 排序。
- `projection.rs:269-379` 每帧对全部 M 资源排序 + Arc clone:随计划缓存。
- `PersistentPipelineCache` 已实现(`pipeline.rs:789`),需确认生产启动路径真的加载它,否则每次启动冷编译。

**游戏侧**
- Shadows×3 各自录制完整 draw list:C2 的 GPU 驱动后 CPU 归零;在此之前至少三级共享一次参数准备。
- egui 每帧 tessellate 全部 shape:UI 无变化时复用上一帧 `ClippedPrimitive`。
- `frame_sampled_table` 的视图快照借用而不是克隆。
- `raster_has_previous_contents` 改为宿主侧每资源区间索引。

**GPU 侧(UHD 630 上决定成败,但要先有 per-pass 时间戳)**
- `world_resolution_scale` 已存在:世界图按 0.75/0.5 渲染,UI 原生分辨率,是最直接的 GPU 杠杆。
- 4×MSAA 最终目标 + resolve:集显上开销不小,做成画质档位。
- 软件 BVH ray-traced GI、SSGI、ReSTIR、体积光:集显上每项都可能超过 8.3 ms 总预算,必须有"关/半分辨率/隔帧更新"档位。
- Nani cull & bucket ×10、Clear ×6:合并 dispatch,减少 pass 边界的 GPU 空泡。
- Vermeer/TAA 历史纹理格式:`R16G16B16A16` 能换 `B10G11R11` 的换掉,省带宽(集显和 CPU 共享内存带宽)。

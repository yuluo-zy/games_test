# 渲染性能重构设计文档(rhapsody + tiny-glade 宿主)

版本 2026-10-21。配套文档:

| 文档 | 内容 |
|---|---|
| [ANALYSIS.md](ANALYSIS.md) | 根因分析、原版对照、线程对照、补充优化点(本设计的证据来源) |
| [IMPLEMENTATION.md](IMPLEMENTATION.md) | 实施顺序、任务清单、每步验收命令 |
| [IMPL-A-host.md](IMPL-A-host.md) | A 层:宿主侧止血,逐项现状 / 改法 / 验证 |
| [IMPL-B-rhapsody.md](IMPL-B-rhapsody.md) | B 层:rhapsody 内部每帧复杂度降到 O(N+A) |
| [IMPL-C-architecture.md](IMPL-C-architecture.md) | C 层:保留图、GPU 驱动绘制、pass 合并 |

所有 file:line 以提交 `f98b40229`(2026-10-21 main)为准。标注 **[C]** 代码确认、**[I]** 推断、**[U]** 未知。

---

## 1. 目的与读者

给要动手改 `crates/rhapsody` 与 `crates/tiny-glade` 渲染链路的工程师一份可直接实施的设计。读者应熟悉 Vulkan 同步模型、render graph 概念与本仓库的 `rg::` API。本文只定目标与决策,代码级改法在 IMPL-* 文档。

## 2. 目标与非目标

### 2.1 目标

| 指标 | 现状 | 目标 | 测量方式 |
|---|---|---|---|
| 持续平均 FPS(Release、960×540、完整画质、真实存档) | ≈ 2 | ≥ 60 最低,≥ 120 合格(`PERFORMANCE-STANDARD.md`) | `TG_FRAME_TIMING=1` 的 `frame n=` 行 + 1% low、p95/p99 |
| 每帧 CPU(update + declare + prewarm + compile + record_submit + resolve + present) | 未测真机;debug/lavapipe ≈ 806 ms | ≤ 3 ms(A+B+C 全部完成后) | `render_stages` 各字段之和 |
| rhapsody 框架开销(160 pass 合成图) | 未测;6 pass 基线 0.30 ms | < 2 ms,稳态堆分配 ≈ 0 | `tests/p0_perf_baseline.rs`、`tests/cpu_frame_allocations.rs`、B0 新建的 `benches/perf.rs`(当前 crate 没有 bench) |
| 帧在飞 | 实际 1 | 2(可配 3) | `present_ticket_wait_ms` 不再等于上一帧 GPU 时长 |
| 帧内 GPU 等待 | 上传每 1 MiB 等一次 fence | 0 次 | `declare_ms` 中无 fence 等待;`transfer` 计数器 |

### 2.2 必须先接受的边界

1. **GPU 上限不是本设计能解决的。** 测试机 Intel UHD 630 约为原版最低配置 GTX 1060 的 1/4~1/5。完整画质含软件 BVH ray-traced GI、SSGI、ReSTIR、体积光、TAA、4×MSAA 最终目标,每帧 160 个 pass。CPU 归零后帧率由 GPU 决定;120 FPS = 8.3 ms GPU 预算,在 UHD 630 上完整画质大概率不可达。**本设计的硬目标是 CPU 线;GPU 线交给第 0 步的 per-pass 时间戳和 §7 的画质档位决策。**
2. **真机没有分阶段数据。** 所有优先级排序来自代码推断。第 0 步(§6)必须先做,之后按实测重新排序。
3. 验收口径不变:不删 pass、不跳 Logo、不降分辨率换帧率;画质档位是显式产品决策。

### 2.3 非目标

- 异步 compute / transfer 队列调度(ARCHITECTURE §4.6):集显是别名队列,收益为零,原版也未用。
- 存档/历史/回放链路(用户 2026-10-20 决定搁置)。
- 移动端画像(ARCHITECTURE §8)。
- 像素级复刻原版:rhapsody 自 2026-10-16 起只以正确性与性能为准。

## 3. 现状架构

### 3.1 每帧流程 [C]

```
winit RedrawRequested(主线程)
 └─ GameLoop::frame                                   game_loop.rs:332
     ├─ App::update(Bevy, SingleThreadedExecutor)      game_loop.rs:244
     ├─ render_loop::render → render_inner             render_frame.rs:298
     │   ├─ build_draw_lists(排序 21 列表 + VAO 快照#1)   production_renderer.rs:1409
     │   ├─ compile_world_graph
     │   │   ├─ bridge.build_frame → session.begin_frame   ← 等 pending>=frames_in_flight
     │   │   │   └─ RenderGraph::new() + build_world W1..W54 (VAO 快照 ×8, W11 同步上传)
     │   │   ├─ ShaderHost::prewarm(扫全部 ticket)
     │   │   └─ compile_built → compile_graph(计划缓存) → instantiate → bind(O(U×S))
     │   ├─ execute_main_graph → execute_prepared(录制 + 逐批 submit, 提交时重放 patch) → end_frame
     │   ├─ acquire_swapchain_image(collect; pending>=fif 则等 PresentTicket; acquire)
     │   ├─ execute_presentation_graph(第二张图: begin_frame ← 再等一次; declare; prewarm; compile; execute)
     │   ├─ present
     │   └─ refresh_memory_budget
     ├─ finish_render
     └─ FpsLimiter::frame
```

### 3.2 线程模型 [C]

| 线程 | 现状 |
|---|---|
| 主线程 | winit 事件泵 + 整个 `GameLoop::frame`(模拟 + 声明 + 录制 + 提交 + 呈现) |
| ECS | `SingleThreadedExecutor`(`bevy_ecs` 未开 `multi_threaded`) |
| `rhapsody-pipeline-0` | 管线编译,默认 1 个 |
| `bvh_builder_worker`、save、texture/config watch、FMOD | 各自独立,与原版一致 |
| ComputeTaskPool | 存在,但并行录制因 PGO 统计按图交换而对主图基本不生效 [I] |

### 3.3 关键数字 [C]

- 主图约 160 次 pass 执行(109 个不同名字),160 条屏障命令、3270 次范围转换、8~9 个 batch。
- debug + lavapipe + validation(480×270):update 24.6 ms、declare 227 ms、compile 11 ms(缓存命中后)、record 195 ms;Shadows×3 录制 92.7 ms、G-buffer 87.8 ms。
- 计划缓存命中率 26/28,未命中来自 temporal 奇偶翻转与截图 pass。

## 4. 原版对照(摘要,证据见 ANALYSIS §1、§5)

| 维度 | 原版 | 当前 | 结论 |
|---|---|---|---|
| 图生命周期 | 每帧重声明、重编译,无缓存 [I] | 每帧重声明 ×2,计划缓存 | 范式相同,常数因子差距大 |
| 帧在飞 | ≈2 [I] | 实际 1(每图计一帧)[C] | 必须修 |
| 屏障 | 录制时按前次访问即时算 [C] | 规划 → 每帧 bind → 提交重放,算三遍 [C] | 必须修 |
| 绘制 | HZB cull + multi-draw-indirect [C] | 逐 draw 录制 + 逐 draw 参数块 [C] | C2 对齐原版 |
| 上传 | setup 命令缓冲,不等待 [C] | 同步等 fence [C] | 必须修 |
| ECS | `MultiThreadedExecutor` [C] | 单线程 [C] | 一行 feature |
| 主循环线程 | `run_game_thread` + `compute_on_dedicated_thread` [C 存在,I 用途] | 主线程 | 对齐原版 |
| 队列 | 单图形队列 [C] | 可配置,默认别名 | 不动 |

## 5. 目标架构

### 5.1 每帧流程(A+B+C 完成后)

```
主线程: winit 事件泵 → channel → 游戏线程
游戏线程:
 ├─ App::update(MultiThreadedExecutor, ComputeTaskPool)
 ├─ FrameParams 组装(相机、时间、draw list 修订号、temporal parity、可选 pass 开关)
 ├─ retained.ensure_plan(&params)        仅拓扑变化时重编译(C1)
 ├─ session.begin_display_frame()        一个显示帧一个槽(A1),等第 n-2 帧
 ├─ uploads.flush()                      异步 ticket,图通过 SyncPoint 依赖(A2),不等待
 ├─ bind(增量: 只对导入资源初态变化的部分)(B1),从计划切片取 transitions/patches(B3)
 ├─ record(并行分块,PGO 按图分槽)(A6);光栅 pass = 每桶一次 DrawIndexedIndirectCount(C2)
 ├─ submit(只 publish,不重放)(B2)         主图与展示图同一事务,交换链图像 gate_first_use
 ├─ present
 └─ session.end_display_frame()
```

### 5.2 线程模型

| 线程 | 职责 |
|---|---|
| 主线程 | winit 事件泵;resize/close 协议;不做任何渲染 |
| 游戏线程(`run_game_thread`) | update + 声明 + 录制 + 提交 + 呈现 |
| ComputeTaskPool(核数-1) | ECS 并行系统;并行录制分块;draw list 并行排序 |
| `rhapsody-pipeline-{0..n/2}` | 管线编译 |
| 上传线程(可选,A2 之后) | staging 拷贝、flush、ticket 完成通知 |
| 现有 BVH / save / watch / FMOD | 不变 |

模拟/渲染双线程流水(第 N 帧录制与第 N+1 帧 update 重叠)作为 C 层之后的可选项,需要 draw list 双缓冲与 World 只读快照,本轮不承诺。

### 5.3 数据所有权

| 数据 | 所有者 | 生命周期 |
|---|---|---|
| `GraphDefinition` / `Arc<CompiledPlan>` | `RetainedGraph`(宿主) | 拓扑变化时重建 |
| `BoundTransition` 切片、`patches_by_batch` | `CompiledPlan` 附属 | 随计划 |
| 导入资源初态键 `ImportStateKey` | session,每帧比较 | 每帧 |
| `FrameParams` | 游戏线程栈上 | 一帧 |
| `DrawParamsSsbo`(每 draw list) | `DrawListGpu`(宿主) | 列表修订号变化时重写 |
| 上传 ticket | `UploadManager` | 至 SyncPoint 完成 |
| 运行期 `RangeStateTable` | 只保留给 `Import` / persistent / temporal 资源 | 资源生命周期 |

## 6. 设计决策(D1–D10)

**D1 帧在飞按显示帧计数,不按图计数。** 选项 (a) 引入 `DisplayFrameToken`,主图与展示图共享一个槽;选项 (b) 把展示图并入主图,交换链图像经 `gate_first_use` 导入。推荐 **先 (a) 后 (b)**:(a) 改动小、一天内可验证;(b) 是 ARCHITECTURE §3.8 的终态,与 C1 一起做。细节见 IMPL-A §A1。

**D2 帧内绝不等待 GPU。** 所有 `*_wait(u64::MAX)` 从生产路径删除;上传走 ticket,图通过导入的 `SyncPoint` 等待;需要 CPU 读回的(地形高度 W5)走双缓冲延迟一帧读。违反此决策的代码在 review 时直接打回。

**D3 保留图(C1)而不是继续优化每帧重声明。** 理由:计划缓存已 26/28 命中,说明拓扑逐帧不变;声明的 227 ms(debug)全部是可避免的重复工作。原版每帧重建是因为它的常数极小;我们选择让声明成本变成 O(变化量),这是对当前架构的明确偏离,收益最大、风险在回调参数化。保留 `TG_RETAINED_GRAPH=0` 回退到每帧重建路径至少两个里程碑。

**D4 GPU 驱动绘制(C2)对齐原版。** 原版光栅 pass 是 HZB cull + multi-draw-indirect;当前逐 draw 录制是回退。先做 Shadows 与 G-buffer 两类 pass(录制耗时前三),参数 SSBO 按列表修订号重写。

**D5 图内瞬态资源不进运行期状态机。** 规划器对 `Create`/别名/瞬态资源拥有全部访问信息,运行期 `RangeStateTable` 校验只保留给 `Import`、persistent、temporal。提交阶段只 `publish`,不重放。修订号校验在 debug / `validation` feature 下保留(B2)。

**D6 去字符串。** 资源名、temporal 键、别名键、计划缓存比较全部改成 interned `Ustr` / 整数 id(ARCHITECTURE §3.5 已定,未落地)。

**D7 ECS 多线程 + 主循环专用线程,对齐原版。** `bevy_ecs` 开 `multi_threaded`;`GameLoop::frame` 搬到专用线程,主线程只泵事件。`Rc` 持有的 bridge session 需要整体搬到游戏线程创建(见 IMPL-A §A6)。

**D8 每项优化带开关与 A/B。** 环境变量或 `DeviceConfig` 字段;同负载比较 p50/p95/p99、分配数、等待时长;未证明收益的保持可选(ARCHITECTURE §4.9)。

**D9 不做 async compute,不做多队列调度。** 见 §2.3。

**D10 GPU 画质档位是产品决策,不在本设计内实施。** 本设计只交付 per-pass GPU 时间戳数据与 `world_resolution_scale` 等已有杠杆的接线说明。

## 7. 测量协议(第 0 步)

### 7.1 真机分阶段计时

```powershell
# Windows, Release, 完整存档, 960x540 窗口
$env:TG_FRAME_TIMING = "1"
.\tiny-glade.exe --windowed --window-size 960x540 2>&1 | Tee-Object tmp\frame-timing-baseline.log
```

采集 ≥ 300 个 gameplay 帧。日志中两类行:
- `frame n=... update_ms= render_ms= finish_ms= limiter_ms= total_ms=`(`game_loop.rs:427-433`)
- `render_stages gameplay_n= draw_lists_ms= declare_ms= prewarm_ms= compile_ms= record_submit_ms= collect_ms= present_ticket_wait_ms= acquire_ms= resolve_inclusive_ms= present_ms= queue_trim_actual_ms= ...`(`production_renderer.rs:1251`,前 3 帧 + 每 30 帧)

把 `render_stages` 改为每帧输出(或采样间隔改为 1)以便算 p95/p99;这是一行改动,见 IMPL-A §A0。

### 7.2 GPU per-pass 时间戳

rhapsody 已有 `VulkanGraphSession::configure_gpu_timing(Option<GpuTimingConfig>)`(`rg/vulkan.rs:754`)与 `rg/profiling.rs` 的 `GpuTimingRing`,生产路径未接。第 0 步接线:`TG_GPU_TIMING=1` 时在 `production_renderer.rs` 创建 session 后调用 `configure_gpu_timing`,每 30 帧把各 pass 的 GPU span 汇总到日志(按名字聚合,输出 top 20 与总和),并把顶层 span 回填 `last_gpu_frame_ns`(当前返回 `None`,`production_renderer.rs:1739`)。

### 7.3 判读表

| 观察 | 结论 | 优先 |
|---|---|---|
| `present_ticket_wait_ms` ≈ 上一帧 GPU 总 span | 一帧在飞串行 | A1 |
| `declare_ms` 大且 transfer 等待计数 > 0 | 同步上传 | A2 |
| `declare_ms` 大且 VAO refresh 计数 ≥ 8 | 快照重建 | A4 |
| `prewarm_ms` > 0.5 ms 且 pending ticket = 0 | 全表扫描 | A3 |
| `compile_ms` > 1 ms | bind O(U×S) | B1 |
| `record_submit_ms` 大,Shadows/G-buffer 占比高 | 逐 draw 参数 | C2(提前) |
| `update_ms` > 5 ms | ECS 单线程 | A5 |
| GPU 总 span > 8.3 ms | GPU 上限 | §2.2 决策 |

### 7.4 记录模板

每次测量记录:硬件、驱动版本、exe SHA256、存档、窗口、vsync、构建 profile、帧区间、平均 FPS、1% low、p50/p95/p99 帧时间、各 `render_stages` 字段 p50/p95、GPU 总 span p50/p95、top 10 GPU pass。写入 `reports/render-perf-architecture-2026-10-21/measurements/<date>-<hw>.md`。

## 8. 验收门

| 阶段 | 门 | 工具 |
|---|---|---|
| 第 0 步 | 有一份 §7.4 模板的真机记录 | 日志 |
| A 层完成 | `present_ticket_wait_ms` p50 < 上一帧 GPU span 的 20%;帧内 fence 等待计数 0;VAO refresh/帧 = 1;prewarm p50 < 0.1 ms;`update_ms` 下降;真机 FPS ≥ 10(估计,需实测) | render_stages |
| B 层完成 | rhapsody 160 pass / 3000 use 合成图(B0 新建 `benches/perf.rs`,CPU 后端):`bind_ns + record_ns + submit_ns` < 2 ms,稳态分配 ≤ 10/帧,`shadow_cells_visited` 计数器不回退;`check_bound_synchronization` 与 `check_state_publication` 在 validation 下全绿;`tests/p0_stability.rs` 1000 帧计数平稳 | p0_perf_baseline / benches/perf.rs |
| C1 完成 | `declare_ms` p50 < 0.5 ms;像素等价测试(保留图 vs 重建图)通过;`TG_RETAINED_GRAPH=0` 仍可跑 | 新增 `retained_graph_equivalence` 测试 |
| C2 完成 | Shadows + G-buffer `record` 合计 < 0.5 ms;像素等价;`TG_GPU_DRIVEN=0` 仍可跑 | 新增 `gpu_driven_equivalence` 测试 |
| 总验收 | `PERFORMANCE-STANDARD.md`:Release、960×540、完整画质、持续平均 + 1% low + p95/p99 | 真机 |

所有 GPU 测试本机只能用 lavapipe 验证功能(`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`),耗时数字只认真机。

## 9. 阶段计划与依赖

```
第0步 测量 ──┬─> A1 帧在飞 ──┐
             ├─> A2 异步上传 ─┤
             ├─> A3 prewarm ──┼─> A 层验收 ──> B4 去字符串 ──> B0 基准+检查器接线 ──> B3 索引化 ──> B1 bind 增量 ──> B2 提交不重放 ──> B5 ──> B 层验收
             ├─> A4 VAO 快照 ─┤                 B6/B7 独立小项(任意时机)
             ├─> A5 ECS 多线程┤
             ├─> A6 PGO+主循环线程┘
             └─> A7 小项
C1 保留图:依赖 A1(a)、B4(Ustr 键)、B5(按图槽);与 B1-B3 并行开发,合入在 B 层验收后
C2 GPU 驱动:依赖 A4(VAO 快照稳定)、C1 的 FrameParams(可先用闭包版);若第 0 步显示 record 占大头则提前到 A 层后
C3 pass 合并:C1 之后,按 per-pass GPU 时间戳决定
D1 选项 (b) 展示图并入主图:与 C1 一起
```

人力估计(单人,熟悉代码):第 0 步 0.5 天;A 层 3~5 天;B 层 8~12 天;C1 8~10 天;C2 6~10 天;C3 2~3 天。并行两人可压到 4 周。

## 10. 风险

| 风险 | 缓解 |
|---|---|
| 排序基于推断,真机数据可能推翻 | 第 0 步先做;每项带开关,A/B 验证 |
| 异步上传后图读到未完成数据 | 上传返回 `SyncPoint`,导入时作为依赖;lavapipe 下用 `RHAPSODY_SUBMIT_TRACE` 校验等待链;新增 `upload_visibility` 测试 |
| 图内资源跳过运行期状态机后同步错误 | 独立检查器 `plan_check` 在 debug/validation 下继续对全部资源校验;B2 分两步:先跳过校验但仍记录,比较一致后再删 |
| 保留图回调参数化遗漏某个每帧变化的输入 | 像素等价测试跑 ≥ 60 帧含相机移动、编辑、季节切换;`FrameParams` 字段清单由 C1 文档列出,审查逐项打勾 |
| 主循环搬线程后 `Rc`/`!Send` 对象 | bridge 在游戏线程创建;主线程只持 `Arc<Window>` 与 channel |
| UHD 630 GPU 上限 | §2.2;第 0 步 GPU 数据出来后由用户决定验收硬件或画质档位 |
| 磁盘:并行开发多份 target | 遵守 `agent-build-dirs` 规则:独立 `CARGO_TARGET_DIR`、`CARGO_PROFILE_DEV_DEBUG=0`、完成即删 |

## 11. 附录:涉及文件索引

| 层 | 文件 |
|---|---|
| 宿主帧驱动 | `crates/tiny-glade/src/{game_loop,event_loop,run_game,render_loop,production_renderer,render_bridge,render_host,raster_host,scene_raster,world_graph,world_terrain,world_lighting,world_surface,world_postprocess}.rs` |
| country-core 渲染 | `crates/country-core/src/systems/render_loop/render_frame.rs`、`compute_passes/*`、`render/shader.rs`、`render/ssbo.rs` |
| rhapsody 会话/执行 | `crates/rhapsody/src/rg/{vulkan,vulkan_queue,vulkan_presentation,frame_loop,executor,graph,plan,cache,sync_plan,sync_plan/bind,recorder,transient_resources,temporal,profiling,parallel,pass_bindings/projection,attachments}.rs` |
| rhapsody 资源/同步 | `crates/rhapsody/src/resource/{range_state,tracked}.rs`、`raw/{command_buffer,dynamic_constants,buffer,binding}.rs`、`raw/shader/pipeline_cache.rs`、`transfer.rs`、`pipeline.rs`、`device/{init,lifetime}.rs`、`sync/timeline.rs` |
| 构建配置 | 工作区 `Cargo.toml`(bevy features、profile.release) |

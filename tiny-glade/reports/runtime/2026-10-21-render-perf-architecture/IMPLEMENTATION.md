# 渲染性能重构实施文档

版本 2026-10-21。设计与决策见 [DESIGN.md](DESIGN.md);逐项代码级改法见 [IMPL-A-host.md](IMPL-A-host.md)、[IMPL-B-rhapsody.md](IMPL-B-rhapsody.md)、[IMPL-C-architecture.md](IMPL-C-architecture.md)。本文只管顺序、任务拆分、验证命令与完成定义。

## 0. 通用规则

- 基线提交 `f98b40229`。每个任务独立分支,PR 标题 `perf(<层><编号>): <摘要>`,正文附第 0 步模板的前后数字。
- 每项改动必须有开关(环境变量或 `DeviceConfig` 字段),默认值在 PR 中写明;未证明收益的默认关。
- 构建与测试(本机 Linux):
  ```bash
  export PATH=$HOME/.cargo/bin:$PATH
  export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
  export CARGO_TARGET_DIR=/tmp/tg-target            # 并行 agent 各用自己的目录,完成即删
  export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0
  cd tiny-glade
  cargo test -p rhapsody --locked --offline --test <name>        # 只编译需要的测试二进制
  cargo test -p tiny-glade --locked --offline --test <name>
  cargo clippy --config <ustc.toml> -p rhapsody --no-deps --all-targets -- -D warnings
  cargo fmt -p rhapsody --check
  ```
  lavapipe 只能证明功能与计数器;耗时只认真机。
- 性能门(注意:`crates/rhapsody` 目前**没有** `benches/`,`cargo bench --bench perf` 是旧 rhapsody-rebuild 的命令;B0 先新建):
  ```bash
  cargo test --release -p rhapsody --test p0_perf_baseline -- --ignored --nocapture   # representative_graph_frame_cost_baseline / render_perf_throughput
  cargo test -p rhapsody --test cpu_frame_allocations                                  # 预热后 allocations == 0
  cargo bench -p rhapsody --bench perf                                                  # B0 之后:160 pass / 3000 use 合成图,计数器 bind_ns/record_ns/submit_ns/allocations/shadow_cells_visited
  ```
- 真机(Windows,Release):
  ```powershell
  $env:TG_FRAME_TIMING="1"; $env:TG_GPU_TIMING="1"
  .\tiny-glade.exe --windowed --window-size 960x540 2>&1 | Tee-Object tmp\<task>-after.log
  ```
  记录写入 `measurements/<date>-<hw>-<task>.md`(模板 DESIGN §7.4)。
- 不改:存档/历史链路、country-core 的逆向规则、pass 的算法与画质。

## 1. 第 0 步:测量(0.5 天)

| # | 任务 | 文件 | 完成定义 |
|---|---|---|---|
| 0.1 | `render_stages` 每帧输出(或采样间隔可配 `TG_FRAME_TIMING_EVERY=1`) | `production_renderer.rs:1240-1260` | 日志每帧一行 |
| 0.2 | 接 `configure_gpu_timing`,`TG_GPU_TIMING=1` 时启用;每 30 帧输出 top 20 pass GPU span 与总和;顶层 span 回填 `last_gpu_frame_ns` | `production_renderer.rs`(session 创建处、`:1739`)、`rhapsody/src/rg/vulkan.rs:754`、`rg/profiling.rs` | 日志含 `gpu_spans` 行;`queue_trim` 开始有实际值 |
| 0.3 | transfer 等待计数器:`upload_buffer_ranges_wait` 调用次数与总等待时长进 `render_stages` | `render_host.rs:173-221`、`rhapsody/src/transfer.rs` | 字段 `upload_wait_n= upload_wait_ms=` |
| 0.4 | VAO refresh 计数接入 `render_stages`(`raster_host.rs:406` 已有计数) | `production_renderer.rs` | 字段 `vao_refresh_n=` |
| 0.5 | prewarm 计数:ticket 总数、pending 数、实际等待时长 | `render_host.rs:1595` | 字段 `prewarm_tickets= prewarm_pending=` |
| 0.6 | 真机采集 300 帧,填 `measurements/` 模板;按 DESIGN §7.3 判读,更新本文 §2 的顺序 | — | 文件存在 |

## 2. A 层:宿主止血(3~5 天)

> 实施结果与偏差(A5 改为可选特性、A6.2 默认关闭、审阅修复、本机构建环境)见 [PROGRESS.md](PROGRESS.md);下表保留原计划口径。

顺序按 DESIGN §7.3 判读结果调整;默认顺序 A1 → A2 → A3 → A4 → A5 → A6.1 → A7 → A6.2(线程改动最大,放最后)。每项细节在 IMPL-A 对应节;实施前先过 IMPL-A 文末的 20 条【未核实】grep 清单。

| # | 任务 | IMPL-A 节 | 开关 | 完成定义 |
|---|---|---|---|---|
| A1 | 帧在飞按显示帧计数(选项 a):`PendingSessionFrame` 加 `DisplayFrameId`,`FrameThrottle::in_flight_groups()` 按显示帧计组;新 API `begin_display_frame / begin_graph / end_display_frame`,旧 `begin_frame` 保持兼容;`frames_in_flight` 暂不提到 3 | §A1 | 旧 API 即回退 | `present_ticket_wait_ms` p50 < 上一帧 GPU span 20%;`declare_ms` 不再含节流等待;`tests/frame_command_pools.rs`、`tests/p0_stability.rs`、`tests/raw_fixes_swapchain.rs` 绿 |
| A2 | `update_buffer_bytes` 默认 `enqueue_upload`,在 `build_frame` 闭包返回后、图提交前 `flush_uploads` 到图形队列(bind 读 `last_use` 表自动有序);staging `slots 1→3, max_bytes 4→16 MiB`;启动保留 `update_buffer_bytes_wait`;W11 `GroundedUploadCache` 逐元素比较跳过未变更 | §A2 | `TG_SYNC_UPLOAD=1` 回退 | `upload_wait_n=0`;新增 `upload_visibility` 测试(上传后同帧图读到新数据) |
| A3 | `ShaderHost.pending_tickets` 在三处注册点登记,prewarm `mem::take` 后只等待决;Ready 丢弃,Timeout/Failed 放回并按原格式报错 | §A3 | 无(行为等价) | `prewarm_tickets` 稳态 0;Timeout/Failed 语义测试绿 |
| A4 | `VaoTableToken { owner, library_revision, table_revision }`;`AssetVAOLibrary.revision`(add/rebuild/remove 自增)、`RasterShared.vao_table_revision`;`SceneRasterHost::with_snapshot(.., &mut token)` 从 W23 贯穿到 W53,W5 子集刷新使令牌失效 | §A4 | `TG_VAO_SNAPSHOT_REUSE=0` 回退 | `vao_refresh_n` 9→2(第二步合并写入到 1);W5 读回 + 下一帧 rebuild_vao + App 切换 三个场景测试 |
| A5 | `Cargo.toml:117` `bevy_ecs` 加 `multi_threaded`;`run_game_thread` 前显式 `TaskPoolBuilder` 定线程数;诊断期 `ambiguity_detection: Warn` | §A5 | feature | `update_ms` 下降;全部 country-core 测试绿;无 ambiguity 报错 |
| A6.1 | PGO `PgoTable` 按 `PgoKey`(pass 名序列 xxh3,`compile_graph` 时算入 `PreparedGraph`)LRU 8 槽;`max_chunks` 按核数 | §A6.1 | `ParallelRecording.pgo` | 主图 `chunks > 1`(`ExecutionStats`) |
| A6.2 | 主循环专用线程:`App`/`GameLoop`/`CoreRenderBridge` 均 !Send,全部在游戏线程内构造;主线程 `mpsc` 送 `ToGame`,`EventLoopProxy<ToMain>` 回传窗口操作;`frame(target)` 去参数 | §A6.2 | `TG_GAME_THREAD=0` 回退 | 长帧内关闭窗口立即响应;resize 协议测试;macOS 重验 |
| A7 | `refresh_memory_budget_if_due`(250 ms 或预算压力;现状每帧 2~3 次);PGO Instant/SipHash 随 A6.1 加开关 | §A7 | — | — |

A 层验收:DESIGN §8 第二行;真机复测并记录。

## 3. B 层:rhapsody 内部(8~12 天)

顺序(IMPL-B §0):B4 → B0 基准 → B3 → B1 → B2 → B5 → B6 → B7。实施前先过 IMPL-B 的 25 条【未核实】。

| # | 任务 | IMPL-B 节 | 依赖 | 完成定义 |
|---|---|---|---|---|
| B4 | 去字符串:rhapsody 加 `ustr.workspace = true`(工作区已有 `ustr = "=1.0.0"`);`specifications`/`LentAliases.resources` 从 `BTreeMap<String,_>` 改按逻辑下标的 `Vec<Option<_>>`;`NamedKey{name,kind,parity}` 替换 `format!("slot/..")`/`("pair/..:0/1")`;`ImportName{base,ordinal}` 替换 `format!("{name}-{idx}")`;别名键加 u64 哈希预筛 | §B4 | — | 稳态分配下降;`tests/graph_cache_eviction.rs`、`tests/temporal_reset_retirement.rs`、`tests/transient_aliasing.rs` 绿 |
| B0 | 新建 `benches/perf.rs`:CPU 后端 160 pass / 3000 use 合成图;`ExecutionStats` 新增 `bind_ns`、`shadow_cells_visited`;把 `check_bound_synchronization`(目前只在 `vulkan_presentation.rs:857` 与测试中调用)在 `validation_enabled()` 下接进 `bind_queue_resources`,`check_state_publication` 接进提交 | §0 | — | bench 可跑并输出基线数字 |
| B3 | `transitions_by_pass: Vec<Range<u32>>`、`patches_by_batch` 在 bind 时构建一次,recorder/batch 切片访问 | §B3 | B0 | recorder 无全表扫描;`check_bound_synchronization` 绿 |
| B1 | 每资源 `SmallVec<[u32;4]>` shadow cell 索引 + `alias_flags` + 每 batch wait 索引;`BoundTemplate` 按 `GraphKind` 槽缓存,键 = `Arc::ptr_eq(plan)` + `ImportStateKey`(每 cell 布局/owner/scopes/队列掩码,**不含** revision 与 SyncPoint 值,后者每帧经 `PatchBase{initial_slot, delta}` / `ExternalWait` 重基);owner 为 `Released` 或存在 `OwnershipPrelude` 批次时不缓存 | §B1 | B4、B3 | `shadow_cells_visited` 从 O(U·S) 降到 O(U);`tests/plan_checker.rs`、`tests/native_ranges.rs`、`tests/multi_queue_graph.rs` 绿 |
| B2 | bind 产出每 (resource, batch) 的 `FinalCell`;`attach_range_patches` 每 batch 一个 `BatchStateHandoff`(每资源一个 `Arc<SyncCell>`);submit 快路径 = 修订号基线检查 + `stage_final`/`publish_final`,无 `apply_preview`、无 coalesce;opaque/上传/非图命令保留重放;`Create` 资源不再安装 `RangeStateTable`(初态由 `SyncCell::snapshot()` 合成,池命中图像每帧从 UNDEFINED 开始,`barrier_transitions` 允许小变化,记录前后基线);`GpuSubmissionCapture` → `SubmissionCapture::{Off,Counts,Full}`;`submissions` 改 `VecDeque`;三处读 `c.batches` 的测试改 `batch_count`(`p0_perf_baseline.rs:545`、`native_ranges.rs:653/781`) | §B2 | B1 | submit 中无 `apply`;`check_state_publication` 绿;`tests/command_retention.rs`、`tests/host_buffer_ranges.rs`、`tests/resource_contracts.rs` 绿 |
| B5 | `PlanKey` 加 FxHash `declaration_fingerprint` + 桶表;`last_hit: [Option<_>; 2]` 按 `GraphKind`(Main/Presentation,识别方式【未核实】,回退为宿主提示);保留 `graph.validate()` + `validate_matching_initialization` | §B5 | B4 | compile p50 < 5 µs(bench) |
| B6 | 页级 host 租约在 `allocate` 打开一次,`write_bytes_leased` 零锁,封存时一次 `publish_host`;页 opaque use 改 `OpaqueUseRange::Whole`,表恒一 cell | §B6 | — | `tests/dynamic_constants_ring.rs` 绿;K 次写入 O(K) |
| B7 | 删 `vulkan.rs:2932` 的重复 `validate_graph`;`collect_named_retired` 在飞 token 引用计数;`finish_frame` 用 FrameStorage 内按物理资源的 bool 表;PGO 改按 pass 下标 Vec、仅 Chunked 策略下计时、按 `GraphKind` 分槽;`refresh_memory_budget` 节流;timeline `poll_epoch`;descriptor 写批量;附件视图 HashMap;管线表 RwLock + 哈希索引;projection token 序列快路径 | §B7 | 任意 | 各自单测 |

B 层验收:`benches/perf.rs` 合成图 `bind_ns + record_ns + submit_ns` < 2 ms、分配 ≤ 10/帧、`shadow_cells_visited` 不回退;validation 下两个检查器全绿;`p0_stability` 1000 帧。

## 4. C 层:架构(2~4 周)

前置事实(IMPL-C §C1.1):今天计划缓存"命中"仍先重建整张 `GraphDeclaration` 再做 O(资源+pass+access) 的 `matches_declaration`,所以命中不省声明;C1 的实例化收益依赖 B1,否则只省声明不省 bind。C2 的上传收益依赖 A2;桶压缩比取决于 VAO 共享程度,动工前先用列表日志统计;`shaderDrawParameters` 在目标集显上的支持面需先确认(`device/caps.rs` 目前没有该 feature)。

| # | 任务 | IMPL-C 节 | 依赖 | 完成定义 |
|---|---|---|---|---|
| C1-M0 | 基建:`GraphShapeKey`、`RetainedGraph`、`PassRecorder` trait、`FrameParams/FrameControls` 字段清单;rhapsody `begin_frame_with(&Arc<CompiledPlan>, FrameInputs)`、`execute_prepared_with`、`compile_frame_retained`、`resolve_temporal_pair`、`CompiledPlan.shape_hash`;`LegacyRecorder` 过渡 | §C1.2、§C1.5 | A1、B4、B5 | 单测:同一计划连续 100 帧无声明、无 `validate_graph`、无 `matches_declaration` |
| C1-M1 | Shadow 组迁移到 `PassRecorder` | §C1.6 | M0 | `retained_graph_matches_rebuilt_path` 30 帧逐字节比较阈值 0 |
| C1-M2 | G-buffer 组 | §C1.6 | M1 | 同上 |
| C1-M3 | Lighting 组 | §C1.6 | M2 | 同上 |
| C1-M4 | Post 组(DoF/TAA-TGSR 开关走 `FrameControls` 或失效) | §C1.6 | M3 | 同上 + 开关切换测试 |
| C1-M5 | 前段(上传/程序化/Nani)+ 失效规则 20 行事件表全部接线;截图 pass 与空列表跳过改为"固定声明、录空" | §C1.3、§C1.6 | M4 | 场景切换测试;`declare_ms` p50 < 0.5 ms |
| C1-M6 | 展示图并入主图(D1 选项 b,`import_swapchain_image + gate_first_use`) | IMPL-A §A1、§C1.5 | M5 | 每显示帧一个 session 帧 |
| C2-GD0 | ABI 等价断言:128 B `DrawParams` 布局、16 B 头、命令区、每桶计数区;`DrawListRevision` 哈希 + 逐条目增量上传 | §C2.2(a) | A2、A4 | 单测:列表不变不重写;上传量计数 |
| C2-GD1 | shader 侧 `[[vk::builtin("DrawIndex")]]` + `Feature::ShaderDrawParameters`,回退 `firstInstance` 编码;每 pass 常量移到 pass 参数块 push 区 | §C2.2(b) | shader-compiler | shader-compiler 测试;lavapipe/ANV 支持确认 |
| C2-GD2 | Shadows 改桶 =(shader,variant,state,stencil_ref,VAO),每桶 `bind_pipeline + 1× DrawIndexedIndirectCount`;三级联共用 SSBO,cascade flag 走 push | §C2.2(c)(e) | GD0/GD1 | 像素等价;Shadows record < 0.2 ms |
| C2-GD3 | G-buffer 组;材质回调拆 `bind_static` 纯函数(最大工作量) | §C2.3 | GD2 | 像素等价;record 合计 < 0.5 ms |
| C3.1 | Nani cull & bucket 每轮 7→3(Main+RoofTile+WoodenPlank+Owned 合 1;Flower/Bush 合 args 1 + cull 1) | §C3.1 | C1、GPU 时间戳 | 屏障计数下降,像素等价 |
| C3.2 | Clear buffer 6→2(瞬态 clear 合到帧首;第二轮 Nani 可见性 clear 不能提前) | §C3.2 | C1 | 同上 |
| C3.3 | Shadows ×3 合成单 pass 三个 rendering section,深度范围 `[0,3)` | §C3.3 | C2-GD2 | 同上;`d_main_tests` 断言更新 |

## 5. 完成定义(全局)

1. `measurements/` 下有第 0 步、A 层后、B 层后、C 层后四份真机记录。
2. `PERFORMANCE-STANDARD.md` 口径的持续平均、1% low、p95/p99 全部记录;达不到 120 FPS 时,报告必须给出 GPU 总 span 与 top 10 pass,证明剩余瓶颈在 GPU。
3. 每个开关的默认值与 A/B 结果写入 `crates/rhapsody/REFACTOR_PROGRESS.md` 或本目录。
4. `python3 tools/verify_inventory.py` 对 rhapsody 的规则按 P7 约定调整后通过。

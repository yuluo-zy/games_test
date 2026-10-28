---
doc: rhapsody-file-audit
schema: findings-v1
date: 2026-10-28
scope: crates/rhapsody/src
files_total: 389
loc_total: 84278
passes:
  - pass-3: 2026-10-28 对仍未关闭的条目做 symbol 级 rg 核对与关键位置读取；已修复、不成立、已删除的条目全部移除
counts:
  findings: 36
  critical: 0
  major: 7
  minor: 29
  refuted: 0
  info: 0
  suspected: 11
  confirmed: 25
related_tasks: IMPROVEMENT_TASKS.md
---

# rhapsody 发现台账（机器可读版）

## 0. 给 AI 读者的说明

- 本文是**当前未关闭发现的台账**，不是审查记录。只保留仍在源码中成立（或尚未核实）的条目；已修复、误报和已删除代码的条目不再保留。可执行动作、验收与顺序在 [IMPROVEMENT_TASKS.md](IMPROVEMENT_TASKS.md)，任务侧用 `关联发现` 字段回指本文 ID。
- 每条发现一行，字段固定：`ID | 严重度 | 状态 | 类别 | 位置 | 发现`。
- 位置以**当前源码的符号与路径**为准，行号仅供参考。源码已按目录重组（`accounting/`、`bindless/`、`collections/`、`command/`、`device/`、`native/`、`present/`、`resource/`、`rg/`、`tracking/`、`transfer/`），旧的 `src/raw/*` 路径已不存在。
- 严重度：`critical` > `major` > `minor`。当前无 `critical` 条目。
  - major：明确行为错误、校验缺口或每帧热路径上的明显超线性开销
  - minor：不影响正确性的性能、测试缺口、文档偏差或错误分类问题
- 状态：`confirmed`（机制在当前源码中可见）| `suspected`（机制线索成立，但触发条件、调用频率或文档措辞**未核实**，需运行验证或进一步阅读）。
- 类别：`correctness` `memory-safety` `concurrency` `resource` `perf` `validation` `api` `error-handling` `observability` `test` `doc` `dead-code`。
- §1 是**唯一权威索引**，每行一个 ID；ID 不再复用。

## 1. 发现索引

| ID | 严重度 | 状态 | 类别 | 位置 | 发现 |
| --- | --- | --- | --- | --- | --- |
| F-092 | major | confirmed | perf | `src/rg/sync_plan/bind/access.rs`（`wait`） | `wait()` 对已有 wait 列表用线性 `find` 去重，跨队列等待多时每帧 O(W²)，位于每帧绑定热路径。 |
| F-093 | major | confirmed | perf | `src/rg/sync_plan/bind/full.rs:134` 附近 | 依赖计算对每个 target usage 调用 `uses(source).any(...)` 重扫 source 全部 usage，`uses()` 每次重新展开，O(U²)。 |
| F-097 | major | confirmed | perf | `src/rg/queue_schedule/mod.rs`（`validate_resource_queue_contracts`） | 每个资源扫描全部 retained pass/access，跨族重叠访问逐对执行 `precedes` BFS；最坏超线性，是计划编译路径上最明显的超线性检查。 |
| F-098 | major | suspected | perf | `src/rg/queue_schedule/mod.rs`（export 批次条件，约 :105-118） | 最终导出的跨队列等待由"任何导出资源的 pass"触发（含只读消费者），可能比必要更保守。**未核实**批次条件的读写语义。 |
| F-104 | major | suspected | perf | `src/rg/bindless_binding.rs`（`build_coverage`） | 历史记录指出 `build_coverage` 对每个 access 线性查找 semantics，首次构建 O(accesses×semantics)。**未核实**当前查找形式。 |
| F-110 | major | suspected | perf | `src/pipeline/request.rs`（`matches_key`）、`src/pipeline/compiler.rs`（`request_inner`） | pipeline 缓存命中判定为逐条语义比较，每个缓存条目都要做字节级比较。**未核实**整段 SPIR-V 字节比较的确切分支（compute 分支比较 shader handle 与 entry 字节）。 |
| F-148 | major | confirmed | perf | `src/pipeline/shader/library.rs`（`register` 与 :142 附近） | 每次 `register` 调用 `layouts.clear()` 与 `cache.invalidate()`，注册或热替换单个 shader 会丢弃全部已建布局与缓存管线。 |
| F-001 | minor | confirmed | perf | `src/accounting/stats/state.rs`（`materialize_peak`） | 峰值快照在全局状态锁内克隆全部 `records`（含 `String`），每次新峰值后的首个释放都做一次 O(n) 分配与复制。 |
| F-032 | minor | suspected | perf | `src/device/memory/mod.rs`（`evict_idle_caches`，压力回收路径） | 淘汰命中后的全量回收不经 `ReclamationConfig` 的配额，可能一次销毁大量对象。**未核实**当前调用链是否仍绕过配额。 |
| F-036 | minor | suspected | doc | `src/device/queue_config/mod.rs`（模块文档与 `QueuePreference::Prefer`） | 模块文档称传输类要求 1×1×1 粒度，但 `Prefer` 无候选时直接别名到 graphics 且不复查粒度。**未核实**文档措辞与实现的确切偏差。 |
| F-050 | minor | confirmed | perf | `src/present/swapchain/fullscreen/mod.rs`（约 :171、:203） | 每次配置查询都新建 `get_surface_capabilities2` 与 `full_screen_exclusive` loader，同一 surface 查询两轮。 |
| F-062 | minor | suspected | test | `src/native/physical_device/mod.rs`（`DriverVendor`、`DeviceDriverVersion`） | 驱动版本与厂商解码的内联测试缺口。**未核实**：`tests/raw_init_devices.rs` 等集成测试可能已覆盖部分。 |
| F-064 | minor | confirmed | perf | `src/transfer/setup.rs`（`update_buffer_bytes`） | 先把 ranges 收拢进 `gathered: Vec` 再写 staging，多一次整段拷贝。 |
| F-072 | minor | confirmed | perf | `src/command/crash/mod.rs`（约 :168、:301） | 两处重复的 `wait_idle` + 读回 + 解析路径，其中一处多一次 `wait_idle`。 |
| F-074 | minor | confirmed | perf | `src/device/logical.rs`（`debug_utils()`，约 :262）、`src/resource/device_api.rs`（`set_debug_name`） | `debug_utils()` 每次都新建 `DebugUtils`（重新取函数指针）。**调用频率未核实**。 |
| F-076 | minor | suspected | perf | `src/device/semaphore.rs`（`completed`，约 :71） | 队列语义量的 `completed()` 经 `Timelines::poll` 逐队列查询，逐个语义量轮询时驱动查询次数增长。**未核实**具体循环形式。 |
| F-084 | minor | confirmed | perf | `src/rg/graph/instantiate.rs`（约 :204） | `pool_hits.contains(...)` 在失败回收路径上逐资源调用，O(N²)（低频）。 |
| F-086 | minor | confirmed | perf | `src/rg/plan/compile.rs`（约 :131） | gated resource 的首次使用者用 `retained_passes.iter().position(...)` 内层再扫 accesses，编译期 O(G×P×A)。 |
| F-091 | minor | suspected | perf | `src/rg/sync_plan/mod.rs`（`transitions_for`、`patches_for`、`waits_for`） | `PhaseIndex` 失效时三者退化为每次线性过滤全表。**未核实**回退分支的触发条件。 |
| F-107 | minor | suspected | perf | `src/bindless/parameters/mod.rs`（`with_push_credit`，约 :447） | 逐 draw 更新路径中每次重建 `SmallVec` 与 lease 克隆。**未核实**是否仍在逐 draw 调用。 |
| F-116 | minor | confirmed | perf | `src/transfer/queue/mod.rs`（`next`，约 :702、:735） | 对每个候选扫描 `jobs[..i]` 查找同 target，O(n²)（n 受 `max_jobs` 限制）。 |
| F-117 | minor | confirmed | perf | `src/transfer/queue/mod.rs`（`task`，约 :395） | 每次入队线性扫描全部 ticket 槽。 |
| F-122 | minor | confirmed | perf | `src/tracking/range_state/storage.rs`（约 :23，`try_reserve_exact`） | 每条新分裂边界都整表重分配并复制，新旧两份同时计入记账。 |
| F-152 | minor | confirmed | perf | `src/transfer/setup.rs`（约 :342，`VulkanCommandPool::attach`） | 每次 record 都 `attach` 新建 wrapper（含 SyncCell 与 TempList），仅作临时句柄。 |
| F-156 | minor | confirmed | perf | `src/command/resource_access/mod.rs`（约 :347） | `find(|stages| !issued.contains(stages))` 每轮线性扫描，O(k·n)，k 通常很小。 |
| F-157 | minor | confirmed | correctness | `src/format/mod.rs`（约 :105，`bytes[9] = bytes[8]`） | 格式投影的第 10 字节是推断值，观测表只含 word00/word04/byte08。当前消费者（country-core 的 texture_library）未使用该字节，但公开投影仍暴露它。 |
| F-159 | minor | suspected | perf | `src/pipeline/shader_interface/mod.rs`（`validate_spirv_layout`，约 :132） | 每次重新做 SPIR-V 反射与 JSON 序列化，未复用已缓存的 interfaces。**未核实**调用频率。 |
| F-162 | minor | confirmed | perf | `src/pipeline/shader/reflection.rs`（约 :201、:241） | 每个 entry point 都 `module.assemble()` 重序列化整模块，O(entries×module)，仅加载期。 |
| F-168 | minor | confirmed | test | `src/rg/transient_resources/tests.rs` | `lend` / `give_back` / `evict` / `collect_retiring` 的时序无单元测试（检索无对应用例）。 |
| F-170 | minor | confirmed | perf | `src/rg/resource/mod.rs`（`current_version`，约 :148） | 每次调用全量扫描所有 pass 的全部访问；`buffer_id` / `texture_id` 与 `PassBuilder::latest_version` 都经过它，typed 构图呈超线性。 |
| F-175 | minor | confirmed | perf | `src/rg/temporal/mod.rs`（`requested`，约 :295） | 每次分配并 clone 一个 `Vec`，同帧调用者会重复调用。 |
| F-176 | minor | confirmed | perf | `src/rg/temporal/mod.rs`（`owns_token`，约 :180） | 线性扫描 `resources`；遍历全部后端 token 时逐个调用，O(n·m)。 |
| F-187 | minor | confirmed | perf | `src/rg/resource_contracts/mod.rs`（`covers_iter`，约 :130） | 每个 read 线性扫描全部前序 write，且反复 `writes.clone()`。 |
| F-201 | minor | confirmed | perf | `src/rg/graph_caches.rs`（`evict_idle`，约 :162-170） | 只要拿到 `try_lock` 就无条件丢弃投影快照与投影池，即便 parked 已足够 target；任何一次压力驱逐都会强制下一帧重建 CPU 投影。 |
| F-222 | minor | suspected | test | `src/transfer/texture/tests.rs`、`src/transfer/texture/mod.rs` | `pack_regions` 与 `record_transfer_dst_copies` 的直接测试**未核实**（texture 测试文件中只见布局用例）。 |
| F-225 | minor | confirmed | test | `src/resource/sampler.rs` | 文件内无内联测试；`tests/` 中也未检索到对 `create_info()` 与 13 个内置 spec 映射的覆盖。 |

## 2. 按文件分布

| 文件（或模块） | 发现 ID |
| --- | --- |
| `src/rg/sync_plan/bind/`、`src/rg/sync_plan/mod.rs` | F-092 F-093 F-091 |
| `src/rg/queue_schedule/mod.rs` | F-097 F-098 |
| `src/rg/bindless_binding.rs`、`src/bindless/parameters/mod.rs` | F-104 F-107 |
| `src/pipeline/` | F-110 F-148 F-159 F-162 |
| `src/accounting/stats/state.rs`、`src/device/memory/mod.rs` | F-001 F-032 |
| `src/device/queue_config/mod.rs`、`src/device/logical.rs`、`src/device/semaphore.rs` | F-036 F-074 F-076 |
| `src/present/swapchain/fullscreen/mod.rs` | F-050 |
| `src/native/physical_device/mod.rs` | F-062 |
| `src/transfer/`（setup、queue、texture） | F-064 F-116 F-117 F-152 F-222 |
| `src/command/crash/mod.rs`、`src/command/resource_access/mod.rs` | F-072 F-156 |
| `src/rg/graph/instantiate.rs`、`src/rg/plan/compile.rs` | F-084 F-086 |
| `src/tracking/range_state/storage.rs` | F-122 |
| `src/format/mod.rs` | F-157 |
| `src/rg/resource/mod.rs`、`src/rg/temporal/mod.rs`、`src/rg/resource_contracts/mod.rs` | F-170 F-175 F-176 F-187 |
| `src/rg/graph_caches.rs` | F-201 |
| `src/rg/transient_resources/tests.rs`、`src/resource/sampler.rs` | F-168 F-225 |

## 3. 与其他文档的矛盾

以下矛盾需要由项目所有者处理；本文件不修改代码或其他文档。

1. **工作区根 `AGENTS.md`** 指向的 `reverse-engineering/tiny-glade/` 目录在当前工作区不存在，相关入口不可用（与 `WORK_PACKAGE_AUDIT.md` 的源目录缺失一致）。

## 4. 方法与限制

- **覆盖**：`crates/rhapsody/src` 下 389 个 `.rs` 文件（含内联测试），合计 84278 行（`wc -l`）。本轮**没有**重新逐行走查全部源码；只对未关闭的旧发现做 symbol 级 `rg` 核对与关键位置阅读，已修复、误报、已删除的条目从台账中移除。
- **已关闭条目**：不在台账中保留。判定依据是当前代码中的符号与调用点；未逐条复核的已关闭条目不再重新列出。
- **限制**：静态阅读无法证明并发时序、GPU 行为与 Drop 顺序。性能条目是静态复杂度分析，不等于已测到热点。
- **复核命令**（行号漂移后可用）：`rg -n "<发现文本中的特征串>" crates/rhapsody/src`。

## 5. 需要运行验证或文档确认的条目

| 发现 | 为什么静态不可判定 | 建议验证方式 |
| --- | --- | --- |
| F-098、F-104、F-110 | 批次条件、查找形式与字节比较分支需要对照调用点确认 | 阅读调用点；对 pipeline 缓存构造两个仅 SPIR-V 不同的请求，计数字节比较次数 |
| F-032 | 压力路径是否仍绕过 `ReclamationConfig` 取决于当前调用链 | 构造压力场景，统计单次回收销毁对象数 |
| F-074、F-076、F-091、F-107、F-159 | 调用频率决定是否为热路径 | 计数器或基准测试，区分一次性路径与每帧路径 |
| F-062、F-222 | 集成测试是否已覆盖 | 阅读 `tests/` 中对应用例后决定是否补测试 |

## 6. 优先关注（按处理顺序）

| 顺序 | 发现 | 一句话 |
| ---: | --- | --- |
| 1 | F-148 | 注册单个 shader 会清空全部布局与缓存管线 |
| 2 | F-092、F-093 | 每帧绑定路径的 O(n²) 去重与依赖扫描 |
| 3 | F-201 | 任何压力驱逐都会强制下一帧重建 CPU 投影 |
| 4 | F-097 | 计划编译中的跨队列契约检查超线性 |
| 5 | F-170 | `current_version` 全表扫描，typed 构图超线性 |
| 6 | F-168、F-225 | 时序与格式映射的测试缺口 |
